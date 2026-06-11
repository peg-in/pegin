//! pegin-wasm — browser mini wallet (BIP39 → BLS keys, DID lookup, JWT mint).
//! `#[wasm_bindgen]` surface only; logic lives in `modules/`.

mod modules;

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

/// Verifies a DID launcher coin exists on-chain via coinset.org.
///
/// * `did_or_launcher_id` — `did:chia:1...` bech32m or 64-char launcher hex
/// * `base_url` — coinset endpoint; defaults to testnet11
/// * returns the canonical bech32m DID string
// async on both targets so wasm-bindgen emits a JS Promise; the native stub is sync.
#[allow(clippy::unused_async)]
#[wasm_bindgen(js_name = getDid)]
pub async fn get_did(
    did_or_launcher_id: &str,
    base_url: Option<String>,
) -> Result<String, JsError> {
    #[cfg(target_arch = "wasm32")]
    {
        modules::did::service::get_did_inner(did_or_launcher_id, base_url.as_deref())
            .await
            .map_err(|e| JsError::new(&e))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        modules::did::service::get_did_inner(did_or_launcher_id, base_url.as_deref())
            .map_err(|e| JsError::new(&e))
    }
}

/// Mints a self-signed JWT with the DID key (`BLS12381_G2`). Infallible.
///
/// * `did` — fills the `iss` and `sub` claims
/// * `aud` — relying-party audience
/// * `ttl_seconds` — lifetime from now; sets the `exp` claim
#[wasm_bindgen(js_name = mintJwt)]
pub fn mint_jwt(keys: &WalletKeys, did: &str, aud: &str, ttl_seconds: u32) -> String {
    modules::jwt::service::mint_jwt_inner(keys, did, aud, ttl_seconds)
}

/// Verifies a JWT minted by `mintJwt`.
///
/// * `did_pk_hex` — 96-char hex DID public key (`WalletKeys.didPkHex`)
/// * returns `false` for expired, missing-`exp`, or bad-signature tokens
/// * throws on malformed tokens
#[wasm_bindgen(js_name = verifyJwt)]
pub fn verify_jwt(token: &str, did_pk_hex: &str) -> Result<bool, JsError> {
    modules::jwt::service::verify_jwt_inner(token, did_pk_hex).map_err(|e| JsError::new(&e))
}

// Phase 2/3 hooks (next epics): deriveKeysFromPasskeyAssertion(assertion, credentialId),
// signWithExternalSigner(request) — see the feat-9 issue for signatures.
