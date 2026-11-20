# ghalbol.com — static site + Firebase Analytics

Package **`ghalbol-website`**. Same Hosting layout as Kumkaj/Wadhu sites
(`sites/<name>/`), without a Makepad wasm SPA: invite handoff, Linux download,
privacy, and App Links.

```text
static/                 ← source tree (edit here)
  index.html            ← brand landing + SPA routes (/connect, /privacy, /terms, /support, /data-deletion, /about, /download/linux)
  media/                ← icon.png, feature.png (app art)
  .well-known/assetlinks.json
  downloads/            ← Linux tarball before deploy (gitignored .tar.gz)
  js/firebase-init.js   ← built by npm (gitignored); from client/

client/firebase-init.js ← Firebase web app config (Analytics)
public/                 ← staged Hosting root (gitignored)
```

Chat runs in **`ghalbol_app`** / Android — not in the browser.

## Firebase

| Item | Value |
| --- | --- |
| Project | **`ghalbol`** (`.firebaserc`) |
| Hosting site | `ghalbol` → https://ghalbol.web.app |
| Custom domains | **ghalbol.com**, **www.ghalbol.com** |
| Web app | Analytics (`client/firebase-init.js`) |

Config comes from the Firebase console Web app. Do **not** put API secrets in
the Makepad app — Hosting-only + client Analytics is intentional.

## Local preview (before deploy)

```bash
./sites/ghalbol/scripts/preview-web.sh
# http://127.0.0.1:8080/
```

| Page | Example |
| --- | --- |
| Home | http://127.0.0.1:8080/ |
| Linux | http://127.0.0.1:8080/download/linux |
| Privacy | http://127.0.0.1:8080/privacy |
| Terms | http://127.0.0.1:8080/terms |
| Support | http://127.0.0.1:8080/support |
| Data deletion | http://127.0.0.1:8080/data-deletion |
| About | http://127.0.0.1:8080/about |
| Invite | http://127.0.0.1:8080/connect/\<64-or-66-hex\>?alias=Name |

Re-serve last stage: `./sites/ghalbol/scripts/preview-web.sh --no-build`.

## Deploy (when ready)

```bash
./scripts/package_linux_release.sh          # optional: fills static/downloads/
./sites/ghalbol/scripts/deploy-firebase.sh
./sites/ghalbol/scripts/deploy-firebase.sh --dry-run
```

One-time: `npm i -g firebase-tools && firebase login`.

## Routes

| URL | Page |
| --- | --- |
| `/` | Home — Play Store + Linux download |
| `/download/linux` | Linux instructions + tarball link |
| `/downloads/ghalbol-linux-x64.tar.gz` | Static file when present |
| `/connect/<public-key-hex>` | Invite handoff (`ghalbol://`, copy links) |
| `/privacy` | Privacy (mirrors `docs/PRIVACY_POLICY.md`) |
| `/terms` | Terms (`docs/legal/TERMS.md`) |
| `/support` | Support (`docs/legal/SUPPORT.md`) |
| `/data-deletion` | Data deletion (`docs/legal/DATA_DELETION.md`) |
| `/about` | About (`docs/legal/ABOUT.md`) |
| `/.well-known/assetlinks.json` | Android App Links |

Details: [`docs/WEB_SITE.md`](../../docs/WEB_SITE.md),
[`docs/ANDROID_APP_LINKS.md`](../../docs/ANDROID_APP_LINKS.md).
