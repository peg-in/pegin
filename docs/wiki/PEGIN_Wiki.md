# PEGIN Wiki

> **PEGIN** — Penguin Gateway Identity  
> Decentralized SSO and (planned) permission management on Chia + DIG Network.

**Canonical docs (prefer these for accuracy):**

| Role | Start here |
|------|------------|
| **Principles (anchor)** | [pegin-manifest.md](../01-vision/pegin-manifest.md) |
| **How we work** | [09-how-we-work/README.md](../09-how-we-work/README.md) |
| **Architecture** | [10-architecture/README.md](../10-architecture/README.md) |
| **Specs** | [04-technical/specs/](../04-technical/specs/README.md) |
| **Programmers** | [08-developer/README.md](../08-developer/README.md) |
| **Docs hub** | [docs/README.md](../README.md) |
| **POC scope** | [mvp-strategy.md](../03-use-cases/mvp-strategy.md) |
| **Roadmap** | [roadmap.md](../03-use-cases/roadmap.md) |

This wiki is a **single-file overview**. Technical detail, official links, and checklists live under `docs/08-developer/`.

---

## Contents

1. [Overview](#1-overview)
2. [Core Philosophy](#2-core-philosophy)
3. [Products](#3-products)
 - [3.1 PEGIN SSO Engine](#31-sso-engine--login-with-pegin)
 - [3.2 Permission Platform (PePP)](#32-permission-platform-pepp)
 - [3.3 Penguin Gateway (Web3 SSO)](#33-penguin-gateway--web3-sso)
 - [3.4 Penguin Vault (Key Custody)](#34-penguin-vault--key-custody)
 - [3.5 Developer SDK](#35-developer-sdk)
4. [Use Cases](#4-use-cases)
5. [First Customer Segments](#5-first-customer-segments)
6. [Technical Architecture](#6-technical-architecture)
 - [6.1 Layer Overview](#61-layer-overview)
 - [6.2 Smart Contracts](#62-smart-contracts)
 - [6.3 On-Chain Data Structures](#63-on-chain-data-structures)
 - [6.4 Zero-Knowledge Proofs](#64-zero-knowledge-proofs)
 - [6.5 Recovery Mechanisms](#65-recovery-mechanisms)
 - [6.6 Rust Dependencies](#66-rust-dependencies)
 - [6.7 TypeScript Dependencies](#67-typescript-dependencies)
 - [6.8 Key External Repositories](#68-key-external-repositories)
 - [6.9 Developer documentation (canonical)](#69-developer-documentation-canonical)
7. [DIG Network](#7-dig-network)
8. [Business Model](#8-business-model)
 - [8.1 Revenue Streams](#81-revenue-streams)
 - [8.2 Financial Projections](#82-financial-projections)
 - [8.3 Enterprise Pricing](#83-enterprise-pricing)
 - [8.4 Token Economics](#84-token-economics)
9. [Competitive Position](#9-competitive-position)
10. [Development Roadmap](#10-development-roadmap)
11. [Integrations](#11-integrations)
12. [Strategic Context](#12-strategic-context)
13. [Branding](#13-branding)
14. [Canonical markdown index](#14-canonical-markdown-index)

---

## 1. Overview

PEGIN solves two of the highest-cost problems in enterprise IT:

| Problem | Typical enterprise pain | PEGIN direction |
|---|---|---|
| **Authentication / SSO** | Per-seat IdP cost; password/MFA friction | Passkey + Chia DID; open core |
| **Permission management** | Slow tickets and directory sync | PePP (Phase 2): mobile approve, DIG grants |

**No validated dollar savings in this repository** — measure in pilots .

The deeper innovation is **ownership**: employer grants *permission* to your DID; revocation does not delete the DID.

**Planning targets (not measured yet)**

| Metric | Target / intent |
|---|---|
| POC | Passkey login + JWT on testnet (~8 weeks) |
| v1.0 | Hardened OIDC/SAML path (~24 weeks plan) |
| PePP | Faster approve/revoke workflow — pilot metrics TBD |
| Licensing model | Open source core; SLA/services TBD |

---

## 2. Core Philosophy

> **Canonical anchor:** [pegin-manifest.md](../01-vision/pegin-manifest.md) — **trust is the staple** (foundation & company); layers E1–E11, P1–P14, W1–W11, B1–B6. The sections below are the narrative wiki; if anything conflicts, the manifest wins.

### Identity you own

When you work at a company using Okta or Azure AD, the company owns your login. Every app you visit, every file you open, every login time and location is recorded by the company. When you leave, your account is deleted — and with it, any proof you worked there.

PEGIN inverts this. You hold a Chia DID in your device's secure enclave. Your employer adds a signed permission to that DID — `"Alice can access Company A systems"` — and can revoke the permission when you leave. They can never delete the DID itself, and the credentials it carries (employment records, endorsements, certifications) remain in your wallet permanently.

```
Traditional SSO: Company owns identity → revokes on exit → history gone
PEGIN: You own DID → company grants permission → DID and credentials persist forever
```

### Privacy by design

Your employer sees that you authenticated — not which URLs you visited, at what time, or from where. Your login activity is private. Interviewing while employed remains private. The company receives only what it needs: proof that you are who you claim to be.

### Decentralization first

PEGIN is not a company. It is a protocol — like DNS, SSL, or HTTP: foundational infrastructure that everyone uses, but no single entity controls.

Design principles (see [manifest P1–P6](../01-vision/pegin-manifest.md#layer-ii--pegin-product-principles)):
- **No single point of failure** — protocol + DIG replication; operators and peers can keep serving without one company
- **Anchors on Chia** — DIDs and store commitments; heavy data on DIG, not on chain
- **Open-source code** — anyone can fork, audit, build alternative implementations
- **Minimal mandatory trust** — verify cryptographically; choose your operator and peer set

### Survives its creator

PEGIN is designed to be obsolete. The identity system runs on open-source smart contracts on Chia blockchain and P2P storage on DIG Network. If PEGIN Inc. closes tomorrow, customers keep their DIDs, data stays on DIG, and the open-source community can fork and continue. This is not a limitation — it is the product's core value proposition, and it is structurally impossible for any centralised vendor to match.

```
What happens if PEGIN Inc. disappears:
├─ Smart contracts: immutable on Chia — still work forever
├─ User DIDs: on-chain — still exist and can be verified
├─ Credential NFTs: in user wallets — still owned by users
├─ Data on DIG: replicated across peers — still accessible
├─ Code on GitHub: open-source — community forks and maintains
└─ Revenue keeps flowing: DIG peer income continues without PEGIN managing servers
```

---

## 3. Products

### 3.1 SSO Engine — Login with PEGIN

The SSO engine replaces passwords entirely. Users authenticate using a FIDO2 passkey stored in their device's secure enclave (Face ID, fingerprint, Windows Hello). The passkey is linked to a Chia DID on first registration; every subsequent login is a biometric-only operation that takes less than one second.

**Login flow**

```
User taps "Login with PEGIN"
 → Browser redirects to PEGIN authorize endpoint
 → User scans Face ID / fingerprint
 → Device signs challenge with passkey private key
 → PEGIN resolves DID on Chia blockchain
 → PEGIN queries DIG Network for app permissions
 → Signed JWT issued to relying party
 → User is logged in (< 2 seconds end-to-end)
```

**Protocol support**

| Protocol | POC | v1.0 | Enterprise | Purpose |
|---|:---:|:---:|:---:|---|
| WebAuthn / FIDO2 | ✅ | ✅ | ✅ | Passkey login |
| OIDC | — | ✅ | ✅ | Modern app SSO |
| SAML 2.0 | — | ✅ | ✅ | Enterprise web SSO |
| OAuth 2.0 | — | ✅ | ✅ | API authorisation |
| SCIM 2.0 | — | — | ✅ | Bulk user provisioning |
| LDAP | — | — | ✅ | Legacy directory access |
| WS-Federation | — | — | ✅ | Microsoft 365 compat |
| Kerberos | — | — | ✅ | Windows domain auth |

**Active Directory feature parity**

| AD Feature | PEGIN Equivalent | Phase |
|---|---|---|
| User Authentication | Passkey + Chia DID | POC |
| Single Sign-On | OIDC + SAML | v1.0 |
| User Provisioning | SCIM → Merkle tree on Chia | v1.0 |
| Group Management | Group NFTs or DIG group stores | v1.0 |
| Role-Based Access | Credential NFTs with role attributes | v1.0 |
| Multi-Factor Auth | Passkey (biometric) + Chia Signer | v1.0 |
| Password Reset | Email recovery + multi-sig | v1.0 |
| Audit Logging | Immutable DIG logs | POC |
| Conditional Access | Rue smart contract conditions | Enterprise |
| Device Registration | Device NFTs linked to DID | Enterprise |
| Certificate Services | DID documents as PKI | Enterprise |
| Directory Sync | SCIM + DIG replication | Enterprise |

**Azure AD migration path**

1. **Coexist** — PEGIN federates as SP, Azure as IdP via SAML bridge
2. **Sync** — Azure AD Connector mirrors users bidirectionally (real-time or every 5 minutes)
3. **Parallel** — Non-M365 apps use PEGIN; M365 stays on Azure
4. **Optional full cutover** — enterprise decides if/when to go all-in

**Bulk user provisioning**

Instead of N users = N API calls (Azure AD model), PEGIN uses: N users = 1 blockchain transaction.

```
POST /enterprise/bulk-provision
{
 "users": [
 { "email": "alice@acme.com", "department": "Engineering", "roles": ["dev", "admin"] },
 { "email": "bob@acme.com", "department": "Sales", "roles": ["user"] }
 // ... 10,000 users
 ],
 "signature": "0x..."
}
```

---

### 3.2 Permission Platform (PePP)

PePP replaces Citrix + Active Directory for access control. Instead of static security groups that batch-replicate every four hours, PePP issues **capability tokens** — cryptographic proofs tied to a user's DID that carry scope, expiry, and conditions in-band.

**Speed comparison**

| Step | Citrix + AD | PEGIN PePP |
|---|---|---|
| Manager approval | 2–3 days (ticket backlog) | < 1 minute (phone notification) |
| AD security group sync | 4–8 hours | Instant (event-driven) |
| Cache flush | 1–2 hours (manual) | Automatic |
| Legacy app integration | 1–3 days per app | < 1 hour (one webhook) |
| **Total per request** | **3–7 days** | **< 2 minutes** |
| **Employee offboarding** | **3–7 days (risk window)** | **< 1 second (automatic)** |

**Manager workflow on phone**

```
Push notification: "Alice needs GitHub push access — 1 week"

[Approve] → Face ID → Done (10 seconds total)

Behind the scenes:
 Signed grant on DIG: "Alice may push to main until 2026-06-23"
 Audit append on DIG; store update anchored on Chia (commitment only)
 GitHub polls DIG / webhook → Alice has access immediately
 At expiry: grant revoked on DIG → denied on next check
```

**Rules engine (no code required)**

Managers define access rules in plain language through the phone UI:

- *All interns: read-only on staging — expires with onboarding contract*
- *Emergency prod access: requires manager + security lead approval, 1-hour window, business hours only*
- *Contractors: view files assigned to them — auto-revokes 24 hours after contract end date*

For custom logic, engineers write conditions as Rue smart contracts.

**Multi-sig for high-risk access**

Production database access can require two approvals via two separate passkeys. If either person denies or the time window expires, access is not granted. The attempt and outcome are appended to the DIG audit store; the store update is anchored on Chia.

**Permission schema (app-defined JSON)**

Each app self-describes its permission model:

```json
{
 "app_id": "github.com",
 "permission_groups": [
 {
 "name": "read",
 "scope": ["repos:read"]
 },
 {
 "name": "push",
 "scope": ["repos:write", "push:main"],
 "requires_approval": true,
 "max_duration_hours": 168
 },
 {
 "name": "admin",
 "scope": ["admin:*"],
 "requires_multi_sig": true,
 "max_duration_hours": 1
 }
 ]
}
```

Apps query DIG Network for a user's current permissions with a single HTTP call (< 50ms response). No PEGIN approval needed to update the schema — apps own their own permission definitions.

**Compliance output**

Every grant, expiry, and revocation is appended to **DIG audit stores**; **on-chain anchors** record store updates (not full payloads). Auditors export from DIG and verify against Chia commitments.

```
[2026-05-16 10:23:45] grant.approved — github.com push — 7 days
 approver: sarah@company.com (passkey)
 store_anchor_tx: 0x8f3d9e2c...

[2026-05-23 10:23:46] grant.expired — auto-revoked
 store_anchor_tx: 0x4a2b1f9d...
```

---

### 3.3 Penguin Gateway — Web3 SSO

Penguin Gateway is the Web3-native variant of PEGIN, designed for dApps, DeFi protocols, and metaverses. Where PEGIN SSO targets enterprises replacing Okta/Azure AD, Penguin Gateway targets Web3 dApps that need decentralized authentication with no central server.

**How it works**

```
User clicks "Login with Penguin Gateway" on dApp
 → dApp redirects to Penguin Login UI
 → User authenticates (Passkey or Chia Signer)
 → dApp requests credential (e.g., "prove user is KYC-verified")
 → User approves; ZK proof generated — no underlying data exposed
 → Callback to dApp with JWT + proof
 → dApp verifies proof on Chia blockchain → user logged in
```

**Three-line dApp integration (TypeScript)**

```typescript
import { PenguinGateway } from "@penguin-gateway/sdk";
const pg = new PenguinGateway({
 clientId: "your-dapp-id",
 redirectUri: "https://yourapp.com/callback"
});
pg.login({ scopes: ["profile", "email", "kyc"] });

// In callback:
const user = await pg.callback();
console.log(user.did, user.isKycVerified);
```

**Credential types supported**

| Credential | Description |
|---|---|
| KYC / AML | Identity verification from banks |
| Age verification | Prove age ≥ 18 without revealing exact age |
| Employment | Proof of employment at specific organisation |
| Education | Diplomas and degrees |
| Credit score | Encrypted score from financial institutions |
| Account balance | Proof of funds without revealing exact amount |
| Custom | Any claim issued by an authorized party |

**Competitive position (Penguin Gateway specific)**

| Competitor | Strength | Gap |
|---|---|---|
| Auth0 / Okta | Enterprise scale | Centralized, not Web3 native |
| MetaMask | Mass adoption | Wallet-first, not SSO |
| Lit Protocol | Threshold encryption | Nascent, limited dApp adoption |
| ENS | Network effect | Domain name only, not SSO |

---

### 3.4 Penguin Vault — Key Custody

Penguin Vault is a decentralized identity and key custody platform for users and enterprises who need institutional-grade recovery. It is PEGIN's Phase 2 product, built on the same Chia + DIG infrastructure.

**Three core components**

1. **Penguin Login** — serverless auth via Chia Signer or Passkeys; no central server stores credentials
2. **Penguin KCS** (Key Custody Service) — biometric-verified custodial recovery via trusted institutions (banks, legal guardians, enterprises); timelocked with cancellation window
3. **Vault System** — multi-tier vault architecture for managing DIDs, NFTs, coins, credentials, and access permissions

**Multi-tier vault architecture**

```
Level 1: Personal Vault
 User's primary vault — locked by user's primary key pair
 Stores: DIDs, credentials, personal assets

Level 2: App-Specific Child Vaults
 Isolated vault per integrated dApp
 Separate keys per app; app can only see its own data

Level 3: Master Vault (Enterprise)
 Root vault for hierarchical key management
 Controls all child vaults; requires multi-sig approval
```

**Vault state machine**

```
CREATED → ACTIVE → LOCKED → RECOVERY_INITIATED → RECOVERY_PENDING → RECOVERY_COMPLETED
 ↘ (cancelled) → ACTIVE
 ACTIVE → ARCHIVED
```

**Timelocked recovery**

Recovery is enforced by Chialisp smart contracts at the blockchain level — the process cannot be rushed:

1. Custodian initiates rekey with proof-of-identity
2. Timelock window opens: 24–72 hours
3. Owner can cancel at any time during the window
4. After timelock expires: new keys become active
5. All steps logged immutably on DIG network

**Custody providers (integration partners)**

PEGIN Vault does not hold keys. It orchestrates multi-sig recovery across existing regulated providers:

| Provider type | Examples | What they provide |
|---|---|---|
| Tier-1 banks | JPMorgan, BNY Mellon, Fidelity | Regulated custody with insurance |
| Self-custody | Chia Signer, hardware wallets | User + trusted contact (2-of-3) |
| Hardware wallets | Ledger, Trezor | User + backup seed + recovery contact |
| Web3 institutional | Fireblocks, Copper, Anchorage | Institutional DeFi custody |

**Recovery security matrix**

| Scenario | Email Recovery | Multi-Sig | Time |
|---|:---:|:---:|---|
| Forgot password | ✅ | — | 1 minute |
| Lost device | ✅ | ✅ | 1–48 hours |
| Suspected breach | — | ✅ | 1 hour |
| Employee termination | — | ✅ (revoke) | Instant |
| Contractor offboarding | ✅ (disable) | ✅ required | 1–5 minutes |

---

### 3.5 Developer SDK

**Full guide:** [integration/sdk-guide.md](../08-developer/integration/sdk-guide.md) · **Hub:** [08-developer/README.md](../08-developer/README.md)

The SDK makes PEGIN adoption viral. The target is 5 minutes from `npm install` to a working login button (POC — see [mvp-strategy.md](../03-use-cases/mvp-strategy.md)).

**Install**

```bash
npm install @pegin/sdk
```

**Frontend — one component**

```tsx
import { PeginLogin } from '@pegin/sdk'

export default function Home() {
 return (
 <PeginLogin
 onSuccess={(user) => window.location.href = '/dashboard'}
 />
 )
}
```

`<PeginLogin />` handles the redirect, the passkey challenge, token storage, and the callback. The app receives a `user` object with `did`, `name`, `email`, and `permissions`.

**Backend — one middleware**

```typescript
import { peginAuth } from '@pegin/sdk'

app.get('/api/me', peginAuth(), async (req, res) => {
 const { did, permissions } = req.pegin
 const user = await db.users.findByDid(did)
 res.json({ user, permissions })
})

// Permission guard in one line
app.get('/api/admin', peginAuth(), requirePermission('admin'), handler)
```

**DIG Network queries**

```typescript
import { DIG } from '@pegin/sdk'

const dig = new DIG({ endpoint: 'https://dig.chia.net' })

const { permissions } = await dig.get(did, 'your-app-id')

await dig.updatePermissions(did, 'your-app-id', {
 add: ['write'],
 expires_at: '2026-06-23T00:00:00Z'
})

const admins = await dig.query({ app_id: 'your-app-id', permission: 'admin' })
```

**Existing user databases — zero migration**

```sql
ALTER TABLE users ADD COLUMN did VARCHAR(255) UNIQUE;
```

Users with an existing account link their DID through a one-time flow. New users get a DID on first login. Both can use email+password and PEGIN simultaneously during the transition period.

**SDK release phases**

| Phase | Scope |
|---|---|
| MVP (Weeks 1–4) | React component, Express middleware, Node.js user bridge |
| Production (Weeks 5–8) | Vue, Angular, Svelte; FastAPI, Django, Rails; PostgreSQL, MongoDB, Firebase adapters |
| Enterprise (Weeks 9–12) | Migration tools from Okta/Auth0, audit log export, GDPR data export, compliance reports |

---

## 4. Use Cases

Use cases are ranked by importance and the order in which they should be built.

### #1 — Private, user-owned login (POC foundation)

**The core shift.** Users own their DID. Companies add and remove permissions. No company can delete a user's identity or their proof of past work.

Why it matters first: it is the simplest thing to build, has the broadest market appeal, and every other use case depends on it.

Scenarios it enables immediately:
- **Interviewing privately** — employer cannot see which websites you visit through PEGIN
- **Company shutdown** — DID and signed credentials persist on blockchain; new employer can verify your work history without calling anyone
- **Multi-employer contractor** — one DID, five clients, five permissions; no login hell

### #2 — One-click permission management (Phase 2)

The highest-value enterprise feature. Replaces Citrix and Active Directory for access control. A manager taps Approve on their phone; the developer has access in under two minutes. The same workflow auto-revokes when the time window closes — no manual cleanup, no forgotten accounts.

Savings: quantify per pilot (no generic figure in canonical docs).

### #3 — App-defined permission schema (Phase 2)

Any app defines its own permissions in JSON and stores the schema on DIG Network. Other apps query the same standard API. PEGIN is not the bottleneck — each app controls its own permission model, versioned and updated independently.

### #4 — Freelancer portable credentials (Phase 3)

A freelancer working for five clients accumulates blockchain-signed credentials from each. Past clients can endorse work. New clients can verify the portfolio cryptographically, without calling anyone. The credentials remain in the freelancer's wallet regardless of whether any past client still exists.

### #5 — Credentials as economic assets (Phase 3)

A certification body issues a credential as an NFT to a user's DID. The credential is user-owned. The issuer encodes a royalty puzzle (2–5%) into the NFT — when the credential is transferred or leased, the issuer earns automatically. Clients can hire "verified AWS architect" time and verify the claim in milliseconds.

### #6 — Cross-organisation identity federation (Phase 3)

A contractor working across multiple companies simultaneously holds one DID. Each company issues permissions to the same DID. All companies see the same verified person, without trusting each other's identity systems. One audit trail across all organisations.

### #7 — Global workforce data sovereignty (Phase 4)

A multinational cannot legally store EU employee data on US servers and Chinese employee data outside China simultaneously — Okta cannot comply with all three at once. DIG peers can be deployed country-by-country. Each jurisdiction's peers comply with local law. The DID works everywhere; the data stays local.

---

## 5. First Customer Segments

PEGIN's first customers are not Fortune 500 enterprises. They are organisations that feel identity vendor costs most acutely, already operate infrastructure they want to monetise, or care about open-source and decentralisation enough to adopt early.

### Segment A — Small tech companies and startups (10–200 people)

PEGIN gives them:
- Zero licensing cost
- Passkey login across all apps — no passwords to manage
- SCIM-automated onboarding in < 5 minutes per new hire
- Automatic offboarding: HR removes someone → all access gone in < 1 second

These early adopters drive word-of-mouth. Developers who love PEGIN at a startup carry the preference to their next job, which is often the enterprise customer PEGIN needs in Phase 2.

### Segment B — Public transport and infrastructure operators (500–10,000 staff)

A city metro or regional bus network employs 3,000 people: drivers, station staff, operations centre workers, and administrators. They run a mix of legacy systems (ticketing, scheduling, CCTV) and modern SaaS. They already operate on-premises server rooms for operational technology.

| | Current state | PEGIN |
|---|---|---|
| Permission change | 3–7 days via IT ticket | < 2 minutes, manager's phone |
| Contractor offboarding | Manual, often missed | Automatic on contract-end date |
| Audit trail | Patchwork logs, editable | DIG append-only + on-chain store anchors |
| Data residency | Cloud provider's jurisdiction | DIG peer runs in their own server room |

Why public infrastructure fits especially well:
- **Regulatory audit pressure** — critical infrastructure faces government scrutiny; DIG audit stores with verifiable chain anchors answer "who had access to what and when"
- **High contractor turnover** — seasonal staff and maintenance contractors rotate constantly; automatic deprovisioning eliminates the most common access-control audit finding
- **Existing server rooms** — the metro already maintains rack space; a DIG peer runs on spare hardware and earns XCH passively
- **No vendor dependency** — a government-adjacent body cannot afford vendor lock-in to a company that may be acquired or raise prices

**DIG peer example (3,000-staff metro)**

| | Year 1 | Year 3 |
|---|---|---|

### Segment C — Managed hosting and server service providers

A European VPS or managed hosting company already sells server capacity to thousands of customers. They have idle rack space, high-bandwidth uplinks, and a team that keeps servers online around the clock.

**As a DIG peer operator (immediate passive income):**
The hosting company runs DIG peers using spare capacity and receives XCH incentives paid automatically on-chain. No invoicing, no accounts receivable — the smart contract pays per epoch.

| Capacity | Marginal cost | DIG peer income (Year 3) |
|---|---|---|

**As a PEGIN reseller:**
The hosting company bundles PEGIN as a managed identity service for hosted customers — the startups and small businesses already with them. They offer PEGIN included in the hosting plan and earn DIG peer income from the identity data their customers generate.

### Segment D — Community banks and credit unions (200–5,000 staff)

A regional bank or credit union operates under strict financial regulation (PSD2, GDPR, SOC 2, national banking authority rules). They employ branch staff, back-office teams, and rotating contractors. They typically run a small on-premises server room for core banking systems and cannot legally route identity data through foreign cloud infrastructure.

| | Current state | PEGIN |
|---|---|---|
| Permission change | 3–7 days via IT helpdesk | < 2 minutes, branch manager's phone |
| Contractor offboarding | 3–7 day risk window | Instant, automatic on contract end |
| Audit evidence | Manual log exports, editable | DIG-signed events + chain-anchored store updates |
| Data sovereignty | Cloud provider's jurisdiction | DIG peer in their own server room |

Why banks and credit unions fit:
- **Regulatory audit evidence** — banking regulators require proof of who accessed what systems and when; export from DIG with integrity checks against on-chain store commitments
- **Strict data sovereignty** — community banks under EU, UK, Australian, or Canadian banking law often cannot legally route identity data through US cloud vendors; a self-hosted DIG peer resolves this
- **Staff certification verification** — bank staff hold regulatory certifications (AML officer, licensed financial adviser) that can be issued as verifiable credential NFTs signed by the regulator

**DIG peer example (1,500-staff community bank)**

| | Year 1 | Year 3 |
|---|---|---|

---

## 6. Technical Architecture

### 6.1 Layer Overview

```
┌──────────────────────────────────────────────────────────────────┐
│ CLIENT LAYER │
│ @pegin/sdk (TypeScript/React) Tauri v2 Desktop App │
│ "Login with PEGIN" button Rust backend │
│ @simplewebauthn/browser React + Shadcn UI │
│ JWT session management (same pattern as Sage Wallet) │
└────────────────────────────┬─────────────────────────────────────┘
 │
┌────────────────────────────▼─────────────────────────────────────┐
│ PROTOCOL LAYER (Rust / Axum) │
│ WebAuthn RP OIDC Provider SAML 2.0 IdP │
│ passkey crate openidconnect samael │
│ │ │
│ ┌───────────────────────────▼──────────────────────────────┐ │
│ │ PEGIN CORE ENGINE │ │
│ │ DID manager chia-wallet-sdk DidInfo │ │
│ │ Credential manager chia-wallet-sdk NftInfo │ │
│ │ Session manager JWT + DIG store │ │
│ │ Recovery manager email + multi-sig │ │
│ │ Audit logger → DIG append-only; chain anchors store updates │ │
│ └───────────────────────────┬──────────────────────────────┘ │
└───────────────────────────────┼──────────────────────────────────┘
 │
┌───────────────────────────────▼──────────────────────────────────┐
│ BLOCKCHAIN LAYER │
│ │
│ Chia Network DIG Network │
│ chia_rs / chia-protocol dig-l2-storage (RocksDB) │
│ chia-wallet-sdk (DID/NFT/Merkle) chia-block-listener │
│ clvm_rs (CLVM VM) DataLayer-Driver │
│ Rue smart contracts slot-machine (alice.pegin) │
└──────────────────────────────────────────────────────────────────┘
```

**Language decisions**

| Layer | Technology | Rationale |
|---|---|---|
| Identity engine | Rust + Axum | Performance, memory safety, matches Chia/DIG ecosystem |
| Smart contracts | Rue | Type-safe Chialisp alternative; Rust-like syntax; compile-time error catching |
| Web SDK | TypeScript + React | Browser compatibility, developer adoption, OIDC library ecosystem |
| WASM bridge | Rust → WebAssembly | Cryptographic operations in-browser, no server round-trip |
| Desktop app | Tauri v2 | Reference pattern from xch-dev/sage (Rust backend + React UI) |
| ZK Proofs | circom + snarkjs | ZK circuit compilation and proof generation in TypeScript |
| Testing / scripting | Python | Chia ecosystem tooling compatibility |

### 6.2 Smart Contracts

[Rue](https://github.com/xch-dev/rue) compiles to CLVM bytecode — the same bytecode as Chialisp — but with Rust-like syntax and a type system that catches errors at compile time rather than on-chain.

Contracts live in `pegin-contracts/src/`:

| Contract | Purpose |
|---|---|
| `pegin_did.rue` | DID registration and updates |
| `pegin_credential.rue` | Credential NFT issuance with royalty puzzle |
| `pegin_issuer.rue` | Issuer registration (charges 0.1–1 XCH minting fee) |
| `pegin_recovery.rue` | Multi-sig timelocked account recovery |
| `pegin_revoke.rue` | Credential revocation registry |
| `pegin_vault.rue` | Vault creation and custody rekey (Penguin Vault) |

Compile: `rue-cli build` (install via `cargo install rue-cli`).
Test: `chia-sdk-test` blockchain simulator.
IDE: `rue-vscode` extension.

**Example: Issuer registration contract**

```lisp
(defun issuer_nft ((issuer_data (f issuer_name issuer_pubkey)))
 (assert (valid_public_key issuer_pubkey))
 (assert (not_empty issuer_name))
 (create_coin issuer_nft_puzzle issuer_data 1)
)
```

**Example: Credential verification contract**

```lisp
(defun verify_credential ((credential_data) (issuer_pubkey) (signature))
 ;; Check issuer signature
 (assert (verify_signature issuer_pubkey credential_data signature))
 ;; Check revocation status
 (assert (not (is_revoked credential_id)))
 ;; Check expiration
 (assert (> expiration_date (current_block_time)))
 (return TRUE)
)
```

**Example: Vault creation contract (Penguin Vault)**

```lisp
(defun vault-creation
 (owner-pubkey recovery-keys dig-store-hash)
 (assert (> (len recovery-keys) 0) "At least 1 recovery key required")
 (assert (valid-pubkey owner-pubkey) "Invalid owner key")
 (create-coin owner-pubkey vault-metadata)
)
```

### 6.3 On-Chain Data Structures

**User DID (W3C compatible)**

```
Format: did:chia:pubkeyhash (e.g., did:chia:xyz123abc456)
```

**Identity NFT structure**

```json
{
 "id": "pegin-identity-{did}",
 "type": "PEGIN-Identity-NFT",
 "owner": "{wallet_address}",
 "did": "did:chia:xyz123",
 "publicKey": "0x...",
 "credentialRoot": "{merkle_root}",
 "issuedDate": "2024-01-15",
 "recoveryKeys": ["key1", "key2"],
 "metadata": {
 "displayName": "Alice",
 "avatar": "ipfs://..."
 }
}
```

**Credential NFT structure (W3C Verifiable Credential)**

```json
{
 "@context": "https://www.w3.org/2018/credentials/v1",
 "type": ["VerifiableCredential", "KYCCredential"],
 "issuer": "did:chia:bank123",
 "issuanceDate": "2024-01-15T00:00:00Z",
 "expirationDate": "2025-01-15T00:00:00Z",
 "credentialSubject": {
 "id": "did:chia:user456",
 "kycVerified": true,
 "verificationLevel": "high"
 },
 "proof": {
 "type": "Ed25519Signature2020",
 "verificationMethod": "did:chia:bank123#key1",
 "signatureValue": "ABC123..."
 },
 "tradeable": true,
 "royaltyPercentage": 3
}
```

**DID Coin (on-chain)**

| Field | Value |
|---|---|
| Puzzle hash | Chialisp program hash (verifies ownership) |
| Amount | 1 mojo (minimal cost) |
| Parent coin | References previous DID transaction |
| DIG store ref | Pointer to credential store on DIG Network |
| Credential merkle root | Root hash of user's credential index |

**Credential lifecycle**

```
Issuance → Ownership Transfer → Storage (wallet) → Presentation → Verification
 → Trading (optional, 2–5% royalty)
 → Expiration / Revocation
```

### 6.4 Zero-Knowledge Proofs

Penguin Gateway uses zero-knowledge proofs to allow users to prove credential attributes without revealing underlying data.

**ZK implementation stack**

- Library: [circom](https://github.com/iden3/circom) (ZK circuit language) + [snarkjs](https://github.com/iden3/snarkjs) (proof generation)
- Curve: BLS12-381 (post-quantum alternatives available via lattice-based proofs)
- Performance: Proof generation < 1 second; verification < 100ms

**ZK proof types**

| Type | Purpose | Example |
|---|---|---|
| Range proofs | Prove numeric bounds | "Age ≥ 18" without revealing birth date |
| Existence proofs | Prove credential exists | "User is KYC verified" without content |
| Signature proofs | Prove issuer signed | Without exposing signature |
| Selective disclosure | Reveal only selected attributes | "Is employee" without revealing department |
| Aggregation proofs | Combine multiple credentials | One proof for KYC + employment + age |

**Example: Age verification ZK proof**

```
User has credential: ageProof = { dob: "2000-01-15" }
dApp requests: "Prove user is ≥ 18"

Penguin Gateway generates:
 - Proof that SHA256(dob + salt) = commitment
 - Range proof that age ≥ 18
 - Proof signed with user's key

dApp verifies:
 - Verify ZK proof on-chain
 - Verify commitment matches blockchain
 - Grant access

Result: dApp knows "user is 18+" but not their exact age or date of birth
```

**Cryptographic standards**

| Function | Algorithm |
|---|---|
| Signing | Ed25519 (Chia standard) |
| Hashing | SHA-256 (merkle trees, content hashing) |
| Encryption (DIG metadata) | ChaCha20-Poly1305 |
| Key derivation (passkey) | PBKDF2-SHA256 |
| ZK proofs | BLS12-381 |
| Credential storage | AES-256-GCM |

### 6.5 Recovery Mechanisms

**Email-based recovery (standard users)**

```
1. User submits "Recover Access" with email address
2. Recovery email sent via DIG email service (not a central email server)
3. User clicks time-limited link (valid 1 hour)
4. Identity verification: security questions OR trusted device approval
5. User creates new passkey/biometric auth
6. Recovery attempt logged immutably on blockchain
```

**Multi-signature recovery (enterprise)**

```
Setup: Enterprise IT configures 3-of-5 recovery administrators

Trigger: User requests recovery OR security team initiates it

Process:
 1. Recovery request broadcast to 5 approvers (via DIG network)
 2. 48-hour approval window (reversible during window)
 3. 3 of 5 approvers confirm via passkey
 4. Smart contract executes recovery on Chia blockchain
 5. Immutable record: which admins approved, when, at what IP

Result: New keys active; old keys invalidated; blockchain timestamp proves compliance
```

**DIG email service (decentralized recovery)**

Recovery emails are distributed via DIG network — not via a central email server:

- DIG email peers (privacy-focused providers like Proton Mail, Tutanota) run DIG email nodes
- No central honeypot; email not on one server
- User selects which email providers replicate their mailbox
- Anyone can run a DIG email peer (open protocol)

### 6.6 Rust Dependencies

```toml
[workspace]
members = ["pegin-core", "pegin-protocols", "pegin-contracts", "pegin-cli"]

[workspace.dependencies]
# Chia / xch-dev — https://github.com/xch-dev/chia-wallet-sdk
chia-wallet-sdk = "0.23" # DID (DidInfo), NFT (NftInfo), MerkleTree, SpendContext
chia-protocol = "0.25" # Network protocol types
chia-bls = "0.14" # BLS12-381 signatures
clvm-traits = "0.14" # CLVM type system
chia-sdk-test = "0.26" # Blockchain simulator for tests

# Authentication
passkey = "0.3" # WebAuthn / FIDO2 — https://github.com/1Password/passkey-rs
samael = "0.0.17" # SAML 2.0 — https://github.com/njaremko/samael
openidconnect = "4.0" # OpenID Connect — https://github.com/ramosbugs/openidconnect-rs
oauth2 = "5.0" # OAuth 2.0 — https://github.com/ramosbugs/oauth2-rs
jsonwebtoken = "9.3" # JWT — https://github.com/Keats/jsonwebtoken

# Cryptography
ed25519-dalek = "2.1" # https://github.com/dalek-cryptography/curve25519-dalek
sha2 = "0.10" # https://github.com/RustCrypto/hashes
chacha20poly1305 = "0.10" # https://github.com/RustCrypto/AEADs

# Web framework
axum = "0.8" # https://github.com/tokio-rs/axum
tower-http = "0.6" # https://github.com/tower-rs/tower-http
tokio = { version = "1.40", features = ["full"] } # https://tokio.rs

# Data
serde = { version = "1.0", features = ["derive"] } # https://serde.rs
serde_json = "1.0"
quick-xml = "0.37" # SAML XML — https://github.com/tafia/quick-xml
rocksdb = "0.22" # DIG L2 storage — https://github.com/rust-rocksdb/rust-rocksdb
uuid = "1.11"

# WASM
wasm-bindgen = "0.2" # https://rustwasm.github.io/wasm-bindgen
```

**Additional (Penguin Vault)**

```toml
# BIP-32 HD key derivation
bip32 = "0.5" # https://github.com/iqlusioninc/crates/tree/main/bip32

# ZK (native Rust circuits, optional alongside circom)
bellman = "0.14" # https://github.com/zkcrypto/bellman
```

### 6.7 TypeScript Dependencies

```json
{
 "dependencies": {
 "@simplewebauthn/browser": "^13.0.0", // https://simplewebauthn.dev
 "@simplewebauthn/server": "^13.0.0",
 "next": "^15.0.0", // https://nextjs.org
 "react": "^19.0.0",
 "@radix-ui/react-*": "latest", // https://www.radix-ui.com
 "tailwindcss": "^4.0.0", // https://tailwindcss.com
 "jose": "^6.0.0", // JWT — https://github.com/panva/jose
 "zod": "^3.0.0", // Schema validation
 "snarkjs": "^0.7.0", // ZK proofs — https://github.com/iden3/snarkjs
 "@penguin-gateway/sdk": "latest" // Penguin Gateway SDK
 }
}
```

### 6.8 Key External Repositories

| Repository | Language | Role | Link |
|---|---|---|---|
| xch-dev/chia-wallet-sdk | Rust | DID, NFT, coin operations — critical dependency | https://github.com/xch-dev/chia-wallet-sdk |
| xch-dev/rue | Rust | Smart contract compiler | https://github.com/xch-dev/rue |
| xch-dev/sage | Rust + TS | Reference desktop app architecture (Tauri v2) | https://github.com/xch-dev/sage |
| DIG-Network/dig-l2-storage | Rust | RocksDB-backed L2 block storage | https://github.com/DIG-Network/dig-l2-storage |
| DIG-Network/chia-block-listener | Rust | On-chain event listener | https://github.com/DIG-Network/chia-block-listener |
| DIG-Network/DataLayer-Driver | Rust | Chia Datalayer interface | https://github.com/DIG-Network/DataLayer-Driver |
| DIG-Network/slot-machine | Rust | Decentralised naming (alice.pegin) | https://github.com/DIG-Network/slot-machine |
| Chia-Network/chia_rs | Rust | Consensus, signing, protocol types | https://github.com/Chia-Network/chia_rs |
| Chia-Network/clvm_rs | Rust | CLVM virtual machine | https://github.com/Chia-Network/clvm_rs |
| iden3/circom | Rust/DSL | ZK circuit compiler | https://github.com/iden3/circom |
| iden3/snarkjs | TypeScript | ZK proof generation (browser/Node.js) | https://github.com/iden3/snarkjs |
| tauri-apps/tauri | Rust + TS | Desktop app framework (v2) | https://tauri.app |

**Key documentation sites**

| Resource | URL |
|---|---|
| Chia Network docs | https://docs.chia.net |
| DIG Network | https://dig.network |
| Rue language playground | https://www.rue-lang.org |
| chia-wallet-sdk docs | https://docs.rs/chia-wallet-sdk |
| WebAuthn guide | https://webauthn.guide |
| W3C DID spec | https://www.w3.org/TR/did-core/ |
| W3C Verifiable Credentials | https://www.w3.org/TR/vc-data-model/ |
| Passkey overview (FIDO) | https://fidoalliance.org/passkeys/ |
| Axum docs | https://docs.rs/axum |
| Tokio docs | https://tokio.rs/tokio/tutorial |
| Tauri docs | https://tauri.app/v2/guide/ |

**Performance targets**

| Operation | Target |
|---|---|
| Login latency | < 200ms (p99) |
| DID registration | 1 Chia block (10–20 seconds) |
| Credential issuance | 1–2 Chia blocks (20–40 seconds) |
| Credential verification | < 100ms (smart contract) |
| ZK proof generation | < 1 second |
| ZK proof verification | < 100ms |
| Permission DIG query | < 50ms (cached) |
| Bulk provisioning (10K users) | < 5 seconds (1 blockchain transaction) |

### 6.9 Developer documentation (canonical)

Use **`docs/08-developer/`** for implementation truth. This wiki section is a summary.

| Path | Document |
|------|----------|
| [08-developer/README.md](../08-developer/README.md) | Developer hub and build path |
| [specs/tech-stack.md](../04-technical/specs/tech-stack.md) | **Spec 1** — stack, POC, official links |
| [specs/enterprise-identity-spec.md](../04-technical/specs/enterprise-identity-spec.md) | **Spec 2** — OIDC, SAML, SCIM, Entra |
| [integration/sdk-guide.md](../08-developer/integration/sdk-guide.md) | SDK and app integration |
| [architecture/on-chain-architecture.md](../10-architecture/on-chain-architecture.md) | On-chain model |
| [architecture/dig-incentives-integration.md](../10-architecture/dig-incentives-integration.md) | DIG incentives |
| [architecture/dig-enterprise-transformation.md](../10-architecture/dig-enterprise-transformation.md) | Enterprise + DIG |
| [permissions/permission-data-model.md](../10-architecture/permission-data-model.md) | PePP (Phase 2) |
| [products/gateway-architecture.md](../10-architecture/products/gateway-architecture.md) | Penguin Gateway |
| [products/vault-architecture.md](../10-architecture/products/vault-architecture.md) | Penguin Vault |

---

## 7. DIG Network

### What DIG is

DIG Network is a **paid decentralised storage layer** built on Chia blockchain. Unlike IPFS — where peers serve files for free and eventually stop because there is no revenue — DIG pays peers in XCH automatically via on-chain smart contracts. Peers publish cryptographic "Proof of Living Storage" each epoch (weekly); the reward distributor contract verifies the proof and pays proportionally.

PEGIN uses DIG as its data layer. User profiles, permission grants, **audit logs (append-only)**, and session state live on DIG — not on PEGIN's servers. Chia records **anchors** (e.g. Merkle roots) when stores change; heavy payloads are not written on chain. If PEGIN Inc. closes, peers can continue serving replicated DIG data per network rules.

### Architecture: PEGIN founder is just a DIG peer

Unlike traditional SaaS, PEGIN has no central servers:

```
What PEGIN does NOT operate:
├─ No central database (user data on DIG — not PostgreSQL)
├─ No central API server (dApps query DIG/blockchain directly)
├─ No central auth server (verification on-chain via smart contracts)
├─ No central email server (recovery via DIG email service)
└─ No audit logging system (logs on DIG — immutable)

What PEGIN DOES operate:
├─ DIG peer node (like running a Bitcoin node — for storage)
├─ Optional indexer (for faster queries; community can fork)
├─ SDKs and documentation (open-source)
└─ Token/governance stewardship
```

This means the cost structure is fundamentally different from traditional SaaS:

```
Traditional SaaS (Auth0):
 Scale up → costs scale up → must raise prices → customers leave

PEGIN peer model:
 Scale up → costs stay same (DIG replicates across peers)
 Customers leave → still get paid for storage they used
```

### DIG store structure

Each PEGIN user gets an encrypted DIG store:

| Store | Contents | Encrypted |
|---|---|---|
| User credentials | DIDs, issued credentials, NFTs | With user's key — only user can decrypt |
| Audit logs | Every auth event, recovery attempt, custody action | Immutable, distributed across peers |
| Recovery state | Email recovery tokens, multi-sig approval states | Time-limited, replicated |
| Vault metadata | Key custody info, recovery contacts (Penguin Vault) | Encrypted |

### Storage economics

- Data volume: ~1 TB

### How incentives work

```
Customer sets incentive: "0.1 XCH/week to store our identity data"
 → DIG Network broadcasts on-chain
 → Peers compete to store it
 → Multiple peers replicate (geographic redundancy, no extra cost)
 → Each epoch: smart contract verifies Proof of Living Storage
 → Pays peers proportionally from escrow
 → If PEGIN goes offline: peers keep earning → keep serving data
```

### The network flywheel

```
More PEGIN customers
 → more identity data on DIG Network
 → larger on-chain incentive pool
 → more DIG peers join (profitability)
 → better geographic redundancy and latency
 → better PEGIN service quality
 → more customers adopt PEGIN
 → DIG ecosystem grows → XCH demand rises
 → founder, customers, and peers all benefit
 → [loop repeats, compounding]
```

### Participant summary

| Participant | What they do | What they earn |
|---|---|---|
| Community peer operator | Runs DIG node on rented servers | Proportional XCH from incentive pool |
| Hosting provider | Runs peers on spare rack capacity | XCH income + PEGIN reseller revenue |
| PEGIN developers | Hold tokens + run optional DIG peer | Token appreciation + optional XCH peer income |

---

## 8. Business Model

### 8.1 Revenue Streams

PEGIN charges zero per-user licensing. Revenue comes from five participation-aligned streams:

| Stream | Mechanism | Year 5 projection |
|---|---|---|

**The Linux / Red Hat model**

The right analogy is not Okta (per-user extraction) — it is Linux / Red Hat:

```
Linux kernel: Open-source, free, no revenue

PEGIN protocol: Open-source, free (like Linux kernel)
PEGIN Enterprise: Managed service, compliance, support (like Red Hat)
PEGIN Vault (Phase 2): Premium custody service
PEGIN Token: Ownership stake in ecosystem (like stock in Red Hat early days)
```

The wrong model: charge per API call (death spiral — customer scales → you pay more to serve them → raise prices → customers migrate → start over).

The right model: charge fixed annual subscription. Customer scales from 1,000 to 50,000 users — cost to you stays the same. Customer happy (their cost didn't change). They renew and buy vault. Word of mouth accelerates.

### 8.2 Financial Projections

**Five-year model**

| Year | Customers | Revenue | Costs | Net | Token value |
|---|---|---|---|---|---|

Year 2 is the self-funding threshold. By Year 4, founder can step into advisory role. By Year 5, PEGIN Foundation governs development.

**Revenue by product (Year 3)**

| Product | Revenue |
|---|---|

**Penguin Gateway standalone (Year 3)**

| Stream | Revenue |
|---|---|

**Penguin Vault standalone (Year 3)**

| Stream | Revenue |
|---|---|

**User and network growth**

| Metric | Year 1 | Year 2 | Year 3 | Year 5 |
|---|---|---|---|---|
| Enterprise customers | 5 | 20 | 80 | 400 |
| Total PEGIN users | 10K | 100K | 800K | 12M |
| Credential issuers | 10 | 100 | 500 | 2,000+ |
| DIG peers running PEGIN | 1 | 5 | 20 | 100+ |

**Funding plan**

| Round | Amount | Purpose |
|---|---|---|
| Profitability | Year 3 | Positive unit economics |

### 8.3 Enterprise Pricing

**SLA tiers**

| Tier | Uptime | Support | Annual Price |
|---|---|---|---|

**Pricing examples (annual)**

| Company size | Users | Annual cost | vs. Azure AD |
|---|---|---|---|

**Deployment options**

| Model | Description | Cost |
|---|---|---|
| Hybrid | SaaS for staging + private cloud for production | Negotiated |

**Professional services**

**Compliance certifications supported**

SOC2 Type II · ISO 27001 · GDPR/CCPA · HIPAA · FedRAMP · PCI DSS · Blockchain audit trail

### 8.4 Token Economics

| Parameter | Value |
|---|---|
| Total supply | 100M PEGIN tokens |
| Founder / team | 20M (20%) — 4-year vest |
| Adoption incentives | 20M (20%) — early issuers, developers, community |
| Community / governance | 60M (60%) |
| Governance rights | 1 PEGIN = 1 vote on protocol changes |

**Token value drivers**

- Governance rights (holders vote on protocol changes)
- Staking rewards (optional passive income for holders)
- Transaction fees paid in PEGIN tokens (velocity)
- Fixed supply (100M cap)

**Early issuer rewards**

- First 50 issuers: 10,000 PEGIN tokens (free)
- First issuers: 5% revenue share on credential trading (vs 1–3% later)

**Founder wealth trajectory**

| Year | Source | Value |
|---|---|---|

**Succession (Year 4+)**

```
Year 3-4: Establish PEGIN Foundation (like Linux Foundation)
 Foundation owns governance token treasury
 Community-elected board

Year 4: Transfer governance to token holders
 Recruit community leader from Chia ecosystem
 Encourage other companies to run PEGIN peers

Year 5+: Founder becomes contributor, not leader
 Community maintains code
 Passive income continues forever
```

---

## 9. Competitive Position

### Head-to-head

| Dimension | Okta | Azure AD | Citrix | PEGIN |
|---|---|---|---|---|
| Lock-in | Contract lock-in | Microsoft ecosystem | Enterprise contracts | **None (fork anytime)** |
| Customer upside | None | None | None | **DIG peer income + tokens** |
| Permission speed | 3–7 days | 3–7 days | 3–7 days | **< 2 minutes** |
| Offboarding | 3–7 day risk window | 3–7 day risk window | 3–7 day risk window | **< 1 second** |
| Audit trail | Editable logs | Editable logs | Editable logs | **DIG audit + chain anchors** |
| Vendor risk | Company controls you | Microsoft controls you | Private equity controls you | **Open source; code lives forever** |
| Identity ownership | Vendor owns it | Vendor owns it | Vendor owns it | **User owns it** |
| Survives vendor failure | No | No | No | **Yes (blockchain is permanent)** |

### Why competitors cannot copy this

Okta, Azure AD, and Citrix generate 80–90% of their revenue from per-user licensing. Going free would destroy their revenue model overnight:

```
Okta → go free → revenue collapses → stock halted → insolvency within a quarter
Azure → go free → Microsoft loses tens of billions → shareholders revolt
PEGIN → born free → designed around participation → no conflict to resolve
```

Their moat is built on extraction. PEGIN's moat is built on participation. A company cannot hold both simultaneously.

### The pull model

Okta must be sold. A sales rep identifies a prospect, navigates procurement, and persuades a budget holder to sign. The customer is a reluctant buyer who resents annual invoices and wants to escape the moment a cheaper option appears.

PEGIN is pulled. A developer hears "it's free, open source, and you can earn money from it." Their first question is when they can start. They introduce it to their manager, who sees pilot-measured value (not a generic savings claim). No sales rep was involved.

Customers who stay because they are making money — from DIG peer income and token appreciation — do not churn. The retention dynamic is structurally different from subscription SaaS.

---

## 10. Development Roadmap

### Phase 0 — Environment setup (Weeks 1–2)

```
□ Create Rust workspace: pegin-core, pegin-protocols, pegin-contracts, pegin-cli
□ Create TypeScript workspace: @pegin/sdk, pegin-dashboard
□ cargo install rue-cli — Rue smart contract compiler
□ Configure rue-vscode IDE extension
□ Deploy Chia testnet node (chia-blockchain)
□ Deploy DIG testnet peer via chia-dig-node Docker compose
□ Study xch-dev/sage for Tauri v2 architecture reference
□ Study xch-dev/docs for chia-wallet-sdk DID and NFT examples
□ Set up circom + snarkjs for ZK circuit development
```

### Phase 1 — POC "Login with PEGIN" (Weeks 3–8)

**POC MVP goal**: one thing normal SSO cannot do — *identity that survives the company that issued it*.

Demo script: User registers → Company A issues credential (signs to DID) → Company A goes bankrupt (server off) → User still has credential on blockchain → New company verifies it. Proof: identity survived company death.

**Rust (pegin-core)**
```
□ DID creation via chia-wallet-sdk DidInfo
□ WebAuthn registration and login via passkey crate
□ Link passkey credential ID to Chia DID on first registration
□ JWT issuance after successful passkey verification
□ OIDC /.well-known/openid-configuration endpoint
□ Audit log writes to DIG store
□ Axum HTTP server with WebAuthn RP endpoints
```

**Contracts (pegin-contracts)**
```
□ pegin_did.rue — DID registration and update
□ pegin_credential.rue — credential NFT issuance with royalty puzzle
□ pegin_issuer.rue — issuer registration (0.1–1 XCH minting fee)
□ Compile with rue-cli, test with chia-sdk-test simulator
```

**TypeScript (@pegin/sdk)**
```
□ <PeginLogin /> React component (redirect + callback handling)
□ @simplewebauthn/browser passkey challenge flow
□ JWT session management and storage
□ Demo website (pegin-demo) — end-to-end login demo
```

**POC success criteria**
- Passkey registration: < 5 seconds
- Passkey login: < 1 second
- DID anchored on Chia testnet
- Works across Chrome, Safari, Firefox
- Zero passwords, zero seed phrases in the UX
- Credential survives "issuer server shutdown" demo

### Phase 2 — Enterprise protocols (Weeks 9–14)

```
□ OIDC Provider (openidconnect crate)
□ SAML 2.0 IdP (samael crate)
□ OAuth 2.0 authorisation server
□ SCIM 2.0 provisioning endpoint
□ Bulk user provisioning via chia-wallet-sdk MerkleTree
□ Azure AD SAML federation test
□ Azure AD Connector (bidirectional sync)
□ Hardware passkey support (YubiKey / FIDO2 security keys)
□ Email-based account recovery (via DIG email service)
□ Multi-sig recovery via Chia Signer
□ DIG Network permission caching (< 50ms)
□ pegin_recovery.rue — timelocked multi-sig contract
```

### Phase 3 — Permission Platform (Weeks 15–20)

```
□ Capability token engine (PePP core)
□ Mobile app: push notification → manager approval flow
□ Multi-sig approval for high-risk access
□ Real-time revocation via webhooks
□ App-defined JSON permission schema
□ LDAP gateway (translate queries to DIG lookups)
□ Group management (group NFTs)
□ RBAC via credential NFT role attributes
□ Conditional access via Rue smart contract conditions
□ Tauri v2 desktop admin dashboard
□ Compliance audit report export from DIG logs
□ Security audit of all Rue contracts
```

### Phase 4 — Token, Vault, and business layer (Weeks 21–24+)

```
□ PEGIN CAT governance token (Rue contract)
□ Credential NFT royalty puzzle (2–5% on trades)
□ Issuer registration contract (0.1–1 XCH minting fee)
□ DIG peer with storage fee collection
□ Token distribution via reward-distributor-clsp
□ Naming system via slot-machine (alice.pegin)
□ Testnet beta launch

Penguin Vault (Phase 4 parallel):
□ pegin_vault.rue — timelocked rekey contract (48-hour window)
□ Personal Vault + App-Specific Child Vaults + Master Vault
□ Custody provider integrations (Fireblocks API, Chia Signer multi-sig)
□ Biometric custody verification flow
□ ZK proofs for selective disclosure (circom circuits)
□ SDK: npm install @penguin-vault/sdk + pip install penguin-vault

Penguin Gateway (Phase 4 parallel):
□ ZK proof engine (circom + snarkjs)
□ Age/range/existence/aggregation proof circuits
□ Credential marketplace with issuer registry
□ dApp portal for discovering integrated apps
□ Cross-chain identity (Ethereum, Solana bridge)
□ SDK: npm install @penguin-gateway/sdk
```

---

## 11. Integrations

### Notbot — humanity verification layer

[Notbot](https://notbot.io) solves a different problem from PEGIN and the two are designed to compose.

| Question | Notbot | PEGIN | Together |
|---|:---:|:---:|:---:|
| Are you human (not a bot or deepfake)? | ✅ | — | ✅ |
| Who are you (verified identity)? | — | ✅ | ✅ |
| What can you access (permissions)? | — | ✅ | ✅ |
| Prevent content forgery / deepfakes? | ✅ | — | ✅ |
| Enterprise SSO? | — | ✅ | ✅ |

Notbot verifies that a credential holder is a real human through passport-based verification and issues a cryptographic "humanity proof" sticker. PEGIN verifies *which* human they are and *what* they are allowed to do.

Combined authentication chain: "Real human [Alice] + verified [Company A] + can access [GitHub]."

```
Layer 1: Notbot → "You are a real human"
Layer 2: PEGIN → "You are Alice from Company A"
Layer 3: PePP → "Alice can push code to this repo"
```

The PEGIN SDK provides a `requireHumanVerification()` middleware that enforces a valid Notbot sticker alongside the PEGIN session for routes that require it (high-value transactions, regulatory approvals, content publishing).

**Integration phases**

| Phase | Timeline | Goal |
|---|---|---|
| Phase 1: Loose integration | Weeks 1–4 | PEGIN DID references Notbot verification; no code changes |
| Phase 2: API integration | Weeks 5–12 | PEGIN login optionally requires Notbot; combined audit logs on DIG |
| Phase 3: Enterprise suite | Weeks 13–24 | Joint product; "deepfake-proof enterprise authentication" positioning |

### Fireblocks — institutional custody

When enterprises using PEGIN need custody for crypto keys or high-value assets, PEGIN refers them to Fireblocks (and similar providers: Copper, Anchorage). PEGIN earns 10–20% referral fee.

PEGIN acts as the orchestration and audit layer — never holding keys. All recovery approvals are logged immutably on DIG.

### Chia ecosystem

PEGIN proposes its DID standard through the CHIPs (Chia Improvement Proposals) process for ecosystem interoperability. More PEGIN adoption → more DIG usage → more XCH on-chain activity → aligned incentives.

---

## 12. Strategic Context

### Position within the Chia ecosystem

Chia Network Inc. focuses on regulated financial products (Permuto Capital, One Market, DataMkt). Their identity layer, when built, will target KYC/AML for financial instruments — proving that someone is an eligible investor.

PEGIN targets enterprise employee identity — a structurally different market:

| | Chia Network Inc. | PEGIN |
|---|---|---|
| Identity focus | Investor / KYC identity | Employee and user identity |
| Core question | "Are you an eligible investor?" | "Who are you, and what can you access?" |
| Issuer model | Regulated financial entities | Any organisation |
| Relationship | Complementary | Complementary |

### Four founding principles

1. **Alignment of incentives** — Revenue comes from network participation, not per-user extraction. The business can only succeed when the network creates real value for customers.

2. **Fixed infrastructure, variable revenue** — A DIG peer costs roughly the same to operate whether PEGIN has 100 customers or 100,000. Marginal cost per new customer approaches zero; margins improve with scale.

3. **Credible founder exit** — Founder wealth comes from token appreciation and DIG peer income — neither requires the founder to remain employed. The system is designed to outlive its creator.

4. **Unique leverage** — Only PEGIN can offer identity that survives the company that issued it. Every centralised vendor's business model *requires* them to remain indispensable. PEGIN's business model *requires* being dispensable. This is a structural moat that features and pricing cannot replicate.

### Why this model avoids the "greed trap"

```
Greedy model (death spiral):
 Charge per API call like Auth0
 Customer scales → they pay 10x more
 They get angry → migrate to competitor
 You constantly need new customers to replace churn
 Eventually acquisition only at low valuation

PEGIN model (network effects):
 Customer scales → you pay the same to serve them
 Customer happy (costs stayed same, they scaled) → renew + buy Vault
 Happy customer tells peers in same industry
 Eventually you're "the standard" for enterprise auth
 
KEY INSIGHT: You make money by SUCCEEDING, not by EXTRACTING.
```

---

## 13. Branding

**Full name:** PEGIN — Penguin Gateway Identity

**Products**
- PEGIN SSO — enterprise login replacing Okta / Azure AD
- PePP — PEGIN Permission Platform replacing Citrix + AD
- Penguin Gateway — decentralized Web3 SSO for dApps
- Penguin Vault — decentralized key custody with timelocked recovery

**Taglines**
- "Waddle in, authenticated out."
- "Your identity. Your blockchain. Your rules."
- "Identity infrastructure that outlives its creator."
- "No passwords. No servers. No problems."
- "Passwords and seed phrases go bye bye."

**Hashtags:** `#pegin` `#dsso` `#decentralizedSSO` `#nopasswords` `#chianetwork` `#dignetwork` `#waddlein`

**Key numbers for any conversation**

| | |
|---|---|
| Pilot ROI | Customer-specific; measure per engagement |
| < 2 min | Permission approval (vs. 3–7 days) |
| < 1 sec | Employee offboarding (vs. 3–7 day risk window) |
| 8 weeks | POC timeline (2–3 developers) |
| 24 weeks | MVP to full v1.0 |

---

## 14. Canonical markdown index

| Folder | Key files |
|--------|-----------|
| **08-developer/** | [README](../08-developer/README.md), [tech-stack](../04-technical/specs/tech-stack.md), [sdk-guide](../08-developer/integration/sdk-guide.md) |
| **03-use-cases/** | [roadmap](../03-use-cases/roadmap.md), [mvp-strategy](../03-use-cases/mvp-strategy.md), [differentiators](../03-use-cases/differentiators.md) |
| **02-product/** | [permission-platform](../02-product/permission-platform.md), [complete-ecosystem](../02-product/complete-ecosystem.md) |
| **01-vision/** | [pegin-manifest](../01-vision/pegin-manifest.md), [fully-decentralized](../01-vision/fully-decentralized.md), [core-value](../01-vision/core-value-user-owned-login.md) |
| **05-business/** | [competitive-moat](../05-business/competitive-moat.md), [enterprise-business-plan](../05-business/enterprise-business-plan.md), [sustainable-funding](../05-business/sustainable-funding.md) |
| **06-strategy/** | [chia-ecosystem-position](../06-strategy/chia-ecosystem-position.md), [notbot-integration](../06-strategy/notbot-integration.md) |
| **07-penguin-products/** | [gateway](../07-penguin-products/gateway-business-plan.md), [vault](../07-penguin-products/vault-business-plan.md) |
| **wiki/** | [README](README.md), [HTML wiki](PEGIN_Wiki_Knowledge_Base.html) |

---

*PEGIN — Free software. Aligned incentives. Identity that outlives its creator.*

*v5.0 — May 2026 — Wiki overview; canonical technical docs in `docs/08-developer/`*