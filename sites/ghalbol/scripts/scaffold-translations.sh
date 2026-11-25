#!/usr/bin/env bash
# Fill sites/ghalbol/translations/<code>.tsv from sites/ghalbol/i18n/en.tsv.
#
# English has no pack. Alias codes read another table and have no pack:
# pt-BR→pt, es-MX→es, yue→zh-Hant, gom→kok (same as site-i18n.js).
# A row that already has text keeps it. A new key copies the English source.
# Hindi and Urdu must be real translations — check-translations.sh rejects
# English and @@EN@@ in those two packs.
#
#   ./sites/ghalbol/scripts/scaffold-translations.sh
set -euo pipefail

SITE="$(cd "$(dirname "$0")/.." && pwd)"
EN="$SITE/i18n/en.tsv"
LANG_JS="$SITE/static/js/lang-nav.js"
OUT="$SITE/translations"

[[ -f "$EN" ]] || { echo "error: missing $EN" >&2; exit 1; }
[[ -f "$LANG_JS" ]] || { echo "error: missing $LANG_JS" >&2; exit 1; }
mkdir -p "$OUT"

mapfile -t CODES < <(python3 - "$LANG_JS" <<'PY'
import re, sys
text = open(sys.argv[1], encoding="utf-8").read()
codes = re.findall(r'\{"code": "([^"]+)"', text)
skip = {"en", "pt-BR", "es-MX", "yue", "gom"}
seen = []
for code in codes:
    if code in skip or code in seen:
        continue
    seen.append(code)
    print(code)
PY
)

if [[ "${#CODES[@]}" -ne 110 ]]; then
  echo "error: expected 110 pack languages from lang-nav.js, found ${#CODES[@]}" >&2
  exit 1
fi

n_keys="$(awk -F'\t' '$0 !~ /^#/ && index($0, "\t") > 0 { n++ } END { print n+0 }' "$EN")"
if [[ "$n_keys" -lt 20 ]]; then
  echo "error: read only $n_keys keys from $EN" >&2
  exit 1
fi

made=0
for code in "${CODES[@]}"; do
  file="$OUT/$code.tsv"
  tmp="$(mktemp)"
  {
    printf '# %s. key<TAB>text in i18n/en.tsv order. Hindi and Urdu are translated; other packs may still be the English source. Keep {placeholders}. \\n = line break.\n' "$code"
    awk -F'\t' -v old="$file" '
      BEGIN {
        if ((getline line < old) > 0) {
          do {
            if (line == "" || substr(line, 1, 1) == "#") continue
            i = index(line, "\t")
            if (i == 0) continue
            have[substr(line, 1, i - 1)] = substr(line, i + 1)
          } while ((getline line < old) > 0)
        }
      }
      {
        if ($0 == "" || substr($0, 1, 1) == "#") next
        i = index($0, "\t")
        if (i == 0) next
        key = substr($0, 1, i - 1)
        english = substr($0, i + 1)
        sub(/\r$/, "", english)
        if (key in have && have[key] != "") print key "\t" have[key]
        else print key "\t" english
      }
    ' "$EN"
  } > "$tmp"
  [[ -f "$file" ]] || made=$((made + 1))
  mv "$tmp" "$file"
done

echo "ghalbol: $n_keys keys; packs created $made; dir $OUT"
