#!/usr/bin/env bash
# Deploy ghalbol.com Hosting tree (Khaynjo/Wadhu pattern).
# Firebase project: ghalbol (.firebaserc). Custom domains: ghalbol.com, www.
#
#   ./sites/ghalbol/scripts/deploy-firebase.sh
#   ./sites/ghalbol/scripts/deploy-firebase.sh --dry-run
#
# One-time: npm i -g firebase-tools && firebase login
# Linux tarball (optional): ./scripts/package_linux_release.sh before deploy.
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

if [[ ! -f "$SITE/public/index.html" || ! -f "$SITE/public/js/firebase-init.js" ]]; then
  echo "error: staging incomplete under $SITE/public" >&2
  exit 1
fi
if [[ ! -f "$SITE/public/.well-known/assetlinks.json" ]]; then
  echo "error: missing assetlinks.json in public/" >&2
  exit 1
fi

echo "Deploy Firebase Hosting → project ghalbol"
cd "$SITE"
if [[ "$DRY" -eq 1 ]]; then
  firebase deploy --only hosting --dry-run
else
  firebase deploy --only hosting
fi

echo "Done → https://ghalbol.com/ (confirm /privacy, /download/linux, /.well-known/assetlinks.json)"
