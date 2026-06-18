//! Pure seal/unseal crypto: Argon2id password KDF + ChaCha20-Poly1305 AEAD. No I/O.

use argon2::Argon2;
use chacha20poly1305::aead::Aead;
use chacha20poly1305::{ChaCha20Poly1305, KeyInit, Nonce};
use rand::rngs::OsRng;
use rand::RngCore;

use super::entities::SealedSeed;
use super::entities::{PasskeyVaultBlob, VaultFile};
use super::error::SeedVaultError;

const SEAL_VERSION: u8 = 1;
const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;
const KEY_LEN: usize = 32;

/// Validates `mnemonic` (BIP-39 checksum) and seals it under `password`.
pub fn seal(mnemonic: &str, password: &str) -> Result<SealedSeed, SeedVaultError> {
    let phrase = mnemonic.trim();
    bip39::Mnemonic::parse(phrase).map_err(|_| SeedVaultError::InvalidMnemonic)?;

    let mut salt = [0u8; SALT_LEN];
    OsRng.fill_bytes(&mut salt);
    let key = derive_key(password, &salt)?;

    let mut nonce = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut nonce);
    let cipher = ChaCha20Poly1305::new_from_slice(&key).map_err(|_| SeedVaultError::Crypto)?;
    let ct = cipher
        .encrypt(&Nonce::from(nonce), phrase.as_bytes())
        .map_err(|_| SeedVaultError::Crypto)?;

    Ok(SealedSeed {
        v: SEAL_VERSION,
        kdf_salt_hex: hex::encode(salt),
        nonce_hex: hex::encode(nonce),
        ct_hex: hex::encode(ct),
    })
}

/// Recovers the mnemonic from `sealed` using `password`. A wrong password fails AEAD
/// authentication and returns `WrongPassword`.
pub fn unseal(sealed: &SealedSeed, password: &str) -> Result<String, SeedVaultError> {
    let salt = hex::decode(&sealed.kdf_salt_hex).map_err(|_| SeedVaultError::Crypto)?;
    let nonce = hex::decode(&sealed.nonce_hex).map_err(|_| SeedVaultError::Crypto)?;
    let ct = hex::decode(&sealed.ct_hex).map_err(|_| SeedVaultError::Crypto)?;
    let nonce: [u8; NONCE_LEN] = nonce
        .as_slice()
        .try_into()
        .map_err(|_| SeedVaultError::Crypto)?;

    let key = derive_key(password, &salt)?;
    let cipher = ChaCha20Poly1305::new_from_slice(&key).map_err(|_| SeedVaultError::Crypto)?;
    let pt = cipher
        .decrypt(&Nonce::from(nonce), ct.as_ref())
        .map_err(|_| SeedVaultError::WrongPassword)?;
    String::from_utf8(pt).map_err(|_| SeedVaultError::Crypto)
}

/// Generates a fresh BIP-39 mnemonic with `word_count` words (12 or 24), from OS entropy.
pub fn generate_mnemonic(word_count: usize) -> Result<String, SeedVaultError> {
    let entropy_len = match word_count {
        12 => 16,
        24 => 32,
        _ => return Err(SeedVaultError::InvalidMnemonic),
    };
    let mut entropy = vec![0u8; entropy_len];
    OsRng.fill_bytes(&mut entropy);
    let mnemonic = bip39::Mnemonic::from_entropy(&entropy).map_err(|_| SeedVaultError::Crypto)?;
    Ok(mnemonic.to_string())
}

/// Validates a quick-unlock PIN (4–8 digits).
pub fn validate_pin(pin: &str) -> Result<(), SeedVaultError> {
    let digits = pin.trim();
    if (4..=8).contains(&digits.len()) && digits.chars().all(|c| c.is_ascii_digit()) {
        Ok(())
    } else {
        Err(SeedVaultError::InvalidPin)
    }
}

/// Encrypts `mnemonic` under a WebAuthn PRF secret (AES-256-GCM, SDK-compatible).
pub fn seal_with_prf(
    prf_secret: &[u8],
    mnemonic: &str,
) -> Result<PasskeyVaultBlob, SeedVaultError> {
    use aes_gcm::aead::{Aead, KeyInit};
    use aes_gcm::{Aes256Gcm, Nonce};

    if prf_secret.len() < 32 {
        return Err(SeedVaultError::Crypto);
    }
    let key = &prf_secret[..32];
    let mut iv = [0u8; 12];
    OsRng.fill_bytes(&mut iv);
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| SeedVaultError::Crypto)?;
    let ct = cipher
        .encrypt(&Nonce::from(iv), mnemonic.as_bytes())
        .map_err(|_| SeedVaultError::Crypto)?;
    Ok(PasskeyVaultBlob {
        iv: base64::Engine::encode(&base64::engine::general_purpose::STANDARD, iv),
        ct: base64::Engine::encode(&base64::engine::general_purpose::STANDARD, ct),
    })
}

/// Parses a legacy single-blob file or a v2 [`VaultFile`].
pub fn parse_vault_file(bytes: &[u8]) -> Result<VaultFile, SeedVaultError> {
    let value: serde_json::Value =
        serde_json::from_slice(bytes).map_err(|_| SeedVaultError::Crypto)?;
    if value.get("primary").is_some() {
        serde_json::from_value(value).map_err(|_| SeedVaultError::Crypto)
    } else {
        let primary: SealedSeed =
            serde_json::from_value(value).map_err(|_| SeedVaultError::Crypto)?;
        Ok(VaultFile::from_primary(primary, None))
    }
}

// Argon2id stretches the user password into a 32-byte AEAD key; the random salt makes each
// seal independent and resists precomputation.
fn derive_key(password: &str, salt: &[u8]) -> Result<[u8; KEY_LEN], SeedVaultError> {
    let mut key = [0u8; KEY_LEN];
    Argon2::default()
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|_| SeedVaultError::Crypto)?;
    Ok(key)
}

#[cfg(test)]
mod tests {
    use super::*;

    // A valid BIP-39 test vector (zero-entropy 12 words).
    const SEED: &str =
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

    #[test]
    fn round_trips_the_seed_with_the_right_password() {
        let sealed = seal(SEED, "correct horse battery staple").unwrap();
        let out = unseal(&sealed, "correct horse battery staple").unwrap();
        assert_eq!(out, SEED);
    }

    #[test]
    fn ciphertext_never_contains_the_plaintext_seed() {
        let sealed = seal(SEED, "pw").unwrap();
        let json = serde_json::to_string(&sealed).unwrap();
        assert!(!json.contains("abandon"));
    }

    #[test]
    fn wrong_password_is_rejected() {
        let sealed = seal(SEED, "right").unwrap();
        let err = unseal(&sealed, "wrong").unwrap_err();
        assert!(matches!(err, SeedVaultError::WrongPassword));
    }

    #[test]
    fn invalid_mnemonic_is_rejected_before_sealing() {
        let err = seal("not a real seed phrase", "pw").unwrap_err();
        assert!(matches!(err, SeedVaultError::InvalidMnemonic));
    }

    #[test]
    fn each_seal_uses_a_fresh_salt_and_nonce() {
        let a = seal(SEED, "pw").unwrap();
        let b = seal(SEED, "pw").unwrap();
        assert_ne!(a.kdf_salt_hex, b.kdf_salt_hex);
        assert_ne!(a.nonce_hex, b.nonce_hex);
    }

    #[test]
    fn generates_valid_12_and_24_word_mnemonics() {
        for (words, count) in [(generate_mnemonic(12), 12), (generate_mnemonic(24), 24)] {
            let phrase = words.unwrap();
            assert_eq!(phrase.split_whitespace().count(), count);
            // A generated phrase must itself be sealable (valid BIP-39 checksum).
            assert!(seal(&phrase, "pw").is_ok());
        }
    }

    #[test]
    fn rejects_unsupported_word_counts() {
        assert!(matches!(
            generate_mnemonic(15),
            Err(SeedVaultError::InvalidMnemonic)
        ));
    }
}
