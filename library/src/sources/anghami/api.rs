use serde_json::Value;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

const BASE_URL: &str = "https://api.anghami.com/gateway.php";

pub fn unix_ts() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

pub fn base_request(builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
    builder
        .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .header("Referer", "https://play.anghami.com/")
        .header("Origin", "https://play.anghami.com")
}

pub async fn api_request(
    client: &Arc<reqwest::Client>,
    udid: &str,
    params: Vec<(&str, &str)>,
) -> Option<Value> {
    let mut url = reqwest::Url::parse(BASE_URL).ok()?;
    {
        let mut q = url.query_pairs_mut();
        for (k, v) in params {
            q.append_pair(k, v);
        }
    }
    let resp = base_request(client.get(url))
        .header("X-ANGH-UDID", udid)
        .header("X-ANGH-TS", unix_ts().to_string())
        .send()
        .await
        .ok()?;
    if !resp.status().is_success() {
        return None;
    }
    resp.json::<Value>().await.ok()
}
