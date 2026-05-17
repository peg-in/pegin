# Structural differentiators vs centralized SSO

> **Purpose:** Explain what a user-owned DID enables that traditional IdPs are not designed to do. 
## Centralized SSO (typical)

- Identity record lives with the IdP or employer directory.
- Employer can delete the account; history and access proofs often go with it.
- Each org commonly issues a separate corporate identity.

## PEGIN model (target)

- User holds a **Chia DID** (keys in device secure enclave).
- Employer/app issues **signed permissions** to that DID; revocation does not delete the DID.
- PEGIN software facilitates protocols; it does not need to custody the identity.

---

## Differentiator 1 — Identity outlives the employer

If a company shuts down or wipes directory accounts, a DID can still exist on-chain. **Verifiable credentials** (who issued them, what they attest) are a separate product decision — Phase 1+ with clear issuer trust model.

## Differentiator 2 — One DID, many grantors

A contractor could hold one DID and collect permissions from multiple organizations without merging five corporate accounts — requires federation and issuer trust (Phase 3).

## Differentiator 3 — Revocation without “delete the person”

Offboarding revokes **permissions** on DIG (or equivalent); the person keeps their DID. Contrast with deleting `user@company.com` in a central directory.

## Differentiator 4 — No single vendor kill switch

Open source + on-chain anchor: operations can continue if PEGIN Inc. stops; customers can self-host or run peers. **Operational burden** is real — document runbooks, don’t oversimplify.

## Differentiator 5 — App-defined permissions (with PePP)

Apps publish permission vocabularies; grants are stored for fast lookup (see [permission-data-model.md](../10-architecture/permission-data-model.md)). Central IdPs often own the permission model inside their product — PEGIN aims for app-owned schemas on shared storage.

---

## What traditional SSO already does well

PEGIN should not claim to “replace everything on day one”:

- Mature SAML/OIDC ecosystems, HR-driven provisioning, Gartner-approved support models.
- Passkeys — already on Okta, Entra, Auth0.

**Honest pitch:** PEGIN competes on **ownership model, deployment model, and permission architecture**, once POC and pilots prove operability.

## Validate with design partners

| Question | Method |
|----------|--------|
| Do users understand DID + passkey UX? | Usability tests |
| Is revocation fast enough for security team? | Timed revoke drills |
| Will legal/compliance accept audit exports? | Review with customer counsel |
| TCO vs current stack? | Pilot spreadsheet, not doc estimates |