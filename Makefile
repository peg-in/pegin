# PEGIN CI targets — the single source of truth for build/lint/test commands.
# Every forge (GitHub, Codeberg/Forgejo, GitLab) and the Docker image call these,
# so the commands live here once instead of being duplicated per-forge YAML.

.PHONY: ci ci-core ci-web ci-wasm \
        lint-rust test-rust \
        lint-web build-web test-web \
        wasm-test wasm-build smoke sdk-test

# ── Aggregate ─────────────────────────────────────────────────────────────────
ci: ci-core ci-web ci-wasm

# ── Core libraries (Rust workspace: domain · identity · wallet · gateway) ──────
ci-core: lint-rust test-rust

lint-rust:
	cargo fmt --all -- --check
	cargo clippy --workspace --all-targets -- -D warnings

test-rust:
	cargo test --workspace

# ── Login SDK & web apps (TypeScript) ──────────────────────────────────────────
ci-web: lint-web build-web test-web

lint-web:
	pnpm -r lint
	pnpm exec eslint "crates/**/*.mjs"

build-web:
	pnpm -r build

test-web:
	pnpm -r test

# ── Browser wallet (WASM) ───────────────────────────────────────────────────────
# One build for both consumer targets; smoke + SDK tests reuse those artifacts.
ci-wasm: wasm-test wasm-build smoke sdk-test

wasm-test:
	wasm-pack test --headless --chrome crates/pegin-wasm

wasm-build:
	pnpm build:wasm:node
	pnpm build:wasm

smoke:
	pnpm smoke

sdk-test:
	pnpm --filter @pegin/sdk test
