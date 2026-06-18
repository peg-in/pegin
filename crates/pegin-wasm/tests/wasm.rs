#![cfg(target_arch = "wasm32")]

mod common;

use common::{deterministic_test_phrase, DETERMINISTIC_DID_PK, DETERMINISTIC_WALLET_PK};
use js_sys::Reflect;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

use pegin_wasm::{
    derive_keys, derive_wallet_keys, hello, identity_key, mint_jwt, sign_challenge, sign_login,
    verify_jwt,
};

fn js_field(obj: &JsValue, key: &str) -> JsValue {
    Reflect::get(obj, &JsValue::from_str(key)).expect("readable object field")
}

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

// ── Watch-only identity key (relay resolves it — no chain I/O in the browser) ──

#[wasm_bindgen_test]
fn identity_key_exposes_account_pk_in_browser() {
    let keys = derive_wallet_keys(&deterministic_test_phrase()).expect("valid mnemonic");
    let identity = identity_key(&keys);
    let account_pk = js_field(&identity, "accountPk")
        .as_string()
        .expect("accountPk is a string");
    assert_eq!(
        account_pk.len(),
        96,
        "accountPk is a 48-byte BLS key in hex"
    );
    assert_eq!(account_pk, keys.account_pk_hex());
}

// ── Login signing (relay-resolved DID + owner index → JWT + challenge sig) ─────

#[wasm_bindgen_test]
fn sign_login_mints_verifiable_jwt_in_browser() {
    let keys = derive_wallet_keys(&deterministic_test_phrase()).expect("valid mnemonic");
    let result = sign_login(
        &keys,
        TEST_DID,
        0,
        TEST_AUD,
        3600,
        Some("browser-nonce".to_owned()),
    )
    .expect("sign_login must succeed");

    let jwt = js_field(&result, "jwt")
        .as_string()
        .expect("jwt is a string");
    assert_eq!(jwt.split('.').count(), 3, "JWT must have 3 parts");
    assert!(verify_jwt(&jwt, TEST_AUD, Some("browser-nonce".to_owned())));
    assert!(
        js_field(&result, "challengeSig").is_string(),
        "a nonce login carries a challenge signature"
    );
}

#[wasm_bindgen_test]
fn sign_login_without_nonce_omits_challenge_sig_in_browser() {
    let keys = derive_wallet_keys(&deterministic_test_phrase()).expect("valid mnemonic");
    let result =
        sign_login(&keys, TEST_DID, 0, TEST_AUD, 3600, None).expect("sign_login must succeed");
    assert!(
        js_field(&result, "challengeSig").is_undefined(),
        "no nonce ⇒ no challenge signature"
    );
}
