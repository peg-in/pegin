# PEGIN documentation

PEGIN (Penguin Gateway Identity) is decentralized SSO on Chia + DIG. **If you build or integrate PEGIN, start with [08-developer/README.md](08-developer/README.md)** — that is the primary technical documentation.

---

## Primary: developer documentation

| | |
|---|---|
| **Index** | **[08-developer/README.md](08-developer/README.md)** |
| **Spec 1 (stack)** | [08-developer/specs/tech-stack.md](08-developer/specs/tech-stack.md) |
| **Spec 2 (enterprise)** | [08-developer/specs/enterprise-identity-spec.md](08-developer/specs/enterprise-identity-spec.md) |
| **SDK** | [08-developer/integration/sdk-guide.md](08-developer/integration/sdk-guide.md) |
| **POC scope** | [03-use-cases/mvp-strategy.md](03-use-cases/mvp-strategy.md) |

```
08-developer/
├── specs/ Spec 1 & 2
├── architecture/ On-chain · DIG
├── integration/ SDK
├── permissions/ PePP (Phase 2)
└── products/ Gateway & Vault (future)
```

---

## AI / knowledge base

| Resource | Use |
|----------|-----|
| [ai/CONTEXT.md](ai/CONTEXT.md) | Compact system context for LLMs |
| [ai/manifest.json](ai/manifest.json) | Document catalog |
| [ai/chunks.jsonl](ai/chunks.jsonl) | RAG chunks |
| [../llms.txt](../llms.txt) | [llms.txt](https://llmstxt.org/) index |

Regenerate: `python3 scripts/generate-ai-knowledge-base.py`

**Not for RAG:** HTML wiki, `wiki/PEGIN_Wiki.md`, `wiki/_archive/`, redirect stubs in `08-developer/` root.

---

## Quick links

| Goal | Start here |
|------|------------|
| **Build / integrate** | [08-developer/README.md](08-developer/README.md) |
| POC scope | [03-use-cases/mvp-strategy.md](03-use-cases/mvp-strategy.md) |
| Master roadmap | [03-use-cases/roadmap.md](03-use-cases/roadmap.md) |
| Philosophy (15 min) | [01-vision/core-value-user-owned-login.md](01-vision/core-value-user-owned-login.md) |
| Business / funding | [05-business/](05-business/) |
| Browser wiki | [wiki/README.md](wiki/README.md) · [HTML](wiki/PEGIN_Wiki_Knowledge_Base.html) |

---

## Folder map

```
docs/
├── 08-developer/ ★ Primary — specs, architecture, SDK (start here)
├── 03-use-cases/ Roadmap, MVP, differentiators
├── 02-product/ Product narrative (PePP, ecosystem)
├── 01-vision/ Philosophy and principles
├── 05-business/ Business plans, funding
├── 06-strategy/ Ecosystem positioning
├── 07-penguin-products/ Future product business plans
├── 00-getting-started/ Wiki hosting
├── 04-technical/ Redirect → 08-developer
├── ai/ RAG manifest and CONTEXT
└── wiki/ HTML bundle
```

---

## 08 — Developer (canonical technical docs)

Full index: **[08-developer/README.md](08-developer/README.md)**

| Section | Key files |
|---------|-----------|
| [specs/](08-developer/specs/) | [tech-stack.md](08-developer/specs/tech-stack.md), [enterprise-identity-spec.md](08-developer/specs/enterprise-identity-spec.md) |
| [architecture/](08-developer/architecture/) | [on-chain-architecture.md](08-developer/architecture/on-chain-architecture.md), DIG docs |
| [integration/](08-developer/integration/) | [sdk-guide.md](08-developer/integration/sdk-guide.md) |
| [permissions/](08-developer/permissions/) | [permission-data-model.md](08-developer/permissions/permission-data-model.md) |
| [products/](08-developer/products/) | Gateway & Vault architecture |

---

## 03 — Use cases & plan

| Document | Description |
|----------|-------------|
| [roadmap.md](03-use-cases/roadmap.md) | Phases 0–4 |
| [mvp-strategy.md](03-use-cases/mvp-strategy.md) | POC only |
| [differentiators.md](03-use-cases/differentiators.md) | vs centralized SSO |

---

## 02 — Product

| Document | Description |
|----------|-------------|
| [complete-ecosystem.md](02-product/complete-ecosystem.md) | SSO + PePP overview |
| [permission-platform.md](02-product/permission-platform.md) | PePP design (Phase 2) |

---

## 01 — Vision

| Document | Description |
|----------|-------------|
| [core-value-user-owned-login.md](01-vision/core-value-user-owned-login.md) | User-owned identity |
| [business-principles.md](01-vision/business-principles.md) | Principles |
| [fully-decentralized.md](01-vision/fully-decentralized.md) | Decentralization model |

---

## 05 — Business · 06 — Strategy · 07 — Penguin

See [05-business/business-plan.md](05-business/business-plan.md), [06-strategy/](06-strategy/), [07-penguin-products/](07-penguin-products/).

---

## Reading paths

### Engineer / builder (~2 hours)

1. **[08-developer/README.md](08-developer/README.md)**
2. [mvp-strategy.md](03-use-cases/mvp-strategy.md)
3. [specs/tech-stack.md](08-developer/specs/tech-stack.md)
4. [integration/sdk-guide.md](08-developer/integration/sdk-guide.md)

### Business leader (~1 hour)

1. [core-value-user-owned-login.md](01-vision/core-value-user-owned-login.md)
2. [business-principles.md](01-vision/business-principles.md)
3. [sustainable-funding.md](05-business/sustainable-funding.md)

### Enterprise IT (~1.5 hours)

1. [specs/enterprise-identity-spec.md](08-developer/specs/enterprise-identity-spec.md)
2. [architecture/dig-enterprise-transformation.md](08-developer/architecture/dig-enterprise-transformation.md)
3. [permission-platform.md](02-product/permission-platform.md)

---

## Executive summary

| Topic | Status |
|-------|--------|
| **Build** | [08-developer/](08-developer/) + repo [README](../README.md) |
| **POC** | Login with PEGIN — [mvp-strategy.md](03-use-cases/mvp-strategy.md) |
| **Roadmap** | [roadmap.md](03-use-cases/roadmap.md) |
| **Claims** | |
