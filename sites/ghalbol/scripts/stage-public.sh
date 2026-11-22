#!/usr/bin/env bash
# Stage sites/ghalbol/public/ from static/ + bundled Analytics (same idea as
# wadhu/khaynjo staging before preview or deploy).
#
#   ./sites/ghalbol/scripts/stage-public.sh
set -euo pipefail

SITE="$(cd "$(dirname "$0")/.." && pwd)"
ROOT="$(cd "$SITE/../.." && pwd)"
OUT="$SITE/public"

echo "1/2 Bundle Firebase Analytics → static/js/firebase-init.js"
(
  cd "$SITE"
  if [[ ! -d node_modules/firebase ]]; then
    npm install
  fi
  npm run build
)
if [[ ! -f "$SITE/static/js/firebase-init.js" ]]; then
  echo "error: missing static/js/firebase-init.js after npm run build" >&2
  exit 1
fi

echo "2/2 Stage $OUT from static/"
rm -rf "$OUT"
mkdir -p "$OUT"
# Keep .well-known (Firebase ignore must not use **/.* — see firebase.json).
rsync -a "$SITE/static/" "$OUT/"

BUILD_ID="$(date -u +%Y%m%d%H%M%S)-$(git -C "$ROOT" rev-parse --short HEAD 2>/dev/null || echo local)"
INDEX="$OUT/index.html"
if [[ ! -f "$INDEX" ]]; then
  echo "error: missing $INDEX" >&2
  exit 1
fi
if [[ ! -f "$OUT/js/firebase-init.js" ]]; then
  echo "error: missing $OUT/js/firebase-init.js (static/js must be staged)" >&2
  exit 1
fi
if [[ ! -f "$OUT/.well-known/assetlinks.json" ]]; then
  echo "error: missing $OUT/.well-known/assetlinks.json" >&2
  exit 1
fi

# Inject / refresh Analytics script with cache-bust query (Khaynjo/Wadhu pattern).
if grep -q 'firebase-init\.js' "$INDEX"; then
  sed -i -E "s#(/js/firebase-init\\.js)\\?v=[^\"]*#\\1?v=${BUILD_ID}#g; s#src=\"(/js/firebase-init\\.js)\"#src=\"\\1?v=${BUILD_ID}\"#" "$INDEX"
else
  awk -v bid="$BUILD_ID" '
    /<\/body>/ && !done {
      print "  <script src=\"/js/firebase-init.js?v=" bid "\" defer></script>"
      done=1
    }
    { print }
    END { if (!done) exit 2 }
  ' "$INDEX" >"$INDEX.tmp" && mv "$INDEX.tmp" "$INDEX"
fi

# The link-preview picture's address carries a short hash of the file:
# WhatsApp, Telegram and Facebook keep a preview for days and only fetch
# again for a new address.
PREVIEW="$OUT/media/link-preview.jpg"
if [[ ! -f "$PREVIEW" ]]; then
  echo "error: missing static/media/link-preview.jpg — run sites/ghalbol/scripts/make-link-preview.sh" >&2
  exit 1
fi
PREVIEW_V="$(sha256sum "$PREVIEW" | cut -c1-10)"
# Home + static secondary shells (privacy/terms/support/about/data-deletion).
for html in "$INDEX" \
  "$OUT/privacy/index.html" \
  "$OUT/terms/index.html" \
  "$OUT/support/index.html" \
  "$OUT/about/index.html" \
  "$OUT/data-deletion/index.html"; do
  if [[ ! -f "$html" ]]; then
    echo "error: missing $html" >&2
    exit 1
  fi
  sed -i -E "s#(https://ghalbol\\.com/media/link-preview\\.jpg)(\\?v=[0-9a-f]+)?\"#\\1?v=${PREVIEW_V}\"#g" "$html"
  if ! grep -q "link-preview.jpg?v=${PREVIEW_V}" "$html"; then
    echo "error: link-preview tags missing from $html" >&2
    exit 1
  fi
done

printf '%s\n' "$BUILD_ID" >"$OUT/build-id.txt"
echo "Staged public/ (build-id=$BUILD_ID)"
