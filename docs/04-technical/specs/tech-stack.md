# Spec 1 — PEGIN technical stack (Chia + POC)

> **PEGIN** = **P**enguin **G**ateway **I**de**n**tity — decentralized SSO on Chia + DIG.  
> **Specs hub:** [../README.md](../README.md) · **Developer hub:** [../../08-developer/README.md](../../08-developer/README.md)  
> **Application structure (PoEAA / DDD):** [../../10-architecture/application-architecture.md](../../10-architecture/application-architecture.md)  
> **Spec 2 (Entra / AD / SAML):** [enterprise-identity-spec.md](enterprise-identity-spec.md)  
> **Phases:** [roadmap.md](../../03-use-cases/roadmap.md) · **POC scope:** [mvp-strategy.md](../../03-use-cases/mvp-strategy.md)

---

## Table of Contents

1. [Official documentation index](#0-official-documentation-index)
2. [POC: Passkey login on Chia](#1-poc-focus-passkey-login-on-chia)
3. [xch-dev ecosystem](#2-xch-dev-ecosystem)
4. [Yakuhito: slot-machine & XCHandles](#2b-yakuhito-slot-machine--xchandles)
5. [DIG Network stack](#3-dig-network-stack)
6. [Chia Network (official)](#4-chia-network-repositories)
7. [Enterprise SSO (Spec 2 summary)](#5-enterprise-sso-spec-2)
8. [Optimal tech stack](#6-optimal-tech-stack)
9. [Business stack (separated)](#7-business-stack-separated-from-tech)
10. [Implementation roadmap](#8-implementation-roadmap)

---

## 0. Official documentation index

### xch.dev (wallet SDK, Sage, Rue)

| Resource                       | Link                                                                               |
| ------------------------------ | ---------------------------------------------------------------------------------- |
| Organization                   | [github.com/xch-dev](https://github.com/xch-dev)                                   |
| Documentation site             | [docs.xch.dev](https://docs.xch.dev/)                                              |
| Getting started                | [docs.xch.dev/getting-started](https://docs.xch.dev/getting-started/)              |
| Wallet SDK patterns            | [Application patterns](https://docs.xch.dev/sdk/patterns)                          |
| **chia-wallet-sdk** (repo)     | [xch-dev/chia-wallet-sdk](https://github.com/xch-dev/chia-wallet-sdk)              |
| **chia-wallet-sdk** (Rust API) | [docs.rs/chia-wallet-sdk](https://docs.rs/chia-wallet-sdk/latest/chia_wallet_sdk/) |
| **rue** (compiler)             | [xch-dev/rue](https://github.com/xch-dev/rue)                                      |
| Rue docs site                  | [xch-dev/rue-docs](https://github.com/xch-dev/rue-docs)                            |
| Wallet SDK + Sage docs         | [xch-dev/docs](https://github.com/xch-dev/docs)                                    |
| **sage** (reference app)       | [xch-dev/sage](https://github.com/xch-dev/sage)                                    |
| Sage dApp example              | [xch-dev/sage-dapp-example](https://github.com/xch-dev/sage-dapp-example)          |
| Rue VS Code extension          | [xch-dev/rue-vscode](https://github.com/xch-dev/rue-vscode)                        |
| Rue playground                 | [play.rue-lang.com](https://play.rue-lang.com)                                     |
| Explorer                       | [xch-dev/explorer](https://github.com/xch-dev/explorer)                            |

### Chia Network (official)

| Resource               | Link                                                                               |
| ---------------------- | ---------------------------------------------------------------------------------- |
| Documentation          | [docs.chia.net](https://docs.chia.net/)                                            |
| DIDs                   | [Academy — DIDs](https://docs.chia.net/academy-did)                                |
| Verifiable credentials | [VC guide](https://docs.chia.net/guides/verifiable-credentials-guide/)             |
| DID RPC                | [DID RPC reference](https://docs.chia.net/reference-client/rpc-reference/did-rpc)  |
| DID CLI                | [DID CLI reference](https://docs.chia.net/reference-client/cli-reference/did-cli/) |
| Primitives             | [Chia primitives](https://docs.chia.net/guides/primitives)                         |
| **chia_rs**            | [Chia-Network/chia_rs](https://github.com/Chia-Network/chia_rs)                    |
| **clvm_rs**            | [Chia-Network/clvm_rs](https://github.com/Chia-Network/clvm_rs)                    |
| **chips** (standards)  | [Chia-Network/chips](https://github.com/Chia-Network/chips)                        |
| Full node reference    | [chia-blockchain](https://github.com/Chia-Network/chia-blockchain)                 |

### Yakuhito (naming: slot-machine, XCHandles)

| Resource                                 | Link                                                                                |
| ---------------------------------------- | ----------------------------------------------------------------------------------- |
| GitHub profile                           | [github.com/Yakuhito](https://github.com/Yakuhito)                                  |
| **slot-machine** (CLI + slots primitive) | [Yakuhito/slot-machine](https://github.com/Yakuhito/slot-machine)                   |
| **XCHandles** docs                       | [docs.xchandles.com](https://docs.xchandles.com/)                                   |
| XCHandles technical manual               | [docs.xchandles.com/technical-manual](https://docs.xchandles.com/techincal-manual/) |
| **CATalog** docs (related CAT registry)  | [docs.catalog.cat](https://docs.catalog.cat/)                                       |
| CHIP-0054 / 0055 (XCHandles & CATalog)   | [chips PR #192](https://github.com/Chia-Network/chips/pull/192)                     |
| CHIP-0050 / 0051 (action layer & slots)  | [chips PR #165](https://github.com/Chia-Network/chips/pull/165)                     |

### Enterprise identity (Spec 2 — full link list)

→ **[enterprise-identity-spec.md](enterprise-identity-spec.md)** (WebAuthn, OIDC, SAML, SCIM, LDAP, Microsoft Entra)

---

## 1. POC Focus: Passkey Login on Chia

> **Philosophy:** Ship one feature that works perfectly before building everything else. That feature is **Passkey Login anchored to Chia blockchain DID**.

### The One Feature: "Login with PEGIN"

A user clicks "Login with PEGIN" on any website. They authenticate with their device biometrics (Face ID, fingerprint, PIN). Behind the scenes, a Chia DID verifies their identity. No passwords. No central server. The website gets a signed JWT proving the user is who they claim to be.

That's the entire POC. Everything else (SAML, SCIM, LDAP, custody, vault, tokens) comes later.

### Why Passkey-First?

- **Users already know it:** Face ID, fingerprint, Windows Hello — already on every device
- **Zero learning curve:** No seed phrases, no wallet setup for the initial login
- **FIDO2 standard:** Phishing-resistant by design, backed by Apple/Google/Microsoft
- **Blockchain invisible:** User doesn't know Chia exists (it's infrastructure, not UX)
- **Enterprise ready:** FIDO2/WebAuthn is already approved by most enterprise security teams
- **Differentiator:** No other SSO provider anchors passkeys to a blockchain DID

### POC Architecture (Minimal)

```
User Device                  PEGIN Service              Chia Blockchain
┌──────────┐                ┌──────────────┐           ┌──────────────┐
│ Passkey   │───register───▶│ WebAuthn RP  │──anchor──▶│ DID Coin     │
│ (Face ID) │               │ (Rust/Axum)  │           │ (Chialisp)   │
│           │───login──────▶│              │──verify──▶│              │
│           │◀──JWT─────────│              │           │              │
└──────────┘                └──────┬───────┘           └──────────────┘
                                   │
                            ┌──────▼───────┐
                            │ DIG Network  │
                            │ (user data,  │
                            │  audit logs) │
                            └──────────────┘
```

### POC Deliverables

1. `pegin-core` — Rust crate: DID creation on Chia, WebAuthn registration/login
2. `pegin-contracts` — Rue smart contracts: DID registration, credential anchoring
3. `@pegin/sdk` — TypeScript SDK: "Login with PEGIN" button for any website
4. `pegin-demo` — Demo web app showing the login flow

### POC Success Criteria

- User registers with passkey in under 5 seconds
- User logs in with passkey in under 1 second
- DID anchored on Chia testnet
- Login works across Chrome, Safari, Firefox
- Zero passwords, zero seed phrases in the user flow
- Demo website shows "Login with PEGIN" button working end-to-end

---

## 2. xch-dev Ecosystem

**Source:** [xch.dev](https://xch.dev) · [github.com/xch-dev](https://github.com/xch-dev) — unofficial org focused on making Chia development easier ([org README](https://github.com/xch-dev)).

This is the **primary Rust toolchain** for PEGIN: wallet operations, contracts (Rue), and the Sage app pattern.

### xch-dev repositories

| Repository                                                        | Docs                                                                                                                   | PEGIN use                                                         |
| ----------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------- |
| [**chia-wallet-sdk**](https://github.com/xch-dev/chia-wallet-sdk) | [docs.rs](https://docs.rs/chia-wallet-sdk/latest/chia_wallet_sdk/) · [SDK patterns](https://docs.xch.dev/sdk/patterns) | **Critical** — `DidInfo`, `NftInfo`, `MerkleTree`, `SpendContext` |
| [**rue**](https://github.com/xch-dev/rue)                         | [rue-docs](https://github.com/xch-dev/rue-docs) · [playground](https://play.rue-lang.com)                              | **Critical** — PEGIN contracts → CLVM                             |
| [**sage**](https://github.com/xch-dev/sage)                       | [docs.xch.dev](https://docs.xch.dev/)                                                                                  | **Reference** — Tauri v2 + Rust + React                           |
| [sage-dapp-example](https://github.com/xch-dev/sage-dapp-example) | —                                                                                                                      | WalletConnect / dApp integration                                  |
| [rue-vscode](https://github.com/xch-dev/rue-vscode)               | —                                                                                                                      | IDE support                                                       |
| [docs](https://github.com/xch-dev/docs)                           | [docs.xch.dev](https://docs.xch.dev/)                                                                                  | Wallet SDK + Sage documentation source                            |
| [explorer](https://github.com/xch-dev/explorer)                   | —                                                                                                                      | On-chain debugging                                                |
| [auctions](https://github.com/xch-dev/auctions)                   | —                                                                                                                      | Complex contract patterns                                         |

### Rue

Typed language → CLVM ([rue](https://github.com/xch-dev/rue)). Learn from [rue-docs](https://github.com/xch-dev/rue-docs); test in [play.rue-lang.com](https://play.rue-lang.com). PEGIN contracts (`pegin_did.rue`, etc.) compile with `rue-cli` (install from xch-dev/rue releases).

### Sage (reference architecture)

[Sage](https://github.com/xch-dev/sage) — Rust core + React + [Tauri v2](https://v2.tauri.app/) + [chia-wallet-sdk](https://github.com/xch-dev/chia-wallet-sdk). Documented at [docs.xch.dev](https://docs.xch.dev/). PEGIN dashboard/desktop should mirror this layout.

### chia-wallet-sdk (core dependency)

| Type / API     | Use in PEGIN                                                                                          |
| -------------- | ----------------------------------------------------------------------------------------------------- |
| `DidInfo`      | Create/update DIDs — see [Chia DID RPC](https://docs.chia.net/reference-client/rpc-reference/did-rpc) |
| `NftInfo`      | Credential NFTs                                                                                       |
| `MerkleTree`   | Bulk provisioning commitments                                                                         |
| `SpendContext` | Transaction building — see [SDK patterns](https://docs.xch.dev/sdk/patterns)                          |

```toml
# Pin versions from https://github.com/xch-dev/chia-wallet-sdk/releases
chia-wallet-sdk = "0.23"
chia-protocol = "0.25"
chia-bls = "0.14"
clvm-traits = "0.14"
chia-sdk-test = "0.26"
```

---

## 2b. Yakuhito: slot-machine & XCHandles

**Not part of xch-dev** — maintained by [Yakuhito](https://github.com/Yakuhito) (warp.green, TibetSwap, XCHandles, CATalog).

| Project          | Repository                                                        | Official docs                                                                                                    | PEGIN use                                            |
| ---------------- | ----------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------- |
| **slot-machine** | [Yakuhito/slot-machine](https://github.com/Yakuhito/slot-machine) | [XCHandles docs](https://docs.xchandles.com/) · [CATalog slots](https://docs.catalog.cat/technical-manual/slots) | Decentralized naming (`alice.pegin`), slot primitive |
| **XCHandles**    | (via slot-machine + on-chain deployment)                          | [docs.xchandles.com](https://docs.xchandles.com/)                                                                | Human-readable names → DID / address                 |
| **CATalog**      | related registry                                                  | [docs.catalog.cat](https://docs.catalog.cat/)                                                                    | CAT / metadata patterns                              |

**Chia standards (official proposals):**

- [CHIP-0050 / 0051 — Action layer & slots](https://github.com/Chia-Network/chips/pull/165)
- [CHIP-0054 / 0055 — XCHandles & CATalog](https://github.com/Chia-Network/chips/pull/192)

Verify mainnet deployment: `cargo r xchandles verify-deployment` (documented in [XCHandles FAQ](https://docs.xchandles.com/)).

---

## 3. DIG Network Stack

**Source:** [github.com/DIG-Network](https://github.com/DIG-Network) — 45 repositories

DIG is the **data layer** for PEGIN. User data, audit logs, credentials, and session state live on DIG network (not a central database). PEGIN operates as one DIG peer among many.

### Key Repositories for PEGIN

| Repository                        | Language     | Purpose                        | PEGIN Use                        |
| --------------------------------- | ------------ | ------------------------------ | -------------------------------- |
| **`dig-l2-storage`**              | **Rust**     | RocksDB-based L2 block storage | Core storage for PEGIN user data |
| **`chia-block-listener`**         | **Rust**     | Blockchain event listener      | React to on-chain PEGIN events   |
| **`DataLayer-Driver`**            | **Rust**     | Chia Datalayer Rust driver     | Interface with Chia datalayer    |
| **`dig-wallet`**                  | **Rust**     | DIG wallet implementation      | Key management                   |
| **`dig-key-store`**               | **Rust**     | Key storage                    | Secure key storage               |
| **`dig-collateral-coin`**         | **Rust**     | Collateral coin driver         | DIG economics                    |
| **`digstore`**                    | **Rust**     | CLI for local store management | Create/manage credential stores  |
| **`SingletonActionLayerDriver`**  | **Rust**     | Singleton action layer         | Smart contract interaction       |
| **`proof-of-storage-continuity`** | **Rust**     | Storage proof verification     | Verify DIG peer data integrity   |
| `dig-chia-sdk`                    | TypeScript   | DIG SDK for JavaScript         | Client-side DIG integration      |
| `dig-sdk-v2`                      | TypeScript   | Next-gen DIG SDK               | Evaluate for new features        |
| `dig-chia-cli`                    | TypeScript   | DIG CLI tool                   | Dev tooling                      |
| `chia-dig-node`                   | Shell/Docker | Docker compose for DIG node    | Run PEGIN as DIG peer            |
| `coinscript`                      | TypeScript   | Chialisp transcompiler         | Alternative to Rue for TS devs   |
| `gun.rs`                          | **Rust**     | GunJS port (P2P database)      | Real-time P2P session sync       |
| `reward-distributor-clsp`         | Shell        | Reward distribution            | PEGIN token distribution         |
| `data-capsules`                   | JavaScript   | Data encapsulation             | Credential packaging             |
| `proof-of-work`                   | JavaScript   | PoW module                     | Spam prevention                  |

### DIG Architecture Summary

DIG encodes data into a Merkle tree, stores the Merkle root on Chia blockchain, and serves data via a P2P network of peers. Any peer can verify data integrity by checking the root. PEGIN stores user data in DIG stores and runs as one peer — if PEGIN shuts down, other peers still serve the data.

---

## 4. Chia Network Repositories

**Source:** [Chia-Network](https://github.com/Chia-Network) · documentation at [docs.chia.net](https://docs.chia.net/).

| Repository        | Official link                                                                              | PEGIN use                            |
| ----------------- | ------------------------------------------------------------------------------------------ | ------------------------------------ |
| `chia_rs`         | [github.com/Chia-Network/chia_rs](https://github.com/Chia-Network/chia_rs)                 | Signing, protocol types              |
| `clvm_rs`         | [github.com/Chia-Network/clvm_rs](https://github.com/Chia-Network/clvm_rs)                 | CLVM execution                       |
| `chia-blockchain` | [github.com/Chia-Network/chia-blockchain](https://github.com/Chia-Network/chia-blockchain) | Testnet / protocol reference         |
| `chips`           | [github.com/Chia-Network/chips](https://github.com/Chia-Network/chips)                     | Standards (DID, VC, XCHandles CHIPs) |

**Wallet operations:** use [xch-dev/chia-wallet-sdk](https://github.com/xch-dev/chia-wallet-sdk), not legacy CNI wallet SDK paths.

### Chia Crate Ecosystem (crates.io)

```
chia                    # Meta-crate exporting all Chia crates
├── chia-protocol       # Network protocol message types
├── chia-consensus      # Block validation, weight proofs
├── chia-bls            # BLS12-381 signatures
├── chia-puzzles        # Standard puzzle library
├── chia-traits         # Core trait definitions
├── chia-ssl            # TLS/SSL for peer connections
└── clvm-traits         # CLVM serialization traits

chia-wallet-sdk         # xch-dev: wallet operations
├── chia-sdk-driver     # Coin spending drivers
├── chia-sdk-types      # SDK type definitions
├── chia-sdk-signer     # Transaction signing
├── chia-sdk-test       # Testing simulator
└── chia-sdk-bindings   # FFI/WASM bindings

rue-cli                 # xch-dev: Rue compiler CLI
rue-parser              # Rue parser
rue-compiler            # Rue → CLVM compiler
rue-typing              # Rue type system
```

---

## 5. Enterprise SSO (Spec 2)

Full **Active Directory / Microsoft Entra / SAML / OIDC / SCIM** specification with **official documentation links only**:

→ **[enterprise-identity-spec.md](enterprise-identity-spec.md)**

Summary:

| Protocol         | Phase | Official spec                                                                                        |
| ---------------- | ----- | ---------------------------------------------------------------------------------------------------- |
| WebAuthn / FIDO2 | 0     | [W3C WebAuthn](https://www.w3.org/TR/webauthn-3/)                                                    |
| OIDC             | 1     | [OIDC Core 1.0](https://openid.net/specs/openid-connect-core-1_0.html)                               |
| SAML 2.0         | 1     | [OASIS SAML 2.0](https://www.oasis-open.org/standard/saml/)                                          |
| OAuth 2.0        | 1     | [RFC 6749](https://www.rfc-editor.org/rfc/rfc6749)                                                   |
| SCIM 2.0         | 3     | [RFC 7644](https://www.rfc-editor.org/rfc/rfc7644)                                                   |
| LDAP             | 3     | [RFC 4511](https://www.rfc-editor.org/rfc/rfc4511)                                                   |
| Entra federation | 1–3   | [Microsoft SAML/OIDC docs](https://learn.microsoft.com/en-us/entra/identity-platform/saml-sso-setup) |

**Competing with Azure SSO** = implementing these open standards as a SAML/OIDC IdP plus SCIM provisioning; not cloning every Entra portal feature. On-prem AD DS (Kerberos domain) is out of scope for v1 — see Spec 2.

---

## 6. Optimal Tech Stack

### Language Choices

| Layer                     | Language               | Rationale                                                  |
| ------------------------- | ---------------------- | ---------------------------------------------------------- |
| **Identity Engine**       | **Rust**               | Performance, safety, matches Chia/DIG ecosystem direction  |
| **Smart Contracts**       | **Rue**                | Type-safe Chialisp alternative with Rust-like syntax       |
| **Web SDK & Dashboard**   | **TypeScript**         | React ecosystem, browser compatibility, developer adoption |
| **WASM Bridge**           | **Rust → WebAssembly** | Run crypto in browser (via `chia-sdk-bindings` or custom)  |
| **Desktop/Mobile App**    | **Rust + TypeScript**  | Tauri v2 (same pattern as Sage Wallet)                     |
| **Scripts & Prototyping** | **Python**             | Chia ecosystem compatibility, testing                      |

### Full Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      CLIENT LAYER                           │
│                                                             │
│  @pegin/sdk (TypeScript)       Tauri v2 Desktop App         │
│  ├── "Login with PEGIN" button ├── Rust backend             │
│  ├── @simplewebauthn/browser   ├── React + Shadcn UI        │
│  ├── JWT session management    └── (same pattern as Sage)   │
│  └── WASM crypto bridge                                     │
└──────────────────────┬──────────────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────────────┐
│                    PROTOCOL LAYER                            │
│                    (Rust / Axum)                              │
│                                                             │
│  ┌─────────────┐ ┌─────────────┐ ┌───────────────────────┐ │
│  │ WebAuthn    │ │ OIDC        │ │ SAML 2.0              │ │
│  │ passkey-rs  │ │openidconnect│ │ samael                │ │
│  │ (1Password) │ │             │ │                       │ │
│  └──────┬──────┘ └──────┬──────┘ └───────────┬───────────┘ │
│         └───────────────┼───────────────────┘               │
│                         ▼                                    │
│  ┌──────────────────────────────────────────────────────┐   │
│  │               PEGIN CORE ENGINE (Rust)                │   │
│  │  ├── DID manager (chia-wallet-sdk DidInfo)            │   │
│  │  ├── Credential manager (chia-wallet-sdk NftInfo)     │   │
│  │  ├── Session manager (JWT + DIG store)                │   │
│  │  ├── Recovery manager (email + multi-sig)             │   │
│  │  └── Audit logger (DIG append-only log)               │   │
│  └──────────────────────┬───────────────────────────────┘   │
└──────────────────────────┼──────────────────────────────────┘
                           │
┌──────────────────────────▼──────────────────────────────────┐
│                   BLOCKCHAIN LAYER                           │
│                                                             │
│  Chia Network                                               │
│  ├── chia_rs / chia-protocol (consensus, signing)           │
│  ├── chia-wallet-sdk (DID, NFT, MerkleTree, coins)          │
│  ├── clvm_rs (CLVM virtual machine)                         │
│  └── Rue smart contracts:                                   │
│      ├── pegin_did.rue (DID registration + update)          │
│      ├── pegin_credential.rue (credential NFT issuance)     │
│      ├── pegin_issuer.rue (issuer registration)             │
│      ├── pegin_recovery.rue (multi-sig timelocked recovery) │
│      └── pegin_revoke.rue (credential revocation)           │
│                                                             │
│  DIG Network                                                │
│  ├── dig-l2-storage (Rust, RocksDB — user data)             │
│  ├── chia-block-listener (Rust — on-chain events)           │
│  ├── DataLayer-Driver (Rust — Chia Datalayer interface)     │
│  ├── dig-wallet / dig-key-store (key management)            │
│  └── XCHandles / slot-machine (naming: alice.pegin)         │
└─────────────────────────────────────────────────────────────┘
```

### Rust workspace and dependencies

**Crate layout (DDD modules, layering, SQL policy):** [../../10-architecture/application-architecture.md](../../10-architecture/application-architecture.md).  
POC may start with a subset (`pegin-domain`, `pegin-identity`, `pegin-auth`, `pegin-infrastructure`, `pegin-api`) before splitting `pegin-protocols`.

```toml
[workspace]
# Target members — see application-architecture.md for boundaries
members = [
  "pegin-domain",
  "pegin-identity",
  "pegin-auth",
  "pegin-infrastructure",
  "pegin-api",
  "pegin-protocols",
  "pegin-contracts",
  "pegin-cli",
]

[workspace.dependencies]
# ── Chia Blockchain (xch-dev + Chia-Network) ─────────────
chia-wallet-sdk = "0.23"       # DID, NFT, wallet (xch-dev)
chia-protocol = "0.25"          # Protocol types (Chia-Network)
chia-bls = "0.14"               # BLS signatures (Chia-Network)
clvm-traits = "0.14"            # CLVM types (Chia-Network)
chia-sdk-test = "0.26"          # Test simulator (xch-dev)

# ── Authentication ───────────────────────────────────────
passkey = "0.3"                 # WebAuthn/FIDO2 (1Password)
samael = "0.0.17"               # SAML 2.0
openidconnect = "4.0"           # OpenID Connect
oauth2 = "5.0"                  # OAuth 2.0
jsonwebtoken = "9.3"            # JWT

# ── Crypto ───────────────────────────────────────────────
ed25519-dalek = "2.1"           # Ed25519
sha2 = "0.10"                   # SHA-256
chacha20poly1305 = "0.10"       # Encryption
rand = "0.8"                    # RNG

# ── Web Framework ────────────────────────────────────────
axum = "0.8"                    # HTTP (async, tower-based)
tower = "0.5"                   # Middleware
tower-http = "0.6"              # CORS, tracing
tokio = { version = "1.40", features = ["full"] }

# ── Data ─────────────────────────────────────────────────
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
quick-xml = "0.37"              # XML for SAML
rocksdb = "0.22"                # DIG L2 storage
uuid = "1.11"

# ── WASM (browser target) ───────────────────────────────
wasm-bindgen = "0.2"
```

### TypeScript Dependencies (package.json)

```json
{
  "dependencies": {
    "@simplewebauthn/browser": "^13.0.0",
    "@simplewebauthn/server": "^13.0.0",
    "next": "^15.0.0",
    "react": "^19.0.0",
    "@radix-ui/react-*": "latest",
    "tailwindcss": "^4.0.0",
    "jose": "^6.0.0",
    "zod": "^3.0.0"
  }
}
```

---

## 7. Business Stack (Separated from Tech)

> **Rule:** Tech team and business team work independently. They share interfaces via smart contracts (fee structures) and DIG store schemas.

### Business Implementation Stack

| Component                  | Technology                                                                                                   | Team     |
| -------------------------- | ------------------------------------------------------------------------------------------------------------ | -------- |
| **PEGIN Governance Token** | Rue CAT contract + `slot-machine`                                                                            | Business |
| **Credential NFTs**        | Rue NFT contract with royalty puzzle                                                                         | Shared   |
| **Issuer Registration**    | Rue contract (minting fee)                                                                                   | Shared   |
| **DIG Peer Revenue**       | `chia-dig-node` Docker deployment                                                                            | Business |
| **Token Distribution**     | `reward-distributor-clsp`                                                                                    | Business |
| **Analytics Dashboard**    | TypeScript + Recharts                                                                                        | Business |
| **Naming System**          | [Yakuhito/slot-machine](https://github.com/Yakuhito/slot-machine) + [XCHandles](https://docs.xchandles.com/) | Shared   |

### Revenue Streams (Technical Implementation)

| Stream                | How It Works Technically                                                    |
| --------------------- | --------------------------------------------------------------------------- |
| **DIG Storage Fees**  | Run DIG peer → enterprises replicate data to my node → DIG pays me          |
| **Indexer Fees**      | Run `chia-block-listener` + custom indexer → dApps query → charge per query |
| **NFT Royalties**     | Rue royalty puzzle in credential NFT → 2-5% on trades                       |
| **Issuer Fees**       | Rue contract charges 0.1-1 XCH to mint issuer NFT                           |
| **Custody Referrals** | Integrate Fireblocks/banks API → referral fee (business contract, not code) |

### Team Boundary

```
TECH TEAM                           BUSINESS TEAM
├── pegin-core (Rust)               ├── Token economics model
├── pegin-protocols (Rust)          ├── Enterprise sales
├── pegin-contracts (Rue)           ├── Partnership/referral deals
├── @pegin/sdk (TypeScript)         ├── Compliance/legal
├── pegin-dashboard (React)         ├── Marketing/community
├── DIG integration                 ├── Pricing strategy
└── WASM bridge                     └── Analytics dashboard

SHARED INTERFACES
├── Rue smart contracts (define fee logic)
├── DIG store schema (define data model)
└── NFT metadata standard (define credential format)
```

---

## 8. Implementation Roadmap

### Phase 0: Setup (Week 1-2)

```
□ Rust workspace per [application-architecture.md](../../10-architecture/application-architecture.md)
□ `pegin-testing` harness + CI tiers per [test-architecture.md](../../08-developer/engineering/test-architecture.md)
□ rustfmt + clippy policy per [linting-and-formatting.md](../../08-developer/engineering/linting-and-formatting.md)
□ TypeScript workspace: @pegin/sdk, pegin-dashboard
□ Install Rue CLI (cargo install rue-cli)
□ Set up rue-vscode extension
□ Deploy Chia testnet node
□ Deploy DIG testnet peer (chia-dig-node Docker)
□ Study Sage Wallet architecture (xch-dev/sage)
□ Study chia-wallet-sdk DID/NFT examples (xch-dev/docs)
```

### Phase 1: "Login with PEGIN" POC (Week 3-8)

```
CORE (Rust):
□ DID creation via chia-wallet-sdk DidInfo
□ WebAuthn registration/login via passkey-rs
□ Link passkey credential to Chia DID
□ JWT token issuance after login
□ Audit log writes to DIG store
□ Axum HTTP server with WebAuthn endpoints

CONTRACTS (Rue):
□ pegin_did.rue — DID registration contract
□ pegin_credential.rue — Credential NFT contract
□ Compile with rue-cli, test with chia-sdk-test

SDK (TypeScript):
□ @pegin/sdk — "Login with PEGIN" button component
□ @simplewebauthn/browser integration
□ JWT session management
□ Demo website showing login flow
```

### Phase 2: Enterprise Protocols (Week 9-14)

```
□ OIDC Provider implementation (openidconnect crate)
□ SAML 2.0 IdP implementation (samael crate)
□ OAuth 2.0 authorization server
□ SCIM 2.0 server (bulk provisioning)
□ Merkle tree bulk user provisioning via chia-wallet-sdk MerkleTree
□ Azure AD SAML federation test
□ Email recovery via DIG network
□ Multi-sig recovery via Chia Signer
```

### Phase 3: Enterprise Features (Week 15-20)

```
□ LDAP gateway (translate LDAP queries to DIG lookups)
□ Group management (group NFTs)
□ RBAC (role-based credential attributes)
□ Conditional access (Rue smart contract conditions)
□ Compliance audit reports from DIG logs
□ Tauri v2 desktop app (like Sage)
□ Security audit of Rue contracts
```

### Phase 4: Business & Token (Week 21-24)

```
□ PEGIN CAT token (Rue contract + slot-machine)
□ Credential NFT royalties (Rue royalty puzzle)
□ Issuer registration fees
□ DIG peer with storage fee collection
□ Token distribution (reward-distributor-clsp)
□ Naming system (slot-machine: alice.pegin)
□ Launch testnet beta
```

---

## Quick Reference Card

```
PEGIN DSSO — Quick Reference

POC Feature:  Passkey Login anchored to Chia DID
Languages:    Rust (core) + Rue (contracts) + TypeScript (SDK/UI)
Blockchain:   Chia (chia-wallet-sdk, chia_rs, clvm_rs)
Contracts:    Rue language (xch-dev/rue) → CLVM bytecode
Data:         DIG Network (dig-l2-storage, DataLayer-Driver)
Auth:         passkey-rs (WebAuthn) → OIDC → SAML → SCIM → LDAP
App Pattern:  Tauri v2 (Rust + React), same as Sage Wallet
Recovery:     Email (DIG) + Multi-sig (Chia Signer)
Token:        PEGIN CAT (Rue) + NFT credentials
Revenue:      DIG peer fees + NFT royalties + custody referrals
Naming:       slot-machine (alice.pegin)
POC:          8 weeks, 2-3 devs
Full v1.0:    24 weeks, 4-7 people
```

---

_Built with 🐧 by the PEGIN team. Waddle in, authenticated out._
