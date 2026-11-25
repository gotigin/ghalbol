#!/usr/bin/env bash
# ghalbol.com packs: every catalogue table has every en.tsv key, in order.
# Hindi and Urdu must be real translations (not English, not @@EN@@).
# data-i18n / siteT / T keys in the static site must exist in en.tsv,
# and the inline English must match the source row.
set -euo pipefail

SITE="$(cd "$(dirname "$0")/.." && pwd)"
python3 - "$SITE" <<'PY'
import html, re, sys
from pathlib import Path

site = Path(sys.argv[1])
en_path = site / "i18n" / "en.tsv"
lang_js = site / "static" / "js" / "lang-nav.js"
packs = site / "translations"
problems = []

def load(path):
    rows = []
    text = path.read_text(encoding="utf-8")
    if text.startswith("\ufeff"):
        problems.append(f"{path.name}: BOM")
    for line in text.splitlines():
        if not line or line.startswith("#"):
            continue
        if "\t" not in line:
            problems.append(f"{path.name}: no tab: {line[:40]!r}")
            continue
        key, value = line.split("\t", 1)
        rows.append((key.strip(), value))
    return rows

en_rows = load(en_path)
en = dict(en_rows)
if len(en) != len(en_rows):
    problems.append("en.tsv: duplicate keys")
keys = [k for k, _ in en_rows]

text = lang_js.read_text(encoding="utf-8")
codes = re.findall(r'\{"code": "([^"]+)"', text)
skip = {"en", "pt-BR", "es-MX", "yue", "gom"}
catalogue = []
for code in codes:
    if code in skip or code in catalogue:
        continue
    catalogue.append(code)
if len(catalogue) != 110:
    problems.append(f"lang-nav.js: expected 110 pack codes, found {len(catalogue)}")

ph = re.compile(r"\{[a-z_]+\}")
for code in catalogue:
    path = packs / f"{code}.tsv"
    if not path.is_file():
        problems.append(f"{code}: missing pack")
        continue
    rows = load(path)
    got = [k for k, _ in rows]
    if got != keys:
        missing = [k for k in keys if k not in dict(rows)]
        extra = [k for k in got if k not in en]
        problems.append(f"{code}: key order/set mismatch missing={missing[:5]} extra={extra[:5]}")
        continue
    for key, value in rows:
        if value.startswith("@@EN@@"):
            problems.append(f"{code}/{key}: @@EN@@ stand-in")
        if ph.findall(value) != ph.findall(en[key]):
            problems.append(f"{code}/{key}: placeholders {ph.findall(value)} != {ph.findall(en[key])}")
        if code in ("hi", "ur") and value == en[key]:
            problems.append(f"{code}/{key}: still English")

# Shared chrome copies stay identical.
shared = site.parent / "_shared" / "chrome"
for name in ("lang-nav.js", "site-i18n.js"):
    a = shared / name
    b = site / "static" / "js" / name
    if not a.is_file() or not b.is_file():
        problems.append(f"missing chrome copy {name}")
    elif a.read_bytes() != b.read_bytes():
        problems.append(f"{name}: sites/_shared/chrome and static/js differ")

attr_key = re.compile(
    r"data-i18n(?:-(placeholder|aria-label|title|alt))?=\"([a-z0-9_]+)\""
)
js_call = re.compile(
    r"(?:siteT|\bT)\(\s*\"([a-z0-9_]+)\"\s*,\s*\"((?:\\.|[^\"\\])*)\"\s*\)"
)

def unesc_js(s):
    out = []
    i = 0
    while i < len(s):
        if s[i] == "\\" and i + 1 < len(s):
            n = s[i + 1]
            out.append({"n": "\n", "t": "\t", "\\": "\\", '"': '"'}.get(n, n))
            i += 2
        else:
            out.append(s[i])
            i += 1
    return "".join(out)

sources = [p for p in (site / "static").rglob("*.html")]
sources += [
    site / "client" / "account-app.js",
    site / "static" / "js" / "nav-menu.js",
    site / "static" / "js" / "auth-nav.js",
]
for path in sources:
    raw = path.read_text(encoding="utf-8")
    # Tag text for data-i18n="key">english<
    for m in re.finditer(
        r"data-i18n=\"([a-z0-9_]+)\"([^>]*)>([^<]*)",
        raw,
    ):
        key, _rest, inner = m.group(1), m.group(2), m.group(3)
        if key not in en:
            problems.append(f"{path.name}: unknown data-i18n {key}")
            continue
        got = html.unescape(inner).strip()
        if got and got != en[key]:
            problems.append(f"{path.name}:{key}: html {got!r} != en {en[key]!r}")
    for m in re.finditer(r"<[^>]+>", raw):
        tag = m.group(0)
        for kind, key in attr_key.findall(tag):
            if not kind:
                continue
            if key not in en:
                problems.append(f"{path.name}: unknown data-i18n-{kind} {key}")
                continue
            attr = {"placeholder": "placeholder", "aria-label": "aria-label", "title": "title", "alt": "alt"}[kind]
            am = re.search(attr + r"=\"([^\"]*)\"", tag)
            if not am:
                problems.append(f"{path.name}:{key}: missing {attr}")
                continue
            got = html.unescape(am.group(1))
            if got != en[key]:
                problems.append(f"{path.name}:{key}: {attr} {got!r} != en {en[key]!r}")
    if path.suffix == ".js" or path.name == "index.html":
        for key, literal in js_call.findall(raw):
            if key not in en:
                problems.append(f"{path.name}: unknown siteT {key}")
                continue
            got = unesc_js(literal)
            if got != en[key]:
                problems.append(f"{path.name}:{key}: js {got!r} != en {en[key]!r}")

if problems:
    print("\n".join(problems), file=sys.stderr)
    sys.exit(1)
print(f"ok: {len(keys)} keys, {len(catalogue)} packs, hi/ur translated")
PY
