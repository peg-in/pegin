# AI coding tools — optional strategy & privacy

> **Hub:** [environment-and-tooling-index.md](environment-and-tooling-index.md) · **Setup:** [developer-environment.md](developer-environment.md) · **All roles (docs):** [collaboration-tooling.md](../../09-how-we-work/collaboration-tooling.md) · **Repo context:** [AGENTS.md](../../../AGENTS.md) · [CONTEXT.md](../../ai/CONTEXT.md)

**Optional** guidance for programmers and leads. PEGIN does **not** require AI tools. When you use them, follow this strategy and privacy policy so customer data, keys, and strategy stay under control.

**Not legal advice.** Customer contracts and GDPR may impose stricter rules — escalate to counsel for regulated pilots.

---

## Strategy at a glance

| Tier | Best for | Data leaves your machine? |
|------|----------|---------------------------|
| **A — Local agents** | Architecture spikes, `docs/`, refactors with secrets risk | **No** (if configured correctly) |
| **B — Hybrid** | Daily coding: local for sensitive files, cloud for boilerplate | **Partial** — you choose per task |
| **C — Cloud IDE agents** | Fast iteration when no customer/secret context | **Yes** — vendor inference |

Pick a tier per task, not once per career. Default for **PEGIN repo work with customer or key material:** **Tier A or B**.

---

## Tier A — Local AI agents (recommended for sensitive work)

Run an **open-source inference stack** on your machine or on a **company-controlled** Hetzner VM (not a public chat UI). Examples (May 2026 — use what your team standardizes):

| Component | Examples | Role |
|-----------|----------|------|
| **Runtime** | [Ollama](https://ollama.com/), llama.cpp, vLLM | Host models locally |
| **Models** | Llama, Mistral, Qwen, DeepSeek Coder (check license) | Code + reasoning |
| **IDE / agent** | Continue, Cody (offline mode), custom CLI agents, IDE plugins pointed at `localhost` | Edit with repo context |
| **Strategy & docs** | Same stack + `docs/`, `AGENTS.md`, `CONTEXT.md` | Roadmap drafts, ADR spikes, Basecamp prep **without cloud upload** |

### Company development strategy on local AI

Use local agents for work that should **not** train vendor models or leave the EU:

- Comparing architecture options against [10-architecture/](../../10-architecture/architecture-overview.md)
- Drafting sections of `docs/` or pilot worksheets (human review before merge)
- Exploring Rue/Chia integration ideas against [tech-stack.md](../../04-technical/specs/tech-stack.md)
- Internal “what if we shipped X in Phase 2?” — output stays on your machine or Hetzner

**Workflow:** load repo context from git (not a zip of customer exports) → prompt with specific file paths → paste **summaries** into Basecamp only after redacting customer names and metrics unless approved.

### Local agent checklist

- [ ] Model and runtime are **open source** or team-approved; license fits commercial use  
- [ ] **No** customer DB dumps, production logs, or `.env` in prompts  
- [ ] **Telemetry off** where the tool allows it  
- [ ] Output reviewed like any other PR — you own merges  

---

## Tier B — Hybrid

Common pattern for day-to-day engineering:

| Use local (Tier A) | Cloud agent OK (Tier C) |
|--------------------|-------------------------|
| Auth, crypto, DID, grant/audit code | Generic tests, fmt fixes, public API stubs |
| Files touching customer pilot details | Open-source dependency upgrades |
| Anything under `deploy/` secrets or operator config | Docs for already-public concepts |

**Rule:** if the buffer might contain **credentials, customer PII, or unreleased pilot numbers**, use Tier A only.

---

## Tier C — Cloud IDE agents (optional)

Cloud tools (e.g. **Cursor Agent**, GitHub Copilot, Claude Code, Codex) are allowed for PEGIN work when **privacy settings** are understood and scope is limited.

| Practice | Why |
|----------|-----|
| Enable **privacy / no-training** modes when the vendor offers them | Reduces retention and model training risk |
| Use **@ file** references to `docs/` and `AGENTS.md` instead of pasting whole repos | Smaller, controlled context |
| **No** customer exports, production `.env`, or seed phrases in prompts | Non-negotiable |
| Prefer **small PRs** from agent output | Easier human review |

Cloud agents are **not** a project-paid SaaS line item ([infrastructure-and-tooling-principles.md](infrastructure-and-tooling-principles.md)) — subscriptions are typically **individual** unless the company later adopts a team plan via explicit decision.

---

## Privacy policy (PEGIN contributors)

### Never send to any AI (local or cloud)

| Data | Reason |
|------|--------|
| Private keys, mnemonics, API keys, `.env` | Irreversible exposure risk |
| Production user data, auth logs with PII | GDPR / customer trust |
| Unreleased pilot metrics presented as fact | Evidence policy |
| Customer contracts or named logos without approval | Confidentiality |
| Full database dumps or DIG store exports | Bulk PII |

### Allowed with care

| Data | Condition |
|------|-----------|
| **Public** `docs/`, specs, manifest | Primary context — prefer repo paths over paste |
| **Synthetic** test fixtures | No real user emails or IDs |
| **Redacted** error messages | Strip hostnames, tokens, user ids |
| **Open-source** dependency code | Standard stack overflow–level help |

### Local vs cloud retention

| | Local (Tier A) | Cloud (Tier C) |
|---|----------------|----------------|
| **Prompt retention** | You control disk / VM | Vendor policy — read yearly |
| **Training on our code** | None if offline | Disable if offered; assume unless stated |
| **EU data residency** | Your machine / Hetzner EU | Check vendor DPA and region settings |

### Human accountability

1. **You** are responsible for merged code and docs — not the model.  
2. **Security-sensitive** changes (WebAuthn, JWT, DID, grants) require **human review** — no agent-only merge.  
3. **Incidents:** if secrets were pasted into a cloud tool, rotate credentials and report per team process.

---

## Suggested team defaults (optional adoption)

| Role | Suggested default |
|------|-------------------|
| **Lead / architect** | Tier A for strategy and architecture docs |
| **Implementer** | Tier B — local for `pegin-core` / auth paths |
| **Docs / product** | Tier A or chat with **no** customer attachments |
| **CI** | No cloud AI in pipelines unless approved; secrets in GitHub Actions only |

Document your chosen stack in a **personal or team Basecamp note** — not required in repo unless the team adopts one standard.

---

## Repo context for agents (all tiers)

Load in this order:

1. [pegin-manifest.md](../../01-vision/pegin-manifest.md)  
2. [CONTEXT.md](../../ai/CONTEXT.md)  
3. Relevant `docs/10-architecture/` and `docs/04-technical/specs/` files  
4. [AGENTS.md](../../../AGENTS.md) for coding agents  

Regenerate RAG artifacts after large doc moves: `python3 scripts/generate-ai-knowledge-base.py` (when script exists).

**Do not** point agents at [PEGIN_Wiki.md](../../wiki/PEGIN_Wiki.md) as sole source — prefer canonical topic docs.

---

## Related

| Document | Topic |
|----------|--------|
| [developer-environment.md](developer-environment.md) | Containers, toolchain |
| [collaboration-tooling.md](../../09-how-we-work/collaboration-tooling.md) | AI for docs (all roles) |
| [infrastructure-and-tooling-principles.md](infrastructure-and-tooling-principles.md) | Paid SaaS = GitHub + Basecamp only |
| [how-we-work.md](../../09-how-we-work/how-we-work.md) | Small PRs, healthy pace |

*Optional policy v1.0 · May 2026 · Adopt by team agreement; revise via PR labeled `ai-policy`.*
