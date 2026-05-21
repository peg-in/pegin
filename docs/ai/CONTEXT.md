# PEGIN — AI system context (compact)

Use this file as a system prompt or project context when the full knowledge base is too large. For RAG, prefer `docs/ai/chunks.jsonl`. For the full catalog, see `docs/ai/manifest.json` or `llms.txt`.

**Evidence policy:** Pre-POC. Do not state dollar savings, adoption %, or customer names as facts unless backed by a dated pilot or contract.

**Principles anchor:** `docs/01-vision/pegin-manifest.md` — **trust is the staple**; foundation & company **mottos** (meeting-free, library days, no drama); E1–E11, P1–P14, W1–W11, B1–B6.

## What PEGIN is

**PEGIN** (Penguin Gateway Identity) is decentralized SSO on **Chia + DIG**. **Chia** anchors DID/vault and DIG store roots; **DIG** is the **application layer** (identity, audit, app/service data — not a central PEGIN database). **Wallet** = decentralized IdP (JWT for websites). Passkey login; blockchain hidden in UX. Apps receive **JWT**; user-owned data targets **DIG stores**.

## MVP scope (two steps — not DIG-first)

**Step 1:** **One button** (best UI, **no redirect**), instant if JWT valid, else one-tap Face ID. Account in app (username + DID); JWT `preferred_username`. See `user-facing-ux-principles.md`.

**Step 2:** **Vault** (chia-wallet-sdk / Rigidity Rue) — recover DID on **multiple devices**; **seed phrase** (recovery only) + **passkey** (login + re-bind).

**Post-MVP:** DIG application layer, email guardian, Chia Signer, PePP.

Detail: `docs/03-use-cases/mvp-strategy.md` · `docs/10-architecture/mini-wallet-and-recovery-vault.md` · roadmap: `docs/03-use-cases/roadmap.md`.

## Planned codebase

| Component | Stack | Role |
|-----------|-------|------|
| `pegin-wallet` | Rust, chia-wallet-sdk | Mini wallet: DID + recovery vault txs |
| `pegin-auth` | Rust, Axum, passkey | WebAuthn, JWT, OIDC, faucet |
| `pegin-contracts` | Rue → CLVM | DID + `pegin_recovery_vault.rue` |
| `pegin-mini` | Tauri v2 + React | Client shell (Sage pattern) |
| `@pegin/sdk` | TypeScript | Login button, browser WebAuthn |
| `pegin-protocols` | Rust | OIDC, SAML, … (post-POC) |

Key deps: `chia-wallet-sdk`, `passkey`, `rue-cli`, DIG storage (`dig-l2-storage`).

## Architecture layers

```
User (passkey) → PEGIN Service (Rust) → Chia DID + Rue contracts
 ↓
 DIG Network (permissions, audit, user data) — Phase 2+
 Chia anchors store updates only — no heavy audit payloads on chain
```

**Code structure:** Fowler layers + DDD crates — `docs/10-architecture/application-architecture.md`.  
**Quality:** Pragmatic Clean Code + rustfmt/clippy — `docs/08-developer/engineering/linting-and-formatting.md`. TDD not required; test harness early — `docs/08-developer/engineering/test-architecture.md`. **Culture:** remote, library days — `docs/09-how-we-work/how-we-work.md`. **Paid SaaS:** GitHub + Basecamp only; OSS stack; hosting Hetzner (early) → DIG; dev setup (Podman/Docker planned when code lands) — `docs/08-developer/environment/`. **AI (optional):** local agents for sensitive work; cloud IDE allowed with privacy rules — `docs/08-developer/environment/ai-coding-tools.md`.

**Identity model:** User owns DID; employer grants revocable **permissions** (PePP, not in POC). **Audit:** append on DIG; on-chain = store commitment when DIG updates.

## Product surface

1. **SSO engine** — passkey login, OIDC/SAML federation (phased) 
2. **Permission platform (PePP)** — Phase 2; see `docs/10-architecture/permission-data-model.md` 
3. **Penguin Gateway / Vault** — future; `07-penguin-products/`

## Business model (hypothesis)

- Open source core; revenue ideas in `sustainable-funding.md` (DIG, SLA, services) — **not validated**. 
- Enterprise value must be measured in pilots before it is claimed externally.

## Chia ecosystem position

Complementary to **Chia Network Inc.** Uses xch-dev toolchain (`chia-wallet-sdk`, `rue`, `sage` as reference app architecture).

## Topic doc map (canonical paths)

| Topic | Path |
|-------|------|
| **Docs hub** | `docs/README.md` |
| **How we work (all roles)** | `docs/09-how-we-work/README.md` |
| **Architecture (all roles)** | `docs/10-architecture/README.md` |
| **Programmer docs** | `docs/08-developer/README.md` |
| Tech stack (Spec 1) | `docs/04-technical/specs/tech-stack.md` |
| SDK guide | `docs/08-developer/integration/sdk-guide.md` |
| Existing apps + SSO protocols | `docs/08-developer/integration/existing-apps-and-sso-protocols.md` |
| On-chain architecture | `docs/10-architecture/on-chain-architecture.md` |
| Enterprise SSO (Spec 2) | `docs/04-technical/specs/enterprise-identity-spec.md` |
| Roadmap | `docs/03-use-cases/roadmap.md` |
| POC | `docs/03-use-cases/mvp-strategy.md` |
| Mini wallet + recovery vault | `docs/10-architecture/mini-wallet-and-recovery-vault.md` |
| Recovery (email guardian, Chia Signer) | `docs/10-architecture/recovery-vault-and-guardians.md` |
| DIG application layer | `docs/10-architecture/dig-as-application-layer.md` |
| xch-dev | https://github.com/xch-dev · https://docs.xch.dev |
| slot-machine / XCHandles | https://github.com/Yakuhito/slot-machine · https://docs.xchandles.com |
| Full index | `docs/README.md` |

## Do not use for RAG

- `docs/wiki/PEGIN_Wiki_Knowledge_Base.html` 
- `docs/wiki/PEGIN_Wiki.md` (duplicate; may contain outdated ROI figures) 

## Glossary

| Term | Meaning |
|------|---------|
| DID | Decentralized Identifier on Chia |
| DIG | Decentralized storage / incentive layer |
| DSSO | Decentralized SSO |
| PePP | PEGIN Permission Platform |
| Rue | Typed language compiling to Chialisp/CLVM |