//! DID lookup: derive puzzle hash from wallet keys → coinset.org → `did:chia:…`.
#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

use chia_protocol::Bytes32;
use chia_puzzle_types::standard::StandardArgs;

use crate::modules::keys::WalletKeys;

use super::helper::{encode_did, is_singleton_launcher};
use super::peer::{CoinRecordJson, CoinsetClient};

pub const LOOKUP_TIMEOUT_MS: u32 = 10_000;

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
    let puzzle_hash = did_puzzle_hash_from_wallet(keys);
    let hint_hex = format!("0x{}", hex::encode(puzzle_hash.to_bytes()));

    let records = client.get_coin_records_by_hint(&hint_hex).await?;
    let Some(record) = pick_unspent_did_record(&records) else {
        return Ok(None);
    };
    let coin_name = coin_id_hex(record)?;
    let launcher_id = find_launcher_id(client, &coin_name).await?;
    encode_did(&launcher_id).map(Some)
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
    loop {
        let Some(record) = client.get_coin_record_by_name(&coin_name).await? else {
            return Err("DID coin lineage not found on-chain".to_owned());
        };

        if is_singleton_launcher(&record.coin.puzzle_hash) {
            return parse_bytes32_hex(coin_name.trim_start_matches("0x"));
        }

        coin_name = record.coin.parent_coin_info;
    }
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

    let client = CoinsetRestClient::new(rest_base_url(peer_url));
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
    }

    impl MockClient {
        fn new() -> Self {
            Self {
                hints: HashMap::new(),
                coins: HashMap::new(),
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
                "get_coin_records_by_hint" => {
                    let hint = body["hint"]
                        .as_str()
                        .ok_or_else(|| "mock: missing hint".to_owned())?;
                    let records = self.hints.get(hint).cloned().unwrap_or_default();
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

    const TEST_MNEMONIC: &str = "abandon abandon abandon abandon abandon abandon \
         abandon abandon abandon abandon abandon about";
    const LAUNCHER_HEX: &str = "42fd7ee4b5735f88c58ff5ab6b3912216525e262bb99fa10dc66e4a3ec109c24";
    const SINGLETON_LAUNCHER_PH: &str =
        "eff07522495060c066f66f32acc2a77e3a3e737aca8baea4d1a64ea4cdc13da9";

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
    fn did_puzzle_hash_is_deterministic() {
        let keys = derive_wallet_keys_inner(TEST_MNEMONIC).unwrap();
        let a = did_puzzle_hash_from_wallet(&keys);
        let b = did_puzzle_hash_from_wallet(&keys);
        assert_eq!(a, b);
    }

    #[tokio::test]
    async fn returns_none_when_hint_has_no_unspent_coins() {
        let keys = derive_wallet_keys_inner(TEST_MNEMONIC).unwrap();
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
        let keys = derive_wallet_keys_inner(TEST_MNEMONIC).unwrap();
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
}
