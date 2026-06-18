//! pegin-wasm — browser mini wallet (BIP39 → BLS keys, login signing, JWT mint).
//! `#[wasm_bindgen]` surface only; logic lives in `modules/`.
//!
//! The browser performs **no chain I/O** (feat-37): it exposes the watch-only account key
//! the relay resolves, and signs logins for the DID + owner index the relay returns.

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

/// Watch-only identity the relay resolves to `{ did, ownerIndex }` — no secret leaves WASM.
///
/// The browser performs **no chain I/O**; `POST /resolve { accountPk }` maps this key to
/// the wallet's DID on the relay.
///
/// * `keys` — BLS keys from [`deriveWalletKeys`](crate::derive_wallet_keys)
/// * returns `{ accountPk }` — the observer account public key (48-byte BLS hex)
#[wasm_bindgen(js_name = identityKey)]
pub fn identity_key(keys: &WalletKeys) -> JsValue {
    let obj = js_sys::Object::new();
    let _ = js_sys::Reflect::set(
        &obj,
        &JsValue::from_str("accountPk"),
        &JsValue::from_str(&keys.account_pk_hex()),
    );
    obj.into()
}

/// Proves possession for a relay-resolved DID: mints a JWT and signs the login challenge.
///
/// The relay returned `(did, ownerIndex)` from [`identityKey`]; this signs with the owner
/// key at that index so `cnf.did_pk` binds to the on-chain owner (feat-17). No chain I/O.
///
/// * `did` — canonical `did:chia` from `/resolve` (fills `iss`/`sub`)
/// * `owner_index` — DID-owning address index from `/resolve`
/// * `aud` — relying-party origin bound into the JWT
/// * `ttl_seconds` — JWT lifetime
/// * `nonce` — server challenge; when set, the result carries `challengeSig`
/// * returns `{ jwt, challengeSig? }` — no secret key exposed
#[wasm_bindgen(js_name = signLogin)]
// wasm-bindgen marshals JS `string | null` as an owned `Option<String>`, not `Option<&str>`.
#[allow(clippy::needless_pass_by_value)]
pub fn sign_login(
    keys: &WalletKeys,
    did: &str,
    owner_index: u32,
    aud: &str,
    ttl_seconds: u32,
    nonce: Option<String>,
) -> Result<JsValue, JsError> {
    let signed = modules::auth::service::sign_login_inner(
        keys,
        did,
        owner_index,
        aud,
        ttl_seconds,
        nonce.as_deref(),
    )
    .map_err(|e| JsError::new(&e))?;

    let obj = js_sys::Object::new();
    let set = |k: &str, v: &JsValue| {
        let _ = js_sys::Reflect::set(&obj, &JsValue::from_str(k), v);
    };
    set("jwt", &JsValue::from_str(&signed.jwt));
    if let Some(sig) = signed.challenge_sig {
        set("challengeSig", &JsValue::from_str(&sig));
    }
    Ok(obj.into())
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
