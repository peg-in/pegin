//! Resolve canonical `did:chia` when the JWT `iss` is an owner-key assertion (feat-37).

use chia_wallet_sdk::prelude::{Bytes32, Coin};

use super::did_helper::{encode_did, is_singleton_launcher, launcher_id_hex};
use super::did_repository::{CoinRecord, CoinsetClient};
use super::owner::expected_owner_p2;

const MAX_LINEAGE_STEPS: u32 = 128;

/// Canonical DID for login verification.
///
/// When `jwt_iss` is already `did:chia:…`, returns it unchanged. On first login the
/// mini wallet asserts `iss=sub=<owner_pk>`; the relay resolves the launcher via coinset
/// hints derived from `cnf.did_pk`.
pub async fn resolve_login_did(
    jwt_iss: &str,
    owner_pk: &[u8; 48],
    client: &CoinsetClient,
) -> Result<String, String> {
    if jwt_iss.starts_with("did:chia:") {
        launcher_id_hex(jwt_iss)?;
        return Ok(jwt_iss.to_owned());
    }
    if jwt_iss.len() == 96 && jwt_iss.chars().all(|c| c.is_ascii_hexdigit()) {
        return lookup_did_by_owner_pk(client, owner_pk).await;
    }
    Err(format!("unrecognized login subject '{jwt_iss}'"))
}

async fn lookup_did_by_owner_pk(
    client: &CoinsetClient,
    owner_pk: &[u8; 48],
) -> Result<String, String> {
    let hint = owner_address_hint(owner_pk)?;
    let records = client.coins_by_hints(&[hint]).await?;
    let Some(record) = pick_unspent_did(&records) else {
        return Err("no on-chain DID for this owner key".to_owned());
    };
    let coin_id = coin_id_hex(record)?;
    let launcher = find_launcher_id(client, &coin_id).await?;
    encode_did(&launcher)
}

/// Address puzzle hash hinted on DID coins owned by `owner_pk` (synthetic observer key).
fn owner_address_hint(owner_pk: &[u8; 48]) -> Result<String, String> {
    let hash = expected_owner_p2(owner_pk)?;
    Ok(format!("0x{}", hex::encode(hash)))
}

/// The owner's DID singleton among hinted coins. `coins_by_hints` already excludes
/// spent coins, so any unspent amount-1 coin is a live singleton candidate.
pub(super) fn pick_unspent_did(records: &[CoinRecord]) -> Option<&CoinRecord> {
    records.iter().find(|r| !r.spent && r.coin.amount == 1)
}

pub(super) fn coin_id_hex(record: &CoinRecord) -> Result<String, String> {
    let parent = hex32(&record.coin.parent_coin_info)?;
    let puzzle_hash = hex32(&record.coin.puzzle_hash)?;
    let coin = Coin::new(
        Bytes32::from(parent),
        Bytes32::from(puzzle_hash),
        record.coin.amount,
    );
    Ok(hex::encode(coin.coin_id().to_bytes()))
}

async fn find_launcher_id(client: &CoinsetClient, coin_id_hex: &str) -> Result<String, String> {
    let mut coin_name = coin_id_hex.to_owned();
    for _ in 0..MAX_LINEAGE_STEPS {
        let Some(record) = client.coin_record(&coin_name).await? else {
            return Err("DID coin lineage not found on-chain".to_owned());
        };
        if is_singleton_launcher(&record.coin.puzzle_hash) {
            return Ok(coin_name.trim_start_matches("0x").to_ascii_lowercase());
        }
        coin_name = record
            .coin
            .parent_coin_info
            .trim_start_matches("0x")
            .to_ascii_lowercase();
    }
    Err("DID coin lineage exceeds maximum depth".to_owned())
}

fn hex32(s: &str) -> Result<[u8; 32], String> {
    hex::decode(s.trim_start_matches("0x"))
        .map_err(|e| format!("invalid coin hex: {e}"))?
        .try_into()
        .map_err(|_| "coin field must be 32 bytes".to_owned())
}
