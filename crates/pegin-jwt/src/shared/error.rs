//! JWT mint/verify errors returned to callers.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JwtError {
    InvalidToken(String),
    InvalidSignature,
    Expired,
    AudienceMismatch { expected: String, actual: String },
    IssuerSubjectMismatch,
    NonceMismatch,
    MissingNonce,
    MissingConfirmationKey,
    UnsupportedAlgorithm(String),
}

impl fmt::Display for JwtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidToken(msg) => write!(f, "{msg}"),
            Self::InvalidSignature => f.write_str("invalid JWT signature"),
            Self::Expired => f.write_str("JWT expired"),
            Self::AudienceMismatch { expected, actual } => {
                write!(f, "audience mismatch: expected {expected}, got {actual}")
            }
            Self::IssuerSubjectMismatch => f.write_str("iss and sub must match"),
            Self::NonceMismatch => f.write_str("nonce mismatch"),
            Self::MissingNonce => f.write_str("nonce required but missing from JWT"),
            Self::MissingConfirmationKey => f.write_str("cnf.did_pk missing from JWT"),
            Self::UnsupportedAlgorithm(alg) => write!(f, "unsupported JWT algorithm: {alg}"),
        }
    }
}

impl std::error::Error for JwtError {}
