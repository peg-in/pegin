use chia_protocol::{Bytes32, CoinSpend, CoinState, SpendBundle};
use pegin_domain::error::AppError;

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
