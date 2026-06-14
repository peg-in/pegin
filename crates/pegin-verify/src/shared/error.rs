//! Verification errors for relying parties.

use pegin_jwt::JwtError;

#[derive(Debug, thiserror::Error)]
pub enum VerifyError {
    #[error("JWT verification failed: {0}")]
    Jwt(#[from] JwtError),
    #[error("challenge signature invalid")]
    ChallengeInvalid,
    #[error("challenge signature required")]
    ChallengeRequired,
    #[error("JWT key does not own the on-chain DID")]
    DidNotOwned,
    #[error("coinset request failed: {0}")]
    Coinset(String),
    #[error("invalid DID: {0}")]
    InvalidDid(String),
}
