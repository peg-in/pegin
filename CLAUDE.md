# PEGIN — Monorepo Layout (AI Development Guide)

PEGIN = Penguin Gateway Identity — decentralized SSO anchored to Chia blockchain DIDs.

## Where to find work (read this first)

**All tasks live in `docs/pegin-issues/issues/`** (an Obsidian-vault git submodule). Before
starting any feature or fix, open the matching issue file — it holds the user story, scenarios,
acceptance criteria, and the git branch name to use.

```
docs/pegin-issues/issues/
├── epics/        # epic-N — multi-feature goals (e.g. "Browser login prototype")
├── features/     # feat-N — the unit of work; one branch per feature
├── bugs/         # bug-N — defects
└── proposals/    # prop-N — design proposals (architecture decisions)
```

- File naming: `feat-10 - pegin-wasm crate scaffold and build pipeline.md`.
- Each issue has YAML front-matter: `id`, `status` (`backlog` → `in-progress` → `done`),
  `branch`, and `[[wikilinks]]` to its epic / blockers / blocked features.
- **Workflow:** pick the issue → check out its `branch` → implement against its acceptance
  criteria → update the issue `status`. Keep changes scoped to that issue's "In Scope".
- **Feature sizing:** Each `feat-N` should target **~1 000 lines of code** per PR (soft guidance,
  not CI-enforced). Above that, split into smaller features before coding — humans must be able to
  review the diff in one pass. Epics span many features; umbrella tickets that outgrew their scope
  should defer to sliced children. See `docs/pegin-issues/CLAUDE.md` § Feature sizing.
- Cross-cutting design docs live in the wiki submodule: `docs/pegin-wiki/`.

## Repository structure

```
pegin/
├── crates/                  # Rust workspace members
│   ├── pegin-domain/        # Shared kernel: Did, Username, Sub, AppError — no external Chia deps
│   ├── pegin-identity/      # Bounded context: IdentityStore + PasskeyAuthenticator traits
│   ├── pegin-wallet/        # Use cases: CreateAccount, SignJwt, AssertPasskey
│   ├── pegin-infrastructure/# Adapters: ChiaGateway (coinset.org), LocalProfileStore
│   ├── pegin-wasm/          # Browser mini wallet: #[wasm_bindgen] surface, builds to packages/sdk/wasm
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
        ├── <name>.dto.rs/ts               — types that cross the module boundary (commands, responses)
        └── <name>.entities.rs/ts          — domain types that live inside this module
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

## CI / build pipeline

Three parallel tracks on GitHub Actions — **no Docker**:

| Track | Jobs | Caching |
|-------|------|---------|
| Rust | `rust-build` → `rust-clippy` + `rust-test` (+ `rust-fmt` in parallel) | `Swatinem/rust-cache@v2` with shared key `pegin-rust-v1` |
| TypeScript | `typescript` (lint · build · test) | `actions/setup-node` pnpm cache |
| WASM | `wasm` (after `rust-build`) | same Rust cache + pre-built wasm32 test deps |

`rust-build` compiles the workspace once (`cargo build --workspace --tests` plus
`pegin-wasm` wasm32 test deps). Downstream Rust/WASM jobs restore the same `target/`
cache instead of recompiling external crates.

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
pnpm -r lint && pnpm -r build && pnpm -r test
```

- Workflow: `.github/workflows/ci.yml`
- Bump `RUST_CACHE_KEY` in the workflow when you need a cold cache (e.g. after a toolchain bump).

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

## Comment principles

Full guide: `docs/pegin-wiki/10-architecture/code-comment-principles.md`

- **Never more comment lines than code lines in a file.** If it needs that much prose, fix the code.
- Module headers: one line (`//!` in Rust, one JSDoc block in TS/JS) stating what the module *is*.
- Function docs are JSDoc-style and compact: one verb-first summary line, then `* `param` — meaning`
  bullets (only non-obvious params), and returns/errors only when surprising (e.g. `Ok(false)` vs `Err`).
  TS/JS uses real `@param`/`@returns` tags.
- Body comments state only what code can't: why, invariants, platform quirks. Never narrate the next
  line, number steps, or reference change history.
- Every lint suppression carries a one-line reason above it.

## Logging strategy (TypeScript/JavaScript)

Full guide: `docs/pegin-wiki/10-architecture/logging-strategy.md`

- **`console.*` is banned** (ESLint `no-console: error`); log through the logger modules:
  winston via `crates/pegin-wasm/logger.mjs` in Node scripts, `packages/sdk/src/shared/lib/logger.ts`
  in the browser SDK (winston is Node-only).
- Default level is **error**; `info`/`debug` are opt-in dev support (`LOG_LEVEL` env / `setLogLevel()`).
- Logging ≠ program output: CLI results and test reports go to stdout via `process.stdout.write`;
  diagnostics go to stderr through the logger.
- Log an error once where it's handled; never log secrets (mnemonics, keys).

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
- Comment principles: `docs/pegin-wiki/10-architecture/code-comment-principles.md`
- Logging strategy: `docs/pegin-wiki/10-architecture/logging-strategy.md`
- Tech stack + dep pins: `docs/pegin-wiki/04-technical/specs/tech-stack.md`
- Issues / tickets: `docs/pegin-issues/issues/`
