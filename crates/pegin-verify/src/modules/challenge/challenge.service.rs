//! Challenge signature verification (BLS `AugSchemeMPL` on DID key).

use chia_bls::{verify, PublicKey, Signature};

pub fn verify_challenge_signature(
    did_pk_bytes: &[u8],
    challenge: &str,
    signature_hex: &str,
) -> Result<(), String> {
    let pk_bytes: [u8; 48] = did_pk_bytes
        .try_into()
        .map_err(|_| "DID public key must be 48 bytes".to_owned())?;
    let pk =
        PublicKey::from_bytes(&pk_bytes).map_err(|e| format!("invalid DID public key: {e}"))?;
    let sig_bytes =
        hex::decode(signature_hex).map_err(|e| format!("invalid signature hex: {e}"))?;
    let sig_array: [u8; 96] = sig_bytes
        .try_into()
        .map_err(|_| "challenge signature must be 96 bytes".to_owned())?;
    let sig =
        Signature::from_bytes(&sig_array).map_err(|e| format!("invalid BLS signature: {e}"))?;
    if verify(&sig, &pk, challenge.as_bytes()) {
        Ok(())
    } else {
        Err("challenge signature does not verify".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use chia_bls::{sign, SecretKey};

    use super::*;

    #[test]
    fn round_trip_challenge_verify() {
        let sk = SecretKey::from_bytes(&[9u8; 32]).expect("key");
        let pk = sk.public_key();
        let challenge = "login-nonce-abc";
        let sig = sign(&sk, challenge.as_bytes());
        verify_challenge_signature(&pk.to_bytes(), challenge, &hex::encode(sig.to_bytes()))
            .expect("verify");
    }
}
