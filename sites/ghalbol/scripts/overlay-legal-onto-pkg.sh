#!/usr/bin/env bash
# Overlay same-site pages (legal HTML + css/js + connect/linux) onto the Makepad
# wasm package. static/ must not contain a competing index.html.
#
#   ./sites/ghalbol/scripts/overlay-legal-onto-pkg.sh <makepad-wasm-pkg-dir>
set -euo pipefail
SITE="$(cd "$(dirname "$0")/.." && pwd)"
PKG="${1:-}"
if [[ -z "$PKG" || ! -d "$PKG" ]]; then
  echo "usage: $0 <makepad-wasm-pkg-dir>" >&2
  exit 1
fi
if [[ ! -f "$PKG/index.html" ]]; then
  echo "error: missing $PKG/index.html (Makepad home)" >&2
  exit 1
fi
if [[ -f "$SITE/static/index.html" ]]; then
  echo "error: $SITE/static/index.html must not exist — home is the Makepad package only" >&2
  exit 1
fi

rsync -a "$SITE/static/" "$PKG/"

if [[ ! -f "$PKG/privacy/index.html" ]]; then
  echo "error: privacy HTML missing after overlay" >&2
  exit 1
fi
if [[ ! -f "$PKG/connect/index.html" || ! -f "$PKG/download/linux/index.html" ]]; then
  echo "error: connect or download/linux HTML missing after overlay" >&2
  exit 1
fi
if ! grep -qE 'ghalbol-site(\.[0-9a-f]+)?\.wasm|web_gl|full_canvas|makepad' "$PKG/index.html"; then
  echo "error: Makepad index.html was overwritten" >&2
  exit 1
fi

if grep -q '<html>' "$PKG/index.html" && ! grep -q '<html lang=' "$PKG/index.html"; then
  sed -i 's/<html>/<html lang="en">/' "$PKG/index.html"
fi

PREVIEW="$SITE/static/media/link-preview.jpg"
if [[ ! -f "$PREVIEW" ]]; then
  echo "error: missing $PREVIEW — run sites/ghalbol/scripts/make-link-preview.sh" >&2
  exit 1
fi
PREVIEW_URL="https://ghalbol.com/media/link-preview.jpg?v=$(sha256sum "$PREVIEW" | cut -c1-10)"
if ! grep -q 'og:site_name' "$PKG/index.html"; then
  HEAD_INJECT='        <title>Ghal Bol — encrypted messenger</title>
        <meta name="description" content="Ghal Bol is the communication brand of GOTIGIN SOFTWARE &amp; HARDWARE PRIVATE LIMITED. Encrypted 1:1 chat and voice/video calls, no phone number, device-owned identity. Gotigin is the trademark." />
        <link rel="canonical" href="https://ghalbol.com/" />
        <meta name="theme-color" content="#eef2f8" />
        <meta name="robots" content="max-image-preview:large" />
        <link rel="icon" href="./favicon.png" type="image/png" />
        <link rel="apple-touch-icon" href="./media/apple-touch-icon.png" />
        <meta property="og:site_name" content="Ghal Bol" />
        <meta property="og:title" content="Ghal Bol" />
        <meta property="og:description" content="Encrypted messenger under the Ghal Bol brand. Trademark Gotigin. Company GOTIGIN SOFTWARE &amp; HARDWARE PRIVATE LIMITED." />
        <meta property="og:url" content="https://ghalbol.com/" />
        <meta property="og:type" content="website" />
        <meta property="og:locale" content="en_IN" />
        <meta property="og:image" content="'"$PREVIEW_URL"'" />
        <meta property="og:image:type" content="image/jpeg" />
        <meta property="og:image:width" content="1200" />
        <meta property="og:image:height" content="630" />
        <meta property="og:image:alt" content="The ghalbol.com home page: Encrypted chat you own." />
        <meta name="twitter:card" content="summary_large_image" />
        <meta name="twitter:title" content="Ghal Bol — encrypted messenger" />
        <meta name="twitter:description" content="Encrypted messenger under the Ghal Bol brand. Trademark Gotigin. Company GOTIGIN SOFTWARE &amp; HARDWARE PRIVATE LIMITED." />
        <meta name="twitter:image" content="'"$PREVIEW_URL"'" />
        <meta name="twitter:image:alt" content="The ghalbol.com home page: Encrypted chat you own." />
        <script type="application/ld+json">{"@context":"https://schema.org","@type":"SoftwareApplication","name":"Ghal Bol","applicationCategory":"CommunicationApplication","operatingSystem":"Android, Linux","url":"https://ghalbol.com/","image":"https://ghalbol.com/media/link-preview.jpg","description":"Encrypted 1:1 messenger under the Ghal Bol brand. Device-owned identity, no phone number.","offers":{"@type":"Offer","price":"0","priceCurrency":"USD"},"author":{"@type":"Organization","name":"Gotigin","legalName":"GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED","url":"https://gotigin.com/"},"downloadUrl":"https://ghalbol.com/downloads/ghalbol-linux-x64.tar.gz","installUrl":"https://play.google.com/store/apps/details?id=com.ghalbol"}</script>'
  awk -v head="$HEAD_INJECT" '
    /<title>/ && !titled {
      print head
      titled = 1
      next
    }
    { print }
    END { if (!titled) exit 2 }
  ' "$PKG/index.html" > "$PKG/index.html.tmp" && mv "$PKG/index.html.tmp" "$PKG/index.html"
fi

"$SITE/scripts/patch-web-shell.sh" "$PKG/index.html"

if ! grep -q '<noscript>' "$PKG/index.html"; then
  awk '
    /<\/body>/ && !done {
      print "        <noscript>"
      print "          <h1>Ghal Bol — encrypted messenger</h1>"
      print "          <p>Encrypted 1:1 chat and voice/video calls. Device-owned identity. No phone number. Trademark Gotigin · GOTIGIN SOFTWARE &amp; HARDWARE PRIVATE LIMITED.</p>"
      print "          <p><a href=\"/privacy\">Privacy</a> · <a href=\"/terms\">Terms</a> · <a href=\"/support\">Support</a> · <a href=\"/about\">About</a> · <a href=\"/account\">Account</a> · <a href=\"/download/linux\">Linux</a>.</p>"
      print "          <p>Parent: <a href=\"https://gotigin.com/\">gotigin.com</a>.</p>"
      print "        </noscript>"
      done = 1
    }
    { print }
    END { if (!done) exit 2 }
  ' "$PKG/index.html" > "$PKG/index.html.tmp" && mv "$PKG/index.html.tmp" "$PKG/index.html"
fi

echo "Overlayed same-site pages onto $PKG (Makepad index preserved)"
