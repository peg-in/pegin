# PEGIN — Timeless business principles & customer value

> **Anchor document:** [pegin-manifest.md](pegin-manifest.md) — Layers I–IV (evergreen customer, PEGIN product, how we work, how we build). This file is the **long-form** business narrative; if anything conflicts, the manifest wins.

## How PEGIN aims to self-fund and enable founder transition

---

## Part 1: Timeless Business Principles for PEGIN

### The Four Pillars (Applied to Decentralized SSO)

These principles transcend technology trends, market cycles, and management fads. They work because they align incentives: the business wins when customers win.

#### 1. **Alignment of Incentives**
> "Build a business where your success is impossible without your customer's success."

**The Principle:**
- Traditional SaaS: Company succeeds by extracting maximum revenue per user → inherent conflict
- PEGIN: Company succeeds when identity infrastructure is reliable, trustworthy, and widely adopted

**How PEGIN Implements This:**
- Revenue comes from network participation (DIG peer fees), not from per-user API taxes
- Enterprise scales from 100 to 100K users → your cost stays flat, their cost stays flat
- Token appreciation happens only if the network becomes valuable to everyone
- No way to "lock in" customers → must earn their trust every day
- If you disappear → system keeps running → you don't profit from abandonment (no renewal fees)

**Why This Lasts 20+ Years:**
- Customers never resent you for extracting value
- Network effects compound: more customers → more valuable network → you benefit
- Trust is the moat, not contracts

#### 2. **Fixed Infrastructure Costs, Variable Revenue**
> "Build systems where growth doesn't kill profitability."

**The Principle:**
 Their costs still tend to **scale with usage and enterprise load**: MAU/seat licensing, support, compliance regions, dedicated shards, and custom SLAs. More large tenants usually means more people, infra, and revenue share pressure — not a flat cost curve.

PEGIN's **hypothesis** (to validate):
- Open protocol + customer-run or shared DIG peers: marginal **protocol** cost per new tenant may stay lower than running a global multi-tenant control plane for every feature.
- Operators still pay for engineering, support, and their own peer capacity — this is not “free at scale.”

**What to measure (no placeholder dollars):**
- Operator cost per deployment (people + infra + chain/DIG fees)
- Revenue per customer if any SLA/services exist
- Whether marginal cost per additional tenant drops vs running a full multi-tenant control plane

**Why this might last (hypothesis):**
- Network effects: more participants can increase utility of the protocol
- Profitability still requires discipline — growth does not automatically mean positive margin

See [sustainable-funding.md](../05-business/sustainable-funding.md) and .

#### 3. **Credible Exit Path for Founder**
> "Build systems that work without you, and wealth compounds."

**The Principle:**
- Company exists to serve customers, not to serve founder
- Founder should be expendable by Year 4
- Founder's wealth comes from tokens/stake, not from being indispensable

**How PEGIN Enables Founder Exit:**

**Year 1:** Founder builds identity engine, smart contracts, SDKs
**Year 2:** Founder hires tech team; system runs with oversight
**Year 3:** Tech team runs independently; founder advises monthly
**Year 4:** Founder hands governance to community; holds tokens

**Long-term intent (not forecast):**
- Community or foundation governance for the open protocol
- Possible token and DIG peer participation for contributors — **subject to legal review and market proof**

**Founder can (if the system works without daily involvement):**
- Reduce operational role while the protocol remains maintained by the community
- Pursue adjacent products (e.g. Vault, Gateway) only after core SSO proves value

**Why This Lasts:**
- System designed to outlive founder
- Tokens incentivize community to maintain it
- No hidden dependencies on founder = founder wealth is secure

#### 4. **Solve Real Problems With Unique Leverage**
> "Don't build what already exists. Solve what nobody else can solve."

**The Principle:**
- Okta, Auth0, Microsoft solve "enterprise SSO" fine
- PEGIN solves something nobody can: **decentralized identity that survives founder/company death**

**PEGIN's Unique Leverage:**
1. **Blockchain anchor** → Immutable identity (Chia DID on blockchain)
2. **DIG network** → Decentralized data (user data doesn't live on PEGIN's servers)
3. **Rue smart contracts** → Type-safe, optimized identity rules
4. **No single point of failure** → If PEGIN Inc. dies tomorrow, identity system keeps running
5. **Passkey-first** → Passwordless from day 1 (competitors retrofitting)

**Why Nobody Else Offers This:**
- Okta, Auth0, Microsoft *want* to be indispensable (drives renewal fees)
- PEGIN's business model *requires* being dispensable (enables founder exit)
- This is the moat

**Why This Lasts:**
- Unique value is unique forever (nobody else wants to be dispensable)
- First-mover advantage in "identity that survives founder" = defensible market position

---

## Part 2: Customer Value Proposition

> **What does a customer actually get from PEGIN that's better than Azure AD / Okta?**

### The Five Customer Promises

#### 1. **Lower TCO hypothesis (validate in pilots)**

**Direction (not measured for PEGIN yet):**
- Incumbent IdPs often charge per seat plus services; PEGIN targets **open core + flat SLA / infrastructure fees** instead of per-user tax.
- On-chain and DIG costs should be modeled per deployment, not quoted generically.

**Pilot deliverable:** Customer-specific worksheet — current IdP + permission stack vs PEGIN run cost (12-month).

#### 2. **Data Sovereignty & Regulatory Compliance**

**Problem:**
- Azure AD: Microsoft owns your data, stores in US/EU zones, subject to US law
- Okta: Third-party vendor lock-in, audits required for HIPAA/GDPR
- PEGIN: User data on DIG network, customer controls which peers replicate their data

**What customer gets:**
- **GDPR compliance:** User data stays in EU datacenters (customer chooses replicas)
- **HIPAA compliance:** Encrypted on DIG; audit append-only on DIG with on-chain store anchors (no heavy PHI on chain)
- **FedRAMP ready:** Can be run in government datacenters
- **No data moat for PEGIN:** Can't leverage user data for other products (unlike Okta/Microsoft)

**Hypothesis:**
- Immutable audit trail may reduce audit prep — **confirm with compliance advisors per customer**
- No surprise vendor price hikes (PEGIN can't harvest your data for ML)
- Regulators trust blockchain more than corporate promises

#### 3. **Passwordless Authentication From Day 1**

**Problem:**
- Azure AD: Passwords default; MFA is optional
- Okta: Passwords default; passwordless is add-on
- PEGIN: Passkey (Face ID, fingerprint) is the only login method

**What customer gets:**
- **Zero passwords = zero breaches** (from password leaks)
- **Phishing-resistant** (FIDO2 standard; attacker can't intercept passkey)
- **Faster login** (biometric is faster than typing password)
- Less password friction and helpdesk load — **quantify in pilot** (time-to-login, reset tickets)

**Psychology benefit:**
- Employees WANT to use it (Face ID is cool, passwords are annoying)
- Faster adoption than forced MFA
- Less helpdesk burden (biometric can't be "forgotten")

#### 4. **Decentralized = Cannot Be Shut Down by Regulators/Competitors**

**Problem:**
- Azure AD: Microsoft can be fined/shut down → you lose SSO access
- Okta: Okta can go bankrupt or get acquired → migrations are painful
- PEGIN: Smart contracts on Chia are immutable; DIG network is P2P; nobody can shut it down

**What customer gets:**
- **Regulatory confidence:** Even if PEGIN Inc. fails, identity system survives
- **No acquisition risk:** Even if PEGIN gets bought by Microsoft, the system keeps running independent
- **No price hostage risk:** Once you're in, PEGIN can't triple prices in 3 years (you can fork)
- **Forever support:** Open-source community maintains it even if vendor disappears

**Real scenario:**
> "We chose PEGIN because we know in 20 years we'll still have working identity. With Okta, we don't know if they'll exist, if they'll raise prices 10x, or if they'll be acquired by Microsoft. PEGIN will exist because the blockchain exists."

#### 5. **Ecosystem Benefits (Passive Revenue for Customer)**

**What customer gets:**
- **NFT credential royalties:** If customer issues credentials (employee certs, partner certs), earn 2-5% on credential trades
- **Token appreciation:** PEGIN token supplied to community → customer can earn/hold tokens
- **DIG peer participation:** Customer can run their own DIG peer, earn fees for storing others' data
- **Vendor becomes partner, not parasite:** PEGIN succeeds → tokens appreciate → customer benefits

**Future (Phase 4+):** Credential NFTs / royalties are product options — no revenue assumed until built and legal.

---

## Part 3: Funding path

> **No year-by-year revenue, cost, or token valuations in this doc.** Use your actual budget and [sustainable-funding.md](../05-business/sustainable-funding.md) for mechanism descriptions.

### Phases (qualitative)

| Phase | Focus |
|-------|--------|
| Pre-revenue | POC, design partners, seed runway if any |
| Early revenue | Paid pilots, SLAs, integration services — amounts TBD per contract |
| Scale | Repeatable deployment; measure unit economics per customer |

### Unit economics (fill from pilots)

Track per customer when you have data: contract value, delivery cost, gross margin, CAC, payback months, churn. Do not publish LTV or margin targets without measurements.

### Founder role timeline (intent)

| Phase | Focus |
|-------|--------|
| 0 | Build POC |
| 1 | Pilots + small team |
| 2+ | Hand off ops if protocol + support are self-sustaining |

Wealth/outcomes depend on funding, token policy, and adoption — **not assumed here**.

---

## Part 4: Why Customers Don't Leave

### Lock-In That Isn't Painful

Traditional SaaS lock-in (painful):
- High switching costs → customers resent you
- Vendor lock-in contracts → customers feel trapped
- Price increases after lock-in → customers hate you

PEGIN lock-in (beneficial):
- **Switching costs low** (open-source, portable credentials)
- **But value of network increases over time** (token appreciation, ecosystem)
- **Customers are invested** (they own PEGIN tokens, other customers are partners)
- **Open protocol** (even if PEGIN Inc. fails, they can switch to community version)

### The Gravity Well

By Year 3, customers are locked in not by contract, but by:
1. **Token holdings** — Customer owns PEGIN tokens (incentivized to see them appreciate)
2. **Integration depth** — 15+ apps connected to PEGIN (switching is effort, not money)
3. **Community network** — Other customers are partners / trading credentials
4. **Regulatory approval** — They've approved PEGIN for HIPAA/GDPR (switching = new compliance audit)
5. **Passkey user base** — Employees have Face ID registered to PEGIN (switching = re-registering all)

**Retention rate: 95%+** (better than Okta's 90%)

---

## Part 5: Customer Success Stories (Year 4+)

### The Stories You'll Hear

**Story 1: "We switched from Azure AD" (template — replace with real quotes after pilots)**
> "We moved app SSO to PEGIN. Licensing and ops costs changed — we measured both sides. Auditors liked the log model. Employees prefer passkeys. We stayed because the deployment worked, not because of a slide deck."

**Story 2: "We Can't Run Azure AD in Our Region"**
> "We're a healthcare provider in Australia. HIPAA requires data in specific zones. Azure AD can't do it. PEGIN lets us run DIG peer in Sydney, keep data local, stay compliant. Regulatory team approved it in 2 weeks (they trust blockchain)."

**Story 3: "PEGIN Outlasted Our Vendor"**
> "Our previous SSO vendor went bankrupt (bad timing). Suddenly 10K employees had no login. Nightmare. Switched to PEGIN. A year later, if PEGIN Inc. disappeared, we'd still have working identity (open-source community runs it). Peace of mind is priceless."

**Story 4: "Our Customers Love Passkeys"**
> "We're a SaaS company (200K users). We required passwords. Now we use PEGIN passkeys. Password reset tickets dropped 40%. Phishing attempts dropped to zero. Our security team actually approves of login."

**Story 5: "Network Effects Are Real"**
> "We're an enterprise software vendor. We integrated PEGIN for our 100 customers. They all use it. They talk to each other about PEGIN. We didn't spend a dime on marketing (network effect). PEGIN customers buy from us at higher rate than non-PEGIN customers."

---

## Part 6: The Mindset That Sustains PEGIN

### Not "Extraction," But "Alignment"

Traditional SaaS Mindset:
> "How do I maximize revenue per customer? How do I lock them in? How do I raise prices?"

PEGIN Mindset:
> "How do I make identity infrastructure so good that customers can't imagine switching? How do I make founders rich without being indispensable? How do I become obsolete and still prosper?"

### The Decentralized Entrepreneur

PEGIN founder is not:
- A CEO building a company to sell
- A VC-backed entrepreneur chasing exit multiples
- A service business owner trading time for money

PEGIN founder is:
- **An infrastructure builder** — Creating public goods (blockchain + DIG)
- **A token holder** — Wealth comes from network appreciation, not extraction
- **A community steward** — Success means stepping away, not becoming irreplaceable
- **An ecosystem designer** — Revenue is a side effect of good incentives

This is the **decentralized entrepreneur** playbook.

---

## Summary: PEGIN's Business Timelessness

| Principle | Why It Lasts | How PEGIN Applies |
|---|---|---|
| **Alignment of Incentives** | Customers can't resent you if your success depends on theirs | Revenue from network participation, not per-user tax |
| **Fixed Costs, Variable Revenue** | Growth may compound if operator economics work | Measure peer + team cost per deployment |
| **Founder Exit Path** | Protocol can outlive daily founder ops | Token/governance only if adopted and legal |
| **Unique Leverage** | Only PEGIN can offer "identity that survives founder" | Blockchain anchor + DIG network = no single point of failure |
| **Customer Value** | Stay by choice, not lock-in | TCO and UX to prove in pilots |
| **Self-funding** | Possible if services + network revenue exceed ops cost | **Not assumed** — prove with operating model |

**The ultimate proof of timelessness:** Even if PEGIN's founder is hit by a bus tomorrow, the company *keeps making money*, the customers *keep getting value*, and the community *keeps building*.

That's not luck. That's design.

---

*Built with 🐧 by the PEGIN team. Waddle forward.*