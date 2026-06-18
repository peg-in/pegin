//! PEGIN Signer — seed vault, passkey backups, and Sage-style sign requests from web apps.

use std::sync::Mutex;

use chia_protocol::SpendBundle;
use chia_traits::streamable::Streamable;
use pegin_infrastructure::modules::chia::service::{ChiaGateway, CoinsetGateway};
use pegin_infrastructure::modules::seed_vault::{FileSealStore, SeedVault};
use rand::RngCore;
use tauri::{Emitter, Manager};

mod device_key;
mod passkey_bridge;
mod relay_client;
mod signing;

use relay_client::SignRequestDto;

const SIGN_REQUEST_EVENT: &str = "sign-request-pending";

#[derive(serde::Serialize, Clone)]
struct VaultStatusDto {
    sealed: bool,
    name: Option<String>,
    has_device_unlock: bool,
    passkey_count: usize,
}

#[derive(serde::Serialize)]
struct SessionStatusDto {
    unlocked: bool,
}

#[derive(serde::Serialize, Clone)]
struct PasskeyBackupDto {
    credential_id: String,
    label: String,
    relay_synced: bool,
}

#[derive(serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct PendingSignRequestDto {
    request_id: String,
    kind: String,
    origin: String,
    summary: String,
    message: Option<String>,
    return_url: Option<String>,
}

struct VaultSession {
    mnemonic: zeroize::Zeroizing<String>,
}

struct PendingSign {
    request: SignRequestDto,
    relay_url: String,
}

impl Clone for PendingSign {
    fn clone(&self) -> Self {
        Self {
            request: self.request.clone(),
            relay_url: self.relay_url.clone(),
        }
    }
}

struct AppState {
    session: Mutex<Option<VaultSession>>,
    pending_sign: Mutex<Option<PendingSign>>,
}

fn local_vault(app: &tauri::AppHandle) -> Result<SeedVault<FileSealStore>, String> {
    let dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    Ok(SeedVault::new(FileSealStore::new(
        dir.join("seed-vault.json"),
    )))
}

fn with_session<F, T>(state: &AppState, f: F) -> Result<T, String>
where
    F: FnOnce(&str) -> Result<T, String>,
{
    let guard = state
        .session
        .lock()
        .map_err(|_| "session lock poisoned".to_string())?;
    let session = guard
        .as_ref()
        .ok_or_else(|| "unlock the vault first".to_string())?;
    f(session.mnemonic.as_str())
}

fn emit_pending_sign(app: &tauri::AppHandle, request: &SignRequestDto) {
    let dto = PendingSignRequestDto {
        request_id: request.request_id.clone(),
        kind: request.kind.clone(),
        origin: request.origin.clone(),
        summary: request.summary.clone(),
        message: request.message.clone(),
        return_url: request.return_url.clone(),
    };
    let _ = app.emit(SIGN_REQUEST_EVENT, dto);
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    }
}

async fn load_sign_request(
    app: tauri::AppHandle,
    state: &AppState,
    request_id: String,
    relay_url: String,
) -> Result<(), String> {
    let request = relay_client::fetch_sign_request(&relay_url, &request_id).await?;
    if request.status != "pending" {
        return Err(format!("request is already {}", request.status));
    }
    emit_pending_sign(&app, &request);
    let mut slot = state
        .pending_sign
        .lock()
        .map_err(|_| "sign lock poisoned".to_string())?;
    *slot = Some(PendingSign { request, relay_url });
    Ok(())
}

#[tauri::command]
fn vault_status(app: tauri::AppHandle) -> Result<VaultStatusDto, String> {
    let status = local_vault(&app)?.status().map_err(|e| e.to_string())?;
    Ok(VaultStatusDto {
        sealed: status.sealed,
        name: status.name,
        has_device_unlock: status.sealed && device_key::has_device_key(&app),
        passkey_count: status.passkey_count,
    })
}

#[tauri::command]
fn session_status(state: tauri::State<'_, AppState>) -> SessionStatusDto {
    let unlocked = state
        .session
        .lock()
        .map(|guard| guard.is_some())
        .unwrap_or(false);
    SessionStatusDto { unlocked }
}

#[tauri::command]
fn get_pending_sign_request(
    state: tauri::State<'_, AppState>,
) -> Result<Option<PendingSignRequestDto>, String> {
    let guard = state
        .pending_sign
        .lock()
        .map_err(|_| "sign lock poisoned".to_string())?;
    Ok(guard.as_ref().map(|pending| PendingSignRequestDto {
        request_id: pending.request.request_id.clone(),
        kind: pending.request.kind.clone(),
        origin: pending.request.origin.clone(),
        summary: pending.request.summary.clone(),
        message: pending.request.message.clone(),
        return_url: pending.request.return_url.clone(),
    }))
}

#[tauri::command]
async fn open_sign_request(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    request_id: String,
    relay_url: String,
) -> Result<(), String> {
    load_sign_request(app, &state, request_id, relay_url).await
}

#[tauri::command]
async fn approve_sign_request(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let pending = state
        .pending_sign
        .lock()
        .map_err(|_| "sign lock poisoned".to_string())?
        .as_ref()
        .cloned()
        .ok_or_else(|| "no pending sign request".to_string())?;

    let mnemonic = {
        let guard = state
            .session
            .lock()
            .map_err(|_| "session lock poisoned".to_string())?;
        guard
            .as_ref()
            .map(|session| session.mnemonic.to_string())
            .ok_or_else(|| "unlock the vault first".to_string())?
    };

    let (signed_bundle_b64, message_sig_hex, tx_submitted) = match pending.request.kind.as_str() {
        "signMessage" => {
            let message = pending
                .request
                .message
                .as_deref()
                .ok_or_else(|| "request missing message".to_string())?;
            let sig = signing::sign_message(&mnemonic, message)?;
            (None, Some(sig), false)
        }
        "signSpendBundle" => {
            let bundle_b64 = pending
                .request
                .spend_bundle_b64
                .as_deref()
                .ok_or_else(|| "request missing spend bundle".to_string())?;
            let bundle_bytes =
                base64::Engine::decode(&base64::engine::general_purpose::STANDARD, bundle_b64)
                    .map_err(|e| e.to_string())?;
            let signed = signing::sign_spend_bundle(&mnemonic, &bundle_bytes)?;
            let signed_b64 =
                base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &signed);
            let tx_submitted = broadcast_signed_bundle(&signed).await.is_ok();
            (Some(signed_b64), None, tx_submitted)
        }
        other => return Err(format!("unsupported request kind: {other}")),
    };

    relay_client::complete_sign_request(
        &pending.relay_url,
        &pending.request.request_id,
        signed_bundle_b64,
        message_sig_hex,
        tx_submitted,
    )
    .await?;

    if let Some(return_url) = pending.request.return_url.as_deref() {
        let sep = if return_url.contains('?') { '&' } else { '?' };
        let back = format!(
            "{return_url}{sep}peginRequestId={}&status=ok&txSubmitted={tx_submitted}",
            pending.request.request_id
        );
        let _ = open::that(back);
    }

    if let Ok(mut slot) = state.pending_sign.lock() {
        *slot = None;
    }
    let _ = app.emit("sign-request-done", ());
    Ok(())
}

#[tauri::command]
async fn reject_sign_request(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let pending = state
        .pending_sign
        .lock()
        .map_err(|_| "sign lock poisoned".to_string())?
        .take()
        .ok_or_else(|| "no pending sign request".to_string())?;
    relay_client::reject_sign_request(&pending.relay_url, &pending.request.request_id).await
}

async fn broadcast_signed_bundle(signed: &[u8]) -> Result<(), String> {
    let bundle =
        SpendBundle::from_bytes(signed).map_err(|e| format!("invalid signed bundle: {e}"))?;
    CoinsetGateway::from_env()
        .submit_transaction(bundle)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn generate_seed(word_count: usize) -> Result<String, String> {
    pegin_infrastructure::modules::seed_vault::generate_mnemonic(word_count)
        .map_err(|e| e.to_string())
}

fn generate_device_key() -> String {
    let mut key_bytes = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut key_bytes);
    hex::encode(key_bytes)
}

fn open_session(state: &AppState, mnemonic: String) -> Result<(), String> {
    let mut guard = state
        .session
        .lock()
        .map_err(|_| "session lock poisoned".to_string())?;
    *guard = Some(VaultSession {
        mnemonic: zeroize::Zeroizing::new(mnemonic),
    });
    Ok(())
}

#[tauri::command]
fn seal_seed(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    mnemonic: String,
    name: Option<String>,
    preserve_passkeys: Option<bool>,
) -> Result<(), String> {
    let vault = local_vault(&app)?;
    let passkeys_to_restore = if preserve_passkeys.unwrap_or(false) {
        vault.list_passkeys().unwrap_or_default()
    } else {
        Vec::new()
    };

    let device_key = generate_device_key();
    device_key::store_device_key(&app, &device_key)?;
    vault
        .seal(&mnemonic, &device_key, name.as_deref())
        .map_err(|e| e.to_string())?;
    for backup in passkeys_to_restore {
        vault
            .push_passkey_backup(backup)
            .map_err(|e| e.to_string())?;
    }
    let mnemonic = vault
        .unseal_device_key(&device_key)
        .map_err(|e| e.to_string())?;
    open_session(&state, mnemonic)
}

#[tauri::command]
fn unlock_vault(app: tauri::AppHandle, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let device_key = device_key::read_device_key(&app)?;
    let mnemonic = local_vault(&app)?
        .unseal_device_key(&device_key)
        .map_err(|e| e.to_string())?;
    open_session(&state, mnemonic)
}

#[tauri::command]
fn lock_vault(state: tauri::State<'_, AppState>) {
    if let Ok(mut guard) = state.session.lock() {
        *guard = None;
    }
}

#[tauri::command]
fn copy_mnemonic(state: tauri::State<'_, AppState>) -> Result<(), String> {
    with_session(&state, |mnemonic| {
        arboard::Clipboard::new()
            .and_then(|mut clip| clip.set_text(mnemonic))
            .map_err(|e| e.to_string())
    })
}

#[tauri::command]
fn get_mnemonic(state: tauri::State<'_, AppState>) -> Result<String, String> {
    with_session(&state, |mnemonic| Ok(mnemonic.to_string()))
}

#[tauri::command]
fn list_passkey_backups(app: tauri::AppHandle) -> Result<Vec<PasskeyBackupDto>, String> {
    local_vault(&app)?
        .list_passkeys()
        .map(|items| {
            items
                .into_iter()
                .map(|item| PasskeyBackupDto {
                    credential_id: item.credential_id,
                    label: item.label,
                    relay_synced: false,
                })
                .collect()
        })
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn add_passkey_backup(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    label: String,
) -> Result<PasskeyBackupDto, String> {
    let mnemonic = {
        let guard = state
            .session
            .lock()
            .map_err(|_| "session lock poisoned".to_string())?;
        guard
            .as_ref()
            .map(|session| session.mnemonic.to_string())
            .ok_or_else(|| "unlock the vault first".to_string())?
    };

    let backup = passkey_bridge::enroll_via_browser(&mnemonic, &label).await?;
    local_vault(&app)?
        .push_passkey_backup(backup.clone())
        .map_err(|e| e.to_string())?;
    let relay_synced = passkey_bridge::sync_passkey_blob_to_relay(&backup)
        .await
        .is_ok();

    Ok(PasskeyBackupDto {
        credential_id: backup.credential_id,
        label: backup.label,
        relay_synced,
    })
}

#[tauri::command]
async fn resync_passkey_to_relay(
    app: tauri::AppHandle,
    credential_id: String,
) -> Result<(), String> {
    let vault = local_vault(&app)?;
    let backup = vault
        .list_passkeys()
        .map_err(|e| e.to_string())?
        .into_iter()
        .find(|item| item.credential_id == credential_id)
        .ok_or_else(|| "passkey not found on this vault".to_string())?;
    passkey_bridge::sync_passkey_blob_to_relay(&backup).await
}

/// Routes any `pegin-signer://sign?…` deep links found in `args` to the UI as an event.
/// `args` may be process argv (cold start / single-instance relaunch) or full URLs (on_open_url).
fn route_deep_links(app: &tauri::AppHandle, args: &[String]) {
    for arg in args {
        if let Some((request_id, relay_url)) = relay_client::parse_sign_deep_link(arg) {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
            let _ = app.emit(
                "open-sign-request",
                serde_json::json!({ "requestId": request_id, "relayUrl": relay_url }),
            );
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // single-instance must be registered first; it forwards a deep-link relaunch's argv here.
        .plugin(tauri_plugin_single_instance::init(|app, argv, _cwd| {
            route_deep_links(app, &argv);
        }))
        .plugin(tauri_plugin_deep_link::init())
        .manage(AppState {
            session: Mutex::new(None),
            pending_sign: Mutex::new(None),
        })
        .setup(|app| {
            // Cold start: the OS launched us with the deep link as an argument.
            route_deep_links(&app.handle().clone(), &std::env::args().collect::<Vec<_>>());
            #[cfg(desktop)]
            {
                use tauri_plugin_deep_link::DeepLinkExt;
                let handle = app.handle().clone();
                app.deep_link().on_open_url(move |event| {
                    let urls: Vec<String> = event.urls().iter().map(|u| u.to_string()).collect();
                    route_deep_links(&handle, &urls);
                });
                // Dev convenience: point the `pegin-signer` scheme at this binary so the OS routes
                // it here. Installed builds register the scheme from tauri.conf.json.
                let _ = app.deep_link().register("pegin-signer");
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            vault_status,
            session_status,
            get_pending_sign_request,
            open_sign_request,
            approve_sign_request,
            reject_sign_request,
            generate_seed,
            seal_seed,
            unlock_vault,
            lock_vault,
            copy_mnemonic,
            get_mnemonic,
            list_passkey_backups,
            add_passkey_backup,
            resync_passkey_to_relay,
        ])
        .run(tauri::generate_context!())
        .expect("error while running PEGIN Signer");
}
