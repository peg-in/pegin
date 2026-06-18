//! Device unlock key — OS keychain with a local file fallback when secret storage is unavailable.

use std::fs;
use std::io::Write;
use std::path::PathBuf;

use tauri::{AppHandle, Manager};

const KEYCHAIN_SERVICE: &str = "app.pegin.signer";
const KEYCHAIN_USER: &str = "device-unlock";
const DEVICE_KEY_FILE: &str = "device-unlock.key";

fn keyring_entry() -> Result<keyring::Entry, String> {
    keyring::Entry::new(KEYCHAIN_SERVICE, KEYCHAIN_USER).map_err(|e| e.to_string())
}

fn device_key_path(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join(DEVICE_KEY_FILE))
}

fn write_key_file(path: &PathBuf, device_key: &str) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .map_err(|e| e.to_string())?;
    file.write_all(device_key.as_bytes())
        .map_err(|e| e.to_string())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o600)).map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn read_key_file(path: &PathBuf) -> Result<String, String> {
    fs::read_to_string(path)
        .map(|s| s.trim().to_string())
        .map_err(|e| e.to_string())
}

/// Persists the device key in the OS keychain and verifies a read-back; falls back to a local file.
pub fn store_device_key(app: &AppHandle, device_key: &str) -> Result<(), String> {
    if let Ok(entry) = keyring_entry() {
        if entry.set_password(device_key).is_ok()
            && entry.get_password().ok().as_deref() == Some(device_key)
        {
            let _ = write_key_file(&device_key_path(app)?, device_key);
            return Ok(());
        }
    }
    write_key_file(&device_key_path(app)?, device_key)?;
    read_device_key(app).and_then(|read| {
        if read == device_key {
            Ok(())
        } else {
            Err("device key could not be verified after save".into())
        }
    })
}

/// Reads the device key from the keychain or the local fallback file.
pub fn read_device_key(app: &AppHandle) -> Result<String, String> {
    if let Ok(entry) = keyring_entry() {
        if let Ok(key) = entry.get_password() {
            if !key.is_empty() {
                return Ok(key);
            }
        }
    }
    read_key_file(&device_key_path(app)?)
}

/// Whether unlock credentials exist for this install.
pub fn has_device_key(app: &AppHandle) -> bool {
    read_device_key(app).is_ok()
}
