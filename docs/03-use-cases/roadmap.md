# PEGIN product roadmap

> Single plan for what we build, in what order, and what is explicitly **future**. Timelines are planning estimates until the POC ships.

See also: [mvp-strategy.md](mvp-strategy.md) (POC scope only), [differentiators.md](differentiators.md) (why decentralized identity is structurally different).

---

## Principles

1. **One feature first:** “Login with PEGIN” (passkey → Chia DID → JWT).
2. **Protocols in order:** WebAuthn → OIDC → SAML → OAuth → SCIM → LDAP (enterprise path).
3. **Permissions after SSO works:** PePP builds on the same DID; do not block POC on PePP.
4. **Measure before claiming:** Enterprise outcomes require pilot data before external claims.

---

## Phase 0 — POC (target: ~8 weeks)

**Goal:** Prove passkey login anchored to a Chia DID on testnet.

| Deliverable | Done when |
|-------------|-----------|
| `pegin-core` | Register + login via WebAuthn; issue JWT |
| `pegin-contracts` | DID anchor on testnet (Rue) |
| `@pegin/sdk` | “Login with PEGIN” on a demo app |
| OIDC | `/.well-known/openid-configuration` + token endpoint for 1–2 test apps |

**Out of scope:** SAML, SCIM, admin dashboard, PePP, production SLA, billing.

**Success criteria:** Register &lt; 5s, login &lt; 1s, Chrome/Safari/Firefox, no passwords in user flow.

---

## Phase 1 — v1.0 identity (post-POC, ~16 weeks)

**Goal:** Usable by early adopters (startups, internal tools).

- Harden auth (recovery, rate limits, basic audit events)
- OIDC hardening + documented SDK integration
- SAML 2.0 (enterprise bridge)
- Minimal operator docs (self-host or managed beta)
- Security review of Rue contracts

**Not yet:** Full PePP, Citrix replacement narrative, enterprise SCIM at scale.

---

## Phase 2 — Permission platform (PePP)

**Goal:** App-defined permissions on DIG; mobile approve/deny; revocation on next auth check.

Depends on: stable DID login + DIG read/write path.

| Capability | Notes |
|------------|--------|
| Permission schema on DIG | See [permission-data-model.md](../10-architecture/permission-data-model.md) |
| Manager approve flow | Target: much faster than typical ticket + AD sync — **measure in pilots** |
| Time-bound grants | Auto-expire |
| Revocation | Update DIG record; apps re-query |

**Out of scope for first PePP release:** Full AD/Citrix parity, custom Rue rule marketplace.

---

## Phase 3 — Enterprise operations

**Goal:** Teams that run Okta/Azure AD today can federate or migrate gradually.

- SCIM provisioning / deprovisioning
- Azure AD / Okta federation patterns (parallel run)
- Bulk identity (merkle root on-chain where applicable)
- Compliance-oriented audit export (design with legal/compliance advisors)

---

## Phase 4 — Ecosystem & economics

**Goal:** Sustainable network, not seat licenses.

- DIG incentive alignment (storage fees, peer operation) — model in pilots
- Credential NFTs / royalties (if product still fits market)
- PEGIN token / CAT (only if legal and product clear)
- Optional: customer-run DIG peers

---

## Future products (separate from core PEGIN SSO)

Documented under `07-penguin-products/` — **not part of Phase 0–2**.

| Product | Intent |
|---------|--------|
| **Penguin Gateway** | Web3 / dApp SSO |
| **Penguin Vault** | Key custody |

Ship only after core SSO (+ optional PePP) has users and learnings.

---

## Capability matrix

| Capability | Phase 0 | Phase 1 | Phase 2 | Phase 3 | Future |
|------------|---------|---------|---------|---------|--------|
| Passkey + DID + JWT | ✅ | ✅ | ✅ | ✅ | |
| OIDC | ✅ basic | ✅ | ✅ | ✅ | |
| SAML | | ✅ | ✅ | ✅ | |
| SCIM | | | | ✅ | |
| PePP permissions | | | ✅ | ✅ | |
| Credential NFTs | | | | | Phase 4 |
| Penguin Gateway / Vault | | | | | 07-penguin-products |

---

## Use cases by phase (not separate documents)

| User outcome | Phase |
|--------------|-------|
| Passwordless login to a SaaS app | 0 |
| Federate with existing IdP (OIDC/SAML) | 1 |
| User-owned DID; employer grants permission | 0–1 (identity); 2 (permissions) |
| Faster access requests / revocation | 2 (validate in pilots) |
| Portable credentials after employer gone | 1+ (DID persists); issuers TBD |
| Cross-org contractor, one DID | 3 |
| Global / data-sovereignty DIG topology | 3–4 |

Structural “why decentralized” stories: [differentiators.md](differentiators.md).

---

## Next actions (project)

1. Ship Phase 0 POC on Chia testnet.
2. Recruit 1–3 design partners; define metrics (login time, revoke time, support tickets).
3. Update with pilot results.
4. Regenerate AI knowledge base: `python3 scripts/generate-ai-knowledge-base.py`