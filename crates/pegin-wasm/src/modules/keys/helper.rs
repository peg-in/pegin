//! Secret-key memory hygiene — overwrite BLS scalars before deallocation.

use chia_bls::SecretKey;
use zeroize::Zeroize;

/// Overwrites the 32-byte BLS scalar in `sk` with zeros before the value is freed.
///
/// Shrinks the window where signing keys stay readable in WASM heap memory after login
/// or `WalletKeys.free()` — relevant if another script or a memory dump inspects the heap.
/// Does not erase every transient copy made while signing; pairing with `loginWithSeed`
/// keeps keys off the JS boundary entirely.
pub fn zeroize_secret_key(sk: &mut SecretKey) {
    let mut scratch = sk.to_bytes();
    scratch.zeroize();
    if let Ok(zero) = SecretKey::from_bytes(&[0u8; 32]) {
        *sk = zero;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::keys::service::derive_wallet_keys_inner;

    use crate::test_util::deterministic_test_phrase;

    #[test]
    fn zeroize_secret_key_clears_scalar_bytes() {
        let keys = derive_wallet_keys_inner(&deterministic_test_phrase()).unwrap();
        let mut sk = keys.wallet_sk.clone();
        assert!(!sk.to_bytes().iter().all(|&b| b == 0));
        zeroize_secret_key(&mut sk);
        assert!(sk.to_bytes().iter().all(|&b| b == 0));
    }
}
