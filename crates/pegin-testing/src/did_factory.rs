use chia_protocol::Bytes32;
use pegin_domain::did::Did;

use crate::harness::TestHarness;

// Returned by [`create_did`] — the DID and the underlying launcher coin id.
#[derive(Debug, Clone)]
pub struct DidInfo {
    pub did: Did,
    // The launcher coin id that anchors the DID on the simulator.
    pub launcher_id: Bytes32,
}

// Registers a synthetic DID on the simulator and returns its identity info.
// A fresh coin is inserted with the wallet's puzzle hash; its `coin_id` becomes the launcher id (`did:chia:<64-hex-char>`).
// No real singleton CLVM runs — that lives in feat-8.  This is enough for test scenarios that only need a valid DID value in scope.
// `Bytes32::Display` always produces 64 lowercase hex chars, so the `expect` is unreachable.
#[allow(clippy::expect_used, clippy::missing_panics_doc)]
pub fn create_did(harness: &mut TestHarness, _username: &str) -> DidInfo {
    let launcher_coin = harness.sim.new_coin(harness.wallet.puzzle_hash, 1);
    let launcher_id: Bytes32 = launcher_coin.coin_id();
    // Bytes32 Display is lowercase hex — produces the 64-char launcher id.
    // Bytes32 Display is lowercase hex — the resulting DID is always valid.
    let did = Did::try_from(format!("did:chia:{launcher_id}"))
        .expect("Bytes32 hex is always a valid launcher_id");
    DidInfo { did, launcher_id }
}
