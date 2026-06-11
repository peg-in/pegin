#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

use pegin_wasm::{
    derive_keys, derive_wallet_keys, get_did, hello, mint_jwt, sign_challenge, verify_jwt,
};

const TEST_MNEMONIC: &str = "abandon abandon abandon abandon abandon abandon \
     abandon abandon abandon abandon abandon about";

const KNOWN_DID_PK: &str =
    "aee8545e9cef0270cb54069a9ed81a6b1e657f68ee7e102853a0887df68f28455b79a14f86823a2b81eacc29af9d9b85";

// Set PEGIN_MNEMONIC to a testnet wallet with an on-chain DID before running manually.
const LIVE_MNEMONIC: Option<&str> = option_env!("PEGIN_MNEMONIC");

const TEST_DID: &str = "did:chia:deadbeef01020304050607080900aabbccddeeff01020304050607080900aabb";

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
fn derive_keys_alias_matches_derive_wallet_keys() {
    let via_wallet = derive_wallet_keys(TEST_MNEMONIC).expect("valid mnemonic");
    let via_keys = derive_keys(TEST_MNEMONIC).expect("valid mnemonic");
    assert_eq!(via_wallet.did_pk_hex(), via_keys.did_pk_hex());
    assert_eq!(via_wallet.wallet_pk_hex(), via_keys.wallet_pk_hex());
}

#[wasm_bindgen_test]
fn did_public_key_is_48_bytes_matching_hex() {
    let keys = derive_wallet_keys(TEST_MNEMONIC).expect("valid mnemonic");
    let bytes = keys.did_public_key();
    assert_eq!(bytes.len(), 48);
    assert_eq!(keys.did_pk_hex(), KNOWN_DID_PK);
}

#[wasm_bindgen_test]
fn derive_keys_matches_known_did_pk_in_browser() {
    let keys = derive_keys(TEST_MNEMONIC).expect("valid mnemonic");
    assert_eq!(keys.did_pk_hex(), KNOWN_DID_PK);
}

#[wasm_bindgen_test]
fn rejects_invalid_mnemonic_in_browser() {
    assert!(derive_keys("not a valid mnemonic").is_err());
}

// ── Challenge signing ─────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn sign_challenge_returns_192_hex_chars_in_browser() {
    let keys = derive_wallet_keys(TEST_MNEMONIC).expect("valid mnemonic");
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
    let token = mint_jwt(&keys, TEST_DID, 3600);

    assert_eq!(
        token.split('.').count(),
        3,
        "JWT must have 3 dot-separated parts"
    );

    assert!(verify_jwt(&token, &keys.did_public_key()));
}

#[wasm_bindgen_test]
fn tampered_jwt_fails_verification_in_browser() {
    let keys = derive_wallet_keys(TEST_MNEMONIC).expect("valid mnemonic");
    let token = mint_jwt(&keys, TEST_DID, 3600);
    let parts: Vec<&str> = token.split('.').collect();
    // base64url of {"iss":"attacker","sub":"attacker","exp":9999999999}
    const EVIL_PAYLOAD: &str =
        "eyJpc3MiOiJhdHRhY2tlciIsInN1YiI6ImF0dGFja2VyIiwiZXhwIjo5OTk5OTk5OTk5fQ";
    let tampered = format!("{}.{}.{}", parts[0], EVIL_PAYLOAD, parts[2]);
    assert!(!verify_jwt(&tampered, &keys.did_public_key()));
}

#[wasm_bindgen_test]
fn wrong_public_key_fails_verification_in_browser() {
    let keys_a = derive_wallet_keys(TEST_MNEMONIC).expect("valid mnemonic");
    let keys_b = derive_wallet_keys("zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong")
        .expect("valid mnemonic");
    let token = mint_jwt(&keys_a, TEST_DID, 3600);
    assert!(!verify_jwt(&token, &keys_b.did_public_key()));
}

// ── DID lookup (live coinset.org — manual only) ───────────────────────────────

#[wasm_bindgen_test]
async fn get_did_returns_null_for_fresh_keys_in_browser() {
    let keys = derive_wallet_keys(TEST_MNEMONIC).expect("valid mnemonic");
    let did = get_did(&keys, None).await.expect("lookup must not error");
    assert!(
        did.is_none(),
        "throwaway mnemonic must have no on-chain DID"
    );
}

#[wasm_bindgen_test]
#[ignore = "manual: requires PEGIN_MNEMONIC with a testnet DID at compile time"]
async fn get_did_live_coinset_testnet() {
    let Some(mnemonic) = LIVE_MNEMONIC else {
        return;
    };
    let keys = derive_wallet_keys(mnemonic).expect("valid mnemonic");
    let did = get_did(&keys, None)
        .await
        .expect("live lookup must not error");
    assert!(
        did.as_deref().is_some_and(|d| d.starts_with("did:chia:1")),
        "expected on-chain DID for PEGIN_MNEMONIC wallet"
    );
}
