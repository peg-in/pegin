//! Where the sealed blob lives. The `SealStore` trait keeps storage pluggable; `FileSealStore`
//! is the local-device backend the PEGIN Signer app uses today.

use std::fs;
use std::path::PathBuf;

use super::entities::VaultFile;
use super::error::SeedVaultError;
use super::helper;

/// A backend that persists the full vault file. The plaintext seed and password never reach a
/// store — only encrypted blobs do.
pub trait SealStore {
    fn load(&self) -> Result<Option<VaultFile>, SeedVaultError>;
    fn save(&self, vault: &VaultFile) -> Result<(), SeedVaultError>;
    fn clear(&self) -> Result<(), SeedVaultError>;
}

/// Local single-file backend (JSON in the app data dir). The default.
pub struct FileSealStore {
    path: PathBuf,
}

impl FileSealStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }
}

impl SealStore for FileSealStore {
    fn load(&self) -> Result<Option<VaultFile>, SeedVaultError> {
        match fs::read(&self.path) {
            Ok(bytes) => helper::parse_vault_file(&bytes).map(Some),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(SeedVaultError::Storage(e.to_string())),
        }
    }

    fn save(&self, vault: &VaultFile) -> Result<(), SeedVaultError> {
        let json = serde_json::to_vec_pretty(vault).map_err(|_| SeedVaultError::Crypto)?;
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).map_err(|e| SeedVaultError::Storage(e.to_string()))?;
        }
        fs::write(&self.path, json).map_err(|e| SeedVaultError::Storage(e.to_string()))
    }

    fn clear(&self) -> Result<(), SeedVaultError> {
        match fs::remove_file(&self.path) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(SeedVaultError::Storage(e.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::seed_vault::helper;

    fn temp_path() -> PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_nanos());
        std::env::temp_dir().join(format!("pegin-seal-{nanos}.json"))
    }

    #[test]
    fn file_store_round_trips_and_clears() {
        let path = temp_path();
        let store = FileSealStore::new(&path);
        assert!(store.load().unwrap().is_none());

        let primary = helper::seal(
            "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
            "pw",
        )
        .unwrap();
        let vault = VaultFile::from_primary(primary, Some("test".into()));
        store.save(&vault).unwrap();
        assert!(store.load().unwrap().is_some());

        store.clear().unwrap();
        assert!(store.load().unwrap().is_none());
    }

    #[test]
    fn migrates_legacy_single_blob_format() {
        let path = temp_path();
        let store = FileSealStore::new(&path);
        let primary = helper::seal(
            "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
            "pw",
        )
        .unwrap();
        let legacy = serde_json::to_vec_pretty(&primary).unwrap();
        fs::write(&path, legacy).unwrap();

        let loaded = store.load().unwrap().expect("vault");
        assert_eq!(loaded.primary.kdf_salt_hex, primary.kdf_salt_hex);
        assert!(loaded.pin.is_none());
    }
}
