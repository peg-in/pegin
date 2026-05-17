# PEGIN developer documentation

**Start here if you build or integrate PEGIN.** Specs, architecture, SDK, and protocol reference live in this folder. Vision, product narrative, and business docs are elsewhere — linked below.

| | |
|---|---|
| **POC** | [Login with PEGIN](../03-use-cases/mvp-strategy.md) — passkey → Chia DID → JWT |
| **Phases** | [roadmap.md](../03-use-cases/roadmap.md) |
| **Repo layout** | [CLAUDE.md](../../CLAUDE.md) · [AGENTS.md](../../AGENTS.md) |
| **Evidence** | No unvalidated performance or pricing claims in specs — measure in pilots |

---

## Data plane (read once)

| Layer | What goes here |
|-------|----------------|
| **Chia** | DIDs, Rue contracts, credential NFTs, **Merkle roots / store-update anchors** |
| **DIG** | User profiles, sessions, **grants**, **audit append-only logs**, permission schemas |
| **Never on Chia** | Full audit bodies, bulk JSON, session payloads |

PePP and enterprise audit: [permissions/permission-data-model.md](permissions/permission-data-model.md) · [fully-decentralized.md §4](../01-vision/fully-decentralized.md#4-user-data-and-logs-on-dig)

---

## Documentation map

```
08-developer/
├── README.md                 ← you are here
├── specs/
│   ├── tech-stack.md         Spec 1 — crates, Chia, DIG, xch-dev, POC
│   └── enterprise-identity-spec.md   Spec 2 — OIDC, SAML, SCIM, Entra scope
├── architecture/
│   ├── on-chain-architecture.md
│   ├── dig-incentives-integration.md
│   └── dig-enterprise-transformation.md
├── integration/
│   └── sdk-guide.md          WebAuthn, JWT, OIDC, app integration
├── permissions/
│   └── permission-data-model.md    PePP (Phase 2)
└── products/
    ├── gateway-architecture.md
    └── vault-architecture.md
```

Root filenames (`tech-stack.md`, `sdk-guide.md`, …) are **redirect stubs** — use the paths above.

---

## Build path

### Phase 0 — POC

| # | Document | Why |
|---|----------|-----|
| 1 | [specs/tech-stack.md](specs/tech-stack.md) | Dependencies, official repos, workspace layout |
| 2 | [integration/sdk-guide.md](integration/sdk-guide.md) | WebAuthn, JWT, client/server integration |
| 3 | [architecture/on-chain-architecture.md](architecture/on-chain-architecture.md) | DIDs, contracts, verification |

### Phase 1 — Enterprise bridge

| # | Document | Why |
|---|----------|-----|
| 4 | [specs/enterprise-identity-spec.md](specs/enterprise-identity-spec.md) | SAML/OIDC/SCIM; what v1 does **not** include |
| 5 | [architecture/dig-enterprise-transformation.md](architecture/dig-enterprise-transformation.md) | Enterprise apps on DIG |

### Phase 2+ — Permissions & DIG ops

| # | Document | Why |
|---|----------|-----|
| 6 | [permissions/permission-data-model.md](permissions/permission-data-model.md) | PePP schemas and audit anchoring |
| 7 | [architecture/dig-incentives-integration.md](architecture/dig-incentives-integration.md) | Peer economics |

### Future products

| Document | When |
|----------|------|
| [products/gateway-architecture.md](products/gateway-architecture.md) | Penguin Gateway |
| [products/vault-architecture.md](products/vault-architecture.md) | Penguin Vault |

---

## All developer docs (index)

| Path | Description |
|------|-------------|
| [specs/tech-stack.md](specs/tech-stack.md) | **Spec 1** — stack, POC deliverables, implementation checklist |
| [specs/enterprise-identity-spec.md](specs/enterprise-identity-spec.md) | **Spec 2** — enterprise protocols and Microsoft links |
| [architecture/on-chain-architecture.md](architecture/on-chain-architecture.md) | On-chain identity and contracts |
| [architecture/dig-incentives-integration.md](architecture/dig-incentives-integration.md) | DIG storage and incentives |
| [architecture/dig-enterprise-transformation.md](architecture/dig-enterprise-transformation.md) | Enterprise + DIG architecture |
| [integration/sdk-guide.md](integration/sdk-guide.md) | SDK and integration patterns |
| [permissions/permission-data-model.md](permissions/permission-data-model.md) | PePP data model |
| [products/gateway-architecture.md](products/gateway-architecture.md) | Gateway (future) |
| [products/vault-architecture.md](products/vault-architecture.md) | Vault (future) |

Section indexes: [specs/](specs/README.md) · [architecture/](architecture/README.md) · [integration/](integration/README.md) · [permissions/](permissions/README.md) · [products/](products/README.md)

---

## Related (non-developer)

| Need | Document |
|------|----------|
| Why PEGIN | [01-vision/](../01-vision/) |
| PePP product story | [permission-platform.md](../02-product/permission-platform.md) |
| MVP & roadmap | [03-use-cases/](../03-use-cases/) |
| EU sovereignty / GTM | [competitive-moat.md](../05-business/competitive-moat.md) |
| Browser wiki | [PEGIN_Wiki.md](../wiki/PEGIN_Wiki.md) · [HTML bundle](../wiki/PEGIN_Wiki_Knowledge_Base.html) |
| Docs hub | [docs/README.md](../README.md) |
| AI / RAG | [ai/CONTEXT.md](../ai/CONTEXT.md) — regenerate: `python3 scripts/generate-ai-knowledge-base.py` |

---

*Canonical technical documentation. `04-technical/` redirects here.*
