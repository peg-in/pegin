# Developer documentation (programmers)

**Start here only if you write or integrate code.** Everyone else: [09-how-we-work/](../09-how-we-work/README.md) and [10-architecture/](../10-architecture/README.md).

| | |
|---|---|
| **Environment** | [environment/](environment/README.md) — setup policy, infra principles |
| **Engineering** | [engineering/](engineering/README.md) — lint, tests |
| **Integration** | [integration/](integration/README.md) — SDK |
| **Spec 1 & 2** | [04-technical/specs/](../04-technical/specs/README.md) |
| **Architecture** | [10-architecture/](../10-architecture/README.md) |

---

## Folder map

```
08-developer/
├── environment/          Setup policy, paid-SaaS & hosting (compose planned)
│   ├── developer-environment.md
│   ├── ai-coding-tools.md          optional: local agents, privacy
│   └── infrastructure-and-tooling-principles.md
├── engineering/          rustfmt, clippy, test pyramid, CI
├── integration/          SDK, WebAuthn, JWT
└── (root stubs)          redirects to moved docs
```

---

## Build path (engineers)

| # | Document |
|---|----------|
| 1 | [09-how-we-work/how-we-work.md](../09-how-we-work/how-we-work.md) |
| 2 | [environment/infrastructure-and-tooling-principles.md](environment/infrastructure-and-tooling-principles.md) |
| 3 | [environment/developer-environment.md](environment/developer-environment.md) |
| 4 | [10-architecture/application-architecture.md](../10-architecture/application-architecture.md) |
| 5 | [engineering/test-architecture.md](engineering/test-architecture.md) |
| 6 | [engineering/linting-and-formatting.md](engineering/linting-and-formatting.md) |
| 7 | [04-technical/specs/tech-stack.md](../04-technical/specs/tech-stack.md) |
| 8 | [integration/sdk-guide.md](integration/sdk-guide.md) |
| 9 | [10-architecture/on-chain-architecture.md](../10-architecture/on-chain-architecture.md) |

---

## Index

| Section | Key files |
|---------|-----------|
| [environment/](environment/README.md) | Dev setup, AI tools (optional), infra principles |
| [engineering/](engineering/README.md) | Linting, test architecture |
| [integration/](integration/README.md) | SDK guide |

System design: [10-architecture/](../10-architecture/README.md) (not under `08-developer`).

---

*Programmers only. Culture: `09-how-we-work/`. Specs: `04-technical/specs/`.*
