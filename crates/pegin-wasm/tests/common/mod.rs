//! Shared helpers for integration tests — no English mnemonics in source.

use bip39::{Language, Mnemonic};

include!("../../test_vectors.rs");

const FIXED_TEST_ENTROPY: [u8; 32] = [0u8; 32];
const ALT_TEST_ENTROPY: [u8; 32] = [0xff; 32];

pub fn deterministic_test_phrase() -> String {
    phrase_from_entropy(&FIXED_TEST_ENTROPY)
}

pub fn alternate_test_phrase() -> String {
    phrase_from_entropy(&ALT_TEST_ENTROPY)
}

fn phrase_from_entropy(entropy: &[u8; 32]) -> String {
    Mnemonic::from_entropy_in(Language::English, entropy)
        .expect("valid 24-word test phrase")
        .to_string()
}

pub fn mnemonic_from_env() -> Option<String> {
    let env_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(".env");
    let _ = dotenvy::from_path(env_path);
    std::env::var("PEGIN_MNEMONIC").ok()
}
