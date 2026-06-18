//! Errors surfaced by the password-sealed seed vault.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum SeedVaultError {
    #[error("invalid seed phrase")]
    InvalidMnemonic,
    #[error("wrong password")]
    WrongPassword,
    #[error("wrong pin")]
    WrongPin,
    #[error("pin must be 4–8 digits")]
    InvalidPin,
    #[error("pin backup already configured")]
    PinAlreadySet,
    #[error("device unlock already configured")]
    DeviceAlreadySet,
    #[error("device unlock is not available on this platform")]
    DeviceUnavailable,
    #[error("no sealed seed on this device")]
    Empty,
    #[error("storage error: {0}")]
    Storage(String),
    #[error("seal/unseal failed")]
    Crypto,
}
