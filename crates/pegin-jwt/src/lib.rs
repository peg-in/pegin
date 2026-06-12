//! PEGIN JWT mint and verify — ES256K (RFC 8812) with legacy BLS support.

mod modules;
mod shared;

pub use modules::jwt::{
    mint_bls_legacy, mint_es256k, verify_bls_legacy_with_pubkey, verify_token, VerifiedJwt,
};
pub use shared::error::JwtError;
