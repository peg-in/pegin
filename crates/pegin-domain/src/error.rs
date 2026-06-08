use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum DomainError {
    #[error("invalid DID: {0}")]
    InvalidDid(String),
    #[error("invalid username: {0}")]
    InvalidUsername(String),
}

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
