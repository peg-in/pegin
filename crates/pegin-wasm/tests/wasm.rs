#![cfg(target_arch = "wasm32")]

mod common;

use common::{
    deterministic_test_phrase, fresh_wallet_phrase, mnemonic_from_env, DETERMINISTIC_DID_PK,
    DETERMINISTIC_WALLET_PK,
};
use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

use pegin_wasm::{
    derive_keys, derive_wallet_keys, get_did, hello, mint_jwt, sign_challenge, verify_jwt,
};

const TEST_DID: &str = "did:chia:deadbeef01020304050607080900aabbccddeeff01020304050607080900aabb";
const TEST_AUD: &str = "https://demo.example";

// ── Scaffold smoke test ─────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn hello_returns_ready_in_browser() {
    assert_eq!(hello(), "pegin-wasm ready");
}

// ── Key derivation ────────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn derives_wallet_keys_in_browser() {
    let keys = derive_wallet_keys(&deterministic_test_phrase()).expect("valid mnemonic");
    assert_eq!(keys.wallet_pk_hex().len(), 96);
    assert_eq!(keys.did_pk_hex().len(), 96);
}

#[wasm_bindgen_test]
fn known_did_pk_vector_matches_in_browser() {
    let keys = derive_wallet_keys(&deterministic_test_phrase()).expect("valid mnemonic");
    assert_eq!(keys.did_pk_hex(), DETERMINISTIC_DID_PK);
}

#[wasm_bindgen_test]
fn known_wallet_pk_vector_matches_in_browser() {
    let keys = derive_wallet_keys(&deterministic_test_phrase()).expect("valid mnemonic");
    assert_eq!(keys.wallet_pk_hex(), DETERMINISTIC_WALLET_PK);
}

#[wasm_bindgen_test]
fn derive_keys_alias_matches_derive_wallet_keys() {
    let phrase = deterministic_test_phrase();
    let via_wallet = derive_wallet_keys(&phrase).expect("valid mnemonic");
    let via_keys = derive_keys(&phrase).expect("valid mnemonic");
    assert_eq!(via_wallet.did_pk_hex(), via_keys.did_pk_hex());
    assert_eq!(via_wallet.wallet_pk_hex(), via_keys.wallet_pk_hex());
}

#[wasm_bindgen_test]
fn did_public_key_is_48_bytes_matching_hex() {
    let keys = derive_wallet_keys(&deterministic_test_phrase()).expect("valid mnemonic");
    let bytes = keys.did_public_key();
    assert_eq!(bytes.len(), 48);
    assert_eq!(keys.did_pk_hex(), DETERMINISTIC_DID_PK);
}

#[wasm_bindgen_test]
fn derive_keys_matches_known_did_pk_in_browser() {
    let keys = derive_keys(&deterministic_test_phrase()).expect("valid mnemonic");
    assert_eq!(keys.did_pk_hex(), DETERMINISTIC_DID_PK);
}

#[wasm_bindgen_test]
fn rejects_invalid_mnemonic_in_browser() {
    assert!(derive_keys("not a valid mnemonic").is_err());
}

// ── Challenge signing ─────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn sign_challenge_returns_192_hex_chars_in_browser() {
    let keys = derive_wallet_keys(&deterministic_test_phrase()).expect("valid mnemonic");
    let sig = sign_challenge(&keys, "browser-test-nonce");
    assert_eq!(sig.len(), 192);
}

#[wasm_bindgen_test]
fn sign_challenge_is_deterministic_in_browser() {
    let keys = derive_wallet_keys(&deterministic_test_phrase()).expect("valid mnemonic");
    assert_eq!(
        sign_challenge(&keys, "same-challenge"),
        sign_challenge(&keys, "same-challenge")
    );
}

// ── JWT mint + verify ─────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn mint_and_verify_round_trip_in_browser() {
    let keys = derive_wallet_keys(&deterministic_test_phrase()).expect("valid mnemonic");
    let token = mint_jwt(&keys, TEST_DID, TEST_AUD, 3600, None).expect("mint jwt");

    assert_eq!(
        token.split('.').count(),
        3,
        "JWT must have 3 dot-separated parts"
    );

    assert!(verify_jwt(&token, TEST_AUD, None));
}

#[wasm_bindgen_test]
fn tampered_jwt_fails_verification_in_browser() {
    let keys = derive_wallet_keys(&deterministic_test_phrase()).expect("valid mnemonic");
    let token = mint_jwt(&keys, TEST_DID, TEST_AUD, 3600, None).expect("mint jwt");
    let parts: Vec<&str> = token.split('.').collect();
    // base64url of {"iss":"attacker","sub":"attacker","exp":9999999999}
    const EVIL_PAYLOAD: &str =
        "eyJpc3MiOiJhdHRhY2tlciIsInN1YiI6ImF0dGFja2VyIiwiZXhwIjo5OTk5OTk5OTk5fQ";
    let tampered = format!("{}.{}.{}", parts[0], EVIL_PAYLOAD, parts[2]);
    assert!(!verify_jwt(&tampered, TEST_AUD, None));
}

#[wasm_bindgen_test]
fn wrong_audience_fails_verification_in_browser() {
    let keys = derive_wallet_keys(&deterministic_test_phrase()).expect("valid mnemonic");
    let token = mint_jwt(&keys, TEST_DID, TEST_AUD, 3600, None).expect("mint jwt");
    assert!(!verify_jwt(&token, "https://other.example", None));
}

// ── DID lookup (live coinset.org — manual only) ───────────────────────────────

#[wasm_bindgen_test]
async fn get_did_returns_null_for_fresh_keys_in_browser() {
    let keys = derive_wallet_keys(&fresh_wallet_phrase()).expect("valid mnemonic");
    let did = get_did(&keys, None).await.expect("lookup must not error");
    assert!(
        did.is_none(),
        "fresh_wallet_phrase must have no on-chain DID on testnet11"
    );
}

#[wasm_bindgen_test]
#[ignore = "manual: set PEGIN_MNEMONIC in .env to a testnet wallet with an on-chain DID"]
async fn get_did_live_coinset_testnet() {
    let Some(mnemonic) = mnemonic_from_env() else {
        return;
    };
    let keys = derive_wallet_keys(&mnemonic).expect("valid mnemonic");
    let did = get_did(&keys, None)
        .await
        .expect("live lookup must not error");
    assert!(
        did.as_deref().is_some_and(|d| d.starts_with("did:chia:1")),
        "expected on-chain DID for PEGIN_MNEMONIC wallet"
    );
}
