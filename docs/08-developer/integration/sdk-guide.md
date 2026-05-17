# PEGIN SDK integration guide

> **Developer hub:** [../README.md](../README.md) · **Spec 1:** [../04-technical/specs/tech-stack.md](../../04-technical/specs/tech-stack.md) · **POC scope:** [../../03-use-cases/mvp-strategy.md](../../03-use-cases/mvp-strategy.md)

## The truth: login must be dead simple

> **The Core Principle:** PEGIN login should be the easiest, fastest, most frictionless login system anywhere. One line of code to add it. One tap to log in.

---

## Part 1: The Vision

### Why SDK is #1 Priority

```
Traditional SSO:
├─ Okta SDK: Complex integration (weeks)
├─ Auth0: Documentation nightmare (days)
├─ Azure AD: Enterprise bloat (days)
└─ Result: Developers avoid it

PEGIN SDK:
├─ One line of code: npm install @pegin/sdk
├─ One button: <PeginLogin />
├─ One tap: Face ID → logged in
└─ Result: Every developer wants it

The SDK makes PEGIN viral.
Without it, PEGIN is just another identity system.
With it, PEGIN becomes the obvious default.
```

### Success Criteria for SDK

```
✅ Add PEGIN login in < 5 minutes (no reading docs)
✅ One line of code to add button
✅ One line of code to check authentication
✅ One line of code to get user info
✅ Works with existing user databases (no migration)
✅ Faster than any competitor (< 2 seconds login)
✅ Works everywhere (web, mobile, desktop, CLI)
✅ Zero configuration (sensible defaults)
✅ Works with existing identity (existing user, just add PEGIN)
```

---

## Part 2: SDK Architecture

### The Three-Layer Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                        │
│  (Your website, app, service)                               │
├─────────────────────────────────────────────────────────────┤
│                      PEGIN SDK                              │
│  (Client library + server middleware)                       │
├─────────────────────────────────────────────────────────────┤
│                    PEGIN Backend                            │
│  ├─ DID authentication (Chia)                               │
│  ├─ DIG Network queries (permissions)                       │
│  └─ Session management                                      │
├─────────────────────────────────────────────────────────────┤
│                   External Services                         │
│  ├─ Chia Blockchain (DID anchor)                            │
│  ├─ DIG Network (permissions storage)                       │
│  ├─ Your Existing Database (user data)                      │
│  └─ Optional: Notbot (humanity verification)                │
└─────────────────────────────────────────────────────────────┘
```

---

## Part 3: SDK Components

### Component 1: Frontend (React/Vue/Vanilla)

```typescript
// React component (simplest integration)
import { PeginLogin } from '@pegin/sdk'

function MyApp() {
  return (
    <div>
      <h1>Welcome</h1>
      {/* That's it. One line. */}
      <PeginLogin 
        onSuccess={(user) => console.log(user)}
      />
    </div>
  )
}

// What <PeginLogin /> does:
// 1. Shows "Login with PEGIN" button
// 2. On click: Opens PEGIN (browser or app)
// 3. User: Face ID scan
// 4. Returns: JWT token + user DID
// 5. SDK: Stores token (automatic)
// 6. App: Receives user info (automatic)
// 7. Total time: < 2 seconds
```

### Component 2: Backend (Node.js example)

```typescript
// Backend middleware (express)
import { peginAuth } from '@pegin/sdk'

const app = require('express')()

// Protect any route with one line
app.get('/api/me', peginAuth(), async (req, res) => {
  // User is authenticated
  const user = req.pegin.user // { did, name, email, etc }
  const permissions = req.pegin.permissions // from DIG Network
  
  // Check permissions
  if (permissions.includes('admin')) {
    res.json({ admin: true })
  }
})

// How it works:
// 1. Client sends JWT in Authorization header
// 2. Middleware: Validates token (signature check)
// 3. Middleware: Queries DIG for permissions (cached, < 50ms)
// 4. Middleware: Checks if permission revoked (instant)
// 5. Request handler: User + permissions available
```

### Component 3: User Data Bridge

```typescript
// SDK connects PEGIN to your existing database
import { PeginSDK } from '@pegin/sdk'

const pegin = new PeginSDK({
  // PEGIN backend configuration
  peginBackend: 'https://api.pegin.app',
  
  // Your database configuration
  userDatabase: {
    provider: 'postgresql', // or mysql, mongodb, etc
    connection: process.env.DATABASE_URL,
    // OR custom handler
    handler: async (did) => {
      // Custom logic to fetch user by DID
      return db.users.findByDid(did)
    }
  },
  
  // DIG Network for permissions
  digNetwork: {
    endpoint: 'https://dig.chia.net', // Public DIG endpoint
    // OR run your own peer for performance
  }
})

// When user logs in:
pegin.onLogin(async (did) => {
  // 1. Check: Does user exist in your DB?
  let user = await db.users.findByDid(did)
  
  if (!user) {
    // 2. If not: Create user (linked to DID)
    user = await db.users.create({
      did: did,
      name: did.substring(0, 10), // default name
      email: null, // can be filled later
      created_at: new Date()
    })
  }
  
  // 3. Load permissions from DIG Network
  const permissions = await pegin.getPermissions(did, 'your-app-id')
  
  // 4. Store in session/JWT
  return {
    user: user,
    permissions: permissions
  }
})
```

---

## Part 4: How Existing User Data Works

### The Bridge Between PEGIN and Your DB

```
Scenario: You have existing users in your database

Option A: New User (First Time PEGIN Login)
┌──────────────┐
│ New User     │
│ (no account) │
└──────┬───────┘
       │
       ├─ PEGIN: Creates DID (Chia blockchain)
       │
       ├─ Your DB: Creates new user record
       │  ├─ did: "did:chia:alice123"
       │  ├─ name: "Alice"
       │  ├─ email: null (can add later)
       │  └─ pegin_created: true
       │
       └─ Result: User created with DID link

Option B: Existing User (Migrating to PEGIN)
┌──────────────┐
│ Existing User│
│ (email login)│
└──────┬───────┘
       │
       ├─ User: Creates PEGIN account (separate)
       │  └─ Gets DID
       │
       ├─ Your DB: Link existing user to DID
       │  ├─ UPDATE users SET did = "did:chia:alice123" WHERE email = "alice@example.com"
       │  └─ Now: User can login with PEGIN OR email
       │
       └─ Result: Dual login (transition period)

Option C: Batch Migration
┌──────────────────┐
│ Existing Users   │
│ (batch import)   │
└──────┬───────────┘
       │
       ├─ Your DB: Create DID for each user
       │  └─ CREATE FUNCTION generate_did_for_users()
       │
       ├─ PEGIN: Issue DIDs for batch
       │  └─ POST /batch-import { emails: [list] }
       │
       ├─ Email: "Your PEGIN is ready"
       │  └─ User clicks link, registers passkey
       │
       └─ Result: All users migrated at once
```

### Example: SQL Schema Updates

```sql
-- Original users table
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email VARCHAR(255) UNIQUE,
  name VARCHAR(255),
  password_hash VARCHAR(255), -- bcrypt hash
  created_at TIMESTAMP
)

-- Updated: Add DID column (no migration needed)
ALTER TABLE users ADD COLUMN did VARCHAR(255) UNIQUE;
ALTER TABLE users ADD COLUMN pegin_created BOOLEAN DEFAULT false;

-- Now user can have:
-- ├─ email + password (legacy login)
-- ├─ did (PEGIN login)
-- └─ Both (gradual migration)

-- Query examples:
SELECT * FROM users WHERE did = 'did:chia:alice123'; -- PEGIN lookup
SELECT * FROM users WHERE email = 'alice@example.com'; -- Email lookup
SELECT * FROM users WHERE did IS NOT NULL; -- PEGIN-enabled users
```

### Example: JavaScript Integration

```typescript
// Your backend (Node.js)
import { PeginSDK } from '@pegin/sdk'
import db from './database'

const pegin = new PeginSDK()

// Scenario 1: Existing email user migrating to PEGIN
app.post('/api/link-pegin', async (req, res) => {
  const { email, did } = req.body
  
  // Find existing user by email
  const user = await db.query('SELECT * FROM users WHERE email = ?', [email])
  
  if (!user) {
    return res.status(404).json({ error: 'User not found' })
  }
  
  // Link DID to existing user
  await db.query('UPDATE users SET did = ? WHERE email = ?', [did, email])
  
  // Now user can login with PEGIN
  res.json({ success: true, user_id: user.id })
})

// Scenario 2: Existing user logs in with PEGIN
app.post('/api/login', peginAuth(), async (req, res) => {
  const did = req.pegin.did
  
  // Look up user by DID
  let user = await db.query('SELECT * FROM users WHERE did = ?', [did])
  
  if (!user) {
    // User doesn't exist: Create new record
    user = await db.query(
      'INSERT INTO users (did, name, pegin_created) VALUES (?, ?, true) RETURNING *',
      [did, `User_${did.substring(0, 10)}`]
    )
  }
  
  // Create session/JWT
  const token = jwt.sign({ user_id: user.id, did }, SECRET)
  
  res.json({ 
    success: true,
    token,
    user: {
      id: user.id,
      did: user.did,
      name: user.name,
      email: user.email
    }
  })
})
```

---

## Part 5: DIG Network Integration for Identity Storage

### How Identity is Stored on DIG

```
User Data on DIG Network (v2):

┌─────────────────────────────────────────────────────────────┐
│                   DIG Network Storage                       │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│ Key: did:chia:alice123                                      │
│                                                             │
│ Value (encrypted):                                          │
│ {                                                           │
│   "identity": {                                             │
│     "did": "did:chia:alice123",                             │
│     "name": "Alice",                                        │
│     "email": "alice@example.com",                           │
│     "created_at": "2026-05-16T10:23:45Z",                   │
│     "updated_at": "2026-05-16T15:30:12Z"                    │
│   },                                                        │
│   "permissions": {                                          │
│     "your-app-id": [                                        │
│       {                                                     │
│         "name": "read",                                     │
│         "expires_at": "2026-06-16T10:23:45Z"                │
│       },                                                    │
│       {                                                     │
│         "name": "write",                                    │
│         "expires_at": null                                  │
│       }                                                     │
│     ]                                                       │
│   },                                                        │
│   "plugins": {                                              │
│     "notbot": {                                             │
│       "verified_human": true,                               │
│       "verified_at": "2026-05-16T10:23:45Z"                 │
│     },                                                      │
│     "custom_app_data": {                                    │
│       "user_id": 12345,  // Link to your DB                │
│       "custom_field": "value"                               │
│     }                                                       │
│   }                                                         │
│ }                                                           │
│                                                             │
└─────────────────────────────────────────────────────────────┘

Who owns this data?
├─ User owns the DID (private key in their passkey)
├─ User owns the data (encrypted, only they can decrypt fully)
├─ Apps can read permissions (public part)
├─ DIG Network peers store it (replicated, instant access)
└─ Nobody (not PEGIN, not your company) can delete it

How to query it:
├─ App: GET /did/did:chia:alice123/permissions
├─ DIG: Returns permissions (< 50ms)
├─ App: Checks expiration, grants access
└─ Instant decision (no server roundtrip needed)
```

### DIG Network Storage SDK

```typescript
import { DIG } from '@pegin/sdk'

const dig = new DIG({
  endpoint: 'https://dig.chia.net', // or your own peer
})

// Store identity on DIG (encrypted with user's key)
await dig.store(
  did, // User's DID (key)
  {
    identity: {
      name: 'Alice',
      email: 'alice@example.com'
    },
    permissions: {
      'your-app-id': ['read', 'write']
    },
    plugins: {
      custom_app_data: {
        user_id: 12345, // Link to your DB
        subscription_tier: 'pro'
      }
    }
  },
  userPrivateKey // User signs (only they can store)
)

// Retrieve identity from DIG (public part, no decryption needed)
const identity = await dig.get(did, 'your-app-id')
// Returns: { name, permissions, etc }

// Query all users in a permission group (for admin)
const usersWithAdmin = await dig.query({
  app_id: 'your-app-id',
  permission: 'admin'
})
// Returns: [did1, did2, did3, ...]
// Useful for: Listing admins, finding users with specific permission

// Update permissions (admin action)
await dig.updatePermissions(did, 'your-app-id', {
  add: ['admin'],
  remove: ['write'],
  expires_at: '2026-06-16T10:23:45Z'
})
// Updates stored on DIG instantly
// Apps checking permissions see new values immediately
```

---

## Part 6: Login Flow (Step by Step)

### The Complete User Journey

```
1. USER OPENS YOUR WEBSITE
┌──────────────────────────────┐
│ www.myapp.com                │
│ ┌──────────────────────────┐ │
│ │ [Login with PEGIN] ◄─────┼─ SDK button
│ └──────────────────────────┘ │
└──────────────────────────────┘

2. USER CLICKS "LOGIN WITH PEGIN"
┌──────────────────────────────┐
│ Browser redirects to PEGIN    │
│ https://pegin.app/authorize  │
│ ?client_id=myapp             │
│ &redirect_uri=myapp.com/cb   │
└──────────────────────────────┘

3. USER SCANS FACE ID (In PEGIN app)
┌──────────────────────────────┐
│ PEGIN App                     │
│ [📱 Scanning Face ID...]     │
│                              │
│ ✅ Face match!              │
│ ✅ Private key unlocked      │
│ ✅ Creating DID proof        │
└──────────────────────────────┘

4. PEGIN BACKEND VERIFIES
┌──────────────────────────────┐
│ PEGIN Backend                │
│ ├─ Validate Face ID proof    │
│ ├─ Check DID on blockchain   │
│ ├─ Query DIG for permissions │
│ └─ Generate JWT token        │
└──────────────────────────────┘

5. REDIRECT BACK TO YOUR APP
┌──────────────────────────────┐
│ Browser redirects back        │
│ myapp.com/callback           │
│ ?code=auth_code              │
│ &state=nonce                 │
└──────────────────────────────┘

6. YOUR BACKEND EXCHANGES CODE
┌──────────────────────────────┐
│ Your Backend                 │
│ POST /api/auth/callback      │
│ ├─ Exchange code for JWT     │
│ ├─ Verify JWT signature      │
│ ├─ Extract user DID          │
│ ├─ Look up user in DB        │
│ ├─ Query DIG for permissions │
│ └─ Create session token      │
└──────────────────────────────┘

7. USER IS LOGGED IN
┌──────────────────────────────┐
│ Your Website                 │
│ ✅ Welcome, Alice!           │
│ ├─ Session created           │
│ ├─ Permissions loaded        │
│ └─ User data ready           │
└──────────────────────────────┘

TOTAL TIME: < 2 seconds (faster than any competitor)
```

---

## Part 7: SDK Implementation Checklist

### Phase 1: MVP SDK (Weeks 1-4)

```typescript
// What MVP SDK includes:

// 1. Frontend Component
export function PeginLogin(props) {
  // - Shows button
  // - Handles redirect
  // - Stores token
  // - Calls onSuccess
}

// 2. Backend Middleware
export function peginAuth() {
  // - Validates JWT
  // - Extracts user info
  // - Populates req.pegin
}

// 3. User Lookup
export async function getUser(did) {
  // - Queries your DB by DID
  // - Creates if not exists
  // - Returns user object
}

// 4. Permission Check
export async function hasPermission(did, app_id, permission) {
  // - Queries DIG Network
  // - Checks expiration
  // - Returns boolean
}

// 5. Session Management
export function createSession(did, user_id) {
  // - Creates JWT
  // - Signs with PEGIN secret
  // - Returns token
}

API:
├─ POST /auth/login (initiate login)
├─ POST /auth/callback (exchange code)
├─ GET /auth/user (get current user)
├─ POST /auth/logout (clear session)
└─ GET /auth/permissions (get user permissions)
```

### Phase 2: Production SDK (Weeks 5-8)

```typescript
// What production SDK adds:

// 1. Multiple Frameworks
export { PeginLogin as React }
export { PeginLogin as Vue }
export { PeginLogin as Angular }
export { PeginLogin as Svelte }
export const VanillaJS = { /* */ }

// 2. Multiple Backends
export { createPeginMiddleware as Express }
export { createPeginMiddleware as FastAPI }
export { createPeginMiddleware as Django }
export { createPeginMiddleware as Rails }

// 3. Multiple Databases
export { DatabaseAdapter as PostgreSQL }
export { DatabaseAdapter as MySQL }
export { DatabaseAdapter as MongoDB }
export { DatabaseAdapter as Firebase }

// 4. Better Error Handling
export class PeginError extends Error { /* */ }
export class PermissionDenied extends PeginError { /* */ }
export class UserNotFound extends PeginError { /* */ }

// 5. Caching & Performance
export class PermissionCache {
  // Cache permissions (expires in 5 min)
  // Cache user data (expires in 1 hour)
  // Invalidate on revocation
}

// 6. Hooks & Events
export const useAuth = () => { /* */ }
export const usePeginLogin = () => { /* */ }
export const usePermissions = () => { /* */ }
```

### Phase 3: Enterprise SDK (Weeks 9-12)

```typescript
// What enterprise SDK adds:

// 1. SSO Integration
export { integrateWithOkta }
export { integrateWithAzureAD }
export { integrateWithAuth0 }
// Allow existing SSO customers to add PEGIN

// 2. Migration Tools
export { migrateFromOkta }
export { migrateFromAuth0 }
// Batch import users, handle dual login period

// 3. Analytics
export { trackLogin }
export { trackPermissionCheck }
export { trackLogout }

// 4. Compliance
export { exportAuditLog } // For auditors
export { exportUserData } // For GDPR
export { generateComplianceReport } // For compliance teams

// 5. Notbot Integration
export { requireHumanVerification }
// Add Notbot layer on top of PEGIN
```

---

## Part 8: Your Website Integration (Practical Example)

### Step 1: Install SDK

```bash
npm install @pegin/sdk
# or
yarn add @pegin/sdk
```

### Step 2: Add Login Button (Frontend)

```tsx
// pages/index.tsx
import { PeginLogin } from '@pegin/sdk'

export default function Home() {
  return (
    <div>
      <h1>Welcome to My App</h1>
      
      {/* That's it. One line. */}
      <PeginLogin 
        onSuccess={(user) => {
          // Redirect or update state
          window.location.href = '/dashboard'
        }}
        onError={(error) => {
          console.error('Login failed:', error)
        }}
      />
    </div>
  )
}
```

### Step 3: Add Backend Middleware

```typescript
// api/middleware.ts
import { peginAuth } from '@pegin/sdk'

// Protect all /api/protected/* routes
export const config = {
  api: {
    routes: ['/api/protected/*']
  }
}

export const middleware = peginAuth()
```

### Step 4: Create Protected Endpoint

```typescript
// api/protected/me.ts
import { peginAuth } from '@pegin/sdk'
import db from '@/lib/database'

export default peginAuth()(async (req, res) => {
  // User is authenticated
  const { did } = req.pegin
  
  // Get user from database
  const user = await db.users.findByDid(did)
  
  // Get permissions from DIG Network
  const permissions = await req.pegin.getPermissions('my-app-id')
  
  res.json({
    user,
    permissions
  })
})
```

### Step 5: Check Permissions

```typescript
// api/protected/admin.ts
import { peginAuth, requirePermission } from '@pegin/sdk'

export default peginAuth()(
  requirePermission('admin')
)(async (req, res) => {
  // Only users with 'admin' permission reach here
  res.json({ message: 'Admin endpoint' })
})
```

### Step 6: Create Admin Interface (Grant Permissions)

```tsx
// pages/admin/users.tsx
import { PeginAdmin } from '@pegin/sdk'

export default function AdminUsers() {
  return (
    <div>
      <h1>User Management</h1>
      
      {/* Admin component that shows all users */}
      <PeginAdmin 
        app_id="my-app-id"
        features={['create-user', 'grant-permission', 'revoke-permission']}
      />
      
      {/* Manual API example */}
      <button onClick={async () => {
        // Grant 'write' permission for 1 week
        await fetch('/api/admin/grant-permission', {
          method: 'POST',
          body: JSON.stringify({
            did: 'did:chia:alice123',
            permission: 'write',
            expires_in: '7d'
          })
        })
      }}>
        Grant Permission
      </button>
    </div>
  )
}
```

---

## Part 9: Database Schema

### Your Database Structure (With PEGIN)

```sql
-- Users table (minimal, most data on DIG)
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  did VARCHAR(255) UNIQUE NOT NULL, -- PEGIN DID
  email VARCHAR(255),                 -- Optional: email
  name VARCHAR(255),                  -- Optional: name
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW()
);

-- Sessions (optional: for stateful sessions)
CREATE TABLE sessions (
  id VARCHAR(255) PRIMARY KEY,
  user_id INT NOT NULL REFERENCES users(id),
  did VARCHAR(255) NOT NULL,
  expires_at TIMESTAMP NOT NULL,
  created_at TIMESTAMP DEFAULT NOW()
);

-- Audit log (who did what)
CREATE TABLE audit_log (
  id SERIAL PRIMARY KEY,
  did VARCHAR(255) NOT NULL,
  action VARCHAR(255) NOT NULL, -- 'login', 'permission_granted', etc
  details JSONB,
  created_at TIMESTAMP DEFAULT NOW()
);

-- Optional: Local permissions cache (for performance)
CREATE TABLE permission_cache (
  did VARCHAR(255) NOT NULL,
  app_id VARCHAR(255) NOT NULL,
  permissions JSONB NOT NULL,
  expires_at TIMESTAMP NOT NULL,
  PRIMARY KEY (did, app_id)
);
```

### Queries

```sql
-- Find user by DID
SELECT * FROM users WHERE did = 'did:chia:alice123';

-- Create new user
INSERT INTO users (did, name, email) 
VALUES ('did:chia:alice123', 'Alice', 'alice@example.com');

-- Get all users
SELECT * FROM users;

-- Audit log
INSERT INTO audit_log (did, action, details)
VALUES ('did:chia:alice123', 'login', '{"ip": "1.2.3.4"}');

-- Check cached permissions
SELECT * FROM permission_cache 
WHERE did = 'did:chia:alice123' 
AND app_id = 'my-app-id'
AND expires_at > NOW();
```

---

## Part 10: Why This SDK Wins

### Competitive Advantage

```
Okta:
├─ Setup time: Days
├─ Code: Hundreds of lines
├─ Complexity: High
├─ Cost: incumbent per-seat (varies by contract)
└─ Speed: 5+ seconds

Auth0:
├─ Setup time: Hours
├─ Code: 50+ lines
├─ Complexity: Medium
├─ Cost: varies by Auth0 plan
└─ Speed: 3-4 seconds

PEGIN SDK:
├─ Setup time: 5 minutes ✅
├─ Code: 3 lines ✅
├─ Complexity: Minimal ✅
├─ Cost: Free ✅
└─ Speed: < 2 seconds ✅

Result: PEGIN is so easy, every developer chooses it
```

### Why Developers Will Use It

```
"I can add PEGIN login in 5 minutes"
"I don't have to read 100-page documentation"
"It works with my existing database"
"It's faster than anything else"
"It's free"
"My users love the privacy"
"I get beautiful, automatic admin panel"
"Permissions are instant (no approval backlog)"

Result: Viral adoption (word-of-mouth developer marketing)
```

---

## Summary: SDK is the MVP

```
Priority ranking:
1. 🎯 SDK (Makes PEGIN easy to use) ← START HERE
2. ✅ Passkey login (Works great)
3. ✅ DIG integration (Permissions)
4. ✅ Admin interface (Manage users)
5. ✅ Notbot integration (Deepfake proof)

Without SDK: PEGIN is interesting but hard to use
With SDK: PEGIN is obvious choice (easiest, fastest)

Timeline:
├─ MVP (Weeks 1-4): React SDK + Express middleware
├─ Phase 2 (Weeks 5-8): Multi-framework, multi-backend
├─ Phase 3 (Weeks 9-12): Enterprise features, migrations
└─ Phase 4 (Weeks 13+): New languages, integrations
```

---

*Built with 🐧 by the PEGIN team. Easiest login system ever created.*