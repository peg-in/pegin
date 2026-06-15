//! Local DID lookup: resolve login identity from wallet keys.
//!
//! Cache hit → instant, zero network. Cache miss → scan **public** coinset hints for
//! the address index that owns the DID (first login only), then cache it. Secrets never
//! leave WASM; the auth relay independently re-verifies ownership from `cnf.did_pk`.

use crate::modules::keys::WalletKeys;

use super::entities::{CachedProfile, ResolvedIdentity, DEFAULT_OWNER_INDEX};
use super::helper::wallet_fingerprint;
use super::repository::ProfileStore;

/// Testnet11 coinset REST base — the only public endpoint the lookup reads.
#[cfg(target_arch = "wasm32")]
const COINSET_REST_BASE: &str = "https://testnet11.api.coinset.org";

/// Owner public key (hex) for the address key at `index` — the JWT's `cnf.did_pk`.
fn owner_pk_hex(keys: &WalletKeys, index: u32) -> String {
    hex::encode(keys.owner_secret_at(index).public_key().to_bytes())
}

/// Identity from a cached profile (relay/scan-confirmed owner index + DID).
fn identity_from_cache(keys: &WalletKeys, cached: &CachedProfile) -> ResolvedIdentity {
    ResolvedIdentity {
        owner_index: cached.owner_index,
        owner_pk: owner_pk_hex(keys, cached.owner_index),
        did: Some(cached.did.clone()),
    }
}

/// Pre-resolution identity: the default owner key with no DID (first login, no scan hit).
fn default_identity(keys: &WalletKeys) -> ResolvedIdentity {
    ResolvedIdentity {
        owner_index: DEFAULT_OWNER_INDEX,
        owner_pk: owner_pk_hex(keys, DEFAULT_OWNER_INDEX),
        did: None,
    }
}

/// Persists the resolved canonical `did` and its owning index for this wallet.
pub fn remember_identity<S: ProfileStore>(store: &S, wallet_fp: &str, did: &str, owner_index: u32) {
    store.write(
        wallet_fp,
        &CachedProfile {
            owner_index,
            did: did.to_owned(),
        },
    );
}

/// JWT `iss`/`sub` subject: the cached DID, or a key-asserted fallback before a DID is
/// known. The relay replaces it with the canonical `did:chia` it verifies on-chain.
pub fn login_subject(identity: &ResolvedIdentity) -> String {
    identity
        .did
        .clone()
        .unwrap_or_else(|| identity.owner_pk.clone())
}

#[cfg(target_arch = "wasm32")]
fn store() -> super::repository::LocalStorageProfileStore {
    super::repository::LocalStorageProfileStore
}

#[cfg(not(target_arch = "wasm32"))]
fn store() -> super::repository::NoopProfileStore {
    super::repository::NoopProfileStore
}

/// Resolves login identity, scanning public coinset hints on a cache miss (first login).
///
/// * `scan_limit` — highest address index probed before giving up
pub async fn lookup_did_inner(
    keys: &WalletKeys,
    scan_limit: u32,
) -> Result<ResolvedIdentity, String> {
    let store = store();
    let fingerprint = wallet_fingerprint(keys);
    if let Some(cached) = store.read(&fingerprint) {
        return Ok(identity_from_cache(keys, &cached));
    }

    if let Some((did, owner_index)) = scan_for_did(keys, scan_limit).await? {
        remember_identity(&store, &fingerprint, &did, owner_index);
        return Ok(identity_from_cache(
            keys,
            &CachedProfile { owner_index, did },
        ));
    }

    Ok(default_identity(keys))
}

/// Scans public coinset hints for the DID owner. Browser only — native has no HTTP client.
/// Per-request timeouts/retries live in `CoinsetRestClient`; total work is bounded by `scan_limit`.
#[cfg(target_arch = "wasm32")]
async fn scan_for_did(keys: &WalletKeys, scan_limit: u32) -> Result<Option<(String, u32)>, String> {
    use super::coinset::CoinsetRestClient;
    use super::scan::resolve_did_and_owner;

    let client = CoinsetRestClient::new(COINSET_REST_BASE.to_owned())?;
    resolve_did_and_owner(&client, keys, scan_limit).await
}

// Kept `async` to share one signature with the WASM scan; native has no live lookup.
#[cfg(not(target_arch = "wasm32"))]
#[allow(clippy::unused_async)]
async fn scan_for_did(
    _keys: &WalletKeys,
    _scan_limit: u32,
) -> Result<Option<(String, u32)>, String> {
    Ok(None)
}

/// Caches a canonical DID, keyed by wallet fingerprint (relay-confirmed override path).
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub fn remember_did_inner(wallet_fp: &str, did: &str, owner_index: u32) {
    remember_identity(&store(), wallet_fp, did, owner_index);
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::collections::HashMap;

    use super::*;
    use crate::modules::keys::service::derive_wallet_keys_inner;
    use crate::test_util::deterministic_test_phrase;

    #[derive(Default)]
    struct MockStore {
        entries: RefCell<HashMap<String, CachedProfile>>,
    }

    impl ProfileStore for MockStore {
        fn read(&self, wallet_fp: &str) -> Option<CachedProfile> {
            self.entries.borrow().get(wallet_fp).cloned()
        }

        fn write(&self, wallet_fp: &str, profile: &CachedProfile) {
            self.entries
                .borrow_mut()
                .insert(wallet_fp.to_owned(), profile.clone());
        }
    }

    #[test]
    fn first_login_has_owner_key_and_no_did() {
        let keys = derive_wallet_keys_inner(&deterministic_test_phrase()).unwrap();
        let identity = default_identity(&keys);
        assert_eq!(identity.owner_index, DEFAULT_OWNER_INDEX);
        assert_eq!(identity.owner_pk.len(), 96);
        assert!(identity.did.is_none());
    }

    #[test]
    fn cached_identity_round_trips_owner_index_and_did() {
        let keys = derive_wallet_keys_inner(&deterministic_test_phrase()).unwrap();
        let store = MockStore::default();
        remember_identity(&store, &wallet_fingerprint(&keys), "did:chia:1cached", 4757);

        let cached = store.read(&wallet_fingerprint(&keys)).unwrap();
        let identity = identity_from_cache(&keys, &cached);
        assert_eq!(identity.owner_index, 4757);
        assert_eq!(identity.did.as_deref(), Some("did:chia:1cached"));
        assert_eq!(identity.owner_pk, owner_pk_hex(&keys, 4757));
    }

    #[test]
    fn login_subject_prefers_cached_did_else_owner_pk() {
        let keys = derive_wallet_keys_inner(&deterministic_test_phrase()).unwrap();
        let first = default_identity(&keys);
        assert_eq!(login_subject(&first), first.owner_pk);

        let cached = CachedProfile {
            owner_index: 0,
            did: "did:chia:1known".to_owned(),
        };
        assert_eq!(
            login_subject(&identity_from_cache(&keys, &cached)),
            "did:chia:1known"
        );
    }
}
