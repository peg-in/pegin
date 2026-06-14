//! BIP39 mnemonic → Chia BLS key derivation (wallet and DID paths).

use bip39::{Language, Mnemonic};
use chia_bls::{DerivableKey, SecretKey};
use zeroize::Zeroize;

use super::entities::WalletKeys;
use super::helper::zeroize_secret_key;

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

    // Empty passphrase (Chia default); the full 64-byte seed feeds the master key —
    // truncating changes every derived key and breaks Sage/reference-wallet compatibility.
    let mut seed = mn.to_seed("");
    let mut master_sk = SecretKey::from_seed(&seed);
    seed.zeroize();

    let wallet_sk = derive_path(
        &master_sk,
        &[PATH_PURPOSE, PATH_COIN_TYPE, PATH_WALLET, PATH_INDEX],
    );
    let did_sk = derive_path(
        &master_sk,
        &[PATH_PURPOSE, PATH_COIN_TYPE, PATH_DID, PATH_INDEX],
    );
    let observer_intermediate_sk = master_sk
        .derive_unhardened(PATH_PURPOSE)
        .derive_unhardened(PATH_COIN_TYPE)
        .derive_unhardened(PATH_WALLET);
    zeroize_secret_key(&mut master_sk);

    Ok(WalletKeys {
        wallet_sk,
        did_sk,
        observer_intermediate_sk,
    })
}

fn derive_path(master: &SecretKey, path: &[u32]) -> SecretKey {
    let mut sk = master.clone();
    for &index in path {
        let next = sk.derive_hardened(index);
        zeroize_secret_key(&mut sk);
        sk = next;
    }
    sk
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::{
        deterministic_test_phrase, DETERMINISTIC_DID_PK, DETERMINISTIC_WALLET_PK,
    };

    #[test]
    #[ignore = "debug helper: prints public-key vectors for deterministic_test_phrase()"]
    fn debug_print_deterministic_vectors() {
        let keys = derive_wallet_keys_inner(&deterministic_test_phrase()).unwrap();
        println!("wallet_pk={}", keys.wallet_pk_hex());
        println!("did_pk={}", keys.did_pk_hex());
    }

    #[test]
    fn accepts_valid_24_word_mnemonic() {
        let phrase = deterministic_test_phrase();
        assert_eq!(phrase.split_whitespace().count(), 24);

        let keys = derive_wallet_keys_inner(&phrase).unwrap();
        assert_eq!(keys.did_pk_hex().len(), 96);
    }

    #[test]
    fn known_public_key_vector() {
        let phrase = deterministic_test_phrase();
        let keys = derive_wallet_keys_inner(&phrase).unwrap();
        assert_eq!(keys.wallet_pk_hex(), DETERMINISTIC_WALLET_PK);
        assert_eq!(keys.did_pk_hex(), DETERMINISTIC_DID_PK);
    }

    #[test]
    fn key_derivation_is_deterministic() {
        let phrase = deterministic_test_phrase();
        let a = derive_wallet_keys_inner(&phrase).unwrap();
        let b = derive_wallet_keys_inner(&phrase).unwrap();
        assert_eq!(a.wallet_pk_hex(), b.wallet_pk_hex());
        assert_eq!(a.did_pk_hex(), b.did_pk_hex());
    }

    #[test]
    fn wallet_and_did_keys_are_different() {
        let keys = derive_wallet_keys_inner(&deterministic_test_phrase()).unwrap();
        assert_ne!(keys.wallet_pk_hex(), keys.did_pk_hex());
    }

    #[test]
    fn public_keys_are_48_bytes_hex() {
        let keys = derive_wallet_keys_inner(&deterministic_test_phrase()).unwrap();
        assert_eq!(keys.wallet_pk_hex().len(), 96);
        assert_eq!(keys.did_pk_hex().len(), 96);
    }

    #[test]
    fn observer_intermediate_pk_matches_address_root_from_seed() {
        use bip39::{Language, Mnemonic};

        let phrase = deterministic_test_phrase();
        let keys = derive_wallet_keys_inner(&phrase).unwrap();

        let mn = Mnemonic::parse_in(Language::English, &phrase).unwrap();
        let seed = mn.to_seed("");
        let master = SecretKey::from_seed(&seed);
        let expected = master
            .public_key()
            .derive_unhardened(PATH_PURPOSE)
            .derive_unhardened(PATH_COIN_TYPE)
            .derive_unhardened(PATH_WALLET);

        assert_eq!(
            keys.observer_intermediate_pk().to_bytes(),
            expected.to_bytes(),
            "observer hints must come from the live master key, not a zeroed copy"
        );
    }

    #[test]
    fn rejects_invalid_mnemonic() {
        let result = derive_wallet_keys_inner("not a valid mnemonic phrase");
        assert!(result.is_err());
        assert_eq!(result.err().as_deref(), Some("invalid mnemonic"));
    }
}
