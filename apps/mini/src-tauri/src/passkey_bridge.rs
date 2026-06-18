//! Opens the system browser for a WebAuthn PRF enrollment ceremony and returns the PRF secret.

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use axum::extract::State;
use axum::http::{HeaderValue, Method, StatusCode};
use axum::routing::{get, post};
use axum::{Json, Router};
use pegin_infrastructure::modules::seed_vault::{seal_with_prf, PasskeyBackup};
use serde::Deserialize;
use tokio::sync::{oneshot, Mutex};
use tower_http::cors::CorsLayer;

const ENROLL_TIMEOUT: Duration = Duration::from_secs(120);

#[derive(Clone)]
struct BridgeState {
    token: String,
    label: String,
    mnemonic: String,
    result: Arc<Mutex<Option<Result<PasskeyBackup, String>>>>,
    done: Arc<Mutex<Option<oneshot::Sender<()>>>>,
}

#[derive(Deserialize)]
struct CompleteBody {
    token: String,
    credential_id: String,
    prf_secret_b64: String,
}

#[derive(serde::Serialize)]
struct CompleteResponse {
    ok: bool,
}

/// Runs a localhost WebAuthn enrollment page in the system browser.
pub async fn enroll_via_browser(mnemonic: &str, label: &str) -> Result<PasskeyBackup, String> {
    let token = uuid::Uuid::new_v4().to_string();
    let result: Arc<Mutex<Option<Result<PasskeyBackup, String>>>> = Arc::new(Mutex::new(None));
    let (done_tx, done_rx) = oneshot::channel();
    let bridge = BridgeState {
        token: token.clone(),
        label: label.to_string(),
        mnemonic: mnemonic.to_string(),
        result: result.clone(),
        done: Arc::new(Mutex::new(Some(done_tx))),
    };

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .map_err(|e| e.to_string())?;
    let addr: SocketAddr = listener.local_addr().map_err(|e| e.to_string())?;

    let app = Router::new()
        .route("/", get(serve_page))
        .route("/complete", post(complete))
        .with_state(bridge)
        .layer(
            CorsLayer::new()
                .allow_origin(HeaderValue::from_static("null"))
                .allow_methods([Method::GET, Method::POST])
                .allow_headers([axum::http::header::CONTENT_TYPE]),
        );

    let result_for_server = result.clone();
    let server = tokio::spawn(async move {
        if let Err(e) = axum::serve(listener, app).await {
            let mut slot = result_for_server.lock().await;
            if slot.is_none() {
                *slot = Some(Err(format!("enrollment server failed: {e}")));
            }
        }
    });

    let url = format!("http://localhost:{}/?token={token}", addr.port());
    open::that(&url).map_err(|e| e.to_string())?;

    match tokio::time::timeout(ENROLL_TIMEOUT, done_rx).await {
        Ok(Ok(())) => {}
        Ok(Err(_)) => return Err("enrollment channel closed".into()),
        Err(_) => return Err("passkey enrollment timed out — complete the browser prompt".into()),
    }

    server.abort();

    let outcome = {
        let mut guard = result.lock().await;
        guard.take()
    };
    outcome.unwrap_or_else(|| Err("enrollment did not finish".into()))
}

async fn serve_page(State(state): State<BridgeState>) -> axum::response::Html<String> {
    let html = ENROLL_HTML
        .replace("__TOKEN__", &state.token)
        .replace("__LABEL__", &html_escape(&state.label));
    axum::response::Html(html)
}

/// POSTs the PRF-encrypted seed blob to the auth relay for browser login.
pub async fn sync_passkey_blob_to_relay(backup: &PasskeyBackup) -> Result<(), String> {
    let relay =
        std::env::var("PEGIN_AUTH_URL").unwrap_or_else(|_| "http://127.0.0.1:8787".to_owned());
    let url = format!("{}/passkey-blob", relay.trim_end_matches('/'));
    let body = serde_json::json!({
        "credentialId": backup.credential_id,
        "iv": backup.blob.iv,
        "ct": backup.blob.ct,
    });
    let resp = reqwest::Client::new()
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("relay sync failed: {e}"))?;
    if !resp.status().is_success() {
        return Err(format!("relay sync returned {}", resp.status()));
    }
    Ok(())
}

async fn complete(
    State(state): State<BridgeState>,
    Json(body): Json<CompleteBody>,
) -> Result<Json<CompleteResponse>, StatusCode> {
    if body.token != state.token {
        return Err(StatusCode::FORBIDDEN);
    }

    let secret = base64::Engine::decode(
        &base64::engine::general_purpose::STANDARD,
        &body.prf_secret_b64,
    )
    .map_err(|_| StatusCode::BAD_REQUEST)?;

    let blob =
        seal_with_prf(&secret, &state.mnemonic).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let backup = PasskeyBackup {
        credential_id: body.credential_id,
        label: state.label.clone(),
        blob,
    };

    *state.result.lock().await = Some(Ok(backup.clone()));
    if let Err(err) = sync_passkey_blob_to_relay(&backup).await {
        eprintln!("pegin-signer: passkey blob relay sync: {err}");
    }
    if let Some(tx) = state.done.lock().await.take() {
        let _ = tx.send(());
    }

    Ok(Json(CompleteResponse { ok: true }))
}

fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

const ENROLL_HTML: &str = r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <title>PEGIN Signer — add passkey backup</title>
  <style>
    body { font-family: system-ui, sans-serif; max-width: 32rem; margin: 3rem auto; padding: 0 1rem; }
    button { font-size: 1rem; padding: 0.75rem 1.25rem; cursor: pointer; }
    .err { color: #b00020; }
    .ok { color: #0a7; }
  </style>
</head>
<body>
  <h1>Add passkey backup</h1>
  <p>Your browser will prompt for a passkey (1Password, YubiKey, Touch ID, …). The seed stays in PEGIN Signer — only the PRF secret returns over localhost.</p>
  <p>Label: <strong>__LABEL__</strong></p>
  <button id="go">Create passkey backup</button>
  <p id="msg"></p>
  <script>
    const token = "__TOKEN__";
    const salt = new TextEncoder().encode("pegin-prf-v1");
    const msg = document.getElementById("msg");
    document.getElementById("go").onclick = async () => {
      msg.textContent = "Waiting for passkey…";
      msg.className = "";
      try {
        const rpId = window.location.hostname;
        const cred = await navigator.credentials.create({
          publicKey: {
            challenge: crypto.getRandomValues(new Uint8Array(32)),
            rp: { id: rpId, name: "PEGIN Signer" },
            user: { id: crypto.getRandomValues(new Uint8Array(16)), name: "__LABEL__", displayName: "__LABEL__" },
            pubKeyCredParams: [{ type: "public-key", alg: -7 }, { type: "public-key", alg: -257 }],
            authenticatorSelection: { residentKey: "required", userVerification: "preferred" },
            extensions: { prf: { eval: { first: salt } } }
          }
        });
        if (!cred) throw new Error("cancelled");
        // Always read PRF from an assertion. Create-time PRF results are unreliable and can differ
        // from get-time on some authenticators (1Password, platform), which would make the sealed
        // blob undecryptable at web login. Login uses get-time PRF, so enrollment must match.
        const assertion = await navigator.credentials.get({
          publicKey: {
            challenge: crypto.getRandomValues(new Uint8Array(32)),
            rpId: rpId,
            allowCredentials: [{ id: cred.rawId, type: "public-key" }],
            userVerification: "preferred",
            extensions: { prf: { eval: { first: salt } } }
          }
        });
        const secret = assertion?.getClientExtensionResults().prf?.results?.first;
        if (!secret) throw new Error("authenticator does not support PRF");
        const bytes = secret instanceof ArrayBuffer ? new Uint8Array(secret) : new Uint8Array(secret.buffer, secret.byteOffset, secret.byteLength);
        let binary = "";
        bytes.forEach(b => binary += String.fromCharCode(b));
        const prfSecretB64 = btoa(binary);
        const credId = btoa(String.fromCharCode(...new Uint8Array(cred.rawId)));
        const res = await fetch("/complete", {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ token, credential_id: credId, prf_secret_b64: prfSecretB64 })
        });
        if (!res.ok) throw new Error("signer rejected enrollment");
        msg.textContent = "Passkey backup added — you can close this tab.";
        msg.className = "ok";
      } catch (e) {
        msg.textContent = e.message || String(e);
        msg.className = "err";
      }
    };
  </script>
</body>
</html>"#;
