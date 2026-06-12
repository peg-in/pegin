//! PEGIN auth backend (single source of truth). Issues login nonces, verifies the JWT +
//! challenge + on-chain DID **ownership**, and manages an `HttpOnly` session cookie.
//!
//! Mount behind the demo at `/api/pegin` (the Vite proxy strips the prefix). Routes:
//! `POST /nonce`, `POST /session`, `GET /session`, `POST /logout`.
//!
//! Env: `PEGIN_AUTH_PORT` (default 8787), `PEGIN_COINSET_URL` (default testnet11),
//! `PEGIN_SESSION_TTL` seconds (default 3600).

// Binary entry point: a poisoned mutex or failed bind is unrecoverable, so `expect` panics
// are the correct response here (unlike library code, which must return errors).
#![allow(clippy::expect_used)]

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use axum::extract::State;
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::json;

use pegin_verify::did::CoinsetClient;
use pegin_verify::{verify_login, VerifyError, VerifyLoginInput};

const SESSION_COOKIE: &str = "pegin_session";
const NONCE_TTL_SECS: u64 = 300;

struct Pending {
    nonce: String,
    aud: String,
    expires_at: u64,
}

struct Session {
    did: String,
    expires_at: u64,
}

struct AppState {
    pending: Mutex<HashMap<String, Pending>>,
    sessions: Mutex<HashMap<String, Session>>,
    coinset: CoinsetClient,
    session_ttl: u64,
}

#[derive(Serialize)]
struct NonceResponse {
    #[serde(rename = "loginId")]
    login_id: String,
    nonce: String,
    aud: String,
}

#[derive(Deserialize)]
struct CompleteLoginBody {
    #[serde(rename = "loginId")]
    login_id: String,
    jwt: String,
    #[serde(rename = "challengeSig")]
    challenge_sig: Option<String>,
}

#[derive(Serialize)]
struct SessionResponse {
    did: String,
    sub: String,
    #[serde(rename = "expiresAt")]
    expires_at: u64,
}

#[tokio::main]
async fn main() {
    let port: u16 = env_parse("PEGIN_AUTH_PORT", 8787);
    let coinset_url = std::env::var("PEGIN_COINSET_URL")
        .unwrap_or_else(|_| "https://testnet11.api.coinset.org".to_owned());
    let session_ttl = env_parse("PEGIN_SESSION_TTL", 3600);

    let state = Arc::new(AppState {
        pending: Mutex::new(HashMap::new()),
        sessions: Mutex::new(HashMap::new()),
        coinset: CoinsetClient::new(coinset_url),
        session_ttl,
    });

    let app = Router::new()
        .route("/nonce", post(handle_nonce))
        .route("/session", post(handle_session).get(handle_get_session))
        .route("/logout", post(handle_logout))
        .with_state(state);

    let addr = format!("127.0.0.1:{port}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|e| panic!("pegin-auth-server: cannot bind {addr}: {e}"));
    // Startup line read by the demo plugin to know the server is ready.
    println!("pegin-auth-server listening on http://{addr}");
    axum::serve(listener, app)
        .await
        .expect("pegin-auth-server failed");
}

async fn handle_nonce(State(state): State<Arc<AppState>>, headers: HeaderMap) -> impl IntoResponse {
    purge_expired(&state);
    let aud = audience(&headers);
    let nonce = random_hex(32);
    let login_id = random_hex(16);
    state.pending.lock().expect("lock").insert(
        login_id.clone(),
        Pending {
            nonce: nonce.clone(),
            aud: aud.clone(),
            expires_at: now_secs() + NONCE_TTL_SECS,
        },
    );
    Json(NonceResponse {
        login_id,
        nonce,
        aud,
    })
    .into_response()
}

async fn handle_session(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(body): Json<CompleteLoginBody>,
) -> impl IntoResponse {
    purge_expired(&state);

    let pending = state.pending.lock().expect("lock").remove(&body.login_id);
    let Some(pending) = pending.filter(|p| p.expires_at >= now_secs()) else {
        return error(StatusCode::UNAUTHORIZED, "login expired — start again");
    };
    if audience(&headers) != pending.aud {
        return error(StatusCode::FORBIDDEN, "audience mismatch");
    }

    let verified = verify_login(VerifyLoginInput {
        jwt: &body.jwt,
        expected_aud: &pending.aud,
        challenge_nonce: Some(&pending.nonce),
        challenge_sig_hex: body.challenge_sig.as_deref(),
        coinset: Some(state.coinset.clone()),
        now: None,
    })
    .await;

    let verified = match verified {
        Ok(v) => v,
        // A rejected login is the client's fault (forged/non-owner token) → 401, never 500.
        Err(VerifyError::Coinset(msg)) => {
            return error(StatusCode::BAD_GATEWAY, &format!("coinset: {msg}"))
        }
        Err(_) => return error(StatusCode::UNAUTHORIZED, "login verification failed"),
    };

    let session_id = random_hex(24);
    let expires_at = now_secs() + state.session_ttl;
    state.sessions.lock().expect("lock").insert(
        session_id.clone(),
        Session {
            did: verified.did.clone(),
            expires_at,
        },
    );

    let cookie = set_cookie(&session_id, state.session_ttl, is_secure(&headers));
    (
        StatusCode::OK,
        [(header::SET_COOKIE, cookie)],
        Json(SessionResponse {
            did: verified.did.clone(),
            sub: verified.did,
            expires_at,
        }),
    )
        .into_response()
}

async fn handle_get_session(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    purge_expired(&state);
    let Some(id) = cookie_value(&headers, SESSION_COOKIE) else {
        return error(StatusCode::UNAUTHORIZED, "not authenticated");
    };
    let guard = state.sessions.lock().expect("lock");
    match guard.get(&id).filter(|s| s.expires_at >= now_secs()) {
        Some(s) => Json(SessionResponse {
            did: s.did.clone(),
            sub: s.did.clone(),
            expires_at: s.expires_at,
        })
        .into_response(),
        None => error(StatusCode::UNAUTHORIZED, "not authenticated"),
    }
}

async fn handle_logout(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if let Some(id) = cookie_value(&headers, SESSION_COOKIE) {
        state.sessions.lock().expect("lock").remove(&id);
    }
    (
        StatusCode::NO_CONTENT,
        [(header::SET_COOKIE, clear_cookie(is_secure(&headers)))],
    )
        .into_response()
}

fn error(status: StatusCode, message: &str) -> axum::response::Response {
    (status, Json(json!({ "error": message }))).into_response()
}

fn audience(headers: &HeaderMap) -> String {
    if let Some(origin) = header_str(headers, "origin") {
        return origin;
    }
    let host = header_str(headers, "host").unwrap_or_else(|| "localhost".to_owned());
    let proto = header_str(headers, "x-forwarded-proto").unwrap_or_else(|| "http".to_owned());
    format!("{proto}://{host}")
}

fn is_secure(headers: &HeaderMap) -> bool {
    header_str(headers, "x-forwarded-proto").as_deref() == Some("https")
}

fn header_str(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get(name)
        .and_then(|v| v.to_str().ok())
        .map(ToOwned::to_owned)
}

fn cookie_value(headers: &HeaderMap, name: &str) -> Option<String> {
    let cookies = header_str(headers, "cookie")?;
    cookies.split(';').find_map(|part| {
        let (key, value) = part.trim().split_once('=')?;
        (key == name).then(|| value.to_owned())
    })
}

fn set_cookie(value: &str, max_age: u64, secure: bool) -> String {
    let base =
        format!("{SESSION_COOKIE}={value}; Path=/; HttpOnly; SameSite=Strict; Max-Age={max_age}");
    if secure {
        format!("{base}; Secure")
    } else {
        base
    }
}

fn clear_cookie(secure: bool) -> String {
    set_cookie("", 0, secure)
}

fn purge_expired(state: &AppState) {
    let now = now_secs();
    state
        .pending
        .lock()
        .expect("lock")
        .retain(|_, p| p.expires_at >= now);
    state
        .sessions
        .lock()
        .expect("lock")
        .retain(|_, s| s.expires_at >= now);
}

fn env_parse<T: std::str::FromStr>(key: &str, default: T) -> T {
    std::env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn random_hex(bytes: usize) -> String {
    let mut buf = vec![0u8; bytes];
    getrandom::getrandom(&mut buf).expect("system RNG");
    hex::encode(buf)
}
