mod modules;

use wasm_bindgen::prelude::*;

pub use modules::keys::entities::WalletKeys;

// ── Key derivation ────────────────────────────────────────────────────────────

/// Derive wallet and DID BLS keys from a BIP39 mnemonic.
///
/// The mnemonic must be a valid 12- or 24-word English BIP39 phrase.
/// Same mnemonic always produces the same keys (deterministic).
#[wasm_bindgen(js_name = deriveWalletKeys)]
pub fn derive_wallet_keys(mnemonic: &str) -> Result<WalletKeys, JsError> {
    modules::keys::service::derive_wallet_keys_inner(mnemonic).map_err(|e| JsError::new(&e))
}

// ── Signing ───────────────────────────────────────────────────────────────────

/// Sign a challenge string with the DID key using BLS `AugSchemeMPL`.
///
/// Returns the 96-byte signature as lowercase hex. Use this to prove DID
/// ownership in high-assurance login flows.
#[wasm_bindgen(js_name = signChallenge)]
pub fn sign_challenge(keys: &WalletKeys, challenge: &str) -> String {
    modules::signing::service::sign_challenge_inner(keys, challenge)
}

/// Sign a spend bundle serialised as Chia streamable bytes.
///
/// Returns the bundle with the wallet key's BLS signature aggregated in.
/// See the pegin-wasm wiki for Phase 1 signing limitations.
#[wasm_bindgen(js_name = signSpendBundle)]
pub fn sign_spend_bundle(keys: &WalletKeys, bundle: &[u8]) -> Result<Vec<u8>, JsError> {
    modules::signing::service::sign_spend_bundle_inner(keys, bundle).map_err(|e| JsError::new(&e))
}

// ── DID verification ──────────────────────────────────────────────────────────

/// Verify a DID is active on-chain via coinset.org REST API.
///
/// `launcher_id_hex` — 64-char lowercase hex of the DID launcher coin ID.
/// `base_url` — optional coinset.org endpoint; defaults to testnet11.
///
/// Returns `"did:chia:<launcher_id_hex>"` on success.
// Must remain `async` so wasm-bindgen generates a JS Promise, even though the
// native (non-WASM) stub is sync. The WASM branch awaits a real HTTP fetch.
#[allow(clippy::unused_async)]
#[wasm_bindgen(js_name = getDid)]
pub async fn get_did(launcher_id_hex: &str, base_url: Option<String>) -> Result<String, JsError> {
    #[cfg(target_arch = "wasm32")]
    {
        modules::did::service::get_did_inner(launcher_id_hex, base_url.as_deref())
            .await
            .map_err(|e| JsError::new(&e))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        modules::did::service::get_did_inner(launcher_id_hex, base_url.as_deref())
            .map_err(|e| JsError::new(&e))
    }
}

// ── JWT (deferred) ────────────────────────────────────────────────────────────

/// Mint a self-signed JWT using the DID key.
///
/// NOT YET IMPLEMENTED — returns an error. See pegin-wasm wiki for the
/// planned ES256K approach (HKDF from BLS `did_sk` → k256 secp256k1 key).
#[wasm_bindgen(js_name = mintJwt)]
pub fn mint_jwt(keys: &WalletKeys, claims_json: &str) -> Result<String, JsError> {
    modules::jwt::service::mint_jwt_inner(keys, claims_json).map_err(|e| JsError::new(&e))
}

// ── Future login phase hooks (leave stubs visible for Phase 2 / Phase 3) ─────

// Phase 2: passkey / WebAuthn credential replaces mnemonic
// #[wasm_bindgen(js_name = deriveKeysFromPasskeyAssertion)]
// pub async fn derive_keys_from_passkey_assertion(
//     assertion_json: &str,
//     credential_id: &str,
// ) -> Result<WalletKeys, JsError> { ... }

// Phase 3: HSM / Chia Signer — signing delegated to an external device
// #[wasm_bindgen(js_name = signWithExternalSigner)]
// pub async fn sign_with_external_signer(request_json: &str) -> Result<String, JsError> { ... }
