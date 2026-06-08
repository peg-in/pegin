use std::sync::{Arc, Mutex};

use chia_protocol::{Bytes32, CoinSpend, CoinState, SpendBundle};
use chia_sdk_test::Simulator;
use pegin_domain::error::AppError;

use super::gateway::ChiaGateway;

pub struct SimulatorGateway {
    sim: Arc<Mutex<Simulator>>,
}

impl SimulatorGateway {
    pub fn new(sim: Arc<Mutex<Simulator>>) -> Self {
        Self { sim }
    }
}

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
