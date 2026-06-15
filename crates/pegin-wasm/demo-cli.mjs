#!/usr/bin/env node
/**
 * Manual test CLI: mnemonic → BLS keys → local DID lookup → self-signed JWT.
 * Identity resolves entirely in WASM (no chain reads); the auth relay resolves the
 * canonical did:chia from the JWT on first login. Runs the browser WASM in Node ≥ 20.12.
 * Build first:
 *   wasm-pack build --target nodejs --out-dir pkg-node
 * Usage: node demo-cli.mjs [--ttl sec] [--aud origin]
 * Prints only the JWT on stdout; diagnostics need LOG_LEVEL=info (wiki: logging-strategy).
 * Mnemonic: PEGIN_MNEMONIC env var > .env next to script > hidden prompt.
 * Test wallets only — never a mnemonic that holds real funds.
 */

import { existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { createInterface } from "node:readline";
import { Writable } from "node:stream";
import { logger } from "./logger.mjs";
import {
  deriveWalletKeys,
  loginWithSeed,
  lookupDid,
  verifyJwt,
} from "./pkg-node/pegin_wasm.js";

// Built-in .env loader — never overrides existing env vars.
const envFile = join(dirname(fileURLToPath(import.meta.url)), ".env");
if (existsSync(envFile)) process.loadEnvFile(envFile);

/**
 * @param {string[]} argv - CLI args without the node/script prefix
 * @returns {{ttl: number, aud: string|null}}
 */
function parseArgs(argv) {
  const args = { ttl: 3600, aud: null };
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
    if (a === "--ttl") args.ttl = Number(argv[++i]);
    else if (a === "--aud") args.aud = argv[++i];
    else if (a.startsWith("--")) {
      logger.error(`unknown flag '${a}'`);
      process.exit(1);
    } else {
      logger.error(`unexpected positional argument '${a}' — DID is resolved from keys, not CLI input`);
      process.exit(1);
    }
  }
  return args;
}

/**
 * Prompts on stderr and reads stdin with echo suppressed.
 * @param {string} question
 * @returns {Promise<string>} trimmed answer
 */
function promptHidden(question) {
  const muted = new Writable({ write: (_c, _e, cb) => cb() });
  const rl = createInterface({ input: process.stdin, output: muted, terminal: true });
  process.stderr.write(question);
  return new Promise((resolve) =>
    rl.question("", (answer) => {
      rl.close();
      process.stderr.write("\n");
      resolve(answer.trim());
    })
  );
}

const args = parseArgs(process.argv.slice(2));
if (!Number.isInteger(args.ttl) || args.ttl <= 0) {
  // NaN would silently coerce to 0 in WASM and mint an already-expired token.
  logger.error(`--ttl must be a positive integer number of seconds, got '${args.ttl}'`);
  process.exit(1);
}

let mnemonic = process.env.PEGIN_MNEMONIC;
if (!mnemonic) {
  if (!process.stdin.isTTY) {
    // CI without the secret set — fail fast instead of hanging on the prompt.
    logger.error("PEGIN_MNEMONIC is not set (env var or .env) and stdin is not a terminal");
    process.exit(1);
  }
  mnemonic = await promptHidden("BIP39 mnemonic (test wallet only, input hidden): ");
}

const keys = deriveWalletKeys(mnemonic);
logger.info(`wallet pk: ${keys.walletPkHex}`);
logger.info(`did pk: ${keys.didPkHex}`);

// Resolves the DID by scanning public coinset hints (first call only; then cached).
const identity = await lookupDid(keys, 0);
keys.free();
logger.info(`owner index ${identity.ownerIndex}, owner pk: ${identity.ownerPk}`);
logger.info(`did: ${identity.did ?? "(no on-chain DID found for this wallet)"}`);

// Mints a JWT signed by the resolved owner key — secrets stay in WASM.
const aud = args.aud ?? "https://localhost";
const session = await loginWithSeed(mnemonic, 0, args.ttl, aud, undefined);
const payload = JSON.parse(Buffer.from(session.jwt.split(".")[1], "base64url").toString());
logger.info(`claims: ${JSON.stringify(payload)}`);

if (!verifyJwt(session.jwt, aud, undefined)) {
  logger.error("self-verification of the minted JWT failed");
  process.exit(1);
}

// The JWT is the program output — stdout stays clean for piping.
process.stdout.write(`${session.jwt}\n`);
