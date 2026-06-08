use pegin_domain::did::Did;
use pegin_identity::authenticator::PasskeyAuthenticator;
use pegin_testing::{did_factory, harness::TestHarness, mock_passkey::MockPasskeyVerifier};
use serde::Deserialize;

// ---------------------------------------------------------------------------
// Persona fixtures
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct Persona {
    username: String,
    credential_id: String,
}

#[derive(Debug, Deserialize)]
struct Personas {
    personas: Vec<Persona>,
}

const PERSONAS_JSON: &str = include_str!("fixtures/personas.json");

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// Scenario 1 — harness boots with a pre-funded wallet.
#[test]
fn harness_boots_with_funded_wallet() {
    let harness = TestHarness::new();
    assert_eq!(harness.wallet.coin.amount, 1_000_000);
}

/// Scenario 2 — DID factory produces a valid did:chia: identifier.
#[test]
fn did_factory_creates_did_on_simulator() {
    let mut harness = TestHarness::new();
    let info = did_factory::create_did(&mut harness, "alice");

    assert!(info.did.as_str().starts_with("did:chia:"));
    // launcher_id is 32 bytes → 64 hex chars
    assert_eq!(info.did.as_str().len(), "did:chia:".len() + 64);
}

/// Scenario 3 — mock verifier returns a fixed challenge and accepts any response.
#[test]
fn mock_passkey_verifier_returns_fixed_assertion() {
    let mock = MockPasskeyVerifier::new().with_credential("alice");
    let did = Did::try_from("did:chia:0000000000000000000000000000000000000000000000000000000000000000")
        .unwrap();

    let challenge = mock.begin_authentication(&did).unwrap();
    assert!(!challenge.is_empty());
    mock.finish_authentication(&did, &challenge).unwrap();

    let reg_challenge = mock.begin_registration(&did).unwrap();
    assert!(!reg_challenge.is_empty());
    mock.finish_registration(&did, &reg_challenge).unwrap();
}

/// Scenario 4 — two harnesses run without sharing state.
#[test]
fn harnesses_are_independent() {
    let mut h1 = TestHarness::new();
    let h2 = TestHarness::new();

    h1.sim.create_block();

    assert_eq!(h1.sim.height(), 1, "h1 should have advanced one block");
    assert_eq!(h2.sim.height(), 0, "h2 must not be affected by h1");
}

/// Persona fixtures are well-formed and contain alice and bob.
#[test]
fn personas_fixture_is_valid() {
    let personas: Personas = serde_json::from_str(PERSONAS_JSON).unwrap();
    let names: Vec<&str> = personas
        .personas
        .iter()
        .map(|p| p.username.as_str())
        .collect();
    assert!(names.contains(&"alice"));
    assert!(names.contains(&"bob"));
    for p in &personas.personas {
        assert!(!p.credential_id.is_empty());
    }
}
