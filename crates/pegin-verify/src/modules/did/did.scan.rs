//! Relay-side DID/owner-index discovery over **public** coinset data (feat-37).
//!
//! The browser no longer scans the chain: it sends the watch-only observer account key
//! and the relay finds the address index whose synthetic p2 puzzle owns the DID singleton.
//! Only public puzzle hashes are queried — no secret ever reaches the relay. A wide batch
//! scan locates the owning window, then a binary refine pins the index, so even a deep
//! index (thousands) costs ~log2 queries instead of one-per-index.

use std::future::Future;

use chia_bls_020::DerivableKey;
use chia_puzzle_types::standard::StandardArgs;
use chia_puzzle_types::DeriveSynthetic;
use chia_wallet_sdk::prelude::PublicKey;

use super::did_helper::{encode_did, is_singleton_launcher};
use super::did_repository::{CoinRecord, CoinsetClient};
use super::resolve::{coin_id_hex, pick_unspent_did};

/// Address hints per coinset query while locating the owning window.
const SCAN_BATCH: u32 = 500;

/// Low addresses probed for any coin history before committing to a deep scan.
/// Wallets generate addresses sequentially under a gap limit, so an unused first
/// block means the wallet owns nothing — far smaller than any real gap.
const ACTIVITY_PROBE: u32 = 256;

/// Upper bound on parent-chain hops while resolving a DID launcher (guards bad data).
const MAX_LINEAGE_STEPS: u32 = 128;

/// Reject oversized hint responses before walking lineage.
const MAX_HINT_RECORDS: usize = 600;

/// The three coin reads the owner scan needs. Abstracted so the algorithm is unit-tested
/// without live coinset traffic; [`CoinsetClient`] is the only production implementation.
pub(super) trait HintSource {
    fn unspent_by_hints(
        &self,
        hints: &[String],
    ) -> impl Future<Output = Result<Vec<CoinRecord>, String>>;
    fn any_by_hints(
        &self,
        hints: &[String],
    ) -> impl Future<Output = Result<Vec<CoinRecord>, String>>;
    fn record(&self, coin_id_hex: &str)
        -> impl Future<Output = Result<Option<CoinRecord>, String>>;
}

impl HintSource for CoinsetClient {
    async fn unspent_by_hints(&self, hints: &[String]) -> Result<Vec<CoinRecord>, String> {
        self.coins_by_hints(hints).await
    }
    async fn any_by_hints(&self, hints: &[String]) -> Result<Vec<CoinRecord>, String> {
        self.coins_by_hints_with_spent(hints).await
    }
    async fn record(&self, coin_id_hex: &str) -> Result<Option<CoinRecord>, String> {
        self.coin_record(coin_id_hex).await
    }
}

/// Resolves the DID and its **owning observer index**, scanning indices `0..scan_limit`.
///
/// * `account_pk` — watch-only observer intermediate key (m/12381/8444/2) the wallet sent
/// * returns `None` when no address key in range owns an unspent DID coin
pub async fn resolve_did_and_owner(
    client: &CoinsetClient,
    account_pk: &PublicKey,
    scan_limit: u32,
) -> Result<Option<(String, u32)>, String> {
    scan_owner(client, account_pk, scan_limit).await
}

/// Generic over [`HintSource`] so the algorithm is exercised against a mock; the public
/// entry point above pins the concrete coinset client.
async fn scan_owner<C: HintSource>(
    client: &C,
    account_pk: &PublicKey,
    scan_limit: u32,
) -> Result<Option<(String, u32)>, String> {
    // Fast path for new/unused wallets: if the first block of addresses has no coin
    // history at all, the wallet generated no further addresses (sequential gap limit)
    // and owns no DID — one query instead of scanning the whole range.
    if !wallet_has_history(client, account_pk, scan_limit).await? {
        return Ok(None);
    }

    let mut start = 0;
    while start < scan_limit {
        let end = (start + SCAN_BATCH).min(scan_limit);
        if window_owns_did(client, account_pk, start, end).await? {
            let index = refine_owner_index(client, account_pk, start, end).await?;
            let did = resolve_did_at_index(client, account_pk, index).await?;
            return Ok(Some((did, index)));
        }
        start = end;
    }
    Ok(None)
}

/// `true` when the wallet's first `ACTIVITY_PROBE` addresses ever held a coin (spent or
/// unspent). Spent coins count so a wallet that emptied low addresses but still owns a
/// deep DID is not mistaken for unused.
async fn wallet_has_history<C: HintSource>(
    client: &C,
    obs: &PublicKey,
    scan_limit: u32,
) -> Result<bool, String> {
    let probe = ACTIVITY_PROBE.min(scan_limit);
    let hints: Vec<String> = (0..probe).map(|i| observer_hint_at(obs, i)).collect();
    let records = client.any_by_hints(&hints).await?;
    Ok(!records.is_empty())
}

/// `true` when some index in `[lo, hi)` hints an unspent DID coin.
async fn window_owns_did<C: HintSource>(
    client: &C,
    obs: &PublicKey,
    lo: u32,
    hi: u32,
) -> Result<bool, String> {
    let hints: Vec<String> = (lo..hi).map(|i| observer_hint_at(obs, i)).collect();
    let records = client.unspent_by_hints(&hints).await?;
    guard_record_count(&records)?;
    Ok(pick_unspent_did(&records).is_some())
}

/// Binary-searches `[lo, hi)` (known to contain an owner) for the lowest owning index.
async fn refine_owner_index<C: HintSource>(
    client: &C,
    obs: &PublicKey,
    mut lo: u32,
    mut hi: u32,
) -> Result<u32, String> {
    while hi - lo > 1 {
        let mid = lo + (hi - lo) / 2;
        if window_owns_did(client, obs, lo, mid).await? {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    Ok(lo)
}

/// Resolves the canonical DID owned by the address key at `index`.
async fn resolve_did_at_index<C: HintSource>(
    client: &C,
    obs: &PublicKey,
    index: u32,
) -> Result<String, String> {
    let hint = observer_hint_at(obs, index);
    let records = client.unspent_by_hints(std::slice::from_ref(&hint)).await?;
    guard_record_count(&records)?;
    let record =
        pick_unspent_did(&records).ok_or_else(|| "owner index lost its DID coin".to_owned())?;
    let launcher = find_launcher_id(client, &coin_id_hex(record)?).await?;
    encode_did(&launcher)
}

/// Walks the parent chain from a DID coin up to its singleton launcher.
async fn find_launcher_id<C: HintSource>(client: &C, coin_id_hex: &str) -> Result<String, String> {
    let mut coin_name = coin_id_hex.to_owned();
    for _ in 0..MAX_LINEAGE_STEPS {
        let Some(record) = client.record(&coin_name).await? else {
            return Err("DID coin lineage not found on-chain".to_owned());
        };
        if is_singleton_launcher(&record.coin.puzzle_hash) {
            return Ok(coin_name.trim_start_matches("0x").to_ascii_lowercase());
        }
        coin_name = record
            .coin
            .parent_coin_info
            .trim_start_matches("0x")
            .to_ascii_lowercase();
    }
    Err("DID coin lineage exceeds maximum depth".to_owned())
}

fn guard_record_count(records: &[CoinRecord]) -> Result<(), String> {
    if records.len() > MAX_HINT_RECORDS {
        return Err("coinset returned too many coin records".to_owned());
    }
    Ok(())
}

/// Address-hint puzzle hash for observer `index`: `StandardArgs(synthetic(obs/index))`.
/// This is the DID's on-chain owner p2 hash when the wallet owns the DID at that index.
fn observer_hint_at(obs: &PublicKey, index: u32) -> String {
    let pk = obs.derive_unhardened(index).derive_synthetic();
    format!(
        "0x{}",
        hex::encode(<[u8; 32]>::from(StandardArgs::curry_tree_hash(pk)))
    )
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::collections::HashMap;

    use super::super::did_repository::CoinJson;
    use super::*;

    const LAUNCHER_HEX: &str = "1111111111111111111111111111111111111111111111111111111111111111";
    const SINGLETON_LAUNCHER_PH: &str =
        "eff07522495060c066f66f32acc2a77e3a3e737aca8baea4d1a64ea4cdc13da9";

    // A throwaway observer key: any valid G1 point works as the scan root in tests.
    fn test_observer_pk() -> PublicKey {
        use chia_bls_020::SecretKey;
        SecretKey::from_seed(&[7u8; 32]).public_key()
    }

    struct MockClient {
        hints: HashMap<String, Vec<CoinRecord>>,
        coins: HashMap<String, CoinRecord>,
        hint_queries: Cell<usize>,
    }

    impl MockClient {
        fn new() -> Self {
            Self {
                hints: HashMap::new(),
                coins: HashMap::new(),
                hint_queries: Cell::new(0),
            }
        }

        fn with_hint(mut self, hint_hex: &str, records: Vec<CoinRecord>) -> Self {
            self.hints.insert(hint_hex.to_owned(), records);
            self
        }

        fn with_coin(mut self, name_hex: &str, record: CoinRecord) -> Self {
            self.coins.insert(name_hex.to_owned(), record);
            self
        }

        fn gather(&self, hints: &[String], include_spent: bool) -> Vec<CoinRecord> {
            hints
                .iter()
                .filter_map(|h| self.hints.get(h))
                .flatten()
                .filter(|r| include_spent || !r.spent)
                .cloned()
                .collect()
        }
    }

    impl HintSource for MockClient {
        async fn unspent_by_hints(&self, hints: &[String]) -> Result<Vec<CoinRecord>, String> {
            self.hint_queries.set(self.hint_queries.get() + 1);
            Ok(self.gather(hints, false))
        }
        async fn any_by_hints(&self, hints: &[String]) -> Result<Vec<CoinRecord>, String> {
            self.hint_queries.set(self.hint_queries.get() + 1);
            Ok(self.gather(hints, true))
        }
        async fn record(&self, coin_id_hex: &str) -> Result<Option<CoinRecord>, String> {
            Ok(self
                .coins
                .get(coin_id_hex.trim_start_matches("0x"))
                .cloned())
        }
    }

    fn record(parent: &str, puzzle_hash: &str, spent: bool) -> CoinRecord {
        CoinRecord {
            coin: CoinJson {
                parent_coin_info: parent.to_owned(),
                puzzle_hash: puzzle_hash.to_owned(),
                amount: 1,
            },
            spent,
            spent_block_index: 0,
        }
    }

    // `record`/`coin_id_hex` operate on fixed vectors; unwrap is intentional in tests.
    #[allow(clippy::unwrap_used)]
    fn client_with_did_at(obs: &PublicKey, owner_index: u32) -> (MockClient, String) {
        let hint = observer_hint_at(obs, owner_index);
        let did_coin = record(LAUNCHER_HEX, &"de".repeat(32), false);
        let did_coin_name = coin_id_hex(&did_coin).unwrap();

        let mut client = MockClient::new()
            .with_hint(&hint, vec![did_coin.clone()])
            .with_coin(&did_coin_name, did_coin)
            .with_coin(LAUNCHER_HEX, record("00", SINGLETON_LAUNCHER_PH, true));
        if owner_index != 0 {
            // A real wallet used low addresses to reach a deep index; a spent coin at
            // index 0 gives the activity probe the history it expects.
            let history_hint = observer_hint_at(obs, 0);
            client = client.with_hint(&history_hint, vec![record("01", "abc", true)]);
        }
        (client, encode_did(LAUNCHER_HEX).unwrap())
    }

    #[tokio::test]
    async fn finds_owner_at_low_index() {
        let obs = test_observer_pk();
        let (client, did) = client_with_did_at(&obs, 0);
        let (got_did, index) = scan_owner(&client, &obs, 10_000).await.unwrap().unwrap();
        assert_eq!(got_did, did);
        assert_eq!(index, 0);
    }

    #[tokio::test]
    async fn finds_owner_at_deep_index_with_few_queries() {
        let obs = test_observer_pk();
        let (client, did) = client_with_did_at(&obs, 4757);
        let (got_did, index) = scan_owner(&client, &obs, 10_000).await.unwrap().unwrap();
        assert_eq!(got_did, did);
        assert_eq!(index, 4757);
        // Window scan + binary refine + lineage — never one query per index.
        assert!(
            client.hint_queries.get() < 40,
            "deep scan must stay logarithmic, used {} queries",
            client.hint_queries.get()
        );
    }

    #[tokio::test]
    async fn unused_wallet_returns_none_after_one_probe() {
        let obs = test_observer_pk();
        let client = MockClient::new();
        assert!(scan_owner(&client, &obs, 10_000).await.unwrap().is_none());
        assert_eq!(
            client.hint_queries.get(),
            1,
            "a new wallet must stop after the activity probe, not scan the range"
        );
    }

    #[tokio::test]
    async fn ignores_spent_did_coins() {
        let obs = test_observer_pk();
        let hint = observer_hint_at(&obs, 3);
        let client = MockClient::new().with_hint(&hint, vec![record("01", "abc", true)]);
        assert!(scan_owner(&client, &obs, 2_000).await.unwrap().is_none());
    }
}
