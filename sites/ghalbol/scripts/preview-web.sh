#!/usr/bin/env bash
# Stage the Hosting tree and serve it locally (SPA rewrites for /connect/…).
# Does NOT deploy. Same shape as kumkaj/khaynjo preview-web.sh.
#
#   ./sites/ghalbol/scripts/preview-web.sh
#   ./sites/ghalbol/scripts/preview-web.sh --no-build   # re-serve last public/
#   PORT=8080 ./sites/ghalbol/scripts/preview-web.sh
#
# Open http://127.0.0.1:8080/ (or $PORT). Try:
#   /  /download/linux  /privacy  /connect/<64-or-66-hex>?alias=Name
set -euo pipefail

SITE="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$SITE/public"
PORT="${PORT:-8080}"
DO_BUILD=1

for arg in "$@"; do
  case "$arg" in
    --no-build) DO_BUILD=0 ;;
    -h|--help)
      sed -n '2,14p' "$0"
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

# Free :$PORT if a previous preview still holds it.
if command -v fuser >/dev/null 2>&1; then
  if fuser "${PORT}/tcp" >/dev/null 2>&1; then
    echo "Port ${PORT} in use — stopping previous listener."
    fuser -k "${PORT}/tcp" >/dev/null 2>&1 || true
    sleep 0.5
  fi
fi

echo "Serving $OUT at http://127.0.0.1:${PORT}/ (Ctrl+C to stop)"
echo "    SPA fallback: unknown paths → /index.html (like Firebase rewrites)"
cd "$OUT"
# Tiny SPA server: existing files win; otherwise index.html (Hosting rewrite).
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
        # Firebase Hosting rewrite: ** → /index.html
        self.path = "/index.html"
        return super().send_head()

with socketserver.TCPServer(("127.0.0.1", port), Handler) as httpd:
    httpd.serve_forever()
PY
