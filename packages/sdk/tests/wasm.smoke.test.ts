import { describe, expect, it } from 'vitest';
import { deriveWalletKeys } from '../wasm/pegin_wasm.js';

const TEST_MNEMONIC =
  'abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about';

// Known-good vectors derived from chia-bls HD path m/12381/8444/{2,3}/0.
// Pinned so any upstream key-derivation change is caught immediately.
const KNOWN_WALLET_PK =
  'a0b24361941efdd2859c984c9a77e8898ee841ac0c6d8d3b5515d54f5fff59cc37a18808b1fa8df4afa6b447b84cbbbb';
const KNOWN_DID_PK =
  'aee8545e9cef0270cb54069a9ed81a6b1e657f68ee7e102853a0887df68f28455b79a14f86823a2b81eacc29af9d9b85';

describe('pegin-wasm smoke test', () => {
  it('derives keys matching known BLS vectors', () => {
    const keys = deriveWalletKeys(TEST_MNEMONIC);
    expect(keys.walletPkHex).toBe(KNOWN_WALLET_PK);
    expect(keys.didPkHex).toBe(KNOWN_DID_PK);
    keys.free();
  });

  it('is deterministic for the same mnemonic', () => {
    const a = deriveWalletKeys(TEST_MNEMONIC);
    const b = deriveWalletKeys(TEST_MNEMONIC);
    expect(a.didPkHex).toBe(b.didPkHex);
    expect(a.walletPkHex).toBe(b.walletPkHex);
    a.free();
    b.free();
  });

  it('wallet and DID keys differ', () => {
    const keys = deriveWalletKeys(TEST_MNEMONIC);
    expect(keys.walletPkHex).not.toBe(keys.didPkHex);
    keys.free();
  });

  it('rejects an invalid mnemonic', () => {
    expect(() => deriveWalletKeys('not a valid mnemonic')).toThrow();
  });
});
