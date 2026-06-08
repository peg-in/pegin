use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionId(Uuid);

impl SessionId {
    pub fn new_v4() -> Self {
        Self(Uuid::new_v4())
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
        assert_eq!(SessionId::new_v4().to_string().len(), 36);
    }
}
