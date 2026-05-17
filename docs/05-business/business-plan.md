# PEGIN Business Plan

**PEGIN** (Penguin Gateway Identity) is decentralized single sign-on on Chia + optional DIG storage: passkey login anchored to a user-owned DID, with credentials and verification on-chain or via open protocols—not a hosted multi-tenant IdP.

---

## At a glance

| | |
|---|---|
| **What we sell** | A protocol and open-source stack—not per-seat SaaS by default |
| **First product** | “Login with PEGIN” (WebAuthn → Chia DID → JWT) — [mvp-strategy.md](../03-use-cases/mvp-strategy.md) |
| **Revenue hypothesis** | Operator services, optional NFT royalties, optional token/governance—see [sustainable-funding.md](sustainable-funding.md) |
| **Moat hypothesis** | Structural (user-owned identity, forkable core, no vendor kill switch)—see [competitive-moat.md](competitive-moat.md) |
| **Build order** | [roadmap.md](../03-use-cases/roadmap.md) (POC → v1 → PePP → enterprise protocols) |

---

## 1. Executive summary

PEGIN targets **foundational identity infrastructure** comparable in role to DNS or TLS: widely used, no single operator required for the protocol to function.

**Near term:** Ship a credible POC and v1 that developers can integrate without passwords or seed phrases in the user path.

**Long term:** Optional enterprise protocols (SAML, OIDC, SCIM), permission platform (PePP), and operator-run DIG peers—without reverting to centralized API billing as the default business model.

**What PEGIN is not (by design):**

- Not a hosted user directory with recurring per-seat license as the core story
- Not dependent on one company’s servers for verification logic (smart contracts + client/SDK)
- Not claiming validated TCO vs Okta/Entra until pilots produce data

---

## 2. Problem and opportunity

### 2.1 Centralized SSO limitations

Enterprise and consumer apps rely on IdPs (Entra, Okta, Auth0) that:

- Tie identity to the vendor’s control plane and contracts
- Scale customer cost with seats, MAU, and feature tiers
- Concentrate breach and outage risk
- Make portable, user-owned credentials secondary

### 2.2 PEGIN direction

- **User-owned anchor:** Chia DID + passkey (no passwords in the happy path)
- **Verifiable credentials:** Issued and checked against on-chain rules and issuer keys
- **Open core:** Forkable code; customers and operators can self-host or run peers
- **Optional DIG layer:** Metadata and app data off-chain with on-chain anchors—not required for core login

Structural differentiation is documented in [differentiators.md](../03-use-cases/differentiators.md).

---

## 3. Product scope and delivery

Product and engineering scope live in dedicated docs; this plan only states **business-relevant boundaries**.

| Topic | Document |
|-------|----------|
| POC scope | [mvp-strategy.md](../03-use-cases/mvp-strategy.md) |
| Phased delivery | [roadmap.md](../03-use-cases/roadmap.md) |
| Tech stack & repos | [tech-stack.md](../08-developer/specs/tech-stack.md) |
| On-chain design | [on-chain-architecture.md](../08-developer/architecture/on-chain-architecture.md) |
| Enterprise protocols (SAML/OIDC/SCIM; not full AD DS in v1) | [enterprise-identity-spec.md](../08-developer/specs/enterprise-identity-spec.md) |
| SDK / “Login with PEGIN” | [sdk-guide.md](../08-developer/integration/sdk-guide.md) |

**Explicitly out of scope for early phases:** Production SLA billing, SCIM at scale, full PePP, and dollar-based ROI claims—see roadmap Phase 0–1.

---

## 4. Business model

PEGIN is positioned as a **protocol**, not a classic SaaS company. Economic alignment is described in [business-principles.md](../01-vision/business-principles.md).

### 4.1 Principles

1. **No default API seat tax** — dApps integrate open SDKs; they pay chain and operator costs, not a PEGIN per-call invoice.
2. **Alignment** — Long-term value tied to adoption and optional operator/token layers, not lock-in renewals.
3. **Measure before claiming** — TCO, margins, and token value are hypotheses until pilots.

### 4.2 Possible revenue streams (hypotheses)

Full detail and status table: [sustainable-funding.md](sustainable-funding.md).

| Stream | Role | Status |
|--------|------|--------|
| Operator / SLA services | Run or support DIG nodes, migration, enterprise support | Hypothesis; price per engagement |
| NFT royalties | Share of credential NFT trades (e.g. 2–5% if enabled) | Hypothesis; depends on marketplace volume |
| Issuer registration | One-time on-chain fee to mint issuer NFT | Hypothesis; fee band TBD |
| Token / governance | Incentives, voting, ecosystem alignment | Requires legal review; no price assumptions |
| DIG peer operation | Storage/proof incentives on the network | Depends on network rules and load |

### 4.3 What we do not rely on

- Recurring per-dApp API metering as the primary business
- Storing end-user secrets or directories in a central PEGIN database
- Unvalidated multi-year revenue tables in documentation

---

## 5. Protocol economics: NFTs and tokens

> **Hypothesis layer** — mechanics below are design intent for Rue contracts and NFT policy; not audited financial projections.

### 5.1 NFT roles

| NFT | Purpose |
|-----|---------|
| **Identity** | User DID, keys, credential index; held in user wallet |
| **Credential** | KYC, education, employment, etc.; issuer-signed; revocable on-chain |
| **Issuer** | Registers an organization; controls issuance and revocation for its credentials |

Technical mapping: [on-chain-architecture.md](../08-developer/architecture/on-chain-architecture.md).

### 5.2 Issuer workflow (target)

1. Register issuer NFT (one-time on-chain step; fee TBD).
2. Issue W3C Verifiable Credentials; sign with issuer key.
3. Optionally publish metadata to DIG.
4. User holds credential NFT in wallet.
5. Relying parties verify via SDK + chain (and optional indexer).

### 5.3 Early issuer incentives (hypothesis)

- Fee waivers or discounts for early issuers (cap and rules TBD).
- Documentation and integration support—not guaranteed revenue share until economics are modeled on testnet.

### 5.4 Governance token (if pursued)

- Fixed supply and allocation are **not** committed in this document.
- Any public sale, employee grants, or liquidity plans require legal review.
- Do not quote token price or “appreciation” in customer-facing materials without a market and disclosure.

---

## 6. Ecosystem participants

| Participant | Value | PEGIN ask |
|-------------|-------|-----------|
| **End users** | Passwordless login; portable DID | Run wallet/passkey; pay chain fees |
| **dApps / developers** | OIDC/JWT integration; no vendor lock-in for core | Integrate SDK; self-host or use operator |
| **Credential issuers** | Verifiable issuance; reduced bespoke infra | Mint issuer NFT; follow issuance spec |
| **Operators** | SLA, hosting, enterprise bridge | Optional paid services—see sustainable-funding |
| **Chia / DIG ecosystem** | Transaction and storage activity | Neutral benefit from adoption |

### 6.1 Growth flywheel (qualitative)

More users → more issuers and dApps → more on-chain activity → stronger protocol utility → more builder interest.

No numeric targets are stated here; phase goals are in [roadmap.md](../03-use-cases/roadmap.md).

---

## 7. Go-to-market and phases

GTM follows the **product roadmap**, not a separate 2024–2025 calendar from legacy drafts.

| Phase | Business focus | Reference |
|-------|----------------|-----------|
| **0 — POC** | One integration story: Login with PEGIN; developer docs | [roadmap.md](../03-use-cases/roadmap.md) § Phase 0 |
| **1 — v1** | Early adopters; OIDC hardening; SAML bridge begins | Phase 1 |
| **2 — PePP** | Permission platform on DIG; enterprise workflows | Phase 2 |
| **3 — Enterprise** | SCIM, scale, compliance narratives with pilot evidence | Phase 3 |
| **4+** | Vault, cross-chain, marketplace—future products | Phase 4 |

**Enterprise path:** [enterprise-business-plan.md](enterprise-business-plan.md) (planning only; no validated pricing).

---

## 8. Sustainability and protocol independence

### 8.1 Designed to outlive the founder

- Smart contracts and open-source clients are the source of truth for verification.
- Indexers and DIG peers can be run by others; no single mandatory API host.
- Governance and token distribution, if used, should trend toward community control over time.

See also [fully-decentralized.md](../01-vision/fully-decentralized.md) for DIG-centric succession design.

### 8.2 Centralized IdP vs PEGIN (qualitative)

| Dimension | Typical centralized IdP | PEGIN direction |
|-----------|---------------------------|-----------------|
| Control | Vendor directory and admin plane | User wallet + on-chain rules |
| Pricing | Per-seat / MAU contracts | Open core; optional operator fees |
| Outage / acquisition | Vendor operational risk | Protocol + many operators |
| Portability | Export/migration projects | DID and forkable stack by design |

No claim that PEGIN is “free” in absolute terms—operators, chain fees, and integration work still cost money.

### 8.3 Success metrics (to define in pilots)

- Time to integrate Login with PEGIN on a greenfield app
- Login/register latency and success rate on target browsers
- Operator cost per deployment vs incumbent stack (12-month TCO worksheet)
- Issuer onboarding friction (steps, chain fees, support hours)

Record results in when available.

---

## 9. Risks and open questions

| Risk | Mitigation |
|------|------------|
| Chia/DIG adoption and fee UX | Testnet POC; clear fee disclosure in UX |
| Regulatory (tokens, KYC credentials) | Legal review before token events; issuer responsibility model |
| Enterprise expectation mismatch | [enterprise-identity-spec.md](../08-developer/specs/enterprise-identity-spec.md) — app SSO vs AD DS |
| Unproven revenue streams | Pilot one stream at a time; document what customers actually pay for |
| Smart contract bugs | Rue tests, simulator, external audit before mainnet |

---

## 10. Related documents

| Document | Why read it |
|----------|-------------|
| [sustainable-funding.md](sustainable-funding.md) | Revenue hypotheses and post-pilot worksheets |
| [competitive-moat.md](competitive-moat.md) | Positioning vs Okta/Entra without fake savings % |
| [business-principles.md](../01-vision/business-principles.md) | Long-term principles and customer alignment |
| [roadmap.md](../03-use-cases/roadmap.md) | What ships when |
| [core-value-user-owned-login.md](../01-vision/core-value-user-owned-login.md) | User-facing value story |
| [enterprise-business-plan.md](enterprise-business-plan.md) | Enterprise GTM (illustrative) |

---

*Last updated: May 2026 · Version 2.0 (restructured plan; supersedes PDF-export layout)*
