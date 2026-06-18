//! The `ChainResolver` trait — the relay's only contract with the chain backend.

use chia_wallet_sdk::prelude::PublicKey;

/// Why a resolve failed. Maps to an HTTP status at the route boundary:
/// `NotFound` → 404, `Invalid` → 400, `Upstream` → 502.
#[derive(Debug)]
pub enum ResolveError {
    /// No on-chain DID is owned by any address key derived from the account key.
    NotFound,
    /// Bad caller input (e.g. malformed public key or DID string).
    Invalid(String),
    /// The chain backend (coinset today) failed or was unreachable.
    Upstream(String),
}

/// Parses a 96-char hex BLS G1 account key (the wire format `/resolve` receives) into a
/// `PublicKey`. Lets callers stay agnostic of the BLS crate version behind the seam.
pub fn account_pk_from_hex(hex_str: &str) -> Result<PublicKey, ResolveError> {
    let bytes =
        hex::decode(hex_str).map_err(|e| ResolveError::Invalid(format!("accountPk hex: {e}")))?;
    let arr: [u8; 48] = bytes
        .as_slice()
        .try_into()
        .map_err(|_| ResolveError::Invalid("accountPk must be 48-byte hex".to_owned()))?;
    PublicKey::from_bytes(&arr)
        .map_err(|e| ResolveError::Invalid(format!("accountPk not a valid BLS key: {e}")))
}

/// Resolve a watch-only account key to its DID, and re-verify on-chain ownership.
///
/// Implementations read only **public** chain data; no secret crosses this boundary.
#[allow(async_fn_in_trait)]
pub trait ChainResolver {
    /// Maps the wallet's observer account key to `(did, owner_index)`.
    async fn resolve_owner(&self, account_pk: &PublicKey) -> Result<(String, u32), ResolveError>;

    /// `true` when `owner_pk` is the on-chain owner of `did` (feat-17 binding check).
    async fn verify_did_owner(&self, did: &str, owner_pk: &PublicKey)
        -> Result<bool, ResolveError>;
}
