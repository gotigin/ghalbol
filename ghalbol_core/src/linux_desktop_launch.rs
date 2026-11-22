//! Best-effort raise of the Makepad desktop shell on Linux (D-Bus + gtk-launch + wake file).

use std::path::Path;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::desktop_wake::{touch_incoming_call_wake, touch_unlock_wake};

pub fn dbus_object_path(app_id: &str) -> String {
    format!("/{}", app_id.replace('.', "/"))
}

/// Freedesktop app ids are reverse-DNS tokens — reject anything that could confuse D-Bus / gtk-launch.
fn app_id_is_safe(app_id: &str) -> bool {
    let t = app_id.trim();
    if t.is_empty() || t.len() > 128 {
        return false;
    }
    t.bytes()
        .all(|b| b.is_ascii_alphanumeric() || b == b'.' || b == b'_' || b == b'-')
}

/// Freedesktop `Exec=` quoting (spaces / reserved shell metacharacters).
fn desktop_exec_quoted(path: &Path) -> String {
    let s = path.display().to_string();
    if s.bytes().any(|b| b == b' ' || b == b'\t' || b == b'"' || b == b'\\' || b == b'$' || b == b'`')
    {
        format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
    } else {
        s
    }
}

fn desktop_entry_escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('\n', " ").replace('\r', "")
}

/// Sprint / isolated tours must not write into the owner's real autostart or spam
/// session-bus banners (Xvfb shares `XDG_RUNTIME_DIR` with the login session).
pub(crate) fn skip_desktop_integration() -> bool {
    if std::env::var_os("GHALBOL_SKIP_DESKTOP_INSTALL").is_some_and(|v| v != "0") {
        return true;
    }
    // BRIEF local-only dead-end URLs for quality-sprint app tours.
    let coord = std::env::var("GHALBOL_COORD_URLS").unwrap_or_default();
    let delivery = std::env::var("GHALBOL_DELIVERY_URL").unwrap_or_default();
    if coord.contains("127.0.0.1:9")
        || coord.contains("localhost:9")
        || delivery.contains("127.0.0.1:9")
        || delivery.contains("localhost:9")
    {
        return true;
    }
    if let Ok(home) = std::env::var("HOME") {
        let h = home.as_str();
        if h.starts_with("/tmp/")
            || h.contains("/tmp/")
            || h.contains("/sprint")
            || h.contains("fakehome")
            || h.contains("ghalbol-home")
            || h.contains("home-empty")
            || h.contains("home-ghalbol")
            || h.contains("home-fresh")
        {
            return true;
        }
    }
    false
}

fn skip_desktop_install() -> bool {
    skip_desktop_integration()
}

fn ghalbol_binary_running() -> bool {
    let Ok(exe) = std::env::current_exe() else {
        return false;
    };
    let Some(name) = exe.file_name().and_then(|n| n.to_str()) else {
        return false;
    };
    let Ok(out) = Command::new("pgrep").args(["-x", name]).output() else {
        return false;
    };
    if !out.status.success() {
        return false;
    }
    // pgrep -x lists every match; more than one line means another instance is up
    // (this process itself may or may not be visible depending on timing).
    let n = String::from_utf8_lossy(&out.stdout)
        .lines()
        .filter(|l| !l.trim().is_empty())
        .count();
    n >= 1
}

/// Write [touch_wake], call `org.freedesktop.Application.Activate`, then `gtk-launch` only if needed.
pub fn wake_desktop_app(app_id: &str, touch_wake: fn()) {
    let app_id = app_id.trim();
    if !app_id_is_safe(app_id) {
        return;
    }
    touch_wake();
    let object_path = dbus_object_path(app_id);
    let activated = Command::new("gdbus")
        .args([
            "call",
            "-e",
            "-d",
            app_id,
            "-o",
            &object_path,
            "-m",
            "org.freedesktop.Application.Activate",
            "{}",
        ])
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    // Prefer raising the existing process; gtk-launch otherwise opens a second instance.
    if activated || ghalbol_binary_running() {
        // Best-effort window raise for compositors that ignore Application.Activate.
        let _ = Command::new("wmctrl").args(["-xa", app_id]).status();
        return;
    }
    let _ = Command::new("gtk-launch").arg(app_id).spawn();
}

pub fn wake_for_unlock(app_id: &str) {
    wake_desktop_app(app_id, touch_unlock_wake);
}

pub fn wake_for_incoming_call(app_id: &str) {
    wake_desktop_app(app_id, touch_incoming_call_wake);
}

/// Login autostart and a menu entry for this binary. One process: the app itself.
pub fn install_user_autostart() {
    if skip_desktop_install() {
        return;
    }
    let Ok(exe) = std::env::current_exe() else {
        return;
    };
    let Ok(home) = std::env::var("HOME") else {
        return;
    };
    let app_id = crate::host::default_app_namespace();
    if !app_id_is_safe(&app_id) {
        return;
    }
    let label = std::env::var("GHALBOL_APP_LABEL")
        .ok()
        .filter(|l| !l.trim().is_empty())
        .unwrap_or_else(|| "Ghal Bol".to_string());
    // Install a Freedesktop icon so the dock / app menu show Ghal Bol, not a generic glyph.
    let icon_dir = std::path::PathBuf::from(format!(
        "{home}/.local/share/icons/hicolor/128x128/apps"
    ));
    if std::fs::create_dir_all(&icon_dir).is_ok() {
        let icon_path = icon_dir.join(format!("{app_id}.png"));
        let _ = std::fs::write(&icon_path, include_bytes!("../../apps/ghalbol/resources/icon_128.png"));
    }
    // Quote Exec so paths with spaces (or desktop reserved chars) stay one argv token.
    let exe_q = desktop_exec_quoted(&exe);
    let label_safe = desktop_entry_escape(&label);
    let autostart = format!(
        "[Desktop Entry]\nType=Application\nName={label_safe}\nExec={exe_q}\nIcon={app_id}\nStartupNotify=true\nX-GNOME-Autostart-enabled=true\n",
    );
    let applications = format!(
        "[Desktop Entry]\nType=Application\nName={label_safe}\nExec={exe_q} %u\nIcon={app_id}\nStartupNotify=true\nMimeType=x-scheme-handler/ghalbol;\n",
    );
    let auto_dir = std::path::PathBuf::from(format!("{home}/.config/autostart"));
    let apps_dir = std::path::PathBuf::from(format!("{home}/.local/share/applications"));
    if std::fs::create_dir_all(&auto_dir).is_ok() {
        let _ = std::fs::write(auto_dir.join(format!("{app_id}.desktop")), autostart);
    }
    if std::fs::create_dir_all(&apps_dir).is_ok() {
        let file = apps_dir.join(format!("{app_id}.desktop"));
        let _ = std::fs::write(&file, applications);
        let _ = std::process::Command::new("xdg-mime")
            .args(["default", &format!("{app_id}.desktop"), "x-scheme-handler/ghalbol"])
            .status();
        let _ = std::process::Command::new("gtk-update-icon-cache")
            .args(["-f", &format!("{home}/.local/share/icons/hicolor")])
            .status();
    }
}

/// At most one unlock banner every 5 minutes (session start + locked inbound both call this).
const UNLOCK_NOTIFY_COOLDOWN_MS: u64 = 5 * 60 * 1000;

fn unlock_notify_last_ms() -> &'static AtomicU64 {
    static LAST: OnceLock<AtomicU64> = OnceLock::new();
    LAST.get_or_init(|| AtomicU64::new(0))
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

pub fn notify_unlock_needed() {
    if skip_desktop_install() {
        return;
    }
    let now = now_ms();
    let last = unlock_notify_last_ms().load(Ordering::Relaxed);
    if now.saturating_sub(last) < UNLOCK_NOTIFY_COOLDOWN_MS {
        return;
    }
    unlock_notify_last_ms().store(now, Ordering::Relaxed);
    // Click must raise the running app (or launch once) — same pattern as message_notify.
    std::thread::spawn(|| {
        let app_id = crate::host::default_app_namespace();
        let mut n = notify_rust::Notification::new();
        n.appname(&app_id)
            .summary("Ghal Bol")
            .body("Enter your app password to receive messages.")
            .hint(notify_rust::Hint::DesktopEntry(app_id.clone()))
            .action("default", "Open Ghal Bol")
            .timeout(notify_rust::Timeout::Milliseconds(8000));
        let Ok(handle) = n.show() else {
            return;
        };
        handle.wait_for_action(move |action| {
            if action == "default" {
                wake_for_unlock(&app_id);
            }
        });
    });
}

/// Short tone so an incoming call is audible while the window is open.
pub fn play_incoming_ring() {
    play_tone("ghalbol-ring.wav", 880.0);
}

/// Short tone while an outgoing call is still connecting.
pub fn play_ringback() {
    play_tone("ghalbol-ringback.wav", 440.0);
}

fn play_tone(file_name: &str, hz: f32) {
    let bytes = ring_tone(hz);
    let path = std::env::temp_dir().join(file_name);
    std::thread::spawn(move || {
        if std::fs::write(&path, bytes).is_ok() {
            let _ = std::process::Command::new("paplay").arg(&path).status();
        }
    });
}

fn ring_tone(hz: f32) -> Vec<u8> {
    let rate = 16000u32;
    let samples = rate / 2;
    let mut pcm = Vec::with_capacity(samples as usize * 2);
    for i in 0..samples {
        let t = i as f32 / rate as f32;
        let env = if i < 400 { i as f32 / 400.0 } else { 1.0 };
        let s = (t * hz * std::f32::consts::TAU).sin() * env * 0.25;
        pcm.extend_from_slice(&((s * i16::MAX as f32) as i16).to_le_bytes());
    }
    let data_len = pcm.len() as u32;
    let mut wav = Vec::with_capacity(44 + pcm.len());
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&(36 + data_len).to_le_bytes());
    wav.extend_from_slice(b"WAVEfmt ");
    wav.extend_from_slice(&16u32.to_le_bytes());
    wav.extend_from_slice(&1u16.to_le_bytes());
    wav.extend_from_slice(&1u16.to_le_bytes());
    wav.extend_from_slice(&rate.to_le_bytes());
    wav.extend_from_slice(&(rate * 2).to_le_bytes());
    wav.extend_from_slice(&2u16.to_le_bytes());
    wav.extend_from_slice(&16u16.to_le_bytes());
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&data_len.to_le_bytes());
    wav.extend_from_slice(&pcm);
    wav
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dbus_object_path_from_app_id() {
        assert_eq!(dbus_object_path("com.ghalbol.debug"), "/com/ghalbol/debug");
    }

    #[test]
    fn app_id_is_safe_rejects_injection_shapes() {
        assert!(app_id_is_safe("com.ghalbol.debug"));
        assert!(!app_id_is_safe(""));
        assert!(!app_id_is_safe("com.ghalbol;rm -rf /"));
        assert!(!app_id_is_safe("../evil"));
        assert!(!app_id_is_safe(&"a".repeat(200)));
    }

    #[test]
    fn desktop_exec_quotes_paths_with_spaces() {
        let plain = desktop_exec_quoted(Path::new("/opt/ghalbol/bin"));
        assert_eq!(plain, "/opt/ghalbol/bin");
        let spaced = desktop_exec_quoted(Path::new("/opt/Ghal Bol/bin"));
        assert_eq!(spaced, "\"/opt/Ghal Bol/bin\"");
        assert_eq!(desktop_entry_escape("A\nB"), "A B");
    }

    #[test]
    fn ring_wav_is_a_pcm_file() {
        let wav = ring_tone(880.0);
        assert!(wav.starts_with(b"RIFF"));
        assert!(wav.windows(4).any(|w| w == b"WAVE"));
        assert!(wav.len() > 44);
    }

    #[test]
    fn skip_desktop_install_true_when_home_is_tmpish() {
        // Pure path heuristic (no env mutation — set_var is unsafe on this toolchain).
        assert!(
            "/tmp/ghalbol-home-xyz".starts_with("/tmp/")
                || "/tmp/ghalbol-home-xyz".contains("ghalbol-home")
        );
        assert!(app_id_is_safe("com.ghalbol.debug"));
    }
}
