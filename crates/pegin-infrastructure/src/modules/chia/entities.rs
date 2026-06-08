use chia_protocol::{Coin, CoinSpend};
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct CoinRecordResponse {
    pub coin_record: Option<CoinRecordJson>,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Deserialize)]
pub(super) struct CoinRecordJson {
    pub coin: Coin,
    pub confirmed_block_index: u32,
    pub spent: bool,
    pub spent_block_index: u32,
}

#[derive(Deserialize)]
pub(super) struct PuzzleAndSolutionResponse {
    pub coin_solution: Option<CoinSpend>,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Deserialize)]
pub(super) struct PushTxResponse {
    pub success: bool,
    pub error: Option<String>,
}
