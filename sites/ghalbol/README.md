# ghalbol.com — static site + Firebase Analytics

Package **`ghalbol-website`**. Same Hosting layout as Kumkaj/Wadhu sites
(`sites/<name>/`), without a Makepad wasm SPA: invite handoff, Linux download,
privacy, and App Links.

```text
static/                 ← source tree (edit here)
  index.html            ← brand landing + SPA routes (/connect, /privacy, /terms, /support, /data-deletion, /about, /download/linux)
                          (WebSite + SoftwareApplication JSON-LD in <head>)
  robots.txt / sitemap.xml
  media/                ← what the page loads: icon.webp, feature.webp; link-preview.jpg
                          (icon.png, feature.png, app_icon.png are the full-size art, kept for links)
  favicon.png
  .well-known/assetlinks.json
  downloads/            ← Linux tarball before deploy (gitignored .tar.gz)
  js/firebase-init.js   ← built by npm (gitignored); from client/
firebase.json           ← Hosting rewrites + safe headers (nosniff, referrer, permissions, frame, CSP)

client/firebase-init.js ← Firebase web app config (Analytics)
public/                 ← staged Hosting root (gitignored)
```

Chat runs in **`ghalbol_app`** / Android — not in the browser.

## Pictures and link previews

- The page draws `media/icon.webp` (192 px, transparent corners) and
  `media/feature.webp` (1072 px wide). It used to load `icon.png` and the
  1.6 MB `feature.png`: a first visit was 2.05 MB and is now about 0.4 MB.
  Keep new page art at the size it is drawn.
- `media/link-preview.jpg` is the picture chat apps and social sites show
  for a pasted ghalbol.com link: a 1200×630 JPEG of the real first screen,
  named by the Open Graph and Twitter tags in `index.html` (one set serves
  WhatsApp, X, LinkedIn, Facebook, Telegram, Discord, Slack, iMessage). The
  old tag named `feature.png`, too big for WhatsApp to show. After the
  first screen changes: `preview-web.sh` in one terminal, then
  `./sites/ghalbol/scripts/make-link-preview.sh http://127.0.0.1:8080/`,
  commit the picture, deploy. `stage-public.sh` adds a hash of the file to
  its address, because those apps keep a preview for days.

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
