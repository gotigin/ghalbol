#!/usr/bin/env bash
# Apply Gotigin Android UX bridges onto the Makepad git checkout used by Ghalbol
# (rev 1f60033 under ~/.cargo/git/checkouts). Idempotent.
# Called from build_android_app.sh / android-dev before cargo-makepad packages.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
REV="${GHALBOL_MAKEPAD_REV:-1f60033ed3b1158cb7bf93accf1c91e0166c9373}"
PATCHES="$ROOT/patches/makepad"
CHECKOUTS="${CARGO_HOME:-$HOME/.cargo}/git/checkouts"

MAKEPAD_ROOT=""
for d in "$CHECKOUTS"/makepad-*/"${REV:0:7}"*; do
  if [[ -d "$d/platform" ]]; then
    MAKEPAD_ROOT="$d"
    break
  fi
done
# Prefer exact short hash folder name used by cargo
if [[ -z "$MAKEPAD_ROOT" && -d "$CHECKOUTS/makepad-ec2f134f34cd9f98/1f60033" ]]; then
  MAKEPAD_ROOT="$CHECKOUTS/makepad-ec2f134f34cd9f98/1f60033"
fi
if [[ -z "$MAKEPAD_ROOT" || ! -d "$MAKEPAD_ROOT" ]]; then
  echo "warn: Makepad checkout for $REV not found — skip UX bridge" >&2
  exit 0
fi

echo "Ghalbol Android UX bridge → $MAKEPAD_ROOT"

# Rust patches (best-effort; may already be applied by hand)
for name in mouse_back_button android_slug_msdf_first_paint android_back_and_shader_ghalbol_1f60033; do
  p="$PATCHES/${name}.rs.patch"
  [[ -f "$p" ]] || continue
  if git -C "$MAKEPAD_ROOT" apply --reverse --check "$p" >/dev/null 2>&1; then
    echo "  $name already applied"
  elif git -C "$MAKEPAD_ROOT" apply --check "$p" >/dev/null 2>&1; then
    git -C "$MAKEPAD_ROOT" apply "$p"
    echo "  applied $name"
  else
    echo "  warn: $name does not apply cleanly (may already be hand-ported)" >&2
  fi
done

JAVA="$MAKEPAD_ROOT/tools/cargo_makepad/src/android/java/dev/makepad/android/MakepadActivity.java"
[[ -f "$JAVA" ]] || { echo "warn: MakepadActivity.java missing" >&2; exit 0; }

# moveTaskToBackFromNative
if ! grep -qF 'moveTaskToBackFromNative' "$JAVA"; then
  python3 - "$JAVA" <<'PY'
from pathlib import Path
import sys
p = Path(sys.argv[1])
t = p.read_text()
anchor = """    public void onBackPressed() {
        // Navigation is handled asynchronously by the Makepad UI. The superclass
        // would finish/background this activity before that UI can dismiss an overlay.
        MakepadNative.onBackPressed();
    }
"""
insert = """    public void onBackPressed() {
        // Navigation is handled asynchronously by the Makepad UI. The superclass
        // would finish/background this activity before that UI can dismiss an overlay.
        MakepadNative.onBackPressed();
    }

    /** Soft Back with nothing left to pop: background the task (Home), don't kill the session. */
    public void moveTaskToBackFromNative() {
        runOnUiThread(new Runnable() {
            @Override
            public void run() {
                moveTaskToBack(true);
            }
        });
    }
"""
if anchor in t:
    p.write_text(t.replace(anchor, insert, 1))
    print("  injected moveTaskToBackFromNative")
else:
    print("  warn: onBackPressed anchor missing", file=sys.stderr)
PY
else
  echo "  moveTaskToBackFromNative already present"
fi

# Shader compile screen Java + register
SHADER_DST="$MAKEPAD_ROOT/tools/cargo_makepad/src/android/java/dev/makepad/android/MakepadShaderCompileScreen.java"
if [[ -f "$PATCHES/MakepadShaderCompileScreen.java" ]]; then
  cp "$PATCHES/MakepadShaderCompileScreen.java" "$SHADER_DST"
fi
RES_VALUES="$MAKEPAD_ROOT/tools/cargo_makepad/src/android/res/values"
mkdir -p "$RES_VALUES"
for xml in makepad_shader_compile_colors.xml makepad_shader_compile_strings.xml; do
  [[ -f "$PATCHES/$xml" ]] && cp "$PATCHES/$xml" "$RES_VALUES/$xml"
done
COMPILE_RS="$MAKEPAD_ROOT/tools/cargo_makepad/src/android/compile.rs"
if [[ -f "$COMPILE_RS" ]] && ! grep -qF 'MakepadShaderCompileScreen.java' "$COMPILE_RS"; then
  sed -i 's|makepad_java_classes_dir.join("MakepadActivity.java"),|makepad_java_classes_dir.join("MakepadActivity.java"),\n        makepad_java_classes_dir.join("MakepadShaderCompileScreen.java"),|' "$COMPILE_RS"
  echo "  registered MakepadShaderCompileScreen.java"
fi

# Minimal showShaderCompileScreen if missing (full Kumkaj latch may need newer Activity)
if ! grep -qF 'showShaderCompileScreen' "$JAVA"; then
  python3 - "$JAVA" <<'PY'
from pathlib import Path
import sys
p = Path(sys.argv[1])
t = p.read_text()
# Fields near cover overlay
if "mIsCoverWaitingForFirstFrame" not in t and "private FrameLayout mSurfaceCoverOverlay;" in t:
    t = t.replace(
        "    private FrameLayout mSurfaceCoverOverlay;",
        "    private FrameLayout mSurfaceCoverOverlay;\n"
        "    private boolean mIsCoverWaitingForFirstFrame = false;\n"
        "    private MakepadShaderCompileScreen mShaderCompileScreen;\n"
        "    private int mAppBackgroundColor = 0xFFF5F5F5;",
        1,
    )
meth = """
    public void showShaderCompileScreen() {
        runOnUiThread(new Runnable() {
            @Override
            public void run() {
                if (mSurfaceCoverOverlay == null || mShaderCompileScreen != null) {
                    return;
                }
                mIsCoverWaitingForFirstFrame = true;
                mSurfaceCoverOverlay.setVisibility(View.VISIBLE);
                mShaderCompileScreen = new MakepadShaderCompileScreen(MakepadActivity.this);
                try {
                    mShaderCompileScreen.applyColors(mAppBackgroundColor);
                } catch (Throwable ignored) {}
                mSurfaceCoverOverlay.addView(mShaderCompileScreen, new FrameLayout.LayoutParams(
                    ViewGroup.LayoutParams.MATCH_PARENT, ViewGroup.LayoutParams.MATCH_PARENT));
            }
        });
    }

    private void removeShaderCompileScreen() {
        final MakepadShaderCompileScreen compileScreen = mShaderCompileScreen;
        if (compileScreen == null) {
            return;
        }
        mShaderCompileScreen = null;
        mIsCoverWaitingForFirstFrame = false;
        ViewGroup parent = (ViewGroup) compileScreen.getParent();
        if (parent != null) {
            parent.removeView(compileScreen);
        }
    }
"""
# Insert before onBackPressed
anchor = "    public void onBackPressed()"
if "showShaderCompileScreen" not in t and anchor in t:
    t = t.replace(anchor, meth + "\n" + anchor, 1)
    p.write_text(t)
    print("  injected showShaderCompileScreen")
else:
    print("  showShaderCompileScreen skip/present")
PY
else
  echo "  showShaderCompileScreen already present"
fi

# Ensure cover starts visible for cold first frame when possible
if grep -qF 'mSurfaceCoverOverlay.setVisibility(View.GONE);' "$JAVA" \
  && ! grep -qF 'mIsCoverWaitingForFirstFrame = true;' "$JAVA"; then
  sed -i 's|mSurfaceCoverOverlay.setVisibility(View.GONE);|mSurfaceCoverOverlay.setVisibility(View.VISIBLE);\n        mIsCoverWaitingForFirstFrame = true;|' "$JAVA" || true
  echo "  armed cover VISIBLE on create (best-effort)"
fi

echo "Ghalbol Android UX bridge done."
