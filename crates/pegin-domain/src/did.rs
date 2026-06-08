use std::fmt;

use serde::{Deserialize, Serialize};

use crate::error::DomainError;

pub const DID_PREFIX: &str = "did:chia:";
pub const LAUNCHER_HEX_LEN: usize = 64;

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
