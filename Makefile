# CI command surface — one definition used locally, in Docker, and in GitHub Actions.
#
#   make ci        run everything (same as the Docker default CMD)
#   make ci-core   Rust fmt + clippy + test
#   make ci-web    TypeScript lint + build + test
#   make ci-wasm   WASM browser test, build, smoke, SDK test
#
# CI builds the image once (ci/Dockerfile), then three parallel jobs each run
# `docker run <image> make <target>`. External Rust deps are pre-compiled in the
# image via cargo-chef; jobs only rebuild workspace crates for their target.

.PHONY: ci ci-core ci-web ci-wasm ci-docker \
        lint-rust test-rust lint-web build-web test-web \
        wasm-test wasm-build smoke sdk-test

ci: ci-core ci-web ci-wasm

# ── Rust workspace ────────────────────────────────────────────────────────────
ci-core: lint-rust test-rust

lint-rust:
	cargo fmt --all -- --check
	cargo clippy --workspace --all-targets -- -D warnings

test-rust:
	cargo test --workspace

# ── TypeScript packages & apps ────────────────────────────────────────────────
ci-web: lint-web build-web test-web

lint-web:
	pnpm -r lint
	pnpm exec eslint "crates/**/*.mjs"

build-web:
	pnpm -r build

test-web:
	pnpm -r test

# ── Browser wallet (WASM) ─────────────────────────────────────────────────────
ci-wasm: wasm-build wasm-test smoke sdk-test

wasm-build:
	pnpm build:wasm:node
	pnpm build:wasm

wasm-test:
	wasm-pack test --headless --chrome crates/pegin-wasm

smoke:
	pnpm smoke

sdk-test:
	pnpm --filter @pegin/sdk test

# ── Local Docker CI (mirrors GitHub Actions) ───────────────────────────────────
ci-docker:
	docker build -f ci/Dockerfile --target builder -t pegin-ci .
	docker run --rm --shm-size=2g pegin-ci make ci
