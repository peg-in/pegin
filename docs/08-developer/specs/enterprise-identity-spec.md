# Spec 2 — Enterprise identity (AD / Entra parity)

> **Developer hub:** [../README.md](../README.md)  
> **Goal:** PEGIN as a decentralized IdP that can **federate with** or **replace** Microsoft Entra ID (Azure AD) for app SSO — standard protocols only (SAML/OIDC apps).  
> **Plan:** [roadmap.md](../../03-use-cases/roadmap.md) Phase 1 → Phase 3 · **Spec 1:** [tech-stack.md](tech-stack.md)

---

## Official documentation index

Only **normative or vendor-maintained** specs and docs are listed below. Rust crate docs (e.g. `samael`, `openidconnect`) implement these standards; they are not listed here.

### Authentication (passkeys)

| Standard | Official documentation |
|----------|------------------------|
| Web Authentication Level 3 | [W3C WebAuthn](https://www.w3.org/TR/webauthn-3/) |
| WebAuthn Level 2 (widely deployed) | [W3C WebAuthn L2](https://www.w3.org/TR/webauthn-2/) |
| FIDO2 / CTAP | [FIDO Alliance specifications](https://fidoalliance.org/specifications/download/) |

### OIDC & OAuth (modern app SSO)

| Standard | Official documentation |
|----------|------------------------|
| OpenID Connect Core 1.0 | [openid-connect-core-1_0](https://openid.net/specs/openid-connect-core-1_0.html) |
| OpenID Connect Discovery 1.0 | [openid-connect-discovery-1_0](https://openid.net/specs/openid-connect-discovery-1_0.html) |
| Dynamic Client Registration | [openid-connect-registration-1_0](https://openid.net/specs/openid-connect-registration-1_0.html) |
| All Connect specs | [OpenID Connect specifications](https://openid.net/developers/specs/connect/) |
| OAuth 2.0 Authorization Framework | [RFC 6749](https://www.rfc-editor.org/rfc/rfc6749) |
| OAuth 2.0 Bearer Token | [RFC 6750](https://www.rfc-editor.org/rfc/rfc6750) |
| JSON Web Token (JWT) | [RFC 7519](https://www.rfc-editor.org/rfc/rfc7519) |
| JSON Web Signature (JWS) | [RFC 7515](https://www.rfc-editor.org/rfc/rfc7515) |

### SAML 2.0 (enterprise web SSO)

| Standard | Official documentation |
|----------|------------------------|
| SAML 2.0 (OASIS standard page) | [OASIS SAML v2.0](https://www.oasis-open.org/standard/saml/) |
| SAML 2.0 Technical Overview | [SAML 2.0 Technical Overview](https://docs.oasis-open.org/security/saml/Post2.0/sstc-saml-tech-overview-2.0.html) |
| SAML 2.0 specifications (index) | [docs.oasis-open.org/security/saml/v2.0/](https://docs.oasis-open.org/security/saml/v2.0/) |
| SAML Core | [saml-core-2.0-os](https://docs.oasis-open.org/security/saml/v2.0/saml-core-2.0-os.html) |
| SAML Bindings | [saml-bindings-2.0-os](https://docs.oasis-open.org/security/saml/v2.0/saml-bindings-2.0-os.html) |
| SAML Profiles | [saml-profiles-2.0-os](https://docs.oasis-open.org/security/saml/v2.0/saml-profiles-2.0-os.html) |
| SAML Metadata | [saml-metadata-2.0-os](https://docs.oasis-open.org/security/saml/v2.0/saml-metadata-2.0-os.html) |

### SCIM (provisioning / deprovisioning)

| Standard | Official documentation |
|----------|------------------------|
| SCIM 2.0 Core Schema | [RFC 7643](https://www.rfc-editor.org/rfc/rfc7643) |
| SCIM 2.0 Protocol | [RFC 7644](https://www.rfc-editor.org/rfc/rfc7644) |

### LDAP (legacy directory access)

| Standard | Official documentation |
|----------|------------------------|
| LDAP: The Protocol | [RFC 4511](https://www.rfc-editor.org/rfc/rfc4511) |
| LDAP: String Representation of DN | [RFC 4514](https://www.rfc-editor.org/rfc/rfc4514) |
| LDAP: Directory Information Tree | [RFC 4512](https://www.rfc-editor.org/rfc/rfc4512) |

### Microsoft Entra ID (Azure AD) — federation & app integration

Use these when PEGIN must interoperate with or migrate from Entra:

| Topic | Official documentation |
|-------|------------------------|
| SAML 2.0 IdP for hybrid SSO | [Use a SAML 2.0 IdP for SSO (Entra Connect)](https://learn.microsoft.com/en-us/entra/identity/hybrid/connect/how-to-connect-fed-saml-idp) |
| SAML/WS-Fed direct federation (overview) | [SAML/WS-Fed identity provider federation](https://learn.microsoft.com/en-us/entra/external-id/direct-federation-overview) |
| Configure external SAML IdP | [Add a SAML identity provider](https://learn.microsoft.com/en-us/entra/external-id/direct-federation) |
| Enterprise application SAML SSO | [Microsoft identity platform SAML SSO](https://learn.microsoft.com/en-us/entra/identity-platform/saml-sso-setup) |
| SAML protocol requirements (claims, endpoints) | [SAML protocol requirements](https://learn.microsoft.com/en-us/entra/identity-platform/saml-protocol-reference) |
| SCIM provisioning to apps | [SCIM provisioning with Microsoft Entra](https://learn.microsoft.com/en-us/entra/identity/app-provisioning/use-scim-to-provision-users-and-groups) |
| SCIM connector development | [Build a SCIM connector](https://learn.microsoft.com/en-us/entra/identity/app-provisioning/use-scim-to-provision-users-and-groups#build-a-scim-endpoint) |
| WS-Federation (legacy Microsoft apps) | [Configure WS-Fed SSO](https://learn.microsoft.com/en-us/entra/identity-platform/v2-protocols-ws-federation) |
| Kerberos / hybrid identity (context) | [Hybrid identity documentation](https://learn.microsoft.com/en-us/entra/identity/hybrid/) |

### Chia identity primitives (anchor layer)

PEGIN anchors users on Chia DIDs; enterprise protocols sit **above** this layer.

| Topic | Official documentation |
|-------|------------------------|
| Chia documentation home | [docs.chia.net](https://docs.chia.net/) |
| DIDs (concept) | [Academy — DIDs](https://docs.chia.net/academy-did) |
| DID RPC | [DID RPC reference](https://docs.chia.net/reference-client/rpc-reference/did-rpc) |
| DID CLI | [DID CLI reference](https://docs.chia.net/reference-client/cli-reference/did-cli/) |
| Verifiable credentials | [Verifiable Credentials guide](https://docs.chia.net/guides/verifiable-credentials-guide/) |
| Primitives overview | [Chia primitives](https://docs.chia.net/guides/primitives) |

---

## PEGIN role in the stack

```
┌─────────────────────────────────────────────────────────────┐
│ Enterprise apps (SaaS, custom SPs) │
│ SAML 2.0 SP · OIDC RP · LDAP client · SCIM consumer │
└───────────────────────────┬─────────────────────────────────┘
 │ standard protocols
┌───────────────────────────▼─────────────────────────────────┐
│ pegin-protocols (Rust) │
│ SAML IdP · OIDC OP · OAuth AS · SCIM server · LDAP gateway │
└───────────────────────────┬─────────────────────────────────┘
 │
┌───────────────────────────▼─────────────────────────────────┐
│ pegin-core — WebAuthn · sessions (JWT) · DID (chia-wallet-sdk)│
└───────────────────────────┬─────────────────────────────────┘
 │
┌───────────────────────────▼─────────────────────────────────┐
│ Chia DID + Rue contracts · DIG (sessions, audit, PePP) │
└─────────────────────────────────────────────────────────────┘
```

**Competing with Azure SSO** means implementing the **same open protocols** Entra exposes to applications, not replicating every Entra portal feature on day one.

---

## Protocol rollout (PEGIN)

| Priority | Protocol | PEGIN role | Typical Azure AD role | Phase |
|----------|----------|------------|------------------------|-------|
| 1 | WebAuthn / FIDO2 | Primary user auth | Passkey / WHfB | 0 |
| 2 | OIDC | IdP (OP) for modern apps | Entra OIDC | 1 |
| 3 | SAML 2.0 | IdP for enterprise SPs | Entra SAML IdP | 1 |
| 4 | OAuth 2.0 | Authorization server | Entra OAuth | 1 |
| 5 | SCIM 2.0 | Provisioning endpoint | Entra → app provisioning | 3 |
| 6 | LDAP | Read-only / auth gateway for legacy | AD LDAP | 3 |
| 7 | WS-Federation | Legacy Microsoft SPs | Entra WS-Fed | 3+ |

Reference implementation crates (non-official): `passkey`, `openidconnect`, `oauth2`, `samael` — see [tech-stack.md](tech-stack.md).

---

## Active Directory / Entra feature map

| Capability | Entra / AD (typical) | PEGIN approach | Protocol / doc |
|------------|----------------------|----------------|----------------|
| User authentication | Password / passkey / MFA | Passkey + Chia DID | [WebAuthn](https://www.w3.org/TR/webauthn-3/), [DIDs](https://docs.chia.net/academy-did) |
| SSO to cloud apps | SAML / OIDC IdP | `pegin-protocols` IdP | [SAML](https://www.oasis-open.org/standard/saml/), [OIDC Core](https://openid.net/specs/openid-connect-core-1_0.html) |
| User provisioning | SCIM / sync | SCIM server + optional on-chain merkle root | [RFC 7644](https://www.rfc-editor.org/rfc/rfc7644) |
| Group / role claims | AD groups → token claims | Groups in DIG + claims in SAML/OIDC assertions | [SAML attributes](https://docs.oasis-open.org/security/saml/v2.0/saml-core-2.0-os.html), [OIDC claims](https://openid.net/specs/openid-connect-core-1_0.html#Claims) |
| Offboarding | Disable account + revoke sessions | Revoke permissions + invalidate sessions | SCIM PATCH/DELETE; PePP Phase 2 |
| Audit | Entra sign-in logs | Append-only DIG + export | Product design; align with customer SIEM |
| Conditional access | Entra CA policies | Rue / policy engine (future) | Phase 3+ |
| M365 apps | Entra-native | **Stay on Entra** or federate | [SAML IdP federation](https://learn.microsoft.com/en-us/entra/identity/hybrid/connect/how-to-connect-fed-saml-idp) |

---

## Migration patterns (official Microsoft guidance)

These Microsoft docs define how enterprises already think about IdP coexistence — PEGIN should support the same patterns as an **external SAML/OIDC IdP**:

1. **PEGIN as SAML IdP, Entra as hub (hybrid)** 
 Follow claim and endpoint requirements: [SAML protocol reference](https://learn.microsoft.com/en-us/entra/identity-platform/saml-protocol-reference).

2. **Federation / direct federation** 
 [Direct federation overview](https://learn.microsoft.com/en-us/entra/external-id/direct-federation-overview) — useful when partners already have their own IdP.

3. **SCIM from Entra into PEGIN (mirror users)** 
 [Automate provisioning with SCIM](https://learn.microsoft.com/en-us/entra/identity/app-provisioning/use-scim-to-provision-users-and-groups) — Entra provisions **into** PEGIN’s SCIM endpoint during migration.

4. **Parallel SSO** 
 Non-Microsoft apps on PEGIN SAML/OIDC; Microsoft 365 remains on Entra until explicitly migrated.

---

## SAML IdP checklist (implementation)

Minimum artifacts PEGIN must publish (per [SAML Metadata](https://docs.oasis-open.org/security/saml/v2.0/saml-metadata-2.0-os.html)):

- Entity ID (URI)
- SSO URL (HTTP-Redirect or HTTP-POST binding per [Bindings](https://docs.oasis-open.org/security/saml/v2.0/saml-bindings-2.0-os.html))
- Signing certificate (X.509)
- NameID format policy
- Attribute statements (email, groups, optional `did:chia:…` as custom attribute)

Test against Microsoft’s enterprise app flow: [Configure SAML SSO for an application](https://learn.microsoft.com/en-us/entra/identity-platform/saml-sso-setup).

---

## OIDC provider checklist

Per [OpenID Connect Core](https://openid.net/specs/openid-connect-core-1_0.html) and [Discovery](https://openid.net/specs/openid-connect-discovery-1_0.html):

- `/.well-known/openid-configuration`
- `authorization_endpoint`, `token_endpoint`, `jwks_uri`
- `id_token` signed (JWS per [RFC 7515](https://www.rfc-editor.org/rfc/rfc7515))
- `sub` stable per user DID or internal subject id mapped to DID

---

## SCIM checklist

Per [RFC 7644](https://www.rfc-editor.org/rfc/rfc7644) and schema [RFC 7643](https://www.rfc-editor.org/rfc/rfc7643):

- `/Users`, `/Groups` resources
- Bearer auth (or mutual TLS per customer policy)
- Compatible with Entra provisioning agent expectations: [Microsoft SCIM doc](https://learn.microsoft.com/en-us/entra/identity/app-provisioning/use-scim-to-provision-users-and-groups)

---

## What “full AD replacement” does **not** mean

- **Not in scope for open protocols alone:** Active Directory Domain Services (Kerberos KDC, GPO, NT LAN Manager, on-prem AD DS replication). See [Hybrid identity](https://learn.microsoft.com/en-us/entra/identity/hybrid/) for Microsoft’s model.
- **Realistic v1:** Replace **cloud SSO + provisioning** for SAML/OIDC/SCIM apps; federate with Entra where M365 requires it.
- **Kerberos / domain join:** Optional late phase; reference [Microsoft hybrid docs](https://learn.microsoft.com/en-us/entra/identity/hybrid/) only if product expands.

---

## Conformance testing (suggested)

| Area | Official test / tooling |
|------|-------------------------|
| OIDC | [OpenID Connect conformance suite](https://openid.net/certification/) (when ready for certification) |
| SAML | OASIS conformance docs: [SAML Conformance](https://docs.oasis-open.org/security/saml/v2.0/saml-conformance-2.0-os.html) |
| WebAuthn | [FIDO Certification](https://fidoalliance.org/certification/) |
| Entra integration | Enterprise app test tenant + [SAML SSO setup guide](https://learn.microsoft.com/en-us/entra/identity-platform/saml-sso-setup) |

---

## Related PEGIN docs

- [tech-stack.md](tech-stack.md) — Spec 1: crates, architecture, xch-dev, Yakuhito naming
- [roadmap.md](../../03-use-cases/roadmap.md) — when each protocol ships
- [permission-data-model.md](../permissions/permission-data-model.md) — PePP (post-SSO)