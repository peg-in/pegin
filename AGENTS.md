# AGENTS.md — PEGIN repository guide for AI coding agents

## Project

PEGIN (Penguin Gateway Identity): decentralized SSO on Chia + DIG. POC = passkey login → Chia DID → JWT.

## Knowledge base (read before large doc dives)

| Resource | When to use |
|----------|-------------|
| [docs/ai/CONTEXT.md](docs/ai/CONTEXT.md) | Compact facts (~1–2k tokens) |
| [docs/ai/manifest.json](docs/ai/manifest.json) | Document catalog + summaries |
| [docs/ai/chunks.jsonl](docs/ai/chunks.jsonl) | RAG / embeddings source |
| [docs/llms.txt](docs/llms.txt) | Standard doc index ([llmstxt.org](https://llmstxt.org/)) |
| [docs/README.md](docs/README.md) | Full human/AI navigation |
| [CLAUDE.md](CLAUDE.md) | Stack, crates, dependencies |

**Do not ingest:** `docs/wiki/PEGIN_Wiki_Knowledge_Base.html`, `docs/wiki/PEGIN_Wiki.md` (duplicate), `docs/wiki/_archive/`.

Regenerate AI artifacts after doc edits: `python3 scripts/generate-ai-knowledge-base.py`

## Implementation

- **Primary docs:** [docs/08-developer/README.md](docs/08-developer/README.md)
- Spec 1: [docs/08-developer/specs/tech-stack.md](docs/08-developer/specs/tech-stack.md)
- SDK: [docs/08-developer/integration/sdk-guide.md](docs/08-developer/integration/sdk-guide.md)
- MVP scope: [docs/03-use-cases/mvp-strategy.md](docs/03-use-cases/mvp-strategy.md)
- Planned layout: `pegin-core/`, `pegin-contracts/`, `@pegin/sdk/`, etc. (see CLAUDE.md)

## Conventions

- Rust (Axum) + Rue contracts + TypeScript SDK; Tauri v2 for desktop pattern (like Sage).
- Ship POC feature first; expand protocols in order: WebAuthn → OIDC → SAML → …
