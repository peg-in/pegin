# PEGIN on-chain architecture

> **Hub:** [architecture-overview.md](architecture-overview.md) · **Programmers:** [../08-developer/developer-documentation.md](../08-developer/developer-documentation.md) · **Spec 1:** [../04-technical/specs/tech-stack.md](../04-technical/specs/tech-stack.md)  
> **Data plane:** DIDs and contracts on Chia; user data and audit on **DIG** (see [fully-decentralized.md](../../01-vision/fully-decentralized.md)).

*Chia anchoring + verification — not “everything on chain.”*

## Table of contents

- [Architecture Overview](#architecture-overview)
- [Smart Contracts (Chialisp)](#smart-contracts-chialisp)
- [User Identity System](#user-identity-system)
- [Credential Model (NFTs)](#credential-model-nfts)
- [Verification Flow](#verification-flow)
- [Data Structures](#data-structures)
- [SDK & Client Libraries](#sdk-client-libraries)
- [DIG Network Integration (Optional)](#dig-network-integration-optional)
- [Security & Cryptography](#security-cryptography)
- [Performance & Scalability](#performance-scalability)
- [Deployment Model](#deployment-model)
- [Code Examples](#code-examples)

## 1. Architecture Overview
PEGIN is built entirely on Chia blockchain. No central servers. No databases. No APIs. Everything
lives on-chain.
### 1.1 Core Components
- Smart Contracts (Chialisp): DID registration, credential issuance, verification, revocation
- User Wallets: Store identity NFTs and credential NFTs in Chia wallets
- Blockchain: Chia blockchain is the single source of truth
- SDK/Clients: Open-source libraries for developers to interact with smart contracts
- Indexers (Optional): Community-run indexers for faster queries (not required)
- DIG Network (Optional): Off-chain metadata storage with on-chain merkle roots
### 1.2 Why This Design?
- No Single Point of Failure: If I disappear, Chia blockchain keeps PEGIN alive
- Immutable: Smart contracts deployed once, cannot be changed
- Transparent: All code open-source, community can audit
- Censorship-Resistant: No central entity can shut down PEGIN
- Sustainable: System runs on Chia network, I don't need to maintain servers

## 2. Smart Contracts (Chialisp)
PEGIN uses immutable Chialisp smart contracts deployed on Chia blockchain. These contracts
handle all core PEGIN functionality.
### 2.1 Core Smart Contracts
- DID Registration Contract: Allows users to register Decentralized Identifier (did:chia:xyz) on
blockchain. Output: DID coin with public key.
- Identity NFT Contract: Creates user's identity NFT. Stores DID, public key, credential index merkle
root. Transferable between wallets.
- Credential Issuer Contract: Registers new credential issuer (bank, university). Mints issuer NFT.
Only issuer NFT holder can issue credentials.
- Credential NFT Contract: Issues individual credentials as NFTs. Stores issuer signature, issue
date, expiry, credential type. Tradeable on secondary market.
- Verification Contract: Verifies credential signature + issuer authorization. Called by dApps during
login. Returns boolean (valid/invalid).
- Revocation Contract: Allows issuers to revoke credentials. Stores revoked credential hashes
on-chain. Verification contract checks revocation list.
### 2.2 Contract Deployment
- Deploy once on Chia mainnet (testnet first)
- Contracts are immutable (cannot be upgraded)
- If bug found, deploy new version with higher version number
- Old contracts remain on blockchain for backward compatibility
- Governance voting determines if contracts need updates

## 3. User Identity System
### 3.1 Decentralized Identifier (DID)
- Format: did:chia:pubkeyhash (e.g., did:chia:xyz123abc456)
- Created by: User's public key hash + nonce
- Unique: One DID per user (can have multiple identities)
- Immutable: DID never changes after creation
- Transferable: User can transfer DID to new wallet
- W3C Compatible: Follows W3C DID specification
### 3.2 Identity NFT Structure
{
 "id": "pegin-identity-{did}",
 "type": "PEGIN-Identity-NFT",
 "owner": "{wallet_address}",
 "did": "did:chia:xyz123",
 "publicKey": "0x...",
 "credentialRoot": "{merkle_root}",
 "issuedDate": "2024-01-15",
 "recoveryKeys": ["key1", "key2"],
 "metadata": {
 "displayName": "Alice",
 "avatar": "https://..."
 }
}
### 3.3 Key Management
- Master Key: User's primary key (stored in Chia Signer or hardware wallet)
- Signing Key: Used for all PEGIN transactions (same as master key)
- Public Key: Published on blockchain for signature verification
- Recovery Keys: Optional backup keys for account recovery
- Hardware Support: Works with Ledger, Trezor, or any hardware wallet

## 4. Credential Model (Nfts)
Every credential is an NFT stored on Chia blockchain. Credentials are W3C Verifiable Credentials
with NFT wrapper.
### 4.1 Credential NFT Structure
{
 "id": "pegin-credential-{uuid}",
 "type": "PEGIN-Credential-NFT",
 "issuer": "did:chia:bank123",
 "subject": "did:chia:user456",
 "credentialType": "KYCCredential",
 "claims": {
 "kycVerified": true,
 "verificationLevel": "high",
 "verificationDate": "2024-01-15"
 },
 "issueDate": "2024-01-15",
 "expirationDate": "2025-01-15",
 "issuerSignature": "0xabc123...",
 "issuerNFT": "pegin-issuer-bank123",
 "revocationStatus": "active",
 "tradeable": true,
 "royaltyPercentage": 3
}
### 4.2 Credential Lifecycle
1. Issuance: Issuer creates credential NFT, signs with issuer key, broadcasts to Chia
2. Ownership Transfer: Credential NFT transferred to user's wallet
3. Storage: User stores credential in Chia wallet or cold storage
4. Presentation: User proves credential (shows signature + issuer verification)
5. Verification: dApp verifies issuer signature + checks on-chain revocation status
6. Trading (Optional): User sells credential on NFT marketplace (PEGIN gets 2-5% royalty)
7. Expiration/Revocation: Credential expires or issuer revokes it (added to revocation registry)
### 4.3 Credential Types Supported
- KYC/AML: Know-Your-Customer identity verification
- Education: Diplomas, degrees, certifications
- Employment: Job title, employment proof, salary verification

- Credit: Credit score, credit history
- Medical: Vaccination records, health credentials
- Legal: Power of attorney, notarization
- Custom: Any issuer can define custom credential type

## 5. Verification Flow
How dApps verify user credentials without calling a central API.
### 5.1 Verification Steps
1. User provides credential: Sends credential NFT ID + signature proof
2. dApp queries blockchain: Fetches credential details from Chia RPC
3. Verify Issuer NFT: Confirms issuer NFT exists and is authorized
4. Verify Signature: Checks issuer's digital signature on credential
5. Check Revocation: Queries revocation contract - is credential revoked?
6. Check Expiration: Verifies current date < expirationDate
7. Result: Returns boolean (verified = true/false)
### 5.2 Example: Login Flow
User clicks "Login with PEGIN" on dApp
↓
dApp redirects to PEGIN client (local or web)
↓
User authenticates (Chia Signer or Passkey)
↓
User selects which credential to share (e.g., "KYC verified")
↓
PEGIN client signs credential proof
↓
User approves and sends to dApp
↓
dApp verifies credential on Chia blockchain
↓
Smart contract returns: valid ✓
↓
dApp creates session, user logged in

## 6. Data Structures
### 6.1 DID Coin (On-Chain)
- Puzzle Hash: Chialisp program hash (verifies ownership)
- Amount: 1 mojo (minimal cost)
- Parent Coin: References previous DID transaction
- Metadata: DID info in puzzle solution
### 6.2 Credential Coin (On-Chain)
- Puzzle Hash: Credential verification logic
- Amount: 1 mojo
- Parent Coin: References issuer NFT
- Metadata: Credential hash, issuer signature, expiry
### 6.3 Issuer NFT (On-Chain)
- ID: Unique issuer identifier
- Name: Issuer name (e.g., 'Bank of America')
- Public Key: Issuer's signing key
- Authorization Level: Which credential types can issue
- Revocation Count: Number of revoked credentials
- Metadata: Logo, website, contact info (optional)
### 6.4 Merkle Tree (Credential Index)
- Purpose: Efficiently store all user credentials without bloating blockchain
- Root Hash: Stored in user's identity NFT
- Leaves: Each credential's hash
- Proof: User can prove credential exists without revealing others
- Update: When new credential issued, merkle root updated on-chain

## 7. Sdk & Client Libraries
Developers use open-source SDKs to interact with PEGIN smart contracts.
### 7.1 TypeScript SDK
- Package: npm install @pegin/sdk
- Classes: PeginUser, PeginIssuer, PeginVerifier
- Methods: registerDID(), issueCredential(), verifyCredential()
- Type Safe: Full TypeScript support
- No Backend Needed: SDK queries blockchain directly
### 7.2 Example Code (TypeScript)
import { PeginUser, PeginVerifier } from '@pegin/sdk';
// User registers DID
const user = await PeginUser.registerDID({
 publicKey: userPublicKey
});
console.log(user.did); // did:chia:xyz123
// Issuer issues credential
const issuer = new PeginIssuer(issuerKey);
const credential = await issuer.issueCredential({
 subject: user.did,
 type: 'KYCCredential',
 claims: { kycVerified: true }
});
// dApp verifies credential
const verifier = new PeginVerifier();
const isValid = await verifier.verify(credential);
console.log(isValid); // true or false
### 7.3 Other SDKs
- Python: pip install pegin-sdk
- Go: import 'github.com/pegin/go-sdk'
- Rust: cargo add pegin-sdk
- Direct RPC: Query Chia RPC endpoint directly (no SDK needed)

## 8. Dig Network Integration (Optional)
DIG network can optionally store credential metadata off-chain while keeping merkle roots on-chain.
### 8.1 Why DIG (Optional)?
- Scalability: Store large credential documents off-chain
- Privacy: Metadata encrypted; only merkle root on-chain
- Bandwidth: Users don't download entire credential set
- Optional: dApps can use just Chia blockchain if preferred
### 8.2 DIG Store Structure
- One store per user: User's DIG datastore for credentials
- Encrypted: All data encrypted with user's key
- Merkle Root: Root hash published on Chia blockchain
- Proof of Inclusion: User can prove credential exists without revealing details

## 9. Security & Cryptography
### 9.1 Cryptographic Standards
- Signing: Ed25519 (Chia standard, secure, post-quantum resistant)
- Hashing: SHA-256 for merkle trees and content hashing
- Encryption (Optional): ChaCha20-Poly1305 for DIG metadata
- Key Derivation: PBKDF2-SHA256 for Passkey-based keys
### 9.2 Threat Model
- Stolen Wallet: User's private key compromised → Attacker can issue credentials. Mitigated: User
can recover with recovery keys or freeze identity.
- Compromised Issuer: Issuer's key stolen → Attacker issues fake credentials. Mitigated:
Community/governance can revoke issuer NFT.
- Blockchain Fork: Chia blockchain forks → PEGIN follows the majority fork. Mitigated: Use finalized
blocks only.
- Zero-Day in Chialisp: Bug in smart contracts → Exploit found. Mitigated: Immutable contracts
mean deploy v2 contract, keep v1 for backward compatibility.
### 9.3 Auditing & Testing
- Code Audits: Third-party audit of Chialisp contracts (CertiK, Trail of Bits)
- Formal Verification: Mathematically prove contract correctness (optional)
- Testnet: 6+ months of testnet before mainnet launch
- Bug Bounty: HackerOne program after launch
- Community Review: Open-source code for community audit

## 10. Performance & Scalability
### 10.1 On-Chain Performance
- DID Registration: 1 Chia block (10-20 seconds)
- Credential Issuance: 1-2 Chia blocks (20-40 seconds)
- Verification: Smart contract execution (<100ms)
- Revocation: 1 Chia block (10-20 seconds)
- Throughput: 100+ transactions/second (Chia network capacity)
### 10.2 Scalability Solutions
- Batch Issuance: Issuer batches 1000 credentials in single transaction
- Indexing: Community run indexers for faster queries
- Layer 2 (Future): If needed, sidechains or rollups (not in v1)
- DIG Network: Off-chain metadata storage scales horizontally
### 10.3 Cost Per Transaction
- Chia Fees: ~0.000000001 XCH per transaction (sub-microcent)
- DIG Storage: Negligible (P2P cost)

## 11. Deployment Model
### 11.1 No Central Infrastructure
- No Central API: dApps query blockchain directly or run own indexer
- No Central Database: All data on Chia blockchain
- No Central Authentication: Users sign with Chia Signer or Passkey
- No Central Storage: Credentials in user's wallet
- Result: PEGIN works even if my servers go offline forever
### 11.2 Deployment Steps
1. Smart Contracts: Deploy Chialisp contracts to Chia testnet
2. Testing: 6+ months of testnet phase, community testing
3. Mainnet: Deploy to Chia mainnet (immutable from then on)
4. Indexers: Community members can deploy indexers (optional)
5. SDKs: Release SDKs on npm, PyPI, GitHub
6. Adoption: dApps integrate PEGIN SDKs
### 11.3 Open-Source Repositories
- pegin-contracts: Chialisp smart contracts
- pegin-sdk-ts: TypeScript SDK
- pegin-sdk-py: Python SDK
- pegin-docs: Documentation and guides
- pegin-indexer: Example indexer implementation

## 12. Code Examples
### 12.1 Issuer Registration (On-Chain)
// Issuer creates NFT coin
// Amount: 1 mojo
// Puzzle: issuer_nft_puzzle
// Solution: {
// issuer_name: "Bank of America",
// issuer_pubkey: 0xabc123,
// credential_types: ["KYC", "employment"]
// }
(defun issuer_nft ((issuer_data (f issuer_name issuer_pubkey)))
 (assert (valid_public_key issuer_pubkey))
 (assert (not_empty issuer_name))
 (create_coin issuer_nft_puzzle issuer_data 1)
)
### 12.2 Credential Verification (On-Chain)
// Verify credential signature
(defun verify_credential ((credential_data) (issuer_pubkey) (signature))
 ;; 1. Check issuer signature
 (assert (verify_signature issuer_pubkey credential_data signature))
 
 ;; 2. Check revocation status
 (assert (not (is_revoked credential_id)))
 
 ;; 3. Check expiration
 (assert (> expiration_date (current_block_time)))
 
 ;; Return: valid
 (return TRUE)
)
 For technical questions: tech@pegin.io
