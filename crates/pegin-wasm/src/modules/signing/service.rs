use chia_bls::{aggregate, sign, Signature};
use chia_protocol::SpendBundle;
use chia_traits::streamable::Streamable;

use crate::modules::keys::entities::WalletKeys;

/// Signs a UTF-8 challenge string with the DID key using BLS `AugSchemeMPL`.
/// Returns the 96-byte signature as lowercase hex.
pub fn sign_challenge_inner(keys: &WalletKeys, challenge: &str) -> String {
    let sig: Signature = sign(&keys.did_sk, challenge.as_bytes());
    hex::encode(sig.to_bytes())
}

/// Signs a `SpendBundle` serialised as Chia streamable bytes.
///
/// Phase 1 simplified signing: the wallet key signs the raw `bundle_bytes`.
/// This is sufficient for testing the signing pipeline end-to-end but is NOT
/// the full `AGG_SIG_ME` protocol (which requires CLVM execution of each puzzle
/// to extract signing messages). Full protocol support is deferred to a follow-up
/// ticket that adds `chia-wallet-sdk` sub-crates once their WASM compat is confirmed.
pub fn sign_spend_bundle_inner(keys: &WalletKeys, bundle_bytes: &[u8]) -> Result<Vec<u8>, String> {
    let mut bundle = SpendBundle::from_bytes(bundle_bytes)
        .map_err(|e| format!("failed to parse SpendBundle: {e}"))?;

    let msg_sig: Signature = sign(&keys.wallet_sk, bundle_bytes);

    // Aggregate with the existing signature if it is non-default.
    let default_bytes = Signature::default().to_bytes();
    let aggregated = if bundle.aggregated_signature.to_bytes() == default_bytes {
        msg_sig
    } else {
        aggregate([&bundle.aggregated_signature, &msg_sig])
    };

    bundle.aggregated_signature = aggregated;
    bundle
        .to_bytes()
        .map_err(|e| format!("failed to serialise SpendBundle: {e}"))
}

#[cfg(test)]
mod tests {
    use chia_bls::verify;

    use super::*;
    use crate::modules::keys::service::derive_wallet_keys_inner;

    const TEST_MNEMONIC: &str = "abandon abandon abandon abandon abandon abandon \
         abandon abandon abandon abandon abandon about";

    #[test]
    fn sign_challenge_produces_valid_bls_signature() {
        let keys = derive_wallet_keys_inner(TEST_MNEMONIC).unwrap();
        let challenge = "test-server-nonce-abc123";

        let sig_hex = sign_challenge_inner(&keys, challenge);

        let sig_bytes: [u8; 96] = hex::decode(&sig_hex).unwrap().try_into().expect("96 bytes");
        let sig = Signature::from_bytes(&sig_bytes).unwrap();
        let pk = keys.did_sk.public_key();

        assert!(
            verify(&sig, &pk, challenge.as_bytes()),
            "BLS signature must verify against the DID public key"
        );
    }

    #[test]
    fn sign_challenge_is_deterministic() {
        let keys = derive_wallet_keys_inner(TEST_MNEMONIC).unwrap();
        let a = sign_challenge_inner(&keys, "challenge");
        let b = sign_challenge_inner(&keys, "challenge");
        assert_eq!(a, b);
    }

    #[test]
    fn sign_challenge_differs_for_different_inputs() {
        let keys = derive_wallet_keys_inner(TEST_MNEMONIC).unwrap();
        let a = sign_challenge_inner(&keys, "challenge-a");
        let b = sign_challenge_inner(&keys, "challenge-b");
        assert_ne!(a, b);
    }
}
