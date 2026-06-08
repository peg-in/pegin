use pegin_domain::{
    did::{Did, DID_PREFIX, LAUNCHER_HEX_LEN},
    error::DomainError,
};

const VALID: &str = "did:chia:deadbeef000000000000000000000000000000000000000000000000cafebabe";

#[test]
fn accepts_valid_did() {
    let did = Did::try_from(VALID).unwrap();
    assert_eq!(did.as_str(), VALID);
    assert_eq!(did.to_string(), VALID);
}

#[test]
fn try_from_string_works() {
    let did = Did::try_from(VALID.to_owned()).unwrap();
    assert_eq!(did.as_str(), VALID);
}

#[test]
fn rejects_wrong_prefix() {
    let err = Did::try_from("did:example:deadbeef").unwrap_err();
    assert!(matches!(err, DomainError::InvalidDid(_)));
}

#[test]
fn rejects_short_launcher_id() {
    let err = Did::try_from("did:chia:deadbeef").unwrap_err();
    assert!(matches!(err, DomainError::InvalidDid(_)));
}

#[test]
fn rejects_uppercase_hex() {
    let err =
        Did::try_from("did:chia:DEADBEEF000000000000000000000000000000000000000000000000CAFEBABE")
            .unwrap_err();
    assert!(matches!(err, DomainError::InvalidDid(_)));
}

#[test]
fn rejects_non_hex_chars() {
    let err =
        Did::try_from("did:chia:gggggggg000000000000000000000000000000000000000000000000cafebabe")
            .unwrap_err();
    assert!(matches!(err, DomainError::InvalidDid(_)));
}

#[test]
fn constants_match_valid_did() {
    assert!(VALID.starts_with(DID_PREFIX));
    assert_eq!(VALID.len(), DID_PREFIX.len() + LAUNCHER_HEX_LEN);
}
