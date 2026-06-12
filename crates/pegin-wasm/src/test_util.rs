//! Test-only helpers — no English mnemonics in source; real wallets load from `.env`.

pub use crate::test_vectors::{DETERMINISTIC_DID_PK, DETERMINISTIC_WALLET_PK};

use bip39::{Language, Mnemonic};

/// Fixed entropy for deterministic offline tests (32 zero bytes → valid 24-word phrase).
const FIXED_TEST_ENTROPY: [u8; 32] = [0u8; 32];

/// Second fixed entropy for tests that need a different valid wallet.
const ALT_TEST_ENTROPY: [u8; 32] = [0xff; 32];

/// Builds a valid 24-word phrase from fixed zero entropy — no mnemonic strings in the repo.
pub fn deterministic_test_phrase() -> String {
    phrase_from_entropy(&FIXED_TEST_ENTROPY)
}

/// Builds a second valid phrase (all-0xff entropy) for wrong-key / cross-wallet tests.
pub fn alternate_test_phrase() -> String {
    phrase_from_entropy(&ALT_TEST_ENTROPY)
}

fn phrase_from_entropy(entropy: &[u8; 32]) -> String {
    Mnemonic::from_entropy_in(Language::English, entropy)
        .expect("valid 24-word test phrase")
        .to_string()
}
