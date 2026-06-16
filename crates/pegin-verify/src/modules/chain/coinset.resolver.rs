//! `CoinsetResolver` — the M1 `ChainResolver` backed by coinset.org REST.

use chia_wallet_sdk::prelude::PublicKey;

use super::resolver::{ChainResolver, ResolveError};
use crate::modules::did::{
    launcher_id_hex, resolve_did_and_owner, verify_did_owner, CoinsetClient,
};

/// Highest address index the owner scan probes before giving up.
const DEFAULT_SCAN_LIMIT: u32 = 10_000;

/// Resolves DIDs and verifies ownership against coinset.org (temporary backend).
#[derive(Clone)]
pub struct CoinsetResolver {
    client: CoinsetClient,
    scan_limit: u32,
}

impl CoinsetResolver {
    pub fn new(client: CoinsetClient) -> Self {
        Self {
            client,
            scan_limit: DEFAULT_SCAN_LIMIT,
        }
    }

    /// Overrides the address-scan ceiling (env-tunable on the server).
    #[must_use]
    pub fn with_scan_limit(mut self, scan_limit: u32) -> Self {
        if scan_limit > 0 {
            self.scan_limit = scan_limit;
        }
        self
    }
}

impl ChainResolver for CoinsetResolver {
    async fn resolve_owner(&self, account_pk: &PublicKey) -> Result<(String, u32), ResolveError> {
        match resolve_did_and_owner(&self.client, account_pk, self.scan_limit).await {
            Ok(Some(found)) => Ok(found),
            Ok(None) => Err(ResolveError::NotFound),
            Err(e) => Err(ResolveError::Upstream(e)),
        }
    }

    async fn verify_did_owner(
        &self,
        did: &str,
        owner_pk: &PublicKey,
    ) -> Result<bool, ResolveError> {
        let launcher = launcher_id_hex(did).map_err(ResolveError::Invalid)?;
        verify_did_owner(&self.client, &launcher, &owner_pk.to_bytes())
            .await
            .map_err(ResolveError::Upstream)
    }
}
