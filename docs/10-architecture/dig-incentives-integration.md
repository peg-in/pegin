# 🐧 DIG Network Incentives + PEGIN DSSO Integration


## How DIG Network may reward participants

> **Direction:** DIG stores PEGIN identity data; peers may earn network incentives (e.g. XCH) for storage and proofs. Customer and operator outcomes must be modeled per deployment.

---

## Part 1: DIG Network Incentive Mechanism

### The Problem DIG Solves

**Traditional decentralized storage (IPFS):**
- Files get stored
- Peers serve files
- Nobody pays for it
- Peers stop running (no revenue)
- Files disappear

**DIG Network solution:**
- Files stored with **guaranteed incentives**
- Peers earn **XCH (Chia)** for serving files
- Incentives paid on-chain (trustless, automatic)
- Peers keep running (profitable)
- Files persist indefinitely

### How DIG Network Incentives Work

#### 1. The Core Mechanism: "Pay to Propagate"

```
Data Publisher (e.g., PEGIN customer)
 ↓
Sets incentive: "1 XCH per week for storing my data"
 ↓
DIG Network broadcasts incentive (on-chain)
 ↓
DIG Peers see incentive
├─ Peer 1: "1 XCH per week? Yes, I'll store that"
├─ Peer 2: "Already have enough storage? No, I'll take it"
├─ Peer 3: "Can't afford server space? I'll take it"
 ↓
Multiple peers offer to store (competition)
 ↓
Incentive distributed equally among active peers
├─ If 1 peer: Gets entire 1 XCH/week
├─ If 2 peers: Each gets 0.5 XCH/week
├─ If 5 peers: Each gets 0.2 XCH/week
 ↓
Market equilibrium: Peers join/leave until ROI equals Chia farming
```

**Example (real economics):**
- Chia farming with 10TB: ~0.5 XCH/week
- DIG peer with 10TB: ~0.5 XCH/week
- Result: DIG peers become as profitable as farming

#### 2. Epoch-Based Payments

```
Weekly Epoch (7 days Tuesday-Monday UTC)
├─ Week 1
│ ├─ Incentive: 1 XCH for storing "PEGIN" data
│ ├─ Active peers: 3 (Peer A, Peer B, Peer C)
│ ├─ Payout per peer: 0.33 XCH
│ ├─ Each peer "spends" Server Coin (proof they're online)
│ └─ Smart contract pays out: 0.33 XCH to each
│
├─ Week 2
│ ├─ Incentive: 1 XCH (renewed)
│ ├─ Active peers: 4 (A, B, C + new peer D)
│ ├─ Payout per peer: 0.25 XCH (diluted by more peers)
│ ├─ Peer D "spends" Server Coin (registers as active)
│ └─ Smart contract pays out: 0.25 XCH to each
│
└─ Equilibrium
 └─ Peers join until profit = farming
 → Maybe 8-10 peers for the incentive
 → Each gets 0.1 XCH/week (~farming ROI)
```

#### 3. On-Chain Trustless Payments

**Traditional centralized storage (AWS):**
```
Publisher: "Pay me for storing your data"
Customer: "OK, transfer money"
[Customer sends money]
[Provider might not actually store]
[Customer has no way to verify]
[Trust required]
```

**DIG Network (blockchain-based):**
```
Publisher commits 1 XCH to escrow (on-chain)
 ↓
Smart contract locks funds
 ↓
Peers provide Proof of Living Storage (cryptographic)
 ↓
Smart contract verifies proof (automatic)
 ↓
Smart contract pays peers from escrow (automatic)
 ↓
Peers reliably get paid (no trust needed)
```

**Key innovations:**
- Proof of Living Storage: Cryptographic proof peer is actually storing data
- Reward Distributor: Smart contract pays out automatically per epoch
- Server Coin: On-chain registration (peers announce availability)
- Validator: Independent node validates peer proofs + distributes payments

---

## Part 2: PEGIN + DIG Network Integration

### How PEGIN Uses DIG Network

```
PEGIN Identity Data Flow
├─ PEGIN stores:
│ ├─ User identities (encrypted)
│ ├─ Permission rules (encrypted)
│ ├─ Audit logs (encrypted)
│ └─ Session state (encrypted)
│
├─ Stored on DIG Network (not on PEGIN servers)
│ └─ Data encoded in Merkle tree
│ └─ Merkle root stored on Chia blockchain
│
├─ Multiple DIG peers replicate data
│ ├─ Peer 1: Tokyo
│ ├─ Peer 2: Berlin
│ ├─ Peer 3: Sydney
│ └─ Ensures data always available
│
└─ Customers participate in incentive pool
 ├─ Pay for DIG storage (pass-through)
 ├─ Earn from running own DIG peer
 └─ Get paid for bandwidth usage
```

### Why PEGIN Chose DIG Network

| Requirement | Traditional Approach | DIG Network | PEGIN Choice |
|---|---|---|---|
| **Data storage** | Centralized servers | Decentralized peers | DIG (no servers) |
| **Redundancy** | Multiple datacenters (expensive) | Peer consensus (cheap) | DIG (cheaper) |
| **Uptime** | Company promises uptime | Peers incentivized (guaranteed) | DIG (guaranteed) |
| **Vendor lock-in** | Company controls data | Users control data | DIG (users control) |
| **Geographic distribution** | Company's datacenters | Global peer network | DIG (global) |
| **Cost model** | Per-gigabyte per month | Market-based (decreases) | DIG (decreases with peers) |

---

## Part 3: Customer economics (model — measure in pilots)

### What to model

| Input | Notes |
|-------|--------|
| Identity data per user | Encrypted profile, audit, permissions — size TBD |
| Replication factor | How many DIG peers hold copies |
| Operator run cost | Hardware, bandwidth, staff |
| Network incentives | XCH or other rewards per protocol rules — not USD until converted |

### Comparison worksheet (no default numbers)

Build two columns for each design partner:

1. **Status quo** — IdP + permission + storage line items from their actual invoices 
2. **PEGIN + DIG** — operator SLA, chain fees, peer capex/opex, internal labor 

Do not use fictional “10K users = $X/year” examples in external decks.

### Peer operator flywheel (hypothesis)

- More enterprises need storage → more peer demand 
- More peers → potentially better availability and competition 
- Incentive design must be validated on network testnet/mainnet with real publishers 

---

## Part 4: Customer as DIG participant (hypothesis)

Some enterprises may run their own peer on existing datacenter capacity. **Whether that is net-positive** depends on utilization, incentive parameters, and XCH market — model per customer.

**Removed:** Fictional enterprise and “Goldman Sachs” scenarios with dollar totals. Replace with named pilot case studies when available.

---

## Part 5: Revenue mechanisms (qualitative)

See [sustainable-funding.md](../05-business/sustainable-funding.md) — no dollar pass-through, margin, or token valuation examples here until pilots exist.

| Mechanism | Idea |
|-----------|------|
| Storage / peer operation | Operators may earn network incentives; customer may run own peer |
| Pass-through billing | Optional: bill storage at cost — terms TBD per contract |
| Token / ecosystem | Only if legal and adopted — no price assumptions |

---

## Part 6: Complete PEGIN-DIG Ecosystem

### Data Flow: End to End

```
Enterprise User (Alice)
├─ Logs in with PEGIN (passkey)
├─ Enters permission request: "I need GitHub access for 1 week"
│
PEGIN Processing:
├─ Generates identity proof (Chia DID)
├─ Creates permission token (capability)
├─ Stores both on DIG Network (encrypted)
├─ Records on Chia blockchain (immutable hash)
│
DIG Network:
├─ Merkle tree contains:
│ ├─ Alice's DID
│ ├─ Alice's permissions
│ ├─ Audit trail
│ └─ Encrypted with Alice's key
├─ Multiple peers replicate:
│ ├─ Peer in US (Tokyo datacenter peer runs in US)
│ ├─ Peer in EU (Berlin peer)
│ ├─ Peer in Asia (Sydney peer)
│ └─ All earning XCH from incentive pool
│
Chia Blockchain:
├─ Records:
│ ├─ Merkle root of Alice's identity data
│ ├─ Permission grant (immutable)
│ ├─ Audit trail hash
│ └─ Timestamp
├─ Cannot be modified (blockchain property)
└─ Proof for auditors (immutable audit log)

GitHub Integration:
├─ Checks: "Is there valid permission token for Alice?"
├─ Verifies token (signed with PEGIN key)
├─ Checks expiration: "Expires 2026-05-23"
├─ Checks device health: "Passed security scan"
├─ Result: Grant access
│
Alice's GitHub Access Granted (1 week)

Week 1 Passes:
├─ Automatic revocation triggered
├─ Permission token expires: "2026-05-23 10:23:45"
├─ GitHub checks token: "Expired, revoke access"
├─ Alice's access denied (automatic)
├─ No manual cleanup needed

DIG Peer Economics During This Week:
├─ PEGIN customer data (1TB) stored on DIG peers
├─ DIG peers earning XCH continuously
├─ Peer operator in Tokyo: Earns ~0.05 XCH/day
├─ Peer operator in Berlin: Earns ~0.05 XCH/day
├─ Peer operator in Sydney: Earns ~0.05 XCH/day
├─ PEGIN customer (if running peer): Also earning
└─ Everyone gets paid, data always available
```

### The Ecosystem Flywheel

```
PEGIN Adoption Increases
 ↓
More Identity Data on DIG Network
 ↓
More DIG Peer Incentive Opportunities
 ↓
More Peers Join (profitability increases)
 ↓
Better Geographic Distribution
 ↓
Better Redundancy + Lower Latency
 ↓
Better PEGIN Service
 ↓
More Enterprise Adoption
 ↓
[REPEAT → Exponential Growth]

Parallel Effects:
├─ DIG Network Success → XCH appreciates
├─ XCH appreciation → PEGIN customers' tokens worth more
├─ PEGIN customers run DIG peers → earn more XCH
├─ More XCH earned → reinvest in bigger peers
├─ Bigger peers → more capacity → more customers
└─ [VIRTUOUS CYCLE]
```

---

## Part 7: Incentive Alignment

### Everyone Wins

| Participant | Incentive | Benefit | Win |
|---|---|---|---|
| **DIG Peer Operator** | Store identity data | Earn profitable XCH | ✅ Yes |
| **Chia Network** | DIG adoption | XCH ecosystem grows | ✅ Yes |
| **End User (Alice)** | Free SSO + instant permissions | Use PEGIN without cost | ✅ Yes |

### Why This Model Is Unstoppable

```
Traditional SaaS (Okta):
├─ Customer pays for access
├─ Okta keeps all profit
├─ Customer feels extracted from
└─ Adversarial relationship

PEGIN-DIG Model:
├─ PEGIN makes money: Token appreciation + peer operations
├─ Customer doesn't pay (uses free software)
├─ Customer can earn (run DIG peer)
├─ Customer owns PEGIN tokens (share upside)
└─ Aligned relationship

Result:
├─ Customers stay (earning money vs. paying)
├─ Network effects compound
├─ DIG peers increase (better service)
├─ PEGIN founder + customers get wealthy together
└─ System becomes inevitably dominant
```

---

## Part 8: Economic Model with DIG Participation

### Year 5 Full Ecosystem

```
PEGIN Adoption: 1,000 enterprises

Customer Data on DIG:
├─ [1,000 enterprises total]
└─ Total DIG data: 5,000 TB

DIG Incentive Pool (Annual):
├─ DIG peers competing: ~50 peer operators

Peer Operator Economics (Year 5):
├─ Peer with 500TB capacity:
│ 
│ Solution: Infrastructure costs decrease
│ └─ Equilibrium: Peers join until profit ≈ expected risk

XCH Price Appreciation Path:
├─ Profitable + building wealth in XCH
```

### PEGIN Customer Becoming DIG Peer (Goldman Sachs Example)

```
Goldman Sachs Decision: Run DIG peer for PEGIN data

Investment:
└─ Real cost: Just hardware refresh

Year 1 Earnings:
├─ Capacity: 500 TB
├─ PEGIN data stored: 50 TB (10% of capacity)
├─ Other enterprise data: 100 TB (via incentives)
├─ Not great, but covers hardware refresh
└─ Plus: PEGIN identity storage works perfectly

Year 5 Earnings:
├─ Same hardware, now handling:
│ ├─ PEGIN data: 100 TB (more enterprises)
│ ├─ Other dApp data: 300 TB
│ └─ Total capacity utilized: 400 TB
└─ Passive income created

Goldman's PEGIN Costs:

Cumulative 5-year value:
```

---

## Summary: DIG + PEGIN = Inevitable

### The Loop

```
PEGIN needs storage
 ↓
Uses DIG Network
 ↓
Pays incentives to peers
 ↓
Peers become profitable
 ↓
Peers expand capacity
 ↓
Better service for PEGIN
 ↓
More customers adopt PEGIN
 ↓
More data on DIG
 ↓
[LOOP REPEATS, EXPONENTIALLY]

Meanwhile:
├─ Customers see: Free SSO + free storage + earn money
├─ XCH appreciates (DIG adoption matters)
├─ PEGIN founder's tokens worth more
├─ PEGIN customers' tokens worth more
├─ Peer operators getting wealthier
└─ Everyone wins = inevitable dominance
```

### Why This Works

1. **Free software** (open source, no license fees)
2. **Decentralized storage** (DIG Network, not PEGIN servers)
3. **Customer participation** (can run peers, earn money)
4. **Token upside** (XCH appreciation benefits everyone)
5. **Aligned incentives** (PEGIN succeeds when customers succeed)
6. **Network effects** (more peers = better service = more adoption)
7. **Sustainable economics** (DIG storage is profitable at market rate)
8. **Self-reinforcing** (loop compounds exponentially)

**This is not a SaaS company. This is a self-reinforcing ecosystem.**

And ecosystems always beat companies.

---

*Built with 🐧 by the PEGIN team. DIG powers PEGIN. XCH rewards peers. Everybody wins.*