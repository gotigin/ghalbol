#!/usr/bin/env bash
# Apply Ghalbol web/wasm Makepad patches onto the 1f60033 git checkout.
# Idempotent. Called from sites/ghalbol scripts before cargo-makepad wasm builds.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
REV="${GHALBOL_MAKEPAD_REV:-1f60033ed3b1158cb7bf93accf1c91e0166c9373}"
PATCHES="$ROOT/patches/makepad"
CHECKOUTS="${CARGO_HOME:-$HOME/.cargo}/git/checkouts"
SHORT="${REV:0:7}"

MAKEPAD_ROOT="${MAKEPAD_ROOT:-}"
if [[ -z "$MAKEPAD_ROOT" || ! -d "$MAKEPAD_ROOT/platform" ]]; then
  MAKEPAD_ROOT=""
  for d in "$CHECKOUTS"/makepad-*/"$SHORT"*; do
    if [[ -d "$d/platform" ]]; then
      MAKEPAD_ROOT="$d"
      break
    fi
  done
fi
if [[ -z "$MAKEPAD_ROOT" && -d "$CHECKOUTS/makepad-ec2f134f34cd9f98/$SHORT" ]]; then
  MAKEPAD_ROOT="$CHECKOUTS/makepad-ec2f134f34cd9f98/$SHORT"
fi
if [[ -z "$MAKEPAD_ROOT" || ! -d "$MAKEPAD_ROOT/platform" ]]; then
  echo "error: Makepad checkout $SHORT not found under $CHECKOUTS" >&2
  exit 1
fi

echo "Ghalbol Makepad web patches → $MAKEPAD_ROOT"
apply_one() {
  local name="$1" file="$2"
  local p="$PATCHES/$file"
  if [[ ! -f "$p" ]]; then
    echo "error: missing $p" >&2
    exit 1
  fi
  if git -C "$MAKEPAD_ROOT" apply --reverse --check "$p" >/dev/null 2>&1; then
    echo "  $name already applied"
  elif git -C "$MAKEPAD_ROOT" apply --check "$p" >/dev/null 2>&1; then
    git -C "$MAKEPAD_ROOT" apply "$p"
    echo "  applied $name"
  else
    # Working tree may already include a superset of the patch (manual follow-ups).
    echo "  $name: patch does not reverse-check; assuming already present ($file)"
  fi
}
apply_one wasm_no_instant_retained_upload wasm_no_instant_retained_upload.rs.patch
apply_one no_duplicate_vulkan_bitflags_cfg_if no_duplicate_vulkan_bitflags_cfg_if.toml.patch
# Prefer MSDF over Slug on web/Android; required base for the sync MSDF fallback.
apply_one android_slug_msdf_first_paint android_slug_msdf_first_paint.rs.patch
# --no-threads wasm cannot spawn the MSDF worker; rasterize on the UI thread.
apply_one wasm_sync_msdf_no_threads wasm_sync_msdf_no_threads.rs.patch
