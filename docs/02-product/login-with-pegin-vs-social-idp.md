# Login with PEGIN — same UX as Apple, different trust model

> **Discussion doc** — how mini wallet + DID relate to JWT for normal websites, and what “better than Sign in with Apple” means without overselling MVP.

**Related:** [existing-apps-and-sso-protocols.md](../08-developer/integration/existing-apps-and-sso-protocols.md) · [mini-wallet-and-recovery-vault.md](../10-architecture/mini-wallet-and-recovery-vault.md) · [pegin-manifest.md](../01-vision/pegin-manifest.md)

---

## Two layers (users see one; engineers get both)

| Layer | Who sees it | What it does |
|-------|-------------|--------------|
| **A — Public SSO (websites)** | User + every SaaS app | **JWT** from **wallet (IdP)** with `exp`, like Sign in with Apple |
| **B — Mini wallet + DID (PEGIN)** | Inside wallet at login | Passkey + DID verify → wallet **signs JWT** → site receives token |

```
MySaaS page (stays open)  ── ONE BUTTON ──▶  overlay/popup  ──▶  Face ID
       ◀── JWT (postMessage) ──  instant next visit if session valid
       (no redirect, no PEGIN branding when already logged in)
```

**MVP ships layer A completely; layer B runs behind the auth service** (not “connect wallet” on each site).

---

## Parity with Sign in with Apple (table stakes)

| Expectation | Sign in with Apple | Login with PEGIN (target) |
|-------------|-------------------|---------------------------|
| One button on site | Yes | Yes — **best-in-class** `<PeginButton />` |
| Face ID / passkey | Yes | Yes (WebAuthn) |
| No full-page redirect | Apple often uses sheet | **Yes** — popup/overlay + `postMessage` (default SDK) |
| App gets JWT / stable `sub` | Yes | Yes (`id_token`, `sub`, `exp`) |
| No crypto on developer side | Yes | Yes |
| Works in browser | Yes | Yes |
| Privacy: hide email option | Apple relay | Policy + minimal claims (product choice) |

If any of these fail, developers will not adopt — **decentralization is not a substitute for OIDC**.

---

## Additional benefits (honest — mostly post-anchor, some MVP)

| Benefit | What it means | MVP | Later |
|---------|---------------|-----|-------|
| **Portable identity** | Same `sub`/DID across apps; not owned by one SaaS directory | Anchor DID | PePP, credentials |
| **Decentralized anchor** | Identity survives PEGIN Inc. drama; verifiable on Chia | Testnet DID | Mainnet + open specs |
| **Privacy of login event** | Where/when auth happened — user-held or DIG, not sold as ads graph | Basic audit on DIG | Selective disclosure |
| **No seat-tax IdP** | Open core; self-host IdP | Self-host path doc | Operator market |
| **Login → wallet (future)** | Same account could sign txs | **Out of MVP** | Mini wallet exposes sign API |
| **Recovery you control** | Multi-key vault, not Apple-only account recovery | Design + partial | Full guardian + Signer |

**Do not claim in MVP:** “replace Apple Pay,” “sign transactions on every login,” or “fully private from all operators” without defining threat model.

---

## User journey (one story)

1. **Alice** taps **Login with PEGIN** on `shop.example` (same mental model as Apple).
2. Popup / redirect to PEGIN → **Face ID** (~1s).
3. PEGIN **mini wallet** (server-side or Tauri) already has her DID; passkey proves “it’s Alice today.”
4. PEGIN returns OIDC **JWT** to `shop.example` → shop maps `sub` → `users` row → session cookie.
5. Alice never sees Chia, XCH, or “wallet” on the shop site.
6. **Later:** Alice uses same PEGIN on `work.example` — same identity, no new corporate-owned directory row required (shop/work still store their own profiles).

---

## What websites integrate (only JWT/OIDC)

Developers add:

- Client id, redirect URI, JWKS validation
- Optional: `pegin_did` claim for apps that want on-chain audit

They **do not** integrate DID libraries, vaults, or Chia RPC.

---

## What PEGIN operates (mini wallet + IdP)

| Component | Role |
|-----------|------|
| `pegin-auth` | OIDC OP, passkey RP, JWT signing, session refresh |
| `pegin-wallet` | Create/hold DID + vault; background txs; future spend API |
| DIG | Passkey↔DID binding, login audit metadata (not full surveillance) |
| Faucet | Testnet fees at signup only |

---

## Privacy: “where login happened”

**Apple:** hides email; Apple still knows auth events.  
**PEGIN direction:**

- **Apps** get standard OIDC claims (what you configure).
- **Login metadata** (IP, device, RP id) → user-visible DIG store or self-hosted peer — not resold as behavioral graph (manifest P5).
- **Relying parties** verify JWT offline; optional future: ZK proof “logged in” without revealing RP list (not MVP).

Discuss openly: self-hosted PEGIN node = stronger locality; managed PEGIN = easier, more trust in operator.

---

## Future: login account = mini wallet (not MVP)

Same `sub`/DID could later:

- Sign Chia spends with explicit user consent in Signer UI
- Hold credentials (VCs)
- Approve PePP grants on phone

**MVP boundary:** JWT SSO only; wallet signing API **disabled** or not exposed to third-party sites.

---

## Positioning sentence

> **Login with PEGIN** feels like **Sign in with Apple** for every website; under the hood your identity is **anchored to a DID you can take everywhere**, with optional decentralization and wallet features when you need them — not before login works perfectly.

*Product discussion v0.1 · May 2026*
