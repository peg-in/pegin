//! Password-sealed local seed vault — the PEGIN Signer app's enrollment store.

pub mod entities;
pub mod error;
mod helper;
pub mod repository;
pub mod service;

pub use entities::{PasskeyBackup, PasskeyVaultBlob, SealedSeed, VaultFile, VaultStatus};
pub use error::SeedVaultError;
pub use helper::{generate_mnemonic, seal_with_prf, validate_pin};
pub use repository::{FileSealStore, SealStore};
pub use service::SeedVault;
