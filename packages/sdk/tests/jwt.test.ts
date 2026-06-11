import { existsSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { describe, expect, it } from 'vitest';

const WASM_ENTRY = new URL('../wasm/pegin_wasm.js', import.meta.url);
const wasmBuilt = existsSync(fileURLToPath(WASM_ENTRY));
const { deriveWalletKeys, mintJwt, verifyJwt } = wasmBuilt ? await import(WASM_ENTRY.href) : {};

const TEST_MNEMONIC =
  'abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about';

const TEST_DID =
  'did:chia:deadbeef01020304050607080900aabbccddeeff01020304050607080900aabb';

function decodePayload(token: string): Record<string, unknown> {
  const payloadB64 = token.split('.')[1];
  return JSON.parse(Buffer.from(payloadB64, 'base64url').toString()) as Record<string, unknown>;
}

describe.skipIf(!wasmBuilt)('pegin-wasm JWT mint + verify', () => {
  it('mints a three-part JWT signed with the DID key', () => {
    const keys = deriveWalletKeys(TEST_MNEMONIC);
    const token = mintJwt(keys, TEST_DID, 3600);

    expect(token.split('.')).toHaveLength(3);
    expect(verifyJwt(token, keys.didPublicKey)).toBe(true);
    keys.free();
  });

  it('payload contains iss, sub, iat, and exp only', () => {
    const keys = deriveWalletKeys(TEST_MNEMONIC);
    const before = Math.floor(Date.now() / 1000);
    const token = mintJwt(keys, TEST_DID, 600);
    const after = Math.floor(Date.now() / 1000);
    const payload = decodePayload(token);

    expect(payload.iss).toBe(TEST_DID);
    expect(payload.sub).toBe(TEST_DID);
    expect(payload.aud).toBeUndefined();

    const iat = payload.iat as number;
    const exp = payload.exp as number;
    expect(iat).toBeGreaterThanOrEqual(before);
    expect(iat).toBeLessThanOrEqual(after);
    expect(exp).toBe(iat + 600);
    keys.free();
  });

  it('rejects a tampered payload', () => {
    const keys = deriveWalletKeys(TEST_MNEMONIC);
    const token = mintJwt(keys, TEST_DID, 3600);
    const [header, , sig] = token.split('.');
    const evil = Buffer.from(
      JSON.stringify({ iss: 'attacker', sub: 'attacker', exp: 9_999_999_999 }),
    ).toString('base64url');

    expect(verifyJwt(`${header}.${evil}.${sig}`, keys.didPublicKey)).toBe(false);
    keys.free();
  });

  it('rejects verification with a different public key', () => {
    const keysA = deriveWalletKeys(TEST_MNEMONIC);
    const keysB = deriveWalletKeys(
      'zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong',
    );
    const token = mintJwt(keysA, TEST_DID, 3600);

    expect(verifyJwt(token, keysB.didPublicKey)).toBe(false);
    keysA.free();
    keysB.free();
  });

  it('returns false for malformed tokens', () => {
    const keys = deriveWalletKeys(TEST_MNEMONIC);
    expect(verifyJwt('not-a-jwt', keys.didPublicKey)).toBe(false);
    keys.free();
  });
});
