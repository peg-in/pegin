# 🐧 PEGIN Complete Ecosystem — SSO + Permission Management

## Executive summary

PEGIN combines **passwordless SSO** (user-held DID) with a planned **permission layer (PePP)** on DIG. This doc describes how the pieces fit; metrics come from future pilots.

---

## The two problems (directional)

### Pain point #1 — SSO

**Problem:** Tool sprawl, password friction, vendor-coupled identity. 
**PEGIN direction:** Passkey login, OIDC/SAML path, decentralized anchor. 
**Timeline:** POC ~8 weeks → v1 per [roadmap](../03-use-cases/roadmap.md).

### Pain point #2 — Permissions and remote access (PePP, later)

**Problem:** Slow request/approve/sync cycles, plus Citrix/VPN stacks that deliver **network or desktop sessions** instead of **app-level, revocable access**. 
**PEGIN direction:** PePP for grants on DIG; identity-bound app gateway instead of full-tunnel VPN for most web/SaaS — see [permission-platform.md](permission-platform.md) (Citrix critique, migration wedge, value table).

**Time to value:** 8 weeks rollout, full ROI in < 1 month

---

## How PEGIN SSO + Permission Platform Work Together

### Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│ PEGIN ECOSYSTEM │
│ │
│ ┌──────────────────────────────────────────────────────┐ │
│ │ PEGIN PLATFORM LAYER │ │
│ │ │ │
│ │ ┌─────────────────┐ ┌──────────────────┐ │ │
│ │ │ SSO Engine │ │ Permission Eng. │ │ │
│ │ │ (Passkeys) │◀───────→│ (Capabilities) │ │ │
│ │ │ ├─ WebAuthn │ │ ├─ Rules Engine │ │ │
│ │ │ ├─ OIDC │ │ ├─ Multi-sig │ │ │
│ │ │ ├─ SAML │ │ ├─ Auto-revoke │ │ │
│ │ │ └─ LDAP │ │ └─ Audit logs │ │ │
│ │ └─────────────────┘ └──────────────────┘ │ │
│ │ │ │
│ └──────────────┬───────────────┬──────────────────────┘ │
│ │ │ │
│ ┌──────────────▼────────────────▼─────────────────────┐ │
│ │ BLOCKCHAIN + DIG DATA LAYER │ │
│ │ │ │
│ │ ┌──────────────────────────────────────────────┐ │ │
│ │ │ Chia Blockchain │ │ │
│ │ │ ├─ DID registration (identity anchor) │ │ │
│ │ │ ├─ Credential NFTs (optional proofs) │ │ │
│ │ │ └─ Store-update anchors (hash/root) │ │ │
│ │ └──────────────────────────────────────────────┘ │ │
│ │ │ │
│ │ ┌──────────────────────────────────────────────┐ │ │
│ │ │ DIG Network (P2P Data Storage) │ │ │
│ │ │ ├─ User identity data (encrypted) │ │ │
│ │ │ ├─ Grants & permission rules │ │ │
│ │ │ ├─ Audit logs (append-only, replicated) │ │ │
│ │ │ └─ Session state (temporary) │ │ │
│ │ └──────────────────────────────────────────────┘ │ │
│ │ │ │
│ └─────────────────────────────────────────────────────┘ │
│ │
│ ┌──────────────────────────────────────────────────────┐ │
│ │ USER INTERACTION LAYER │ │
│ │ │ │
│ │ ┌─────────────────┐ ┌──────────────────┐ │ │
│ │ │ Employee App │ │ Manager App │ │ │
│ │ │ (Login) │ │ (Grant Access) │ │ │
│ │ │ │ │ │ │ │
│ │ │ "Login with │ │ Notification: │ │ │
│ │ │ PEGIN" │ │ "Alex needs │ │ │
│ │ │ ↓ │ │ GitHub access" │ │ │
│ │ │ [Face ID] │ │ [Approve 1 week] │ │ │
│ │ │ ✓ Logged in │ │ [Deny] │ │ │
│ │ └─────────────────┘ │ [Grant forever] │ │ │
│ │ └──────────────────┘ │ │
│ │ │ │
│ │ ┌──────────────────────────────────────────────┐ │ │
│ │ │ Admin Dashboard (Desktop) │ │ │
│ │ │ ├─ Permission rules editor │ │ │
│ │ │ ├─ Audit log viewer │ │ │
│ │ │ ├─ User management │ │ │
│ │ │ └─ Compliance reports │ │ │
│ │ └──────────────────────────────────────────────┘ │ │
│ │ │ │
│ └──────────────────────────────────────────────────────┘ │
│ │
│ ┌──────────────────────────────────────────────────────┐ │
│ │ APPLICATION INTEGRATION LAYER │ │
│ │ │ │
│ │ ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌────────┐ │ │
│ │ │ GitHub │ │ Slack │ │ Database │ │ Legacy │ │ │
│ │ │ (OAuth) │ │(SAML+JWT)│ │(LDAP+WH) │ │(custom)│ │ │
│ │ └──────────┘ └──────────┘ └──────────┘ └────────┘ │ │
│ │ │ │
│ └──────────────────────────────────────────────────────┘ │
│ │
└─────────────────────────────────────────────────────────────┘
```

---

## Real-World User Journey: SSO + Permission Together

### Day 1: New Employee Onboarding

**9:00 AM — Employee's First Login**

```
New employee (Alice) opens company web app
 ↓
"Login with PEGIN" button visible
 ↓
[Click] → Passkey registration:
 - Face ID scan
 - Creates Chia DID (blockchain anchor)
 - Creates PEGIN capability NFT
 ↓
Redirected to dashboard (logged in)
 ↓
Sees only apps she has access to:
 - Email (SAML from AD)
 - GitHub (OAuth, read-only)
 - Slack (JWT from PEGIN)
 - Staging database (Webhook)
 ↓
[Later] Alice tries to push code to main GitHub → DENIED
 (She only has read-only access)
```

### Day 1: Manager Grants Access

**9:30 AM — Manager Gets Permission Request**

```
Manager (Sarah) gets phone notification:
 "Alice needs: GitHub push-to-main, 2 weeks"
 [View Request] [Approve] [Deny]
 ↓
Sarah taps [Approve]
 - Face ID / Fingerprint
 - Confirms in 1 second
 ↓
PEGIN creates capability token:
 "Alice can push to main, until 2026-05-30"
 - NFT minted on Chia blockchain
 - Recorded in immutable audit log
 - GitHub webhook receives token
 ↓
Alice tries to push code 10 seconds later
 - GitHub checks: "Is there valid capability?"
 - Yes → Code pushed successfully
```

### Day 7: Alice Tries Unapproved Access

```
Alice tries to access production database
 ↓
Database checks PEGIN:
 "Does Alice have capability for prod-database?"
 ↓
PEGIN checks:
 - Alice's current role (junior engineer)
 - Time of access (2 AM = after hours!)
 - Device health (passed security check)
 - Manager approval (none for prod access)
 ↓
PEGIN rules say:
 "Prod access requires:
 - Manager approval
 - Business hours only
 - Device must pass security check
 - NOT after hours"
 ↓
Access DENIED
 ↓
Alice's manager gets alert:
 "Alice attempted unauthorized prod access at 2 AM"
 [Review incident]
```

### Day 14: Access Expires Automatically

```
Today: May 30, 10:23:45 AM
 ↓
PEGIN checks: "Is Alice's GitHub push capability still valid?"
 ↓
NO — Capability expired at May 30, 10:23:45
 ↓
PEGIN automatically revokes:
 - GitHub webhook: "Alice's push capability revoked"
 - Blockchain: "Revocation recorded"
 - Audit log: "Alice's access ended (normal expiration)"
 ↓
Alice tries to push code
 ↓
GitHub checks: "Is there valid capability?"
 ↓
NO → Access DENIED
 ↓
Alice or manager can request new access
 (Process repeats)
```

### Day 30: Employee Quits

```
HR system marks Alice as terminated
 ↓
SCIM sync sends termination event to PEGIN
 ↓
PEGIN immediately revokes ALL capabilities:
 - GitHub push
 - Slack access
 - Database access
 - Staging access
 ↓
Cascade to all apps (via webhooks):
 - GitHub: "Alice revoked"
 - Slack: "Alice revoked"
 - Database: "Alice revoked"
 - Legacy app: "Alice revoked"
 ↓
All happening in < 1 second
 ↓
DIG audit store (append) shows:
 "2026-05-30 16:45:32 — Employee ABC terminated"
 "2026-05-30 16:45:33 — 47 grants revoked on DIG"
 "Reason: Termination"
 (Store update anchored on Chia — payloads stay on DIG)
 ↓
No 3-7 day risk window
No forgotten systems
No data breach opportunity
```

---

## Value to measure in pilots

| Dimension | What to measure |
|-----------|-----------------|
| Login UX | Time to register/login; passkey adoption |
| Access workflow | Median request → grant (PePP) |
| Offboarding | Time until all integrated apps deny access |
| Operations | Run cost vs incumbent stack (customer-specific) |
| Compliance | Audit artifact quality (customer counsel sign-off) |

---

## Implementation paths (high level)

### Path A — Greenfield

Deploy SSO (Phase 0–1), add PePP when SSO is stable (Phase 2). Timeline depends on app count and team — **estimate per engagement**, not generic weeks in this doc.

### Path B — Alongside Azure AD / Okta

Parallel federation → migrate apps incrementally. See [enterprise-business-plan.md](../05-business/enterprise-business-plan.md) for migration **patterns**.

---

## Commercial model

See [sustainable-funding.md](../05-business/sustainable-funding.md). No validated price list or customer ROI in this repository.

---

## Stakeholder goals (to validate)

| Stakeholder | Goal |
|-------------|------|
| Employees | Fast passkey login; clear access status |
| Managers | Simple approve/deny on mobile |
| IT | Fewer access tickets; reliable offboarding |
| Security / compliance | Auditable grants and revokes |

---

## Conclusion

PEGIN’s bet is **user-held identity + phased SSO and permissions on DIG** — not a feature parity race with Entra ID on day one. Ship [POC](../03-use-cases/mvp-strategy.md), execute [roadmap](../03-use-cases/roadmap.md), replace generic ROI slides with pilot data.

---

*Built with 🐧 by the PEGIN team.*