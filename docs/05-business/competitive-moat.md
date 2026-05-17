# PEGIN competitive positioning

## Core idea

PEGIN does not win by quoting a lower **per-seat price** than Okta or Entra before anyone has run a pilot. It wins — if it wins — on **how identity is owned, deployed, and governed**.

| Lead with | Do not lead with |
|-----------|------------------|
| **Open core** — protocol source is forkable; no single vendor kill switch | “Cheaper than Okta” on a spreadsheet with no customer data |
| **Decentralized deployment** — DIDs on Chia; grants, audit, and user data on DIG; operators run peers | “We have every Entra feature on day one” |
| **Structural trust** — user-held identity; employer grants revocable permissions (PePP) | Secret sauce nobody can copy (stacks are learnable) |
| **Operator economics** — SLAs, integration, managed peers priced per engagement | Promised token or peer income as a sales hook |

**In one line:** PEGIN’s moat hypothesis is **open core + decentralized deployment**, not seat-license arbitrage.

What “structural” means here:

- **Open source** — the protocol can outlive any one company.
- **User-held DIDs** — different trust model than directory-owned accounts that vanish when you leave.
- **DIG + on-chain anchors** — data and audit off central Postgres; store updates committed on Chia, not heavy payloads on chain.
- **Optional network participation** — DIG peer and incentive economics are **TBD** until pilots prove them.

Until pilots publish TCO and workflow metrics, use this doc for **strategy**, not sales quotes.

---

## Incumbent model (qualitative)

| Player | How they usually make money | What PEGIN does differently |
|--------|----------------------------|-----------------------------|
| **Azure AD / Entra** | Per-seat licensing, M365 bundle, enterprise sales | No default per-seat tax on open core |
| **Okta / Auth0** | Per-seat or MAU SaaS, app catalog | Deployable without mandatory PEGIN Inc. control plane |
| **Citrix + AD** | Bundled remote access + slow group-based permissions | PePP + app-level access (see [permission-platform.md](../02-product/permission-platform.md)) |

PEGIN does **not** claim a specific total cost for N users without a named pilot worksheet.

---

## PEGIN model (qualitative)

| Element | Direction |
|---------|-----------|
| **Core software** | Open source — no seat license as the default business model |
| **Operator revenue** | SLAs, integration, managed peers — priced per engagement |
| **Customer upside** | Possible peer incentives / tokens — **hypothesis**, not promised returns |
| **Switching** | Protocol portability vs contractual lock-in to one IdP UI |

---

## Why incumbents rarely copy “free core”

Large vendors are built around **recurring seat revenue** and shareholder expectations. Shipping passkeys or another SAML endpoint is a feature; giving away the control plane is a **business-model change**.

PEGIN’s bet: some buyers care enough about **ownership, deployment model, and survivability** to adopt an early protocol — not that incumbents cannot add biometrics.

---

## What is not the moat

| Weak claim | Why |
|------------|-----|
| “Secret technology” | WebAuthn, DID, Rue/DIG are learnable; execution and network matter more |
| “More features than Entra” | Out of scope for v1 — see [enterprise-identity-spec.md](../08-developer/specs/enterprise-identity-spec.md) |
| “Proven cheaper” | Needs pilot TCO worksheets — see [sustainable-funding.md](sustainable-funding.md) |
| “Audit lives on blockchain” | Audit on **DIG**; Chia anchors store updates only |

---

## What might be the moat (validate in pilots)

1. **Aligned incentives** — core protocol not metered per seat by default.
2. **Survivability** — DID and credentials persist if one operator or employer exits (design goal).
3. **Community & integrations** — apps standardize on PEGIN OIDC/SAML and PePP on DIG.
4. **PePP workflows** — faster grant/revoke than ticket + AD + Citrix where measured.
5. **EU sovereignty fit** — buyers reducing US-hyperscaler identity and productivity cloud where law and policy require EU control (see below).

---

## European data sovereignty and regulation (opportunity)

> **Not legal advice.** Regulations below are summarized for product strategy. Each deployment needs counsel, DPIA, and sector-specific compliance (public sector, health, finance).

### Why European buyers are rethinking Microsoft and US SaaS identity

Many EU organizations treat **Entra ID / Azure AD** and **Microsoft 365** as one stack: identity, mail, files, and audit often flow through US-headquartered operators. That creates recurring questions under EU law:

- **Where is personal data processed?** (GDPR Chapter V — transfers outside the EEA)
- **Can a foreign authority compel access?** (debated under Schrems II and supplementary measures — even with Standard Contractual Clauses)
- **Can we exit the vendor?** (EU Data Act switching obligations for cloud)
- **Do we meet sector rules?** (NIS2 for essential entities; DORA for financial firms; national schemes such as France’s SecNumCloud)

**Sovereignty-oriented buyers** (public administration, regulated industries, defense suppliers, health, energy) are actively **reducing dependence** on proprietary US clouds — not always removing Microsoft overnight, but stripping features that require US processing, default telemetry, or immovable control planes.

**Example (Germany):** Schleswig-Holstein’s state government approved a move toward a **digitally sovereign workplace** (open document standards, open-source mail/collaboration, UCS instead of Active Directory in scope). Official announcement: [Digitally sovereign administration (Apr 2024, DE)](https://www.schleswig-holstein.de/DE/landesregierung/ministerien-behoerden/I/_startseite/Artikel2024/II/240403_digitalsouveraene_verwaltung.html).

**Example (Germany, federal):** The federation explores “sovereign” government cloud models (e.g. Delos / SAP–Microsoft structures) while states and civil-society critics argue about **real** vs **nominal** sovereignty — see [Bundestag press on Delos Cloud (DE)](https://www.bundestag.de/presse/hib/kurzmeldungen-1019308) and ongoing BSI evaluation. The debate itself shows demand for **EU-operated, auditable** alternatives.

PEGIN does **not** replace LibreOffice or full M365 migration. It targets the **identity and access gap**: SSO + permissions + audit without a mandatory US SaaS control plane for every login and grant.

### Regulatory map (official sources)

| Instrument | What it requires (high level) | Why identity / IAM matters | Official link |
|------------|------------------------------|----------------------------|---------------|
| **[GDPR](https://eur-lex.europa.eu/eli/reg/2016/679/oj)** (Regulation (EU) 2016/679) | Lawful basis, minimization, security, DPIA, processor agreements, **restrictions on transfers** outside EEA | Auth logs, directory attributes, and session metadata are often **personal data** | [EUR-Lex — GDPR](https://eur-lex.europa.eu/eli/reg/2016/679/oj) |
| **Schrems II** (CJEU C-311/18) + **[EDPB transfer guidance](https://www.edpb.europa.eu/sme-data-protection-guide/international-data-transfers_en)** | Transfers need adequacy or Art. 46 tools; assess third-country law; **supplementary measures** if needed | US IdP SaaS may trigger transfer impact assessments for EU employee/customer data | [EDPB — international transfers](https://www.edpb.europa.eu/sme-data-protection-guide/international-data-transfers_en) · [Recommendations 01/2020 (supplementary measures)](https://www.edpb.europa.eu/our-work-tools/our-documents/recommendations/recommendations-012020-measures-supplement-transfer_en) |
| **[EU–US Data Privacy Framework](https://commission.europa.eu/law/law-topic/data-protection/international-dimension-data-protection/adequacy-decisions_en)** (adequacy, 2023) | Allows some transfers to certified US organisations | Reduces friction for **certified** US vendors; does not end **all** sovereignty or sector policies | [Commission — adequacy decisions](https://commission.europa.eu/law/law-topic/data-protection/international-dimension-data-protection/adequacy-decisions_en) |
| **[EU Data Act](https://eur-lex.europa.eu/eli/reg/2023/2854/oj)** (Regulation (EU) 2023/2854) | Fair access to data; **switching** between cloud / on-prem processing; B2B unfair terms | Favours architectures customers can **move** or self-host — aligns with open core | [EUR-Lex — Data Act](https://eur-lex.europa.eu/eli/reg/2023/2854/oj) · [Commission — Data Act](https://digital-strategy.ec.europa.eu/en/policies/data-act) |
| **[NIS2](https://eur-lex.europa.eu/eli/dir/2022/2555/oj)** (Directive (EU) 2022/2555) | Cyber risk management, incident reporting, supply chain security for **essential** and important entities | Access control, logging, timely revocation, vendor oversight | [EUR-Lex — NIS2](https://eur-lex.europa.eu/eli/dir/2022/2555/oj) · [ENISA — NIS2](https://www.enisa.europa.eu/topics/state-of-cybersecurity-in-the-eu/cybersecurity-policies/nis-directive-2) |
| **[DORA](https://eur-lex.europa.eu/eli/reg/2022/2554/oj)** (Regulation (EU) 2022/2554) | ICT risk, resilience testing, incident reporting; **oversight of critical ICT third parties** (from 2025) | Core banking IdP / cloud identity often classified as **ICT third-party risk** | [EUR-Lex — DORA](https://eur-lex.europa.eu/eli/reg/2022/2554/oj) · [EBA — DORA oversight](https://www.eba.europa.eu/activities/direct-supervision-and-oversight/digital-operational-resilience-act/dora-oversight) |
| **[EU Cloud Sovereignty Framework](https://commission.europa.eu/document/download/09579818-64a6-4dd5-9577-446ab6219113_en)** (Commission, 2025) | Criteria for **legal, data, operational, and strategic** sovereignty of cloud services | Buyers will score vendors on EU jurisdiction, control, and extraterritorial exposure | [PDF — Cloud Sovereignty Framework](https://commission.europa.eu/document/download/09579818-64a6-4dd5-9577-446ab6219113_en) |
| **[SecNumCloud](https://cyber.gouv.fr/enjeux-technologiques/cloud/faq-qualification-secnumcloud/)** (France, ANSSI) | Qualification for trusted cloud services used by sensitive French public/private sectors | Pattern for **EU-hosted, EU-operated** stacks PEGIN could run on | [ANSSI — SecNumCloud FAQ](https://cyber.gouv.fr/enjeux-technologiques/cloud/faq-qualification-secnumcloud/) |

Member states may add **national** rules (e.g. Germany BSI IT-Grundschutz, sector KRITIS guidance). Treat the table as a reading list, not a compliance checklist.

### The gap PEGIN can address (hypothesis)

| Buyer pain (EU sovereignty) | Incumbent pattern | PEGIN direction |
|----------------------------|-------------------|-----------------|
| Identity tied to US SaaS control plane | Entra / Okta hosted or operated under US corporate structure | **Open protocol**; operator or customer runs SSO in **EU jurisdiction** |
| Permission + audit in vendor Postgres | M365 / Entra logs exportable but vendor-held | **Grants and audit on DIG** (EU peers); **store-update anchors** on Chia — see [fully-decentralized.md](../01-vision/fully-decentralized.md) |
| Cannot exit without re-provisioning thousands of apps | Contractual lock-in | **OIDC/SAML** federation; parallel run then cutover ([enterprise-identity-spec.md](../08-developer/specs/enterprise-identity-spec.md)) |
| Removing M365 but still need SSO + access | Ad-hoc LDAP / on-prem AD remains | PEGIN SSO + PePP without restoring Citrix-style sprawl |

**Positioning sentence (strategy only):** For sovereignty-minded EU organisations, PEGIN is a candidate **identity and permission layer** that can run on **EU infrastructure** they choose — complementary to open-source workplace projects (e.g. open mail, UCS/LDAP bridges), not a claim of automatic GDPR certification.

### What PEGIN does not solve by default

| Expectation | Reality |
|-------------|---------|
| “GDPR-compliant out of the box” | Requires lawful basis, retention, subprocessors, DPIA, DPO processes — product design enables; legal sign-off is customer-specific |
| “No US law exposure ever” | Depends on operator, hosting, support staff, and whether any US subprocessor remains in the chain |
| “Replaces SecNumCloud qualification” | SecNumCloud / EUCS are **separate** certifications of the **hosting and operations** stack |
| “Eliminates need for SCCs” | If **any** personal data still flows to non-EEA processors, Chapter V analysis still applies |

### Go-to-market notes (EU)

1. **Lead with jurisdiction and deployment model**, not price — align with [Cloud Sovereignty Framework](https://commission.europa.eu/document/download/09579818-64a6-4dd5-9577-446ab6219113_en) language (legal / data / operational sovereignty).
2. **Offer EU operator playbook** — peer in customer DC or qualified EU cloud (SecNumCloud-style host where required).
3. **Ship compliance artifacts** — DPIA template, RoPA hints, audit export from DIG, subprocessors list (when product exists).
4. **Pilot with one sovereignty buyer** — public sector, community bank, or critical supplier; document transfer analysis and revocation drill.

---

## Proof required

- Case studies with **measured** cost, time-to-grant, and time-to-revoke.
- Security and compliance sign-off per vertical (including **EU DPIA** and sector rules where claimed).
- Production deployments on testnet/mainnet.
- At least one **EU sovereignty** pilot with documented hosting jurisdiction and transfer analysis.

---

## Related docs

- [business-principles.md](../01-vision/business-principles.md)
- [sustainable-funding.md](sustainable-funding.md)
- [differentiators.md](../03-use-cases/differentiators.md)
- [permission-platform.md](../02-product/permission-platform.md)
- [fully-decentralized.md](../01-vision/fully-decentralized.md) — DIG deployment and audit model
- [enterprise-business-plan.md](enterprise-business-plan.md) — enterprise GTM (illustrative only)
