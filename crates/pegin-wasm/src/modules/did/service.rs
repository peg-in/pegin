/// Native (non-WASM) stub — validates format only.
/// The real fetch runs in the browser via the `#[cfg(target_arch = "wasm32")]` branch.
#[cfg(not(target_arch = "wasm32"))]
pub fn get_did_inner(launcher_id_hex: &str, _base_url: Option<&str>) -> Result<String, String> {
    if launcher_id_hex.len() != 64 || !launcher_id_hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(format!(
            "launcher_id must be 64 lowercase hex chars, got '{launcher_id_hex}'"
        ));
    }
    Ok(format!("did:chia:{launcher_id_hex}"))
}

#[cfg(target_arch = "wasm32")]
pub async fn get_did_inner(
    launcher_id_hex: &str,
    base_url: Option<&str>,
) -> Result<String, String> {
    use gloo_net::http::Request;

    const TESTNET11_URL: &str = "https://testnet11.api.coinset.org";
    let base = base_url.unwrap_or(TESTNET11_URL);
    let url = format!("{base}/get_coin_record_by_name");
    let body = serde_json::json!({ "name": format!("0x{launcher_id_hex}") });

    let resp = Request::post(&url)
        .json(&body)
        .map_err(|e| format!("serialisation error: {e}"))?
        .send()
        .await
        .map_err(|e| format!("network error: {e}"))?;

    #[derive(serde::Deserialize)]
    struct CoinRecordResponse {
        success: bool,
        error: Option<String>,
        coin_record: Option<serde_json::Value>,
    }

    let data: CoinRecordResponse = resp.json().await.map_err(|e| format!("parse error: {e}"))?;

    if !data.success {
        let msg = data.error.unwrap_or_else(|| "unknown error".to_owned());
        return Err(format!("coinset: {msg}"));
    }

    data.coin_record
        .ok_or_else(|| "DID coin not found on-chain".to_owned())
        .map(|_| format!("did:chia:{launcher_id_hex}"))
}

// Tests only run on native (non-WASM) target; browser fetch is not testable
// with cargo test. WASM-target behaviour is validated via wasm-pack test.
#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    #[test]
    fn rejects_invalid_launcher_id_format() {
        let err = get_did_inner("not-hex", None).unwrap_err();
        assert!(err.contains("64 lowercase hex chars"));
    }

    #[test]
    fn rejects_wrong_length() {
        let err = get_did_inner("deadbeef", None).unwrap_err();
        assert!(err.contains("64 lowercase hex chars"));
    }

    #[test]
    fn returns_did_string_for_valid_input() {
        let launcher_id = "deadbeef".repeat(8); // 64 hex chars
        let result = get_did_inner(&launcher_id, None).unwrap();
        assert_eq!(result, format!("did:chia:{launcher_id}"));
    }
}
