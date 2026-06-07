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

## Key commands

```bash
# Rust
cargo build --workspace        # build all Step 1 crates
cargo test --workspace         # run all tests

# TypeScript
pnpm install                   # install all packages
pnpm build                     # build all packages and apps

# Tauri (apps/mini)
pnpm --filter @pegin/mini dev  # launch desktop dev mode
```

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
