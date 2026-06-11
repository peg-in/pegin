#!/usr/bin/env node
/**
 * Smoke test for the pegin-wasm Node build. Never prints the mnemonic.
 *   Part 1 — public throwaway mnemonic, offline, always runs.
 *   Part 2 — real on-chain flow with a personal testnet wallet; runs when
 *            PEGIN_MNEMONIC and PEGIN_DID are set (env vars > .env), else skips.
 * Build first: wasm-pack build --target nodejs --out-dir pkg-node
 */

import assert from "node:assert/strict";
import { existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import {
  deriveWalletKeys,
  getDid,
  mintJwt,
  signChallenge,
  verifyJwt,
} from "./pkg-node/pegin_wasm.js";

const envFile = join(dirname(fileURLToPath(import.meta.url)), ".env");
if (existsSync(envFile)) process.loadEnvFile(envFile);

// Reporter output is the program's product — stdout/stderr, not logging.
const out = (line) => process.stdout.write(`${line}\n`);
const err = (line) => process.stderr.write(`${line}\n`);

// Public BIP39 test vector and the DID public key it must always derive.
const THROWAWAY_MNEMONIC =
  "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
const THROWAWAY_DID_PK =
  "aee8545e9cef0270cb54069a9ed81a6b1e657f68ee7e102853a0887df68f28455b79a14f86823a2b81eacc29af9d9b85";

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

out("Part 1 — throwaway mnemonic (offline, deterministic)");

const keys = deriveWalletKeys(THROWAWAY_MNEMONIC);

await test("derives the known DID public key", () => {
  assert.equal(keys.didPkHex, THROWAWAY_DID_PK);
});

await test("challenge signature is 96 bytes and deterministic", () => {
  const sig = signChallenge(keys, "smoke-test-nonce");
  assert.equal(sig.length, 192);
  assert.equal(sig, signChallenge(keys, "smoke-test-nonce"));
});

const fakeDid = "did:chia:1gt7hae94wd0c33v07k4kkwgjy9jjtcnzhwvl5yxuvmj28mqsnsjqvgw9uu";
const token = mintJwt(keys, fakeDid, "https://smoke.test", 600);

await test("minted JWT verifies against its own DID key", () => {
  assert.equal(verifyJwt(token, keys.didPkHex), true);
});

await test("tampered JWT payload fails verification", () => {
  const [header, , sig] = token.split(".");
  const evil = Buffer.from('{"iss":"attacker","exp":9999999999}').toString("base64url");
  assert.equal(verifyJwt(`${header}.${evil}.${sig}`, keys.didPkHex), false);
});

await test("JWT fails verification with a different key", () => {
  const other = deriveWalletKeys(
    "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong"
  );
  assert.equal(verifyJwt(token, other.didPkHex), false);
});

out("\nPart 2 — personal testnet wallet (on-chain via coinset.org)");

const { PEGIN_MNEMONIC, PEGIN_DID } = process.env;
if (!PEGIN_MNEMONIC || !PEGIN_DID) {
  out("  SKIPPED — set PEGIN_MNEMONIC and PEGIN_DID (env or .env) to enable");
} else {
  const myKeys = deriveWalletKeys(PEGIN_MNEMONIC);
  let myDid = await getDid(myKeys, null);

  if (myDid == null) {
    out(
      "  SKIPPED — no on-chain DID for PEGIN_MNEMONIC on testnet11; " +
        "ensure the mnemonic created PEGIN_DID (or unset Part 2 env vars)"
    );
  } else {
    await test("personal DID is verified on-chain", () => {
      assert.match(String(myDid), /^did:chia:1/);
      if (PEGIN_DID) assert.equal(myDid, PEGIN_DID);
    });

    await test("personal JWT mints and verifies", () => {
      const myToken = mintJwt(myKeys, myDid, "https://smoke.test", 600);
      assert.equal(verifyJwt(myToken, myKeys.didPkHex), true);
    });
  }
}

if (failures > 0) {
  err(`\n${failures} test(s) failed`);
  process.exit(1);
}
out("\nall tests passed");
