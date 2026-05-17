# Pengin Gateway — Technical Architecture

_Decentralized SSO on Chia + DIG_

Version 1.0 · May 2026

## Table of contents

- [System Overview](#system-overview)
- [Authentication Flow](#authentication-flow)
- [Chia Blockchain Integration](#chia-blockchain-integration)
- [DIG Network Architecture](#dig-network-architecture)
- [Credential Model](#credential-model)
- [Zero-Knowledge Proofs](#zero-knowledge-proofs)
- [dApp Integration (SDK)](#dapp-integration-sdk)
- [API Design](#api-design)
- [Security & Cryptography](#security-cryptography)
- [Performance & Scalability](#performance-scalability)
- [Deployment Architecture](#deployment-architecture)
- [Code Examples](#code-examples)

## 1. System Overview

Penguin Gateway is a decentralized SSO protocol where users control identity and dApps verify
credentials cryptographically. No central server; all verification happens on-chain or peer-to-peer.
System Components:

- User Identity Module: Manages user DID, public keys, and credential index
- Blockchain Layer: Chia smart contracts for identity anchoring and revocation
- Data Layer: DIG network stores encrypted credentials with on-chain merkle roots
- API Gateway: REST/GraphQL endpoints for dApp integration
- SDK Libraries: Easy integration for dApps (3-line setup)
- dApp Portal: Marketplace for discovering apps using Penguin Gateway
- Credential Issuer Interface: Banks, universities, employers issue credentials

## 2. Authentication Flow

Penguin Gateway provides a streamlined authentication flow similar to OAuth2, but completely
decentralized.

### 2.1 User Registration (One-Time)

1. Create Passkey: User creates WebAuthn passkey (Face ID, fingerprint, PIN)
2. Generate DID: Penguin Gateway generates Decentralized Identifier (W3C DID:chia)
3. Register on Blockchain: DID and public key anchored on Chia blockchain
4. DIG Store Created: User gets DIG store for credential storage and management
5. Account Ready: User can now authenticate to any Penguin Gateway-integrated dApp

### 2.2 Login Flow (Standard OAuth2-Like)

1. User clicks 'Login with Penguin Gateway': on dApp
2. Redirected to Penguin Login UI: Secure modal or popup
3. User authenticates: Passkey (Face ID) or Chia Signer
4. Authorization Request: dApp requests specific credential (e.g., 'is user KYC verified?')
5. User approves: User reviews and approves credential sharing
6. ZK Proof Generated: Penguin Gateway generates zero-knowledge proof without exposing
 data
7. Callback to dApp: Redirect to dApp with JWT token + proof
8. Token Verified: dApp verifies token on-chain; user logged in

### 2.3 Logout Flow

- User clicks logout on dApp
- JWT token invalidated; session cleared
- Penguin Gateway session can remain active for other dApps

## 3. Chia Blockchain Integration

### 3.1 Identity Anchoring

- DID Registration: User DIDs (e.g., did:chia:xyz123) registered on blockchain
- Public Keys: User's public key(s) stored in blockchain for signature verification
- Credential Index Root: Merkle root of user's DIG credential store on-chain
- Revocation Registry: Revoked credentials tracked on-chain with timestamps
- Issuer Registry: Authorized credential issuers registered and verified on-chain

### 3.2 Smart Contract Design (Chialisp)

- DID Creation Contract: Issues new DID coin on blockchain
- Credential Anchor Contract: Records credential issuance and merkle root updates
- Revocation Contract: Revokes or suspends credentials (issuer + owner signature required)
- Issuer Authorization: Whitelist of authorized credential issuers with revocation capability
- Access Control Contract: Enforces granular permissions on credential access

### 3.3 On-Chain Data Structures

- DID Coin: Represents user's identity on blockchain
- DID (format: did:chia:pubkeyhash)
- Owner pubkey
- DIG store reference
- Credential merkle root
- Revocation bitmask
- Creation & last-update timestamp

## 4. Dig Network Architecture

DIG (Decentralized Interoperable Governance) provides off-chain credential storage with on-chain
verification via merkle proofs.

### 4.1 DIG Store Structure

- User DIG Store: One encrypted store per Penguin Gateway user
- Contents: User DIDs, credentials, metadata, revocation status
- Encryption: All data encrypted with user's master key (derived from Passkey)
- Access Control: User controls who can read (fine-grained permissions per dApp)
- Merkle Root: Root hash published on blockchain for verification
- P2P Replication: DIG nodes replicate user's store across network for redundancy

### 4.2 Credential Storage in DIG

- Credential Document: W3C Verifiable Credential format
- Issuer Signature: Digital signature from credential issuer
- Issuance & Expiry: Timestamps for credential lifecycle
- Subject Data: The actual claim (e.g., 'KYC verified', 'age >= 18')
- Metadata: Issuer info, credential type, proof requirements

### 4.3 Selective Disclosure

- User Approves: When dApp requests credential, user reviews and approves sharing
- ZK Proof Generated: Penguin Gateway creates proof of selected attributes
- No Data Exposed: dApp only learns what user explicitly approved
- Proof Verification: dApp verifies proof without seeing underlying credential
- Example: dApp gets 'user is KYC verified' but not user's real name/ID number

## 5. Credential Model

Penguin Gateway uses W3C Verifiable Credentials (VC) standard for interoperability and portability.

### 5.1 Credential Types Supported

- KYC Credentials: Identity verification from banks or KYC providers
- Age Verification: Proof of age >= 18 without revealing exact age
- Employment: Proof of employment at specific company
- Education: Diplomas and degrees from universities
- Credit Score: Encrypted credit score from financial institutions
- Account Balance: Proof of funds without revealing exact amount
- Custom Credentials: Any claim issued by authorized parties

### 5.2 Credential Lifecycle

1. Issuance: Credential issuer (bank, university) issues credential to user
2. Storage: User stores credential in DIG store (encrypted)
3. Presentation: User presents credential proof to dApp
4. Verification: dApp verifies issuer's signature and merkle proof
5. Usage: dApp grants access based on verified credential
6. Expiry: Credential expires at specified time (user can renew)
7. Revocation: Issuer can revoke credential early if needed

### 5.3 Credential JSON Example

{
"@context": "https://www.w3.org/2018/credentials/v1",
"type": ["VerifiableCredential", "KYCCredential"],
"issuer": "did:chia:bank123",
"issuanceDate": "2024-01-15T00:00:00Z",
"expirationDate": "2025-01-15T00:00:00Z",
"credentialSubject": {
"id": "did:chia:user456",
"kycVerified": true,
"verificationLevel": "high"
},
"proof": {
"type": "Ed25519Signature2020",

"verificationMethod": "did:chia:bank123#key1",
"signatureValue": "ABC123..."
}
}

## 6. Zero-Knowledge Proofs

Penguin Gateway uses zero-knowledge proofs to allow users to prove credential attributes without
revealing underlying data.

### 6.1 ZK Proof Types

- Range Proofs: Prove age >= 18 without revealing exact age
- Existence Proofs: Prove credential exists without revealing content
- Signature Proofs: Prove issuer signed credential without exposing signature
- Selective Disclosure: Prove specific attributes without revealing others
- Aggregation Proofs: Combine multiple credentials into single proof

### 6.2 ZK Implementation

- Library: circom + snarkjs for zero-knowledge circuits
- Verification: Smart contracts verify ZK proofs on-chain or server-side
- Performance: Proof generation < 1 second; verification < 100ms
- Security: Proofs are post-quantum resistant (lattice-based option available)

### 6.3 Example: Age Verification ZK Proof

User has credential: ageProof = { dob: "2000-01-15" }
dApp requests: "Prove user is >= 18"
Penguin Gateway generates ZK proof:

- Proof that SHA256(dob + salt) = commitment
- Proof that age >= 18 (using range proof circuit)
- Proof signed with user's key
 dApp verifies:
- Verify ZK proof (proof that age >= 18)
- Verify commitment on-chain
- Grant access
 Result: dApp knows user is 18+ but not exact age

## 7. Dapp Integration (Sdk)

Integrating Penguin Gateway into a dApp is simple—similar to OAuth2.

### 7.1 Three-Line Integration (TypeScript)

import { PenguinGateway } from "@penguin-gateway/sdk";
const pg = new PenguinGateway({
clientId: "your-dapp-id",
redirectUri: "https://yourapp.com/callback"
});
// In login button handler:
pg.login({ scopes: ["profile", "email", "kyc"] });
// In callback handler:
const user = await pg.callback();
console.log(user.did, user.isKycVerified);

### 7.2 SDK Methods

- pg.login(scopes): Redirect to Penguin Login UI
- pg.callback(): Process callback after user authentication
- pg.verify(proof): Verify ZK proof server-side
- pg.getCredentials(): List user's available credentials
- pg.requestCredential(type): Ask user for specific credential type
- pg.logout(): Clear session

### 7.3 SDK Availability

- TypeScript/JavaScript: npm install @penguin-gateway/sdk
- Python: pip install penguin-gateway
- Go: import 'github.com/penguin-gateway/go-sdk'
- REST API: Direct HTTP for any language
- Web3.js Plugin: Integrate with existing Web3 wallet flows

## 8. Api Design

### 8.1 REST Endpoints

- POST /auth/register: Create new Penguin Gateway account
- POST /auth/login: Authenticate user
- POST /auth/logout: End session
- GET /credentials: List user credentials
- POST /credentials/request: Request credential from issuer
- POST /credentials/prove: Generate ZK proof for credential
- POST /verify: Verify ZK proof (dApp backend)
- GET /issuers: List registered credential issuers
- GET /dapps: Discover integrated dApps

### 8.2 GraphQL API

- Full GraphQL endpoint for complex queries
- Query user credentials with filtering and sorting
- Subscription support for real-time credential updates
- Compatible with Apollo Client and urql

### 8.3 Authentication Protocol

- JWT Tokens: Signed JWTs returned after authentication
- Token Structure: Header, Payload (user DID, scopes), Signature
- Refresh Tokens: Rotate tokens every 1 hour; refresh valid for 30 days
- Token Verification: Verify signature using user's public key from blockchain
- Revocation: Tokens revoked immediately upon logout or credential change

## 9. Security & Cryptography

### 9.1 Cryptographic Standards

- Key Derivation: PBKDF2-SHA256 for passkey-based key derivation
- Signing: Ed25519 for signatures (Chia standard)
- Encryption: AES-256-GCM for credential storage
- Hashing: SHA-256 for merkle trees and commitment schemes
- Zero-Knowledge: BLS12-381 curve for ZK proofs (post-quantum alternatives available)

### 9.2 Key Management

- Master Key: Derived from Passkey; never transmitted
- DID Key Pair: Used for DID registration and credential verification
- Hardware Security: Keys can be stored in hardware wallets (Ledger, Trezor)
- Key Rotation: Users can rotate keys without changing DID
- Recovery Keys: Optional backup keys for account recovery

### 9.3 Privacy & Data Protection

- End-to-End Encryption: All data encrypted with user's keys
- No Server Storage: Credentials stored on DIG, not on central servers
- Zero Tracking: No tracking of user activity or credential usage
- GDPR Compliance: Users can export and delete all data
- DIG Privacy: DIG network is private; users control read access

### 9.4 Security Audits & Testing

- Annual Security Audits: Third-party audits by CertiK, Trail of Bits
- Penetration Testing: Quarterly pen tests and bug bounty program
- Code Review: All code reviewed before deployment
- Automated Testing: CI/CD pipeline with security scanning
- Incident Response: 24/7 security team and SLA-based response times

## 10. Performance & Scalability

### 10.1 Performance Targets

- Login Latency: <500ms (p99)
- Credential Proof Generation: <1 second
- ZK Proof Verification: <100ms
- API Response Time: <50ms (p50), <200ms (p99)
- Availability: 99.99% uptime SLA
- Throughput: 10,000+ login requests per second

### 10.2 Scaling Strategy

- Horizontal Scaling: API servers scale with load balancing
- Caching Layer: Redis caching for frequent lookups
- DIG Network Scaling: DIG nodes replicate across regions
- Blockchain Batching: Batch multiple operations into single transaction
- Database Optimization: PostgreSQL sharding by user DID
- CDN Deployment: Global edge deployment for low latency

### 10.3 Cost Per Authentication

## 11. Deployment Architecture

### 11.1 Infrastructure Stack

- Orchestration: Kubernetes (EKS on AWS)
- API Servers: Node.js/FastAPI in containers
- Database: PostgreSQL (multi-AZ replication)
- Cache: Redis cluster for session management
- Blockchain Nodes: Full Chia nodes in each region
- DIG Nodes: DIG network replication across regions
- CDN: Cloudflare for API and static asset delivery

### 11.2 Multi-Region Deployment

- Primary: AWS us-east-1 (US East)
- Secondary: AWS eu-west-1 (Europe) for GDPR
- Tertiary: AWS ap-southeast-1 (Asia-Pacific)
- Cross-Region Replication: DIG nodes sync across all regions
- Failover: Automatic failover if primary region down
- RTO/RPO: <5 minute recovery time; zero data loss

### 11.3 Monitoring & Observability

- Logging: Centralized logging with ELK stack
- Metrics: Prometheus + Grafana for real-time monitoring
- Tracing: Jaeger for distributed tracing
- Alerting: PagerDuty integration for incident response
- Health Checks: Continuous health monitoring of all components

## 12. Code Examples

### 12.1 User Registration (Backend)

async function registerUser(passkey) {
// 1. Derive master key from passkey
const masterKey = await deriveKey(passkey);

// 2. Generate DID
const keyPair = await generateKeyPair();
const did = createDID(keyPair.publicKey);

// 3. Register DID on blockchain
const tx = await registerDIDOnChia(did, keyPair.publicKey);

// 4. Create DIG store
const digStore = await createDIGStore(did, masterKey);

// 5. Return user object
return {
did,
publicKey: keyPair.publicKey,
digStoreRef: digStore.reference
};
}

### 12.2 Credential Verification (dApp Backend)

async function verifyLogin(zkProof, userDID) {
// 1. Verify ZK proof
const isProofValid = await verifyZKProof(zkProof);
if (!isProofValid) throw new Error("Invalid proof");

// 2. Get user's DID from blockchain
const didData = await queryChiaBlockchain(userDID);

// 3. Verify signature using user's public key
const isSignatureValid = verify(
zkProof.signature,
didData.publicKey
);
if (!isSignatureValid) throw new Error("Invalid signature");

// 4. Verify credential on DIG
const credentialRoot = didData.credentialMerkleRoot;
const isInMerkleTree = verifyMerkleProof(
zkProof.credentialHash,
credentialRoot
);
if (!isInMerkleTree) throw new Error("Credential not found");

// 5. User authenticated
return createJWT({ sub: userDID });
}
For technical questions: tech@penguingateway.io
