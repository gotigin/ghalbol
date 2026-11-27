// ghalbol.com — Makepad brand SPA (Wadhu pattern).
// Trademark: Gotigin. Company: GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED.
// Brand: Ghal Bol. Legal/account/connect/download are HTML in the same Hosting tree.
script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    // Tokens from the HTML site :root (surface / ink / primary / navy).
    let Surface = #xeef2f8
    let Hub = #xe4eaf3
    let PanelBg = #xffffff
    let Ink = #x0b1220
    let Muted = #x5a6b7d
    let Faint = #x7a8b9c
    let Border = #xc5d0de
    let Primary = #x0183fb
    let PrimaryHover = #x0170d8
    let Navy = #x10236d
    let Online = #x01c98a
    let Select = #x0183fb44

    // Do not force sdf_luma_bias = 0.0 on Makepad 1f60033 — that collapses SDF
    // glyphs into solid bars on wasm. Leave the theme default.
    mod.widgets.Markdown.selectable = true
    mod.widgets.Markdown.draw_selection.color = Select
    mod.widgets.TextFlow.draw_selection.color = Select

    // Same pattern as apps/ghalbol splash_faces: point theme + widget classes at
    // allowlisted site fonts before any face prototypes capture text_style.
    let SiteFonts = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
    }
    let SiteFontsBold = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-SemiBold.ttf") asc: -0.1 desc: 0.0}
    }
    mod.theme.font_regular.font_family = SiteFonts
    mod.theme.font_label.font_family = SiteFonts
    mod.theme.font_bold.font_family = SiteFontsBold
    mod.theme.font_italic.font_family = SiteFonts
    mod.theme.font_bold_italic.font_family = SiteFontsBold
    mod.widgets.Label.draw_text.text_style.font_family = SiteFonts
    mod.widgets.Button.draw_text.text_style.font_family = SiteFonts
    mod.widgets.Markdown.draw_text.text_style.font_family = SiteFonts

    let Eyebrow = Markdown{
        selectable: true
        height: Fit
        width: Fill
        padding: 0.
        paragraph_spacing: 0.
        pre_code_spacing: 0.
        font_color: Primary
        font_size: 13.
        text_style_normal: theme.font_bold{font_size: 13. line_spacing: 1.3}
        text_style_bold: theme.font_bold{font_size: 13. line_spacing: 1.3}
        draw_text +: { color: Primary text_style: theme.font_bold{font_size: 13. line_spacing: 1.3} }
        draw_selection +: { color: Select }
        draw_block +: { selection_color: Select }
    }
    let Headline = Markdown{
        selectable: true
        height: Fit
        width: Fill
        padding: 0.
        paragraph_spacing: 0.
        pre_code_spacing: 0.
        font_color: Ink
        font_size: 42.
        text_style_normal: theme.font_bold{font_size: 42. line_spacing: 1.08}
        text_style_bold: theme.font_bold{font_size: 42. line_spacing: 1.08}
        draw_text +: { color: Ink text_style: theme.font_bold{font_size: 42. line_spacing: 1.08} }
        draw_selection +: { color: Select }
        draw_block +: { selection_color: Select }
    }
    let Subhead = Markdown{
        selectable: true
        height: Fit
        width: Fill
        padding: 0.
        paragraph_spacing: 0.
        pre_code_spacing: 0.
        font_color: Ink
        font_size: 28.
        text_style_normal: theme.font_bold{font_size: 28. line_spacing: 1.15}
        text_style_bold: theme.font_bold{font_size: 28. line_spacing: 1.15}
        draw_text +: { color: Ink text_style: theme.font_bold{font_size: 28. line_spacing: 1.15} }
        draw_selection +: { color: Select }
        draw_block +: { selection_color: Select }
    }
    let Lede = Markdown{
        selectable: true
        height: Fit
        width: Fill
        padding: 0.
        paragraph_spacing: 0.
        pre_code_spacing: 0.
        font_color: Muted
        font_size: 18.
        text_style_normal: theme.font_regular{font_size: 18. line_spacing: 1.45}
        text_style_bold: theme.font_bold{font_size: 18. line_spacing: 1.45}
        draw_text +: { color: Muted text_style: theme.font_regular{font_size: 18. line_spacing: 1.45} }
        draw_selection +: { color: Select }
        draw_block +: { selection_color: Select }
    }
    let Body = Markdown{
        selectable: true
        height: Fit
        width: Fill
        padding: 0.
        paragraph_spacing: 0.
        pre_code_spacing: 0.
        font_color: Muted
        font_size: 15.5
        text_style_normal: theme.font_regular{font_size: 15.5 line_spacing: 1.45}
        text_style_bold: theme.font_bold{font_size: 15.5 line_spacing: 1.45}
        draw_text +: { color: Muted text_style: theme.font_regular{font_size: 15.5 line_spacing: 1.45} }
        draw_selection +: { color: Select }
        draw_block +: { selection_color: Select }
    }
    let CardTitle = Markdown{
        selectable: true
        height: Fit
        width: Fill
        padding: 0.
        paragraph_spacing: 0.
        pre_code_spacing: 0.
        font_color: Navy
        font_size: 20.
        text_style_normal: theme.font_bold{font_size: 20. line_spacing: 1.25}
        text_style_bold: theme.font_bold{font_size: 20. line_spacing: 1.25}
        draw_text +: { color: Navy text_style: theme.font_bold{font_size: 20. line_spacing: 1.25} }
        draw_selection +: { color: Select }
        draw_block +: { selection_color: Select }
    }
    let PrimaryBtn = Button{
        width: Fit height: Fit
        padding: Inset{left: 18. right: 18. top: 16. bottom: 16.}
        text: "Open"
        draw_bg +: { color: Primary }
        draw_text +: { color: #xffffff text_style: theme.font_bold{font_size: 14.} }
    }
    let GhostBtn = Button{
        width: Fit height: Fit
        padding: Inset{left: 14. right: 14. top: 16. bottom: 16.}
        draw_bg +: { color: PanelBg border_color: Border border_size: 1. border_radius: 10. }
        draw_text +: { color: Ink text_style: theme.font_bold{font_size: 13.} }
    }
    let NavBtn = Button{
        width: Fit height: Fit
        padding: Inset{left: 12. right: 12. top: 16. bottom: 16.}
        draw_bg +: { color: #x00000000 }
        draw_text +: { color: Ink text_style: theme.font_bold{font_size: 13.} }
    }
    let Band = SolidView{
        width: Fill height: Fit
        padding: Inset{left: 64. right: 64. top: 64. bottom: 64.}
        draw_bg +: { color: Surface }
        flow: Down
        spacing: 18.
    }
    let BandAlt = SolidView{
        width: Fill height: Fit
        padding: Inset{left: 64. right: 64. top: 64. bottom: 64.}
        draw_bg +: { color: Hub }
        flow: Down
        spacing: 18.
    }
    let Panel = RoundedView{
        width: Fill height: Fit
        padding: Inset{left: 24. right: 24. top: 22. bottom: 22.}
        draw_bg +: {
            color: PanelBg
            border_color: Border
            border_size: 1.
            border_radius: 14.
        }
        flow: Down
        spacing: 12.
    }

    let UiCyrillic = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
    }
    let UiGreek = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
    }
    let UiDevanagari = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansDevanagari-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiBengali = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansBengali-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiGurmukhi = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansGurmukhi-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiGujarati = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansGujarati-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiOriya = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansOriya-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiTamil = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansTamil-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiTelugu = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansTelugu-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiKannada = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansKannada-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiMalayalam = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansMalayalam-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiSinhala = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansSinhala-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiArabic = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansArabic-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiHebrew = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansHebrew-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiThaana = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansThaana-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiTibetan = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSerifTibetan-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiOlChiki = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansOlChiki-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiMeetei = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansMeeteiMayek-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiThai = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansThai-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiLao = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansLao-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiKhmer = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansKhmer-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiMyanmar = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansMyanmar-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiEthiopic = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansEthiopic-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiArmenian = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansArmenian-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiGeorgian = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansGeorgian-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiCjkSc = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/SiteCJK-SC.otf") asc: 0.0 desc: 0.0}
    }
    let UiCjkTc = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/SiteCJK-TC.otf") asc: 0.0 desc: 0.0}
    }
    let UiCjkJp = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/SiteCJK-JP.otf") asc: 0.0 desc: 0.0}
    }
    let UiCjkKr = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/SiteCJK-KR.otf") asc: 0.0 desc: 0.0}
    }
    let UiLatinExt = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSans-Regular.ttf") asc: 0.0 desc: 0.0}
    }

    startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                window.title: "Ghal Bol — encrypted messenger"
                window.inner_size: vec2(1280, 800)
                pass.clear_color: Surface
                body +: {
                    width: Fill
                    height: Fill
                    flow: Down
                    shell := SolidView{
                        width: Fill height: Fill
                        draw_bg +: { color: Surface }
                        flow: Down
                        top := SolidView{
                            width: Fill height: Fit
                            padding: Inset{left: 24. right: 24. top: 12. bottom: 12.}
                            draw_bg +: { color: PanelBg }
                            flow: Down
                            spacing: 8.
                            align: Align{y: 0.5}
                            top_chrome := View{
                                width: Fill height: Fit
                                flow: Right
                                spacing: 10.
                                align: Align{y: 0.5}
                                brand_home := View{
                                    width: Fit height: Fit
                                    flow: Right
                                    spacing: 10.
                                    align: Align{y: 0.5}
                                    cursor: MouseCursor.Hand
                                    brand_icon := Image{
                                        width: 40 height: 40
                                        fit: ImageFit.Stretch
                                        src: crate_resource("self:resources/media/icon.webp")
                                    }
                                    brand_name := Label{
                                        width: Fit
                                        text: "Ghal Bol"
                                        draw_text +: {
                                            color: Navy
                                            text_style: theme.font_bold{font_size: 16.}
                                        }
                                    }
                                }
                                top_spacer := View{ width: Fill height: 1. }
                                nav_toggle := NavBtn{ text: "Menu" }
                            }
                            topnav := View{
                                visible: false
                                width: Fill height: Fit
                                flow: Down
                                spacing: 8.
                                wrap_spacing: 8.
                                align: Align{y: 0.5}
                                nav_messenger := NavBtn{ text: "Messenger" }
                                nav_linux := NavBtn{ text: "Linux" }
                                nav_privacy := NavBtn{ text: "Privacy" }
                                nav_play := NavBtn{ text: "Play Store" }
                                nav_login := NavBtn{ text: "Login" }
                                go_github := View{
                                    width: 44 height: 44
                                    flow: Overlay
                                    align: Align{x: 0.5 y: 0.5}
                                    View{
                                        width: Fill height: Fill
                                        flow: Down
                                        align: Align{x: 0.5 y: 0.5}
                                        Image{
                                            width: 18 height: 18
                                            fit: ImageFit.Stretch
                                            src: crate_resource("self:resources/media/github-mark.webp")
                                        }
                                    }
                                    open_nav_github := Button{
                                        width: Fill height: Fill
                                        text: ""
                                        draw_bg +: { color: #x00000000 }
                                    }
                                }
                                nav_lang := NavBtn{ text: "EN" }
                            }
                        }
                        page := ScrollYView{
                            width: Fill
                            height: Fill
                            flow: Down

                            hero := Band{
                                hero_row := View{
                                    width: Fill height: Fit
                                    flow: Right
                                    spacing: 32.
                                    wrap_spacing: 24.
                                    align: Align{y: 0.5}
                                    hero_copy := View{
                                        width: Fill height: Fit
                                        flow: Down
                                        spacing: 16.
                                        Eyebrow{ body: "Ghal Bol" }
                                        hero_title := Headline{ body: "Encrypted chat you own." }
                                        hero_lede := Lede{ body: "1:1 text and voice/video calls — no phone number, keys and transcripts on your device." }
                                        cta_row := View{
                                            width: Fill height: Fit
                                            flow: Right { wrap: true }
                                            spacing: 12.
                                            wrap_spacing: 10.
                                            cta_see := PrimaryBtn{ text: "See the messenger" }
                                            cta_play := GhostBtn{ text: "Get it on Play" }
                                        }
                                    }
                                    hero_visual := View{
                                        width: Fill height: Fit
                                        flow: Overlay
                                        align: Align{x: 0.5 y: 0.5}
                                        Image{
                                            width: Fill height: Fit
                                            fit: ImageFit.Horizontal
                                            src: crate_resource("self:resources/media/feature.webp")
                                        }
                                    }
                                }
                            }

                            product := BandAlt{
                                product_eyebrow := Eyebrow{ body: "Product" }
                                product_title := Subhead{ body: "Ghal Bol messenger" }
                                product_lead := Body{ body: "The shipping product under the Ghal Bol brand — peer-to-peer encrypted text, delivery and read receipts, and native voice/video calls between online peers. The public app name is Ghal Bol (com.ghalbol)." }
                                feature_grid := View{
                                    width: Fill height: Fit
                                    flow: Right
                                    spacing: 16.
                                    feat_e2e := Panel{
                                        feat_e2e_title := CardTitle{ body: "End-to-end chat" }
                                        feat_e2e_body := Body{ body: "Messages are sealed on your device. The delivery mailbox holds only opaque ciphertext until the recipient acknowledges it." }
                                    }
                                    feat_call := Panel{
                                        feat_call_title := CardTitle{ body: "Voice & video calls" }
                                        feat_call_body := Body{ body: "Calls connect peer-to-peer on LAN when possible, otherwise through a lightweight coord bridge that never decrypts media." }
                                    }
                                    feat_id := Panel{
                                        feat_id_title := CardTitle{ body: "Device-owned identity" }
                                        feat_id_body := Body{ body: "Each install creates a local cryptographic identity. Invite with QR or link — no phone number or social login." }
                                    }
                                }
                                product_cta := View{
                                    width: Fill height: Fit
                                    flow: Right { wrap: true }
                                    spacing: 12.
                                    wrap_spacing: 10.
                                    cta_google_play := PrimaryBtn{ text: "Google Play" }
                                    cta_linux_dl := GhostBtn{ text: "Linux download" }
                                }
                            }

                            invite := Band{
                                invite_eyebrow := Eyebrow{ body: "Ghal Bol · Invites" }
                                invite_title := Subhead{ body: "Scan once. Connect when online." }
                                invite_lead := Body{ body: "Guest scans the host QR (or opens an invite link). The guest stores the host public key and dials. Mutual QR is never required." }
                                invite_step1 := Body{ body: "1 — Share your https://ghalbol.com/connect/… link or QR from Invite in Ghal Bol." }
                                invite_step2 := Body{ body: "2 — They open it in Ghal Bol (or install from Play / Linux first)." }
                                invite_step3 := Body{ body: "3 — When both are reachable, chat and calls connect — LAN and WAN paths run in parallel for calls." }
                            }

                            platforms := BandAlt{
                                plat_eyebrow := Eyebrow{ body: "Ghal Bol · Platforms" }
                                plat_title := Subhead{ body: "Android and Linux today" }
                                plat_lead := Body{ body: "Windows packages are built from the same workspace. The public download on this site is the Linux desktop bundle; Android ships on Google Play as com.ghalbol." }
                                plat_cards := View{
                                    width: Fill height: Fit
                                    flow: Right
                                    spacing: 16.
                                    plat_android := Panel{
                                        plat_android_title := CardTitle{ body: "Android" }
                                        plat_android_body := Body{ body: "Install Ghal Bol from Play. Verified App Links open invites straight into the app when possible." }
                                        cta_open_play := PrimaryBtn{ text: "Open Play Store" }
                                    }
                                    plat_linux := Panel{
                                        plat_linux_title := CardTitle{ body: "Linux x86_64" }
                                        plat_linux_body := Body{ body: "Extract the release archive, then ./install.sh (per-user) or ./run.sh to try in place." }
                                        cta_linux_plat := GhostBtn{ text: "Linux download" }
                                    }
                                }
                            }

                            names := Band{
                                names_eyebrow := Eyebrow{ body: "Names" }
                                names_title := Subhead{ body: "Trademark, company, brand" }
                                names_body := Body{ body: "Gotigin is the trademark (Where Technology Meets Discovery). GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED is the company. Ghal Bol is the brand for communication products; this site is ghalbol.com. The messenger is what ships today. Package names and the domain stay ghalbol — one word. There is no separate camel-case brand." }
                            }

                            closing := BandAlt{
                                foot_nav := View{
                                    width: Fill height: Fit
                                    flow: Right { wrap: true }
                                    spacing: 10.
                                    wrap_spacing: 8.
                                    foot_privacy := GhostBtn{ text: "Privacy" }
                                    foot_terms := GhostBtn{ text: "Terms" }
                                    foot_support := GhostBtn{ text: "Support" }
                                    foot_deletion := GhostBtn{ text: "Data deletion" }
                                    foot_cancellation := GhostBtn{ text: "Cancellation" }
                                    foot_about := GhostBtn{ text: "About" }
                                }
                                foot_links := View{
                                    width: Fill height: Fit
                                    flow: Right { wrap: true }
                                    spacing: 10.
                                    wrap_spacing: 8.
                                    foot_play := GhostBtn{ text: "Play" }
                                    foot_linux := GhostBtn{ text: "Linux" }
                                    foot_gotigin := GhostBtn{ text: "gotigin.com" }
                                }
                                Body{ body: "Gotigin · GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED · Ghal Bol" }
                                Body{ body: "© 2026 GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED" }
                            }

                            i18n_fonts := View{
                                visible: false
                                width: 0 height: 0
                                font_cyrillic := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiCyrillic}}}
                                font_greek := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiGreek}}}
                                font_devanagari := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiDevanagari}}}
                                font_bengali := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiBengali}}}
                                font_gurmukhi := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiGurmukhi}}}
                                font_gujarati := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiGujarati}}}
                                font_oriya := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiOriya}}}
                                font_tamil := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiTamil}}}
                                font_telugu := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiTelugu}}}
                                font_kannada := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiKannada}}}
                                font_malayalam := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiMalayalam}}}
                                font_sinhala := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiSinhala}}}
                                font_arabic := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiArabic}}}
                                font_hebrew := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiHebrew}}}
                                font_thaana := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiThaana}}}
                                font_tibetan := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiTibetan}}}
                                font_olchiki := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiOlChiki}}}
                                font_meetei := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiMeetei}}}
                                font_thai := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiThai}}}
                                font_lao := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiLao}}}
                                font_khmer := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiKhmer}}}
                                font_myanmar := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiMyanmar}}}
                                font_ethiopic := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiEthiopic}}}
                                font_armenian := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiArmenian}}}
                                font_georgian := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiGeorgian}}}
                                font_cjk_sc := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiCjkSc}}}
                                font_cjk_tc := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiCjkTc}}}
                                font_cjk_jp := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiCjkJp}}}
                                font_cjk_kr := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiCjkKr}}}
                                font_latin_ext := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiLatinExt}}}
                            }
                        }
                    }
                }
            }
        }
    }
}
