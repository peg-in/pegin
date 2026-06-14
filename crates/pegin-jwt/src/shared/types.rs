//! Shared JWT claim shapes.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfirmationClaim {
    /// BLS DID public key (96-char lowercase hex) bound to this session.
    #[serde(rename = "did_pk")]
    pub did_pk_hex: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeginJwtPayload {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub iat: u64,
    pub exp: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    pub cnf: ConfirmationClaim,
}
