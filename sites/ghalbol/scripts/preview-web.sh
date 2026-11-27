#!/usr/bin/env bash
# Stage the Hosting tree (wasm + static) and serve it locally. No deploy.
#
#   ./sites/ghalbol/scripts/preview-web.sh
#   ./sites/ghalbol/scripts/preview-web.sh --no-build
#   PORT=8080 ./sites/ghalbol/scripts/preview-web.sh
set -euo pipefail

SITE="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$SITE/public"
PORT="${PORT:-8080}"
DO_BUILD=1

for arg in "$@"; do
  case "$arg" in
    --no-build) DO_BUILD=0 ;;
    -h|--help)
      sed -n '2,12p' "$0"
      exit 0
      ;;
    *)
      echo "Unknown arg: $arg" >&2
      exit 1
      ;;
  esac
done

if [[ "$DO_BUILD" -eq 1 ]]; then
  "$SITE/scripts/stage-public.sh"
else
  if [[ ! -f "$OUT/index.html" ]]; then
    echo "error: $OUT missing — run without --no-build first" >&2
    exit 1
  fi
fi

if command -v fuser >/dev/null 2>&1; then
  if fuser "${PORT}/tcp" >/dev/null 2>&1; then
    echo "Port ${PORT} in use — stopping previous listener."
    fuser -k "${PORT}/tcp" >/dev/null 2>&1 || true
    sleep 0.5
  fi
fi

echo "Serving $OUT at http://127.0.0.1:${PORT}/ (Ctrl+C to stop)"
echo "    / = Makepad; /privacy /connect/<hex> /download/linux = HTML"
cd "$OUT"
# Existing files win; directories with index.html work; Firebase-style rewrites
# for /connect/<hex> → /connect/index.html and unknown → /index.html.
exec python3 - "$PORT" <<'PY'
import http.server, socketserver, sys, urllib.parse
from pathlib import Path

port = int(sys.argv[1])
root = Path(".").resolve()

class Handler(http.server.SimpleHTTPRequestHandler):
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
        self.path = "/index.html"
        return super().send_head()

with socketserver.TCPServer(("127.0.0.1", port), Handler) as httpd:
    httpd.serve_forever()
PY
