//! HKDF + JWK helpers for ES256K JWT signing.

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chia_bls::SecretKey;
use hkdf::Hkdf;
use k256::ecdsa::SigningKey;
use serde_json::{json, Value};
use sha2::Sha256;

use crate::shared::hkdf_config::{hkdf_info, hkdf_salt};

const ES256K_ALG: &str = "ES256K";
const BLS_ALG: &str = "BLS12381_G2";

pub fn es256k_alg() -> &'static str {
    ES256K_ALG
}

pub fn bls_alg() -> &'static str {
    BLS_ALG
}

/// Derives the ES256K signing key deterministically from the DID BLS secret key.
pub fn es256k_signing_key(did_sk: &SecretKey) -> Result<SigningKey, String> {
    let salt = hkdf_salt();
    let ikm = did_sk.to_bytes();
    let hk = Hkdf::<Sha256>::new(Some(salt), &ikm);
    let mut okm = [0u8; 32];
    hk.expand(hkdf_info(), &mut okm)
        .map_err(|_| "HKDF expand failed".to_owned())?;
    SigningKey::from_bytes((&okm).into()).map_err(|e| format!("invalid ES256K key: {e}"))
}

pub fn es256k_jwk(signing_key: &SigningKey) -> Value {
    let encoded = signing_key.verifying_key().to_encoded_point(false);
    let x = encoded
        .x()
        .map(|coord| URL_SAFE_NO_PAD.encode(coord))
        .unwrap_or_default();
    let y = encoded
        .y()
        .map(|coord| URL_SAFE_NO_PAD.encode(coord))
        .unwrap_or_default();
    json!({
        "kty": "EC",
        "crv": "secp256k1",
        "x": x,
        "y": y,
    })
}

pub fn es256k_header(kid: &str, jwk: &Value) -> String {
    let header = json!({
        "alg": ES256K_ALG,
        "typ": "JWT",
        "kid": kid,
        "jwk": jwk,
    });
    URL_SAFE_NO_PAD.encode(header.to_string())
}

pub fn bls_header() -> String {
    let header = json!({ "alg": BLS_ALG, "typ": "JWT" });
    URL_SAFE_NO_PAD.encode(header.to_string())
}

pub fn split_token(token: &str) -> Result<[&str; 3], String> {
    token
        .split('.')
        .collect::<Vec<_>>()
        .try_into()
        .map_err(|_| "JWT must have exactly 3 dot-separated parts".to_owned())
}

pub fn decode_json_segment(segment: &str, name: &str) -> Result<Value, String> {
    let bytes = URL_SAFE_NO_PAD
        .decode(segment)
        .map_err(|e| format!("invalid {name} encoding: {e}"))?;
    serde_json::from_slice(&bytes).map_err(|e| format!("invalid {name} JSON: {e}"))
}

pub fn decode_bytes_segment<const N: usize>(segment: &str, name: &str) -> Result<[u8; N], String> {
    let bytes = URL_SAFE_NO_PAD
        .decode(segment)
        .map_err(|e| format!("invalid {name} encoding: {e}"))?;
    bytes
        .try_into()
        .map_err(|_| format!("{name} must be {N} bytes"))
}
