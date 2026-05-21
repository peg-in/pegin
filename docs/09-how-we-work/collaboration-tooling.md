# Collaboration tooling

> **Hub:** [team-how-we-work.md](team-how-we-work.md) · **Culture:** [how-we-work.md](how-we-work.md) · **Programmers:** [developer-environment.md](../08-developer/environment/developer-environment.md)

How **everyone on the team** coordinates — whether you write code, run pilots, or own product. No Rust, no IDE setup here.

**Anchored in:** [pegin-manifest.md § Layer III](../01-vision/pegin-manifest.md#layer-iii--how-we-work).

---

## Paid project software (only two)

PEGIN intentionally pays for **two** services only. Everything else in the stack should be **open source** or **self-hosted** (see [infrastructure-and-tooling-principles.md](../08-developer/environment/infrastructure-and-tooling-principles.md)).

| Tool | Role | Who uses it |
|------|------|-------------|
| **[GitHub](https://github.com/)** | **Source of truth** — code, `docs/`, issues, PRs | Everyone |
| **[Basecamp](https://basecamp.com/)** | **Project management** — milestones, to-dos, pilots | Everyone |

**Not a project SaaS line item:** team email, chat, or personal AI tools — use whatever the team already has; keep decisions in GitHub/Basecamp.

**Hosting (not SaaS):** early VMs on **[Hetzner](https://www.hetzner.com/)**; production app data on **[DIG Network](https://github.com/DIG-Network)** — programmers: [developer-environment.md](../08-developer/environment/developer-environment.md).

---

## What goes where

| Content | Put it in |
|---------|-----------|
| Vision, principles, architecture | `docs/` (e.g. [01-vision/](../01-vision/), [10-architecture/](../10-architecture/)) + PR |
| Implementation specs | [04-technical/specs/](../04-technical/specs/) |
| Code, SDK, tests | GitHub — see [08-developer/](../08-developer/developer-documentation.md) |
| Sprint goals, launch checklist, pilot status | Basecamp |
| Security incident, legal, customer intro | Team email → then GitHub/Basecamp as needed |

**Rule:** If it must survive a personnel change, it belongs in **GitHub** (`docs/` or code). Basecamp is for coordination, not canonical truth.

---

## Writing and decisions (all roles)

- Prefer **async docs** over meetings — see [how-we-work.md](how-we-work.md).
- **Library days:** protect focus; no internal meetings on team library day.
- **Evidence:** Do not state dollar savings, customer names, or metrics as facts without a dated pilot — see [CONTEXT.md](../ai/CONTEXT.md).
- **Healthy pace:** No “ASAP” by default; protect focus and rest — see [how-we-work.md § Healthy team mindset](how-we-work.md#healthy-team-mindset-rework).

---

## Using AI safely (all roles)

PEGIN does not mandate one chat product. We **do** mandate **where truth lives**:

| Priority | Source |
|----------|--------|
| 1 | [pegin-manifest.md](../01-vision/pegin-manifest.md) |
| 2 | Topic docs under `docs/` (vision, architecture, product, business) |
| 3 | [docs/ai/CONTEXT.md](../ai/CONTEXT.md) |

**Rules:**

1. Do not invent ARR, customers, or performance numbers.
2. Do not put secrets in prompts (keys, customer data).
3. When AI helps draft `docs/`, a **human reviews** before merge.
4. For code changes, programmers follow [ai-coding-tools.md](../08-developer/environment/ai-coding-tools.md) (optional strategy & privacy) and [developer-environment.md](../08-developer/environment/developer-environment.md).

---

## GitHub (when you contribute docs or code)

| Step | Practice |
|------|----------|
| Branch | Short-lived branches off `main` |
| PR | Explain **why**; link Basecamp card when useful |
| Review | Human approval for non-trivial changes |
| Merge | No force-push to `main` |

Programmers: also see CI expectations in [08-developer/engineering/](../08-developer/engineering/).

---

## Quick reference

```
Canonical truth  → GitHub (docs/ + code)
Planning         → Basecamp (only paid PM)
Paid SaaS        → GitHub + Basecamp only
Hosting          → Hetzner (early) → DIG (target)
Architecture     → docs/10-architecture/
Code & SDK       → docs/08-developer/ (programmers)
```

---

## Related

| Document | Topic |
|----------|--------|
| [how-we-work.md](how-we-work.md) | Remote, library days, *Rework* |
| [developer-environment.md](../08-developer/environment/developer-environment.md) | Rust, Node, editors (programmers) |
| [10-architecture/architecture-overview.md](../10-architecture/architecture-overview.md) | System design for all roles |

*Last updated: May 2026*
