#!/usr/bin/env bash
# Resolve cargo-makepad for Makepad rev 1f60033 (same as ghalbol apps).
# Source from other scripts:  source "$SITE/scripts/makepad-tools.sh"
REV="${GHALBOL_MAKEPAD_REV:-1f60033ed3b1158cb7bf93accf1c91e0166c9373}"
SHORT="${REV:0:7}"
CHECKOUTS="${CARGO_HOME:-$HOME/.cargo}/git/checkouts"

resolve_makepad_root() {
  if [[ -n "${MAKEPAD_ROOT:-}" && -d "$MAKEPAD_ROOT/tools/cargo_makepad" ]]; then
    return 0
  fi
  MAKEPAD_ROOT=""
  local d
  for d in "$CHECKOUTS"/makepad-*/"$SHORT"; do
    if [[ -d "$d/tools/cargo_makepad" ]]; then
      MAKEPAD_ROOT="$d"
      break
    fi
  done
  if [[ -z "$MAKEPAD_ROOT" && -d "$CHECKOUTS/makepad-ec2f134f34cd9f98/$SHORT" ]]; then
    MAKEPAD_ROOT="$CHECKOUTS/makepad-ec2f134f34cd9f98/$SHORT"
  fi
  if [[ -z "$MAKEPAD_ROOT" || ! -d "$MAKEPAD_ROOT/tools/cargo_makepad" ]]; then
    echo "error: Makepad checkout $SHORT not found under $CHECKOUTS" >&2
    echo "  cargo check -p ghalbol-site once to fetch it, or set MAKEPAD_ROOT" >&2
    return 1
  fi
}

cargo_makepad() {
  resolve_makepad_root || return 1
  # Web Instant panic fix (and any other site wasm patches) before build.
  local ROOT_FOR_PATCH
  ROOT_FOR_PATCH="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
  if [[ -x "$ROOT_FOR_PATCH/scripts/apply-makepad-web-patches.sh" ]]; then
    MAKEPAD_ROOT="$MAKEPAD_ROOT" "$ROOT_FOR_PATCH/scripts/apply-makepad-web-patches.sh"
  fi
  # Prefer a built cargo-makepad binary so we do not rebuild the tool (and its
  # upstream deprecation noise) on every site run.
  local BIN="$MAKEPAD_ROOT/target/debug/cargo-makepad"
  if [[ ! -x "$BIN" ]]; then
    cargo build -q --manifest-path "$MAKEPAD_ROOT/tools/cargo_makepad/Cargo.toml"
  fi
  "$BIN" "$@"
}
