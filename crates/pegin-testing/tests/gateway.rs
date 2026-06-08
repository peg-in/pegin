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

/// Requires a live coinset.org connection — run manually with `cargo test -- --ignored`.
#[tokio::test]
#[ignore = "requires live coinset.org connection"]
async fn coinset_gateway_reads_known_testnet_coin() {
    use hex_literal::hex;
    use pegin_infrastructure::chia::coinset::CoinsetGateway;

    // Reward coin confirmed at testnet11 block 4242678, amount 1.75 XCH, unspent.
    let coin_id = Bytes32::from(hex!(
        "0827b6f235843af417ded1ebc5c2720403ccffd432e6c8f0a599f684aff16ae2"
    ));

    let gw = CoinsetGateway::testnet11();
    let state = gw
        .get_coin_state(coin_id)
        .await
        .expect("known testnet11 coin should be found");

    assert_eq!(state.coin.coin_id(), coin_id);
    assert_eq!(state.coin.amount, 1_750_000_000_000);
    assert_eq!(state.created_height, Some(4_242_678));
    assert!(state.spent_height.is_none(), "coin should still be unspent");
}
