use pegin_domain::{
    error::DomainError,
    identity::{Sub, Username},
};

#[test]
fn accepts_valid_username() {
    let u = Username::try_from("alice").unwrap();
    assert_eq!(u.as_str(), "alice");
    assert_eq!(u.to_string(), "alice");
}

#[test]
fn accepts_username_with_digits_and_underscore() {
    Username::try_from("bob_42").unwrap();
}

#[test]
fn rejects_too_short() {
    let err = Username::try_from("ab").unwrap_err();
    assert!(matches!(err, DomainError::InvalidUsername(_)));
}

#[test]
fn rejects_too_long() {
    let err = Username::try_from("a".repeat(33).as_str()).unwrap_err();
    assert!(matches!(err, DomainError::InvalidUsername(_)));
}

#[test]
fn rejects_uppercase() {
    let err = Username::try_from("Alice").unwrap_err();
    assert!(matches!(err, DomainError::InvalidUsername(_)));
}

#[test]
fn rejects_special_chars() {
    let err = Username::try_from("invalid user!").unwrap_err();
    assert!(matches!(err, DomainError::InvalidUsername(_)));
}

#[test]
fn try_from_string_works() {
    Username::try_from("carol".to_owned()).unwrap();
}

#[test]
fn sub_display() {
    let sub = Sub::new("did:chia:deadbeef");
    assert_eq!(sub.to_string(), "did:chia:deadbeef");
}
