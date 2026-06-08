use chia_sdk_test::{BlsPairWithCoin, Simulator};

/// In-process test environment: a funded BLS wallet on a fresh simulator.
///
/// Each `TestHarness::new()` call creates an isolated instance — tests running
/// in parallel cannot share state.
#[derive(Debug)]
pub struct TestHarness {
    pub sim: Simulator,
    /// Pre-funded wallet with `1_000_000` mojo, ready to spend.
    pub wallet: BlsPairWithCoin,
}

impl TestHarness {
    pub fn new() -> Self {
        let mut sim = Simulator::new();
        let wallet = sim.bls(1_000_000);
        Self { sim, wallet }
    }
}

impl Default for TestHarness {
    fn default() -> Self {
        Self::new()
    }
}
