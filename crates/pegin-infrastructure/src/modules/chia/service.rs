use std::env;

use chia_protocol::{Bytes32, CoinSpend, CoinState, SpendBundle};
use pegin_domain::error::AppError;
use reqwest::Client;
use serde::Deserialize;

use super::entities::{CoinRecordResponse, PushTxResponse, PuzzleAndSolutionResponse};

// Internal trait — only used within this workspace, so `Send` bound on the Future is not needed.
#[allow(async_fn_in_trait)]
pub trait ChiaGateway {
    async fn get_coin_state(&self, coin_id: Bytes32) -> Result<CoinState, AppError>;
    async fn submit_transaction(&self, bundle: SpendBundle) -> Result<(), AppError>;
    async fn get_puzzle_and_solution(
        &self,
        coin_id: Bytes32,
        height: Option<u32>,
    ) -> Result<CoinSpend, AppError>;
}

const TESTNET11_URL: &str = "https://testnet11.api.coinset.org";
const MAINNET_URL: &str = "https://api.coinset.org";

pub struct CoinsetGateway {
    base_url: String,
    client: Client,
}

impl CoinsetGateway {
    pub fn testnet11() -> Self {
        Self::new(TESTNET11_URL.to_owned())
    }

    pub fn mainnet() -> Self {
        Self::new(MAINNET_URL.to_owned())
    }

    pub fn from_env() -> Self {
        let url = env::var("CHIA_PEER_URL").unwrap_or_else(|_| TESTNET11_URL.to_owned());
        Self::new(url)
    }

    fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: Client::new(),
        }
    }

    async fn post<R: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
        body: serde_json::Value,
    ) -> Result<R, AppError> {
        let url = format!("{}/{endpoint}", self.base_url);
        self.client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::Infrastructure(e.to_string()))?
            .json::<R>()
            .await
            .map_err(|e| AppError::Infrastructure(e.to_string()))
    }
}

impl ChiaGateway for CoinsetGateway {
    async fn get_coin_state(&self, coin_id: Bytes32) -> Result<CoinState, AppError> {
        let resp: CoinRecordResponse = self
            .post(
                "get_coin_record_by_name",
                serde_json::json!({ "name": format!("0x{coin_id}") }),
            )
            .await?;
        if !resp.success {
            let msg = resp.error.unwrap_or_else(|| "unknown error".to_owned());
            return Err(AppError::Infrastructure(format!("coinset: {msg}")));
        }
        let record = resp.coin_record.ok_or(AppError::NotFound)?;
        Ok(CoinState::new(
            record.coin,
            record.spent.then_some(record.spent_block_index),
            Some(record.confirmed_block_index),
        ))
    }

    async fn submit_transaction(&self, bundle: SpendBundle) -> Result<(), AppError> {
        let coin_spends: Vec<serde_json::Value> = bundle
            .coin_spends
            .iter()
            .map(|cs| {
                serde_json::json!({
                    "coin": {
                        "amount": cs.coin.amount,
                        "parent_coin_info": format!("0x{}", cs.coin.parent_coin_info),
                        "puzzle_hash": format!("0x{}", cs.coin.puzzle_hash),
                    },
                    "puzzle_reveal": format!("0x{}", hex::encode(cs.puzzle_reveal.to_vec())),
                    "solution": format!("0x{}", hex::encode(cs.solution.to_vec())),
                })
            })
            .collect();
        let sig_hex = hex::encode(bundle.aggregated_signature.to_bytes());
        let body = serde_json::json!({
            "spend_bundle": {
                "coin_spends": coin_spends,
                "aggregated_signature": format!("0x{sig_hex}"),
            }
        });
        let resp: PushTxResponse = self.post("push_tx", body).await?;
        if !resp.success {
            let msg = resp.error.unwrap_or_else(|| "unknown error".to_owned());
            return Err(AppError::Infrastructure(format!("chia peer: {msg}")));
        }
        Ok(())
    }

    async fn get_puzzle_and_solution(
        &self,
        coin_id: Bytes32,
        height: Option<u32>,
    ) -> Result<CoinSpend, AppError> {
        let resp: PuzzleAndSolutionResponse = self
            .post(
                "get_puzzle_and_solution",
                serde_json::json!({
                    "coin_id": format!("0x{coin_id}"),
                    "height": height,
                }),
            )
            .await?;
        if !resp.success {
            let msg = resp.error.unwrap_or_else(|| "unknown error".to_owned());
            return Err(AppError::Infrastructure(format!("coinset: {msg}")));
        }
        resp.coin_solution.ok_or(AppError::NotFound)
    }
}
