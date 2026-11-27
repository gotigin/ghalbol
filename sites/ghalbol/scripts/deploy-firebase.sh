#!/usr/bin/env bash
# Deploy ghalbol.com Hosting tree (Makepad home + legal/connect/linux HTML).
# Firebase project: ghalbol (.firebaserc). Custom domains: ghalbol.com, www.
#
#   ./sites/ghalbol/scripts/deploy-firebase.sh
#   ./sites/ghalbol/scripts/deploy-firebase.sh --dry-run
set -euo pipefail

SITE="$(cd "$(dirname "$0")/.." && pwd)"
DRY=0

for arg in "$@"; do
  case "$arg" in
    --dry-run) DRY=1 ;;
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

export PATH="${HOME}/.local/bin:${PATH}"

if ! command -v firebase >/dev/null 2>&1; then
  echo "error: firebase CLI not found (npm i -g firebase-tools)" >&2
  exit 1
fi

"$SITE/scripts/stage-public.sh"

mkdir -p "$SITE/public/i18n"
cp "$SITE"/translations/*.tsv "$SITE/public/i18n/"
if [[ ! -f "$SITE/public/i18n/hi.tsv" || ! -f "$SITE/public/i18n/ur.tsv" ]]; then
  echo "error: public/i18n missing hi.tsv or ur.tsv" >&2
  exit 1
fi

if [[ ! -f "$SITE/public/index.html" || ! -f "$SITE/public/js/firebase-init.js" ]]; then
  echo "error: staging incomplete under $SITE/public" >&2
  exit 1
fi
if ! grep -qE 'ghalbol-site(\.[0-9a-f]+)?\.wasm|web_gl|full_canvas' "$SITE/public/index.html"; then
  echo "error: public/index.html is not the Makepad home" >&2
  exit 1
fi
if [[ ! -f "$SITE/public/.well-known/assetlinks.json" ]]; then
  echo "error: missing assetlinks.json in public/" >&2
  exit 1
fi
if [[ ! -f "$SITE/public/connect/index.html" || ! -f "$SITE/public/download/linux/index.html" ]]; then
  echo "error: missing connect or download/linux after stage" >&2
  exit 1
fi
if ! grep -q 'lang-nav.js' "$SITE/public/index.html"; then
  echo "error: wasm shell missing lang-nav.js" >&2
  exit 1
fi

PROJECT="$(sed -n 's/.*"default"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p' "$SITE/.firebaserc" 2>/dev/null | head -1)"
if [[ -z "$PROJECT" ]]; then
  echo "error: no default project in sites/ghalbol/.firebaserc" >&2
  exit 1
fi
echo "Deploy Firebase Hosting → project $PROJECT"
cd "$SITE"
if [[ "$DRY" -eq 1 ]]; then
  firebase deploy --only hosting --project "$PROJECT" --dry-run
else
  firebase deploy --only hosting --project "$PROJECT"
fi

echo "Done → https://ghalbol.com/ (confirm /privacy, /connect/…, /download/linux, /.well-known/assetlinks.json)"
