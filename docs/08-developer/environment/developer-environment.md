# Developer environment

> **Hub:** [README.md](README.md) · **Developer index:** [../README.md](../README.md) · **Principles:** [infrastructure-and-tooling-principles.md](infrastructure-and-tooling-principles.md) · **Team tools:** [collaboration-tooling.md](../../09-how-we-work/collaboration-tooling.md) · **Spec 1:** [../../04-technical/specs/tech-stack.md](../../04-technical/specs/tech-stack.md)

**Programmers only** — how we will set up machines when the codebase lands. Non-developers: [09-how-we-work/](../../09-how-we-work/README.md).

**Repo phase:** Documentation and wiki foundation only — **no `deploy/` compose stack in the repo yet.** Podman/Docker dev setup is a **planned** direction, not something you run today.

---

## Principles (short)

| Rule | Detail |
|------|--------|
| **Paid SaaS** | **[GitHub](https://github.com/)** + **[Basecamp](https://basecamp.com/)** only — see [infrastructure-and-tooling-principles.md](infrastructure-and-tooling-principles.md) |
| **Stack** | **Open source** — no project license fees for databases, CI add-ons, or IdPs |
| **Hosting** | **DIG** for app data (target); **Hetzner** OK for POC/staging VMs until DIG peers are production-ready |
| **Dev setup (future)** | **Podman or Docker Compose** for a one-command dev shell — added when `pegin-core` exists |

---

## Supported host OS (*when coding starts*)

| OS | Support |
|----|---------|
| **macOS** | ✅ |
| **Linux** | ✅ |
| **Windows** | ⚠️ **WSL2 only** — clone under `~/projects/pegin`, not `/mnt/c/` |
| **Windows (native)** | ❌ |

---

## Toolchain (*when code lands*)

| Tool | Install |
|------|---------|
| **Git** | OS package manager |
| **Rust** | [rustup](https://rustup.rs/) `stable` |
| **Node.js** | 20 or 22 LTS — [fnm](https://github.com/Schniz/fnm) / [mise](https://mise.jdx.dev/) |
| **rue-cli** | `cargo install rue-cli` |

```bash
git clone git@github.com:<org>/pegin.git
cd pegin
# rustup, fnm, rue-cli — then:
cargo test --workspace   # when workspace exists
```

Before your first implementation PR, read [application-architecture.md](../../10-architecture/application-architecture.md) and [tech-stack.md](../../04-technical/specs/tech-stack.md).

### Future: Podman / Docker (not in repo now)

When implementation starts, we plan optional **Compose** under `deploy/dev/` (Rust + Node image, repo bind-mount). **Podman** preferred; **Docker** supported. Until that lands, use host-native toolchain above.

---

## Editors (preference-based)

No mandated IDE. **rustfmt** + **clippy** when code exists — [linting-and-formatting.md](../engineering/linting-and-formatting.md).

| Editor | Notes |
|--------|--------|
| Cursor / VS Code / Zed / Neovim / RustRover | All fine; Rue: [rue-vscode](https://github.com/xch-dev/rue-vscode) |

---

## AI for code (optional)

| Topic | Document |
|-------|----------|
| **Strategy & privacy** | [ai-coding-tools.md](ai-coding-tools.md) |
| **Docs & all roles** | [collaboration-tooling.md](../../09-how-we-work/collaboration-tooling.md) |

**Short rules:** no secrets in prompts; human review before merge — see [ai-coding-tools.md](ai-coding-tools.md).

---

## Related

| Document | Topic |
|----------|--------|
| [infrastructure-and-tooling-principles.md](infrastructure-and-tooling-principles.md) | Paid SaaS, OSS, Hetzner → DIG |
| [../engineering/linting-and-formatting.md](../engineering/linting-and-formatting.md) | Style gates |
| [../engineering/test-architecture.md](../engineering/test-architecture.md) | Tests, CI |
| [../integration/sdk-guide.md](../integration/sdk-guide.md) | WebAuthn, JWT |

*Last updated: May 2026*
