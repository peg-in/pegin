# PEGIN — AI system context (compact)

Use this file as a system prompt or project context when the full knowledge base is too large. For RAG, prefer `docs/ai/chunks.jsonl`. For the full catalog, see `docs/ai/manifest.json` or `llms.txt`.

**Evidence policy:** Pre-POC. Do not state dollar savings, adoption %, or customer names as facts unless backed by a dated pilot or contract.

## What PEGIN is

**PEGIN** (Penguin Gateway Identity) is a fully decentralized SSO on the **Chia blockchain** and **DIG Network**. Users authenticate with **passkeys** (WebAuthn); identity is anchored to a **Chia DID**; apps receive a signed **JWT**.

## POC scope (build this first)

Single feature: **"Login with PEGIN"** — register/login with device biometrics, DID on Chia testnet, JWT to relying party. Target: register &lt; 5s, login &lt; 1s. Protocol expansion: WebAuthn → OIDC → SAML → …

Full plan: `docs/03-use-cases/roadmap.md`.

## Planned codebase

| Component | Stack | Role |
|-----------|-------|------|
| `pegin-core` | Rust, Axum | DID, WebAuthn, JWT |
| `pegin-contracts` | Rue → CLVM | DID, credential NFT, recovery, revoke |
| `pegin-protocols` | Rust | OIDC, SAML, OAuth, SCIM |
| `@pegin/sdk` | TypeScript | Login button, browser WebAuthn |
| `pegin-dashboard` | React, Tauri v2 | Admin UI (Sage wallet pattern) |

Key deps: `chia-wallet-sdk`, `passkey`, `rue-cli`, DIG storage (`dig-l2-storage`).

## Architecture layers

```
User (passkey) → PEGIN Service (Rust) → Chia DID + Rue contracts
 ↓
 DIG Network (permissions, audit, user data) — Phase 2+
 Chia anchors store updates only — no heavy audit payloads on chain
```

**Identity model:** User owns DID; employer grants revocable **permissions** (PePP, not in POC). **Audit:** append on DIG; on-chain = store commitment when DIG updates.

## Product surface

1. **SSO engine** — passkey login, OIDC/SAML federation (phased) 
2. **Permission platform (PePP)** — Phase 2; see `docs/08-developer/permissions/permission-data-model.md` 
3. **Penguin Gateway / Vault** — future; `07-penguin-products/`

## Business model (hypothesis)

- Open source core; revenue ideas in `sustainable-funding.md` (DIG, SLA, services) — **not validated**. 
- Enterprise value must be measured in pilots before it is claimed externally.

## Chia ecosystem position

Complementary to **Chia Network Inc.** Uses xch-dev toolchain (`chia-wallet-sdk`, `rue`, `sage` as reference app architecture).

## Topic doc map (canonical paths)

| Topic | Path |
|-------|------|
| **Developer docs (primary)** | `docs/08-developer/README.md` |
| Tech stack (Spec 1) | `docs/08-developer/specs/tech-stack.md` |
| SDK guide | `docs/08-developer/integration/sdk-guide.md` |
| On-chain architecture | `docs/08-developer/architecture/on-chain-architecture.md` |
| Enterprise SSO (Spec 2) | `docs/08-developer/specs/enterprise-identity-spec.md` |
| Roadmap | `docs/03-use-cases/roadmap.md` |
| POC | `docs/03-use-cases/mvp-strategy.md` |
| xch-dev | https://github.com/xch-dev · https://docs.xch.dev |
| slot-machine / XCHandles | https://github.com/Yakuhito/slot-machine · https://docs.xchandles.com |
| Full index | `docs/README.md` |

## Do not use for RAG

- `docs/wiki/PEGIN_Wiki_Knowledge_Base.html` 
- `docs/wiki/PEGIN_Wiki.md` (duplicate; may contain outdated ROI figures) 
- `docs/wiki/_archive/`

## Glossary

| Term | Meaning |
|------|---------|
| DID | Decentralized Identifier on Chia |
| DIG | Decentralized storage / incentive layer |
| DSSO | Decentralized SSO |
| PePP | PEGIN Permission Platform |
| Rue | Typed language compiling to Chialisp/CLVM |