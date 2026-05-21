# How we work

> **Inspired by:** [*Rework*](https://37signals.com/books/rework) — Jason Fried & David Heinemeier Hansson (Basecamp / 37signals).  
> Reference edition: [Internet Archive — Rework v0.9](https://archive.org/details/fried-jason-heinemeier-hansson-david-rework-v.-0.9_202502/Fried%2C%20Jason%20%26%20Heinemeier%20Hansson%2C%20David%20-%20Rework%20-%20ctrl/)  
> **Sibling docs:** [collaboration-tooling.md](collaboration-tooling.md) · [team-how-we-work.md](team-how-we-work.md) · **Programmers:** [08-developer/](../08-developer/developer-documentation.md)

PEGIN is built as a **remote, async, low-meeting** project. This is part of the product culture: decentralized infrastructure deserves decentralized collaboration — not calendar theater.

**Anchored in:** [pegin-manifest.md § Layer III](../../01-vision/pegin-manifest.md#layer-iii--how-we-work) (W1–W11) · **Mottos:** [Foundation & company mottos](../../01-vision/pegin-manifest.md#foundation--company-mottos)

These are **defaults for contributors and operators** (foundation & company), not HR policy. Adjust for legal, security incidents, or customer pilots with explicit SLAs.

---

## Foundation & company mottos

Canonical list lives in the [manifest](../../01-vision/pegin-manifest.md#foundation--company-mottos). Say them, mean them:

| Motto | Maps to |
|-------|---------|
| **Don’t be an asshole!** | Respectful async disagreement |
| **No drama allowed!** | Facts in GitHub/Basecamp — no gossip |
| **Your ego is not your amigo!** | Principles and pilots beat pride |
| **Meeting free zone!** | W2 — meetings are rare |
| **Library Zone Days** | W4 — focus day, no internal meetings |
| **Keep quiet — someone is working.** | W7 — protect maker time |

---

## Core defaults

| Practice | PEGIN stance |
|----------|--------------|
| **Meetings** | Not a default. Use writing first. |
| **Remote** | Fully distributed; no mandatory office. |
| **Library days** | Recurring focus blocks — no meetings, deep work only. |
| **Async communication** | Decisions and specs live in the repo (`docs/`, issues, PRs). |
| **Shipping** | Small releases; POC before roadmap fiction. |
| **Healthy pace** | Sustainable work; sleep and time off are part of doing good work — not optional extras. |

---

## Healthy team mindset (*Rework*)

Paraphrased from [*Rework*](https://37signals.com/books/rework) — culture defaults for **every role**, not only engineering. A tired team ships worse identity software; burnout is a security and quality risk.

### Workaholics aren’t heroes

Working more hours does not mean you care more or produce more. **Heroics are a planning failure** — if everything depends on last-minute pushes, the plan was wrong.

| Do | Don’t |
|----|--------|
| Leave on time most days | Brag about all-nighters |
| Treat overtime as an exception | Normalize “always on” chat status |
| Fix planning when crunch repeats | Reward exhaustion as commitment |

### Sleep, rest, and recovery

Clear thinking — architecture, security reviews, protocol design — requires rest. *Rework* treats sleep and breaks as **inputs to quality**, not laziness.

- **Protect evenings and weekends** except real incidents (security, production down, customer SLA).
- **Vacation and time off** are encouraged; hand off context in writing (Basecamp + GitHub), not guilt.
- **No performative availability** — you are not paid to be online, you are paid to move the project forward.

### ASAP is poison

“ASAP” and fake urgency create stress without clarity. Replace with **specific deadlines** or **priority order** in Basecamp/GitHub.

| Instead of… | Say… |
|-------------|------|
| “ASAP” | “Need by Thursday EOD” or “Next library day” |
| “Drop everything” | “This outranks X — here’s why” |
| Panic pings | One written brief with context and owner |

Default urgency: **calm and specific**, not ambient anxiety.

### Say no to protect the team

A small team survives by **not** doing everything. Saying no to scope, meetings, and vanity work is how we protect health and the POC ([mvp-strategy.md](../../03-use-cases/mvp-strategy.md)).

- **No** to meeting creep, slide-deck theater, and duplicate process.
- **No** to roadmap fiction that implies heroics will close the gap.
- **Yes** to sustainable batches and honest re-planning.

### Stress signals we take seriously

If someone is consistently working nights, skipping rest, or carrying silent overload — that is a **team problem**, not an individual badge. Response: reduce scope, cut meetings, reassign, or slip dates — not “try harder.”

### Managers and partners

- Do not measure commitment by **hours online** or meeting attendance.
- Do not praise exhaustion; praise **clear docs, merged work, and honest estimates**.
- Customer deadlines are real; **internal** panic is optional — plan in small batches instead.

---

## Rework principles we adopt (collaboration)

Paraphrased from *Rework* — apply with judgment, not as slogans.

### Meetings are toxic

| Instead of… | Do… |
|-------------|-----|
| Status meetings | Written update in issue/PR or short async post |
| Kickoff calls | One-pager in `docs/` + comments |
| Brainstorm live | Document proposal; comment thread |
| “Quick sync” | Message with concrete question and deadline |

**When a meeting is OK (rare):**

- Conflict that async failed to resolve in **48h**
- Pairing on a **time-boxed** hard problem (≤ 45 min)
- Customer pilot with **scheduled** external stakeholders (not internal standups)

Default answer: **“Can this be a doc?”**

### Library days (focus days)

**Library day** = one weekday (team-aligned or per-person) where:

- No internal meetings
- Notifications off except incidents
- Goal: flow state — code, tests, specs ([test-architecture.md](../08-developer/engineering/test-architecture.md))

Suggested team rule:

- Pick **one fixed day per week** (e.g. Wednesday) as org-wide library day
- Other days: meetings only in a **narrow window** (e.g. 14:00–16:00 local) if needed at all

### Planning is guessing

Roadmap lives in [roadmap.md](../../03-use-cases/roadmap.md) as **direction**, not a Gantt contract. We plan in **small batches** (POC → v1 → PePP). Re-plan when we learn — especially from pilots ([sustainable-funding.md](../../05-business/sustainable-funding.md)).

### Interruption is the enemy

Protect maker time. Batch questions into one daily async thread if needed. Urgent = production/security/customer SLA — not “I was curious.”

### Build half a product — not half an ass

POC scope: **Login with PEGIN** only ([mvp-strategy.md](../../03-use-cases/mvp-strategy.md)). Say no to feature creep in the same release. Half-done Entra parity helps nobody.

### Start at the epicenter

For engineering, the epicenter is:

1. Test harness + simulator wallets ([test-architecture.md](../08-developer/engineering/test-architecture.md))
2. Passkey → DID → JWT
3. Everything else waits

### Outside money is plan B

Prefer open core and operator economics over VC-driven roadmap inflation. See [business-principles.md](../../01-vision/business-principles.md).

### Inspiration is perishable

When you see the fix, open the PR. Don’t wait for permission if it’s inside agreed scope. Small PRs merge faster.

### Embrace constraints

Small team, no 24/7 war room, no meeting culture — **constraints force better architecture** (DIG, open protocol — see [10-architecture/](../10-architecture/architecture-overview.md)).

---

## Remote work

| Topic | Practice |
|-------|----------|
| **Hours** | Overlap **optional**; document core overlap if the team wants it (e.g. 4h EU afternoon) |
| **Location** | Work where you focus; respect data rules for customer deployments ([competitive-moat.md](../../05-business/competitive-moat.md)) |
| **Tools** | Git + PRs, issues, `docs/` — not slide decks |
| **Visibility** | Outcome-based: merged PRs, green CI, updated docs — not online presence |
| **Onboarding** | [08-developer/developer-documentation.md](../08-developer/developer-documentation.md) build path; no “watch these 20 meetings” |

---

## Communication norms

1. **Write it down** — if it isn’t in the repo, it didn’t happen for engineering.
2. **Context in PRs** — what, why, how to test; link issue/spec section.
3. **No CC theater** — don’t pull people “for visibility”; use `@` only when you need their action.
4. **Decisions** — propose in markdown or PR comment; **lazy consensus** after 48h silence on non-controversial items.
5. **Incidents** — break library day / async rules; fix first, postmortem doc after.

---

## What we don’t do

| Anti-pattern | Why |
|--------------|-----|
| Daily standups | Status belongs in CI and written updates |
| Meeting-heavy sprint planning | Small backlog + ship |
| Presentations before prototypes | Demo working login, not slides |
| Process for process’s sake | Matches *Rework*; matches PEGIN’s product story |
| Glorifying crunch / all-nighters | Sustainable pace; overtime = planning bug |
| “ASAP” without a date | Named deadline or priority |
| Guilt about time off | Written handoff; vacation is normal |

---

## For managers and partners

- Measure **shipped POC milestones** and **pilot learnings**, not hours in meetings.
- Customer calls are **product**, not internal coordination — schedule them, don’t normalize internal calls.
- EU sovereignty and enterprise buyers care about **docs and audits**, not how many standups we held.

---

## Related

| Doc | Why |
|-----|-----|
| [collaboration-tooling.md](collaboration-tooling.md) | GitHub, Basecamp, what goes where |
| [pegin-manifest.md](../../01-vision/pegin-manifest.md) | Trust, mottos, W1–W11 |
| [10-architecture/architecture-overview.md](../10-architecture/architecture-overview.md) | System design |
| [business-principles.md](../../01-vision/business-principles.md) | Why the business aligns with users |

*How we work v1.1 · May 2026 · Culture doc for all roles; not employment contract.*
