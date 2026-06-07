# PEGIN — Penguin Gateway Identity

Fully decentralized SSO on **Chia** + **DIG Network**. Passkey login anchored to a Chia DID. No passwords, no seed phrases in the user flow.

---

## Documentation

**[docs/README.md](Wiki%20Links.md)** — full map by role.

| Role            | **Start** here                                                                                                                                                        |
| --------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Everyone**    | [Manifest](docs/01-vision/pegin-manifest.md) → [How we work](docs/09-how-we-work/team-how-we-work.md) → [Architecture](docs/10-architecture/architecture-overview.md) |
| **Programmers** | Then [08-developer/](docs/08-developer/developer-documentation.md) + [Spec 1](docs/04-technical/specs/tech-stack.md)                                                  |

Also: [CLAUDE.md](CLAUDE.md) · [llms.txt](llms.txt)

---

## MVP Step 1 (build first)

**Wallet-as-IdP:** account app + DID + passkey → **JWT in wallet** → `@pegin/sdk` + demo site. No DIG-first; no vault until Step 2.

**Start coding:** [docs/08-developer/engineering/step1-implementation-bootstrap.md](docs/08-developer/engineering/step1-implementation-bootstrap.md)

**Success criteria:** silent session when JWT valid; one-button login; username in JWT; DID on testnet.

---

## Planned workspace (Step 1 subset)

| Path | Role |
|------|------|
| `crates/pegin-wallet/` | DID, passkey, JWT sign (core) |
| `crates/pegin-domain/` | Types, errors |
| `apps/mini/` | Tauri account app |
| `packages/sdk/` | `@pegin/sdk` — one button, no redirect |
| `contracts/` | Rue vault — **Step 2** |
| `pegin-protocols/` | OIDC/SAML — post-MVP |

---

## Documentation map

| Folder | Role |
|--------|------|
| [docs/01-vision/pegin-manifest.md](docs/01-vision/pegin-manifest.md) | Principles anchor |
| [docs/09-how-we-work/](docs/09-how-we-work/) | Team culture & tools (all roles) |
| [docs/10-architecture/](docs/10-architecture/) | System design (all roles) |
| [docs/08-developer/](docs/08-developer/) | Code, SDK, tests (programmers) |
| [docs/04-technical/specs/](docs/04-technical/specs/) | Spec 1 & 2 |
| [docs/03-use-cases/](docs/03-use-cases/) | Roadmap, MVP |
| [docs/05-business/](docs/05-business/) | Business plans |

---

## Prerequisites

Install these tools before working on the project:

| Tool | Version | Install |
|------|---------|---------|
| **Rust** | stable (≥ 1.80) | `curl https://sh.rustup.rs -sSf \| sh` |
| **Node.js** | 26.2.0 (pinned in `.nvmrc`) | [nvm](https://github.com/nvm-sh/nvm) or [mise](https://mise.jdx.dev/) |
| **pnpm** | ≥ 9 | `npm install -g pnpm` |
| **Tauri system deps** | — | [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) (Linux: `libgtk`, `libwebkit2gtk`, etc.) |

> **Node version:** run `nvm use` (or `mise use`) at the repo root to switch to the pinned version automatically.

---

## Getting started

```bash
# 1. Clone with submodules (docs)
git clone --recurse-submodules https://github.com/peg-in/pegin.git
cd pegin

# 2. Switch to the pinned Node version
nvm use        # or: mise use

# 3. Install TypeScript dependencies
pnpm install

# 4. Build Rust workspace
cargo build --workspace

# 5. Build TypeScript packages and apps
pnpm build
```

---

Built with 🐧 — *Waddle in, authenticated out.*
