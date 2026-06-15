use chia_bls::{DerivableKey, SecretKey};
use wasm_bindgen::prelude::*;

use super::helper::zeroize_secret_key;

/// BLS keys derived from a BIP39 mnemonic.
/// Secret keys never leave Rust memory — only public key bytes cross into JS.
/// On `free()` or drop, wallet/DID scalars are zeroed before deallocation.
// The `_sk` suffix marks secret keys (vs derived public keys); it is meaningful, not noise.
#[allow(clippy::struct_field_names)]
#[wasm_bindgen]
pub struct WalletKeys {
    /// Wallet path m/12381/8444/2/0 — used for spend bundle signing.
    pub(crate) wallet_sk: SecretKey,
    /// DID path m/12381/8444/3/0 — used for challenge signing and DID proofs.
    pub(crate) did_sk: SecretKey,
    /// Unhardened (observer) secret key at m/12381/8444/2 — derives the wallet's
    /// address keys, which own the DID singleton (its on-chain owner puzzle is the
    /// standard p2 of the synthetic address key). Children of this sign DID proofs.
    pub(crate) observer_intermediate_sk: SecretKey,
}

impl Drop for WalletKeys {
    fn drop(&mut self) {
        zeroize_secret_key(&mut self.wallet_sk);
        zeroize_secret_key(&mut self.did_sk);
        zeroize_secret_key(&mut self.observer_intermediate_sk);
    }
}

impl WalletKeys {
    /// Unhardened observer public key at m/12381/8444/2 — the address-key root the
    /// DID-owner scan derives address hints from (`did_lookup::scan`, browser/tests only).
    #[cfg(any(target_arch = "wasm32", test))]
    pub(crate) fn observer_intermediate_pk(&self) -> chia_bls::PublicKey {
        self.observer_intermediate_sk.public_key()
    }

    /// Raw secret key of the wallet address at observer `index` — the DID owner key.
    /// Login signs the challenge and JWT with this key so `cnf.did_pk` binds to the owner.
    pub(crate) fn owner_secret_at(&self, index: u32) -> SecretKey {
        self.observer_intermediate_sk.derive_unhardened(index)
    }
}

#[wasm_bindgen]
impl WalletKeys {
    /// BLS public key for the wallet path (48 bytes, lowercase hex).
    #[wasm_bindgen(getter, js_name = "walletPkHex")]
    pub fn wallet_pk_hex(&self) -> String {
        hex::encode(self.wallet_sk.public_key().to_bytes())
    }

    /// BLS public key for the DID path (48 bytes, lowercase hex).
    #[wasm_bindgen(getter, js_name = "didPkHex")]
    pub fn did_pk_hex(&self) -> String {
        hex::encode(self.did_sk.public_key().to_bytes())
    }

    /// DID public key as raw bytes (48-byte BLS G1 point).
    #[wasm_bindgen(getter, js_name = "didPublicKey")]
    pub fn did_public_key(&self) -> Vec<u8> {
        self.did_sk.public_key().to_bytes().to_vec()
    }
}
