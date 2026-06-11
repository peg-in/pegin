#!/usr/bin/env node
/**
 * Manual test CLI: mnemonic → BLS keys → on-chain DID check → self-signed JWT.
 * Runs the browser WASM in Node ≥ 20.12. Build first:
 *   wasm-pack build --target nodejs --out-dir pkg-node
 * Usage: node demo-cli.mjs <did:chia:1...|launcher_hex> [--aud url] [--ttl sec] [--base-url url]
 *        node demo-cli.mjs --skip-chain
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
  getDid,
  mintJwt,
  verifyJwt,
} from "./pkg-node/pegin_wasm.js";

// Built-in .env loader — never overrides existing env vars.
const envFile = join(dirname(fileURLToPath(import.meta.url)), ".env");
if (existsSync(envFile)) process.loadEnvFile(envFile);

/**
 * @param {string[]} argv - CLI args without the node/script prefix
 * @returns {{didInput?: string, aud: string, ttl: number, baseUrl: string|null, skipChain?: boolean}}
 */
function parseArgs(argv) {
  const args = { aud: "https://app.example.com", ttl: 3600, baseUrl: null };
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
    if (a === "--aud") args.aud = argv[++i];
    else if (a === "--ttl") args.ttl = Number(argv[++i]);
    else if (a === "--base-url") args.baseUrl = argv[++i];
    else if (a === "--skip-chain") args.skipChain = true;
    else if (!a.startsWith("--")) args.didInput = a;
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
if (!args.didInput && !args.skipChain) {
  logger.error(
    "usage: node demo-cli.mjs <did:chia:1... | launcher_id_hex> [--aud <url>] [--ttl <seconds>] [--base-url <url>]\n" +
    "      node demo-cli.mjs --skip-chain  (mint without on-chain DID check)"
  );
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

let did;
if (args.skipChain) {
  did = "did:chia:0000000000000000000000000000000000000000000000000000000000000000";
  logger.info(`did: ${did} (on-chain check SKIPPED)`);
} else {
  try {
    did = await getDid(args.didInput, args.baseUrl);
  } catch (e) {
    logger.error(`DID verification failed: ${e.message}`);
    process.exit(1);
  }
  logger.info(`did: ${did} (verified on-chain)`);
}

const token = mintJwt(keys, did, args.aud, args.ttl);
const payload = JSON.parse(Buffer.from(token.split(".")[1], "base64url").toString());
logger.info(`claims: ${JSON.stringify(payload)}`);

if (!verifyJwt(token, keys.didPkHex)) {
  logger.error("self-verification of the minted JWT failed");
  process.exit(1);
}

// The JWT is the program output — stdout stays clean for piping.
process.stdout.write(`${token}\n`);
