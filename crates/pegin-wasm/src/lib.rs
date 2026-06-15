//! pegin-wasm — browser mini wallet (BIP39 → BLS keys, DID lookup, JWT mint).
//! `#[wasm_bindgen]` surface only; logic lives in `modules/`.

mod modules;

#[cfg(test)]
#[path = "../test_vectors.rs"]
mod test_vectors;

#[cfg(test)]
mod test_util;

use wasm_bindgen::prelude::*;

pub use modules::keys::WalletKeys;

/// Smoke test: proves the WASM module initialised. Carries no wallet logic.
#[wasm_bindgen]
pub fn hello() -> String {
    "pegin-wasm ready".into()
}

/// Derives wallet and DID BLS keys from a BIP39 mnemonic. Deterministic.
///
/// * `mnemonic` — valid 12- or 24-word English BIP39 phrase
#[wasm_bindgen(js_name = deriveWalletKeys)]
pub fn derive_wallet_keys(mnemonic: &str) -> Result<WalletKeys, JsError> {
    derive_keys_export(mnemonic)
}

/// Alias for [`derive_wallet_keys`] (feat-11 export name).
#[wasm_bindgen(js_name = deriveKeys)]
pub fn derive_keys(mnemonic: &str) -> Result<WalletKeys, JsError> {
    derive_keys_export(mnemonic)
}

fn derive_keys_export(mnemonic: &str) -> Result<WalletKeys, JsError> {
    modules::keys::service::derive_wallet_keys_inner(mnemonic).map_err(|e| JsError::new(&e))
}

/// Signs a challenge string with the DID key (BLS `AugSchemeMPL`).
///
/// * `challenge` — server-issued nonce, proves DID ownership
/// * returns the 96-byte signature as lowercase hex
#[wasm_bindgen(js_name = signChallenge)]
pub fn sign_challenge(keys: &WalletKeys, challenge: &str) -> String {
    modules::signing::service::sign_challenge_inner(keys, challenge)
}

/// Signs a spend bundle with the wallet key.
///
/// * `bundle` — Chia streamable bytes
/// * returns the bundle with the signature aggregated in
#[wasm_bindgen(js_name = signSpendBundle)]
pub fn sign_spend_bundle(keys: &WalletKeys, bundle: &[u8]) -> Result<Vec<u8>, JsError> {
    modules::signing::service::sign_spend_bundle_inner(keys, bundle).map_err(|e| JsError::new(&e))
}

/// Resolves login identity for derived wallet keys — secrets never leave WASM.
///
/// Cache hit returns instantly; a first-login cache miss scans **public** coinset
/// hints for the address index that owns the DID, then caches it.
///
/// * `keys` — BLS keys from [`deriveWalletKeys`](crate::derive_wallet_keys)
/// * `scan_limit` — highest address index probed (`0` ⇒ default 10 000)
/// * returns `{ ownerIndex, ownerPk, did? }`; `did` is set once resolved on-chain
#[wasm_bindgen(js_name = lookupDid)]
pub async fn lookup_did(keys: &WalletKeys, scan_limit: u32) -> Result<JsValue, JsError> {
    modules::did_lookup::service::lookup_did_inner(keys, scan_limit_or_default(scan_limit))
        .await
        .map(|identity| identity_to_js(&identity))
        .map_err(|e| JsError::new(&e))
}

fn scan_limit_or_default(scan_limit: u32) -> u32 {
    if scan_limit == 0 {
        modules::did_lookup::DEFAULT_SCAN_LIMIT
    } else {
        scan_limit
    }
}

/// Caches the relay-resolved canonical DID so the next `lookupDid` returns it.
///
/// * `wallet_fp` — fingerprint from a prior [`loginWithSeed`] result
/// * `did` — canonical `did:chia` the auth relay resolved from `cnf.did_pk`
/// * `owner_index` — address key the relay confirmed as the DID owner
#[wasm_bindgen(js_name = rememberDid)]
// wasm-bindgen marshals JS strings as owned `String`, not `&str`.
#[allow(clippy::needless_pass_by_value)]
pub fn remember_did(wallet_fp: String, did: String, owner_index: u32) {
    modules::did_lookup::service::remember_did_inner(&wallet_fp, &did, owner_index);
}

fn identity_to_js(identity: &modules::did_lookup::ResolvedIdentity) -> JsValue {
    let obj = js_sys::Object::new();
    let set = |k: &str, v: &JsValue| {
        let _ = js_sys::Reflect::set(&obj, &JsValue::from_str(k), v);
    };
    set(
        "ownerIndex",
        &JsValue::from_f64(f64::from(identity.owner_index)),
    );
    set("ownerPk", &JsValue::from_str(&identity.owner_pk));
    if let Some(did) = &identity.did {
        set("did", &JsValue::from_str(did));
    }
    obj.into()
}

/// Mints a self-signed ES256K JWT with the DID key, bound to `aud`.
///
/// * `did` — fills the `iss` and `sub` claims
/// * `aud` — relying-party origin or client id
/// * `ttl_seconds` — lifetime from now; sets the `exp` claim
/// * `nonce` — optional server nonce embedded in the JWT for replay resistance
#[wasm_bindgen(js_name = mintJwt)]
// wasm-bindgen marshals JS `string | null` as an owned `Option<String>`, not `Option<&str>`.
#[allow(clippy::needless_pass_by_value)]
pub fn mint_jwt(
    keys: &WalletKeys,
    did: &str,
    aud: &str,
    ttl_seconds: u32,
    nonce: Option<String>,
) -> Result<String, JsError> {
    modules::jwt::service::mint_jwt_inner(keys, did, aud, ttl_seconds, nonce.as_deref())
        .map_err(|e| JsError::new(&e))
}

/// Verifies a JWT minted by [`mint_jwt`].
///
/// * `expected_aud` — must match the token `aud` claim
/// * `expected_nonce` — when set, must match the token `nonce` claim
/// * returns `false` for expired, tampered, or bad-signature tokens
#[wasm_bindgen(js_name = verifyJwt)]
// wasm-bindgen marshals JS `string | null` as an owned `Option<String>`, not `Option<&str>`.
#[allow(clippy::needless_pass_by_value)]
pub fn verify_jwt(token: &str, expected_aud: &str, expected_nonce: Option<String>) -> bool {
    modules::jwt::service::verify_jwt_inner(token, expected_aud, expected_nonce.as_deref())
        .unwrap_or(false)
}

/// Resolves identity and mints a JWT inside WASM — secrets never leave the browser.
///
/// * `mnemonic` — BIP39 seed phrase (never stored; callers should clear UI state immediately)
/// * `scan_limit` — highest address index probed on a first-login DID scan (`0` ⇒ default)
/// * `ttl_seconds` — JWT lifetime
/// * `aud` — relying-party origin bound into the JWT
/// * `challenge_nonce` — when set, signs the nonce with the owner key and embeds it in the JWT
/// * returns `{ did, jwt, challengeSig?, walletFp, ownerIndex }` — no secret keys exposed.
#[wasm_bindgen(js_name = loginWithSeed)]
// wasm-bindgen marshals JS `string | null` as an owned `Option<String>`, not `Option<&str>`.
#[allow(clippy::needless_pass_by_value)]
pub async fn login_with_seed(
    mnemonic: &str,
    scan_limit: u32,
    ttl_seconds: u32,
    aud: &str,
    challenge_nonce: Option<String>,
) -> Result<JsValue, JsError> {
    modules::auth::service::login_with_seed_inner(
        mnemonic,
        scan_limit_or_default(scan_limit),
        ttl_seconds,
        aud,
        challenge_nonce.as_deref(),
    )
    .await
    .map(login_session_to_js)
    .map_err(|e| JsError::new(&e))
}

fn login_session_to_js(session: modules::auth::service::LoginSession) -> JsValue {
    let obj = js_sys::Object::new();
    let set = |k: &str, v: &JsValue| {
        let _ = js_sys::Reflect::set(&obj, &JsValue::from_str(k), v);
    };
    set("did", &JsValue::from_str(&session.did));
    set("jwt", &JsValue::from_str(&session.jwt));
    set("walletFp", &JsValue::from_str(&session.wallet_fp));
    set(
        "ownerIndex",
        &JsValue::from_f64(f64::from(session.owner_index)),
    );
    if let Some(sig) = session.challenge_sig {
        set("challengeSig", &JsValue::from_str(&sig));
    }
    obj.into()
}

// Phase 2/3 hooks (next epics): deriveKeysFromPasskeyAssertion(assertion, credentialId),
// signWithExternalSigner(request) — see the feat-9 issue for signatures.
