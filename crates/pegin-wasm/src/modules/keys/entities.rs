use chia_bls::SecretKey;
use wasm_bindgen::prelude::*;

/// BLS keys derived from a BIP39 mnemonic.
/// Secret keys never leave Rust memory — only public key hex crosses into JS.
#[wasm_bindgen]
pub struct WalletKeys {
    /// Wallet path m/12381/8444/2/0 — used for spend bundle signing.
    pub(crate) wallet_sk: SecretKey,
    /// DID path m/12381/8444/3/0 — used for challenge signing and DID proofs.
    pub(crate) did_sk: SecretKey,
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
}
