//! coinset.org REST client — transport layer for DID coin lookups.
#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

use std::future::Future;

use serde::{Deserialize, Serialize};

/// Raw coin object returned by coinset.org JSON-RPC-style endpoints.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CoinJson {
    pub parent_coin_info: String,
    pub puzzle_hash: String,
    pub amount: u64,
}

/// Coin record with chain metadata.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CoinRecordJson {
    pub coin: CoinJson,
    pub spent: bool,
    pub confirmed_block_index: u32,
}

#[derive(Debug, Deserialize)]
struct CoinRecordsResponse {
    success: bool,
    error: Option<String>,
    coin_records: Option<Vec<CoinRecordJson>>,
}

#[derive(Debug, Deserialize)]
struct CoinRecordResponse {
    success: bool,
    error: Option<String>,
    coin_record: Option<CoinRecordJson>,
}

/// Swappable coinset transport (REST today; WebSocket peer protocol later).
#[allow(async_fn_in_trait)]
pub trait CoinsetClient {
    fn post_json(
        &self,
        endpoint: &str,
        body: serde_json::Value,
    ) -> impl Future<Output = Result<String, String>>;

    /// Unspent coins for every candidate hint in one batched query (bursts of
    /// per-hint requests trip Cloudflare). Finds the live DID singleton.
    async fn get_coin_records_by_hints(
        &self,
        hints_hex: &[String],
    ) -> Result<Vec<CoinRecordJson>, String> {
        self.fetch_hints(hints_hex, false).await
    }

    /// Coins for these hints **including spent** — detects whether addresses were
    /// ever used (the activity probe), not just whether they hold a balance now.
    async fn get_coin_records_by_hints_with_spent(
        &self,
        hints_hex: &[String],
    ) -> Result<Vec<CoinRecordJson>, String> {
        self.fetch_hints(hints_hex, true).await
    }

    async fn fetch_hints(
        &self,
        hints_hex: &[String],
        include_spent: bool,
    ) -> Result<Vec<CoinRecordJson>, String> {
        let body = serde_json::json!({
            "hints": hints_hex,
            "include_spent_coins": include_spent,
        });
        let text = self.post_json("get_coin_records_by_hints", body).await?;
        let data: CoinRecordsResponse =
            serde_json::from_str(&text).map_err(|e| format!("parse error: {e}"))?;
        if !data.success {
            let msg = data.error.unwrap_or_else(|| "unknown error".to_owned());
            return Err(format!("coinset: {msg}"));
        }
        Ok(data.coin_records.unwrap_or_default())
    }

    async fn get_coin_record_by_name(
        &self,
        coin_name_hex: &str,
    ) -> Result<Option<CoinRecordJson>, String> {
        let body = serde_json::json!({ "name": coin_name_hex });
        let text = self.post_json("get_coin_record_by_name", body).await?;
        let data: CoinRecordResponse =
            serde_json::from_str(&text).map_err(|e| format!("parse error: {e}"))?;
        if !data.success {
            let msg = data.error.unwrap_or_else(|| "unknown error".to_owned());
            return Err(format!("coinset: {msg}"));
        }
        Ok(data.coin_record)
    }
}

/// Live coinset.org REST client (browser fetch via gloo-net).
#[cfg(target_arch = "wasm32")]
pub struct CoinsetRestClient {
    pub base_url: String,
}

/// Per-request ceiling so one slow coin lookup can't stall the whole scan.
#[cfg(target_arch = "wasm32")]
const REQUEST_TIMEOUT_MS: u32 = 15_000;

/// Extra attempts after the first, on a per-request timeout or transient network error.
/// Reads are idempotent, so retrying is safe; a persistent failure still surfaces as `Err`.
#[cfg(target_arch = "wasm32")]
const REQUEST_RETRIES: u32 = 2;

#[cfg(target_arch = "wasm32")]
impl CoinsetRestClient {
    pub fn new(base_url: String) -> Result<Self, String> {
        if !base_url.starts_with("https://") {
            return Err("coinset client requires HTTPS".to_owned());
        }
        Ok(Self { base_url })
    }

    /// One HTTP attempt — no timeout/retry (the caller owns that).
    async fn send_once(url: &str, body: &serde_json::Value) -> Result<String, String> {
        use gloo_net::http::Request;

        let resp = Request::post(url)
            .json(body)
            .map_err(|e| format!("serialisation error: {e}"))?
            .send()
            .await
            .map_err(|e| format!("network error: {e}"))?;
        if !resp.ok() {
            return Err(format!("coinset returned HTTP {}", resp.status()));
        }
        resp.text().await.map_err(|e| format!("network error: {e}"))
    }
}

#[cfg(target_arch = "wasm32")]
impl CoinsetClient for CoinsetRestClient {
    // Each request gets its own timeout and a few retries, so one stalled coin lookup
    // is isolated and retried rather than killing a long (deep-wallet) scan.
    async fn post_json(&self, endpoint: &str, body: serde_json::Value) -> Result<String, String> {
        use futures_util::future::{select, Either};
        use gloo_timers::future::TimeoutFuture;
        use std::pin::pin;

        let url = format!("{}/{}", self.base_url.trim_end_matches('/'), endpoint);
        let mut last_err = String::new();
        for attempt in 0..=REQUEST_RETRIES {
            let send = pin!(Self::send_once(&url, &body));
            let outcome = match select(send, TimeoutFuture::new(REQUEST_TIMEOUT_MS)).await {
                Either::Left((result, _)) => result,
                Either::Right(_) => Err(format!(
                    "request timed out after {} s",
                    REQUEST_TIMEOUT_MS / 1000
                )),
            };
            match outcome {
                Ok(text) => return Ok(text),
                Err(e) => last_err = e,
            }
            if attempt < REQUEST_RETRIES {
                // Linear backoff before retrying a timed-out / failed request.
                TimeoutFuture::new(500 * (attempt + 1)).await;
            }
        }
        Err(format!(
            "coinset request to '{endpoint}' failed after {} attempts: {last_err}",
            REQUEST_RETRIES + 1
        ))
    }
}
