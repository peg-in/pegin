//! Seed vault use cases: validate + seal a seed under a password, and unseal it back —
//! over any [`SealStore`] backend (local file or 1Password).

use super::entities::{PasskeyBackup, VaultFile, VaultStatus};
use super::error::SeedVaultError;
use super::helper;
use super::repository::SealStore;

pub struct SeedVault<S: SealStore> {
    store: S,
}

impl<S: SealStore> SeedVault<S> {
    pub fn new(store: S) -> Self {
        Self { store }
    }

    /// Whether a sealed seed already exists in the backend.
    pub fn status(&self) -> Result<VaultStatus, SeedVaultError> {
        match self.store.load()? {
            None => Ok(VaultStatus {
                sealed: false,
                name: None,
                has_pin: false,
                has_device_unlock: false,
                passkey_count: 0,
            }),
            Some(vault) => Ok(VaultStatus {
                sealed: true,
                name: vault.name.clone(),
                has_pin: vault.pin.is_some(),
                has_device_unlock: vault.device.is_some(),
                passkey_count: vault.passkeys.len(),
            }),
        }
    }

    /// Validates `mnemonic` (BIP-39), seals it under `password`, and persists the blob.
    /// Replaces any existing vault.
    pub fn seal(
        &self,
        mnemonic: &str,
        password: &str,
        name: Option<&str>,
    ) -> Result<(), SeedVaultError> {
        let primary = helper::seal(mnemonic, password)?;
        let vault = VaultFile::from_primary(primary, name.map(str::to_string));
        self.store.save(&vault)
    }

    /// Recovers the seed phrase from the primary password seal.
    pub fn unseal(&self, password: &str) -> Result<String, SeedVaultError> {
        let vault = self.store.load()?.ok_or(SeedVaultError::Empty)?;
        helper::unseal(&vault.primary, password)
    }

    /// Recovers the seed phrase from the PIN backup seal.
    pub fn unseal_pin(&self, pin: &str) -> Result<String, SeedVaultError> {
        let vault = self.store.load()?.ok_or(SeedVaultError::Empty)?;
        let sealed = vault.pin.as_ref().ok_or(SeedVaultError::Empty)?;
        helper::unseal(sealed, pin).map_err(|e| match e {
            SeedVaultError::WrongPassword => SeedVaultError::WrongPin,
            other => other,
        })
    }

    /// Adds a PIN quick-unlock backup after verifying the primary password.
    pub fn add_pin_backup(&self, password: &str, pin: &str) -> Result<(), SeedVaultError> {
        helper::validate_pin(pin)?;
        let mut vault = self.store.load()?.ok_or(SeedVaultError::Empty)?;
        if vault.pin.is_some() {
            return Err(SeedVaultError::PinAlreadySet);
        }
        let mnemonic = helper::unseal(&vault.primary, password)?;
        vault.pin = Some(helper::seal(&mnemonic, pin)?);
        self.store.save(&vault)
    }

    /// Adds a passkey backup without re-verifying a password (caller must hold an unlocked session).
    pub fn push_passkey_backup(&self, backup: PasskeyBackup) -> Result<(), SeedVaultError> {
        let mut vault = self.store.load()?.ok_or(SeedVaultError::Empty)?;
        if vault
            .passkeys
            .iter()
            .any(|existing| existing.credential_id == backup.credential_id)
        {
            return Ok(());
        }
        vault.passkeys.push(backup);
        self.store.save(&vault)
    }

    /// Recovers the seed using a device key — tries the primary seal first, then the device backup.
    pub fn unseal_device_key(&self, device_key: &str) -> Result<String, SeedVaultError> {
        let vault = self.store.load()?.ok_or(SeedVaultError::Empty)?;
        if let Ok(mnemonic) = helper::unseal(&vault.primary, device_key) {
            return Ok(mnemonic);
        }
        if let Some(sealed) = vault.device.as_ref() {
            return helper::unseal(sealed, device_key);
        }
        Err(SeedVaultError::WrongPassword)
    }

    /// Adds a passkey backup (PRF-encrypted blob) after verifying the primary password.
    pub fn add_passkey_backup(
        &self,
        password: &str,
        backup: PasskeyBackup,
    ) -> Result<(), SeedVaultError> {
        let mut vault = self.store.load()?.ok_or(SeedVaultError::Empty)?;
        helper::unseal(&vault.primary, password)?;
        if vault
            .passkeys
            .iter()
            .any(|existing| existing.credential_id == backup.credential_id)
        {
            return Ok(());
        }
        vault.passkeys.push(backup);
        self.store.save(&vault)
    }

    /// Adds a device-unlock backup sealed under a random key stored in the OS keychain.
    pub fn add_device_backup(
        &self,
        password: &str,
        device_key: &str,
    ) -> Result<(), SeedVaultError> {
        let mut vault = self.store.load()?.ok_or(SeedVaultError::Empty)?;
        if vault.device.is_some() {
            return Err(SeedVaultError::DeviceAlreadySet);
        }
        let mnemonic = helper::unseal(&vault.primary, password)?;
        vault.device = Some(helper::seal(&mnemonic, device_key)?);
        self.store.save(&vault)
    }

    /// Recovers the seed phrase from the device-unlock backup.
    pub fn unseal_device(&self, device_key: &str) -> Result<String, SeedVaultError> {
        let vault = self.store.load()?.ok_or(SeedVaultError::Empty)?;
        let sealed = vault.device.as_ref().ok_or(SeedVaultError::Empty)?;
        helper::unseal(sealed, device_key)
    }

    /// Lists passkey backups (metadata only — blobs are safe to expose to the UI).
    pub fn list_passkeys(&self) -> Result<Vec<PasskeyBackup>, SeedVaultError> {
        Ok(self
            .store
            .load()?
            .map(|vault| vault.passkeys)
            .unwrap_or_default())
    }

    /// Removes the sealed seed from the backend.
    pub fn clear(&self) -> Result<(), SeedVaultError> {
        self.store.clear()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::seed_vault::entities::PasskeyVaultBlob;
    use crate::modules::seed_vault::repository::FileSealStore;

    const SEED: &str =
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

    fn temp_path() -> std::path::PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_nanos());
        std::env::temp_dir().join(format!("pegin-vault-{nanos}.json"))
    }

    #[test]
    fn seals_then_unseals_through_the_store() {
        let vault = SeedVault::new(FileSealStore::new(temp_path()));
        assert!(!vault.status().unwrap().sealed);

        vault.seal(SEED, "hunter2", Some("main")).unwrap();
        let status = vault.status().unwrap();
        assert!(status.sealed);
        assert_eq!(status.name.as_deref(), Some("main"));
        assert_eq!(vault.unseal("hunter2").unwrap(), SEED);

        vault.clear().unwrap();
        assert!(!vault.status().unwrap().sealed);
    }

    #[test]
    fn pin_backup_unlocks_the_same_seed() {
        let vault = SeedVault::new(FileSealStore::new(temp_path()));
        vault.seal(SEED, "hunter2", None).unwrap();
        vault.add_pin_backup("hunter2", "123456").unwrap();
        assert!(vault.status().unwrap().has_pin);
        assert_eq!(vault.unseal_pin("123456").unwrap(), SEED);
    }

    #[test]
    fn device_key_unlocks_primary_seal() {
        let vault = SeedVault::new(FileSealStore::new(temp_path()));
        vault.seal(SEED, "device-key-hex", None).unwrap();
        assert_eq!(vault.unseal_device_key("device-key-hex").unwrap(), SEED);
    }

    #[test]
    fn push_passkey_without_password() {
        let vault = SeedVault::new(FileSealStore::new(temp_path()));
        vault.seal(SEED, "device-key", None).unwrap();
        vault
            .push_passkey_backup(PasskeyBackup {
                credential_id: "abc".into(),
                label: "YubiKey".into(),
                blob: PasskeyVaultBlob {
                    iv: "a".into(),
                    ct: "b".into(),
                },
            })
            .unwrap();
        assert_eq!(vault.list_passkeys().unwrap().len(), 1);
    }

    #[test]
    fn unseal_before_seal_reports_empty() {
        let vault = SeedVault::new(FileSealStore::new(temp_path()));
        assert!(matches!(vault.unseal("pw"), Err(SeedVaultError::Empty)));
    }
}
