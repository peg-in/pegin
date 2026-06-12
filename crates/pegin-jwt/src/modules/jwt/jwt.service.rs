//! ES256K and legacy BLS JWT mint/verify for PEGIN wallet-as-IdP.

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chia_bls::{sign, verify, PublicKey, SecretKey, Signature};
use k256::ecdsa::{
    signature::Signer, signature::Verifier, Signature as EcdsaSignature, VerifyingKey,
};
use serde_json::json;
use sha2::{Digest, Sha256};

use crate::shared::error::JwtError;
use crate::shared::types::{ConfirmationClaim, PeginJwtPayload};

use super::jwt_helper::{
    bls_alg, bls_header, decode_bytes_segment, decode_json_segment, es256k_alg, es256k_header,
    es256k_jwk, es256k_signing_key, split_token,
};

/// Mints an ES256K JWT bound to `aud` with optional replay-resistant `nonce`.
pub fn mint_es256k(
    did_sk: &SecretKey,
    did: &str,
    did_pk_hex: &str,
    aud: &str,
    ttl_seconds: u32,
    nonce: Option<&str>,
    now: u64,
) -> Result<String, JwtError> {
    let signing_key = es256k_signing_key(did_sk).map_err(JwtError::InvalidToken)?;
    let payload = PeginJwtPayload {
        iss: did.to_owned(),
        sub: did.to_owned(),
        aud: aud.to_owned(),
        iat: now,
        exp: now + u64::from(ttl_seconds),
        nonce: nonce.map(str::to_owned),
        cnf: ConfirmationClaim {
            did_pk_hex: did_pk_hex.to_owned(),
        },
    };
    let payload_json = serde_json::to_string(&payload)
        .map_err(|e| JwtError::InvalidToken(format!("payload encode failed: {e}")))?;
    let header_b64 = es256k_header(did, &es256k_jwk(&signing_key));
    let payload_b64 = URL_SAFE_NO_PAD.encode(payload_json);
    let signing_input = format!("{header_b64}.{payload_b64}");
    let digest = Sha256::digest(signing_input.as_bytes());
    let sig: EcdsaSignature = signing_key
        .try_sign(&digest)
        .map_err(|e| JwtError::InvalidToken(format!("ES256K sign failed: {e}")))?;
    let sig_b64 = URL_SAFE_NO_PAD.encode(sig.to_bytes());
    Ok(format!("{signing_input}.{sig_b64}"))
}

/// Legacy BLS JWT mint (no `aud`) — retained for existing WASM tests during migration.
pub fn mint_bls_legacy(did_sk: &SecretKey, did: &str, ttl_seconds: u32, now: u64) -> String {
    let payload = json!({
        "iss": did,
        "sub": did,
        "iat": now,
        "exp": now + u64::from(ttl_seconds),
    });
    let header_b64 = bls_header();
    let payload_b64 = URL_SAFE_NO_PAD.encode(payload.to_string());
    let signing_input = format!("{header_b64}.{payload_b64}");
    let sig = sign(did_sk, signing_input.as_bytes());
    format!("{signing_input}.{}", URL_SAFE_NO_PAD.encode(sig.to_bytes()))
}

/// Verified session extracted from a valid PEGIN JWT.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedJwt {
    pub did: String,
    pub aud: String,
    pub did_pk_hex: String,
    pub nonce: Option<String>,
    pub iat: u64,
    pub exp: u64,
}

/// Verifies a PEGIN JWT (ES256K preferred; legacy BLS supported).
pub fn verify_token(
    token: &str,
    expected_aud: &str,
    expected_nonce: Option<&str>,
    now: u64,
) -> Result<VerifiedJwt, JwtError> {
    let [header_b64, payload_b64, sig_b64] = split_token(token).map_err(JwtError::InvalidToken)?;
    let header = decode_json_segment(header_b64, "JWT header").map_err(JwtError::InvalidToken)?;
    let alg = header
        .get("alg")
        .and_then(|v| v.as_str())
        .ok_or_else(|| JwtError::InvalidToken("JWT header missing alg".to_owned()))?;

    let signing_input = &token[..header_b64.len() + 1 + payload_b64.len()];
    let payload: PeginJwtPayload = if alg == es256k_alg() {
        verify_es256k_signature(&header, signing_input, sig_b64)?;
        decode_payload(payload_b64)?
    } else if alg == bls_alg() {
        return Err(JwtError::UnsupportedAlgorithm(
            "legacy BLS JWT — use verify_bls_legacy_with_pubkey".to_owned(),
        ));
    } else {
        return Err(JwtError::UnsupportedAlgorithm(alg.to_owned()));
    };

    validate_claims(&payload, expected_aud, expected_nonce, now)?;

    Ok(VerifiedJwt {
        did: payload.iss,
        aud: payload.aud,
        did_pk_hex: payload.cnf.did_pk_hex,
        nonce: payload.nonce,
        iat: payload.iat,
        exp: payload.exp,
    })
}

fn decode_payload(payload_b64: &str) -> Result<PeginJwtPayload, JwtError> {
    let bytes = URL_SAFE_NO_PAD
        .decode(payload_b64)
        .map_err(|e| JwtError::InvalidToken(format!("invalid payload encoding: {e}")))?;
    serde_json::from_slice(&bytes)
        .map_err(|e| JwtError::InvalidToken(format!("invalid payload JSON: {e}")))
}

fn legacy_payload(payload_b64: &str) -> Result<PeginJwtPayload, JwtError> {
    let value = decode_json_segment(payload_b64, "JWT payload").map_err(JwtError::InvalidToken)?;
    let iss = value
        .get("iss")
        .and_then(|v| v.as_str())
        .ok_or_else(|| JwtError::InvalidToken("legacy JWT missing iss".to_owned()))?
        .to_owned();
    let sub = value
        .get("sub")
        .and_then(|v| v.as_str())
        .ok_or_else(|| JwtError::InvalidToken("legacy JWT missing sub".to_owned()))?
        .to_owned();
    let iat = value
        .get("iat")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| JwtError::InvalidToken("legacy JWT missing iat".to_owned()))?;
    let exp = value
        .get("exp")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| JwtError::InvalidToken("legacy JWT missing exp".to_owned()))?;
    Ok(PeginJwtPayload {
        iss,
        sub,
        aud: String::new(),
        iat,
        exp,
        nonce: None,
        cnf: ConfirmationClaim {
            did_pk_hex: String::new(),
        },
    })
}

fn validate_claims(
    payload: &PeginJwtPayload,
    expected_aud: &str,
    expected_nonce: Option<&str>,
    now: u64,
) -> Result<(), JwtError> {
    if payload.exp < now {
        return Err(JwtError::Expired);
    }
    if payload.iss != payload.sub {
        return Err(JwtError::IssuerSubjectMismatch);
    }
    if payload.aud != expected_aud {
        return Err(JwtError::AudienceMismatch {
            expected: expected_aud.to_owned(),
            actual: payload.aud.clone(),
        });
    }
    match (expected_nonce, payload.nonce.as_deref()) {
        (Some(expected), Some(actual)) if expected == actual => {}
        (Some(_), Some(_)) => return Err(JwtError::NonceMismatch),
        (Some(_), None) => return Err(JwtError::MissingNonce),
        (None, _) => {}
    }
    if payload.cnf.did_pk_hex.is_empty() && !payload.aud.is_empty() {
        return Err(JwtError::MissingConfirmationKey);
    }
    Ok(())
}

fn verify_es256k_signature(
    header: &serde_json::Value,
    signing_input: &str,
    sig_b64: &str,
) -> Result<(), JwtError> {
    let jwk = header
        .get("jwk")
        .ok_or_else(|| JwtError::InvalidToken("ES256K JWT header missing jwk".to_owned()))?;
    let x = decode_coord(jwk.get("x"), "jwk.x")?;
    let y = decode_coord(jwk.get("y"), "jwk.y")?;
    let mut uncompressed = [0u8; 65];
    uncompressed[0] = 0x04;
    uncompressed[1..33].copy_from_slice(&x);
    uncompressed[33..65].copy_from_slice(&y);
    let verifying_key = VerifyingKey::from_sec1_bytes(&uncompressed)
        .map_err(|e| JwtError::InvalidToken(format!("invalid jwk: {e}")))?;
    let sig_bytes: [u8; 64] =
        decode_bytes_segment(sig_b64, "JWT signature").map_err(JwtError::InvalidToken)?;
    let signature = EcdsaSignature::from_bytes(&sig_bytes.into())
        .map_err(|e| JwtError::InvalidToken(format!("invalid ES256K signature: {e}")))?;
    let digest = Sha256::digest(signing_input.as_bytes());
    verifying_key
        .verify(&digest, &signature)
        .map_err(|_| JwtError::InvalidSignature)
}

fn decode_coord(value: Option<&serde_json::Value>, name: &str) -> Result<[u8; 32], JwtError> {
    let text = value
        .and_then(|v| v.as_str())
        .ok_or_else(|| JwtError::InvalidToken(format!("{name} missing")))?;
    let bytes = URL_SAFE_NO_PAD
        .decode(text)
        .map_err(|e| JwtError::InvalidToken(format!("invalid {name}: {e}")))?;
    bytes
        .try_into()
        .map_err(|_| JwtError::InvalidToken(format!("{name} must be 32 bytes")))
}

/// Verifies a legacy BLS JWT when the DID public key bytes are already known.
pub fn verify_bls_legacy_with_pubkey(
    token: &str,
    did_public_key: &[u8],
    now: u64,
) -> Result<bool, JwtError> {
    let [header_b64, payload_b64, sig_b64] = split_token(token).map_err(JwtError::InvalidToken)?;
    let header = decode_json_segment(header_b64, "JWT header").map_err(JwtError::InvalidToken)?;
    let alg = header
        .get("alg")
        .and_then(|v| v.as_str())
        .ok_or_else(|| JwtError::InvalidToken("JWT header missing alg".to_owned()))?;
    if alg != bls_alg() {
        return Err(JwtError::UnsupportedAlgorithm(alg.to_owned()));
    }
    let payload = legacy_payload(payload_b64)?;
    if payload.exp < now {
        return Ok(false);
    }
    if payload.iss != payload.sub {
        return Ok(false);
    }
    let sig_bytes: [u8; 96] =
        decode_bytes_segment(sig_b64, "JWT signature").map_err(JwtError::InvalidToken)?;
    let sig = Signature::from_bytes(&sig_bytes)
        .map_err(|e| JwtError::InvalidToken(format!("invalid BLS signature: {e}")))?;
    let pk_bytes: [u8; 48] = did_public_key
        .try_into()
        .map_err(|_| JwtError::InvalidToken("DID public key must be 48 bytes".to_owned()))?;
    let pk = PublicKey::from_bytes(&pk_bytes)
        .map_err(|e| JwtError::InvalidToken(format!("invalid BLS public key: {e}")))?;
    let signing_input = &token[..header_b64.len() + 1 + payload_b64.len()];
    Ok(verify(&sig, &pk, signing_input.as_bytes()))
}

#[cfg(test)]
mod tests {
    use chia_bls::SecretKey;

    use super::*;
    use crate::modules::jwt::jwt_helper::{bls_header, es256k_signing_key};

    fn test_did_sk() -> SecretKey {
        SecretKey::from_bytes(&[7u8; 32]).expect("valid test key")
    }

    #[test]
    fn es256k_round_trip_with_aud_and_nonce() {
        let did_sk = test_did_sk();
        let did = "did:chia:1gt7hae94wd0c33v07k4kkwgjy9jjtcnzhwvl5yxuvmj28mqsnsjqvgw9uu";
        let did_pk_hex = hex::encode(did_sk.public_key().to_bytes());
        let token = mint_es256k(
            &did_sk,
            did,
            &did_pk_hex,
            "https://demo.example",
            3600,
            Some("server-nonce-1"),
            1_000_000,
        )
        .expect("mint");
        let verified = verify_token(
            &token,
            "https://demo.example",
            Some("server-nonce-1"),
            1_000_000,
        )
        .expect("verify");
        assert_eq!(verified.did, did);
        assert_eq!(verified.did_pk_hex, did_pk_hex);
    }

    #[test]
    fn rejects_wrong_audience() {
        let did_sk = test_did_sk();
        let did_pk_hex = hex::encode(did_sk.public_key().to_bytes());
        let token = mint_es256k(
            &did_sk,
            "did:chia:test",
            &did_pk_hex,
            "https://a.example",
            60,
            None,
            1,
        )
        .expect("mint");
        let err = verify_token(&token, "https://b.example", None, 1).unwrap_err();
        assert!(matches!(err, JwtError::AudienceMismatch { .. }));
    }

    #[test]
    fn expired_token_is_rejected() {
        let did_sk = test_did_sk();
        let did_pk_hex = hex::encode(did_sk.public_key().to_bytes());
        let token = mint_es256k(
            &did_sk,
            "did:chia:test",
            &did_pk_hex,
            "https://a.example",
            60,
            None,
            1_000_000,
        )
        .expect("mint");
        let err = verify_token(&token, "https://a.example", None, 1_000_061).unwrap_err();
        assert!(matches!(err, JwtError::Expired));
    }

    #[test]
    fn legacy_token_missing_exp_is_rejected() {
        let did_sk = test_did_sk();
        let payload = json!({ "iss": "did:chia:test", "sub": "did:chia:test", "iat": 1 });
        let payload_b64 = URL_SAFE_NO_PAD.encode(payload.to_string());
        let signing_input = format!("{}.{payload_b64}", bls_header());
        let sig = sign(&did_sk, signing_input.as_bytes());
        let token = format!("{signing_input}.{}", URL_SAFE_NO_PAD.encode(sig.to_bytes()));
        let err =
            verify_bls_legacy_with_pubkey(&token, &did_sk.public_key().to_bytes(), 1).unwrap_err();
        assert!(matches!(err, JwtError::InvalidToken(_)));
    }

    #[test]
    fn expired_legacy_token_fails_verification() {
        let did_sk = test_did_sk();
        let token = mint_bls_legacy(&did_sk, "did:chia:test", 60, 1_000_000);
        let valid =
            verify_bls_legacy_with_pubkey(&token, &did_sk.public_key().to_bytes(), 2_000_000)
                .expect("verify");
        assert!(!valid);
    }

    #[test]
    fn es256k_signing_key_is_deterministic() {
        let did_sk = test_did_sk();
        let a = es256k_signing_key(&did_sk).expect("key");
        let b = es256k_signing_key(&did_sk).expect("key");
        assert_eq!(a.to_bytes(), b.to_bytes());
    }
}
