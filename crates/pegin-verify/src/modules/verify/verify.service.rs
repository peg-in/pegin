//! Full login verification for relying parties.

use std::time::{SystemTime, UNIX_EPOCH};

use pegin_jwt::verify_token;

use crate::modules::challenge::verify_challenge_signature;
use crate::modules::did::{launcher_id_hex, verify_did_owner, CoinsetClient};
use crate::shared::error::VerifyError;

/// Inputs for [`verify_login`].
#[derive(Debug, Clone)]
pub struct VerifyLoginInput<'a> {
    pub jwt: &'a str,
    pub expected_aud: &'a str,
    pub challenge_nonce: Option<&'a str>,
    pub challenge_sig_hex: Option<&'a str>,
    pub coinset: Option<CoinsetClient>,
    pub now: Option<u64>,
}

/// Successful verification result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedLogin {
    pub did: String,
    pub aud: String,
    pub did_pk_hex: String,
    pub nonce: Option<String>,
}

pub async fn verify_login(input: VerifyLoginInput<'_>) -> Result<VerifiedLogin, VerifyError> {
    let now = input.now.unwrap_or_else(current_unix_secs);
    let jwt = verify_token(input.jwt, input.expected_aud, input.challenge_nonce, now)?;

    if input.challenge_nonce.is_some() || input.coinset.is_some() {
        let pk_bytes =
            hex::decode(&jwt.did_pk_hex).map_err(|e| VerifyError::InvalidDid(e.to_string()))?;

        if let Some(nonce) = input.challenge_nonce {
            let sig = input
                .challenge_sig_hex
                .ok_or(VerifyError::ChallengeRequired)?;
            verify_challenge_signature(&pk_bytes, nonce, sig)
                .map_err(|_| VerifyError::ChallengeInvalid)?;
        }

        if let Some(client) = input.coinset {
            // Bind cnf.did_pk to the DID's on-chain owner — not just existence. Fail-closed:
            // any coinset/parse error propagates as Err, so an incomplete proof rejects the login.
            let launcher = launcher_id_hex(&jwt.did).map_err(VerifyError::InvalidDid)?;
            let pk_bytes: [u8; 48] = pk_bytes.as_slice().try_into().map_err(|_| {
                VerifyError::InvalidDid("cnf.did_pk must be 48-byte hex".to_owned())
            })?;
            let owned = verify_did_owner(&client, &launcher, &pk_bytes)
                .await
                .map_err(VerifyError::Coinset)?;
            if !owned {
                return Err(VerifyError::DidNotOwned);
            }
        }
    }

    Ok(VerifiedLogin {
        did: jwt.did,
        aud: jwt.aud,
        did_pk_hex: jwt.did_pk_hex,
        nonce: jwt.nonce,
    })
}

fn current_unix_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use chia_bls::{sign, SecretKey};
    use pegin_jwt::mint_es256k;

    use super::*;

    #[tokio::test]
    async fn verifies_jwt_and_optional_challenge() {
        let sk = SecretKey::from_bytes(&[3u8; 32]).expect("key");
        let did = "did:chia:1gt7hae94wd0c33v07k4kkwgjy9jjtcnzhwvl5yxuvmj28mqsnsjqvgw9uu";
        let did_pk_hex = hex::encode(sk.public_key().to_bytes());
        let nonce = "nonce-42";
        let challenge_sig = hex::encode(sign(&sk, nonce.as_bytes()).to_bytes());
        let jwt = mint_es256k(
            &sk,
            did,
            &did_pk_hex,
            "https://app.example",
            600,
            Some(nonce),
            100,
        )
        .expect("mint");

        let verified = verify_login(VerifyLoginInput {
            jwt: &jwt,
            expected_aud: "https://app.example",
            challenge_nonce: Some(nonce),
            challenge_sig_hex: Some(&challenge_sig),
            coinset: None,
            now: Some(100),
        })
        .await
        .expect("verify");

        assert_eq!(verified.did, did);
        assert_eq!(verified.did_pk_hex, did_pk_hex);
    }
}
