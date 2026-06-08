use chia_protocol::Bytes32;
use hex_literal::hex;
use pegin_infrastructure::chia::{coinset::CoinsetGateway, gateway::ChiaGateway};

/// Requires a live coinset.org connection — run manually with `cargo test -- --ignored`.
///
/// # Network-specific fixture
/// The coin ID below is tied to **testnet11**. If the target network changes
/// (e.g. mainnet or a future testnet), replace it with a coin looked up via:
///
/// ```bash
/// curl -s https://<network>.api.coinset.org/get_coin_record_by_name \
///   -H "Content-Type: application/json" \
///   -d '{"name": "0x<coin_id>"}'
/// ```
#[tokio::test]
#[ignore = "requires live coinset.org connection"]
async fn reads_known_testnet11_coin() {
    // TESTNET11 ONLY — reward coin at block 4_242_678, 1.75 XCH, unspent.
    // To switch networks: find a confirmed coin on the target network and
    // update this ID, amount, created_height, and the CoinsetGateway constructor.
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
