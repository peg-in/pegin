//! Coinset REST client for DID launcher anchoring + singleton lineage walks.

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CoinRecordResponse {
    success: bool,
    error: Option<String>,
    coin_record: Option<serde_json::Value>,
}

/// A coin's identifying fields as returned by coinset.
#[derive(Debug, Clone, Deserialize)]
pub struct CoinJson {
    pub parent_coin_info: String,
    pub puzzle_hash: String,
    pub amount: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CoinRecord {
    pub coin: CoinJson,
    pub spent: bool,
    #[serde(default)]
    pub spent_block_index: u32,
}

#[derive(Debug, Deserialize)]
struct CoinRecordsResponse {
    success: bool,
    error: Option<String>,
    coin_records: Option<Vec<CoinRecord>>,
}

#[derive(Debug, Deserialize)]
struct SingleCoinRecordResponse {
    success: bool,
    error: Option<String>,
    coin_record: Option<CoinRecord>,
}

#[derive(Debug, Deserialize)]
struct CoinSolutionJson {
    puzzle_reveal: String,
    solution: String,
}

#[derive(Debug, Deserialize)]
struct PuzzleSolutionResponse {
    success: bool,
    error: Option<String>,
    coin_solution: Option<CoinSolutionJson>,
}

/// A parent coin's spend: the CLVM puzzle reveal and solution it was spent with.
pub struct CoinSpend {
    pub puzzle_reveal: Vec<u8>,
    pub solution: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct CoinsetClient {
    base_url: String,
    http: reqwest::Client,
}

impl CoinsetClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        // Cap every coinset call so an unresponsive upstream can't hang a request.
        // `build()` only fails on TLS/resolver init — and the `default()` fallback is
        // `Client::new()`, which panics on that same failure, so there's no path to a
        // silently no-timeout client.
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap_or_default();
        Self {
            base_url: base_url.into().trim_end_matches('/').to_owned(),
            http,
        }
    }

    pub fn testnet11() -> Self {
        Self::new("https://testnet11.api.coinset.org")
    }

    /// Returns true when the launcher coin record exists on coinset (spent or unspent).
    pub async fn launcher_exists(&self, launcher_hex: &str) -> Result<bool, String> {
        let name = format!("0x{launcher_hex}");
        let url = format!("{}/get_coin_record_by_name", self.base_url);
        let resp: CoinRecordResponse = self
            .http
            .post(&url)
            .json(&serde_json::json!({ "name": name }))
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json()
            .await
            .map_err(|e| e.to_string())?;
        if !resp.success {
            let msg = resp.error.unwrap_or_else(|| "unknown error".to_owned());
            if msg.contains("not found") {
                return Ok(false);
            }
            return Err(format!("coinset: {msg}"));
        }
        Ok(resp.coin_record.is_some())
    }

    async fn post<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        body: serde_json::Value,
    ) -> Result<T, String> {
        self.http
            .post(format!("{}/{endpoint}", self.base_url))
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json()
            .await
            .map_err(|e| e.to_string())
    }

    /// Fetches a single coin record by its id (`0x…`), or `None` when absent.
    pub async fn coin_record(&self, coin_id_hex: &str) -> Result<Option<CoinRecord>, String> {
        let resp: SingleCoinRecordResponse = self
            .post(
                "get_coin_record_by_name",
                serde_json::json!({ "name": format!("0x{coin_id_hex}") }),
            )
            .await?;
        check(resp.success, resp.error)?;
        Ok(resp.coin_record)
    }

    /// Unspent coin records whose creation was hinted with one of `hints_hex` (`0x…`).
    pub async fn coins_by_hints(&self, hints_hex: &[String]) -> Result<Vec<CoinRecord>, String> {
        self.fetch_hints(hints_hex, false).await
    }

    /// Coin records for these hints **including spent** — the owner scan's activity probe
    /// detects whether an address window was ever used, not just whether it holds coins now.
    pub async fn coins_by_hints_with_spent(
        &self,
        hints_hex: &[String],
    ) -> Result<Vec<CoinRecord>, String> {
        self.fetch_hints(hints_hex, true).await
    }

    async fn fetch_hints(
        &self,
        hints_hex: &[String],
        include_spent: bool,
    ) -> Result<Vec<CoinRecord>, String> {
        let resp: CoinRecordsResponse = self
            .post(
                "get_coin_records_by_hints",
                serde_json::json!({
                    "hints": hints_hex,
                    "include_spent_coins": include_spent,
                }),
            )
            .await?;
        check(resp.success, resp.error)?;
        Ok(resp.coin_records.unwrap_or_default())
    }

    /// Coin records created by spending `parent_id_hex` (`0x…`).
    pub async fn coins_by_parent(&self, parent_id_hex: &str) -> Result<Vec<CoinRecord>, String> {
        let resp: CoinRecordsResponse = self
            .post(
                "get_coin_records_by_parent_ids",
                serde_json::json!({
                    "parent_ids": [format!("0x{parent_id_hex}")],
                    "include_spent_coins": true,
                }),
            )
            .await?;
        check(resp.success, resp.error)?;
        Ok(resp.coin_records.unwrap_or_default())
    }

    /// The puzzle reveal + solution a coin was spent with.
    pub async fn coin_spend(&self, coin_id_hex: &str, height: u32) -> Result<CoinSpend, String> {
        let resp: PuzzleSolutionResponse = self
            .post(
                "get_puzzle_and_solution",
                serde_json::json!({ "coin_id": format!("0x{coin_id_hex}"), "height": height }),
            )
            .await?;
        check(resp.success, resp.error)?;
        let cs = resp
            .coin_solution
            .ok_or_else(|| "coinset: missing coin_solution".to_owned())?;
        Ok(CoinSpend {
            puzzle_reveal: decode_hex(&cs.puzzle_reveal)?,
            solution: decode_hex(&cs.solution)?,
        })
    }
}

fn check(success: bool, error: Option<String>) -> Result<(), String> {
    if success {
        Ok(())
    } else {
        Err(format!(
            "coinset: {}",
            error.unwrap_or_else(|| "unknown error".to_owned())
        ))
    }
}

fn decode_hex(s: &str) -> Result<Vec<u8>, String> {
    hex::decode(s.trim_start_matches("0x")).map_err(|e| format!("invalid hex from coinset: {e}"))
}
