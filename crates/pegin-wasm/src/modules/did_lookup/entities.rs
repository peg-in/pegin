//! Local login identity resolved from wallet keys — no chain reads.

use serde::{Deserialize, Serialize};

/// Owner observer index used before a first-login scan resolves the real one.
pub const DEFAULT_OWNER_INDEX: u32 = 0;

/// Default address-index ceiling for first-login DID discovery (overridable from JS).
pub const DEFAULT_SCAN_LIMIT: u32 = 10_000;

/// Login identity derived entirely in WASM.
///
/// `owner_pk` is the `cnf.did_pk` material the relay binds to the on-chain DID owner.
/// `did` is present only when a prior session cached the canonical `did:chia:…`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedIdentity {
    pub owner_index: u32,
    pub owner_pk: String,
    pub did: Option<String>,
}

/// Cached login facts for one wallet, persisted by [`crate::modules::did_lookup`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CachedProfile {
    pub owner_index: u32,
    pub did: String,
}
