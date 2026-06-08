#[cfg(feature = "test-utils")]
use std::sync::{Arc, Mutex};

#[cfg(feature = "test-utils")]
use chia_protocol::{Bytes32, CoinSpend, CoinState, SpendBundle};
#[cfg(feature = "test-utils")]
use chia_sdk_test::Simulator;
#[cfg(feature = "test-utils")]
use pegin_domain::error::AppError;

#[cfg(feature = "test-utils")]
use super::service::ChiaGateway;

#[cfg(feature = "test-utils")]
pub struct SimulatorGateway {
    sim: Arc<Mutex<Simulator>>,
}

#[cfg(feature = "test-utils")]
impl SimulatorGateway {
    pub fn new(sim: Arc<Mutex<Simulator>>) -> Self {
        Self { sim }
    }
}

#[cfg(feature = "test-utils")]
impl ChiaGateway for SimulatorGateway {
    async fn get_coin_state(&self, coin_id: Bytes32) -> Result<CoinState, AppError> {
        let sim = self
            .sim
            .lock()
            .map_err(|e| AppError::Infrastructure(e.to_string()))?;
        sim.coin_state(coin_id).ok_or(AppError::NotFound)
    }

    async fn submit_transaction(&self, bundle: SpendBundle) -> Result<(), AppError> {
        let mut sim = self
            .sim
            .lock()
            .map_err(|e| AppError::Infrastructure(e.to_string()))?;
        sim.new_transaction(bundle)
            .map(|_| ())
            .map_err(|e| AppError::Infrastructure(format!("simulator: {e}")))
    }

    async fn get_puzzle_and_solution(
        &self,
        coin_id: Bytes32,
        _height: Option<u32>,
    ) -> Result<CoinSpend, AppError> {
        let sim = self
            .sim
            .lock()
            .map_err(|e| AppError::Infrastructure(e.to_string()))?;
        let (puzzle_reveal, solution) =
            sim.puzzle_and_solution(coin_id).ok_or(AppError::NotFound)?;
        let coin_state = sim.coin_state(coin_id).ok_or(AppError::NotFound)?;
        Ok(CoinSpend {
            coin: coin_state.coin,
            puzzle_reveal,
            solution,
        })
    }
}
