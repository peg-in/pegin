//! BIP39 mnemonic → Chia BLS key derivation (wallet and DID paths).

use bip39::{Language, Mnemonic};
use chia_bls::SecretKey;

use super::entities::WalletKeys;

/// Chia wallet HD derivation path components (all hardened).
/// m/12381/8444/2/0 → wallet key, m/12381/8444/3/0 → DID key
const PATH_PURPOSE: u32 = 12381;
const PATH_COIN_TYPE: u32 = 8444;
const PATH_WALLET: u32 = 2;
const PATH_DID: u32 = 3;
const PATH_INDEX: u32 = 0;

/// Derives the wallet and DID secret keys from a BIP39 mnemonic. Deterministic.
pub fn derive_wallet_keys_inner(mnemonic: &str) -> Result<WalletKeys, String> {
    let mn = Mnemonic::parse_in(Language::English, mnemonic)
        .map_err(|e| format!("invalid mnemonic: {e}"))?;

    // Empty passphrase (Chia default); first 32 of the 64 seed bytes feed the master key.
    let seed = mn.to_seed("");
    let master_sk = SecretKey::from_seed(&seed[..32]);

    let wallet_sk = derive_path(
        &master_sk,
        &[PATH_PURPOSE, PATH_COIN_TYPE, PATH_WALLET, PATH_INDEX],
    );
    let did_sk = derive_path(
        &master_sk,
        &[PATH_PURPOSE, PATH_COIN_TYPE, PATH_DID, PATH_INDEX],
    );

    Ok(WalletKeys { wallet_sk, did_sk })
}

fn derive_path(master: &SecretKey, path: &[u32]) -> SecretKey {
    path.iter()
        .fold(master.clone(), |sk, &index| sk.derive_hardened(index))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Standard BIP39 test mnemonic — 12 words for brevity (bip39 accepts both 12 and 24).
    const TEST_MNEMONIC: &str = "abandon abandon abandon abandon abandon abandon \
         abandon abandon abandon abandon abandon about";

    #[test]
    fn key_derivation_is_deterministic() {
        let a = derive_wallet_keys_inner(TEST_MNEMONIC).unwrap();
        let b = derive_wallet_keys_inner(TEST_MNEMONIC).unwrap();
        assert_eq!(a.wallet_pk_hex(), b.wallet_pk_hex());
        assert_eq!(a.did_pk_hex(), b.did_pk_hex());
    }

    #[test]
    fn wallet_and_did_keys_are_different() {
        let keys = derive_wallet_keys_inner(TEST_MNEMONIC).unwrap();
        assert_ne!(keys.wallet_pk_hex(), keys.did_pk_hex());
    }

    #[test]
    fn public_keys_are_48_bytes_hex() {
        let keys = derive_wallet_keys_inner(TEST_MNEMONIC).unwrap();
        // BLS G1 point = 48 bytes = 96 hex chars
        assert_eq!(keys.wallet_pk_hex().len(), 96);
        assert_eq!(keys.did_pk_hex().len(), 96);
    }

    #[test]
    fn rejects_invalid_mnemonic() {
        assert!(derive_wallet_keys_inner("not a valid mnemonic phrase").is_err());
    }
}
