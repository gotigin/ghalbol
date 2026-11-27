#!/usr/bin/env bash
# Put ghalbol.com's start-up screen into a staged Makepad wasm index.html.
#
#   ./sites/ghalbol/scripts/patch-web-shell.sh <path/to/index.html>
set -euo pipefail

INDEX="${1:?usage: patch-web-shell.sh <path/to/index.html>}"
SITE="$(cd "$(dirname "$0")/.." && pwd)"
SHELL_SRC="$SITE/shell"

if [[ ! -f "$INDEX" ]]; then
  echo "error: missing $INDEX" >&2
  exit 1
fi
PKG="$(cd "$(dirname "$INDEX")" && pwd)"
for f in boot.css boot.html boot.js logo.webp; do
  if [[ ! -f "$SHELL_SRC/$f" ]]; then
    echo "error: missing $SHELL_SRC/$f" >&2
    exit 1
  fi
done
# Debug packages use ghalbol-site.<hash>.wasm; release may be plain or hashed.
WASM_FILE=""
for cand in "$PKG"/ghalbol-site.wasm "$PKG"/ghalbol-site.*.wasm; do
  if [[ -f "$cand" ]]; then
    WASM_FILE="$cand"
    break
  fi
done
if [[ -z "$WASM_FILE" ]]; then
  echo "error: missing ghalbol-site*.wasm in $PKG (needed for the progress ring)" >&2
  exit 1
fi

inject_lang_nav() {
  local index="$1"
  if grep -q 'lang-nav.js' "$index"; then
    return 0
  fi
  awk '
    /<\/body>/ && !done {
      print "        <script src=\"./js/lang-nav.js\" defer></script>"
      done = 1
    }
    { print }
    END { if (!done) exit 3 }
  ' "$index" > "$index.langtmp"
  mv "$index.langtmp" "$index"
}

if grep -q 'id="boot"' "$INDEX"; then
  inject_lang_nav "$INDEX"
  echo "Already patched: $INDEX"
  exit 0
fi
if ! grep -q "class='canvas_loader'" "$INDEX"; then
  echo "error: $INDEX has no canvas_loader block — cargo-makepad changed its shell" >&2
  exit 1
fi

WASM_BYTES="$(stat -c %s "$WASM_FILE")"
BODY_TMP="$(mktemp)"
trap 'rm -f "$BODY_TMP" "$INDEX.tmp"' EXIT
{
  printf 's|__WASM_BYTES__|%s|\n' "$WASM_BYTES"
  printf 's|__LOGO_DATA_URI__|data:image/webp;base64,%s|\n' "$(base64 -w0 "$SHELL_SRC/logo.webp")"
} > "$BODY_TMP.sed"
sed -f "$BODY_TMP.sed" "$SHELL_SRC/boot.html" > "$BODY_TMP"
rm -f "$BODY_TMP.sed"

GUARD_SRC="$SHELL_SRC/mobile-viewport-guard.js"
if [[ ! -f "$GUARD_SRC" ]]; then
  echo "error: missing $GUARD_SRC" >&2
  exit 1
fi

awk -v css="$SHELL_SRC/boot.css" -v body="$BODY_TMP" -v js="$SHELL_SRC/boot.js" -v guard="$GUARD_SRC" '
  function cat(file,   line) {
    while ((getline line < file) > 0) print line
    close(file)
  }
  skip > 0 {
    if ($0 ~ /<\/div>/) skip--
    next
  }
  /name="viewport"/ {
    print "        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1, maximum-scale=1, user-scalable=no, viewport-fit=cover\" />"
    next
  }
  /<\/head>/ && !styled {
    print "        <style>"
    cat(css)
    print "        </style>"
    print "        <script>"
    cat(guard)
    print "        </script>"
    styled = 1
  }
  /class=.canvas_loader./ && !booted {
    cat(body)
    print "        <script>"
    cat(js)
    print "        </script>"
    booted = 1
    skip = 2
    next
  }
  /<\/body>/ && !langjs {
    print "        <link rel=\"stylesheet\" href=\"./css/auth-nav.css\" />"
    print "        <script src=\"./js/lang-nav.js\" defer></script>"
    langjs = 1
  }
  { print }
  END { if (!styled || !booted || !langjs) exit 3 }
' "$INDEX" > "$INDEX.tmp"
mv "$INDEX.tmp" "$INDEX"

if ! grep -q 'id="boot"' "$INDEX" || grep -q 'Loading\.\.$' "$INDEX"; then
  echo "error: start-up screen was not placed in $INDEX" >&2
  exit 1
fi
if ! grep -q 'lang-nav.js' "$INDEX"; then
  echo "error: lang-nav.js was not injected into $INDEX" >&2
  exit 1
fi
if ! grep -q 'auth-nav.css' "$INDEX"; then
  echo "error: auth-nav.css was not injected into $INDEX" >&2
  exit 1
fi
echo "Patched $INDEX (start-up screen, lang-nav, auth-nav.css, wasm $WASM_BYTES bytes)"
