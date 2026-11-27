//! Slim site i18n for ghalbol.com on Makepad rev 1f60033.
//! Packs: `/i18n/<code>.tsv`. Lang chip bridge: `#l=<code>` from lang-nav.js.

#[cfg(target_arch = "wasm32")]
use makepad_widgets::LiveId;
use std::collections::HashMap;

#[path = "languages.rs"]
mod languages;
pub use languages::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScriptGroup {
    Latin,
    LatinExt,
    Cyrillic,
    Greek,
    Devanagari,
    Bengali,
    Gurmukhi,
    Gujarati,
    Oriya,
    Tamil,
    Telugu,
    Kannada,
    Malayalam,
    Sinhala,
    Arabic,
    Hebrew,
    Thaana,
    Tibetan,
    OlChiki,
    MeeteiMayek,
    Thai,
    Lao,
    Khmer,
    Myanmar,
    Ethiopic,
    Armenian,
    Georgian,
    CjkSc,
    CjkTc,
    CjkJp,
    CjkKr,
}

pub fn is_rtl(code: &str) -> bool {
    matches!(
        code.split(['-', '_']).next().unwrap_or(code),
        "ar" | "ur" | "fa" | "he" | "dv" | "ps" | "sd" | "ks" | "yi" | "ckb" | "ug"
    )
}

pub fn script_group(code: &str) -> ScriptGroup {
    let primary = code.split(['-', '_']).next().unwrap_or(code);
    match primary {
        "ru" | "uk" | "bg" | "sr" | "mk" | "be" | "kk" | "ky" | "tg" | "mn" | "uz" => {
            ScriptGroup::Cyrillic
        }
        "el" => ScriptGroup::Greek,
        "hi" | "mr" | "ne" | "sa" | "mai" | "bho" | "awa" | "mag" | "hne" | "raj" | "kok"
        | "doi" | "brx" => ScriptGroup::Devanagari,
        "bn" | "as" | "bpy" => ScriptGroup::Bengali,
        "pa" => ScriptGroup::Gurmukhi,
        "gu" => ScriptGroup::Gujarati,
        "or" => ScriptGroup::Oriya,
        "ta" => ScriptGroup::Tamil,
        "te" => ScriptGroup::Telugu,
        "kn" | "tcy" => ScriptGroup::Kannada,
        "ml" => ScriptGroup::Malayalam,
        "si" => ScriptGroup::Sinhala,
        "ar" | "ur" | "fa" | "ps" | "sd" | "ks" => ScriptGroup::Arabic,
        "he" | "yi" => ScriptGroup::Hebrew,
        "dv" => ScriptGroup::Thaana,
        "bo" => ScriptGroup::Tibetan,
        "sat" => ScriptGroup::OlChiki,
        "mni" => ScriptGroup::MeeteiMayek,
        "th" => ScriptGroup::Thai,
        "lo" => ScriptGroup::Lao,
        "km" => ScriptGroup::Khmer,
        "my" => ScriptGroup::Myanmar,
        "am" => ScriptGroup::Ethiopic,
        "hy" => ScriptGroup::Armenian,
        "ka" => ScriptGroup::Georgian,
        "zh" => ScriptGroup::CjkSc,
        "zh-Hant" | "yue" => ScriptGroup::CjkTc,
        "ja" => ScriptGroup::CjkJp,
        "ko" => ScriptGroup::CjkKr,
        "vi" | "tr" | "az" | "tk" | "ro" | "pl" | "cs" | "sk" | "hu" | "fi" | "et" | "lv"
        | "lt" | "mt" | "sq" | "hr" | "bs" | "sl" | "ga" | "cy" | "eu" | "ca" | "gl" | "is"
        | "sv" | "no" | "da" | "nl" | "de" | "pt" | "es" | "fr" | "it" | "af" | "sw" | "ha"
        | "yo" | "ig" | "zu" | "xh" | "so" | "rw" | "mg" | "ht" | "qu" | "gn" | "fil" | "id"
        | "ms" | "lus" | "kha" | "trp" => ScriptGroup::LatinExt,
        _ => ScriptGroup::Latin,
    }
}

pub fn lang_from_hash(hash: &str) -> Option<&'static str> {
    for part in hash.trim_start_matches('#').split('&') {
        let part = part.trim();
        if let Some(raw) = part.strip_prefix("l=") {
            let raw = raw.split('&').next().unwrap_or(raw);
            let decoded = urlencoding_decode(raw);
            if let Some(lang) = language_by_code(&decoded) {
                return Some(lang.code);
            }
        }
    }
    None
}

fn urlencoding_decode(s: &str) -> String {
    // Enough for language codes (ASCII / %2D).
    let mut out = String::new();
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let (Some(a), Some(b)) = (from_hex(bytes[i + 1]), from_hex(bytes[i + 2])) {
                out.push((a * 16 + b) as char);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    out
}

fn from_hex(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

#[cfg(target_arch = "wasm32")]
pub fn pack_web_path(table: &str) -> String {
    format!("./i18n/{table}.tsv")
}

#[derive(Default)]
pub struct SiteLang {
    pub wanted: String,
    pub last_hash_lang: Option<String>,
    packs: HashMap<String, HashMap<String, String>>,
    #[cfg(target_arch = "wasm32")]
    pending: Option<(LiveId, String)>,
}

impl SiteLang {
    pub fn want(&mut self, code: &str) -> Option<String> {
        self.wanted = code.to_string();
        let table = table_language(code);
        if table == "en" {
            self.packs
                .entry("en".into())
                .or_insert_with(HashMap::new);
            return None;
        }
        if self.packs.contains_key(table) {
            return None;
        }
        Some(table.to_string())
    }

    pub fn has_pack(&self, code: &str) -> bool {
        let table = table_language(code);
        table == "en" || self.packs.contains_key(table)
    }

    pub fn insert_pack(&mut self, table: &str, text: &str) {
        let mut map = HashMap::new();
        for line in text.lines() {
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((k, v)) = line.split_once('\t') {
                map.insert(k.to_string(), v.to_string());
            }
        }
        self.packs.insert(table.to_string(), map);
    }

    #[cfg(target_arch = "wasm32")]
    pub fn set_pending(&mut self, id: LiveId, table: String) {
        self.pending = Some((id, table));
    }

    #[cfg(target_arch = "wasm32")]
    pub fn take_response(&mut self, request_id: LiveId, body: Option<&[u8]>) -> bool {
        let Some((id, table)) = self.pending.take() else {
            return false;
        };
        if id != request_id {
            self.pending = Some((id, table));
            return false;
        }
        if let Some(bytes) = body {
            if let Ok(text) = std::str::from_utf8(bytes) {
                self.insert_pack(&table, text);
                return true;
            }
        }
        false
    }

    pub fn text(&self, key: &str, english: &str) -> String {
        if self.wanted.is_empty() || self.wanted == "en" {
            return english.to_string();
        }
        let table = table_language(&self.wanted);
        let out = self
            .packs
            .get(table)
            .and_then(|m| m.get(key))
            .cloned()
            .unwrap_or_else(|| english.to_string());
        if is_rtl(&self.wanted) {
            rtl_marks(&out)
        } else {
            out
        }
    }
}

const RLM: char = '\u{200F}';

fn rtl_marks(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 3 * (1 + text.matches('\n').count()));
    for (i, line) in text.split('\n').enumerate() {
        if i > 0 {
            out.push('\n');
        }
        if !line.trim().is_empty() && !line.starts_with(RLM) {
            out.push(RLM);
        }
        out.push_str(line);
    }
    out
}
