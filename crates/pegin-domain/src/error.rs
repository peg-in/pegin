use thiserror::Error;

/// Errors raised when a domain value object fails its invariant.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum DomainError {
    #[error("invalid DID: {0}")]
    InvalidDid(String),
    #[error("invalid username: {0}")]
    InvalidUsername(String),
}

/// Top-level application error threaded through all use-case results.
///
/// `IntoResponse` (axum) is implemented in the API layer, not here, so
/// `pegin-domain` stays framework-agnostic.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum AppError {
    #[error(transparent)]
    Domain(#[from] DomainError),
    #[error("infrastructure error: {0}")]
    Infrastructure(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("not found")]
    NotFound,
}
