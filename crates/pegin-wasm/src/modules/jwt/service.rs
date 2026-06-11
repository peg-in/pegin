//! Self-signed JWT mint/verify with the DID BLS key (wallet-as-IdP).

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chia_bls::{sign, verify, PublicKey, Signature};
use serde_json::{json, Value};

use crate::modules::keys::WalletKeys;

/// Fixed JOSE header — BLS over G2 is the only Phase 1 algorithm.
const JWT_HEADER: &str = r#"{"alg":"BLS12381_G2","typ":"JWT"}"#;

/// Mints a compact JWT `header.payload.signature` (base64url). Infallible.
///
/// * `did` — fills the `iss` and `sub` claims
/// * `aud` — audience claim
/// * `ttl_seconds` — sets `exp` relative to now
pub fn mint_jwt_inner(keys: &WalletKeys, did: &str, aud: &str, ttl_seconds: u32) -> String {
    mint_jwt_at(keys, did, aud, ttl_seconds, current_unix_secs())
}

/// Time-injectable core of [`mint_jwt_inner`] so tests can mint expired tokens.
fn mint_jwt_at(keys: &WalletKeys, did: &str, aud: &str, ttl_seconds: u32, iat: u64) -> String {
    let payload = json!({
        "iss": did,
        "sub": did,
        "aud": aud,
        "iat": iat,
        "exp": iat + u64::from(ttl_seconds),
    });

    let signing_input = format!(
        "{}.{}",
        URL_SAFE_NO_PAD.encode(JWT_HEADER),
        URL_SAFE_NO_PAD.encode(payload.to_string())
    );
    let sig: Signature = sign(&keys.did_sk, signing_input.as_bytes());
    format!("{signing_input}.{}", URL_SAFE_NO_PAD.encode(sig.to_bytes()))
}

/// Verifies signature and expiry of a JWT minted by [`mint_jwt_inner`].
///
/// * `did_pk_hex` — 48-byte BLS G1 public key, lowercase hex
/// * returns `Ok(false)` for expired, missing-`exp`, or bad-signature tokens
/// * returns `Err` for malformed encodings
pub fn verify_jwt_inner(token: &str, did_pk_hex: &str) -> Result<bool, String> {
    let [header_b64, payload_b64, sig_b64]: [&str; 3] = token
        .split('.')
        .collect::<Vec<_>>()
        .try_into()
        .map_err(|_| "JWT must have exactly 3 dot-separated parts".to_owned())?;

    // Expiry first — skips the BLS work for stale tokens; mint always sets `exp`.
    let payload_bytes = URL_SAFE_NO_PAD
        .decode(payload_b64)
        .map_err(|e| format!("invalid JWT payload encoding: {e}"))?;
    let payload: Value = serde_json::from_slice(&payload_bytes)
        .map_err(|e| format!("invalid JWT payload JSON: {e}"))?;
    match payload.get("exp").and_then(Value::as_u64) {
        Some(exp) if exp >= current_unix_secs() => {}
        _ => return Ok(false),
    }

    let sig_bytes = URL_SAFE_NO_PAD
        .decode(sig_b64)
        .map_err(|e| format!("invalid JWT signature encoding: {e}"))?;
    let sig = Signature::from_bytes(&to_array::<96>(&sig_bytes, "JWT signature")?)
        .map_err(|e| format!("invalid BLS signature: {e}"))?;

    let pk_bytes =
        hex::decode(did_pk_hex).map_err(|e| format!("invalid DID public key hex: {e}"))?;
    let pk = PublicKey::from_bytes(&to_array::<48>(&pk_bytes, "DID public key")?)
        .map_err(|e| format!("invalid BLS public key: {e}"))?;

    let signing_input = format!("{header_b64}.{payload_b64}");
    Ok(verify(&sig, &pk, signing_input.as_bytes()))
}

/// Converts a decoded byte slice into the fixed-size array chia-bls expects.
fn to_array<const N: usize>(bytes: &[u8], what: &str) -> Result<[u8; N], String> {
    bytes
        .try_into()
        .map_err(|_| format!("{what} must be {N} bytes, got {}", bytes.len()))
}

/// Seconds since the Unix epoch — `Date.now()` in the browser, `SystemTime` natively.
#[cfg(target_arch = "wasm32")]
fn current_unix_secs() -> u64 {
    (js_sys::Date::now() / 1000.0) as u64
}

#[cfg(not(target_arch = "wasm32"))]
fn current_unix_secs() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::keys::service::derive_wallet_keys_inner;

    const TEST_MNEMONIC: &str = "abandon abandon abandon abandon abandon abandon \
         abandon abandon abandon abandon abandon about";
    const TEST_DID: &str =
        "did:chia:deadbeef01020304050607080900aabbccddeeff01020304050607080900aabb";
    const TEST_AUD: &str = "https://app.example.com";

    #[test]
    fn mint_and_verify_round_trip() {
        let keys = derive_wallet_keys_inner(TEST_MNEMONIC).unwrap();
        let token = mint_jwt_inner(&keys, TEST_DID, TEST_AUD, 3600);
        assert!(verify_jwt_inner(&token, &keys.did_pk_hex()).unwrap());
    }

    #[test]
    fn tampered_payload_fails_verification() {
        let keys = derive_wallet_keys_inner(TEST_MNEMONIC).unwrap();
        let token = mint_jwt_inner(&keys, TEST_DID, TEST_AUD, 3600);

        // Swap out the payload for a different (unexpired) one — signature must break.
        let parts: Vec<&str> = token.split('.').collect();
        let evil_payload =
            URL_SAFE_NO_PAD.encode(r#"{"iss":"attacker","sub":"attacker","exp":9999999999}"#);
        let tampered = format!("{}.{}.{}", parts[0], evil_payload, parts[2]);

        assert!(!verify_jwt_inner(&tampered, &keys.did_pk_hex()).unwrap());
    }

    #[test]
    fn expired_jwt_fails_verification() {
        let keys = derive_wallet_keys_inner(TEST_MNEMONIC).unwrap();
        // iat far in the past → exp already elapsed.
        let token = mint_jwt_at(&keys, TEST_DID, TEST_AUD, 60, 1_000_000);
        assert!(!verify_jwt_inner(&token, &keys.did_pk_hex()).unwrap());
    }

    #[test]
    fn missing_exp_claim_fails_verification() {
        let keys = derive_wallet_keys_inner(TEST_MNEMONIC).unwrap();
        // Foreign token without `exp` — built by hand since mint always sets it.
        let signing_input = format!(
            "{}.{}",
            URL_SAFE_NO_PAD.encode(JWT_HEADER),
            URL_SAFE_NO_PAD.encode(r#"{"iss":"did:chia:test","sub":"did:chia:test"}"#)
        );
        let sig = sign(&keys.did_sk, signing_input.as_bytes());
        let token = format!("{signing_input}.{}", URL_SAFE_NO_PAD.encode(sig.to_bytes()));

        assert!(!verify_jwt_inner(&token, &keys.did_pk_hex()).unwrap());
    }

    #[test]
    fn different_key_fails_verification() {
        let keys_a = derive_wallet_keys_inner(TEST_MNEMONIC).unwrap();
        let keys_b =
            derive_wallet_keys_inner("zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong").unwrap();
        let token = mint_jwt_inner(&keys_a, TEST_DID, TEST_AUD, 3600);
        assert!(!verify_jwt_inner(&token, &keys_b.did_pk_hex()).unwrap());
    }

    #[test]
    fn malformed_token_is_an_error() {
        let keys = derive_wallet_keys_inner(TEST_MNEMONIC).unwrap();
        assert!(verify_jwt_inner("not-a-jwt", &keys.did_pk_hex()).is_err());
    }
}
