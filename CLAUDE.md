# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What Is PEGIN

PEGIN is decentralized SSO: **MVP Step 1** = mini wallet + DID + **wallet-issued JWT** (passkey, `@pegin/sdk`) — not DIG-first. **Step 2** = vault + recovery (seed phrase + passkey, multi-device). DIG application layer post-MVP. Detail: `docs/03-use-cases/mvp-strategy.md`.

**Principles anchor:** `docs/01-vision/pegin-manifest.md` (evergreen customer + product + how we work + how we build).

**Team:** `docs/09-how-we-work/` · **Architecture:** `docs/10-architecture/` · **Programmers:** `docs/08-developer/` (`environment/`, `engineering/`, `integration/`).

**Documentation:** [docs/README.md](Wiki%20Links.md) · [docs/08-developer/developer-documentation.md](docs/08-developer/developer-documentation.md) (code only) · [docs/README.md](Wiki%20Links.md). **Specs:** [Spec 1](docs/04-technical/specs/tech-stack.md) · [Spec 2](docs/04-technical/specs/enterprise-identity-spec.md). **Ecosystem:** [xch-dev](https://github.com/xch-dev), [docs.xch.dev](https://docs.xch.dev), [Yakuhito/slot-machine](https://github.com/Yakuhito/slot-machine), [XCHandles docs](https://docs.xchandles.com). **AI/RAG:** [llms.txt](llms.txt), [docs/ai/CONTEXT.md](docs/ai/CONTEXT.md) — regenerate with `python3 scripts/generate-ai-knowledge-base.py`.

## Planned Workspace Structure

**Step 1 bootstrap:** [docs/08-developer/engineering/step1-implementation-bootstrap.md](docs/08-developer/engineering/step1-implementation-bootstrap.md). **Full layout:** [docs/10-architecture/application-architecture.md](docs/10-architecture/application-architecture.md).

```
crates/pegin-domain/       crates/pegin-identity/
crates/pegin-wallet/       # Step 1 IdP: DID, passkey, JWT
crates/pegin-infrastructure/
apps/mini/                  # Tauri account app
packages/sdk/               # @pegin/sdk
```

Later: `contracts/` (Step 2 vault), `pegin-protocols/`, `pegin-authorization/` (PePP), `pegin-api/`.

**Persistence:** domain on DIG + Chia anchors; optional **SQL** (sqlx + Data Mapper) for operator/OIDC cache only.

## Tech Stack Decisions

| Layer | Choice | Reason |
|---|---|---|
| Identity engine | Rust (Axum) | Matches Chia/DIG ecosystem; performance |
| Smart contracts | Rue | Type-safe Chialisp alternative; Rust-like syntax |
| Web SDK & dashboard | TypeScript + React + Shadcn | Browser compat; developer adoption |
| Desktop/mobile | Tauri v2 | Same pattern as Sage Wallet |

## Key Dependencies (once code is written)

**Rust (`Cargo.toml`):**
- `chia-wallet-sdk = "0.23"` — DID (`DidInfo`), NFT (`NftInfo`), `MerkleTree`, `SpendContext`
- `passkey = "0.3"` — WebAuthn/FIDO2 (1Password crate)
- `axum = "0.8"` — HTTP server
- `openidconnect = "4.0"`, `samael = "0.0.17"`, `jsonwebtoken = "9.3"` — protocol layer
- `chia-sdk-test = "0.26"` — blockchain simulator for tests
- `rue-cli` — compile `.rue` smart contracts (`cargo install rue-cli`)

**TypeScript:**
- `@simplewebauthn/browser` + `@simplewebauthn/server` — WebAuthn client/server
- `next = "^15"`, `react = "^19"`, `tailwindcss = "^4"`

## Architecture Overview

```
User Device ──passkey──▶ PEGIN Service (Rust/Axum)
                            │  WebAuthn RP
                            │  DID manager (chia-wallet-sdk)
                            │  Credential manager (NFT)
                            │  Session manager (JWT + DIG store)
                            │  Audit logger (DIG append-only)
                            ├──anchor/verify──▶ Chia Blockchain (DID Coin / Rue contracts)
                            └──store/read──────▶ DIG Network (user data, logs)
```

**Chia layer:** `chia-wallet-sdk` for DID + NFT operations; `clvm_rs` for contract execution; Rue contracts compiled to CLVM bytecode.

**DIG layer:** `dig-l2-storage` (RocksDB) for user data; `chia-block-listener` for on-chain events; `DataLayer-Driver` for Chia Datalayer; `slot-machine` for `alice.pegin` naming.

## Smart Contracts (Rue)

Contracts live in `pegin-contracts/src/`:
- `pegin_did.rue` — DID registration + update
- `pegin_credential.rue` — credential NFT issuance with royalty puzzle
- Recovery via chia-wallet-sdk `VaultInfo` + Rue (Rigidity upstream); see `docs/10-architecture/recovery-vault-and-guardians.md`
- `pegin_revoke.rue` — credential revocation

Compile with `rue-cli`; test with `chia-sdk-test` simulator. VSCode extension: `rue-vscode`.

## Protocol Rollout Order

POC → v1.0 → Enterprise: WebAuthn → OIDC → SAML 2.0 → OAuth 2.0 → SCIM → LDAP → WS-Federation → Kerberos.

Enterprise Azure AD migration path: PEGIN federates as SP (SAML) → SCIM sync from Azure → parallel operation → optional full migration.

## Key External Repositories

- [xch-dev/chia-wallet-sdk](https://github.com/xch-dev/chia-wallet-sdk) — critical Rust SDK for all blockchain ops
- [xch-dev/rue](https://github.com/xch-dev/rue) — Rue language compiler
- [xch-dev/sage](https://github.com/xch-dev/sage) — reference architecture (Rust + Tauri + React)
- [DIG-Network/dig-l2-storage](https://github.com/DIG-Network/dig-l2-storage) — L2 block storage
- [Chia-Network/chia_rs](https://github.com/Chia-Network/chia_rs) — consensus, signing, protocol types
