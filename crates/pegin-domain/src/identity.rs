use std::fmt;

use serde::{Deserialize, Serialize};

use crate::error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Username(String);

impl Username {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn validate(value: &str) -> Result<(), DomainError> {
        let len = value.len();
        if !(3..=32).contains(&len) {
            return Err(DomainError::InvalidUsername(format!(
                "must be 3–32 chars, got {len}"
            )));
        }
        if !value
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
        {
            return Err(DomainError::InvalidUsername(
                "must contain only lowercase letters, digits, and '_'".to_owned(),
            ));
        }
        Ok(())
    }
}

impl TryFrom<&str> for Username {
    type Error = DomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::validate(value)?;
        Ok(Self(value.to_owned()))
    }
}

impl TryFrom<String> for Username {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::validate(&value)?;
        Ok(Self(value))
    }
}

impl fmt::Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// JWT `sub` claim — the user's DID string, stable across sessions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Sub(String);

impl Sub {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Sub {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
