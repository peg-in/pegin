# Project architecture

High-level system design for **everyone on the team** — product, business, operators, and engineers. Describes *what* PEGIN is and *how* components connect (Chia, DIG, PePP, future products).

| Audience | Start here |
|----------|------------|
| **Any team member** | [application-architecture.md](application-architecture.md) (overview) → [on-chain-architecture.md](on-chain-architecture.md) |
| **Programmers** | Then [08-developer/README.md](../08-developer/README.md) + [04-technical/specs/](../04-technical/specs/README.md) |
| **Culture & tools** | [09-how-we-work/](../09-how-we-work/README.md) |

**Repo phase:** Documentation foundation — no deployed services yet.

---

## Read order

| # | Document | Topic |
|---|----------|--------|
| 1 | [application-architecture.md](application-architecture.md) | Layers, modules, data plane, DIG vs Chia vs optional SQL |
| 2 | [on-chain-architecture.md](on-chain-architecture.md) | DIDs, Rue contracts, verification |
| 3 | [permission-data-model.md](permission-data-model.md) | PePP grants & audit (Phase 2) |
| 4 | [dig-enterprise-transformation.md](dig-enterprise-transformation.md) | Enterprise apps on DIG |
| 5 | [dig-incentives-integration.md](dig-incentives-integration.md) | Peer economics |

### Future products

| Document | Topic |
|----------|--------|
| [products/gateway-architecture.md](products/gateway-architecture.md) | Penguin Gateway |
| [products/vault-architecture.md](products/vault-architecture.md) | Penguin Vault |

---

## Data plane (summary)

| Layer | What goes here |
|-------|----------------|
| **Chia** | DIDs, contracts, **store-update anchors** |
| **DIG** | Profiles, sessions, **grants**, **audit logs** |
| **Never on Chia** | Full audit bodies, bulk session JSON |

Detail: [application-architecture.md](application-architecture.md) · vision: [fully-decentralized.md](../01-vision/fully-decentralized.md)

---

## Related

| Folder | Role |
|--------|------|
| [01-vision/](../01-vision/) | Why PEGIN exists |
| [02-product/](../02-product/) | Product narrative (PePP, ecosystem) |
| [04-technical/specs/](../04-technical/specs/) | Implementation specs (Spec 1 & 2) |
| [08-developer/](../08-developer/) | SDK, lint, tests — **programmers only** |
| [09-how-we-work/](../09-how-we-work/) | Collaboration — **all roles** |
