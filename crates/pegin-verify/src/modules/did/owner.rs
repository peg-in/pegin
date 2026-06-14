//! On-chain DID ownership: does `cnf.did_pk` control the claimed DID singleton?
//!
//! A self-signed JWT only proves possession of the key inside it. The relying party must
//! independently confirm that key is the DID's on-chain owner. The owner's authorization
//! puzzle is a standard p2 puzzle curried with the synthetic owner key, so the binding is:
//! `StandardArgs(synthetic(did_pk)) == DID.info.p2_puzzle_hash`.

use chia_puzzle_types::standard::StandardArgs;
use chia_puzzle_types::DeriveSynthetic;
use chia_wallet_sdk::driver::Puzzle;
use chia_wallet_sdk::prelude::{Allocator, Bytes32, Coin, Did, Program, PublicKey, ToClvm};

use super::did_repository::{CoinJson, CoinRecord, CoinsetClient};

/// Bound on singleton hops while walking to the current DID coin (guards bad data).
const MAX_LINEAGE_STEPS: u32 = 256;

/// A coin's three identifying fields, as raw bytes (coinset returns these per record).
pub struct CoinParts {
    pub parent_coin_info: [u8; 32],
    pub puzzle_hash: [u8; 32],
    pub amount: u64,
}

impl CoinParts {
    fn to_coin(&self) -> Coin {
        Coin::new(
            Bytes32::from(self.parent_coin_info),
            Bytes32::from(self.puzzle_hash),
            self.amount,
        )
    }
}

/// The standard p2 puzzle hash a DID owned by `did_pk` must carry on-chain.
///
/// * `did_pk` — 48-byte BLS G1 public key (the JWT's `cnf.did_pk`)
pub fn expected_owner_p2(did_pk: &[u8; 48]) -> Result<[u8; 32], String> {
    let pk = PublicKey::from_bytes(did_pk).map_err(|e| format!("invalid DID public key: {e}"))?;
    let synthetic = pk.derive_synthetic();
    Ok(<[u8; 32]>::from(StandardArgs::curry_tree_hash(synthetic)))
}

/// Parses the current DID coin's owner p2 puzzle hash from its parent's spend.
///
/// * `parent` / `child` — the spent DID singleton and the unspent coin it created
/// * `parent_puzzle_reveal` / `parent_solution` — serialized CLVM from `get_puzzle_and_solution`
/// * returns `Err` when the spend is not a DID singleton (so a forged DID can't pass)
pub fn parse_owner_p2_hash(
    parent: &CoinParts,
    parent_puzzle_reveal: &[u8],
    parent_solution: &[u8],
    child: &CoinParts,
) -> Result<[u8; 32], String> {
    let mut allocator = Allocator::new();

    let puzzle_ptr = Program::from(parent_puzzle_reveal.to_vec())
        .to_clvm(&mut allocator)
        .map_err(|e| format!("invalid DID puzzle reveal: {e}"))?;
    let solution_ptr = Program::from(parent_solution.to_vec())
        .to_clvm(&mut allocator)
        .map_err(|e| format!("invalid DID solution: {e}"))?;

    let puzzle = Puzzle::parse(&allocator, puzzle_ptr);
    let did = Did::<()>::parse_child(
        &mut allocator,
        parent.to_coin(),
        puzzle,
        solution_ptr,
        child.to_coin(),
    )
    .map_err(|e| format!("not a DID singleton spend: {e}"))?
    .ok_or_else(|| "parent spend did not create a DID child".to_owned())?;

    Ok(did.info.p2_puzzle_hash.to_bytes())
}

/// True when `did_pk` is the on-chain owner of the DID coin described by the parent spend.
pub fn is_owner(
    did_pk: &[u8; 48],
    parent: &CoinParts,
    parent_puzzle_reveal: &[u8],
    parent_solution: &[u8],
    child: &CoinParts,
) -> Result<bool, String> {
    let on_chain = parse_owner_p2_hash(parent, parent_puzzle_reveal, parent_solution, child)?;
    Ok(on_chain == expected_owner_p2(did_pk)?)
}

fn hex32(s: &str) -> Result<[u8; 32], String> {
    hex::decode(s.trim_start_matches("0x"))
        .map_err(|e| format!("invalid coin hex: {e}"))?
        .try_into()
        .map_err(|_| "coin field must be 32 bytes".to_owned())
}

impl CoinParts {
    fn from_json(c: &CoinJson) -> Result<Self, String> {
        Ok(Self {
            parent_coin_info: hex32(&c.parent_coin_info)?,
            puzzle_hash: hex32(&c.puzzle_hash)?,
            amount: c.amount,
        })
    }

    fn coin_id_hex(&self) -> String {
        hex::encode(self.to_coin().coin_id().to_bytes())
    }
}

/// The singleton child carrying odd amount (the continuing DID coin).
fn odd_child(records: Vec<CoinRecord>) -> Option<CoinRecord> {
    records.into_iter().find(|r| r.coin.amount % 2 == 1)
}

/// Resolves the live DID: walks launcher → current unspent singleton, parses its owner p2
/// from the parent spend, and checks `StandardArgs(synthetic(did_pk))` matches. Fail-closed:
/// any network/parse error or a missing owner is an `Err`, never a silent `false`.
pub async fn verify_did_owner(
    client: &CoinsetClient,
    launcher_hex: &str,
    did_pk: &[u8; 48],
) -> Result<bool, String> {
    let mut current = odd_child(client.coins_by_parent(launcher_hex).await?)
        .ok_or_else(|| "DID launcher has no singleton child".to_owned())?;

    for _ in 0..MAX_LINEAGE_STEPS {
        if !current.spent {
            let child = CoinParts::from_json(&current.coin)?;
            let parent_id = current
                .coin
                .parent_coin_info
                .trim_start_matches("0x")
                .to_owned();
            let parent_rec = client
                .coin_record(&parent_id)
                .await?
                .ok_or_else(|| "DID parent coin not found".to_owned())?;
            let parent = CoinParts::from_json(&parent_rec.coin)?;
            let spend = client
                .coin_spend(&parent_id, parent_rec.spent_block_index)
                .await?;
            let on_chain =
                parse_owner_p2_hash(&parent, &spend.puzzle_reveal, &spend.solution, &child)?;
            return Ok(on_chain == expected_owner_p2(did_pk)?);
        }
        let coin_id = CoinParts::from_json(&current.coin)?.coin_id_hex();
        current = odd_child(client.coins_by_parent(&coin_id).await?)
            .ok_or_else(|| "DID singleton lineage ended (melted?)".to_owned())?;
    }
    Err("DID lineage exceeds maximum depth".to_owned())
}
