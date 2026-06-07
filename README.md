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

Built with 🐧 — *Waddle in, authenticated out.*
