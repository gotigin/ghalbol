#!/usr/bin/env bash
# Static ghalbol.com: Desktop-site clamp + hardened viewport must stay in index.html.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
INDEX="$ROOT/static/index.html"
fail=0
grep -q 'maximum-scale=1' "$INDEX" || { echo "error: $INDEX missing maximum-scale=1" >&2; fail=1; }
grep -q 'Math.min.apply' "$INDEX" || { echo "error: $INDEX missing Desktop-site width clamp" >&2; fail=1; }
grep -q 'ontouchstart' "$INDEX" || { echo "error: $INDEX missing touch Desktop-site guard" >&2; fail=1; }
[[ "$fail" -eq 0 ]] || { echo "check-web-mobile-layout: FAILED" >&2; exit 1; }
echo "check-web-mobile-layout: OK"
