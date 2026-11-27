#!/usr/bin/env bash
# Run ghalbol.com locally — one tree like deploy:
#   1) apply Makepad web patches + build debug wasm
#   2) overlay legal/connect/linux beside Makepad home
#   3) local server (GET + POST /api/crash + connect rewrites)
#
# Serves at http://127.0.0.1:8030
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
SITE="$ROOT/sites/ghalbol"
PKG="$ROOT/target/makepad-wasm-app/debug/ghalbol-site"
PORT="${PORT:-8030}"

# shellcheck source=makepad-tools.sh
source "$SITE/scripts/makepad-tools.sh"

free_port() {
  local port="$1"
  local pids=""
  if command -v ss >/dev/null 2>&1; then
    # rg exits 1 when nothing matches; keep going under pipefail when the port is free.
    pids="$(ss -ltnp "( sport = :${port} )" 2>/dev/null | rg -o 'pid=[0-9]+' | cut -d= -f2 | sort -u | tr '\n' ' ' || true)"
  fi
  if [[ -z "${pids// }" ]] && command -v fuser >/dev/null 2>&1; then
    pids="$(fuser "${port}/tcp" 2>/dev/null || true)"
  fi
  if [[ -n "${pids// }" ]]; then
    echo "Port ${port} in use (pids:${pids}) — stopping."
    # shellcheck disable=SC2086
    kill -TERM ${pids} 2>/dev/null || true
    sleep 0.4
    # shellcheck disable=SC2086
    kill -KILL ${pids} 2>/dev/null || true
    if command -v fuser >/dev/null 2>&1; then
      fuser -k "${port}/tcp" >/dev/null 2>&1 || true
    fi
    sleep 0.3
  fi
  if command -v ss >/dev/null 2>&1; then
    if ss -ltn "( sport = :${port} )" 2>/dev/null | rg -q ":${port}"; then
      echo "error: port ${port} still in use after kill" >&2
      ss -ltnp "( sport = :${port} )" 2>/dev/null || true
      exit 1
    fi
  fi
}

cd "$ROOT"
# Makepad 1f60033 needs atomics/threads for image decode + glyph workers.
# Local server sends COOP/COEP below so SharedArrayBuffer is available.
cargo_makepad wasm build -p ghalbol-site "$@"

if [[ ! -f "$PKG/index.html" ]]; then
  echo "error: missing $PKG/index.html" >&2
  exit 1
fi

"$SITE/scripts/overlay-legal-onto-pkg.sh" "$PKG"
mkdir -p "$PKG/i18n" "$PKG/js" "$PKG/css"
cp "$SITE"/translations/*.tsv "$PKG/i18n/"
cp "$SITE/static/js/lang-nav.js" "$PKG/js/lang-nav.js"
cp "$SITE/static/js/site-i18n.js" "$PKG/js/site-i18n.js"
cp "$SITE/static/css/auth-nav.css" "$PKG/css/auth-nav.css"

free_port "$PORT"

echo "Serving $PKG at http://127.0.0.1:${PORT}/ (Ctrl+C to stop)"
echo "    One tree: / = Makepad SPA; /privacy /connect /download/linux = HTML"
echo "    POST /api/crash accepted (Makepad crash reporter)"
echo "    COOP/COEP on (threaded wasm / SharedArrayBuffer)"
cd "$PKG"
exec python3 - "$PORT" <<'PY'
import http.server, socketserver, sys, urllib.parse
from pathlib import Path

port = int(sys.argv[1])
root = Path(".").resolve()

class Handler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        # Threaded Makepad wasm needs these for SharedArrayBuffer.
        self.send_header("Cross-Origin-Opener-Policy", "same-origin")
        self.send_header("Cross-Origin-Embedder-Policy", "require-corp")
        self.send_header("Cross-Origin-Resource-Policy", "same-origin")
        super().end_headers()

    def do_POST(self):
        parsed = urllib.parse.urlparse(self.path)
        if parsed.path.rstrip("/") == "/api/crash":
            length = int(self.headers.get("Content-Length", "0") or "0")
            if length:
                self.rfile.read(length)
            self.send_response(204)
            self.send_header("Cache-Control", "no-store")
            self.end_headers()
            return
        self.send_error(404, "Not found")

    def send_head(self):
        parsed = urllib.parse.urlparse(self.path)
        rel = urllib.parse.unquote(parsed.path.lstrip("/"))
        path = (root / rel).resolve() if rel else root
        try:
            path.relative_to(root)
        except ValueError:
            self.send_error(403)
            return None
        if path.is_file():
            return super().send_head()
        if path.is_dir() and (path / "index.html").is_file():
            self.path = ("/" + rel.rstrip("/") + "/index.html") if rel else "/index.html"
            return super().send_head()
        parts = [p for p in rel.split("/") if p]
        if parts and parts[0] == "connect":
            self.path = "/connect/index.html"
            return super().send_head()
        if len(parts) >= 2 and parts[0] == "download" and parts[1] == "linux":
            self.path = "/download/linux/index.html"
            return super().send_head()
        if parts[:1] == ["json"] and parts[1:2] == ["version"]:
            self.send_error(404)
            return None
        # Never SPA-fallback binary/font assets to index.html (poisoned font bytes).
        lower = rel.lower()
        if lower.endswith((
            ".ttf", ".otf", ".woff", ".woff2", ".wasm", ".bin",
            ".png", ".jpg", ".jpeg", ".webp", ".svg", ".glb",
        )):
            self.send_error(404)
            return None
        self.path = "/index.html"
        return super().send_head()

    def log_message(self, fmt, *args):
        sys.stderr.write("%s - - [%s] %s\n" % (self.address_string(), self.log_date_time_string(), fmt % args))

# allow_reuse_address must be set on the class before __init__ binds the socket.
class ReuseThreadingTCPServer(socketserver.ThreadingTCPServer):
    allow_reuse_address = True
    daemon_threads = True

with ReuseThreadingTCPServer(("127.0.0.1", port), Handler) as httpd:
    httpd.serve_forever()
PY
