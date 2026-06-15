//! Local profile cache — persists a wallet's canonical DID across logins.
//!
//! The browser binds this to `localStorage`; native builds (and unit tests) use
//! a no-op store, so resolving identity never depends on the chain or a backend.

use super::entities::CachedProfile;

/// Read/write access to the per-wallet login profile, keyed by wallet fingerprint.
pub trait ProfileStore {
    /// Returns the cached profile for `wallet_fp`, or `None` on a first login.
    fn read(&self, wallet_fp: &str) -> Option<CachedProfile>;

    /// Persists `profile` so the next [`read`](ProfileStore::read) returns the DID.
    fn write(&self, wallet_fp: &str, profile: &CachedProfile);
}

/// Profile store that never persists — native target and tests with no browser storage.
// The browser build uses `LocalStorageProfileStore`; this stays for native and unit tests.
#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
pub struct NoopProfileStore;

impl ProfileStore for NoopProfileStore {
    fn read(&self, _wallet_fp: &str) -> Option<CachedProfile> {
        None
    }

    fn write(&self, _wallet_fp: &str, _profile: &CachedProfile) {}
}

/// localStorage key for a wallet fingerprint (`pegin:did:<fp>`).
#[cfg(any(target_arch = "wasm32", test))]
fn storage_key(wallet_fp: &str) -> String {
    format!("pegin:did:{wallet_fp}")
}

/// Browser `localStorage`-backed profile cache (same-origin, survives reloads).
#[cfg(target_arch = "wasm32")]
pub struct LocalStorageProfileStore;

#[cfg(target_arch = "wasm32")]
impl LocalStorageProfileStore {
    fn storage() -> Option<web_sys::Storage> {
        web_sys::window()?.local_storage().ok().flatten()
    }
}

#[cfg(target_arch = "wasm32")]
impl ProfileStore for LocalStorageProfileStore {
    fn read(&self, wallet_fp: &str) -> Option<CachedProfile> {
        let raw = Self::storage()?.get_item(&storage_key(wallet_fp)).ok()??;
        serde_json::from_str(&raw).ok()
    }

    fn write(&self, wallet_fp: &str, profile: &CachedProfile) {
        // Best-effort cache: a storage failure (private mode, quota) must not break login.
        if let (Some(storage), Ok(json)) = (Self::storage(), serde_json::to_string(profile)) {
            let _ = storage.set_item(&storage_key(wallet_fp), &json);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noop_store_never_persists() {
        let store = NoopProfileStore;
        store.write(
            "fp",
            &CachedProfile {
                owner_index: 3,
                did: "did:chia:1abc".to_owned(),
            },
        );
        assert!(store.read("fp").is_none());
    }

    #[test]
    fn storage_key_namespaces_fingerprint() {
        assert_eq!(storage_key("deadbeef"), "pegin:did:deadbeef");
    }
}
