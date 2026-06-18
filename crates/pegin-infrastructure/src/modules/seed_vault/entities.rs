//! On-disk / at-rest shape of a password-sealed seed and its backups.

use serde::{Deserialize, Serialize};

/// A sealed seed at rest: Argon2id KDF salt + ChaCha20-Poly1305 nonce + ciphertext (all hex).
/// The password never leaves the user; only this blob is persisted (locally or in 1Password).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SealedSeed {
    /// Blob format version, for future migrations.
    pub v: u8,
    pub kdf_salt_hex: String,
    pub nonce_hex: String,
    pub ct_hex: String,
}

/// AES-GCM ciphertext sealed under a passkey PRF secret (matches `@pegin/sdk` passkey-vault).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PasskeyVaultBlob {
    pub iv: String,
    pub ct: String,
}

/// One passkey backup: credential id + PRF-encrypted seed blob for relay / web login sync.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PasskeyBackup {
    pub credential_id: String,
    pub label: String,
    pub blob: PasskeyVaultBlob,
}

/// Full vault file: primary password seal, optional PIN / device unlock, passkey backups.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultFile {
    pub v: u8,
    pub name: Option<String>,
    pub primary: SealedSeed,
    pub pin: Option<SealedSeed>,
    /// Quick unlock via OS keychain (Touch ID / Windows Hello where supported).
    #[serde(default)]
    pub device: Option<SealedSeed>,
    #[serde(default)]
    pub passkeys: Vec<PasskeyBackup>,
}

impl VaultFile {
    pub const VERSION: u8 = 2;

    pub fn from_primary(primary: SealedSeed, name: Option<String>) -> Self {
        Self {
            v: Self::VERSION,
            name,
            primary,
            pin: None,
            device: None,
            passkeys: Vec::new(),
        }
    }
}

/// Whether a seed has been sealed on this device, and where.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VaultStatus {
    pub sealed: bool,
    pub name: Option<String>,
    pub has_pin: bool,
    pub has_device_unlock: bool,
    pub passkey_count: usize,
}
