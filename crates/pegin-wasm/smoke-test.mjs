#!/usr/bin/env node
/**
 * Smoke test for the pegin-wasm Node build. Never prints the mnemonic.
 *   Part 1 — deterministic zero-entropy phrase, offline, always runs.
 *   Part 2 — configured wallet from env/.env (CI: secrets/vars), runs when set, else skips.
 * No real wallet strings are hardcoded; supply them via PEGIN_MNEMONIC / PEGIN_DID.
 * Build first: wasm-pack build --target nodejs --out-dir pkg-node
 */

import assert from "node:assert/strict";
import { existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import {
  deterministicTestPhrase,
  DETERMINISTIC_DID_PK,
} from "./test-support/deterministic-phrase.mjs";
import {
  deriveWalletKeys,
  mintJwt,
  signChallenge,
  verifyJwt,
} from "./pkg-node/pegin_wasm.js";

// Local convenience: load a sibling .env. CI injects PEGIN_* directly (no file present),
// so env vars set by the pipeline take effect without it.
const envFile = join(dirname(fileURLToPath(import.meta.url)), ".env");
if (existsSync(envFile)) process.loadEnvFile(envFile);

const TEST_AUD = "https://smoke.example";

// Reporter output is the program's product — stdout/stderr, not logging.
const out = (line) => process.stdout.write(`${line}\n`);
const err = (line) => process.stderr.write(`${line}\n`);

let failures = 0;
/**
 * Runs one named check; logs ok/FAIL and counts failures instead of throwing.
 * @param {string} name
 * @param {() => void | Promise<void>} fn - assertion body
 */
async function test(name, fn) {
  try {
    await fn();
    out(`  ok    ${name}`);
  } catch (e) {
    failures++;
    err(`  FAIL  ${name}: ${e.message}`);
  }
}

out("Part 1 — deterministic test phrase (offline, zero-entropy)");

const keys = deriveWalletKeys(deterministicTestPhrase());

await test("derives the known DID public key", () => {
  assert.equal(keys.didPkHex, DETERMINISTIC_DID_PK);
});

await test("challenge signature is 96 bytes and deterministic", () => {
  const sig = signChallenge(keys, "smoke-test-nonce");
  assert.equal(sig.length, 192);
  assert.equal(sig, signChallenge(keys, "smoke-test-nonce"));
});

// Synthetic, non-wallet DID (launcher = 0x11 × 32) — used only as a JWT subject.
const syntheticDid = "did:chia:1zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zygsx2z7xu";
const token = mintJwt(keys, syntheticDid, TEST_AUD, 600);

await test("minted ES256K JWT verifies for the bound audience", () => {
  assert.equal(verifyJwt(token, TEST_AUD, undefined), true);
});

await test("tampered JWT payload fails verification", () => {
  const [header, , sig] = token.split(".");
  const evil = Buffer.from('{"iss":"attacker","exp":9999999999}').toString("base64url");
  assert.equal(verifyJwt(`${header}.${evil}.${sig}`, TEST_AUD, undefined), false);
});

await test("JWT fails verification for a different audience", () => {
  assert.equal(verifyJwt(token, "https://other.example", undefined), false);
});

await test("JWT with embedded nonce verifies when nonce matches", () => {
  const nonce = "smoke-nonce-1";
  const withNonce = mintJwt(keys, syntheticDid, TEST_AUD, 600, nonce);
  assert.equal(verifyJwt(withNonce, TEST_AUD, nonce), true);
});

// lookupDid / loginWithSeed scan public coinset on a cache miss, so they need the
// network and a browser-like env — exercised by the wasm-pack headless tests, not here.

out("\nPart 2 — configured wallet (PEGIN_MNEMONIC / PEGIN_DID from env or .env)");

const { PEGIN_MNEMONIC, PEGIN_DID } = process.env;
if (!PEGIN_MNEMONIC) {
  out("  SKIPPED — set PEGIN_MNEMONIC (CI secret / .env) to enable");
} else {
  const myKeys = deriveWalletKeys(PEGIN_MNEMONIC);
  await test("derives a DID public key for the configured wallet", () => {
    assert.match(myKeys.didPkHex, /^[0-9a-f]{96}$/);
  });
  if (PEGIN_DID) {
    await test("mints + verifies a JWT bound to PEGIN_DID", () => {
      const t = mintJwt(myKeys, PEGIN_DID, TEST_AUD, 600);
      assert.equal(verifyJwt(t, TEST_AUD, undefined), true);
    });
  } else {
    out("  note — also set PEGIN_DID to test JWT binding to your DID");
  }
  myKeys.free();
}

if (failures > 0) {
  err(`\n${failures} test(s) failed`);
  process.exit(1);
}
out("\nall tests passed");
