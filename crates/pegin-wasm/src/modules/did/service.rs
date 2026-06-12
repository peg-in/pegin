//! DID lookup: derive puzzle hash from wallet keys → coinset.org → `did:chia:…`.
#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

use chia_bls::DerivableKey;
use chia_protocol::Bytes32;
use chia_puzzle_types::standard::StandardArgs;
use chia_puzzle_types::DeriveSynthetic;

use crate::modules::keys::WalletKeys;

use super::helper::{encode_did, is_singleton_launcher};
use super::peer::{CoinRecordJson, CoinsetClient};

pub const LOOKUP_TIMEOUT_MS: u32 = 10_000;

/// Observer key indices scanned for wallet-created DID hints (Sage hints DID
/// coins with the owner's address puzzle hash; high indices are common).
pub const HINT_SCAN_LIMIT: u32 = 500;

/// Observer indices `0..=this` are queried in the first coinset round-trip (with the DID p2 hash).
/// Most wallets hint DIDs with a low receive address — this avoids a 500-hint payload on every login.
const HINT_QUICK_SCAN_OBSERVER_MAX: u32 = 127;

/// Upper bound on parent-chain hops when resolving a DID launcher (malicious responses).
const MAX_LINEAGE_STEPS: u32 = 128;

/// Reject oversized hint responses before walking lineage.
const MAX_HINT_RECORDS: usize = 600;

/// Resolves a DID string from derived keys, or `None` when no on-chain DID exists.
pub async fn lookup_did_for_keys<C: CoinsetClient>(
    client: &C,
    keys: &WalletKeys,
) -> Result<Option<String>, String> {
    #[cfg(target_arch = "wasm32")]
    {
        use futures_util::future::{select, Either};
        use gloo_timers::future::TimeoutFuture;
        use std::pin::pin;

        let lookup = pin!(lookup_did_for_keys_impl(client, keys));
        match select(lookup, TimeoutFuture::new(LOOKUP_TIMEOUT_MS)).await {
            Either::Left((result, _)) => result,
            Either::Right((_timeout, _)) => Err(format!(
                "request timed out after {} seconds",
                LOOKUP_TIMEOUT_MS / 1000
            )),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        lookup_did_for_keys_impl(client, keys).await
    }
}

async fn lookup_did_for_keys_impl<C: CoinsetClient>(
    client: &C,
    keys: &WalletKeys,
) -> Result<Option<String>, String> {
    if let Some(did) = lookup_did_with_hints(client, &quick_scan_hints(keys)).await? {
        return Ok(Some(did));
    }
    lookup_did_with_hints(client, &remaining_hints(keys)).await
}

async fn lookup_did_with_hints<C: CoinsetClient>(
    client: &C,
    hints: &[String],
) -> Result<Option<String>, String> {
    if hints.is_empty() {
        return Ok(None);
    }
    let records = client.get_coin_records_by_hints(hints).await?;
    if records.len() > MAX_HINT_RECORDS {
        return Err("coinset returned too many coin records".to_owned());
    }
    let Some(record) = pick_unspent_did_record(&records) else {
        return Ok(None);
    };
    let coin_name = coin_id_hex(record)?;
    let launcher_id = find_launcher_id(client, &coin_name).await?;
    encode_did(&launcher_id).map(Some)
}

fn quick_scan_hints(keys: &WalletKeys) -> Vec<String> {
    let mut hints = Vec::with_capacity(HINT_QUICK_SCAN_OBSERVER_MAX as usize + 2);
    hints.push(hint_hex(did_puzzle_hash_from_wallet(keys)));
    hints.extend((0..=HINT_QUICK_SCAN_OBSERVER_MAX).map(|i| observer_hint_at(keys, i)));
    hints
}

fn remaining_hints(keys: &WalletKeys) -> Vec<String> {
    let start = HINT_QUICK_SCAN_OBSERVER_MAX + 1;
    (start..HINT_SCAN_LIMIT)
        .map(|i| observer_hint_at(keys, i))
        .collect()
}

fn observer_hint_at(keys: &WalletKeys, index: u32) -> String {
    let pk = keys
        .observer_intermediate_pk
        .derive_unhardened(index)
        .derive_synthetic();
    hint_hex(StandardArgs::curry_tree_hash(pk).into())
}

/// Coinset hints that may carry this wallet's DID: the PEGIN DID-path p2 hash,
/// then the wallet's address puzzle hashes (synthetic observer keys), which is
/// what Sage and the reference wallet attach to DID coins.
/// Full hint list (DID p2 hash + observer indices `0..HINT_SCAN_LIMIT`); used in unit tests.
#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
pub fn candidate_hints(keys: &WalletKeys) -> Vec<String> {
    let mut hints = quick_scan_hints(keys);
    hints.append(&mut remaining_hints(keys));
    hints
}

fn hint_hex(puzzle_hash: Bytes32) -> String {
    let mut out = String::with_capacity(66);
    out.push_str("0x");
    out.push_str(&hex::encode(puzzle_hash.to_bytes()));
    out
}

/// Standard Chia p2 puzzle hash for the DID public key (coinset hint / puzzle lookup).
pub fn did_puzzle_hash_from_wallet(keys: &WalletKeys) -> Bytes32 {
    StandardArgs::curry_tree_hash(keys.did_sk.public_key()).into()
}

fn pick_unspent_did_record(records: &[CoinRecordJson]) -> Option<&CoinRecordJson> {
    records
        .iter()
        .filter(|r| !r.spent && r.coin.amount == 1)
        .max_by_key(|r| r.confirmed_block_index)
}

fn coin_id_hex(record: &CoinRecordJson) -> Result<String, String> {
    use chia_protocol::Coin;

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
    let hex_str = value.trim_start_matches("0x");
    let bytes = hex::decode(hex_str).map_err(|e| format!("invalid {field}: {e}"))?;
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

#[cfg(target_arch = "wasm32")]
pub async fn get_did_for_keys_inner(
    keys: &WalletKeys,
    peer_url: Option<&str>,
) -> Result<Option<String>, String> {
    use super::helper::rest_base_url;
    use super::peer::CoinsetRestClient;

    let client = CoinsetRestClient::new(rest_base_url(peer_url)?)?;
    lookup_did_for_keys(&client, keys).await
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn get_did_for_keys_inner(
    _keys: &WalletKeys,
    _peer_url: Option<&str>,
) -> Result<Option<String>, String> {
    // Native builds have no HTTP client; browser WASM performs the live lookup.
    std::future::ready(Ok(None)).await
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::modules::keys::service::derive_wallet_keys_inner;
    use chia_protocol::Coin;

    struct MockClient {
        hints: HashMap<String, Vec<CoinRecordJson>>,
        coins: HashMap<String, CoinRecordJson>,
        hint_queries: std::cell::Cell<usize>,
    }

    impl MockClient {
        fn new() -> Self {
            Self {
                hints: HashMap::new(),
                coins: HashMap::new(),
                hint_queries: std::cell::Cell::new(0),
            }
        }

        fn hint_query_count(&self) -> usize {
            self.hint_queries.get()
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
                    self.hint_queries
                        .set(self.hint_queries.get().saturating_add(1));
                    let hints = body["hints"]
                        .as_array()
                        .ok_or_else(|| "mock: missing hints".to_owned())?;
                    let records: Vec<CoinRecordJson> = hints
                        .iter()
                        .filter_map(|h| self.hints.get(h.as_str()?))
                        .flatten()
                        .cloned()
                        .collect();
                    Ok(serde_json::json!({ "success": true, "coin_records": records }).to_string())
                }
                "get_coin_record_by_name" => {
                    let name = body["name"]
                        .as_str()
                        .ok_or_else(|| "mock: missing name".to_owned())?;
                    let record = self.coins.get(name).cloned();
                    Ok(serde_json::json!({ "success": true, "coin_record": record }).to_string())
                }
                _ => Err(format!("unexpected endpoint {endpoint}")),
            }
        }
    }

    use crate::test_util::deterministic_test_phrase;

    const LAUNCHER_HEX: &str = "42fd7ee4b5735f88c58ff5ab6b3912216525e262bb99fa10dc66e4a3ec109c24";
    const SINGLETON_LAUNCHER_PH: &str =
        "eff07522495060c066f66f32acc2a77e3a3e737aca8baea4d1a64ea4cdc13da9";
    const NON_LAUNCHER_PH: &str =
        "0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef";

    fn record(parent: &str, puzzle_hash: &str, spent: bool, height: u32) -> CoinRecordJson {
        CoinRecordJson {
            coin: super::super::peer::CoinJson {
                parent_coin_info: parent.to_owned(),
                puzzle_hash: puzzle_hash.to_owned(),
                amount: 1,
            },
            spent,
            confirmed_block_index: height,
        }
    }

    #[test]
    fn candidate_hints_equals_quick_scan_plus_remaining() {
        let keys = derive_wallet_keys_inner(&deterministic_test_phrase()).unwrap();
        let mut merged = quick_scan_hints(&keys);
        merged.append(&mut remaining_hints(&keys));
        assert_eq!(merged, candidate_hints(&keys));
    }

    #[test]
    fn quick_scan_covers_did_hash_and_low_observer_indices() {
        let keys = derive_wallet_keys_inner(&deterministic_test_phrase()).unwrap();
        let quick = quick_scan_hints(&keys);
        assert_eq!(quick.len(), 1 + (HINT_QUICK_SCAN_OBSERVER_MAX as usize + 1));
        assert_eq!(quick[0], hint_hex(did_puzzle_hash_from_wallet(&keys)));
        assert_eq!(quick[1], candidate_hints(&keys)[1]);
    }

    #[test]
    fn did_puzzle_hash_is_deterministic() {
        let keys = derive_wallet_keys_inner(&deterministic_test_phrase()).unwrap();
        let a = did_puzzle_hash_from_wallet(&keys);
        let b = did_puzzle_hash_from_wallet(&keys);
        assert_eq!(a, b);
    }

    #[test]
    fn candidate_hints_start_with_did_path_then_observer_addresses() {
        let keys = derive_wallet_keys_inner(&deterministic_test_phrase()).unwrap();
        let hints = candidate_hints(&keys);

        assert_eq!(hints.len(), 1 + HINT_SCAN_LIMIT as usize);
        assert_eq!(hints[0], hint_hex(did_puzzle_hash_from_wallet(&keys)));
        // All candidates are distinct 0x-prefixed 32-byte hashes.
        let unique: std::collections::HashSet<_> = hints.iter().collect();
        assert_eq!(unique.len(), hints.len());
        assert!(hints.iter().all(|h| h.len() == 66 && h.starts_with("0x")));
    }

    #[tokio::test]
    async fn resolves_did_hinted_by_observer_address() {
        let keys = derive_wallet_keys_inner(&deterministic_test_phrase()).unwrap();
        // Sage-style hint: the wallet's first address puzzle hash, not the DID path.
        let observer_hint = candidate_hints(&keys)[1].clone();

        let launcher_name = format!("0x{LAUNCHER_HEX}");
        let did_coin = record(
            &launcher_name,
            "0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
            false,
            200,
        );
        let did_coin_name = coin_id_hex(&did_coin).unwrap();

        let client = MockClient::new()
            .with_hint(&observer_hint, vec![did_coin.clone()])
            .with_coin(&did_coin_name, did_coin)
            .with_coin(
                &launcher_name,
                record("0x00", &format!("0x{SINGLETON_LAUNCHER_PH}"), true, 100),
            );

        let result = lookup_did_for_keys(&client, &keys).await.unwrap();
        assert_eq!(
            result.as_deref(),
            Some(encode_did(LAUNCHER_HEX).unwrap().as_str())
        );
        assert_eq!(
            client.hint_query_count(),
            1,
            "low observer index must resolve in the first hint scan phase"
        );
    }

    #[tokio::test]
    async fn resolves_did_in_second_hint_scan_phase() {
        let keys = derive_wallet_keys_inner(&deterministic_test_phrase()).unwrap();
        let late_index = HINT_QUICK_SCAN_OBSERVER_MAX + 50;
        assert!(late_index < HINT_SCAN_LIMIT);
        let observer_hint = candidate_hints(&keys)[1 + late_index as usize].clone();

        let launcher_name = format!("0x{LAUNCHER_HEX}");
        let did_coin = record(
            &launcher_name,
            "0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
            false,
            200,
        );
        let did_coin_name = coin_id_hex(&did_coin).unwrap();

        let client = MockClient::new()
            .with_hint(&observer_hint, vec![did_coin.clone()])
            .with_coin(&did_coin_name, did_coin)
            .with_coin(
                &launcher_name,
                record("0x00", &format!("0x{SINGLETON_LAUNCHER_PH}"), true, 100),
            );

        let result = lookup_did_for_keys(&client, &keys).await.unwrap();
        assert_eq!(
            result.as_deref(),
            Some(encode_did(LAUNCHER_HEX).unwrap().as_str())
        );
        assert_eq!(
            client.hint_query_count(),
            2,
            "late observer index requires the second hint scan phase"
        );
    }

    #[tokio::test]
    async fn returns_none_when_hint_has_no_unspent_coins() {
        let keys = derive_wallet_keys_inner(&deterministic_test_phrase()).unwrap();
        let hint = format!(
            "0x{}",
            hex::encode(did_puzzle_hash_from_wallet(&keys).to_bytes())
        );
        let client = MockClient::new().with_hint(&hint, vec![record("0x01", "0xabc", true, 100)]);

        let result = lookup_did_for_keys(&client, &keys).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn resolves_did_from_hint_and_launcher_chain() {
        let keys = derive_wallet_keys_inner(&deterministic_test_phrase()).unwrap();
        let hint = format!(
            "0x{}",
            hex::encode(did_puzzle_hash_from_wallet(&keys).to_bytes())
        );

        let launcher_name = format!("0x{LAUNCHER_HEX}");
        let did_coin = record(
            &launcher_name,
            "0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
            false,
            200,
        );

        let parent = parse_bytes32(&launcher_name, "parent").unwrap();
        let puzzle_hash = parse_bytes32(
            "0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
            "puzzle_hash",
        )
        .unwrap();
        let did_coin_name = format!(
            "0x{}",
            hex::encode(Coin::new(parent, puzzle_hash, 1).coin_id())
        );

        let client = MockClient::new()
            .with_hint(&hint, vec![did_coin.clone()])
            .with_coin(&did_coin_name, did_coin)
            .with_coin(
                &launcher_name,
                record("0x00", &format!("0x{SINGLETON_LAUNCHER_PH}"), true, 100),
            );

        let result = lookup_did_for_keys(&client, &keys).await.unwrap();
        assert_eq!(
            result.as_deref(),
            Some(encode_did(LAUNCHER_HEX).unwrap().as_str())
        );
    }

    #[tokio::test]
    async fn rejects_lineage_deeper_than_max_steps() {
        let keys = derive_wallet_keys_inner(&deterministic_test_phrase()).unwrap();
        let hint = format!(
            "0x{}",
            hex::encode(did_puzzle_hash_from_wallet(&keys).to_bytes())
        );

        let names: Vec<String> = (0..=MAX_LINEAGE_STEPS)
            .map(|i| format!("0x{:064x}", i + 1000))
            .collect();

        let did_coin = record(&names[0], NON_LAUNCHER_PH, false, 200);
        let did_coin_name = coin_id_hex(&did_coin).unwrap();

        let mut client = MockClient::new()
            .with_hint(&hint, vec![did_coin.clone()])
            .with_coin(&did_coin_name, did_coin);

        for i in 0..MAX_LINEAGE_STEPS as usize {
            client = client.with_coin(
                &names[i],
                record(&names[i + 1], NON_LAUNCHER_PH, false, 100),
            );
        }
        client = client.with_coin(
            &names[MAX_LINEAGE_STEPS as usize],
            record("0x9999", NON_LAUNCHER_PH, false, 50),
        );

        let err = lookup_did_for_keys(&client, &keys).await.unwrap_err();
        assert!(err.contains("maximum depth"));
    }
}
