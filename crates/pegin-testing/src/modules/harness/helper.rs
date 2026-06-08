use chia_protocol::Bytes32;
use pegin_domain::{did::Did, identity::Username};

use super::service::TestHarness;

#[derive(Debug, Clone)]
pub struct DidInfo {
    pub did: Did,
    pub launcher_id: Bytes32,
}

// No real singleton CLVM runs — that lives in feat-8. This is enough for
// test scenarios that only need a valid DID value in scope.
// `Bytes32::Display` always produces 64 lowercase hex chars, so the `expect` is unreachable.
#[allow(clippy::expect_used, clippy::missing_panics_doc)]
pub fn create_did(harness: &mut TestHarness, _username: &str) -> DidInfo {
    let launcher_coin = harness.sim.new_coin(harness.wallet.puzzle_hash, 1);
    let launcher_id: Bytes32 = launcher_coin.coin_id();
    let did = Did::try_from(format!("did:chia:{launcher_id}"))
        .expect("Bytes32 hex is always a valid launcher_id");
    DidInfo { did, launcher_id }
}

// Literal is a hardcoded compile-time constant — panic is unreachable in practice.
#[allow(clippy::expect_used, clippy::missing_panics_doc)]
pub fn test_did() -> Did {
    Did::try_from("did:chia:deadbeef000000000000000000000000000000000000000000000000cafebabe")
        .expect("test DID is valid")
}

// Literal is a hardcoded compile-time constant — panic is unreachable in practice.
#[allow(clippy::expect_used, clippy::missing_panics_doc)]
pub fn test_username() -> Username {
    Username::try_from("alice").expect("test username is valid")
}
