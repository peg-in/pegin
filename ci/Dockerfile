# syntax=docker/dockerfile:1
#
# Forge-independent CI image: rust + wasm + node toolchain with cargo-chef.
# Built identically on GitHub, Codeberg (Forgejo), GitLab, or locally — the
# `cook` layer caches every compiled dependency, so a registry layer cache is
# the only thing CI needs to avoid compiling chia-* from scratch each run.
#
# Bump RUST_VERSION to track the project's stable toolchain.
ARG RUST_VERSION=1.83

# ── Toolchain: rust + wasm target + wasm-pack + node + pnpm + headless chrome ──
FROM rust:${RUST_VERSION}-bookworm AS toolchain
ENV DEBIAN_FRONTEND=noninteractive
ENV CHROMEDRIVER=/usr/bin/chromedriver

RUN apt-get update \
 && apt-get install -y --no-install-recommends \
      chromium chromium-driver ca-certificates curl gnupg \
 && curl -fsSL https://deb.nodesource.com/setup_26.x | bash - \
 && apt-get install -y --no-install-recommends nodejs \
 && corepack enable && corepack prepare pnpm@9.0.0 --activate \
 && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown \
 && rustup component add rustfmt clippy \
 && cargo install cargo-chef wasm-pack --locked

WORKDIR /work

# ── Planner: distil the dependency graph into a recipe (no source baked) ───────
FROM toolchain AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ── Builder: cook deps (cached layer), then bake source + node_modules ─────────
FROM toolchain AS builder

# JS dependency store — cached on the lockfile alone via `pnpm fetch`.
COPY pnpm-lock.yaml ./
RUN pnpm fetch

# Rust dependency compile — the expensive layer, cached on recipe.json.
COPY --from=planner /work/recipe.json recipe.json
RUN cargo chef cook --workspace --tests --recipe-path recipe.json \
 && cargo chef cook --workspace --release --recipe-path recipe.json

# Now the actual source; deps above are reused, only workspace crates rebuild.
COPY . .
RUN pnpm install --frozen-lockfile --offline

# CI runs `docker run <image> make <target>` against this baked, warm tree.
CMD ["make", "ci"]
