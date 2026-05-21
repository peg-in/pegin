# Linting and formatting

> **Hub:** [../README.md](../README.md) · **Architecture:** [../../10-architecture/architecture-overview.md](../../10-architecture/architecture-overview.md) · **Tests:** [test-architecture.md](test-architecture.md) · **Structure:** [../architecture/application-architecture.md](../architecture/application-architecture.md)

PEGIN aims for **readable, consistent code** without treating [*Clean Code*](https://www.pearson.com/en-us/subject-catalog/p/clean-code-a-handbook-of-agile-software-craftsmanship/P200000003285) as a religion. Formatters and linters enforce the mechanical rules; the principles below guide reviews and design.

**TDD is not required.** Tests are still expected — see [test-architecture.md](test-architecture.md).

---

## Tooling (when the workspace exists)

| Stack | Format | Lint | Config location (planned) |
|-------|--------|------|---------------------------|
| **Rust** | `rustfmt` | `clippy` (`cargo clippy --workspace -- -D warnings`) | `rustfmt.toml`, `clippy.toml` at repo root |
| **TypeScript** | Prettier | ESLint (`@typescript-eslint`) | `packages/sdk/`, `pegin-dashboard/` |
| **Rue** | `rue fmt` (if available in rue-cli) | Compiler + `rue-cli build` | `pegin-contracts/` |
| **Markdown** | Editor wrap optional | `markdownlint` optional in CI | `docs/` |

**CI gate (target):** `cargo fmt --check`, `clippy`, `cargo test --workspace`, `npm run lint` / `npm test` for TS crates touched by the PR.

---

## Clean Code principles (pragmatic bar)

These are **guidelines for humans**, not every rule enforced by a machine. Prefer clarity over cleverness.

### Names (CC: Meaningful Names)

| Do | Avoid |
|----|--------|
| Names reveal intent: `GrantId`, `anchor_store_update`, `RegisterPasskeyCommand` | `data`, `tmp`, `handleStuff` |
| Searchable terms for domain concepts | Single-letter except tiny scopes (`i`, `e`) |
| One word per concept (`grant` vs `permission` — pick per bounded context) | `manager`, `helper`, `util` without scope |

### Functions (CC: Functions)

| Do | Avoid |
|----|--------|
| Small functions; one level of abstraction per function | 200-line handlers mixing HTTP + Chia + DIG |
| Few arguments (0–3); use a command struct if more | Long parameter lists |
| No hidden side effects; name `save_and_anchor` if it does both | `checkPassword` that also initializes sessions |
| Prefer pure domain functions testable without I/O | Blockchain calls inside domain entities |

### Comments (CC: Comments)

| Do | Avoid |
|----|--------|
| Explain **why** (policy, Chia quirk, security constraint) | Restate what the code already says |
| Link to spec sections or issues for non-obvious rules | Commented-out dead code (delete it) |
| Public API doc comments on traits and ports | Every line narrated |

### Formatting (CC: Formatting)

| Do | Avoid |
|----|--------|
| Let `rustfmt` / Prettier decide braces and line breaks | Manual alignment columns that fight the formatter |
| Vertical density that fits ~80–100 col editors | Dense blocks with no blank lines between ideas |
| Consistent import order (rustfmt groups) | Random `use` ordering per file |

### Objects & boundaries (CC: Objects and Data Structures, Boundaries)

| Do | Avoid |
|----|--------|
| Rich domain types in `pegin-*` context crates | Anemic structs + 500-line “service” god classes |
| Wrap Chia/DIG/SQL behind traits ([application architecture](../architecture/application-architecture.md)) | `chia-wallet-sdk` types leaking into domain |
| Validate at boundaries (HTTP, DIG JSON ingest) | Trusting external input in domain |

### Error handling (CC: Error Handling)

| Do | Avoid |
|----|--------|
| `Result` + typed errors (`AppError`, `DomainError`) | `unwrap()` / `expect()` in library or handler code |
| Context on infrastructure failures (`failed to anchor DID: …`) | Returning raw strings to HTTP clients |
| Fail fast on invalid invariants at construction | `Option` when the absence is exceptional |

### Classes & modules (CC: Classes, SRP)

| Do | Avoid |
|----|--------|
| One reason to change per module/crate | `pegin-core` that does WebAuthn + SAML + DIG |
| Keep `pegin-protocols` separate from `pegin-identity` | Circular crate dependencies |

### What we do **not** require

| Clean Code idea | PEGIN stance |
|-----------------|--------------|
| **TDD everywhere** | Not required; test harness early — [test-architecture.md](test-architecture.md) |
| **Zero comments** | Comments welcome when they carry “why” |
| **Max function length (e.g. 10 lines)** | Small is good; don’t split readable 40-line flows artificially |
| **No duplication at any cost** | DRY in domain; some repetition OK in tests and mappers |

---

## Review checklist (lightweight)

Before merge:

1. `cargo fmt` / Prettier applied  
2. No new `unwrap` in production paths  
3. Names match domain language ([permission-data-model](../permissions/permission-data-model.md))  
4. New behavior has a test at the **lowest** appropriate level ([test-architecture.md](test-architecture.md))  
5. Secrets not committed (`.env` in [.gitignore](../../../.gitignore))

---

## Related

- [test-architecture.md](test-architecture.md)  
- [application-architecture.md](../architecture/application-architecture.md)  
- [tech-stack.md](../../04-technical/specs/tech-stack.md)

*Engineering standards v1.0 · May 2026*
