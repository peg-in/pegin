//! HKDF salt from `PEGIN_JWT_HKDF_SALT` — never hardcoded in source.

#[cfg(not(target_arch = "wasm32"))]
use std::sync::OnceLock;

#[cfg(not(target_arch = "wasm32"))]
static RUNTIME_SALT: OnceLock<Vec<u8>> = OnceLock::new();

const HKDF_INFO: &[u8] = b"pegin-jwt-es256k-v1";

/// HKDF salt bytes — runtime env on native, compile-time embed on wasm32.
pub fn hkdf_salt() -> &'static [u8] {
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Ok(value) = std::env::var("PEGIN_JWT_HKDF_SALT") {
            return RUNTIME_SALT.get_or_init(|| value.into_bytes()).as_slice();
        }
    }

    env!("PEGIN_JWT_HKDF_SALT").as_bytes()
}

/// HKDF info label (protocol id, not secret).
pub fn hkdf_info() -> &'static [u8] {
    HKDF_INFO
}
