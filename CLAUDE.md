# PEGIN — Monorepo Layout (AI Development Guide)

PEGIN = Penguin Gateway Identity — decentralized SSO anchored to Chia blockchain DIDs.

## Repository structure

```
pegin/
├── crates/                  # Rust workspace members
│   ├── pegin-domain/        # Shared kernel: Did, Username, Sub, AppError — no external Chia deps
│   ├── pegin-identity/      # Bounded context: IdentityStore + PasskeyAuthenticator traits
│   ├── pegin-wallet/        # Use cases: CreateAccount, SignJwt, AssertPasskey
│   ├── pegin-infrastructure/# Adapters: ChiaGateway (coinset.org), LocalProfileStore
│   └── pegin-testing/       # Dev harness: chia-sdk-test helpers, domain fixtures
├── apps/
│   ├── mini/                # Tauri v2 desktop shell (Rust core + React UI, pattern from Sage)
│   └── demo-web/            # Vite + React demo relying party
├── packages/
│   └── sdk/                 # @pegin/sdk — "Login with PEGIN" button + PeginSession
├── contracts/               # Rue smart contracts (Step 2+, empty for now)
├── docs/                    # Submodules: pegin-wiki, pegin-issues
├── Cargo.toml               # Rust workspace root
├── package.json             # pnpm workspace root
└── pnpm-workspace.yaml      # pnpm workspace config
```

## Layering rules

- `pegin-domain` has **no** Chia, DIG, or framework imports — pure value objects and errors.
- `pegin-identity` depends on `pegin-domain` only; exposes traits (no impl).
- `pegin-wallet` depends on `pegin-domain` + `pegin-identity`; orchestrates use cases.
- `pegin-infrastructure` depends on `pegin-domain` + `pegin-identity`; holds all external I/O.
- `pegin-testing` is **dev-dependency only** — never import from production crates.

## Module structure (within every crate and package)

Every Rust crate and TypeScript package uses the same internal folder layout:

```
src/
├── shared/          # only code used by 2+ modules — if one module needs it, keep it there
└── modules/
    └── <name>/
        ├── mod.rs           (Rust) / index.ts (TS)  — declares submodules, re-exports public surface only
        ├── <name>.service.rs/ts    — business logic and use cases
        ├── <name>.repository.rs/ts — data access (storage, network, external APIs)
        ├── <name>.controller.rs/ts — entry points for HTTP / Tauri IPC (only where applicable)
        ├── <name>.helper.rs/ts     — pure functions local to this module
        ├── dto.rs/ts               — types that cross the module boundary (commands, responses)
        └── entities.rs/ts          — domain types that live inside this module
```

**Rules:**

- `shared/` only for things used by two or more modules. One user → stays in that module.
- Modules never import from sibling modules directly. Go through their public entry point.
- Not every file is required — a small module may only have `mod.rs`, `entities.rs`, and `<name>.service.rs`.
- `dto.rs` = types the caller passes in or receives back. `entities.rs` = types internal to the module.
- Controllers only exist in crates/packages that handle external requests (HTTP, Tauri IPC).

**Rust `shared/` folder contents:**

| File | What goes there |
|------|----------------|
| `shared/error.rs` | Cross-module error types |
| `shared/types.rs` | Newtypes or enums referenced by multiple modules |
| `shared/helpers.rs` | Pure utility functions used across modules |

**TypeScript `shared/` folder contents:**

| Folder | What goes there |
|--------|----------------|
| `shared/types/` | TypeScript interfaces / types used by multiple modules |
| `shared/lib/` | Pure helper functions (no domain knowledge, no side effects) |
| `shared/api/` | Typed HTTP / WebSocket client wrappers |

## Key commands

```bash
# Rust
cargo build --workspace          # build all Step 1 crates
cargo test --workspace           # run all tests
cargo fmt --all                  # format all Rust code
cargo fmt --all -- --check       # check formatting (used in pre-commit)
cargo clippy --workspace --all-targets -- -D warnings  # lint (used in pre-commit)

# TypeScript
pnpm install                     # install all packages
pnpm build                       # build all packages and apps

# Tauri (apps/mini)
pnpm --filter @pegin/mini dev    # launch desktop dev mode
```

## Pre-commit hooks (Rust quality gates)

The repo uses [pre-commit](https://pre-commit.com/) to enforce `rustfmt` and `clippy` before every commit.

**Install once after cloning:**

```bash
pip install pre-commit   # or: pipx install pre-commit
pre-commit install
```

**What the hooks run:**

| Hook | Command | Blocks commit on |
|------|---------|-----------------|
| `cargo-fmt` | `cargo fmt --all -- --check` | Non-standard formatting |
| `cargo-clippy` | `cargo clippy --workspace --all-targets -- -D warnings` | Any clippy warning or error |

**Run manually against all files:**

```bash
pre-commit run --all-files
```

**Lint config files:**

| File | Purpose |
|------|---------|
| `rustfmt.toml` | Line width (100), edition, newline style |
| `clippy.toml` | MSRV (1.80), complexity threshold, allow unwrap/expect in tests |
| `Cargo.toml [workspace.lints]` | Lint levels: `unsafe_code = deny`, `unwrap_used = deny`, `pedantic = warn` |
| `.cargo/config.toml` | `rustflags = ["-D", "warnings"]` — treats rustc warnings as errors |

**Rules:**

- `unwrap()` in production code is **always a hard error** (clippy `unwrap_used = deny`).  
  Test code (`#[cfg(test)]` and `pegin-testing`) is exempt via `allow-unwrap-in-tests = true` in `clippy.toml`.
- `#[allow(clippy::...)]` suppressions must include a comment explaining why.

## Dependency pins

All Rust workspace dependencies are pinned in the root `Cargo.toml [workspace.dependencies]`.
TypeScript packages use pnpm workspace protocol (`workspace:*`) for cross-package refs.

## Structural principles

Before adding any file, crate, or package, read:

**`docs/pegin-wiki/10-architecture/project-structure-principles.md`**

It defines:
- **DDD** — which Rust crate code belongs to, dependency direction table, compiler-enforced rules
- **FSD** — TypeScript layer hierarchy, folder layouts for `packages/sdk` / `apps/mini` / `apps/demo-web`, five isolation rules
- **DOD** — flat structs vs nested option soup, normalized stores, pure selectors
- Checklist for new modules and code review structural gates

## Documentation

- Architecture overview: `docs/pegin-wiki/10-architecture/architecture-overview.md`
- Structure principles: `docs/pegin-wiki/10-architecture/project-structure-principles.md`
- Tech stack + dep pins: `docs/pegin-wiki/04-technical/specs/tech-stack.md`
- Issues / tickets: `docs/pegin-issues/issues/`
