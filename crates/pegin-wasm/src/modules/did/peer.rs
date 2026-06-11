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

    async fn get_coin_records_by_hint(
        &self,
        hint_hex: &str,
    ) -> Result<Vec<CoinRecordJson>, String> {
        let body = serde_json::json!({
            "hint": hint_hex,
            "include_spent_coins": true,
        });
        let text = self.post_json("get_coin_records_by_hint", body).await?;
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

#[cfg(target_arch = "wasm32")]
impl CoinsetRestClient {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }
}

#[cfg(target_arch = "wasm32")]
impl CoinsetClient for CoinsetRestClient {
    async fn post_json(&self, endpoint: &str, body: serde_json::Value) -> Result<String, String> {
        use gloo_net::http::Request;

        let url = format!("{}/{}", self.base_url.trim_end_matches('/'), endpoint);
        let resp = Request::post(&url)
            .json(&body)
            .map_err(|e| format!("serialisation error: {e}"))?
            .send()
            .await
            .map_err(|e| format!("network error: {e}"))?;
        resp.text().await.map_err(|e| format!("network error: {e}"))
    }
}
