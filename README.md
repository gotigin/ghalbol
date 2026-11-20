# Ghal Bol

**Ghal Bol** (ghalbol.com) is the communication brand of **GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED**. **Gotigin** is the trademark. The shipping product is the **Ghal Bol messenger** — end-to-end encrypted text with voice and video calls. Names: [`docs/GHALBOL_BRAND.md`](docs/GHALBOL_BRAND.md).

## Workspace Structure (Kumkaj Model)

This workspace follows the Kumkaj pattern: **Ghal Bol** is the brand; concrete apps live under `apps/` and own their UI independently (**copy-then-diverge**). Unlike Kumkaj, Ghal Bol **does** share `ghalbol_core` (crypto, delivery, calls) across communication products — UI still stays per-app. Do not spell the brand **GhalBol**.

```
ghalbol/
├── Cargo.toml                    # Workspace root — brand-level config
├── README.md                     # This file — brand overview & structure docs
├── docs/                         # Global brand docs (shared doctrine)
│   ├── GHALBOL_BRAND.md          # Brand identity & naming conventions
│   ├── DESIGN.md                 # Architecture design
│   ├── TRANSPORT.md              # Transport policy
│   └── ...                       # Other design docs
├── ghalbol_core/                # Shared core library (identity, crypto, P2P, delivery)
├── ghalbol_coord/               # Coordination server (presence, endpoint discovery)
├── ghalbol_delivery/            # Delivery server (text mailbox)
├── ghalbol_ddns/                # DDNS helper for home deployment
├── apps/                         # Product apps — each brand's products live here
│   └── ghalbol/                  # Ghal Bol messenger (current product)
│       ├── Cargo.toml            # App crate config
│       ├── build.rs              # Env baking (development.env → binary)
│       ├── src/                  # Makepad 2 UI source
│       ├── resources/            # Fonts, icons, assets
│       ├── scripts/              # Per-app dev scripts
│       │   ├── dev.sh            # Development build (hot reload + Tweaker)
│       │   ├── windows-dev.sh    # Windows x86_64 dev package
│       │   └── android-dev.sh    # Android debug APK
│       └── env/                  # Per-app environment files
│           ├── development.env   # Dev coord/delivery URLs
│           └── production.env    # Prod coord/delivery URLs
├── tools/                        # Per-product cargo entrypoints (thin wrappers)
│   ├── ghalbol-dev/              # `cargo ghalbol-dev` → dev.sh
│   ├── ghalbol-android-dev/      # `cargo ghalbol-android-dev` → android-dev.sh
│   └── ghalbol-windows-dev/      # `cargo ghalbol-windows-dev` → windows-dev.sh
├── dist/                         # Build outputs (gitignored)
│   ├── windows/                  # Production Windows package (Setup.bat per-user install)
│   └── windows-dev/              # Development Windows package
├── scripts/                      # Shared build/packaging tooling
│   ├── load-ghalbol-env.sh       # Env loader (source into other scripts)
│   ├── desktop-ids.sh            # Desktop install paths, manifest helpers
│   ├── desktop-resources.sh      # Desktop package layout (app + Makepad resources)
│   ├── stage-desktop-package.sh  # Package staging (installers + manifests)
│   ├── linux/                    # install.sh / uninstall.sh shipped in the Linux tarball
│   ├── windows/                  # Setup.bat, install.ps1, uninstall.ps1 for Windows packages
│   ├── windows-env.sh            # Windows cross-compile setup
│   ├── cargo-target-dir.sh       # Resolve cargo target directory
│   ├── package-windows.sh        # Production Windows build
│   ├── package_linux_release.sh  # Linux x86_64 release tarball
│   ├── build_android_app.sh      # Android APK / AAB build (cargo-makepad)
│   ├── check_android.sh          # Android type-check without the SDK (CI)
│   └── with-android-env.sh       # Android NDK environment setup
├── env/                          # Optional developer overrides (gitignored .env.*) + templates
├── sites/                        # ghalbol.com (sites/ghalbol — Firebase Hosting)
└── .gitignore                    # Ignored files pattern
```

## Key Design Decisions

### Apps own their UI; `ghalbol_core` owns product logic
- Each `apps/<product>/` owns its UI, resources, env and scripts
- `ghalbol_core` is the Rust library with product logic (crypto, keystore, delivery, calls, contacts, transcripts) — see `AGENTS.md`
- Future Ghal Bol products under `apps/` copy UI code from the best-fitting sibling, then diverge
- For app code, reuse is **copy-then-diverge**; duplication is preferable to coupling

### Names
Order is **Gotigin** (trademark) → **GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED** (company) → **Ghal Bol** (brand) → **Ghal Bol messenger** (product). Package identifiers stay `ghalbol` with no space. Full table: [`docs/GHALBOL_BRAND.md`](docs/GHALBOL_BRAND.md).

### Per-App Environment
Each app under `apps/` has its own `env/` directory:
- `development.env` — dev coord/delivery URLs, debug app ID
- `production.env` — prod URLs, production app ID
- `build.rs` bakes the selected env into the binary at compile time: `GHALBOL_ENV` when set
  (the scripts set it), else the Cargo profile (`--release` → production)
- `GHALBOL_APP_ID` from that file picks the data directory and desktop id
- At startup the process environment wins, then an optional `env/.env.<profile>` in the working
  directory (developer override, gitignored — see [`env/README.md`](env/README.md)), then the baked values

### Development Workflow
```bash
# Desktop development (hot reload + Tweaker via F12)
cargo ghalbol-dev              # → runs apps/ghalbol with --hot --remote

# Windows dev build (cross-compile from Linux with mingw)
cargo ghalbol-windows-dev      # → dist/windows-dev/

# Android dev APK
cargo ghalbol-android-dev

# Production Windows package (Setup.bat per-user install)
./scripts/package-windows.sh  # → dist/windows/

# Production Linux release
./scripts/package_linux_release.sh

# Direct Cargo run (legacy, still works)
cargo run -p ghalbol_app
```

## Adding a New Product

When adding a new communication product under the Ghal Bol brand:

1. **Create `apps/<product>/`** with its own `Cargo.toml`, `src/`, `resources/`, `env/`, `scripts/`
2. **Create `tools/<product>-dev/` and `tools/<product>-windows-dev/`** as thin wrapper binaries
3. **Add members to workspace Cargo.toml** (`apps/<product>`, `tools/<product>-dev`, `tools/<product>-windows-dev`)
4. **Follow the per-app env pattern**: `env/development.env` + `env/production.env`
5. **Per-app scripts**: `dev.sh`, `windows-dev.sh`, `android-dev.sh`

See [`docs/GHALBOL_BRAND.md`](docs/GHALBOL_BRAND.md) for brand doctrine and naming conventions.

---

**Current product:** Ghal Bol messenger — E2EE text plus voice/video calls (public app name Ghal Bol)
**Infrastructure shared across products:** ghalbol_coord, ghalbol_delivery, ghalbol_ddns
