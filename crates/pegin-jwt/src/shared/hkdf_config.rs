//! HKDF salt + info for ES256K key derivation.
//!
//! The salt comes from `PEGIN_JWT_HKDF_SALT`, embedded at compile time by the
//! crate build script (`build.rs`). Every target uses that one compile-time
//! value: a runtime override on native could derive keys that wasm-minted
//! tokens can't verify, since the two builds must share an identical salt.

const HKDF_INFO: &[u8] = b"pegin-jwt-es256k-v1";

/// HKDF salt bytes, embedded from `PEGIN_JWT_HKDF_SALT` at build time.
pub fn hkdf_salt() -> &'static [u8] {
    env!("PEGIN_JWT_HKDF_SALT").as_bytes()
}

/// HKDF info label (protocol id, not secret).
pub fn hkdf_info() -> &'static [u8] {
    HKDF_INFO
}
