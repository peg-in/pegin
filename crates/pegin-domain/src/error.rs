use std::fmt;

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    Unauthorized(String),
    InvalidInput(String),
    Internal(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "not found: {msg}"),
            AppError::Unauthorized(msg) => write!(f, "unauthorized: {msg}"),
            AppError::InvalidInput(msg) => write!(f, "invalid input: {msg}"),
            AppError::Internal(msg) => write!(f, "internal error: {msg}"),
        }
    }
}

impl std::error::Error for AppError {}
