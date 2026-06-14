//! Ground-truth DID ownership test against a real testnet11 DID (launcher 42fd7ee4…),
//! owned by the wallet in crates/pegin-wasm/.env. Fixture captured via `get_puzzle_and_solution`.
// Module-level allow: the fixture helpers below use `unwrap` outside `#[test]`
// fns, which `allow-unwrap-in-tests` does not cover in an integration test crate.
#![allow(clippy::unwrap_used)]

use pegin_verify::did::{expected_owner_p2, parse_owner_p2_hash, CoinParts};
use serde_json::Value;

fn hex32(s: &str) -> [u8; 32] {
    let h = s.strip_prefix("0x").unwrap_or(s);
    hex::decode(h).unwrap().try_into().unwrap()
}

fn coin_parts(v: &Value) -> CoinParts {
    CoinParts {
        parent_coin_info: hex32(v["parent_coin_info"].as_str().unwrap()),
        puzzle_hash: hex32(v["puzzle_hash"].as_str().unwrap()),
        amount: v["amount"].as_u64().unwrap(),
    }
}

fn fixture() -> Value {
    let raw = include_str!("fixtures/testnet11_did.json");
    serde_json::from_str(raw).unwrap()
}

// The real testnet11 DID (launcher 42fd7ee4…) is owned by the .env wallet's first
// address key — the UNHARDENED observer key m/12381/8444/2/0, synthetic — NOT the
// DID-path key m/.../3/0 that PEGIN historically put in cnf.did_pk. This is the core
// finding behind feat-17: the JWT must bind to the wallet address key that owns the DID.
const OWNER_OBSERVER0_PK: &str = "950cd5418da71b6601ea2e073ca8266cef3297cd3b8e270a145c2290c8a6c2d90ac3c23e33b4ebba43bac0a75b0c2bd5";
const ON_CHAIN_OWNER_P2: &str = "18a9ca2c38d1a71890976e43ff6288abd36ab6d6829e0d362c2d638d42c3ab18";
// The DID-path key the wallet derives — must NOT be accepted as owner.
const DID_PATH_PK: &str = "9583293bcc1c78d1385fdd5478e3a906bbe5b0aba03239cd5eb13632e830f94797c75d5416c06e0b092369bb2e4a7f4f";
// Another valid wallet key (observer-1) — valid point, but not the owning index.
const OBSERVER1_PK: &str = "96f19547e4d0e46aa9c2f7e7517fdeb259a8b079d8a8627fb5ef4067b25d31c04903e647306541677d86792f14cfd138";

fn pk48(s: &str) -> [u8; 48] {
    hex::decode(s).unwrap().try_into().unwrap()
}

#[test]
fn on_chain_owner_matches_observer_address_key_not_did_path() {
    let f = fixture();
    let parent = coin_parts(&f["parent_coin"]);
    let child = coin_parts(&f["child_coin"]);
    let puzzle = hex::decode(
        f["puzzle_reveal"]
            .as_str()
            .unwrap()
            .trim_start_matches("0x"),
    )
    .unwrap();
    let solution = hex::decode(f["solution"].as_str().unwrap().trim_start_matches("0x")).unwrap();

    // 1. The DID singleton parses and yields the real on-chain owner p2 hash.
    let owner_p2 = parse_owner_p2_hash(&parent, &puzzle, &solution, &child).expect("parse owner");
    assert_eq!(hex::encode(owner_p2), ON_CHAIN_OWNER_P2);

    // 2. The wallet's observer-0 address key binds to it (StandardArgs(synthetic(pk))).
    assert_eq!(
        expected_owner_p2(&pk48(OWNER_OBSERVER0_PK)).unwrap(),
        owner_p2,
        "observer-0 address key must own the DID"
    );

    // 3. The DID-path key does NOT — proving cnf.did_pk historically pointed at the wrong key.
    assert_ne!(
        expected_owner_p2(&pk48(DID_PATH_PK)).unwrap(),
        owner_p2,
        "DID-path key must not be accepted as owner"
    );

    // 4. Another valid wallet key (observer-1) is rejected — only the owning index binds.
    assert_ne!(expected_owner_p2(&pk48(OBSERVER1_PK)).unwrap(), owner_p2);
}
