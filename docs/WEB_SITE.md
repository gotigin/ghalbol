# Ghal Bol web (ghalbol.com)

Public site at **https://ghalbol.com** and **https://www.ghalbol.com** (Firebase
Hosting). Marketing home, **Linux desktop download**, **privacy**, and **invite
handoff** for `/connect/…` links. Chat runs in the desktop app (`ghalbol_app`)
or the Android app — not in the browser.

## Languages + countries (hard rule)

Same Gotigin product floor: every Ghal Bol **app and website** supports the
full language catalogue (**115+ languages**) and **199 countries**.
ghalbol.com is static HTML (no Makepad wasm home). Marketing home, Linux
download, invite handoff, account, 404, and the shared nav/footer translate
via `lang-nav.js` (`#l=`) then `site-i18n.js`. Privacy, terms, support,
data deletion, cancellation, and about **bodies** stay English. See
`.cursor/rules/languages-countries-floor.mdc`.

| Piece | Where |
| --- | --- |
| English source | `sites/ghalbol/i18n/en.tsv` (`key<TAB>english`) |
| Packs | `sites/ghalbol/translations/<code>.tsv` — 110 files; Hindi and Urdu are real |
| Served | `/i18n/<code>.tsv` after `stage-public.sh` (English has no pack) |
| Scripts | `sites/_shared/chrome/` kept identical to `static/js/lang-nav.js` and `site-i18n.js` |
| Fonts | `static/fonts/NotoSansDevanagari-Regular.ttf`, `NotoSansArabic-Regular.ttf` (`static/css/site-fonts.css`) |

Alias codes have no pack of their own: `pt-BR` reads `pt`, `es-MX` reads `es`, `yue` reads `zh-Hant`, `gom` reads `kok`. Other packs start as the English source until someone translates them. New keys: edit `i18n/en.tsv`, translate `hi.tsv` and `ur.tsv`, then:

```bash
./sites/ghalbol/scripts/scaffold-translations.sh
./sites/ghalbol/scripts/check-translations.sh
```

### Verify locally (no Hosting upload)

```bash
./sites/ghalbol/scripts/stage-public.sh
python3 -m http.server 8765 --directory sites/ghalbol/public
curl -s -o /dev/null -w '%{http_code}\n' http://127.0.0.1:8765/i18n/hi.tsv   # 200
curl -s -o /dev/null -w '%{http_code}\n' http://127.0.0.1:8765/i18n/ur.tsv   # 200
```

Open `http://127.0.0.1:8765/#l=hi` and `http://127.0.0.1:8765/account#l=ur`.
Hindi must paint in Devanagari and Urdu in Arabic script. `#l=` wins over the
stored choice. Legal pages (`/privacy`) keep English bodies; the nav can change.

Layout follows Kumkaj/Wadhu: everything lives under **`sites/ghalbol/`**
(package `ghalbol-website`). See [`sites/README.md`](../sites/README.md) and
[`sites/ghalbol/README.md`](../sites/ghalbol/README.md).

## Architecture

| Piece | Location |
|-------|----------|
| Pages | `sites/ghalbol/static/index.html` — brand landing + SPA routes (`/connect`, `/privacy`, `/download/linux`) |
| Media | `sites/ghalbol/static/media/` — app icon + feature graphic (from `apps/ghalbol/resources` / `docs/assets`) |
| Firebase JS | `sites/ghalbol/client/firebase-init.js` → `static/js/firebase-init.js` (esbuild) |
| Hosting config | `sites/ghalbol/firebase.json` → staged `public/` |
| Project | `sites/ghalbol/.firebaserc` → **`ghalbol`** |
| Android App Links | `sites/ghalbol/static/.well-known/assetlinks.json` |
| Linux bundle | `sites/ghalbol/static/downloads/ghalbol-linux-x64.tar.gz` before deploy |
| Crawl | `sites/ghalbol/static/robots.txt`, `sitemap.xml` (home, download/linux, legal pages) |
| JSON-LD | `WebSite` + `SoftwareApplication` in `static/index.html` |
| Safe headers | `firebase.json`: nosniff, referrer, permissions, frame, CSP (static HTML + Analytics) |

Home page structure mirrors Kumkaj/Wadhu brand sites (topbar + hero + product bands + platforms + footer), as static HTML with the Ghal Bol palette from [MAKEPAD_UI.md](MAKEPAD_UI.md) § Palette — not an empty download stub.

Staging copies `static/` → `public/` and injects a cache-busted Analytics
script. Firebase serves real files first (tarball, `assetlinks.json`); only the client
routes `/connect`, `/connect/…` and `/download/…` rewrite to `index.html`. Any other
unknown path gets `404.html` with a real 404 status (until 2026-11-22 it answered
with the home page, a soft 404 that search engines index as a duplicate).

**Do not** add `**/.*` to Hosting `ignore` — that would drop `.well-known/` and
break App Links verification.

## Routes

| URL | Page |
|-----|------|
| `/` | Home — Play Store + **Download for Linux** |
| `/download/linux` | Linux instructions + link to the tarball |
| `/downloads/ghalbol-linux-x64.tar.gz` | Static file (when present under `static/downloads/`) |
| `/connect/<public-key-hex>` | Invite handoff — `ghalbol://connect/…`, copy web / app link |
| `/privacy` | Privacy policy (keep in sync with [PRIVACY_POLICY.md](PRIVACY_POLICY.md)) |
| `/terms` | Terms ([legal/TERMS.md](legal/TERMS.md)) |
| `/support` | Support URL for Play and Microsoft Store |
| `/data-deletion` | Data deletion instructions |
| `/cancellation` | No website subscription; store / local uninstall paths |
| `/about` | Trademark, company, brand |
| `/robots.txt` | Crawl rules + sitemap pointer |
| `/sitemap.xml` | Home, download/linux, legal pages |
| `/.well-known/assetlinks.json` | Digital Asset Links for verified HTTPS → app |

Optional query on invites: `?alias=Name` only. The public key is 64 or 66 hex
characters.

## Local preview (preferred before deploy)

```bash
./sites/ghalbol/scripts/preview-web.sh
```

Serves the staged `public/` tree on **http://127.0.0.1:8080/** with the same
SPA fallback Hosting uses (`/connect/…` → `index.html`).

| Page | Example URL |
|------|-------------|
| Home | `http://127.0.0.1:8080/` |
| Linux download | `http://127.0.0.1:8080/download/linux` |
| Invite | `http://127.0.0.1:8080/connect/<hex>?alias=Name` |

`./sites/ghalbol/scripts/preview-web.sh --no-build` re-serves the last stage.

## Linux desktop bundle

```bash
./scripts/package_linux_release.sh
```

Writes `sites/ghalbol/static/downloads/ghalbol-linux-x64.tar.gz` (gitignored).
Stage/deploy pick it up from `static/`.

## Firebase Hosting

### One-time setup

1. [Firebase console](https://console.firebase.google.com/) — project **`ghalbol`** + **Hosting**.
2. Custom domains **ghalbol.com** and **www.ghalbol.com**. Prefer a Hosting
   **www → apex** redirect in the Firebase domain console (not expressible in
   `firebase.json` alone); live www currently answers 200 without redirect.
3. Web app registered (Analytics config is in `client/firebase-init.js`).
4. Locally:

```bash
npm install -g firebase-tools
firebase login
cd sites/ghalbol && npm install
```

### Deploy

```bash
./sites/ghalbol/scripts/deploy-firebase.sh
./sites/ghalbol/scripts/deploy-firebase.sh --dry-run
```

(Wrapper still works: `./scripts/deploy_web_firebase.sh`.)

### Pre-deploy checklist

| Item | Action |
|------|--------|
| Local preview | `./sites/ghalbol/scripts/preview-web.sh` looks right |
| `assetlinks.json` | Play app-signing SHA-256 in `sites/ghalbol/static/.well-known/assetlinks.json` |
| Linux tarball | Present under `static/downloads/` if the download button should work |
| Verify live | `/`, `/download/linux`, `/privacy`, `/terms`, `/support`, `/data-deletion`, `/cancellation`, `/about`, `/robots.txt`, `/sitemap.xml`, the tarball URL, `/.well-known/assetlinks.json` |

## Analytics

The **website** loads Firebase Analytics (Measurement ID in
`client/firebase-init.js`). The **core app** does not ship advertising or
third-party analytics SDKs — see [PRIVACY_POLICY.md](PRIVACY_POLICY.md).

## Invite link behaviour

| Situation | What happens |
|-----------|----------------|
| **Android + app installed + App Links verified** | `https://ghalbol.com/connect/…` opens **Ghal Bol** directly |
| **Android + app installed, verification pending** | Browser shows this page; **Open in Ghal Bol** is a real `<a href="ghalbol://connect/…">` |
| **No app installed** | Web page links to Play Store or the Linux download |
| **Desktop browser** | Same invite page. If Ghal Bol is installed, `ghalbol://connect/…` opens the app and joins after unlock. Otherwise paste the link in **Join**. |

**Open in Ghal Bol** is an HTML link. Do not navigate to `ghalbol://` from
script alone; Chrome blocks that unless the user taps the link.

Details: [ANDROID_APP_LINKS.md](ANDROID_APP_LINKS.md), [GHALBOL_URI_SCHEME.md](GHALBOL_URI_SCHEME.md).

Play listing: `https://play.google.com/store/apps/details?id=com.ghalbol`.
