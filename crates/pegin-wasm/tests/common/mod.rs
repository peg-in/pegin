//! Shared helpers for integration tests — no English mnemonics in source.

use bip39::{Language, Mnemonic};

include!("../../test_vectors.rs");

const FIXED_TEST_ENTROPY: [u8; 32] = [0u8; 32];

pub fn deterministic_test_phrase() -> String {
    phrase_from_entropy(&FIXED_TEST_ENTROPY)
}

fn phrase_from_entropy(entropy: &[u8; 32]) -> String {
    Mnemonic::from_entropy_in(Language::English, entropy)
        .expect("valid 24-word test phrase")
        .to_string()
}
