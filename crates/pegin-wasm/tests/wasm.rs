#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

use pegin_wasm::{derive_wallet_keys, hello, mint_jwt, sign_challenge, verify_jwt};

const TEST_MNEMONIC: &str = "abandon abandon abandon abandon abandon abandon \
     abandon abandon abandon abandon abandon about";

const KNOWN_DID_PK: &str =
    "aee8545e9cef0270cb54069a9ed81a6b1e657f68ee7e102853a0887df68f28455b79a14f86823a2b81eacc29af9d9b85";

// ── Scaffold smoke test ─────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn hello_returns_ready_in_browser() {
    assert_eq!(hello(), "pegin-wasm ready");
}

// ── Key derivation ────────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn derives_wallet_keys_in_browser() {
    let keys = derive_wallet_keys(TEST_MNEMONIC).expect("valid mnemonic");
    assert_eq!(keys.wallet_pk_hex().len(), 96);
    assert_eq!(keys.did_pk_hex().len(), 96);
}

#[wasm_bindgen_test]
fn known_did_pk_vector_matches_in_browser() {
    let keys = derive_wallet_keys(TEST_MNEMONIC).expect("valid mnemonic");
    assert_eq!(keys.did_pk_hex(), KNOWN_DID_PK);
}

#[wasm_bindgen_test]
fn rejects_invalid_mnemonic_in_browser() {
    assert!(derive_wallet_keys("not a valid mnemonic").is_err());
}

// ── Challenge signing ─────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn sign_challenge_returns_192_hex_chars_in_browser() {
    let keys = derive_wallet_keys(TEST_MNEMONIC).expect("valid mnemonic");
    // BLS G2 signature = 96 bytes = 192 hex chars
    let sig = sign_challenge(&keys, "browser-test-nonce");
    assert_eq!(sig.len(), 192);
}

#[wasm_bindgen_test]
fn sign_challenge_is_deterministic_in_browser() {
    let keys = derive_wallet_keys(TEST_MNEMONIC).expect("valid mnemonic");
    assert_eq!(
        sign_challenge(&keys, "same-challenge"),
        sign_challenge(&keys, "same-challenge")
    );
}

// ── JWT mint + verify ─────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn mint_and_verify_round_trip_in_browser() {
    let keys = derive_wallet_keys(TEST_MNEMONIC).expect("valid mnemonic");
    let did = "did:chia:deadbeef01020304050607080900aabbccddeeff01020304050607080900aabb";
    let token = mint_jwt(&keys, did, "https://app.example.com", 3600);

    assert_eq!(
        token.split('.').count(),
        3,
        "JWT must have 3 dot-separated parts"
    );

    let valid = verify_jwt(&token, &keys.did_pk_hex()).expect("verify must not error");
    assert!(valid, "freshly minted JWT must verify");
}

#[wasm_bindgen_test]
fn wrong_public_key_fails_verification_in_browser() {
    let keys_a = derive_wallet_keys(TEST_MNEMONIC).expect("valid mnemonic");
    let keys_b = derive_wallet_keys("zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong")
        .expect("valid mnemonic");
    let did = "did:chia:deadbeef01020304050607080900aabbccddeeff01020304050607080900aabb";
    let token = mint_jwt(&keys_a, did, "https://app.example.com", 3600);

    let valid = verify_jwt(&token, &keys_b.did_pk_hex()).expect("verify must not error");
    assert!(!valid, "JWT must not verify against a different public key");
}
