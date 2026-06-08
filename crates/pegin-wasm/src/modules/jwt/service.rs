use crate::modules::keys::entities::WalletKeys;

/// JWT minting via ES256K — deferred to a follow-up ticket.
///
/// Design (implemented when needed):
///   `did_sk` (BLS secret key)
///     └─► HKDF-SHA256(salt = b"pegin-jwt-es256k-v1") → 32 bytes
///           └─► `k256::SecretKey` → sign JWT with ES256K (RFC 8812)
///
/// Header: { "alg": "ES256K", "kid": "<did>" }
/// Payload: { "iss": did, "sub": did, "aud": aud, "iat": now, "exp": exp }
/// Signature: ES256K over base64url(header).base64url(payload)
///
/// Requires adding `k256` and `base64` to Cargo.toml.
pub fn mint_jwt_inner(_keys: &WalletKeys, _claims: &str) -> Result<String, String> {
    Err("mintJwt not yet implemented — see pegin-wasm JWT deferred ticket".to_owned())
}
