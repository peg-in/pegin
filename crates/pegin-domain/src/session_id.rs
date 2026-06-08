use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Opaque session identifier — a UUID v4 wrapper.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionId(Uuid);

impl SessionId {
    pub fn new_v4() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_v4_produces_unique_ids() {
        let a = SessionId::new_v4();
        let b = SessionId::new_v4();
        assert_ne!(a, b);
    }

    #[test]
    fn display_is_standard_uuid_format() {
        let id = SessionId::new_v4();
        let s = id.to_string();
        // UUID format: 8-4-4-4-12 hex chars separated by hyphens
        assert_eq!(s.len(), 36);
        assert_eq!(s.chars().filter(|&c| c == '-').count(), 4);
    }
}
