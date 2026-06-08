use std::fmt;

use serde::{Deserialize, Serialize};

use crate::error::DomainError;

const DID_PREFIX: &str = "did:chia:";
const LAUNCHER_HEX_LEN: usize = 64;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Did(String);

impl Did {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn validate(value: &str) -> Result<(), DomainError> {
        let Some(id) = value.strip_prefix(DID_PREFIX) else {
            return Err(DomainError::InvalidDid(format!(
                "must start with '{DID_PREFIX}', got '{value}'"
            )));
        };
        if id.len() != LAUNCHER_HEX_LEN {
            return Err(DomainError::InvalidDid(format!(
                "launcher_id must be {LAUNCHER_HEX_LEN} hex chars, got {}",
                id.len()
            )));
        }
        if !id.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f')) {
            return Err(DomainError::InvalidDid(
                "launcher_id must be lowercase hex (0-9, a-f)".to_owned(),
            ));
        }
        Ok(())
    }
}

impl TryFrom<&str> for Did {
    type Error = DomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::validate(value)?;
        Ok(Self(value.to_owned()))
    }
}

impl TryFrom<String> for Did {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::validate(&value)?;
        Ok(Self(value))
    }
}

impl fmt::Display for Did {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID: &str =
        "did:chia:deadbeef000000000000000000000000000000000000000000000000cafebabe";

    #[test]
    fn accepts_valid_did() {
        let did = Did::try_from(VALID).unwrap();
        assert_eq!(did.as_str(), VALID);
        assert_eq!(did.to_string(), VALID);
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
    fn try_from_string_works() {
        let did = Did::try_from(VALID.to_owned()).unwrap();
        assert_eq!(did.as_str(), VALID);
    }
}
