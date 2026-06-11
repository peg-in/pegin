//! DID on-chain verification via the coinset.org REST API.

use super::helper::{encode_did, parse_launcher_id};

/// Native stub — parses and re-encodes only; the wasm branch below does the real fetch.
#[cfg(not(target_arch = "wasm32"))]
pub fn get_did_inner(did_or_launcher_id: &str, _base_url: Option<&str>) -> Result<String, String> {
    let launcher_id = parse_launcher_id(did_or_launcher_id)?;
    encode_did(&launcher_id)
}

/// Verifies the DID launcher coin exists on-chain.
///
/// * `did_or_launcher_id` — bech32m `did:chia:1...` or 64-char launcher hex
/// * `base_url` — coinset endpoint; defaults to testnet11
/// * returns the canonical bech32m DID; rejects coins that are not singleton launchers
#[cfg(target_arch = "wasm32")]
pub async fn get_did_inner(
    did_or_launcher_id: &str,
    base_url: Option<&str>,
) -> Result<String, String> {
    use gloo_net::http::Request;

    use super::helper::is_singleton_launcher;

    let launcher_id = parse_launcher_id(did_or_launcher_id)?;

    const TESTNET11_URL: &str = "https://testnet11.api.coinset.org";
    let base = base_url.unwrap_or(TESTNET11_URL);
    let url = format!("{base}/get_coin_record_by_name");
    let body = serde_json::json!({ "name": format!("0x{launcher_id}") });

    let resp = Request::post(&url)
        .json(&body)
        .map_err(|e| format!("serialisation error: {e}"))?
        .send()
        .await
        .map_err(|e| format!("network error: {e}"))?;

    #[derive(serde::Deserialize)]
    struct Coin {
        puzzle_hash: String,
    }
    #[derive(serde::Deserialize)]
    struct CoinRecord {
        coin: Coin,
    }
    #[derive(serde::Deserialize)]
    struct CoinRecordResponse {
        success: bool,
        error: Option<String>,
        coin_record: Option<CoinRecord>,
    }

    let data: CoinRecordResponse = resp.json().await.map_err(|e| format!("parse error: {e}"))?;

    if !data.success {
        let msg = data.error.unwrap_or_else(|| "unknown error".to_owned());
        return Err(format!("coinset: {msg}"));
    }

    let record = data
        .coin_record
        .ok_or_else(|| "DID coin not found on-chain".to_owned())?;

    if !is_singleton_launcher(&record.coin.puzzle_hash) {
        return Err(format!(
            "coin 0x{launcher_id} exists but is not a DID launcher (wrong puzzle hash)"
        ));
    }

    encode_did(&launcher_id)
}

// Native-only: the browser fetch path is covered by `wasm-pack test` instead.
#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    const DID_BECH32M: &str =
        "did:chia:1gt7hae94wd0c33v07k4kkwgjy9jjtcnzhwvl5yxuvmj28mqsnsjqvgw9uu";
    const LAUNCHER_HEX: &str = "42fd7ee4b5735f88c58ff5ab6b3912216525e262bb99fa10dc66e4a3ec109c24";

    #[test]
    fn rejects_invalid_launcher_id_format() {
        assert!(get_did_inner("not-hex", None).is_err());
    }

    #[test]
    fn rejects_wrong_length() {
        assert!(get_did_inner("deadbeef", None).is_err());
    }

    #[test]
    fn hex_launcher_id_returns_canonical_bech32m_did() {
        assert_eq!(get_did_inner(LAUNCHER_HEX, None).unwrap(), DID_BECH32M);
    }

    #[test]
    fn bech32m_did_passes_through_canonically() {
        assert_eq!(get_did_inner(DID_BECH32M, None).unwrap(), DID_BECH32M);
    }
}
