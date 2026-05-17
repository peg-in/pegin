# PEGIN Enterprise Business Plan

Enterprise-grade decentralized SSO — **planning doc**. Competes with Azure AD / Okta at the **app SSO layer** (SAML/OIDC/SCIM), not full Active Directory in v1. No validated pricing or growth figures.

| | |
|---|---|
| **Audience** | Regulated industries, EU sovereignty buyers, Fortune-scale IT |
| **Technical spec** | [enterprise-identity-spec.md](../08-developer/specs/enterprise-identity-spec.md) |
| **EU / sovereignty** | [competitive-moat.md § European data sovereignty](competitive-moat.md#european-data-sovereignty-and-regulation-opportunity) |
| **Revenue hypotheses** | [sustainable-funding.md](sustainable-funding.md) |
| **Vault (later)** | [vault-business-plan.md](../07-penguin-products/vault-business-plan.md) |

---

## Table of contents

- [Executive summary](#executive-summary)
- [Enterprise requirements](#enterprise-requirements)
- [Bulk provisioning](#bulk-provisioning)
- [Microsoft Entra / Azure AD](#microsoft-entra--azure-ad)
- [Recovery](#recovery)
- [Vault (Phase 2+)](#vault-phase-2)
- [Deployment & SLA](#deployment--sla)
- [Security & compliance](#security--compliance)
- [Business model](#business-model)
- [Go-to-market](#go-to-market)
- [Proof required](#proof-required)
- [Related documents](#related-documents)

---

## Executive summary

PEGIN Enterprise targets organizations that need **passwordless SSO**, **optional EU/sovereign deployment**, and a path to **PePP** (permissions on DIG) without mandatory US SaaS control planes.

| Differentiator | Direction |
|----------------|-----------|
| **Scale** | Merkle-root bulk provisioning — one on-chain commitment for many users (design goal) |
| **Recovery** | Federated email + optional Chia Signer multi-sig (see [fully-decentralized.md](../01-vision/fully-decentralized.md) §5) |
| **Integration** | Parallel run with Entra; SAML/OIDC/SCIM/LDAP gateway phased |
| **Audit** | Events on **DIG**; Chia anchors store updates only — not full logs on chain |
| **Trust** | Open core; operator or customer runs infrastructure in chosen jurisdiction |

**Not claimed without pilots:** specific TCO vs Entra, 99.99% uptime, FedRAMP readiness, or seat-price discounts.

---

## Enterprise requirements

### Scaling model

| | Traditional IdP (typical) | PEGIN direction |
|---|---------------------------|-----------------|
| Bulk onboard | Many API calls / directory writes | Merkle tree of users; **root anchored on Chia** once per batch |
| User data | Vendor database | Encrypted on **DIG** or customer store |
| Membership proof | Directory lookup | Prove inclusion in published tree |

### Performance targets (set in contracts after pilots)

| Metric | Target (hypothesis) | Notes |
|--------|---------------------|--------|
| Login latency (p99) | Sub-second | Passkey + DIG/chain checks — measure per deployment |
| Bulk provision | Seconds for large CSV | Depends on DIG and anchor pipeline |
| Availability | Tiered SLA | Operator-run; not guaranteed by protocol alone |
| Audit | Append-only DIG + anchors | Export for SIEM / regulators |
| Recovery RTO | Minutes–hours by path | Email vs multi-sig |

---

## Bulk provisioning

1. Admin uploads CSV (email, department, roles).
2. Bulk manager builds merkle tree locally; enterprise key signs root.
3. Root committed on Chia; user records encrypted on DIG (or customer datastore).
4. Users authenticate with passkey + membership proof.

**Updates:** new root on add/revoke batch; apps honor revocation on next permission check (PePP) or tree state.

```http
POST /enterprise/bulk-provision
Content-Type: application/json

{
  "users": [{ "email": "alice@acme.com", "department": "Engineering", "roles": ["dev"] }],
  "signature": "0x..."
}
```

Response shape (illustrative): `merkleRoot`, `anchorTx`, `usersProvisioned`, `status`.

---

## Microsoft Entra / Azure AD

Gradual migration — no forced forklift.

| Phase | Scope |
|-------|--------|
| **1 — Pilot** | One department; PEGIN SSO beside Entra |
| **2 — Apps** | Non-M365 apps on PEGIN; Entra for Office where required |
| **3 — Evaluate** | TCO worksheet + sovereignty review |
| **4 — Optional cutover** | PEGIN as primary IdP for integrated apps |

**Sync (when built):** SCIM or connector — users, groups, attributes; schedule or near-real-time.

**Protocols:** SAML 2.0, OIDC, LDAP gateway for legacy, SCIM provisioning — see [roadmap.md](../03-use-cases/roadmap.md).

---

## Recovery

| Path | Use case | Outline |
|------|----------|---------|
| **Email (Phase 1)** | Lockout, lost device | Magic link via federated SMTP; session on DIG; new passkey; DID rotation on Chia |
| **Multi-sig (enterprise)** | High assurance / custody | M-of-N admins; timelock; execution via contracts |

Audit: recovery steps **appended on DIG**; store update **anchored** on Chia.

| Scenario | Email | Multi-sig |
|----------|-------|-----------|
| Forgot passkey | Yes | Optional |
| Lost device | Yes | Yes |
| Suspected breach | No | Yes |
| Termination | Revoke grants (PePP) | Yes |

---

## Vault (Phase 2+)

SSO first; **Penguin Vault** later for institutional key custody — see [vault-business-plan.md](../07-penguin-products/vault-business-plan.md). Same identity layer; separate legal and audit bar.

---

## Deployment & SLA

| Model | Who runs it | Typical buyer |
|-------|-------------|---------------|
| **Managed operator** | Certified EU/global operator + DIG peers | Mid-market, speed to value |
| **Private cloud** | Customer VPC (AWS/Azure/GCP **in chosen region**) | Data residency |
| **On-prem / air-gap** | Customer datacenter | Banking, government, defense suppliers |
| **Hybrid** | Staging SaaS, prod self-hosted | Phased migration |

**SLA tiers (hypothesis — price per engagement, not in this doc):**

| Tier | Uptime target | Support |
|------|---------------|---------|
| Standard | 99.9% | Business hours |
| Professional | 99.95% | Extended |
| Enterprise | 99.99% | 24/7 + named team |
| Regulated | 99.99% + compliance pack | Security + audit export |

---

## Security & compliance

> Design toward common frameworks; **certification is per deployment** with customer counsel.

| Framework | PEGIN relevance |
|-----------|-----------------|
| **GDPR** | DPIA, minimization, EU hosting option, transfer analysis if US subprocessors remain |
| **NIS2 / DORA** | IAM, logging, vendor oversight for essential / financial entities |
| **SOC 2 / ISO 27001** | Operator controls, not automatic from open source |
| **HIPAA / FedRAMP** | Sector-specific; requires scoped deployment and legal review |

**Data residency:** DIG peers and operators in **customer-chosen regions** (e.g. EU-only). Chia anchors are lightweight commitments, not bulk personal data.

**Zero-trust direction:** passkey auth, device posture hooks, continuous re-check for PePP grants.

**EU sovereignty buyers:** see [competitive-moat.md](competitive-moat.md#european-data-sovereignty-and-regulation-opportunity) for GDPR, Data Act, Schrems II, SecNumCloud, and public-sector migration context.

---

## Business model

Open **protocol**; paid **operator services** (hypothesis):

| Stream | Notes |
|--------|--------|
| **SLA / managed PEGIN** | Uptime, patching, support — priced per contract |
| **Professional services** | Entra migration, integration, compliance workshops |
| **DIG peer / hosting** | EU or on-prem peer operation |
| **Tokens / NFTs** | Legal review required; not a sales promise |

No per-seat license on core protocol by default. Details: [sustainable-funding.md](sustainable-funding.md) · [competitive-moat.md](competitive-moat.md).

---

## Go-to-market

1. **Vertical wedge** — finance, health, public sector, or EU sovereignty (one at a time).
2. **Land & expand** — one department, measured grant/revoke and login SLOs.
3. **POC** — time-boxed pilot with success criteria (no generic “% cheaper than Entra”).
4. **Partners** — systems integrators with EU regulated experience.

**Year 1 goal (qualitative):** a small number of named design partners with signed LOI or paid POC — not user or ARR targets in this doc.

---

## Proof required

- [ ] Pilot TCO vs incumbent (named customer, 12-month view).
- [ ] Measured login latency and bulk-provision time.
- [ ] Offboarding / revoke drill (PePP + integrated apps).
- [ ] EU deployment: jurisdiction, subprocessors, DPIA sign-off where claimed.
- [ ] Security assessment for chosen tier (SOC 2 path if operator claims it).

---

## Related documents

| Document | Why |
|----------|-----|
| [enterprise-identity-spec.md](../08-developer/specs/enterprise-identity-spec.md) | Protocol scope |
| [competitive-moat.md](competitive-moat.md) | Positioning + EU regulation |
| [sustainable-funding.md](sustainable-funding.md) | Revenue hypotheses |
| [permission-platform.md](../02-product/permission-platform.md) | PePP / Citrix gap |
| [business-plan.md](business-plan.md) | Protocol-level business |
| [roadmap.md](../03-use-cases/roadmap.md) | Phase gates |

*Version 3.0 · May 2026 · Concise layout; supersedes PDF-export draft*
