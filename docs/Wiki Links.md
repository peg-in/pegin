# Documentation

PEGIN (Penguin Gateway Identity) is decentralized SSO on Chia + DIG.

| You are… | Start here |
|----------|------------|
| **Any team member** | [01-vision/pegin-manifest.md](01-vision/pegin-manifest.md) → [09-how-we-work/](09-how-we-work/team-how-we-work.md) → [10-architecture/](10-architecture/architecture-overview.md) |
| **Programmer** | Above, then [08-developer/](08-developer/developer-documentation.md) + [04-technical/specs/](04-technical/specs/specifications-index.md) |

---

## By folder

| Folder | Audience | Contents |
|--------|----------|----------|
| [01-vision/](01-vision/) | Everyone | Principles, manifest, decentralization story |
| [09-how-we-work/](09-how-we-work/) | Everyone | Culture, GitHub, Basecamp (paid SaaS) |
| [10-architecture/](10-architecture/) | Everyone | System design — Chia, DIG, PePP, products |
| [02-product/](02-product/) | Product / GTM | PePP narrative, ecosystem |
| [03-use-cases/](03-use-cases/) | Everyone | Roadmap, MVP, differentiators |
| [04-technical/specs/](04-technical/specs/) | Engineers | Spec 1 & 2 (implementation contracts) |
| [08-developer/](08-developer/) | **Programmers only** | `environment/`, `engineering/`, `integration/` |
| [05-business/](05-business/) | Business | Plans, funding, moat |
| [06-strategy/](06-strategy/) | Strategy | Ecosystem position |
| [07-penguin-products/](07-penguin-products/) | Product | Future product business plans |

```
docs/
├── README.md                    Docs hub (this file)
├── 01-vision/                   Why PEGIN
├── 09-how-we-work/
│   └── team-how-we-work.md      Culture & collaboration index
├── 10-architecture/
│   └── architecture-overview.md System design index
├── 04-technical/
│   ├── technical-hub.md         Specs entry
│   └── specs/
│       └── specifications-index.md
├── 08-developer/
│   ├── developer-documentation.md   Programmer hub
│   ├── environment/environment-and-tooling-index.md
│   ├── engineering/engineering-standards-index.md
│   └── integration/integration-overview.md
├── 02-product/ 03-use-cases/ 05-business/ …
└── wiki/wiki-overview.md        HTML wiki + consolidated MD
```

Hub pages use **descriptive filenames** (not `README.md`) so Obsidian, search, and AI tools can tell files apart. Only **`docs/README.md`** and the **repo root `README.md`** keep the conventional name.

---

## Quick links

| Goal | Document |
|------|----------|
| Principles anchor | [pegin-manifest.md](01-vision/pegin-manifest.md) |
| How we work | [09-how-we-work/team-how-we-work.md](09-how-we-work/team-how-we-work.md) |
| Project architecture | [10-architecture/architecture-overview.md](10-architecture/architecture-overview.md) |
| Write code | [08-developer/developer-documentation.md](08-developer/developer-documentation.md) |
| Spec 1 / Spec 2 | [04-technical/specs/](04-technical/specs/specifications-index.md) |
| POC scope | [mvp-strategy.md](03-use-cases/mvp-strategy.md) |

---

## AI / knowledge base

| Resource | Use |
|----------|-----|
| [ai/CONTEXT.md](ai/CONTEXT.md) | Compact LLM context |
| [../llms.txt](../llms.txt) | Doc index |

Regenerate: `python3 scripts/generate-ai-knowledge-base.py` (when script exists)

---

## Reading paths

### Any team member (~45 min)

1. [pegin-manifest.md](01-vision/pegin-manifest.md)  
2. [09-how-we-work/how-we-work.md](09-how-we-work/how-we-work.md)  
3. [10-architecture/application-architecture.md](10-architecture/application-architecture.md) (overview)

### Engineer (~2 hours)

1. Team path above  
2. [08-developer/environment/developer-environment.md](08-developer/environment/developer-environment.md)  
3. [04-technical/specs/tech-stack.md](04-technical/specs/tech-stack.md)  
4. [08-developer/integration/sdk-guide.md](08-developer/integration/sdk-guide.md)

### Enterprise IT (~1.5 hours)

1. [04-technical/specs/enterprise-identity-spec.md](04-technical/specs/enterprise-identity-spec.md)  
2. [10-architecture/dig-enterprise-transformation.md](10-architecture/dig-enterprise-transformation.md)  
3. [permission-platform.md](02-product/permission-platform.md)

---

## Executive summary

| Topic | Status |
|-------|--------|
| **Culture** | [09-how-we-work/](09-how-we-work/) |
| **Architecture** | [10-architecture/](10-architecture/) |
| **Code** | [08-developer/](08-developer/) + [04-technical/specs/](04-technical/specs/) |
| **POC** | [mvp-strategy.md](03-use-cases/mvp-strategy.md) |
