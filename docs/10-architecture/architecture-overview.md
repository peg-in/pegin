# Project architecture

High-level system design for **everyone on the team** — product, business, operators, and engineers. Describes *what* PEGIN is and *how* components connect (Chia, DIG, PePP, future products).

| Audience | Start here |
|----------|------------|
| **Any team member** | [application-architecture.md](application-architecture.md) (overview) → [on-chain-architecture.md](on-chain-architecture.md) |
| **Programmers** | Then [08-developer/developer-documentation.md](../08-developer/developer-documentation.md) + [04-technical/specs/](../04-technical/specs/specifications-index.md) |
| **Culture & tools** | [09-how-we-work/](../09-how-we-work/team-how-we-work.md) |

**Repo phase:** Documentation foundation — no deployed services yet.

---

## Read order

| # | Document | Topic |
|---|----------|--------|
| 1 | [mini-wallet-and-recovery-vault.md](mini-wallet-and-recovery-vault.md) | **POC:** mini wallet, 1 DID + 1 recovery vault, faucet, instant login |
| 1b | [recovery-vault-and-guardians.md](recovery-vault-and-guardians.md) | SDK vault (Rigidity), email guardian, Chia Signer recovery |
| 1c | [dig-as-application-layer.md](dig-as-application-layer.md) | DIG for websites, services, and user-owned data |
| 2 | [application-architecture.md](application-architecture.md) | Layers, modules, data plane, DIG vs Chia vs optional SQL |
| 3 | [on-chain-architecture.md](on-chain-architecture.md) | DIDs, Rue contracts, verification |
| 4 | [permission-data-model.md](permission-data-model.md) | PePP grants & audit (Phase 2) |
| 5 | [dig-enterprise-transformation.md](dig-enterprise-transformation.md) | Enterprise apps on DIG |
| 6 | [dig-incentives-integration.md](dig-incentives-integration.md) | Peer economics |

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
| **DIG** | **Application layer** — identity, sessions, audit, **app/service data**, grants |
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
