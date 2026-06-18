//! BIP39 → BLS key derivation and spend-bundle signing for the native Signer wallet.

use bip39::{Language, Mnemonic};
use chia_bls::{aggregate, sign, SecretKey, Signature};
use chia_protocol::SpendBundle;
use chia_traits::streamable::Streamable;

const PATH_PURPOSE: u32 = 12381;
const PATH_COIN_TYPE: u32 = 8444;
const PATH_WALLET: u32 = 2;

/// Signs a streamable `SpendBundle` with the wallet key derived from `mnemonic`.
///
/// Phase 1 uses raw-byte signing (same as pegin-wasm); full `AGG_SIG_ME` lands with
/// chia-wallet-sdk integration in pegin-wallet.
pub fn sign_spend_bundle(mnemonic: &str, bundle_bytes: &[u8]) -> Result<Vec<u8>, String> {
    let wallet_sk = derive_wallet_key(mnemonic)?;
    let mut bundle =
        SpendBundle::from_bytes(bundle_bytes).map_err(|e| format!("invalid spend bundle: {e}"))?;

    let msg_sig: Signature = sign(&wallet_sk, bundle_bytes);
    let default_bytes = Signature::default().to_bytes();
    bundle.aggregated_signature = if bundle.aggregated_signature.to_bytes() == default_bytes {
        msg_sig
    } else {
        aggregate([&bundle.aggregated_signature, &msg_sig])
    };

    bundle
        .to_bytes()
        .map_err(|e| format!("failed to serialise signed bundle: {e}"))
}

/// Signs a UTF-8 message with the wallet key (Sage `signMessage` shape).
pub fn sign_message(mnemonic: &str, message: &str) -> Result<String, String> {
    let wallet_sk = derive_wallet_key(mnemonic)?;
    Ok(hex::encode(sign(&wallet_sk, message.as_bytes()).to_bytes()))
}

fn derive_wallet_key(mnemonic: &str) -> Result<SecretKey, String> {
    let mn = Mnemonic::parse_in(Language::English, mnemonic.trim())
        .map_err(|_| "invalid mnemonic".to_string())?;
    let seed = mn.to_seed("");
    let master = SecretKey::from_seed(&seed);
    Ok(derive_path(
        &master,
        &[PATH_PURPOSE, PATH_COIN_TYPE, PATH_WALLET, 0],
    ))
}

fn derive_path(master: &SecretKey, path: &[u32]) -> SecretKey {
    let mut sk = master.clone();
    for &index in path {
        sk = sk.derive_hardened(index);
    }
    sk
}

#[cfg(test)]
mod tests {
    use super::*;
    use chia_bls::verify;

    const TEST_MNEMONIC: &str =
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

    #[test]
    fn sign_message_verifies() {
        let sig_hex = sign_message(TEST_MNEMONIC, "hello pegin").unwrap();
        let sig_bytes: [u8; 96] = hex::decode(sig_hex).unwrap().try_into().unwrap();
        let sig = Signature::from_bytes(&sig_bytes).unwrap();
        let pk = derive_wallet_key(TEST_MNEMONIC).unwrap().public_key();
        assert!(verify(&sig, &pk, b"hello pegin"));
    }
}
