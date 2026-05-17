# Infrastructure and tooling principles

> **Hub:** [README.md](README.md) · **Developer index:** [../README.md](../README.md) · **Setup:** [developer-environment.md](developer-environment.md) · **Team coordination:** [collaboration-tooling.md](../../09-how-we-work/collaboration-tooling.md)

Defaults for **what we pay for**, **where we host**, and **what we run in the stack**. Anchored in [pegin-manifest.md § Layer IV](../../01-vision/pegin-manifest.md#layer-iv--how-we-build).

**Repo phase:** Foundation docs — compose and images are added when the Rust workspace lands; principles apply now.

---

## Paid project software (only two)

The project budget for **SaaS / collaboration software** is intentionally minimal:

| Service | Role | Paid |
|---------|------|------|
| **[GitHub](https://github.com/)** | Code, `docs/`, issues, PRs, CI | ✅ **Yes** — only paid dev platform |
| **[Basecamp](https://basecamp.com/)** | Project management, milestones, pilot tracking | ✅ **Yes** — only paid PM tool |

**Do not add** paid IdPs, observability suites, feature-flag SaaS, or proprietary CI add-ons without an explicit decision recorded in a PR.

Email, chat, and personal AI subscriptions are **out of band** — not counted as PEGIN project software (see [collaboration-tooling.md](../../09-how-we-work/collaboration-tooling.md)).

---

## Open source by default (no license tax)

Runtime, build, and operator tooling should be **open source** so the project does not accumulate per-seat or per-core license fees.

| Layer | Direction | Examples |
|-------|-----------|----------|
| **Language & build** | OSS | Rust, Node, `rue-cli`, `rustfmt`, `clippy` |
| **Identity / chain** | OSS ecosystem | Chia, `chia-wallet-sdk`, Rue, DIG stack |
| **Data store (operator edge)** | OSS if SQL needed | PostgreSQL — not proprietary warehouse |
| **Containers (planned)** | OSS | Podman, Docker — dev compose when codebase lands |
| **CI** | OSS runners + GitHub Actions | Prefer standard Actions; self-hosted runners on Hetzner if needed |

**Avoid** unless there is no OSS alternative and a written exception exists: commercial databases, APM with mandatory SaaS, proprietary test clouds, paid IDE mandates for the whole team.

Editors (Cursor, etc.) are **individual choice** — not a project license line item.

---

## Hosting strategy

| Phase | Where | Notes |
|-------|--------|--------|
| **Now (POC / early ops)** | **[Hetzner](https://www.hetzner.com/)** (or similar EU bare metal / VPS) | Simple VMs for `pegin-core`, CI runners, optional Postgres at the **operator edge** — see [application-architecture.md](../../10-architecture/application-architecture.md) |
| **Target** | **[DIG Network](https://github.com/DIG-Network)** | User profiles, grants, audit, sessions — production data plane |
| **Always** | **Chia** | DIDs, anchors, Rue contracts — not “hosted on Hetzner” as source of truth |

Hetzner is a **bootstrap**: acceptable for first deployments and internal dev/staging. Product direction is **DIG for application data**, not a permanent dependency on one VPS vendor.

**EU sovereignty:** prefer Hetzner EU regions (and EU DIG peers when live) — align with [competitive-moat.md](../../05-business/competitive-moat.md).

---

## Developer setup (planned: container-first)

**Target** when `pegin-core` exists: one-command dev environment via **Podman** (preferred) or **Docker Compose** under `deploy/dev/`. **Not in the repo during the wiki-only phase.**

| Principle | Meaning |
|-----------|---------|
| **Container-first** | Same image on macOS, Linux, and WSL2 — no “works on my laptop” Rust versions |
| **Compose as contract** | `deploy/dev/compose.yaml` will be the supported path when added |
| **Rootless Podman** | Preferred on Linux for parity with production-style ops |
| **Host-native until then** | rustup + Node on host for early work; see [developer-environment.md](developer-environment.md) |

Detail: [developer-environment.md](developer-environment.md)

---

## What this excludes

- Paying for Okta/Auth0/Azure AD for PEGIN’s own internal SSO (we dogfood PEGIN or passkeys + GitHub)
- Running production user identity data only on a single Hetzner Postgres without a DIG migration plan
- Adding a third paid SaaS “for convenience” without updating this doc

---

## Related

| Document | Topic |
|----------|--------|
| [developer-environment.md](developer-environment.md) | Compose commands, platforms |
| [fully-decentralized.md](../../01-vision/fully-decentralized.md) | DIG + Chia data plane |
| [tech-stack.md](../../04-technical/specs/tech-stack.md) | Dependencies and repos |

*Last updated: May 2026*
