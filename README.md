# PEGIN — Penguin Gateway Identity

Fully decentralized SSO on **Chia** + **DIG Network**. Passkey login anchored to a Chia DID. No passwords, no seed phrases in the user flow.

---

## Developer documentation (start here)

**[docs/08-developer/README.md](docs/08-developer/README.md)** — primary specs, architecture, and integration guides.

| Step | Document |
|------|----------|
| 1 | [Spec 1 — tech stack](docs/08-developer/specs/tech-stack.md) |
| 2 | [SDK integration](docs/08-developer/integration/sdk-guide.md) |
| 3 | [On-chain architecture](docs/08-developer/architecture/on-chain-architecture.md) |
| POC scope | [mvp-strategy.md](docs/03-use-cases/mvp-strategy.md) |

Also: [CLAUDE.md](CLAUDE.md) · [docs/README.md](docs/README.md) · [llms.txt](llms.txt) · [docs/ai/CONTEXT.md](docs/ai/CONTEXT.md)

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
| **[docs/08-developer/](docs/08-developer/)** | **Primary** — build & integrate |
| [docs/03-use-cases/](docs/03-use-cases/) | Roadmap, MVP |
| [docs/02-product/](docs/02-product/) | Product narrative |
| [docs/01-vision/](docs/01-vision/) | Philosophy |
| [docs/05-business/](docs/05-business/) | Business plans |

Built with 🐧 — *Waddle in, authenticated out.*
