//! ghalbol.com — Makepad scrolling brand SPA (Wadhu pattern).
//!
//! One Hosting tree: this wasm home + legal/account HTML + thin connect/linux
//! HTML. Legal is not an in-app route; nav opens those pages with open_site.

pub use makepad_widgets;

use makepad_widgets::*;

mod locale;

#[path = "../../_shared/i18n/site_i18n.rs"]
mod site_i18n;

use site_i18n::{ScriptGroup, SiteLang};

// Makepad 1f60033 wasm allowlists fonts via this section. Site `self:resources/fonts/…`
// faces (Ui* in ui.rs) and the theme IBMPlex weights we override onto must be listed or
// cargo-makepad strips them and text draws as empty boxes.
app_main!(
    App,
    font_assets: [
        "ghalbol_site/resources/fonts/IBMPlexSans-Text.ttf",
        "ghalbol_site/resources/fonts/IBMPlexSans-SemiBold.ttf",
        "ghalbol_site/resources/fonts/NotoSans-Regular.ttf",
        "ghalbol_site/resources/fonts/NotoSansDevanagari-Regular.ttf",
        "ghalbol_site/resources/fonts/NotoSansBengali-Regular.ttf",
        "ghalbol_site/resources/fonts/NotoSansGurmukhi-Regular.ttf",
        "ghalbol_site/resources/fonts/NotoSansGujarati-Regular.ttf",
        "ghalbol_site/resources/fonts/NotoSansOriya-Regular.ttf",
        "ghalbol_site/resources/fonts/NotoSansTamil-Regular.ttf",
        "ghalbol_site/resources/fonts/NotoSansTelugu-Regular.ttf",
        "ghalbol_site/resources/fonts/NotoSansKannada-Regular.ttf",
        "ghalbol_site/resources/fonts/NotoSansMalayalam-Regular.ttf",
        "ghalbol_site/resources/fonts/NotoSansSinhala-Regular.ttf",
        "ghalbol_site/resources/fonts/NotoSansArabic-Regular.ttf",
        "ghalbol_site/resources/fonts/NotoSansHebrew-Regular.ttf",
        "ghalbol_site/resources/fonts/NotoSansThaana-Regular.ttf",
        "ghalbol_site/resources/fonts/NotoSerifTibetan-Regular.ttf",
        "ghalbol_site/resources/fonts/NotoSansOlChiki-Regular.ttf",
        "ghalbol_site/resources/fonts/NotoSansMeeteiMayek-Regular.ttf",
        "ghalbol_site/resources/fonts/NotoSansThai-Regular.ttf",
        "ghalbol_site/resources/fonts/NotoSansLao-Regular.ttf",
        "ghalbol_site/resources/fonts/NotoSansKhmer-Regular.ttf",
        "ghalbol_site/resources/fonts/NotoSansMyanmar-Regular.ttf",
        "ghalbol_site/resources/fonts/NotoSansEthiopic-Regular.ttf",
        "ghalbol_site/resources/fonts/NotoSansArmenian-Regular.ttf",
        "ghalbol_site/resources/fonts/NotoSansGeorgian-Regular.ttf",
        "ghalbol_site/resources/fonts/SiteCJK-SC.otf",
        "ghalbol_site/resources/fonts/SiteCJK-TC.otf",
        "ghalbol_site/resources/fonts/SiteCJK-JP.otf",
        "ghalbol_site/resources/fonts/SiteCJK-KR.otf",
    ]
);

include!("ui.rs");

const PUBLIC_HOST: &str = "https://ghalbol.com";
const PLAY_URL: &str = "https://play.google.com/store/apps/details?id=com.ghalbol";
const GITHUB_URL: &str = "https://github.com/gotigin/ghalbol";
const GOTIGIN_URL: &str = "https://gotigin.com/";

const HOME_PATH: &str = "/";
const PRIVACY_PATH: &str = "/privacy";
const TERMS_PATH: &str = "/terms";
const SUPPORT_PATH: &str = "/support";
const ABOUT_PATH: &str = "/about";
const DELETION_PATH: &str = "/data-deletion";
const CANCELLATION_PATH: &str = "/cancellation";
const ACCOUNT_PATH: &str = "/account";
const LINUX_PATH: &str = "/download/linux";

const CARDS_SIDE_BY_SIDE_FROM: f64 = 900.0;
const MIN_WIDTH: f64 = 320.0;
const MAX_CONTENT: f64 = 1080.0;
const MAX_CONTENT_WIDE: f64 = 1200.0;
const NAV_INLINE_FROM: f64 = 960.0;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tier {
    Phone,
    Narrow,
    Tablet,
    Desk,
    Wide,
}

impl Tier {
    fn from_size(w: f64, h: f64) -> Self {
        if w < 480.0 {
            Self::Phone
        } else if w < 720.0 {
            Self::Narrow
        } else if w < 1024.0 {
            Self::Tablet
        } else if w >= 1600.0 && h >= 900.0 {
            Self::Wide
        } else {
            Self::Desk
        }
    }

    fn max_content(self) -> f64 {
        if self == Self::Wide {
            MAX_CONTENT_WIDE
        } else {
            MAX_CONTENT
        }
    }

    fn side_pad(self) -> f64 {
        match self {
            Self::Phone => 16.0,
            Self::Narrow => 24.0,
            Self::Tablet => 40.0,
            Self::Desk | Self::Wide => 64.0,
        }
    }

    fn band_pad_v(self) -> f64 {
        match self {
            Self::Phone => 56.0,
            Self::Narrow => 52.0,
            _ => 64.0,
        }
    }
}

#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    laid_out: Option<(Tier, i64, i64)>,
    #[rust]
    nav_open: bool,
    #[rust]
    lang: SiteLang,
}

pub fn script_font_files(group: ScriptGroup) -> Vec<&'static str> {
    const PLEX: &str = "resources/fonts/IBMPlexSans-Text.ttf";
    let script = match group {
        ScriptGroup::Latin | ScriptGroup::Cyrillic | ScriptGroup::Greek => None,
        ScriptGroup::Devanagari => Some("resources/fonts/NotoSansDevanagari-Regular.ttf"),
        ScriptGroup::Bengali => Some("resources/fonts/NotoSansBengali-Regular.ttf"),
        ScriptGroup::Gurmukhi => Some("resources/fonts/NotoSansGurmukhi-Regular.ttf"),
        ScriptGroup::Gujarati => Some("resources/fonts/NotoSansGujarati-Regular.ttf"),
        ScriptGroup::Oriya => Some("resources/fonts/NotoSansOriya-Regular.ttf"),
        ScriptGroup::Tamil => Some("resources/fonts/NotoSansTamil-Regular.ttf"),
        ScriptGroup::Telugu => Some("resources/fonts/NotoSansTelugu-Regular.ttf"),
        ScriptGroup::Kannada => Some("resources/fonts/NotoSansKannada-Regular.ttf"),
        ScriptGroup::Malayalam => Some("resources/fonts/NotoSansMalayalam-Regular.ttf"),
        ScriptGroup::Sinhala => Some("resources/fonts/NotoSansSinhala-Regular.ttf"),
        ScriptGroup::Arabic => Some("resources/fonts/NotoSansArabic-Regular.ttf"),
        ScriptGroup::Hebrew => Some("resources/fonts/NotoSansHebrew-Regular.ttf"),
        ScriptGroup::Thaana => Some("resources/fonts/NotoSansThaana-Regular.ttf"),
        ScriptGroup::Tibetan => Some("resources/fonts/NotoSerifTibetan-Regular.ttf"),
        ScriptGroup::OlChiki => Some("resources/fonts/NotoSansOlChiki-Regular.ttf"),
        ScriptGroup::MeeteiMayek => Some("resources/fonts/NotoSansMeeteiMayek-Regular.ttf"),
        ScriptGroup::Thai => Some("resources/fonts/NotoSansThai-Regular.ttf"),
        ScriptGroup::Lao => Some("resources/fonts/NotoSansLao-Regular.ttf"),
        ScriptGroup::Khmer => Some("resources/fonts/NotoSansKhmer-Regular.ttf"),
        ScriptGroup::Myanmar => Some("resources/fonts/NotoSansMyanmar-Regular.ttf"),
        ScriptGroup::Ethiopic => Some("resources/fonts/NotoSansEthiopic-Regular.ttf"),
        ScriptGroup::Armenian => Some("resources/fonts/NotoSansArmenian-Regular.ttf"),
        ScriptGroup::Georgian => Some("resources/fonts/NotoSansGeorgian-Regular.ttf"),
        ScriptGroup::CjkSc => Some("resources/fonts/SiteCJK-SC.otf"),
        ScriptGroup::CjkTc => Some("resources/fonts/SiteCJK-TC.otf"),
        ScriptGroup::CjkJp => Some("resources/fonts/SiteCJK-JP.otf"),
        ScriptGroup::CjkKr => Some("resources/fonts/SiteCJK-KR.otf"),
        ScriptGroup::LatinExt => Some("resources/fonts/NotoSans-Regular.ttf"),
    };
    let mut files = vec![PLEX];
    files.extend(script);
    files
}

fn open_lang_picker(cx: &mut Cx) {
    #[cfg(target_arch = "wasm32")]
    {
        let mut url = "#lang".to_string();
        if let OsType::Web(page) = cx.os_type() {
            if let Some(lang) = site_i18n::lang_from_hash(&page.hash) {
                url.push_str(&format!("&l={lang}"));
            }
        }
        cx.open_url(&url, OpenUrlInPlace::Yes);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = cx;
        log!("language picker is web-only");
    }
}

fn open_site(cx: &mut Cx, url: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        cx.open_url(url, OpenUrlInPlace::Yes);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        open_external(cx, url);
    }
}

fn open_external(cx: &mut Cx, url: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        cx.open_url(url, OpenUrlInPlace::No);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = cx;
        #[cfg(target_os = "windows")]
        let spawned = std::process::Command::new("cmd")
            .args(["/C", "start", "", url])
            .spawn();
        #[cfg(target_os = "macos")]
        let spawned = std::process::Command::new("open").arg(url).spawn();
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        let spawned = std::process::Command::new("xdg-open").arg(url).spawn();
        if let Err(err) = spawned {
            log!("could not open {url}: {err}");
        }
    }
}

impl App {
    fn tr(&self, key: &str) -> String {
        self.lang.text(key, locale::en(key))
    }

    fn hash_lang(cx: &Cx) -> Option<&'static str> {
        if let OsType::Web(page) = cx.os_type() {
            return site_i18n::lang_from_hash(&page.hash);
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            std::env::var("SITE_LANG")
                .ok()
                .and_then(|c| site_i18n::language_by_code(&c))
                .map(|l| l.code)
        }
        #[cfg(target_arch = "wasm32")]
        {
            None
        }
    }

    fn paint_nav_toggle(&mut self, cx: &mut Cx) {
        let label = if self.nav_open {
            self.tr("close")
        } else {
            self.tr("menu")
        };
        self.ui.button(cx, ids!(nav_toggle)).set_text(cx, &label);
    }

    fn paint_lang_chip(&mut self, cx: &mut Cx) {
        let code = if self.lang.wanted.is_empty() {
            "en".to_string()
        } else {
            self.lang.wanted.clone()
        };
        self.ui
            .button(cx, ids!(nav_lang))
            .set_text(cx, &code.to_uppercase());
    }

    fn sync_lang(&mut self, cx: &mut Cx) {
        let want = Self::hash_lang(cx).unwrap_or("en");
        if self.lang.last_hash_lang.as_deref() == Some(want) {
            return;
        }
        self.lang.last_hash_lang = Some(want.to_string());
        if let Some(table) = self.lang.want(want) {
            self.fetch_pack(cx, table);
        } else {
            self.apply_lang(cx);
        }
        self.paint_lang_chip(cx);
    }

    fn fetch_pack(&mut self, cx: &mut Cx, table: String) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("translations")
                .join(format!("{table}.tsv"));
            match std::fs::read_to_string(&path) {
                Ok(text) => {
                    self.lang.insert_pack(&table, &text);
                    self.apply_lang(cx);
                }
                Err(err) => log!("ghalbol-site: {}: {err}", path.display()),
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            let id = LiveId::unique();
            cx.http_request(
                id,
                HttpRequest::new(site_i18n::pack_web_path(&table), HttpMethod::GET),
            );
            self.lang.set_pending(id, table);
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn handle_lang_responses(&mut self, cx: &mut Cx, responses: &NetworkResponsesEvent) {
        for response in responses {
            let (request_id, body) = match response {
                NetworkResponse::HttpResponse {
                    request_id,
                    response,
                } => (
                    *request_id,
                    (200..300)
                        .contains(&response.status_code)
                        .then(|| response.body.clone())
                        .flatten(),
                ),
                NetworkResponse::HttpError { request_id, .. } => (*request_id, None),
                _ => continue,
            };
            if self.lang.take_response(request_id, body.as_deref()) {
                self.apply_lang(cx);
            }
        }
    }

    fn apply_lang(&mut self, cx: &mut Cx) {
        let lang = self.lang.wanted.clone();
        if lang.is_empty() || !self.lang.has_pack(&lang) {
            return;
        }
        if let Some(info) = site_i18n::language_by_code(&lang) {
            let script = site_i18n::script_group(info.code);
            let _fonts = script_font_files(script);
            log!(
                "ghalbol-site: lang={} ({}) native={} region={} script={:?} font_files={}",
                info.code,
                info.name_en,
                info.native_name,
                info.region,
                script,
                _fonts.len()
            );
        }
        let md: &[(&[LiveId], &str)] = &[
            (ids!(hero_title), "hero_title"),
            (ids!(hero_lede), "hero_lede"),
            (ids!(product_eyebrow), "product_eyebrow"),
            (ids!(product_title), "product_title"),
            (ids!(product_lead), "product_lead"),
            (ids!(feat_e2e_title), "feat_e2e_title"),
            (ids!(feat_e2e_body), "feat_e2e_body"),
            (ids!(feat_call_title), "feat_call_title"),
            (ids!(feat_call_body), "feat_call_body"),
            (ids!(feat_id_title), "feat_id_title"),
            (ids!(feat_id_body), "feat_id_body"),
            (ids!(invite_eyebrow), "invite_eyebrow"),
            (ids!(invite_title), "invite_band_title"),
            (ids!(invite_lead), "invite_lead"),
            (ids!(invite_step1), "invite_step1"),
            (ids!(invite_step2), "invite_step2"),
            (ids!(invite_step3), "invite_step3"),
            (ids!(plat_eyebrow), "plat_eyebrow"),
            (ids!(plat_title), "plat_title"),
            (ids!(plat_lead), "plat_lead"),
            (ids!(plat_android_title), "plat_android"),
            (ids!(plat_android_body), "plat_android_body"),
            (ids!(plat_linux_title), "plat_linux"),
            (ids!(plat_linux_body), "plat_linux_body"),
            (ids!(names_eyebrow), "names_eyebrow"),
            (ids!(names_title), "names_title"),
            (ids!(names_body), "names_body"),
        ];
        for (path, key) in md {
            let text = self.tr(key);
            self.ui.markdown(cx, path).set_text(cx, &text);
        }
        let btns: &[(&[LiveId], &str)] = &[
            (ids!(nav_messenger), "nav_messenger"),
            (ids!(nav_linux), "nav_linux"),
            (ids!(nav_privacy), "privacy"),
            (ids!(nav_play), "nav_play"),
            (ids!(nav_login), "login"),
            (ids!(cta_see), "cta_see"),
            (ids!(cta_play), "cta_play"),
            (ids!(cta_google_play), "cta_google_play"),
            (ids!(cta_linux_dl), "cta_linux_dl"),
            (ids!(cta_open_play), "cta_open_play"),
            (ids!(cta_linux_plat), "cta_linux_dl"),
            (ids!(foot_privacy), "privacy"),
            (ids!(foot_terms), "terms"),
            (ids!(foot_support), "support"),
            (ids!(foot_deletion), "data_deletion"),
            (ids!(foot_cancellation), "cancellation"),
            (ids!(foot_about), "about"),
            (ids!(foot_play), "play"),
            (ids!(foot_linux), "nav_linux"),
        ];
        for (path, key) in btns {
            let text = self.tr(key);
            self.ui.button(cx, path).set_text(cx, &text);
        }

        self.paint_nav_toggle(cx);
        self.paint_lang_chip(cx);
        let title = self.tr("window_title");
        self.ui.window(cx, ids!(main_window)).set_title(cx, &title);
        self.ui.redraw(cx);
    }

    fn site_url(cx: &Cx, path: &str) -> String {
        if matches!(cx.os_type(), OsType::Web(_)) {
            path.to_string()
        } else {
            format!("{PUBLIC_HOST}{path}")
        }
    }

    fn apply_responsive(&mut self, cx: &mut Cx) {
        let size = self.ui.window(cx, ids!(main_window)).get_inner_size(cx);
        if size.x < 1.0 || size.y < 1.0 {
            return;
        }
        let layout_w = size.x.max(MIN_WIDTH);
        let tier = Tier::from_size(layout_w, size.y);
        let key = (tier, layout_w.round() as i64, size.y.round() as i64);
        if self.laid_out == Some(key) {
            return;
        }
        self.laid_out = Some(key);

        let min_side = tier.side_pad();
        let side = min_side.max((layout_w - tier.max_content()) / 2.0);
        let pad_v = tier.band_pad_v();
        let band_pad = Inset {
            left: side,
            right: side,
            top: pad_v,
            bottom: pad_v,
        };

        for path in [
            ids!(hero),
            ids!(product),
            ids!(invite),
            ids!(platforms),
            ids!(names),
            ids!(closing),
        ] {
            if let Some(mut view) = self.ui.view(cx, path).borrow_mut() {
                view.layout.padding = band_pad;
            }
        }

        let nav_inline = layout_w >= NAV_INLINE_FROM;
        if nav_inline {
            self.nav_open = false;
        }
        if let Some(mut bar) = self.ui.view(cx, ids!(top)).borrow_mut() {
            let pad = if tier == Tier::Phone { 12.0 } else { 24.0 };
            bar.layout.padding = Inset {
                left: pad,
                right: pad,
                top: 12.0,
                bottom: 12.0,
            };
            bar.layout.flow = if nav_inline {
                Flow::right()
            } else {
                Flow::Down
            };
            bar.layout.spacing = if nav_inline { 10.0 } else { 8.0 };
        }
        if let Some(mut chrome) = self.ui.view(cx, ids!(top_chrome)).borrow_mut() {
            chrome.walk.width = Size::fill();
        }
        self.ui
            .button(cx, ids!(nav_toggle))
            .set_visible(cx, !nav_inline);
        self.paint_nav_toggle(cx);
        let show_nav = nav_inline || self.nav_open;
        self.ui.view(cx, ids!(topnav)).set_visible(cx, show_nav);
        if let Some(mut nav) = self.ui.view(cx, ids!(topnav)).borrow_mut() {
            nav.layout.flow = if nav_inline {
                Flow::right()
            } else {
                Flow::Down
            };
            nav.walk.width = if nav_inline {
                Size::fit()
            } else {
                Size::fill()
            };
            nav.layout.align.x = if nav_inline { 1.0 } else { 0.0 };
        }

        let stack = layout_w < CARDS_SIDE_BY_SIDE_FROM;
        for path in [ids!(feature_grid), ids!(plat_cards), ids!(hero_row)] {
            if let Some(mut view) = self.ui.view(cx, path).borrow_mut() {
                view.layout.flow = if stack {
                    Flow::Down
                } else {
                    Flow::right()
                };
            }
        }

        let (title_px, lede_px) = match tier {
            Tier::Phone => (28.0, 16.0),
            Tier::Narrow => (34.0, 17.0),
            Tier::Tablet => (38.0, 18.0),
            Tier::Desk | Tier::Wide => (42.0, 18.0),
        };
        for id in [
            ids!(hero_title),
            ids!(product_title),
            ids!(invite_title),
            ids!(plat_title),
            ids!(names_title),
        ] {
            Self::set_md_size(cx, &self.ui, id, title_px);
        }
        for id in [ids!(hero_lede)] {
            Self::set_md_size(cx, &self.ui, id, lede_px);
        }
    }

    fn set_md_size(cx: &mut Cx, ui: &WidgetRef, id: &[LiveId], size: f64) {
        if let Some(mut md) = ui.markdown(cx, id).borrow_mut() {
            let px = size as f32;
            md.font_size = px;
            md.text_style_normal.font_size = px;
            md.text_style_bold.font_size = px;
            md.draw_text.text_style.font_size = px;
            md.redraw(cx);
        }
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        self.ui
            .window(cx, ids!(main_window))
            .set_title(cx, &self.tr("window_title"));
        self.apply_responsive(cx);
        self.sync_lang(cx);
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(cx, ids!(nav_toggle)).clicked(actions) {
            self.nav_open = !self.nav_open;
            self.laid_out = None;
            self.apply_responsive(cx);
            return;
        }

        if self
            .ui
            .view(cx, ids!(brand_home))
            .finger_up(actions)
            .is_some_and(|e| e.was_tap())
        {
            open_site(cx, &Self::site_url(cx, HOME_PATH));
        }

        let mut close_nav = false;

        // In-page: product band is below the fold — closing the menu is enough.
        if self.ui.button(cx, ids!(nav_messenger)).clicked(actions)
            || self.ui.button(cx, ids!(cta_see)).clicked(actions)
        {
            close_nav = true;
        }

        if self.ui.button(cx, ids!(nav_linux)).clicked(actions)
            || self.ui.button(cx, ids!(cta_linux_dl)).clicked(actions)
            || self.ui.button(cx, ids!(cta_linux_plat)).clicked(actions)
            || self.ui.button(cx, ids!(foot_linux)).clicked(actions)
        {
            close_nav = true;
            open_site(cx, &Self::site_url(cx, LINUX_PATH));
        }

        if self.ui.button(cx, ids!(nav_privacy)).clicked(actions)
            || self.ui.button(cx, ids!(foot_privacy)).clicked(actions)
        {
            close_nav = true;
            open_site(cx, &Self::site_url(cx, PRIVACY_PATH));
        }
        if self.ui.button(cx, ids!(foot_terms)).clicked(actions) {
            close_nav = true;
            open_site(cx, &Self::site_url(cx, TERMS_PATH));
        }
        if self.ui.button(cx, ids!(foot_support)).clicked(actions) {
            close_nav = true;
            open_site(cx, &Self::site_url(cx, SUPPORT_PATH));
        }
        if self.ui.button(cx, ids!(foot_deletion)).clicked(actions) {
            close_nav = true;
            open_site(cx, &Self::site_url(cx, DELETION_PATH));
        }
        if self.ui.button(cx, ids!(foot_cancellation)).clicked(actions) {
            close_nav = true;
            open_site(cx, &Self::site_url(cx, CANCELLATION_PATH));
        }
        if self.ui.button(cx, ids!(foot_about)).clicked(actions) {
            close_nav = true;
            open_site(cx, &Self::site_url(cx, ABOUT_PATH));
        }
        if self.ui.button(cx, ids!(nav_login)).clicked(actions) {
            close_nav = true;
            open_site(cx, &Self::site_url(cx, ACCOUNT_PATH));
        }

        if self.ui.button(cx, ids!(nav_play)).clicked(actions)
            || self.ui.button(cx, ids!(cta_play)).clicked(actions)
            || self.ui.button(cx, ids!(cta_google_play)).clicked(actions)
            || self.ui.button(cx, ids!(cta_open_play)).clicked(actions)
            || self.ui.button(cx, ids!(foot_play)).clicked(actions)
        {
            close_nav = true;
            open_external(cx, PLAY_URL);
        }

        if self.ui.button(cx, ids!(open_nav_github)).clicked(actions) {
            close_nav = true;
            open_external(cx, GITHUB_URL);
        }
        if self.ui.button(cx, ids!(foot_gotigin)).clicked(actions) {
            close_nav = true;
            open_external(cx, GOTIGIN_URL);
        }

        if self.ui.button(cx, ids!(nav_lang)).clicked(actions) {
            close_nav = true;
            open_lang_picker(cx);
        }

        if close_nav && self.nav_open {
            self.nav_open = false;
            self.laid_out = None;
            self.apply_responsive(cx);
        }
    }
}

impl AppMain for App {
    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        crate::makepad_widgets::script_mod(vm);
        self::script_mod(vm)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        #[cfg(target_arch = "wasm32")]
        if let Event::NetworkResponses(responses) = event {
            self.handle_lang_responses(cx, responses);
        }
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
        match event {
            Event::Startup | Event::WindowGeomChange(_) => self.apply_responsive(cx),
            Event::Signal => self.sync_lang(cx),
            _ => {}
        }
    }
}
