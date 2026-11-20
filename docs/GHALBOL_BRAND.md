# Ghal Bol — brand

**Order is always Trademark → Company → Brand → Product.**

| Word | What it is | Not |
| --- | --- | --- |
| **Gotigin** | **Trademark** / house mark. Pronounced *go-tee-geen*; Memon for *find it*. Slogan: *Where Technology Meets Discovery*. | Not a product name |
| **GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED** | Legal **company** (CIN U62011GJ2026PTC179660). Contact [mearaj@gotigin.com](mailto:mearaj@gotigin.com). Site [gotigin.com](https://gotigin.com). | Not interchangeable with the brand in product UI |
| **Ghal Bol** | Communication **brand**. Site [ghalbol.com](https://ghalbol.com). The name means communication. | Not spelled **GhalBol** |
| **Ghal Bol messenger** | Shipping **product** (`com.ghalbol`, crate `ghalbol_app` under `apps/ghalbol/`). Public app name is **Ghal Bol**. | Not a second brand |

Identifiers stay one word with no space: domain `ghalbol.com`, crates `ghalbol_*`, app id `com.ghalbol`, URI scheme `ghalbol://`. Those are package and protocol strings. Do not present camel-case **GhalBol** as a brand.

Sibling brands (Kumkaj, Wadhu) keep their own product names and privacy URLs.

## Products under the Ghal Bol brand

| Product | Type | Status | Description |
|---------|------|--------|-------------|
| **Ghal Bol messenger** | E2EE messenger | Shipping | Voice/video calls + text via delivery mailbox. Local transcripts, device-owned identity. Public name **Ghal Bol**. See [`apps/ghalbol/`](../apps/ghalbol/) |

## Architecture doctrine

Same workspace pattern as Kumkaj: **brand + independent products**, with **copy-then-diverge** for UI.

| Layer | Shared? | Notes |
|-------|---------|--------|
| Workspace / Makepad / packaging scripts | Yes | One Cargo workspace |
| `ghalbol_core` + coord/delivery/ddns | Yes | Communication **product logic** and infra — intentional (Kumkaj deliberately has no shared product crate; Ghal Bol does, because crypto/delivery/calls are the shared communication stack) |
| App UI under `apps/<product>/` | No | Each product owns Splash faces, resources, env, scripts. Reuse faces by copying lessons from siblings / Kumkaj docs — not a shared UI crate |

- Each product under `apps/<product>/` owns all of its app/UI code independently.
- UI lessons (intent→face, Overlay selection, 720 chrome) live in [`MAKEPAD_UI.md`](MAKEPAD_UI.md) and [`.cursor/rules/makepad-layout-faces.mdc`](../.cursor/rules/makepad-layout-faces.mdc).

## Naming convention

| Type | Convention | Example |
|------|-----------|---------|
| Trademark | Gotigin | gotigin.com |
| Company | Legal name, full capitals as registered | GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED |
| Brand | Two words, title case | Ghal Bol, ghalbol.com |
| Product name | Brand name in the app; “messenger” when disambiguating | Ghal Bol, `com.ghalbol` |
| Package names | snake_case or kebab-case, no space | `ghalbol_app`, `ghalbol_core` |
| Directory names | snake_case or kebab-case | `apps/ghalbol/`, `ghalbol_core/` |
| App IDs (production) | `com.ghalbol` | `com.ghalbol` |
| App IDs (development) | `com.ghalbol.debug` | `com.ghalbol.debug` |

Do not rename protocol strings, Noise info, on-disk folder names, or the `ghalbol://` scheme to insert a space or a camel-case brand. See `AGENTS.md`.

## Workspace layout

```
ghalbol/                    # Brand workspace (Ghal Bol)
├── Cargo.toml              # Workspace root
├── README.md               # Brand overview
├── docs/                   # Global brand docs
│   ├── GHALBOL_BRAND.md    # This file — names and layout
│   ├── DESIGN.md           # Architecture design
│   ├── TRANSPORT.md        # Transport policy
│   └── ...                 # Other global docs
├── ghalbol_core/          # Shared core library (product logic)
├── ghalbol_coord/         # Coordination server (shared infra)
├── ghalbol_delivery/      # Delivery server (shared infra)
├── ghalbol_ddns/          # DDNS helper (shared infra)
├── apps/                   # Products: apps/ghalbol/ (Ghal Bol messenger) today
│   └── ghalbol/            # App crate `ghalbol_app`: src/, resources/, env/, scripts/
├── tools/                  # Per-product dev tooling: tools/<product>-dev, tools/<product>-windows-dev
├── dist/                   # Build outputs: dist/windows/, dist/windows-dev/ (Linux tarball → sites/ghalbol/static/downloads/)
├── sites/                  # Public websites (ghalbol.com under sites/ghalbol/)
└── scripts/                # Shared build/packaging tooling
```

## Visual identity (Ghal Bol messenger)

Palette is sampled from [`docs/assets/app_icon.png`](assets/app_icon.png): cool shell `#EEF2F8`, electric blue primary `#0183FB`, navy chrome `#10236D`. Documented in [`MAKEPAD_UI.md`](MAKEPAD_UI.md) § Palette. Future products may diverge (Kumkaj lesson: each product gets the design that serves *its* users).

## Future products

Future communication products under the Ghal Bol brand will live in `apps/<product>/` following the same structure as `apps/ghalbol/`. Each product gets its own:
- App crate with Keystore ID
- Environment files (development.env, production.env)
- Resource bundles (fonts, icons, optional `resources/nav/` marks)
- Per-app scripts (dev.sh, android-dev.sh, windows-dev.sh)
- Tools entry (`tools/<product>-dev`, `tools/<product>-windows-dev`)
- Its own Splash face copy (intent→face vocabulary recommended; palette may differ)
