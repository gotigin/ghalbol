#!/usr/bin/env bash
# Compatibility wrapper — prefer ./sites/ghalbol/scripts/deploy-firebase.sh
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
exec "$ROOT/sites/ghalbol/scripts/deploy-firebase.sh" "$@"
