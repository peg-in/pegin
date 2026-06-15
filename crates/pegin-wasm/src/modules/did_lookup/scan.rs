//! Browser-side DID/owner discovery over **public** coinset data.
//!
//! Finds the wallet address index whose synthetic p2 puzzle owns the DID singleton.
//! Only public puzzle hashes leave the browser — never keys or secrets. A wide batch
//! scan locates the owning window, then a binary refine pins the exact index, so even
//! a deep index (thousands) costs ~log2 queries instead of one-per-index.

use chia_bls::{DerivableKey, PublicKey};
use chia_protocol::{Bytes32, Coin};
use chia_puzzle_types::standard::StandardArgs;
use chia_puzzle_types::DeriveSynthetic;

use crate::modules::keys::WalletKeys;

use super::coinset::{CoinRecordJson, CoinsetClient};
use super::helper::{encode_did, is_singleton_launcher};

/// Address hints per coinset query while locating the owning window.
const SCAN_BATCH: u32 = 500;

/// Low addresses probed for any coin history before committing to a deep scan.
/// Wallets generate addresses sequentially under a gap limit, so an unused first
/// block means the wallet owns nothing — far smaller than any real gap.
const ACTIVITY_PROBE: u32 = 256;

/// Upper bound on parent-chain hops when resolving a DID launcher (guards bad data).
const MAX_LINEAGE_STEPS: u32 = 128;

/// Reject oversized hint responses before walking lineage.
const MAX_HINT_RECORDS: usize = 600;

/// Resolves the DID and the **owning observer index**, scanning indices `0..scan_limit`.
///
/// Returns `None` when no address key in range owns an unspent DID coin (no DID, or
/// the DID is custodied by a key outside the address range).
pub async fn resolve_did_and_owner<C: CoinsetClient>(
    client: &C,
    keys: &WalletKeys,
    scan_limit: u32,
) -> Result<Option<(String, u32)>, String> {
    let obs = keys.observer_intermediate_pk();

    // Fast path for new/unused wallets: if the first block of addresses has no coin
    // history at all, the wallet generated no further addresses (sequential gap limit)
    // and owns no DID — one query instead of scanning the whole range.
    if !wallet_has_history(client, &obs, scan_limit).await? {
        return Ok(None);
    }

    let mut start = 0;
    while start < scan_limit {
        let end = (start + SCAN_BATCH).min(scan_limit);
        if window_owns_did(client, &obs, start, end).await? {
            let index = refine_owner_index(client, &obs, start, end).await?;
            let did = resolve_did_at_index(client, &obs, index).await?;
            return Ok(Some((did, index)));
        }
        start = end;
    }
    Ok(None)
}

/// `true` when the wallet's first `ACTIVITY_PROBE` addresses ever held a coin
/// (spent or unspent). Includes spent coins so a wallet that emptied its low
/// addresses but still owns a deep DID is not mistaken for unused.
async fn wallet_has_history<C: CoinsetClient>(
    client: &C,
    obs: &PublicKey,
    scan_limit: u32,
) -> Result<bool, String> {
    let probe = ACTIVITY_PROBE.min(scan_limit);
    let hints: Vec<String> = (0..probe).map(|i| observer_hint_at(obs, i)).collect();
    let records = client.get_coin_records_by_hints_with_spent(&hints).await?;
    Ok(!records.is_empty())
}

/// `true` when some index in `[lo, hi)` hints an unspent DID coin.
async fn window_owns_did<C: CoinsetClient>(
    client: &C,
    obs: &PublicKey,
    lo: u32,
    hi: u32,
) -> Result<bool, String> {
    let hints: Vec<String> = (lo..hi).map(|i| observer_hint_at(obs, i)).collect();
    let records = client.get_coin_records_by_hints(&hints).await?;
    guard_record_count(&records)?;
    Ok(pick_unspent_did_record(&records).is_some())
}

/// Binary-searches `[lo, hi)` (known to contain an owner) for the lowest owning index.
async fn refine_owner_index<C: CoinsetClient>(
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
async fn resolve_did_at_index<C: CoinsetClient>(
    client: &C,
    obs: &PublicKey,
    index: u32,
) -> Result<String, String> {
    let hint = observer_hint_at(obs, index);
    let records = client
        .get_coin_records_by_hints(std::slice::from_ref(&hint))
        .await?;
    guard_record_count(&records)?;
    let record = pick_unspent_did_record(&records)
        .ok_or_else(|| "owner index lost its DID coin".to_owned())?;
    let launcher_id = find_launcher_id(client, &coin_id_hex(record)?).await?;
    encode_did(&launcher_id)
}

fn guard_record_count(records: &[CoinRecordJson]) -> Result<(), String> {
    if records.len() > MAX_HINT_RECORDS {
        return Err("coinset returned too many coin records".to_owned());
    }
    Ok(())
}

/// Address-hint puzzle hash for observer `index`: `StandardArgs(synthetic(obs/index))`.
/// This is the DID's on-chain owner p2 hash when the wallet owns the DID at that index.
fn observer_hint_at(obs: &PublicKey, index: u32) -> String {
    let pk = obs.derive_unhardened(index).derive_synthetic();
    hint_hex(StandardArgs::curry_tree_hash(pk).into())
}

fn hint_hex(puzzle_hash: Bytes32) -> String {
    format!("0x{}", hex::encode(puzzle_hash.to_bytes()))
}

fn pick_unspent_did_record(records: &[CoinRecordJson]) -> Option<&CoinRecordJson> {
    records
        .iter()
        .filter(|r| !r.spent && r.coin.amount == 1)
        .max_by_key(|r| r.confirmed_block_index)
}

fn coin_id_hex(record: &CoinRecordJson) -> Result<String, String> {
    let parent = parse_bytes32(&record.coin.parent_coin_info, "parent_coin_info")?;
    let puzzle_hash = parse_bytes32(&record.coin.puzzle_hash, "puzzle_hash")?;
    let coin = Coin::new(parent, puzzle_hash, record.coin.amount);
    Ok(format!("0x{}", hex::encode(coin.coin_id())))
}

async fn find_launcher_id<C: CoinsetClient>(
    client: &C,
    coin_name_hex: &str,
) -> Result<String, String> {
    let mut coin_name = coin_name_hex.to_owned();
    for _ in 0..MAX_LINEAGE_STEPS {
        let Some(record) = client.get_coin_record_by_name(&coin_name).await? else {
            return Err("DID coin lineage not found on-chain".to_owned());
        };
        if is_singleton_launcher(&record.coin.puzzle_hash) {
            return parse_bytes32_hex(coin_name.trim_start_matches("0x"));
        }
        coin_name = record.coin.parent_coin_info;
    }
    Err("DID coin lineage exceeds maximum depth".to_owned())
}

fn parse_bytes32(value: &str, field: &str) -> Result<Bytes32, String> {
    let bytes =
        hex::decode(value.trim_start_matches("0x")).map_err(|e| format!("invalid {field}: {e}"))?;
    bytes
        .try_into()
        .map(Bytes32::new)
        .map_err(|_| format!("{field} must be 32 bytes"))
}

fn parse_bytes32_hex(hex_str: &str) -> Result<String, String> {
    if hex_str.len() != 64 || !hex_str.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(format!("expected 64-char launcher hex, got '{hex_str}'"));
    }
    Ok(hex_str.to_ascii_lowercase())
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use std::cell::Cell;
    use std::collections::HashMap;

    use super::*;
    use crate::modules::keys::service::derive_wallet_keys_inner;
    use crate::test_util::deterministic_test_phrase;

    const LAUNCHER_HEX: &str = "42fd7ee4b5735f88c58ff5ab6b3912216525e262bb99fa10dc66e4a3ec109c24";
    const SINGLETON_LAUNCHER_PH: &str =
        "eff07522495060c066f66f32acc2a77e3a3e737aca8baea4d1a64ea4cdc13da9";

    struct MockClient {
        hints: HashMap<String, Vec<CoinRecordJson>>,
        coins: HashMap<String, CoinRecordJson>,
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

        fn with_hint(mut self, hint_hex: &str, records: Vec<CoinRecordJson>) -> Self {
            self.hints.insert(hint_hex.to_owned(), records);
            self
        }

        fn with_coin(mut self, name_hex: &str, record: CoinRecordJson) -> Self {
            self.coins.insert(name_hex.to_owned(), record);
            self
        }
    }

    impl CoinsetClient for MockClient {
        async fn post_json(
            &self,
            endpoint: &str,
            body: serde_json::Value,
        ) -> Result<String, String> {
            match endpoint {
                "get_coin_records_by_hints" => {
                    self.hint_queries.set(self.hint_queries.get() + 1);
                    let records: Vec<CoinRecordJson> = body["hints"]
                        .as_array()
                        .unwrap_or(&Vec::new())
                        .iter()
                        .filter_map(|h| self.hints.get(h.as_str()?))
                        .flatten()
                        .cloned()
                        .collect();
                    Ok(serde_json::json!({ "success": true, "coin_records": records }).to_string())
                }
                "get_coin_record_by_name" => {
                    let name = body["name"].as_str().unwrap_or_default();
                    let record = self.coins.get(name).cloned();
                    Ok(serde_json::json!({ "success": true, "coin_record": record }).to_string())
                }
                other => Err(format!("unexpected endpoint {other}")),
            }
        }
    }

    fn record(parent: &str, puzzle_hash: &str, spent: bool, height: u32) -> CoinRecordJson {
        CoinRecordJson {
            coin: super::super::coinset::CoinJson {
                parent_coin_info: parent.to_owned(),
                puzzle_hash: puzzle_hash.to_owned(),
                amount: 1,
            },
            spent,
            confirmed_block_index: height,
        }
    }

    /// Builds a client whose DID coin is hinted at `owner_index` for the test wallet.
    // Helper for the tests below; unwrap on fixed test vectors is intentional.
    #[allow(clippy::unwrap_used)]
    fn client_with_did_at(obs: &PublicKey, owner_index: u32) -> (MockClient, String) {
        let hint = observer_hint_at(obs, owner_index);
        let launcher_name = format!("0x{LAUNCHER_HEX}");
        let did_coin = record(
            &launcher_name,
            &format!("0x{}", "de".repeat(32)),
            false,
            200,
        );
        let did_coin_name = coin_id_hex(&did_coin).unwrap();

        let mut client = MockClient::new()
            .with_hint(&hint, vec![did_coin.clone()])
            .with_coin(&did_coin_name, did_coin)
            .with_coin(
                &launcher_name,
                record("0x00", &format!("0x{SINGLETON_LAUNCHER_PH}"), true, 100),
            );
        if owner_index != 0 {
            // A real wallet used low addresses to reach a deep index; a spent coin at
            // index 0 gives the activity probe the history it expects.
            let history_hint = observer_hint_at(obs, 0);
            client = client.with_hint(&history_hint, vec![record("0x01", "0xabc", true, 50)]);
        }
        (client, encode_did(LAUNCHER_HEX).unwrap())
    }

    #[tokio::test]
    async fn finds_owner_at_low_index() {
        let keys = derive_wallet_keys_inner(&deterministic_test_phrase()).unwrap();
        let (client, did) = client_with_did_at(&keys.observer_intermediate_pk(), 0);
        let (got_did, index) = resolve_did_and_owner(&client, &keys, 10_000)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(got_did, did);
        assert_eq!(index, 0);
    }

    #[tokio::test]
    async fn finds_owner_at_deep_index_with_few_queries() {
        let keys = derive_wallet_keys_inner(&deterministic_test_phrase()).unwrap();
        let (client, did) = client_with_did_at(&keys.observer_intermediate_pk(), 4757);
        let (got_did, index) = resolve_did_and_owner(&client, &keys, 10_000)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(got_did, did);
        assert_eq!(index, 4757);
        // Window scan (10) + binary refine (~9) + resolve/lineage — never one-per-index.
        assert!(
            client.hint_queries.get() < 40,
            "deep scan must stay logarithmic, used {} queries",
            client.hint_queries.get()
        );
    }

    #[tokio::test]
    async fn unused_wallet_returns_none_after_one_probe() {
        let keys = derive_wallet_keys_inner(&deterministic_test_phrase()).unwrap();
        let client = MockClient::new();
        assert!(resolve_did_and_owner(&client, &keys, 10_000)
            .await
            .unwrap()
            .is_none());
        assert_eq!(
            client.hint_queries.get(),
            1,
            "a new wallet must stop after the activity probe, not scan the range"
        );
    }

    #[tokio::test]
    async fn ignores_spent_did_coins() {
        let keys = derive_wallet_keys_inner(&deterministic_test_phrase()).unwrap();
        let hint = observer_hint_at(&keys.observer_intermediate_pk(), 3);
        let client = MockClient::new().with_hint(&hint, vec![record("0x01", "0xabc", true, 100)]);
        assert!(resolve_did_and_owner(&client, &keys, 2_000)
            .await
            .unwrap()
            .is_none());
    }
}
