//! Relay client for cross-app sign requests (feat-78 subset).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignRequestDto {
    pub request_id: String,
    pub kind: String,
    pub origin: String,
    pub summary: String,
    pub spend_bundle_b64: Option<String>,
    pub message: Option<String>,
    pub return_url: Option<String>,
    pub status: String,
}

#[derive(Debug, Deserialize)]
struct FetchResponse {
    request: SignRequestDto,
}

pub async fn fetch_sign_request(
    relay_url: &str,
    request_id: &str,
) -> Result<SignRequestDto, String> {
    let url = format!("{}/request/{}", relay_url.trim_end_matches('/'), request_id);
    let resp = reqwest::Client::new()
        .get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("relay returned {}", resp.status()));
    }
    resp.json::<FetchResponse>()
        .await
        .map(|body| body.request)
        .map_err(|e| e.to_string())
}

#[derive(Serialize)]
struct CompleteBody {
    #[serde(rename = "requestId")]
    request_id: String,
    #[serde(rename = "signedBundleB64", skip_serializing_if = "Option::is_none")]
    signed_bundle_b64: Option<String>,
    #[serde(rename = "messageSigHex", skip_serializing_if = "Option::is_none")]
    message_sig_hex: Option<String>,
    #[serde(rename = "txSubmitted")]
    tx_submitted: bool,
}

pub async fn complete_sign_request(
    relay_url: &str,
    request_id: &str,
    signed_bundle_b64: Option<String>,
    message_sig_hex: Option<String>,
    tx_submitted: bool,
) -> Result<(), String> {
    let url = format!("{}/request/complete", relay_url.trim_end_matches('/'));
    let resp = reqwest::Client::new()
        .post(url)
        .json(&CompleteBody {
            request_id: request_id.to_string(),
            signed_bundle_b64,
            message_sig_hex,
            tx_submitted,
        })
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if resp.status().is_success() {
        Ok(())
    } else {
        Err(format!("relay complete failed: {}", resp.status()))
    }
}

pub async fn reject_sign_request(relay_url: &str, request_id: &str) -> Result<(), String> {
    let url = format!("{}/request/reject", relay_url.trim_end_matches('/'));
    let resp = reqwest::Client::new()
        .post(url)
        .json(&serde_json::json!({ "requestId": request_id }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if resp.status().is_success() {
        Ok(())
    } else {
        Err(format!("relay reject failed: {}", resp.status()))
    }
}

/// Parses `pegin-signer://sign?requestId=…&relay=…` deep links. The `relay` value is
/// percent-encoded by the relay, so it is decoded back to a usable URL here.
pub fn parse_sign_deep_link(url: &str) -> Option<(String, String)> {
    let rest = url
        .strip_prefix("pegin-signer://sign?")
        .or_else(|| url.strip_prefix("pegin-signer://sign/?"))?;
    let mut request_id = None;
    let mut relay = None;
    for part in rest.split('&') {
        if let Some(value) = part.strip_prefix("requestId=") {
            request_id = Some(value.to_string());
        } else if let Some(value) = part.strip_prefix("relay=") {
            relay = Some(
                urlencoding::decode(value)
                    .map(|decoded| decoded.into_owned())
                    .unwrap_or_else(|_| value.to_string()),
            );
        }
    }
    Some((request_id?, relay?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_decodes_relay() {
        let link = "pegin-signer://sign?requestId=abc-123&relay=http%3A%2F%2F127.0.0.1%3A8787";
        let (id, relay) = parse_sign_deep_link(link).expect("parse");
        assert_eq!(id, "abc-123");
        assert_eq!(relay, "http://127.0.0.1:8787");
    }

    #[test]
    fn rejects_unrelated_urls() {
        assert!(parse_sign_deep_link("https://example.com").is_none());
    }
}
