# PEGIN — Penguin Gateway Identity

Fully decentralized SSO on **Chia** + **DIG Network**. Passkey login anchored to a Chia DID. No passwords, no seed phrases in the user flow.

---

## Documentation

**[docs/README.md](docs/README.md)** — full map by role.

| Role | Start here |
|------|------------|
| **Everyone** | [Manifest](docs/01-vision/pegin-manifest.md) → [How we work](docs/09-how-we-work/README.md) → [Architecture](docs/10-architecture/README.md) |
| **Programmers** | Then [08-developer/](docs/08-developer/README.md) + [Spec 1](docs/04-technical/specs/tech-stack.md) |

Also: [CLAUDE.md](CLAUDE.md) · [llms.txt](llms.txt)

---

## POC: Login with PEGIN

```
User (passkey) → PEGIN Service (Rust/Axum) → Chia DID + DIG → JWT to app
```

**Success criteria:** register &lt; 5s, login &lt; 1s, DID on testnet, Chrome/Safari/Firefox.

---

## Planned workspace

| Crate / package | Role |
|-----------------|------|
| `pegin-core/` | DID, WebAuthn, JWT (Rust/Axum) |
| `pegin-protocols/` | OIDC, SAML, OAuth, SCIM |
| `pegin-contracts/` | Rue smart contracts |
| `pegin-cli/` | Developer CLI |
| `@pegin/sdk/` | Login button + browser WebAuthn |
| `pegin-dashboard/` | React admin UI (Tauri v2, Sage pattern) |

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
