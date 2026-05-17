# PEGIN — Fully Decentralized

*Enterprise SSO + custody on Chia and DIG (no central servers)*

Version 3.0 · May 2026 · Designed for succession — the system runs without the founder.

---

## Table of contents

- [1. Executive summary](#1-executive-summary)
- [2. Architecture: DIG as data layer](#2-architecture-dig-as-data-layer)
- [3. No central servers — operator as DIG peer](#3-no-central-servers--operator-as-dig-peer)
- [4. User data and logs on DIG](#4-user-data-and-logs-on-dig)
- [5. Email recovery (federated → DIG)](#5-email-recovery-federated--dig)
- [6. Multi-signature custody (external providers)](#6-multi-signature-custody-external-providers)
- [7. Revenue model (DIG peer economics)](#7-revenue-model-dig-peer-economics)
- [8. Custody service progression](#8-custody-service-progression)
- [9. Designed for succession](#9-designed-for-succession)
- [10. Go-to-market and adoption](#10-go-to-market-and-adoption)
- [11. Outcomes to validate (not financial projections)](#11-outcomes-to-validate-not-financial-projections)
- [12. Exit: hand off and move on](#12-exit-hand-off-and-move-on)

---

## 1. Executive summary

PEGIN v3 is a **fully decentralized SSO + custody coordination** model: user data on the **DIG network**, identity on **Chia**, recovery via **federated email → DIG mail**, and keys with **external custodians**—not on a single company’s servers.

**Critical principle:** If the founder or one operator disappears, the protocol, data on other DIG peers, and custody at regulated providers can continue.

| Theme | Direction |
|-------|-----------|
| **Data** | Encrypted stores on DIG; operator runs one peer among many |
| **Auth** | Passkey / Chia Signer; verification on chain |
| **Recovery** | Federated privacy-first email now; DIG federated mail later (§5) |
| **Custody** | Orchestration only; banks / Fireblocks-class holders keep keys (§6) |
| **Revenue** | DIG peer services, indexing, referrals, services — not per-seat API tax (§7) |

**Revenue hypothesis (no amounts):** Operators earn by being useful on the network—storage and bandwidth on a DIG peer, optional indexer/query services, custody **referrals** (negotiated terms), ecosystem sponsorship, and professional/SLA services. See [§7.4](#74-revenue-streams-where-money-comes-from) and [sustainable-funding.md](../05-business/sustainable-funding.md).

---

## 2. Architecture: DIG as data layer

PEGIN uses **Chia** for identity anchoring and **DIG** for user data and audit logs.

### 2.1 Three layers

| Layer | Role |
|-------|------|
| **Chia blockchain** | DIDs, issuer NFTs, recovery/rotation rules; **commitments** when DIG stores change |
| **DIG network** | Encrypted profiles, **audit logs**, grants, credential metadata; user-controlled access |
| **DIG peers** | Operators (including you) replicate stores; users choose peers |

**No heavy data on chain.** Audit events and permission payloads live on DIG. Chia records **anchors** (e.g. Merkle root / hash of store head) on update — not log bodies or session dumps.

### 2.2 Data flow (no central server)

**User creates identity**

1. DID on Chia (public anchor).
2. Encrypted profile and metadata on DIG (user-chosen peers).
3. Only the user decrypts content.

**Enterprise uses PEGIN**

1. Employees authenticate (passkey / Chia Signer).
2. Auth events append to DIG audit stores (replicated); store updates anchored on Chia.
3. Operators may earn from storage/bandwidth on their peer (§7).

**Email recovery (phased)**

1. Phase 1: magic link via **SMTP** to a privacy-first inbox (Proton, Tutanota, etc.).
2. Recovery session on DIG; DID rotation on Chia.
3. Phase 2: **DIG federated mail** peers for delivery (open spec).

---

## 3. No central servers — operator as DIG peer

Unlike traditional SaaS, the model avoids a single operator-owned control plane for identity verification.

### 3.1 What the operator does not run

| Component | Where it lives |
|-----------|----------------|
| Central user database | DIG (distributed) |
| Mandatory auth API for verification | On-chain + client/SDK |
| Sole recovery mailbox | Federated SMTP / future DIG mail (§5) |
| Central audit silo | DIG append-only stores |

### 3.2 What the operator may run

- **DIG peer** — storage and replication node.
- **Optional indexer** — faster credential lookup for dApps (community can run alternates).
- **Open-source SDK and docs** — integration surface.
- **Governance participation** — if a token layer is pursued (legal review required).

### 3.3 Cost structure (hypothesis)

Centralized IdPs scale cost with tenants, regions, and compliance surface. The PEGIN hypothesis is that **protocol + peer** economics differ—but operator engineering, bandwidth, and support are **not zero**.

**Validate:** operator P&L per deployment before quoting customers.

---

## 4. User data and logs on DIG

All user data and audit logs target DIG replication, not one private Postgres.

### 4.1 DIG store types

| Store | Contents |
|-------|----------|
| **Credentials index** | DID references, credential metadata (encrypted) |
| **Audit logs** | Auth, recovery, custody orchestration events |
| **Recovery state** | Sessions, timelocks, approval progress |
| **Vault metadata** | Recovery contacts, multisig policy (encrypted) |

### 4.2 Redundancy

- Replication across multiple DIG peers (user- or policy-selected).
- Fees flow per network/operator rules — **amounts TBD** in pilots.
- If one operator stops, data may persist on other peers per replication policy.

### 4.3 Privacy

- End-to-end encryption from the user device.
- Peer operators host ciphertext; they do not hold decryption keys.
- Deletion / export policies must match GDPR-style requirements in product design.

### 4.4 Audit integrity (DIG + chain anchor)

1. **Write** — append event to DIG audit store (grant, revoke, login, recovery step).
2. **Anchor** — on store update, record commitment on Chia (Merkle root / hash of store head).
3. **Verify** — auditor or SIEM exports from DIG and checks against anchored history.

PePP grants and permission audit trails follow this pattern; see [permission-data-model.md](../10-architecture/permission-data-model.md).

---

## 5. Email recovery (federated → DIG)

Recovery splits **delivery** (how the user gets a link) and **authorization** (who may bind a new passkey to the DID).

| Layer | Phase 1 (ship first) | Phase 2 (target) |
|-------|----------------------|------------------|
| **Delivery** | **SMTP federation** → privacy-first inbox | **DIG federated email recovery** |
| **Authorization** | Recovery session on DIG + on-chain rotation | Same |
| **Operator** | Any compatible node; no single `recovery@pegin.com` | Plus DIG mail peers |

> Phase 1 uses existing federated email; Phase 2 is an open DIG mail spec—not claimed as live today.

### 5.1 Phase 1 — Privacy-first federated email

| Provider (examples) | Role |
|---------------------|------|
| [Proton Mail](https://proton.me/mail) | Recovery inbox |
| [Tutanota](https://tutanota.com/) | Recovery inbox |
| [Mailbox.org](https://mailbox.org/) · [Posteo](https://posteo.de/) | EU privacy-oriented hosts |

**Flow:** register recovery email → lockout → recovery session on DIG → SMTP magic link → verify → optional timelock/guardian → new passkey → DID rotation on Chia → recovery events on DIG, store updates anchored on Chia.

### 5.2 Phase 2 — DIG federated email recovery

Publish a **DIG Recovery Mail** spec; pilot with a privacy-first provider or community peer; enterprise option **SMTP / DIG / both**; keep SMTP as fallback.

### 5.3 Recovery flow (both phases)

1. Request → 2. Session on DIG → 3. Deliver (SMTP or DIG mail) → 4. Verify → 5. Strengthen (timelock / guardian / multisig) → 6. Rotate passkey on chain → 7. Audit.

Prefer **backup passkey or guardian approval** over security questions.

### 5.4 Related paths

- **Enterprise:** multisig may replace email (§6).
- **Docs:** [enterprise-business-plan.md](../05-business/enterprise-business-plan.md), [roadmap.md](../03-use-cases/roadmap.md).

---

## 6. Multi-signature custody (external providers)

PEGIN does not hold customer keys; it **orchestrates** recovery with external custodians.

### 6.1 Provider types

| Type | Examples | PEGIN role |
|------|----------|------------|
| Institutional custody | Fireblocks, Copper, Anchorage | SSO + recovery orchestration |
| Banking custody | Regulated bank programs | Integration + audit on DIG |
| Self-custody | Chia Signer, 2-of-3 with trusted contacts | Coordinate approvals |
| Hardware | Ledger, Trezor | Recovery workflow, not key storage |

### 6.2 Operator role

- Never hold live private keys for customers.
- Coordinate multisig approvals and log actions on DIG.
- **Referral / partnership revenue** — terms **negotiated per custodian**, not fixed in protocol docs.

### 6.3 Enterprise recovery example

**Setup:** custody provider + M-of-N approvers + PEGIN integration.

**Recovery:** employee lockout → approval requests on DIG → threshold approvers → custodian executes key rotation → immutable audit trail.

---

## 7. Revenue model (DIG peer economics)

Revenue comes from **network usefulness and services**, not from renting a mandatory login API.

Sections [7.1–7.3](#71-primary-dig-peer-services) summarize categories; **[§7.4](#74-revenue-streams-where-money-comes-from)** is the canonical map of **where money could come from**.

### 7.1 Primary: DIG peer services

| Service | Description | Payer (typical) |
|---------|-------------|-----------------|
| **Storage** | Host encrypted user/tenant data on your peer | Enterprise, operator |
| **Bandwidth / egress** | Serve reads and replication | Same |
| **Redundancy / SLA** | Backup peer or uptime commitment | Enterprise compliance |

Pricing model **TBD** per pilot (per-GB, flat SLA, or hybrid).

### 7.2 Secondary: indexing and enterprise add-ons

| Service | Description | Payer (typical) |
|---------|-------------|-----------------|
| **PEGIN indexer** | Faster credential/DID queries for dApps | Integrators |
| **Analytics (anonymized)** | Usage or identity trend reports | Security / product teams |
| **Compliance packs** | Reports from DIG audit logs | GRC / audit |

### 7.3 Tertiary: partnerships

| Stream | Description |
|--------|-------------|
| **Custody referrals** | Introduce enterprise to regulated custodian |
| **Ecosystem sponsorship** | Chia / wallet ecosystem grants |
| **Bank or SI co-sell** | Host DIG + PEGIN for bank customers |

All partnership economics are **contract-specific**.

### 7.4 Revenue streams (where money comes from)

This table is the **source-of-revenue map** for a DIG-peer operator model. After pilots, record which rows produced revenue in . Broader policy: [sustainable-funding.md](../05-business/sustainable-funding.md).

| # | Stream | Where money comes from | Who typically pays | Phase / notes |
|---|--------|------------------------|--------------------|---------------|
| 1 | DIG peer storage | Hosting tenant/user encrypted stores | Enterprise, heavy users | Early → ongoing |
| 2 | Bandwidth & replication | Egress and sync from your peer | Same | Scales with usage |
| 3 | Redundancy / SLA | Premium peer or uptime guarantee | Regulated enterprises | After reliability proven |
| 4 | Indexer & query | Optional fast lookup vs raw chain/DIG | dApp teams | Post-POC |
| 5 | Analytics & reporting | Anonymized insights or compliance exports | Security / GRC | Enterprise only |
| 6 | Custody referrals | Fee for qualified custodian intro | Custody partner | When enterprise needs keys off-PEGIN |
| 7 | Ecosystem sponsorship | Grants for adoption/reference deploy | Foundations | Opportunistic |
| 8 | Bank / SI partnerships | Co-sell hosting + integration | Financial institution | Negotiated |
| 9 | Professional services | Migration, integration, security review | Per project | Common in early enterprise |
| 10 | Enterprise SLA & support | Named support, on-call, runbooks | Flat engagement | Operator choice |
| 11 | Network incentives | DIG/Chia peer or protocol incentives | Network rules | Separate from SaaS P&L |
| 12 | Token / governance | Alignment (if legal) | Treasury, community | **Not** assumed as operating cash |

#### What is not the default model

- Per-seat login API tax (centralized IdP model).
- Selling identity data — operator stores ciphertext, not a directory monopoly.
- Holding customer keys — custodians hold material; PEGIN orchestrates (§6).

#### How streams stack (qualitative)

| Stage | Likely revenue mix |
|-------|-------------------|
| **Pilot** | Professional services + one peer/storage deal |
| **Early production** | Storage/bandwidth + indexer for integrators |
| **Enterprise** | SLA + compliance packs + custody referrals |
| **Mature protocol** | Multiple peers compete; services differentiate operators |

#### Before quoting any customer

| Question | Why |
|----------|-----|
| Cost to run one peer (people + infra + bandwidth) | Floor for pricing |
| First stream that actually paid | Focus product, not slide decks |
| Referral terms per custodian | No default “protocol %” |
| Operating revenue vs token/XCH exposure | Separate P&L from balance sheet |

---

## 8. Custody service progression

PEGIN starts as SSO; custody depth increases over product phases (see [roadmap.md](../03-use-cases/roadmap.md)).

| Phase | Focus |
|-------|--------|
| **1 — SSO** | Login, DIG data, federated email recovery, external multisig hooks |
| **2 — Custody lite** | Recovery orchestration across providers; compliance reporting |
| **3 — Vault** | Encrypted recovery backups (not live keys); policy on chain |
| **4 — Distributed custody** | Community recovery nodes; optional peer operators |

**Why external custodians first:** licenses, insurance, and compliance already exist; PEGIN stays orchestrator.

---

## 9. Designed for succession

**Goal:** PEGIN continues without the founder.

### 9.1 What persists without one operator

| Asset | Why it survives |
|-------|-----------------|
| Open-source code | Community can maintain and fork |
| DIG replication | Data on other peers |
| Smart contracts | Immutable verification rules on Chia |
| Email recovery | Federated SMTP now; DIG mail later — no single mail API |
| External custody | Keys remain with custodians |
| Multiple operators | Anyone can run a peer + indexer |

### 9.2 Transition (governance)

1. Foundation or community council for governance (if token layer exists).
2. Roadmap by open process.
3. Encourage multiple PEGIN/DIG peers.
4. Transfer or community-run operator peer.
5. Founder moves to contributor role — not sole decision-maker.

### 9.3 After handoff (qualitative)

- **Operating income** may continue from peer fees, referrals, and contracts already signed — **magnitude not stated here**.
- **Token holdings** (if any) are speculative until legal plan and market exist.
- **Personal outcome** is independence from day-to-day ops, not a projected wealth figure in documentation.

---

## 10. Go-to-market and adoption

### 10.1 Early adopters (hypothesis)

| Segment | Why |
|---------|-----|
| Chia ecosystem | Natural DID/passkey audience |
| Internal “Penguin” apps | Dogfood before external GTM |
| Web3 / DeFi | Need wallet-native identity |
| Progressive enterprise | Fintech, crypto-adjacent IT |

### 10.2 Tiering (qualitative — no list prices)

| Tier | Intent |
|------|--------|
| **Community / dev** | Free core protocol; testnet; docs |
| **Production operator** | Paid peer hosting, indexer, or SLA |
| **Enterprise** | Custody integration, compliance, multisig, named support |

Which tier customers pay for is a **pilot learning**, not a doc assumption.

---

## 11. Outcomes to validate (not financial projections)

This section replaces legacy “financial projections.” Do not publish ARR, TAM, or user targets until measured.

### 11.1 Adoption signals to track

| Signal | How you might measure |
|--------|------------------------|
| Integrations | Apps shipping “Login with PEGIN” |
| Peer utilization | Storage/bandwidth on operator peer |
| Recovery success | Time to complete federated email recovery |
| Enterprise pilots | Signed LOI or paid POC |

### 11.2 Assumptions to prove or disprove

| Assumption | Risk if wrong |
|------------|----------------|
| DIG network mature enough for production audit stores | Delay PePP / enterprise |
| Enterprises will pay for peer + SLA vs self-host only | No operator revenue |
| Custody referrals close with real contracts | No partnership stream |
| Chia ecosystem growth helps adoption | Slower integrator interest |
| PEGIN wins on model, not on unproven “cheaper than Okta” slides | Wrong GTM |

---

## 12. Exit: hand off and move on

**Vision:** By the time of handoff, the community operates peers, governance (if any) is distributed, and the founder is optional.

### 12.1 Succession checklist

- [ ] Foundation or council owns roadmap and treasury policy (if applicable).
- [ ] Multiple independent DIG/PEGIN peers in production.
- [ ] Published operator runbooks (peer, indexer, SMTP recovery adapter).
- [ ] Custody referral agreements documented without founder as sole signatory.

### 12.2 What “success” means (non-financial)

- Enterprises use PEGIN without depending on one person’s infrastructure.
- Recovery works via federated email and on-chain/DIG policy.
- Custody remains with regulated providers.
- Community can ship protocol improvements without founder approval.

### 12.3 Then what

Founder may start adjacent work (Vault product, DAO tooling, etc.) **on top of** open PEGIN — not by breaking the protocol. See [roadmap.md](../03-use-cases/roadmap.md) Phase 4+.

---

*Related: [business-principles.md](business-principles.md) · [sustainable-funding.md](../05-business/sustainable-funding.md) · [08-developer/README.md](../08-developer/README.md)*
