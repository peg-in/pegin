# Penguin Vault — Business Plan

**Penguin Vault** is a planned decentralized identity and key-custody layer on Chia + DIG: serverless authentication, user-controlled vaults, and optional institutional custody (KCS) with timelocked recovery—not a centralized password vault.

---

## At a glance

| | |
|---|---|
| **Status** | Planning / future — after PEGIN SSO core and optional PePP |
| **Depends on** | [Login with PEGIN](../../03-use-cases/mvp-strategy.md), stable DID + contracts |
| **Technical reference** | [vault-architecture.md](../../10-architecture/products/vault-architecture.md) |
| **PEGIN core business model** | [business-plan.md](../../05-business/business-plan.md) · [sustainable-funding.md](../../05-business/sustainable-funding.md) |
| **Sibling product** | [gateway-business-plan.md](gateway-business-plan.md) (SSO access layer) |

---

## Table of contents

- [Executive summary](#1-executive-summary)
- [Problem](#2-problem)
- [Solution](#3-solution)
- [Product scope](#4-product-scope)
- [Core features](#5-core-features)
- [Business model (hypotheses)](#6-business-model-hypotheses)
- [Go-to-market](#7-go-to-market)
- [Competitive positioning](#8-competitive-positioning)
- [Risks and what to measure](#9-risks-and-what-to-measure)
- [Related documents](#10-related-documents)

---

## 1. Executive summary

Penguin Vault addresses the **self-custody vs recoverability** gap in Web3: users want control of keys without irreversible loss; enterprises need institutional-grade custody patterns without a single honeypot login server.

**Three pillars (target design):**

1. **Penguin Login** — Passkey / Chia Signer authentication aligned with PEGIN (no central credential store).
2. **Penguin KCS (Key Custody Service)** — Optional regulated or enterprise custodians; biometric + identity-verified rekey with timelocked cancellation.
3. **Vault hierarchy** — Personal vault, app-scoped child vaults on DIG, and a recovery root (“master vault”) for enterprise deployments.

**Near-term business stance:** Do not quote TAM, ARR, or user targets until a vault pilot exists. Custody and identity products require legal, audit, and partner agreements beyond the SSO POC.

---

## 2. Problem

| Pain | Today | Vault direction |
|------|--------|-----------------|
| Key loss | Self-custody is often unrecoverable | Timelocked recovery via custodian or social/shares (hypothesis) |
| Centralized custody risk | Exchange / custodian breaches | User-sovereign keys + optional regulated partners, not one global password DB |
| Fragmented identity | Many wallets, no portable layer | Unified DID + credentials on Chia/DIG |
| Enterprise gap | No standard “institutional Web3 identity” | White-label vault + custodian dashboard (future) |
| Credential silos | DIDs and credentials don’t compose | W3C VC + DIG-isolated app stores |

---

## 3. Solution

### 3.1 Penguin Login

Serverless authentication via passkeys or Chia Signer. Credentials and keys stay in the user’s control; verification uses chain rules and open SDKs (shared with PEGIN core).

### 3.2 Penguin KCS

Custodians (banks, trust companies, enterprise security teams) participate in **rekey and recovery** workflows with:

- Proof-of-identity and biometric gates
- On-chain timelocks (e.g. 24–72 hours) before sensitive changes finalize
- SLAs negotiated per partner—not list prices in this doc

### 3.3 Vault system

| Tier | Purpose |
|------|---------|
| **Personal vault** | User assets, DIDs, credential NFTs |
| **App child vault** | DIG-isolated stores per dApp |
| **Master vault (enterprise)** | Recovery root and policy for org-wide deployments |

### 3.4 DIG integration

Off-chain metadata and app data with on-chain anchors; optional zero-knowledge verification patterns for credentials across dApps.

---

## 4. Product scope

Engineering detail lives in the architecture doc; this plan states **business boundaries only**.

| Topic | Document |
|-------|----------|
| Vault technical design | [vault-architecture.md](../../10-architecture/products/vault-architecture.md) |
| PEGIN stack & contracts | [tech-stack.md](../../04-technical/specs/tech-stack.md) |
| When Vault ships | [roadmap.md](../../03-use-cases/roadmap.md) — Phase 4+ future products |
| SSO sibling (Gateway) | [gateway-business-plan.md](gateway-business-plan.md) |

**Out of scope until PEGIN core ships:** Mainnet custody SLAs, consumer marketing at scale, credential marketplace, validated financial projections.

---

## 5. Core features

Planned capability matrix (order and dates are **planning estimates**—follow [roadmap.md](../../03-use-cases/roadmap.md), not legacy 2024 quarters).

| Feature | Description | Phase (indicative) |
|---------|-------------|-------------------|
| Penguin Login (basic) | Passkey + Chia Signer auth | After SSO POC |
| Personal vault | User credential & asset container | Vault MVP |
| Biometric unlock | Face ID / Touch ID on mobile | Vault MVP+ |
| Penguin KCS (alpha) | Custodian rekey with timelock | Post-MVP |
| DIG app stores | Isolated per-dApp data | With PePP / DIG path |
| Multi-sig vaults | M-of-N control | Enterprise beta |
| Master vault | Org hierarchy | Enterprise |
| SDKs | TypeScript, Python, Go | Parallel with Gateway SDK |
| Custodian dashboard | Recovery requests, audit views | Enterprise |
| Social recovery | Shamir / guardian shares | Hypothesis — legal review |

---

## 6. Business model (hypotheses)

Aligned with [sustainable-funding.md](../../05-business/sustainable-funding.md).

| Stream | Idea | Status |
|--------|------|--------|
| **KCS partnerships** | Custodians pay for integration + SLA; possible rev-share with partners | Negotiated per deal |
| **Vault tiers** | Free personal tier vs enterprise white-label | Pricing TBD after pilot |
| **Professional services** | Security reviews, compliance support, migration | Services revenue |
| **Ecosystem** | dApp/exchange integration fees or referrals | Hypothesis |
| **Open core** | Protocol code forkable; monetize operation not seat tax | Same as PEGIN core |

Do not publish subscription tiers or rev-share splits without signed partner terms.

---

## 7. Go-to-market

Phases are **qualitative**; timing follows PEGIN core + roadmap Phase 4.

### 7.1 Developers and early adopters

- Testnet vault APIs alongside PEGIN login
- Open-source components for audit and community
- Integrate with a small set of DeFi / wallet partners (pilot count TBD)

### 7.2 Custodian and enterprise

- Pilot 1–3 custodian or enterprise design partners
- Pursue SOC 2 / ISO 27001 paths only when product scope is fixed
- Direct sales for regulated industries—no Fortune 500 count targets in docs

### 7.3 Consumer and ecosystem

- Consumer app only after custody legal model is clear
- Credential issuers and marketplace are **future** options, not commitments

### 7.4 Channels

- Developer community (GitHub, docs, conferences)
- Strategic integrations (wallets, exchanges, Gateway SSO)
- Enterprise and custodian direct sales
- Content and standards work (W3C VC, OIDC alignment)

---

## 8. Competitive positioning

| Differentiator | Notes |
|----------------|--------|
| Blockchain-native | Chia + DIG from design, not retrofitted SaaS |
| Privacy-oriented | ZK-friendly credential checks where applicable |
| Ecosystem fit | Complements Penguin Gateway (SSO) and PEGIN core |
| Institutional path | Custodian partnerships vs pure self-custody wallets |
| Open source | Auditable; operator-run deployment |
| No single login server | Recovery via cryptography + policy, not one corporate DB |
| Hardware biometrics | OS-backed Face ID / Touch ID for UX |

**We do not claim:** validated TAM, cheaper than Coinbase Custody, or specific market share.

---

## 9. Risks and what to measure

| Risk | Mitigation |
|------|------------|
| Regulatory (custody, tokens) | Legal review before KCS launch; jurisdiction per partner |
| Key-loss narratives | Clear UX for timelocks and cancellation |
| Depends on PEGIN core delay | Vault stays Phase 4+ in [roadmap.md](../../03-use-cases/roadmap.md) |
| Smart contract / vault bugs | Rue tests, external audit before mainnet |
| Unproven KCS economics | Pilot one custodian; document costs and revenue from that pilot |

**Pilot metrics to define (no numbers until measured):**

- Time to complete custodian-assisted recovery drill
- User comprehension of timelock cancellation
- Operator cost per vault deployment vs incumbent custody quote
- Integration effort for one dApp (SDK hours)

---

## 10. Related documents

| Document | Why |
|----------|-----|
| [vault-architecture.md](../../10-architecture/products/vault-architecture.md) | Technical architecture |
| [gateway-business-plan.md](gateway-business-plan.md) | SSO layer (often integrated first) |
| [business-plan.md](../../05-business/business-plan.md) | PEGIN protocol business model |
| [roadmap.md](../../03-use-cases/roadmap.md) | When Vault ships |
| [fully-decentralized.md](../../01-vision/fully-decentralized.md) | Succession and DIG-centric operations |

---

## Conclusion

Penguin Vault is the **custody and recovery** complement to PEGIN login and Penguin Gateway SSO: user sovereignty first, optional institutional paths second. Success depends on shipping core identity, proving custody workflows with real partners, and measuring economics in pilots—not on spreadsheet projections in planning docs.

*Version 2.0 · May 2026 · Restructured layout; supersedes PDF-export draft*
