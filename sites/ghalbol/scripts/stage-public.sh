#!/usr/bin/env bash
# Stage sites/ghalbol/public/ from release wasm + static overlay (Wadhu pattern).
#
#   ./sites/ghalbol/scripts/stage-public.sh
#   ./sites/ghalbol/scripts/stage-public.sh --no-wasm   # static + i18n only (no Makepad home)
set -euo pipefail

SITE="$(cd "$(dirname "$0")/.." && pwd)"
ROOT="$(cd "$SITE/../.." && pwd)"
OUT="$SITE/public"
WASM_PKG="$ROOT/target/makepad-wasm-app/release/ghalbol-site"
DO_WASM=1

for arg in "$@"; do
  case "$arg" in
    --no-wasm) DO_WASM=0 ;;
    -h|--help)
      sed -n '2,10p' "$0"
      exit 0
      ;;
    *)
      echo "Unknown arg: $arg" >&2
      exit 1
      ;;
  esac
done

# shellcheck source=makepad-tools.sh
source "$SITE/scripts/makepad-tools.sh"

echo "1/4 Bundle Firebase Analytics → static/js/firebase-init.js"
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
if [[ -f "$SITE/static/index.html" ]]; then
  echo "error: static/index.html must not exist — home is Makepad only" >&2
  exit 1
fi

if [[ "$DO_WASM" -eq 1 ]]; then
  echo "2/4 Build Makepad wasm (ghalbol-site, release)"
  (
    cd "$ROOT"
    cargo_makepad wasm --strip --brotli build -p ghalbol-site --release
  )
  shopt -s nullglob
  wasm_files=("$WASM_PKG"/ghalbol-site.wasm "$WASM_PKG"/ghalbol-site.*.wasm)
  shopt -u nullglob
  if [[ ! -f "$WASM_PKG/index.html" || ${#wasm_files[@]} -eq 0 ]]; then
    echo "error: wasm package incomplete at $WASM_PKG" >&2
    exit 1
  fi
else
  echo "2/4 Skipping wasm build (--no-wasm)"
fi

echo "3/4 Stage $OUT = wasm home + static overlay"
rm -rf "$OUT"
mkdir -p "$OUT"
shopt -s nullglob
wasm_files=("$WASM_PKG"/ghalbol-site.wasm "$WASM_PKG"/ghalbol-site.*.wasm)
shopt -u nullglob
if [[ -f "$WASM_PKG/index.html" && ${#wasm_files[@]} -gt 0 ]]; then
  rsync -a --exclude='*.br' "$WASM_PKG/" "$OUT/"
  "$SITE/scripts/overlay-legal-onto-pkg.sh" "$OUT"
else
  if [[ "$DO_WASM" -eq 1 ]]; then
    echo "error: missing release wasm at $WASM_PKG" >&2
    exit 1
  fi
  echo "note: staging static/ only (no Makepad home)" >&2
  rsync -a "$SITE/static/" "$OUT/"
fi

mkdir -p "$OUT/i18n" "$OUT/js" "$OUT/css"
cp "$SITE"/translations/*.tsv "$OUT/i18n/"
cp "$SITE/static/js/lang-nav.js" "$OUT/js/lang-nav.js"
cp "$SITE/static/js/site-i18n.js" "$OUT/js/site-i18n.js"
cp "$SITE/static/css/auth-nav.css" "$OUT/css/auth-nav.css"
cp "$SITE/static/js/firebase-init.js" "$OUT/js/firebase-init.js"
if [[ -f "$SITE/static/js/account.js" ]]; then
  cp "$SITE/static/js/account.js" "$OUT/js/account.js"
fi

BUILD_ID="$(date -u +%Y%m%d%H%M%S)-$(git -C "$ROOT" rev-parse --short HEAD 2>/dev/null || echo local)"
INDEX="$OUT/index.html"
if [[ ! -f "$INDEX" ]]; then
  echo "error: missing $INDEX" >&2
  exit 1
fi

if grep -qE 'ghalbol-site(\.[0-9a-f]+)?\.wasm|web_gl|full_canvas' "$INDEX"; then
  awk -v bid="$BUILD_ID" '
    {
      gsub(/\.\/makepad_platform\/web_gl\.js/, "./makepad_platform/web_gl.js?v=" bid)
      gsub(/\.\/ghalbol-site(\.[0-9a-f]+)?\.wasm/, "&?v=" bid)
    }
    /<meta name="theme-color"/ && !stamped {
      print
      print "        <meta name=\"ghalbol-build\" content=\"" bid "\" />"
      stamped = 1
      next
    }
    { print }
  ' "$INDEX" > "$INDEX.tmp" && mv "$INDEX.tmp" "$INDEX"
elif [[ "$DO_WASM" -eq 1 ]]; then
  echo "error: staged index.html is not the Makepad home" >&2
  exit 1
fi

# Cache-bust Analytics on HTML shells that reference it.
for html in \
  "$OUT/privacy/index.html" \
  "$OUT/terms/index.html" \
  "$OUT/support/index.html" \
  "$OUT/about/index.html" \
  "$OUT/data-deletion/index.html" \
  "$OUT/cancellation/index.html" \
  "$OUT/account/index.html" \
  "$OUT/connect/index.html" \
  "$OUT/download/linux/index.html"; do
  if [[ ! -f "$html" ]]; then
    echo "error: missing $html" >&2
    exit 1
  fi
  if grep -q 'firebase-init\.js' "$html"; then
    sed -i -E "s#(/js/firebase-init\\.js)\\?v=[^\"]*#\\1?v=${BUILD_ID}#g; s#src=\"(/js/firebase-init\\.js)\"#src=\"\\1?v=${BUILD_ID}\"#" "$html"
  fi
  for asset in auth-nav.js lang-nav.js site-i18n.js; do
    if grep -q "$asset" "$html"; then
      sed -i -E "s#(/js/${asset})\\?v=[^\"]*#\\1?v=${BUILD_ID}#g; s#src=\"(/js/${asset})\"#src=\"\\1?v=${BUILD_ID}\"#" "$html"
    fi
  done
done

PREVIEW="$OUT/media/link-preview.jpg"
if [[ ! -f "$PREVIEW" ]]; then
  echo "error: missing static/media/link-preview.jpg — run sites/ghalbol/scripts/make-link-preview.sh" >&2
  exit 1
fi
PREVIEW_V="$(sha256sum "$PREVIEW" | cut -c1-10)"
for html in "$INDEX" \
  "$OUT/privacy/index.html" \
  "$OUT/terms/index.html" \
  "$OUT/support/index.html" \
  "$OUT/about/index.html" \
  "$OUT/data-deletion/index.html" \
  "$OUT/cancellation/index.html" \
  "$OUT/account/index.html" \
  "$OUT/connect/index.html" \
  "$OUT/download/linux/index.html"; do
  sed -i -E "s#(https://ghalbol\\.com/media/link-preview\\.jpg)(\\?v=[0-9a-f]+)?\"#\\1?v=${PREVIEW_V}\"#g" "$html"
  if ! grep -q "link-preview.jpg?v=${PREVIEW_V}" "$html"; then
    echo "error: link-preview tags missing from $html" >&2
    exit 1
  fi
done

echo "4/4 Check language packs → $OUT/i18n"
"$SITE/scripts/check-translations.sh"
pack_n="$(find "$OUT/i18n" -name '*.tsv' | wc -l | tr -d ' ')"
if [[ "$pack_n" -ne 110 ]]; then
  echo "error: expected 110 packs in $OUT/i18n, found $pack_n" >&2
  exit 1
fi
if [[ ! -f "$OUT/.well-known/assetlinks.json" ]]; then
  echo "error: missing $OUT/.well-known/assetlinks.json" >&2
  exit 1
fi
if [[ ! -f "$OUT/js/site-i18n.js" || ! -f "$OUT/js/lang-nav.js" ]]; then
  echo "error: missing lang-nav.js or site-i18n.js in $OUT/js" >&2
  exit 1
fi

printf '%s\n' "$BUILD_ID" >"$OUT/build-id.txt"
echo "Staged public/ (build-id=$BUILD_ID)"
