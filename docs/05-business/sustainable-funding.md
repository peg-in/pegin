# PEGIN sustainable funding model

## Direction

PEGIN targets **open-source core** (no per-seat license as the default story) and revenue from **infrastructure, services, and ecosystem participation** — not from locking customers into annual seat tax.

Incumbent IdPs (Okta, Auth0, Entra) typically charge **per user/seat** and scale sales, support, and compliance with tenant count. PEGIN’s hypothesis is different economics: protocol + optional operator-run DIG peers + optional SLA/services.

---

## Why avoid per-seat license as the core model

| Centralized SaaS pattern | PEGIN direction |
|--------------------------|---------------|
| Per-user recurring fee | Open core; charge for what you operate (SLA, hosting, integration) |
| Vendor lock-in via contract | Portable DIDs; forkable code |
| Vendor keeps all upside | Shared network (peers, tokens, referrals) — **if** those layers ship and are legal |

**Competitive story (qualitative):** Another seat-based IdP can undercut on price; PEGIN aims to compete on **ownership model + deployment model**, not on quoting a cheaper per-user rate without data.

---

## Possible revenue streams (hypotheses)

Each stream needs a **business model canvas** filled from real pilots.

### 1. Infrastructure & operator services

- Run or support DIG storage / PEGIN nodes for enterprises.
- Flat **SLA tiers** (uptime, support hours, migration help) — price **TBD** per engagement.
- Cost to operator: engineering, hardware, bandwidth — track separately from customer quotes.

### 2. Token economics (if pursued)

- Fixed-supply token used for governance, incentives, or ecosystem alignment.
- **Legal and regulatory review required** before any public sale or employee compensation plan.
- No token price or market cap assumptions in docs until there is a market.

### 3. Custody & institutional referrals

- Refer enterprises to regulated custodians for key material.
- Revenue = partnership terms (referral fee %) — **negotiated**, not documented here.

### 4. Audit & compliance services

- Immutable logs may reduce audit **effort** — measure with auditors in pilots.
- Possible referral to third-party auditors; no margin estimates without contracts.

### 5. DIG peer operation

- Operators (including PEGIN Inc. or customers) run peers and may earn **network incentives** (e.g. XCH) for storage/proofs.
- Earnings depend on network rules, utilization, and market — not modeled here.

---

## Developer & contributor funding (without seat licenses)

| Mechanism | Description | Status |
|-----------|-------------|--------|
| Salaries | From fundraising or services revenue | Standard; budget TBD |
| Token grants | Vesting for team — requires legal plan | Hypothesis |
| Bounties | Community pays for issues in tokens or cash | Hypothesis; sizes TBD per bounty |
| DIG peer income | Contributors run infrastructure | Depends on network |

**Principle:** Do not promise “developers earn X from peers” until peer economics are measured on testnet/mainnet with real load.

---

## What to build after first pilot

1. **Customer TCO worksheet** — incumbent stack vs PEGIN run cost (12 months).
2. **Operator P&L** — cost to run one enterprise deployment (people + infra + chain fees).
3. **Revenue mix** — which streams customers actually pay for (SLA vs services vs none).
4. Update with dated rows for each finding.

---

## Related docs

- [business-principles.md](../01-vision/business-principles.md) — strategic principles (no dollar forecasts)
- [competitive-moat.md](competitive-moat.md) — why “free core” is a positioning choice, not a proven margin
- [enterprise-business-plan.md](enterprise-business-plan.md) — enterprise GTM patterns (pricing tables there are **illustrative only**)
- [roadmap.md](../03-use-cases/roadmap.md) — when commercial features ship