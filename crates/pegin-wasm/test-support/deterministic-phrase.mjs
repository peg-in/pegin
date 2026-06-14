/**
 * Deterministic BIP39 phrases from fixed entropy — no English words in source.
 * Public keys must match `crates/pegin-wasm/test_vectors.rs`.
 */
import { entropyToMnemonic } from "@scure/bip39";
import { wordlist } from "@scure/bip39/wordlists/english";

export const DETERMINISTIC_WALLET_PK =
  "8cf9590585eafc287497b15ec7e32e0e5bdcb27a0eef799b66ba685898514976a1933f2e03efeeca81064983a1372d68";

export const DETERMINISTIC_DID_PK =
  "8afb55d134fa5ae1601c5ed734efdbdb160c31e9ce68e1cd6ff4ba85614ac9cfff0c6fda4698f4ecf86d45e2e434edcb";

/** @returns {string} 24-word phrase from 32 zero bytes. */
export function deterministicTestPhrase() {
  return entropyToMnemonic(new Uint8Array(32), wordlist);
}

/** @returns {string} 24-word phrase from 32 0xff bytes (wrong-key tests). */
export function alternateTestPhrase() {
  return entropyToMnemonic(new Uint8Array(32).fill(0xff), wordlist);
}
