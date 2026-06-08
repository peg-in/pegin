use std::sync::{Arc, Mutex};

use chia_protocol::Bytes32;
use chia_sdk_test::Simulator;
use pegin_domain::error::AppError;
use pegin_infrastructure::chia::gateway::ChiaGateway;
use pegin_testing::SimulatorGateway;

fn gateway() -> (SimulatorGateway, Simulator) {
    let sim = Simulator::new();
    let shared = Arc::new(Mutex::new(sim.clone()));
    (SimulatorGateway::new(shared), sim)
}

#[tokio::test]
async fn get_coin_state_returns_state_for_known_coin() {
    let mut sim = Simulator::new();
    let wallet = sim.bls(1_000_000);
    let coin_id = wallet.coin.coin_id();

    let gw = SimulatorGateway::new(Arc::new(Mutex::new(sim)));

    let state = gw.get_coin_state(coin_id).await.unwrap();
    assert_eq!(state.coin, wallet.coin);
    assert!(state.spent_height.is_none());
}

#[tokio::test]
async fn get_coin_state_returns_not_found_for_unknown_coin() {
    let (gw, _sim) = gateway();
    let err = gw
        .get_coin_state(Bytes32::from([0u8; 32]))
        .await
        .unwrap_err();
    assert!(matches!(err, AppError::NotFound));
}

#[tokio::test]
async fn submit_transaction_wraps_simulator_error_in_app_error() {
    let (gw, _sim) = gateway();
    // An empty spend bundle is always invalid — gives us the Infrastructure error path.
    let empty = chia_protocol::SpendBundle::new(vec![], chia_bls::Signature::default());
    let err = gw.submit_transaction(empty).await.unwrap_err();
    assert!(matches!(err, AppError::Infrastructure(_)));
}
