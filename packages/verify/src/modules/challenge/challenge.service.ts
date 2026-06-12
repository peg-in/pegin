import blsInit from 'bls-signatures'

type BlsModule = Awaited<ReturnType<typeof blsInit>>

let blsReady: Promise<BlsModule> | undefined

async function ensureBls(): Promise<BlsModule> {
  blsReady ??= blsInit()
  return blsReady
}

/** Verifies a BLS challenge signature produced by `signChallenge` in pegin-wasm. */
export async function verifyChallengeSignature(
  didPkHex: string,
  challenge: string,
  signatureHex: string,
): Promise<void> {
  const bls = await ensureBls()
  const pkBytes = hexToBytes(didPkHex)
  if (pkBytes.length !== 48) {
    throw new Error('DID public key must be 48 bytes')
  }
  const sigBytes = hexToBytes(signatureHex)
  if (sigBytes.length !== 96) {
    throw new Error('challenge signature must be 96 bytes')
  }
  const pk = bls.G1Element.from_bytes(pkBytes)
  const sig = bls.G2Element.from_bytes(sigBytes)
  const ok = bls.AugSchemeMPL.verify(pk, new TextEncoder().encode(challenge), sig)
  if (!ok) {
    throw new Error('challenge signature does not verify')
  }
}

function hexToBytes(hex: string): Uint8Array {
  const normalized = hex.startsWith('0x') ? hex.slice(2) : hex
  if (normalized.length % 2 !== 0) {
    throw new Error('invalid hex')
  }
  const out = new Uint8Array(normalized.length / 2)
  for (let i = 0; i < out.length; i++) {
    out[i] = Number.parseInt(normalized.slice(i * 2, i * 2 + 2), 16)
  }
  return out
}
