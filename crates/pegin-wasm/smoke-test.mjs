#!/usr/bin/env node
/**
 * Smoke test for the pegin-wasm Node build. Never prints the mnemonic.
 *   Part 1 — deterministic zero-entropy phrase, offline, always runs.
 * Build first: wasm-pack build --target nodejs --out-dir pkg-node
 */

import assert from "node:assert/strict";
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

const fakeDid = "did:chia:1gt7hae94wd0c33v07k4kkwgjy9jjtcnzhwvl5yxuvmj28mqsnsjqvgw9uu";
const token = mintJwt(keys, fakeDid, TEST_AUD, 600);

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
  const withNonce = mintJwt(keys, fakeDid, TEST_AUD, 600, nonce);
  assert.equal(verifyJwt(withNonce, TEST_AUD, nonce), true);
});

// lookupDid / loginWithSeed scan public coinset on a cache miss, so they need the
// network and a browser-like env — exercised by the wasm-pack headless tests, not here.

if (failures > 0) {
  err(`\n${failures} test(s) failed`);
  process.exit(1);
}
out("\nall tests passed");
