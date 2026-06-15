#![cfg(target_arch = "wasm32")]

mod common;

use common::{deterministic_test_phrase, DETERMINISTIC_DID_PK, DETERMINISTIC_WALLET_PK};
use js_sys::Reflect;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

use pegin_wasm::{
    derive_keys, derive_wallet_keys, hello, login_with_seed, lookup_did, mint_jwt, remember_did,
    sign_challenge, verify_jwt,
};

fn js_field(obj: &JsValue, key: &str) -> JsValue {
    Reflect::get(obj, &JsValue::from_str(key)).expect("readable object field")
}

/// Tests share one browser origin, so the local profile cache persists between
/// them — reset it before any assertion that depends on a fresh profile.
fn clear_local_profile() {
    if let Some(Ok(Some(storage))) = web_sys::window().map(|w| w.local_storage()) {
        let _ = storage.clear();
    }
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

// ── Local DID lookup (cached path — no live chain scan in CI) ──────────────────
// The first-login coinset scan is covered by native mock tests in `did_lookup::scan`;
// here we exercise the JS binding + cache so the headless suite stays deterministic.

#[wasm_bindgen_test]
async fn lookup_did_returns_cached_identity_in_browser() {
    clear_local_profile();
    let keys = derive_wallet_keys(&deterministic_test_phrase()).expect("valid mnemonic");
    remember_did(
        keys.did_pk_hex(),
        "did:chia:1cachedbrowser".to_owned(),
        4757,
    );

    let identity = lookup_did(&keys, 0).await.expect("cache hit, no scan");
    assert_eq!(
        js_field(&identity, "ownerIndex").as_f64(),
        Some(4757.0),
        "lookupDid returns the cached owning index"
    );
    let owner_pk = js_field(&identity, "ownerPk")
        .as_string()
        .expect("ownerPk is a string");
    assert_eq!(owner_pk.len(), 96, "owner pk is a 48-byte BLS key in hex");
    assert_eq!(
        js_field(&identity, "did").as_string().as_deref(),
        Some("did:chia:1cachedbrowser"),
        "lookupDid returns the locally cached DID with no chain read"
    );
}

// ── Local login (cached identity — signs with the owning index) ────────────────

#[wasm_bindgen_test]
async fn login_with_seed_mints_verifiable_jwt_in_browser() {
    clear_local_profile();
    let keys = derive_wallet_keys(&deterministic_test_phrase()).expect("valid mnemonic");
    remember_did(keys.did_pk_hex(), "did:chia:1cachedbrowser".to_owned(), 0);

    let result = login_with_seed(
        &deterministic_test_phrase(),
        0,
        3600,
        TEST_AUD,
        Some("browser-nonce".to_owned()),
    )
    .await
    .expect("local login must succeed");

    let jwt = js_field(&result, "jwt")
        .as_string()
        .expect("jwt is a string");
    assert_eq!(jwt.split('.').count(), 3, "JWT must have 3 parts");
    assert!(verify_jwt(&jwt, TEST_AUD, Some("browser-nonce".to_owned())));

    assert_eq!(
        js_field(&result, "did").as_string().as_deref(),
        Some("did:chia:1cachedbrowser"),
        "login carries the cached canonical DID"
    );
    let wallet_fp = js_field(&result, "walletFp")
        .as_string()
        .expect("walletFp is a string");
    assert_eq!(wallet_fp.len(), 96, "wallet fingerprint is the DID pk hex");
    assert!(
        js_field(&result, "challengeSig").is_string(),
        "a nonce login carries a challenge signature"
    );
}
