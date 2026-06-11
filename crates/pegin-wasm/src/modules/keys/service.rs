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

const INVALID_MNEMONIC: &str = "invalid mnemonic";

/// Derives the wallet and DID secret keys from a BIP39 mnemonic. Deterministic.
pub fn derive_wallet_keys_inner(mnemonic: &str) -> Result<WalletKeys, String> {
    let mn = Mnemonic::parse_in(Language::English, mnemonic)
        .map_err(|_| INVALID_MNEMONIC.to_string())?;

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

    // Standard BIP39 test vector (12 words); paths m/12381/8444/{2,3}/0.
    const TEST_MNEMONIC: &str = "abandon abandon abandon abandon abandon abandon \
         abandon abandon abandon abandon abandon about";
    const KNOWN_WALLET_PK: &str =
        "a0b24361941efdd2859c984c9a77e8898ee841ac0c6d8d3b5515d54f5fff59cc37a18808b1fa8df4afa6b447b84cbbbb";
    const KNOWN_DID_PK: &str =
        "aee8545e9cef0270cb54069a9ed81a6b1e657f68ee7e102853a0887df68f28455b79a14f86823a2b81eacc29af9d9b85";

    #[test]
    fn accepts_valid_24_word_mnemonic() {
        use bip39::{Language, Mnemonic};

        let phrase = Mnemonic::from_entropy_in(Language::English, &[0u8; 32])
            .expect("valid 24-word phrase")
            .to_string();
        assert_eq!(phrase.split_whitespace().count(), 24);

        let keys = derive_wallet_keys_inner(&phrase).unwrap();
        assert_eq!(keys.did_pk_hex().len(), 96);
        assert_ne!(keys.did_pk_hex(), KNOWN_DID_PK);
    }

    #[test]
    fn known_public_key_vector() {
        let keys = derive_wallet_keys_inner(TEST_MNEMONIC).unwrap();
        assert_eq!(keys.wallet_pk_hex(), KNOWN_WALLET_PK);
        assert_eq!(keys.did_pk_hex(), KNOWN_DID_PK);
    }

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
        assert_eq!(keys.wallet_pk_hex().len(), 96);
        assert_eq!(keys.did_pk_hex().len(), 96);
    }

    #[test]
    fn rejects_invalid_mnemonic() {
        let result = derive_wallet_keys_inner("not a valid mnemonic phrase");
        assert!(result.is_err());
        assert_eq!(result.err().as_deref(), Some("invalid mnemonic"));
    }
}
