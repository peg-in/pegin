# syntax=docker/dockerfile:1
#
# CI image: rust + wasm + node toolchain with cargo-chef dependency caching.
# Registry layer cache (`buildcache` tag) restores the `cook` layers across PRs so
# chia-* crates are not recompiled when Cargo.lock is unchanged.
#
# Bump RUST_VERSION when cargo-chef / wasm-pack require a newer toolchain.
ARG RUST_VERSION=1.91

# ── Toolchain ─────────────────────────────────────────────────────────────────
FROM rust:${RUST_VERSION}-bookworm AS toolchain
ENV DEBIAN_FRONTEND=noninteractive
ENV CHROMEDRIVER=/usr/bin/chromedriver

RUN apt-get update \
 && apt-get install -y --no-install-recommends \
      chromium chromium-driver ca-certificates curl gnupg clang \
 && curl -fsSL https://deb.nodesource.com/setup_26.x | bash - \
 && apt-get install -y --no-install-recommends nodejs \
 && npm install -g pnpm@9.0.0 \
 && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown \
 && rustup component add rustfmt clippy \
 && cargo install cargo-chef@0.1.77 wasm-pack@0.15.0 --locked

WORKDIR /work

# ── Planner: dependency recipe (fast; cook layer is what we cache) ─────────────
FROM toolchain AS planner
COPY Cargo.toml Cargo.lock ./
COPY crates crates
RUN cargo chef prepare --recipe-path recipe.json

# ── Builder: warm deps, then bake source ─────────────────────────────────────
FROM toolchain AS builder

COPY pnpm-lock.yaml ./
RUN pnpm fetch

COPY --from=planner /work/recipe.json recipe.json
RUN cargo chef cook --workspace --tests --recipe-path recipe.json \
 && cargo chef cook --workspace --release --recipe-path recipe.json \
 && cargo chef cook -p pegin-wasm --tests --target wasm32-unknown-unknown --recipe-path recipe.json \
 && cargo chef cook -p pegin-wasm --release --target wasm32-unknown-unknown --recipe-path recipe.json

COPY . .
RUN pnpm install --frozen-lockfile --offline

CMD ["make", "ci"]
