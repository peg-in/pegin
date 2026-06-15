//! ES256K JWT mint/verify via shared `pegin-jwt` crate.

use chia_bls::SecretKey;
use pegin_jwt::{mint_es256k, verify_token, JwtError};

use crate::modules::keys::WalletKeys;

/// Mints an ES256K JWT signed by `signing_sk`, embedding its public key as `cnf.did_pk`.
/// The relying party binds that key to the on-chain DID owner.
pub fn mint_jwt_with_sk(
    signing_sk: &SecretKey,
    did: &str,
    aud: &str,
    ttl_seconds: u32,
    nonce: Option<&str>,
) -> Result<String, String> {
    let did_pk_hex = hex::encode(signing_sk.public_key().to_bytes());
    mint_es256k(
        signing_sk,
        did,
        &did_pk_hex,
        aud,
        ttl_seconds,
        nonce,
        current_unix_secs(),
    )
    .map_err(|e| e.to_string())
}

/// Mints an ES256K JWT bound to `aud` with optional replay-resistant `nonce`,
/// signed with the DID-path key (standalone helper; login uses the owner key).
pub fn mint_jwt_inner(
    keys: &WalletKeys,
    did: &str,
    aud: &str,
    ttl_seconds: u32,
    nonce: Option<&str>,
) -> Result<String, String> {
    mint_jwt_with_sk(&keys.did_sk, did, aud, ttl_seconds, nonce)
}

/// Verifies an ES256K JWT minted by [`mint_jwt_inner`].
///
/// Returns `Ok(false)` for expired or bad-signature tokens; `Err` for malformed input.
pub fn verify_jwt_inner(
    token: &str,
    expected_aud: &str,
    expected_nonce: Option<&str>,
) -> Result<bool, String> {
    match verify_token(token, expected_aud, expected_nonce, current_unix_secs()) {
        Ok(_) => Ok(true),
        Err(
            JwtError::Expired
            | JwtError::InvalidSignature
            | JwtError::AudienceMismatch { .. }
            | JwtError::NonceMismatch
            | JwtError::IssuerSubjectMismatch,
        ) => Ok(false),
        Err(e) => Err(e.to_string()),
    }
}

/// Seconds since the Unix epoch — `Date.now()` in the browser, `SystemTime` natively.
// `Date.now()` is whole milliseconds since the epoch: always positive and far within u64.
#[cfg(target_arch = "wasm32")]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
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
    use crate::test_util::deterministic_test_phrase;

    const TEST_DID: &str =
        "did:chia:deadbeef01020304050607080900aabbccddeeff01020304050607080900aabb";
    const TEST_AUD: &str = "https://demo.example";

    #[test]
    fn mint_and_verify_round_trip() {
        let keys = derive_wallet_keys_inner(&deterministic_test_phrase()).unwrap();
        let token = mint_jwt_inner(&keys, TEST_DID, TEST_AUD, 3600, None).unwrap();
        assert!(verify_jwt_inner(&token, TEST_AUD, None).unwrap());
    }

    #[test]
    fn payload_contains_aud_and_cnf() {
        use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
        use serde_json::Value;

        let keys = derive_wallet_keys_inner(&deterministic_test_phrase()).unwrap();
        let token = mint_jwt_inner(&keys, TEST_DID, TEST_AUD, 3600, Some("nonce-1")).unwrap();
        let payload_b64 = token.split('.').nth(1).unwrap();
        let payload: Value =
            serde_json::from_slice(&URL_SAFE_NO_PAD.decode(payload_b64).unwrap()).unwrap();

        assert_eq!(payload["iss"], TEST_DID);
        assert_eq!(payload["sub"], TEST_DID);
        assert_eq!(payload["aud"], TEST_AUD);
        assert_eq!(payload["nonce"], "nonce-1");
        assert!(payload.get("cnf").and_then(|c| c.get("did_pk")).is_some());
    }

    #[test]
    fn tampered_payload_fails_verification() {
        use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

        let keys = derive_wallet_keys_inner(&deterministic_test_phrase()).unwrap();
        let token = mint_jwt_inner(&keys, TEST_DID, TEST_AUD, 3600, None).unwrap();

        let parts: Vec<&str> = token.split('.').collect();
        let evil_payload =
            URL_SAFE_NO_PAD.encode(r#"{"iss":"attacker","sub":"attacker","exp":9999999999}"#);
        let tampered = format!("{}.{}.{}", parts[0], evil_payload, parts[2]);

        assert!(!verify_jwt_inner(&tampered, TEST_AUD, None).unwrap());
    }

    #[test]
    fn wrong_audience_fails_verification() {
        let keys = derive_wallet_keys_inner(&deterministic_test_phrase()).unwrap();
        let token = mint_jwt_inner(&keys, TEST_DID, TEST_AUD, 3600, None).unwrap();
        assert!(!verify_jwt_inner(&token, "https://other.example", None).unwrap());
    }

    #[test]
    fn malformed_token_is_an_error() {
        assert!(verify_jwt_inner("not-a-jwt", TEST_AUD, None).is_err());
    }
}
