# POC & MVP — Login with PEGIN

## The one feature

```
User → "Login with PEGIN" → passkey (WebAuthn) → Chia DID verified → JWT → app
```

**In scope:** Registration, login, testnet DID, OIDC for demo apps, basic recovery design. 
**Out of scope:** Permissions (PePP), SAML, SCIM, admin UI, billing, production SLA.

## Why passkey-first

- Aligns with FIDO2/WebAuthn (phishing-resistant, device biometrics).
- Competitors already offer passkeys; PEGIN’s difference is **user-held DID on Chia** + open, decentralized deployment — not “passkeys alone.”
- Blockchain stays infrastructure; users see familiar biometric login.

## POC flow

```
SaaS app → Login with PEGIN → PEGIN (Rust/Axum)
 → first visit: register passkey + create DID
 → return visit: assert passkey + issue JWT
 → redirect back to app
```

## Week plan (estimate)

| Weeks | Work |
|-------|------|
| 1–2 | `pegin-core` skeleton, Chia testnet, minimal DIG hook |
| 3–4 | WebAuthn registration, DID (Rue) |
| 5–6 | Login challenge, JWT, OIDC discovery |
| 7 | Integrate 2–3 test apps |
| 8 | Testnet deploy, docs, basic security pass |

## MVP feature table

| Feature | POC |
|---------|-----|
| Passkey register / login | Yes |
| Chia DID anchor | Yes |
| OIDC | Yes (minimal) |
| Email recovery | Basic only |
| Permissions (PePP) | No → Phase 2 |
| SAML / SCIM | No → Phase 1–3 |
| Admin dashboard | No |

## What POC proves

- WebAuthn + Chia DID + JWT path works end-to-end.
- OIDC relying parties can integrate via SDK.
- Decentralized anchor is practical for developers.

## What POC does not prove

- Enterprise TCO vs Okta/Azure/Citrix.
- Permission approval speed at scale.
- Token economics or DIG peer revenue.

## After POC

Follow [roadmap.md](roadmap.md): harden OIDC/SAML (Phase 1), then PePP (Phase 2) with pilot metrics.