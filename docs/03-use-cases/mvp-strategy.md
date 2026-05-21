# MVP — two steps (wallet + JWT first)

> **MVP does not lead with DIG network features.** Step 1 is **mini wallet + DID + JWT**. Step 2 is **vault + recovery** (seed phrase + passkey, multi-device). DIG as full application layer comes **after** this core works.

**Architecture:** [mini-wallet-and-recovery-vault.md](../10-architecture/mini-wallet-and-recovery-vault.md) · [decentralized-verification-no-central-op.md](../10-architecture/decentralized-verification-no-central-op.md) · [dig-as-application-layer.md](../10-architecture/dig-as-application-layer.md) (post-MVP)

**Start coding:** [step1-implementation-bootstrap.md](../08-developer/engineering/step1-implementation-bootstrap.md) — repo layout, week 0 commands, crate list.

---

## MVP at a glance

| Step | Focus | Ship when |
|------|--------|-----------|
| **1 — Login** | Mini wallet, DID, **wallet issues JWT**, passkey, `@pegin/sdk` | First release |
| **2 — Recovery** | Vault (`VaultInfo` / Rue upstream), **seed phrase + passkey**, multi-device | Immediately after Step 1 stable |

**Not MVP-critical:** DIG per-app stores, email guardian, Chia Signer custody, PePP, SAML/SCIM, full “everything on DIG” application layer.

---

## Step 1 — Mini wallet + DID + JWT (most important)

### Goal

**PEGIN = decentralized IdP implemented in the wallet.** User proves identity with passkey inside the wallet; wallet mints **JWT** for any normal website (like Sign in with Apple). **Chia DID** is the stable identity root.

```
Website  ←── JWT (exp, sub, aud) ──  mini wallet (IdP)
                                      ├─ passkey ✓
                                      ├─ DID on Chia ✓
                                      └─ sign JWT (ES256 key in DID doc)
```

### In scope (Step 1)

| Item | Notes |
|------|--------|
| `pegin-wallet` | `chia-wallet-sdk` — create/hold DID, JWT signing key |
| `pegin-mini` | Tauri + React shell (Sage pattern) |
| `@pegin/sdk` | Button → wallet popup → receive JWT |
| WebAuthn | Register + login passkey |
| **JWT from wallet** | `sub` + **`preferred_username`** (like email); JWKS via DID doc — see [identity-username-and-account-flow.md](../10-architecture/identity-username-and-account-flow.md) |
| **Username + DID in wallet** | User picks unique username at **account create** (wallet); not on shop signup |
| Testnet faucet | Fund DID creation only (no vault yet in Step 1) |
| Demo OIDC-shaped validation | Apps verify JWT + map to local `users` table |
| Local / encrypted profile | **Wallet-local first** — not a DIG-network-led MVP |

### Out of scope (Step 1)

| Item | Deferred to |
|------|-------------|
| Recovery vault on chain | **Step 2** |
| Seed phrase UX | **Step 2** (not at signup) |
| Second device / multi-passkey | **Step 2** |
| DIG replicated stores (identity, audit, app data) | Post-MVP / parallel track |
| Email guardian, Chia Signer recovery | Post-MVP |
| PePP, SAML, SCIM, admin dashboard | Roadmap |

### Step 1 UX rules

- **One button to rule them all** — reference-quality `<PeginButton />`; see [user-facing-ux-principles.md](../02-product/user-facing-ux-principles.md).
- **No redirect** — popup/overlay only; shop tab never navigates to IdP URL.
- **Instant:** valid JWT → **no button**, no PEGIN UI (silent `restore()` on load).
- **Expired session:** one button tap → Face ID → in.
- **No** wallet / blockchain / DID copy on login.
- **Account first (wallet app):** username + passkey + DID once — then SSO everywhere.
- **No** seed or vault at first account create (vault = Step 2).

### Step 1 success criteria

| Metric | Target |
|--------|--------|
| Login (perceived) | &lt; 1s |
| Account created (perceived) | ~3s |
| JWT validates on 2+ demo apps | Yes |
| DID on testnet | Yes |
| User-visible seed at signup | **No** |

---

## Step 2 — Vault + recovery (seed phrase + passkey, multi-device)

### Goal

DID can be **recovered** and used on **multiple devices** via upstream **vault** puzzle + backup material — without complicating daily login.

### In scope (Step 2)

| Item | Notes |
|------|--------|
| **Recovery vault** | One per user; compose [Rigidity `VaultInfo` / Rue](https://github.com/xch-dev/chia-wallet-sdk) upstream |
| **Seed phrase** | Once in **Security → Backup**; one **vault share** — **not** used for normal login |
| **Cross-device login (Step 1)** | New desktop → **phone QR + passkey** — **no seed** — [cross-device-login-and-vault-recovery.md](../10-architecture/cross-device-login-and-vault-recovery.md) |
| **Passkey** | Phone primary; optional **second passkey** on trusted desktop after QR |
| **Multi-device** | Daily: QR or synced passkey; Recovery: vault m-of-n + timelock |
| m-of-n + timelock | e.g. 2-of-3 seed + passkey(s); cancel window on chain |

### Recovery MVP (simplified vs full doc)

| Method | Role in MVP Step 2 |
|--------|---------------------|
| **Seed phrase** | Offline backup share; proves recovery in wallet UI |
| **Passkey** | Daily login; **add/re-bind** passkey on new device after seed verification |
| Email guardian | **Not** MVP |
| Chia Signer | **Not** MVP |

Passkey remains **login**. Seed phrase is **recovery-only** (shown once, never on login path).

### Out of scope (Step 2)

- Full Penguin Vault product (tiers, custody APIs)
- DIG federated mail / guardian peers
- On-site transaction signing for third-party sites

### Step 2 success criteria

| Metric | Target |
|--------|--------|
| Lose device → recover with seed + set up new passkey | Demo path works on testnet |
| Vault spend + timelock in simulator | Yes |
| Daily login still no vault unlock | Yes |

---

## What about DIG?

| Phase | DIG role |
|-------|----------|
| **MVP Step 1–2** | Optional minimal hook later; **not** primary deliverable |
| **Post-MVP** | Application layer — identity replication, audit, per-app data ([dig-as-application-layer.md](../10-architecture/dig-as-application-layer.md)) |

MVP truth can live in **wallet encrypted store + Chia** until DIG integration is product-ready.

---

## Week plan (revised estimate)

| Weeks | Step | Work |
|-------|------|------|
| 1–3 | **1** | `pegin-wallet`, DID, JWT sign/verify, `chia-sdk-test` |
| 4–5 | **1** | WebAuthn, `@pegin/sdk` popup, `pegin-mini`, faucet |
| 6–7 | **1** | Demo apps, testnet, JWT docs |
| 8–10 | **2** | Vault create (upstream), seed setup UX |
| 11–12 | **2** | Multi-device passkey re-bind, recovery e2e |

---

## MVP feature table

| Feature | Step 1 | Step 2 |
|---------|--------|--------|
| Mini wallet + DID | Yes | Yes |
| Wallet-issued JWT | Yes | Yes |
| Passkey login | Yes | Yes |
| `@pegin/sdk` | Yes | Yes |
| Testnet faucet | Yes | Yes |
| Recovery vault | No | Yes |
| Seed phrase recovery | No | Yes |
| Multi-device (passkey) | No | Yes |
| DIG app-layer stores | No | No |
| Email / Chia Signer recovery | No | No |
| PePP / SAML / SCIM | No | No |

---

## What MVP proves

1. **Wallet-as-IdP** with DID-anchored JWT works for real websites.  
2. **Vault + seed + passkey** can restore DID on a new device without central PEGIN DB.  
3. Login stays instant; recovery stays off the daily path.

## After MVP

- DIG application layer integration  
- Email guardian, Chia Signer ([recovery-vault-and-guardians.md](../10-architecture/recovery-vault-and-guardians.md))  
- [roadmap.md](roadmap.md) Phase 1+ (OIDC hardening, SAML, PePP)
