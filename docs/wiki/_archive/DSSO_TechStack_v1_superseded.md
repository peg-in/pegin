# 🐧 PEGIN — Decentralized Single Sign-On (DSSO)

## Technical Stack, Planning & Implementation Guide

> **PEGIN** = **P**enguin **G**ateway **I**de**n**tity
> A fully decentralized SSO built on Chia blockchain + DIG network.
> No central servers. No API billing. If you disappear, PEGIN keeps running.

---

## Table of Contents

1. [DIG Network Tech Stack Scan](#1-dig-network-tech-stack-scan)
2. [Chia Network Repositories for SSO](#2-chia-network-repositories-for-sso)
3. [SSO Protocols (SAML, OIDC, SCIM, LDAP)](#3-sso-protocols-for-enterprise-transition)
4. [Active Directory Feature Parity](#4-active-directory-feature-parity)
5. [Current Login Solutions & How PEGIN Improves](#5-current-login-solutions--how-pegin-improves)
6. [Optimal Tech Stack for DSSO](#6-optimal-tech-stack-for-dsso)
7. [Business Stack (Token, Revenue, Incentives)](#7-business-stack-token-revenue-incentives)
8. [Team Structure (Tech vs. Business)](#8-team-structure-tech-vs-business)
9. [POC Implementation Plan](#9-poc-implementation-plan)
10. [PEGIN Memes & Branding](#10-pegin-memes--branding)

---

## 1. DIG Network Tech Stack Scan

**Source:** [github.com/DIG-Network](https://github.com/DIG-Network) — 45 repositories

### Core Architecture

DIG Network is a **decentralized content delivery network** (D-CDN) built on Chia blockchain. Data is encoded and served from a **Merkle tree** with the root stored on blockchain. Any peer can verify data integrity.

### Key Repositories (Relevant to PEGIN)

| Repository | Language | Purpose | PEGIN Integration |
|---|---|---|---|
| `dig-chia-sdk` | TypeScript | SDK to interact with DIG stores | Primary integration point for TS clients |
| `dig-sdk-v2` | TypeScript | Next-gen SDK | Evaluate for new features |
| `dig-chia-cli` | TypeScript | CLI tool for DIG operations | Dev tooling, testing |
| `chia-dig-node` | Shell/Docker | Docker compose to run DIG node | Run PEGIN as DIG peer |
| `dig-l2-storage` | **Rust** | RocksDB-based L2 block storage | Core storage layer for PEGIN data |
| `dig-network-block` | **Rust** | Block structure definitions | Understand DIG block format |
| `chia-block-listener` | **Rust** | Listen to Chia blockchain events | React to on-chain PEGIN events |
| `DataLayer-Driver` | **Rust** | Chia Datalayer driver | Interface with Chia Datalayer |
| `dig-wallet` | **Rust** | DIG wallet implementation | Key management for PEGIN |
| `dig-key-store` | **Rust** | Key storage for DIG | Secure key management |
| `dig-collateral-coin` | **Rust** | Datalayer collateral coin utility | Understand DIG economics |
| `digstore` | **Rust** | CLI to create/manage local stores | Reference for store management |
| `SingletonActionLayerDriver` | **Rust** | Singleton action layer | Smart contract interaction |
| `slot-machine` | **Rust** | Decentralized naming system for Chia | Could be used for PEGIN identity naming |
| `proof-of-storage-continuity` | **Rust** | Proof of storage | Verify DIG peer data integrity |
| `coinscript` | TypeScript | Higher-level Chialisp transcompiler | Simplify smart contract dev |
| `gun.rs` | **Rust** | Rust port of GunJS (P2P database) | Real-time P2P data sync |
| `webtorrent_rs` | **Rust** | Rust port of WebTorrent | P2P content delivery |
| `data-capsules` | JavaScript | Data capsule management | Credential encapsulation |
| `proof-of-work` | JavaScript | PoW npm module | Spam prevention for PEGIN |
| `chia-wallet-sdk` (fork) | **Rust** | Chia wallet SDK (forked) | Wallet operations |
| `chia_crypto_utils` | Dart | Chia crypto utilities | Mobile app (Flutter) |
| `reward-distributor-clsp` | Shell | Reward distribution (Chialisp) | Token distribution mechanism |
| `docs.dig.net` | TypeScript | Documentation site | Reference documentation |

### DIG Stack Summary

- **Primary Languages:** Rust (core/storage), TypeScript (SDK/CLI), Shell (infra)
- **Storage:** RocksDB-based L2 storage, Merkle tree verification
- **Networking:** libp2p for P2P communication, WebTorrent for content delivery
- **Blockchain:** Chia Datalayer integration via Rust drivers
- **Key Insight:** DIG is **actively transitioning to Rust** — perfect alignment with PEGIN's Rust-first approach

---

## 2. Chia Network Repositories for SSO

**Source:** [github.com/Chia-Network](https://github.com/Chia-Network) — 131 repositories

### Repositories Directly Useful for PEGIN SSO

| Repository | Language | Stars | Purpose | PEGIN Use |
|---|---|---|---|---|
| `chia_rs` | **Rust** | 26 | Consensus code & Rust crate | **Core dependency** — signing, verification, coin model |
| `clvm_rs` | **Rust** | 71 | CLVM (Chialisp VM) in Rust | Execute smart contracts for PEGIN |
| `cni-wallet-sdk` | **Rust** | — | Official Chia wallet SDK (Rust) | DID creation, NFT minting, coin operations |
| `chia-blockchain` | Python | 10.8K | Full node implementation | Reference for protocol, run testnet |
| `chia-blockchain-gui` | TypeScript | 340 | Electron/React GUI | UI patterns for PEGIN dashboard |
| `chia-gaming` | JavaScript | 57 | Chia gaming framework | State channel patterns for fast auth |
| `chips` | JavaScript | — | Chia Improvement Proposals | Propose PEGIN DID standard |

### Community Repositories (Not Chia-Network org)

| Repository | Language | Purpose | PEGIN Use |
|---|---|---|---|
| `xch-dev/chia-wallet-sdk` | **Rust** | Community wallet SDK | **Primary Rust SDK** — DID, NFT, coin operations |
| `xch-dev/sage` | **Rust** + TypeScript | Sage Wallet (light wallet) | Reference for DID management, Tauri app pattern |

### Key Chia Rust Crates (crates.io)

```
chia-wallet-sdk     # Wallet operations, DID, NFT
chia-protocol       # Protocol types and serialization
chia-bls            # BLS12-381 signatures
chia-puzzles        # Standard puzzle implementations
chia-sdk-test       # Testing utilities
clvm-traits         # CLVM type system
clvm-derive         # CLVM derive macros
```

### Chia Stack Summary

- **Core Language:** Python (legacy), **Rust** (future — actively migrating)
- **Smart Contracts:** Chialisp (LISP dialect compiled to CLVM bytecode)
- **Signatures:** BLS12-381 (aggregatable, efficient)
- **Coin Model:** UTXO-like "coin set" model
- **DIDs:** Native DID support in wallet SDK (create, transfer, recover)
- **NFTs:** Native NFT support with royalties and metadata

---

## 3. SSO Protocols for Enterprise Transition

To replace Microsoft SSO seamlessly, PEGIN must support **all major enterprise protocols**.

### Protocol Overview

| Protocol | Purpose | Format | Status | PEGIN Support |
|---|---|---|---|---|
| **SAML 2.0** | Web SSO (enterprise standard) | XML assertions | Mature (since 2005) | **Must have** — every enterprise uses it |
| **OpenID Connect (OIDC)** | Modern SSO + identity | JSON/JWT tokens | Growing fast | **Must have** — modern apps expect it |
| **OAuth 2.0** | Authorization (API access) | Bearer tokens | Standard | **Must have** — API authorization |
| **SCIM 2.0** | User provisioning/deprovisioning | REST/JSON | Enterprise standard | **Must have** — bulk user management |
| **LDAP** | Directory services (on-prem) | Binary protocol | Legacy but critical | **Should have** — LDAP gateway for legacy apps |
| **WS-Federation** | Older Microsoft SSO | XML/SOAP | Legacy (Microsoft) | **Nice to have** — M365 compatibility |
| **FIDO2/WebAuthn** | Passwordless authentication | CBOR/JSON | Modern standard | **Must have** — passkey login for PEGIN |
| **Kerberos** | Network authentication (Windows) | Binary tickets | Windows-specific | **Nice to have** — AD domain integration |
| **RADIUS** | Network access authentication | UDP protocol | Legacy | **Nice to have** — VPN/WiFi auth |

### SAML 2.0 (Critical for Enterprise)

**How it works:** Identity Provider (IdP) creates XML assertion → digitally signs it → sends to Service Provider (SP) → SP validates signature → grants access.

**What PEGIN needs:**
- Act as both **IdP** (PEGIN authenticates users) and **SP** (PEGIN accepts external IdP)
- Generate and validate SAML assertions
- Support IdP-initiated and SP-initiated SSO
- X.509 certificate management for assertion signing
- Metadata exchange for trust establishment

**Rust crates:**
- `samael` — SAML 2.0 implementation in Rust
- `openssl` / `rustls` — Certificate management
- `quick-xml` — XML parsing/generation

### OpenID Connect (OIDC)

**How it works:** Built on OAuth 2.0 + identity layer. Uses JWT tokens instead of XML. Supports Authorization Code Flow, Implicit Flow, PKCE.

**What PEGIN needs:**
- OIDC Provider (OP) implementation
- JWT token issuance and validation
- Discovery endpoint (`/.well-known/openid-configuration`)
- UserInfo endpoint
- Support PKCE for SPAs and mobile apps

**Rust crates:**
- `openidconnect` — OIDC client/server implementation
- `jsonwebtoken` — JWT encoding/decoding
- `oauth2` — OAuth 2.0 framework

### SCIM 2.0 (User Provisioning)

**How it works:** REST API standard for creating, reading, updating, and deleting users across systems. Enterprises use SCIM to sync users from Azure AD / Okta to applications.

**What PEGIN needs:**
- SCIM 2.0 server implementation
- `/Users` and `/Groups` endpoints
- Bulk operations support (critical for enterprise)
- Webhook support for real-time sync
- Map SCIM operations to Chia blockchain (merkle tree updates)

### LDAP Gateway

**How it works:** Binary protocol for querying directory services (Active Directory). Legacy apps expect LDAP interface.

**What PEGIN needs:**
- LDAP server facade that translates queries to DIG network lookups
- Bind operations for authentication
- Search operations for user/group queries
- Support `ldaps://` (LDAP over TLS)

**Rust crates:**
- `ldap3` — LDAP client
- Custom LDAP server implementation needed (or use `glauth` in Go as proxy)

---

## 4. Active Directory Feature Parity

For seamless Microsoft SSO transition, PEGIN must replicate these AD features:

### Core AD Features & PEGIN Equivalents

| AD Feature | What It Does | PEGIN Equivalent |
|---|---|---|
| **User Authentication** | Verify identity via Kerberos/NTLM | Passkey/Chia Signer + SAML/OIDC |
| **Single Sign-On** | Login once, access all apps | PEGIN SSO (SAML + OIDC) |
| **Group Policies (GPO)** | Enforce security policies | DIG-stored policy documents + smart contract enforcement |
| **User Provisioning** | Create/update/delete users | SCIM 2.0 + merkle tree bulk provisioning |
| **Group Management** | Organize users into groups | On-chain group NFTs or DIG group stores |
| **Role-Based Access (RBAC)** | Assign permissions by role | Credential NFTs with role attributes |
| **Multi-Factor Auth (MFA)** | Additional verification steps | Passkey (biometric) + Chia Signer (hardware) |
| **Password Reset** | Self-service password recovery | Email recovery + multi-sig recovery |
| **Audit Logging** | Track all auth events | Immutable DIG network logs |
| **Conditional Access** | Risk-based access policies | Smart contract conditions (device, location, time) |
| **Device Registration** | Track corporate devices | Device NFTs linked to user DID |
| **Certificate Services** | Issue/manage X.509 certs | Chia-based PKI (DID documents contain public keys) |
| **Federation** | Trust external identity providers | SAML/OIDC federation with external IdPs |
| **Self-Service Portal** | Users manage own profile | Web dashboard (React/TypeScript) |
| **Directory Sync** | Sync users across systems | SCIM + DIG store replication |
| **Hierarchical OUs** | Organize users by department/location | DIG store hierarchy (department → team → user) |

### Transition Strategy (Azure AD → PEGIN)

1. **Phase 1:** PEGIN acts as SP; Azure AD remains IdP (SAML federation)
2. **Phase 2:** PEGIN syncs users via SCIM from Azure AD
3. **Phase 3:** PEGIN becomes IdP for non-M365 apps; Azure AD for M365
4. **Phase 4:** Full PEGIN (optional); Azure AD connector maintained for M365

---

## 5. Current Login Solutions & How PEGIN Improves

### Current Solutions Comparison

| Solution | Type | Weakness | How PEGIN Improves |
|---|---|---|---|
| **Microsoft Azure AD** | Centralized cloud IdP | Vendor lock-in, data on MS servers | Decentralized, user owns data |
| **Okta** | Centralized SaaS IdP | Expensive at scale ($2-15/user/mo) | No per-user fees (blockchain fees only) |
| **Auth0** | Developer-friendly CIAM | Acquired by Okta, pricing changes | Open-source, no acquisition risk |
| **Keycloak** | Open-source IdP | Requires server infrastructure | No servers needed (blockchain + DIG) |
| **Firebase Auth** | Google cloud auth | Google dependency, limited enterprise | Fully decentralized |
| **MetaMask / WalletConnect** | Web3 wallet auth | Not enterprise-ready, no SAML/OIDC | Full enterprise protocol support |

### PEGIN's Innovation: Decentralized SSO (DSSO)

Traditional SSO: `User → Central Server → Verify → Grant Access`

PEGIN DSSO: `User → Sign with Passkey → Blockchain verifies → DIG stores session → Access granted`

**No central server in the verification path.** Smart contracts verify identity. DIG network stores session state. User controls keys.

---

## 6. Optimal Tech Stack for DSSO

### Primary Languages

| Layer | Language | Rationale |
|---|---|---|
| **Core Identity Engine** | **Rust** | Performance, safety, aligns with Chia/DIG transition to Rust |
| **Web SDKs & Dashboard** | **TypeScript** | Developer ecosystem, React/Next.js, browser compat |
| **Smart Contracts** | **Chialisp** | Native Chia smart contract language |
| **WASM Bridge** | **Rust → WebAssembly** | Run core crypto in browser without JS overhead |
| **Mobile** | **Rust** (via Tauri/UniFFI) | Cross-platform, single codebase |
| **Scripts/Tooling** | **Python** | Chia ecosystem compatibility, quick prototyping |

### Core Technology Stack

```
┌──────────────────────────────────────────────────────┐
│                    CLIENT LAYER                       │
│  TypeScript (React/Next.js) + Rust-WASM              │
│  @simplewebauthn/browser (Passkey UI)                │
│  @pegin/sdk (TypeScript SDK)                         │
│  Tauri v2 (Desktop/Mobile app)                       │
└──────────────────┬───────────────────────────────────┘
                   │
┌──────────────────▼───────────────────────────────────┐
│                  PROTOCOL LAYER                       │
│  Rust Core Engine                                    │
│  ├── SAML 2.0 (samael crate)                         │
│  ├── OIDC/OAuth2 (openidconnect crate)               │
│  ├── SCIM 2.0 (custom Rust implementation)           │
│  ├── WebAuthn/Passkey (passkey-rs by 1Password)      │
│  ├── LDAP Gateway (custom or glauth proxy)           │
│  └── JWT (jsonwebtoken crate)                        │
└──────────────────┬───────────────────────────────────┘
                   │
┌──────────────────▼───────────────────────────────────┐
│                BLOCKCHAIN LAYER                       │
│  Chia Network                                        │
│  ├── chia_rs (consensus, signing)                    │
│  ├── chia-wallet-sdk (DID, NFT, coins)               │
│  ├── clvm_rs (Chialisp VM)                           │
│  ├── BLS12-381 signatures                            │
│  └── Chialisp smart contracts                        │
│      ├── pegin_did.clsp (DID registration)           │
│      ├── pegin_credential.clsp (credential NFT)      │
│      ├── pegin_issuer.clsp (issuer registration)     │
│      ├── pegin_recovery.clsp (multi-sig recovery)    │
│      └── pegin_revoke.clsp (credential revocation)   │
└──────────────────┬───────────────────────────────────┘
                   │
┌──────────────────▼───────────────────────────────────┐
│                  DATA LAYER                           │
│  DIG Network                                         │
│  ├── dig-chia-sdk (TypeScript integration)           │
│  ├── dig-l2-storage (Rust, RocksDB)                  │
│  ├── chia-block-listener (Rust, event listener)      │
│  ├── DataLayer-Driver (Rust, Chia Datalayer)         │
│  └── Merkle tree verification                        │
│                                                      │
│  Stores:                                             │
│  ├── User credentials (encrypted)                    │
│  ├── Audit logs (immutable)                          │
│  ├── Session state                                   │
│  ├── Recovery tokens                                 │
│  └── Enterprise user directories                     │
└──────────────────────────────────────────────────────┘
```

### Rust Crate Dependency Map

```toml
# Cargo.toml for pegin-core

[dependencies]
# Chia Blockchain
chia-wallet-sdk = "0.23"      # DID, NFT, wallet operations
chia-protocol = "0.23"         # Protocol types
chia-bls = "0.14"              # BLS signatures
clvm-traits = "0.14"           # CLVM type system

# Authentication Protocols
passkey = "0.3"                # WebAuthn/FIDO2 (by 1Password)
samael = "0.0.17"              # SAML 2.0
openidconnect = "4.0"          # OpenID Connect
oauth2 = "5.0"                 # OAuth 2.0
jsonwebtoken = "9.3"           # JWT tokens

# Crypto
ed25519-dalek = "2.1"          # Ed25519 signatures
sha2 = "0.10"                  # SHA-256
chacha20poly1305 = "0.10"      # Encryption
rand = "0.8"                   # Random generation

# Web Server
axum = "0.8"                   # HTTP framework
tower = "0.5"                  # Middleware
tokio = "1.40"                 # Async runtime
reqwest = "0.12"               # HTTP client

# Data
serde = "1.0"                  # Serialization
serde_json = "1.0"             # JSON
quick-xml = "0.37"             # XML (SAML)
rocksdb = "0.22"               # Local storage (DIG L2)

# WASM (optional, for browser)
wasm-bindgen = "0.2"           # Rust-WASM bridge
```

### TypeScript Dependencies

```json
{
  "dependencies": {
    "@simplewebauthn/browser": "^13.0.0",
    "@simplewebauthn/server": "^13.0.0",
    "@pegin/sdk": "workspace:*",
    "next": "^15.0.0",
    "react": "^19.0.0",
    "tailwindcss": "^4.0.0",
    "jose": "^6.0.0"
  }
}
```

---

## 7. Business Stack (Token, Revenue, Incentives)

> **Separation Principle:** The business/incentive layer is architecturally separate from the core SSO tech. Different teams can work on each independently.

### Token Infrastructure

| Component | Technology | Purpose |
|---|---|---|
| **PEGIN Governance Token** | Chialisp CAT (Chia Asset Token) | Voting, staking, incentive distribution |
| **Credential NFTs** | Chia NFT standard (NFT1) | Tradeable credentials with royalties |
| **Issuer NFTs** | Chia NFT standard | One-time registration fee |
| **Royalty System** | Chialisp royalty puzzles | 2-5% on credential trades |
| `reward-distributor-clsp` | DIG Network repo | Token distribution to peers |
| `slot-machine` | DIG Network repo (Rust) | Decentralized naming + CAT registration |

### Revenue Implementation

| Revenue Stream | Implementation | Stack |
|---|---|---|
| **DIG Peer Storage Fees** | Run DIG node, get paid for storage | `chia-dig-node` (Docker) |
| **Indexer Query Fees** | Run PEGIN indexer, charge per query | Rust (custom indexer) + `chia-block-listener` |
| **Credential NFT Royalties** | Royalty puzzle in NFT metadata | Chialisp + `chia-wallet-sdk` |
| **Issuer Registration Fees** | Minting fee for issuer NFT | Chialisp smart contract |
| **Enterprise SLA Support** | Managed service contracts | Business operations (not code) |
| **Token Appreciation** | Hold 20% of PEGIN token supply | `slot-machine` for CAT creation |

### Integration Points (Tech ↔ Business)

```
Tech Team builds:                 Business Team builds:
├── Core DSSO engine (Rust)       ├── Token economics model
├── Protocol support              ├── Pricing strategy
├── DIG integration               ├── Enterprise sales
├── Smart contracts               ├── Partnership deals
├── SDKs                          ├── Compliance/legal
└── WASM bridge                   └── Marketing/community

Shared Interface:
├── Chialisp smart contracts (define fee structure)
├── DIG store schema (define data model)
└── Token distribution logic (reward-distributor-clsp)
```

---

## 8. Team Structure (Tech vs. Business)

### Tech Team (Core DSSO)

| Role | Focus | Skills |
|---|---|---|
| **Rust Core Engineer** (1-2) | Identity engine, protocol layer | Rust, cryptography, WebAuthn |
| **Chialisp Developer** (1) | Smart contracts, DID/NFT puzzles | Chialisp, CLVM, Chia consensus |
| **TypeScript Full-Stack** (1-2) | SDK, dashboard, web client | React, Next.js, TypeScript |
| **DIG Integration** (1) | DIG peer, storage, replication | Rust, DIG SDK, networking |

### Business/Incentive Team

| Role | Focus | Skills |
|---|---|---|
| **Token Economist** (1) | Tokenomics, incentive design | DeFi, game theory, modeling |
| **Enterprise Sales** (1) | Enterprise customer acquisition | B2B sales, identity market |
| **Community Lead** (1) | Chia ecosystem, developer relations | Community building, docs |

---

## 9. POC Implementation Plan

### Phase 0: Foundation (Week 1-2)

```
□ Set up Rust workspace (pegin-core, pegin-protocols, pegin-contracts)
□ Set up TypeScript workspace (pegin-sdk, pegin-dashboard)
□ Deploy Chia testnet node
□ Deploy DIG testnet peer
□ Create GitHub repos and CI/CD
```

### Phase 1: DSSO Core (Week 3-6)

```
□ Implement DID registration on Chia (Rust + Chialisp)
□ Implement Passkey/WebAuthn registration and login (passkey-rs)
□ Implement credential NFT issuance
□ Store user data on DIG network
□ Build basic TypeScript SDK (@pegin/sdk)
□ Build minimal web dashboard (React)
```

### Phase 2: Enterprise Protocols (Week 7-10)

```
□ Implement SAML 2.0 IdP (samael crate)
□ Implement OIDC Provider (openidconnect crate)
□ Implement SCIM 2.0 server (custom)
□ Implement bulk user provisioning (merkle tree)
□ Build LDAP gateway (proxy to DIG)
□ Test with Azure AD federation
```

### Phase 3: Recovery & Custody (Week 11-14)

```
□ Implement email recovery (DIG email service)
□ Implement multi-sig recovery (Chialisp + Chia Signer)
□ Implement credential revocation
□ Build enterprise admin dashboard
□ Security audit of smart contracts
```

### Phase 4: Token & Revenue (Week 15-18)

```
□ Create PEGIN CAT token (Chialisp)
□ Implement credential NFT royalties
□ Implement issuer registration fees
□ Set up DIG peer with storage fee collection
□ Build token distribution system
□ Launch testnet beta
```

---

## 10. PEGIN Memes & Branding 🐧

### Meme Ideas for #pegin

**"Login with PEGIN" Button Meme:**
> *Your password manager remembering 847 passwords*
> vs.
> *PEGIN: one passkey, one blockchain, zero passwords* 🐧

**"The Auth0 Breakup" Meme:**
> Enterprise: "We need SSO"
> Auth0: "$15/user/month"
> PEGIN: "How about $0.0001 per blockchain transaction?"
> Enterprise: 👀🐧

**"Decentralized vs Centralized" Meme:**
> Auth0 gets hacked: 100M users lose access
> PEGIN gets "hacked": Nothing happens. It's a blockchain. 🐧

**"The Migration" Meme:**
> IT Admin: "We're migrating from Azure AD to PEGIN"
> Users: "Do I need to change my password?"
> IT Admin: "You don't have a password anymore" 🐧

**"If I Disappear" Meme:**
> CEO of Auth0 disappears: Service goes down
> CEO of Okta disappears: Service goes down
> Creator of PEGIN disappears: System keeps running forever
> 🐧 *built different*

**Branding Tagline Options:**
- *"PEGIN: Identity for the decentralized era"*
- *"Login like a penguin. No passwords. No servers. No problems."*
- *"PEGIN: The last SSO you'll ever need to build."*
- *"Waddle in, authenticated out."* 🐧
- *"PEGIN: Where your identity belongs to you, not a corporation."*

### Logo Concept

A penguin (🐧) walking through a gateway/door, carrying a key. The gateway is made of blockchain blocks. Clean, minimal, tech-forward.

Colors: Arctic blue (#0f7bff) + Dark navy (#1a1a2e) + White

### Hashtags

`#pegin` `#dsso` `#decentralizedSSO` `#penguingateway` `#nopasswords` `#chianetwork` `#dignetwork` `#web3identity`

---

## Quick Reference Card

```
PEGIN DSSO — Quick Start

Language:     Rust (core) + TypeScript (SDK/UI) + Chialisp (contracts)
Blockchain:   Chia Network (chia_rs, clvm_rs, chia-wallet-sdk)
Data:         DIG Network (dig-l2-storage, dig-chia-sdk)
Auth:         passkey-rs (WebAuthn), samael (SAML), openidconnect (OIDC)
Provisioning: SCIM 2.0 (custom), LDAP gateway
Recovery:     Email (DIG) + Multi-sig (Chia Signer)
Token:        PEGIN CAT (Chialisp) + NFT credentials
Revenue:      DIG peer fees + NFT royalties + enterprise SLA

POC Timeline: 18 weeks
Team Size:    4-7 people (tech + business split)
```

---

*Built with 🐧 by the PEGIN team. Let's waddle into the future of identity.*
