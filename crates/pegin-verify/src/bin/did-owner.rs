//! Live DID-ownership check against coinset. Prints `{"owned":bool}`.
//!
//! Usage: did-owner --launcher <hex> --did-pk <hex48> [--coinset-url <url>]

use pegin_verify::did::{verify_did_owner, CoinsetClient};

#[tokio::main]
async fn main() {
    let mut launcher = None;
    let mut did_pk = None;
    let mut url = "https://testnet11.api.coinset.org".to_owned();

    let mut args = std::env::args().skip(1);
    while let Some(flag) = args.next() {
        match flag.as_str() {
            "--launcher" => launcher = args.next(),
            "--did-pk" => did_pk = args.next(),
            "--coinset-url" => {
                if let Some(v) = args.next() {
                    url = v;
                }
            }
            other => fail(&format!("unexpected argument '{other}'")),
        }
    }

    let launcher = launcher.unwrap_or_else(|| fail("--launcher required"));
    let did_pk_hex = did_pk.unwrap_or_else(|| fail("--did-pk required"));
    let did_pk: [u8; 48] = hex::decode(did_pk_hex.trim_start_matches("0x"))
        .ok()
        .and_then(|b| b.try_into().ok())
        .unwrap_or_else(|| fail("--did-pk must be 48-byte hex"));

    match verify_did_owner(
        &CoinsetClient::new(url),
        launcher.trim_start_matches("0x"),
        &did_pk,
    )
    .await
    {
        Ok(owned) => {
            // Result is program output → stdout.
            println!("{{\"owned\":{owned}}}");
            std::process::exit(i32::from(!owned));
        }
        Err(e) => fail(&e),
    }
}

fn fail(msg: &str) -> ! {
    eprintln!("did-owner: {msg}");
    std::process::exit(2);
}
