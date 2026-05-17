# Permission data model (PePP)

Technical reference for Phase 2. Not implemented in POC.

## Flow

```
App defines permission groups (JSON)
    → stored / referenced on DIG
User DID receives signed grants (manager or policy)
    → apps query DIG at login
Revoke / expire
    → next query denies access
```

## App permission schema (example)

```json
{
  "app_id": "github.com",
  "app_name": "GitHub",
  "permission_groups": [
    {
      "name": "read",
      "description": "Read access to repositories",
      "scope": ["repos:read", "issues:read"]
    },
    {
      "name": "push",
      "description": "Push to protected branches",
      "scope": ["repos:write", "push:main"],
      "requires_approval": true,
      "max_duration_hours": 168
    }
  ]
}
```

## User grant record (on DIG)

```json
{
  "user_did": "did:chia:alice123",
  "permissions": [
    {
      "app_id": "github.com",
      "permission_name": "push",
      "granted_by": "did:chia:sarah_manager456",
      "granted_at": "2026-05-16T10:23:45Z",
      "expires_at": "2026-05-23T10:23:45Z",
      "revoked": false,
      "signature": "0x..."
    }
  ]
}
```

## App check (pseudocode)

```
GET /permissions/{user_did}?app_id=github.com
→ { "permissions": ["read", "push"], "valid": true, "expires_at": "..." }
```

Latency and availability targets depend on DIG deployment — **set in production SLOs, not assumed here**.

## Audit and on-chain anchoring

| Layer | Contents |
|-------|----------|
| **DIG grant store** | Current grants per `user_did` (JSON above) |
| **DIG audit store** | Append-only events: `grant.request`, `grant.approved`, `grant.revoked`, `app.check`, etc. |
| **Chia** | DID and protocol coins; **store-update commitments** (e.g. Merkle root / hash of audit store head) when the DIG store changes |

**Rules:**

- **Never** write heavy payloads to Chia — no full audit bodies, query text, or bulk permission JSON on chain.
- **Auditability** = signed append on DIG + verifiable anchor on each store update.
- Apps and auditors **read events from DIG**; integrity = compare store history to anchored commitments.

PePP product narrative: [permission-platform.md](../../02-product/permission-platform.md). Architecture: [fully-decentralized.md §2–4](../../01-vision/fully-decentralized.md).