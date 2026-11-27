//! Heavy host work off the UI thread (unlock, network start, invites, delivery, attachments).
//!
//! Results land in slots; [`start_ui_poll`] wakes the UI via `SignalToUI` so
//! `drain_events` can apply them without freezing the event loop.

use ghalbol_core::host::{self, UnlockedSession};
use makepad_widgets::makepad_platform::SignalToUI;
use std::sync::{Mutex, OnceLock};

fn wake_ui() {
    SignalToUI::set_ui_signal();
}

fn lock_slot<T>(m: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    match m.lock() {
        Ok(g) => g,
        Err(p) => p.into_inner(),
    }
}

// --- Unlock / create / import ---------------------------------------------

#[derive(Debug)]
pub enum UnlockOutcome {
    Session {
        session: UnlockedSession,
        creating: bool,
        net_status: String,
    },
    RelockOk {
        public_key_hex: String,
    },
    Err(String),
}

struct UnlockSlot {
    busy: bool,
    result: Option<UnlockOutcome>,
}

fn unlock_slot() -> &'static Mutex<UnlockSlot> {
    static S: OnceLock<Mutex<UnlockSlot>> = OnceLock::new();
    S.get_or_init(|| {
        Mutex::new(UnlockSlot {
            busy: false,
            result: None,
        })
    })
}

pub fn unlock_busy() -> bool {
    lock_slot(unlock_slot()).busy
}

/// Start Argon2 unlock (+ optional `start_network`) off the UI thread.
/// Returns false if a job is already running.
pub fn start_unlock(
    namespace: String,
    password: String,
    create_new: bool,
    algorithm: String,
    secret: String,
    relock_only: bool,
) -> bool {
    {
        let mut s = lock_slot(unlock_slot());
        if s.busy {
            return false;
        }
        s.busy = true;
        s.result = None;
    }
    let _ = std::thread::Builder::new()
        .name("ghalbol-unlock".into())
        .spawn(move || {
            let outcome = if relock_only {
                match host::reveal_secret(&namespace, &password) {
                    Ok(_) => UnlockOutcome::RelockOk {
                        public_key_hex: String::new(),
                    },
                    Err(e) => UnlockOutcome::Err(e),
                }
            } else {
                let creating = !host::keystore_exists(&namespace) && create_new;
                let secret_opt = if create_new {
                    None
                } else if secret.trim().is_empty() {
                    None
                } else {
                    Some(secret.as_str())
                };
                let algo = if host::keystore_exists(&namespace) {
                    None
                } else {
                    Some(algorithm.as_str())
                };
                match host::unlock_with_options(&namespace, &password, algo, secret_opt) {
                    Ok(session) => {
                        let net_status = if creating {
                            String::new()
                        } else {
                            match host::start_network(&session.app_namespace) {
                                Ok(_) => String::new(),
                                Err(e) => format!("Network: {e}"),
                            }
                        };
                        UnlockOutcome::Session {
                            session,
                            creating,
                            net_status,
                        }
                    }
                    Err(e) => UnlockOutcome::Err(e),
                }
            };
            {
                let mut s = lock_slot(unlock_slot());
                s.result = Some(outcome);
                s.busy = false;
            }
            wake_ui();
        });
    true
}

pub fn take_unlock_result() -> Option<UnlockOutcome> {
    lock_slot(unlock_slot()).result.take()
}

/// Fill RelockOk public key from the live session (worker does not know it).
pub fn patch_relock_pk(outcome: &mut UnlockOutcome, pk: String) {
    if let UnlockOutcome::RelockOk { public_key_hex } = outcome {
        *public_key_hex = pk;
    }
}

// --- Delivery mailbox summary ---------------------------------------------

struct DeliverySlot {
    busy: bool,
    result: Option<String>,
}

fn delivery_slot() -> &'static Mutex<DeliverySlot> {
    static S: OnceLock<Mutex<DeliverySlot>> = OnceLock::new();
    S.get_or_init(|| Mutex::new(DeliverySlot {
        busy: false,
        result: None,
    }))
}

pub fn delivery_busy() -> bool {
    lock_slot(delivery_slot()).busy
}

pub fn start_delivery_summary() -> bool {
    {
        let mut s = lock_slot(delivery_slot());
        if s.busy {
            return false;
        }
        s.busy = true;
        s.result = None;
    }
    let _ = std::thread::Builder::new()
        .name("ghalbol-delivery".into())
        .spawn(move || {
            let body = host::delivery_summary();
            {
                let mut s = lock_slot(delivery_slot());
                s.result = Some(body);
                s.busy = false;
            }
            wake_ui();
        });
    true
}

pub fn take_delivery_summary() -> Option<String> {
    lock_slot(delivery_slot()).result.take()
}

// --- Password change ------------------------------------------------------

struct PasswordSlot {
    busy: bool,
    result: Option<Result<String, String>>,
}

fn password_slot() -> &'static Mutex<PasswordSlot> {
    static S: OnceLock<Mutex<PasswordSlot>> = OnceLock::new();
    S.get_or_init(|| Mutex::new(PasswordSlot {
        busy: false,
        result: None,
    }))
}

pub fn password_busy() -> bool {
    lock_slot(password_slot()).busy
}

pub fn start_password_change(namespace: String, current: String, new_password: String) -> bool {
    {
        let mut s = lock_slot(password_slot());
        if s.busy {
            return false;
        }
        s.busy = true;
        s.result = None;
    }
    let _ = std::thread::Builder::new()
        .name("ghalbol-password".into())
        .spawn(move || {
            let result = host::change_password(&namespace, &current, &new_password).map(|()| {
                "Password changed. Export a new backup. Older backups still use the old password."
                    .into()
            });
            {
                let mut s = lock_slot(password_slot());
                s.result = Some(result);
                s.busy = false;
            }
            wake_ui();
        });
    true
}

pub fn take_password_change() -> Option<Result<String, String>> {
    lock_slot(password_slot()).result.take()
}

// --- Extend mailbox TTL ---------------------------------------------------

struct TtlSlot {
    busy: bool,
    result: Option<Result<String, String>>,
}

fn ttl_slot() -> &'static Mutex<TtlSlot> {
    static S: OnceLock<Mutex<TtlSlot>> = OnceLock::new();
    S.get_or_init(|| Mutex::new(TtlSlot {
        busy: false,
        result: None,
    }))
}

pub fn start_extend_ttl(mid: String) -> bool {
    {
        let mut s = lock_slot(ttl_slot());
        if s.busy {
            return false;
        }
        s.busy = true;
        s.result = None;
    }
    let _ = std::thread::Builder::new()
        .name("ghalbol-ttl".into())
        .spawn(move || {
            let result = host::extend_mailbox_ttl(&mid).map(|()| "TTL extended".into());
            {
                let mut s = lock_slot(ttl_slot());
                s.result = Some(result);
                s.busy = false;
            }
            wake_ui();
        });
    true
}

pub fn take_extend_ttl() -> Option<Result<String, String>> {
    lock_slot(ttl_slot()).result.take()
}

// --- Accept invite (paste / deep link / QR text) --------------------------------

pub struct InviteJoinResult {
    pub raw: String,
    pub result: Result<String, String>,
}

struct InviteSlot {
    busy: bool,
    result: Option<InviteJoinResult>,
}

fn invite_slot() -> &'static Mutex<InviteSlot> {
    static S: OnceLock<Mutex<InviteSlot>> = OnceLock::new();
    S.get_or_init(|| Mutex::new(InviteSlot {
        busy: false,
        result: None,
    }))
}

pub fn invite_busy() -> bool {
    lock_slot(invite_slot()).busy
}

pub fn start_accept_invite(namespace: String, raw: String) -> bool {
    if raw.trim().is_empty() {
        return false;
    }
    {
        let mut s = lock_slot(invite_slot());
        if s.busy {
            return false;
        }
        s.busy = true;
        s.result = None;
    }
    let _ = std::thread::Builder::new()
        .name("ghalbol-invite".into())
        .spawn(move || {
            let result = host::accept_invite(&namespace, raw.trim());
            let out = InviteJoinResult { raw, result };
            {
                let mut s = lock_slot(invite_slot());
                s.result = Some(out);
                s.busy = false;
            }
            wake_ui();
        });
    true
}

pub fn take_accept_invite() -> Option<InviteJoinResult> {
    lock_slot(invite_slot()).result.take()
}

// --- start_network (can wait on p2p stop join) ----------------------------------

struct NetworkSlot {
    busy: bool,
    result: Option<Result<(), String>>,
}

fn network_slot() -> &'static Mutex<NetworkSlot> {
    static S: OnceLock<Mutex<NetworkSlot>> = OnceLock::new();
    S.get_or_init(|| Mutex::new(NetworkSlot {
        busy: false,
        result: None,
    }))
}

pub fn start_network_bg(namespace: String) -> bool {
    {
        let mut s = lock_slot(network_slot());
        if s.busy {
            return false;
        }
        s.busy = true;
        s.result = None;
    }
    let _ = std::thread::Builder::new()
        .name("ghalbol-network".into())
        .spawn(move || {
            let result = host::start_network(&namespace).map(|_| ());
            {
                let mut s = lock_slot(network_slot());
                s.result = Some(result);
                s.busy = false;
            }
            wake_ui();
        });
    true
}

pub fn take_start_network() -> Option<Result<(), String>> {
    lock_slot(network_slot()).result.take()
}

// --- QR picture decode + join (image decode off UI thread) ----------------------

struct QrJoinSlot {
    busy: bool,
    result: Option<Result<String, String>>,
}

fn qr_join_slot() -> &'static Mutex<QrJoinSlot> {
    static S: OnceLock<Mutex<QrJoinSlot>> = OnceLock::new();
    S.get_or_init(|| Mutex::new(QrJoinSlot {
        busy: false,
        result: None,
    }))
}

pub fn qr_join_busy() -> bool {
    lock_slot(qr_join_slot()).busy
}

pub fn start_qr_picture_join(namespace: String, path: String) -> bool {
    {
        let mut s = lock_slot(qr_join_slot());
        if s.busy {
            return false;
        }
        s.busy = true;
        s.result = None;
    }
    let _ = std::thread::Builder::new()
        .name("ghalbol-qr-join".into())
        .spawn(move || {
            let result =
                host::decode_qr_file(&path).and_then(|text| host::accept_invite(&namespace, &text));
            {
                let mut s = lock_slot(qr_join_slot());
                s.result = Some(result);
                s.busy = false;
            }
            wake_ui();
        });
    true
}

pub fn take_qr_picture_join() -> Option<Result<String, String>> {
    lock_slot(qr_join_slot()).result.take()
}

// --- Attachment tap-to-download (host RPC + p2p lock off UI thread) -------------

pub struct AttachmentFetchJob {
    pub result: Result<String, String>,
}

struct AttachFetchSlot {
    busy: bool,
    result: Option<AttachmentFetchJob>,
}

fn attach_fetch_slot() -> &'static Mutex<AttachFetchSlot> {
    static S: OnceLock<Mutex<AttachFetchSlot>> = OnceLock::new();
    S.get_or_init(|| Mutex::new(AttachFetchSlot {
        busy: false,
        result: None,
    }))
}

pub fn attachment_fetch_busy() -> bool {
    lock_slot(attach_fetch_slot()).busy
}

pub fn start_attachment_fetch(peer: String, offer_id: String) -> bool {
    if peer.trim().is_empty() || offer_id.trim().is_empty() {
        return false;
    }
    {
        let mut s = lock_slot(attach_fetch_slot());
        if s.busy {
            return false;
        }
        s.busy = true;
        s.result = None;
    }
    let _ = std::thread::Builder::new()
        .name("ghalbol-attach".into())
        .spawn(move || {
            let result = host::fetch_attachment(&peer, &offer_id);
            let out = AttachmentFetchJob { result };
            {
                let mut s = lock_slot(attach_fetch_slot());
                s.result = Some(out);
                s.busy = false;
            }
            wake_ui();
        });
    true
}

pub fn take_attachment_fetch() -> Option<AttachmentFetchJob> {
    lock_slot(attach_fetch_slot()).result.take()
}
