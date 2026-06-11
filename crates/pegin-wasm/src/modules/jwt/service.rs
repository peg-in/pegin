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
/// * `ttl_seconds` — sets `exp` relative to now
pub fn mint_jwt_inner(keys: &WalletKeys, did: &str, ttl_seconds: u32) -> String {
    mint_jwt_at(keys, did, ttl_seconds, current_unix_secs())
}

/// Time-injectable core of [`mint_jwt_inner`] so tests can mint expired tokens.
fn mint_jwt_at(keys: &WalletKeys, did: &str, ttl_seconds: u32, iat: u64) -> String {
    let payload = json!({
        "iss": did,
        "sub": did,
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
/// * `did_public_key` — 48-byte BLS G1 public key
/// * returns `Ok(false)` for expired, missing-`exp`, or bad-signature tokens
/// * returns `Err` for malformed encodings
pub fn verify_jwt_inner(token: &str, did_public_key: &[u8]) -> Result<bool, String> {
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

    let pk = PublicKey::from_bytes(&to_array::<48>(did_public_key, "DID public key")?)
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

    fn did_pk(keys: &WalletKeys) -> Vec<u8> {
        keys.did_sk.public_key().to_bytes().to_vec()
    }

    #[test]
    fn mint_and_verify_round_trip() {
        let keys = derive_wallet_keys_inner(TEST_MNEMONIC).unwrap();
        let token = mint_jwt_inner(&keys, TEST_DID, 3600);
        assert!(verify_jwt_inner(&token, &did_pk(&keys)).unwrap());
    }

    #[test]
    fn payload_contains_iss_sub_iat_exp_only() {
        let keys = derive_wallet_keys_inner(TEST_MNEMONIC).unwrap();
        let token = mint_jwt_inner(&keys, TEST_DID, 3600);
        let payload_b64 = token.split('.').nth(1).unwrap();
        let payload: Value =
            serde_json::from_slice(&URL_SAFE_NO_PAD.decode(payload_b64).unwrap()).unwrap();

        assert_eq!(payload["iss"], TEST_DID);
        assert_eq!(payload["sub"], TEST_DID);
        assert!(payload.get("iat").and_then(Value::as_u64).is_some());
        assert!(payload.get("exp").and_then(Value::as_u64).is_some());
        assert!(payload.get("aud").is_none());
    }

    #[test]
    fn tampered_payload_fails_verification() {
        let keys = derive_wallet_keys_inner(TEST_MNEMONIC).unwrap();
        let token = mint_jwt_inner(&keys, TEST_DID, 3600);

        let parts: Vec<&str> = token.split('.').collect();
        let evil_payload =
            URL_SAFE_NO_PAD.encode(r#"{"iss":"attacker","sub":"attacker","exp":9999999999}"#);
        let tampered = format!("{}.{}.{}", parts[0], evil_payload, parts[2]);

        assert!(!verify_jwt_inner(&tampered, &did_pk(&keys)).unwrap());
    }

    #[test]
    fn expired_jwt_fails_verification() {
        let keys = derive_wallet_keys_inner(TEST_MNEMONIC).unwrap();
        let token = mint_jwt_at(&keys, TEST_DID, 60, 1_000_000);
        assert!(!verify_jwt_inner(&token, &did_pk(&keys)).unwrap());
    }

    #[test]
    fn missing_exp_claim_fails_verification() {
        let keys = derive_wallet_keys_inner(TEST_MNEMONIC).unwrap();
        let signing_input = format!(
            "{}.{}",
            URL_SAFE_NO_PAD.encode(JWT_HEADER),
            URL_SAFE_NO_PAD.encode(r#"{"iss":"did:chia:test","sub":"did:chia:test"}"#)
        );
        let sig = sign(&keys.did_sk, signing_input.as_bytes());
        let token = format!("{signing_input}.{}", URL_SAFE_NO_PAD.encode(sig.to_bytes()));

        assert!(!verify_jwt_inner(&token, &did_pk(&keys)).unwrap());
    }

    #[test]
    fn different_key_fails_verification() {
        let keys_a = derive_wallet_keys_inner(TEST_MNEMONIC).unwrap();
        let keys_b =
            derive_wallet_keys_inner("zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong").unwrap();
        let token = mint_jwt_inner(&keys_a, TEST_DID, 3600);
        assert!(!verify_jwt_inner(&token, &did_pk(&keys_b)).unwrap());
    }

    #[test]
    fn malformed_token_is_an_error() {
        let keys = derive_wallet_keys_inner(TEST_MNEMONIC).unwrap();
        assert!(verify_jwt_inner("not-a-jwt", &did_pk(&keys)).is_err());
    }
}
