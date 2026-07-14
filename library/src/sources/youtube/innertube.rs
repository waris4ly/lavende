use crate::common::types::AnyResult;
use serde_json::{Value, json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub struct ClientProfile {
    pub label: &'static str,
    pub client_name: &'static str,
    pub numeric_id: &'static str,
    pub version: &'static str,
    pub user_agent: &'static str,
    pub can_search: bool,
    pub has_streams: bool,
    pub os_name: Option<&'static str>,
    pub os_version: Option<&'static str>,
    pub device_make: Option<&'static str>,
    pub device_model: Option<&'static str>,
    pub android_sdk: Option<&'static str>,
    pub referer: Option<&'static str>,
    pub origin: Option<&'static str>,
}

pub mod profiles {
    use super::ClientProfile;
    use crate::sources::youtube::identity::ua;

    pub static WEB: ClientProfile = ClientProfile {
        label: "Web",
        client_name: "WEB",
        numeric_id: "1",
        version: "2.20260114.01.00",
        user_agent: ua::WEB,
        can_search: true,
        has_streams: true,
        os_name: None, os_version: None, device_make: None, device_model: None,
        android_sdk: None,
        referer: Some("https://www.youtube.com/"),
        origin: Some("https://www.youtube.com"),
    };

    pub static WEB_EMBEDDED: ClientProfile = ClientProfile {
        label: "WebEmbedded",
        client_name: "WEB_EMBEDDED_PLAYER",
        numeric_id: "56",
        version: "1.20240814.01.00",
        user_agent: ua::WEB_EMBEDDED,
        can_search: false,
        has_streams: true,
        os_name: None, os_version: None, device_make: None, device_model: None,
        android_sdk: None,
        referer: Some("https://www.youtube.com/"),
        origin: Some("https://www.youtube.com"),
    };

    pub static WEB_REMIX: ClientProfile = ClientProfile {
        label: "WebRemix",
        client_name: "WEB_REMIX",
        numeric_id: "67",
        version: "1.20241218.01.00",
        user_agent: ua::WEB,
        can_search: true,
        has_streams: false,
        os_name: None, os_version: None, device_make: None, device_model: None,
        android_sdk: None,
        referer: Some("https://music.youtube.com/"),
        origin: Some("https://music.youtube.com"),
    };

    pub static ANDROID: ClientProfile = ClientProfile {
        label: "Android",
        client_name: "ANDROID",
        numeric_id: "3",
        version: "20.01.35",
        user_agent: ua::ANDROID,
        can_search: true,
        has_streams: true,
        os_name: Some("Android"), os_version: Some("14"),
        device_make: Some("Google"), device_model: Some("Pixel 6"),
        android_sdk: Some("34"),
        referer: None, origin: None,
    };

    pub static ANDROID_VR: ClientProfile = ClientProfile {
        label: "AndroidVR",
        client_name: "ANDROID_VR",
        numeric_id: "28",
        version: "1.71.26",
        user_agent: ua::ANDROID_VR,
        can_search: false,
        has_streams: true,
        os_name: Some("Android"), os_version: Some("15"),
        device_make: Some("Oculus"), device_model: Some("Quest 3"),
        android_sdk: Some("35"),
        referer: None, origin: None,
    };

    pub static IOS: ClientProfile = ClientProfile {
        label: "Ios",
        client_name: "IOS",
        numeric_id: "5",
        version: "21.02.1",
        user_agent: ua::IOS,
        can_search: true,
        has_streams: true,
        os_name: Some("iOS"), os_version: Some("18.2.0.22C152"),
        device_make: Some("Apple"), device_model: Some("iPhone16,2"),
        android_sdk: None, referer: None, origin: None,
    };

    pub static TV: ClientProfile = ClientProfile {
        label: "Tv",
        client_name: "TVHTML5",
        numeric_id: "7",
        version: "7.20260113.16.00",
        user_agent: ua::TVHTML5,
        can_search: true,
        has_streams: true,
        os_name: None, os_version: None, device_make: None, device_model: None,
        android_sdk: None, referer: None, origin: None,
    };

    pub static TV_CAST: ClientProfile = ClientProfile {
        label: "TvCast",
        client_name: "TVHTML5_CAST",
        numeric_id: "7",
        version: "7.20190924",
        user_agent: "Mozilla/5.0 (Linux; Android) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/145.0.0.0 Safari/537.36 CrKey/1.54.248666",
        can_search: false,
        has_streams: true,
        os_name: Some("Android"), os_version: None, device_make: None, device_model: None,
        android_sdk: None, referer: None, origin: None,
    };

    pub static TV_EMBEDDED: ClientProfile = ClientProfile {
        label: "TvEmbedded",
        client_name: "TVHTML5_SIMPLY_EMBEDDED_PLAYER",
        numeric_id: "85",
        version: "2.0",
        user_agent: ua::TVHTML5_SIMPLY,
        can_search: false,
        has_streams: true,
        os_name: None, os_version: None, device_make: None, device_model: None,
        android_sdk: None,
        referer: Some("https://www.youtube.com/"),
        origin: Some("https://www.youtube.com"),
    };

    pub static TV_SIMPLY: ClientProfile = ClientProfile {
        label: "TvSimply",
        client_name: "TVHTML5_SIMPLY",
        numeric_id: "85",
        version: "2.0",
        user_agent: ua::TVHTML5_SIMPLY,
        can_search: false,
        has_streams: true,
        os_name: None, os_version: None, device_make: None, device_model: None,
        android_sdk: None, referer: None, origin: None,
    };

    pub static TV_UNPLUGGED: ClientProfile = ClientProfile {
        label: "TvUnplugged",
        client_name: "TVHTML5_UNPLUGGED",
        numeric_id: "7",
        version: "7.20250811.19.00",
        user_agent: ua::TVHTML5_UNPLUGGED,
        can_search: false,
        has_streams: true,
        os_name: None, os_version: None, device_make: None, device_model: None,
        android_sdk: None, referer: None, origin: None,
    };

    pub static MWEB: ClientProfile = ClientProfile {
        label: "MWeb",
        client_name: "MWEB",
        numeric_id: "2",
        version: "2.20241022.01.00",
        user_agent: ua::MWEB,
        can_search: true,
        has_streams: true,
        os_name: None, os_version: None, device_make: None, device_model: None,
        android_sdk: None, referer: None, origin: None,
    };

    pub static MUSIC_ANDROID: ClientProfile = ClientProfile {
        label: "MusicAndroid",
        client_name: "ANDROID_MUSIC",
        numeric_id: "67",
        version: "8.47.54",
        user_agent: "com.google.android.apps.youtube.music/8.47.54 (Linux; U; Android 14 gzip)",
        can_search: true,
        has_streams: false,
        os_name: Some("Android"), os_version: Some("14"),
        device_make: Some("Google"), device_model: Some("Pixel 6"),
        android_sdk: Some("34"), referer: None, origin: None,
    };

    pub static WEB_PARENT_TOOLS: ClientProfile = ClientProfile {
        label: "WebParentTools",
        client_name: "WEB_PARENT_TOOLS",
        numeric_id: "98",
        version: "1.20240726.00.00",
        user_agent: ua::WEB,
        can_search: false,
        has_streams: true,
        os_name: None, os_version: None, device_make: None, device_model: None,
        android_sdk: None,
        referer: Some("https://www.youtube.com/"),
        origin: Some("https://www.youtube.com"),
    };

    pub static ALL: &[&ClientProfile] = &[
        &WEB, &WEB_EMBEDDED, &WEB_REMIX, &ANDROID, &ANDROID_VR,
        &IOS, &TV, &TV_CAST, &TV_EMBEDDED, &TV_SIMPLY, &TV_UNPLUGGED,
        &MWEB, &MUSIC_ANDROID, &WEB_PARENT_TOOLS,
    ];

    pub fn by_name(name: &str) -> Option<&'static ClientProfile> {
        let upper = name.to_uppercase();
        ALL.iter().copied().find(|p| {
            p.label.to_uppercase() == upper
                || p.client_name.to_uppercase() == upper
                || matches_alias(&upper, p.label)
        })
    }

    fn matches_alias(upper: &str, label: &str) -> bool {
        match (upper, label) {
            ("TVHTML5", "Tv") | ("TV", "Tv") => true,
            ("TV_CAST", "TvCast") | ("TVHTML5_CAST", "TvCast") => true,
            ("TV_EMBEDDED", "TvEmbedded") | ("TVHTML5_SIMPLY_EMBEDDED_PLAYER", "TvEmbedded") => true,
            ("TV_SIMPLY", "TvSimply") | ("TVHTML5_SIMPLY", "TvSimply") => true,
            ("TV_UNPLUGGED", "TvUnplugged") | ("TVHTML5_UNPLUGGED", "TvUnplugged") => true,
            ("REMIX", "WebRemix") | ("MUSIC_WEB", "WebRemix") | ("WEB_REMIX", "WebRemix") => true,
            ("MUSIC", "MusicAndroid") | ("MUSIC_ANDROID", "MusicAndroid") | ("ANDROID_MUSIC", "MusicAndroid") => true,
            ("ANDROIDVR", "AndroidVR") | ("ANDROID_VR", "AndroidVR") => true,
            ("WEB_EMBEDDED", "WebEmbedded") | ("WEBEMBEDDED", "WebEmbedded") => true,
            ("WEB_PARENT_TOOLS", "WebParentTools") | ("WEBPARENTTOOLS", "WebParentTools") => true,
            ("MWEB", "MWeb") => true,
            _ => false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerResponse {
    pub playability_status: PlayabilityStatus,
    pub streaming_data: Option<StreamingData>,
    pub video_details: Option<VideoDetails>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayabilityStatus {
    pub status: String,
    pub reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StreamingData {
    pub formats: Option<Vec<Format>>,
    pub adaptive_formats: Option<Vec<Format>>,
    pub hls_manifest_url: Option<String>,
    pub dash_manifest_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Format {
    pub itag: u32,
    pub mime_type: String,
    #[serde(default)]
    pub bitrate: u32,
    pub url: Option<String>,
    pub signature_cipher: Option<String>,
    pub cipher: Option<String>,
    pub content_length: Option<String>,
    pub approx_duration_ms: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VideoDetails {
    pub video_id: String,
    pub title: String,
    pub author: String,
    pub length_seconds: String,
    #[serde(default)]
    pub is_live_content: bool,
    pub thumbnail: Option<ThumbnailList>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThumbnailList {
    pub thumbnails: Vec<Thumbnail>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Thumbnail {
    pub url: String,
}

fn build_context(profile: &ClientProfile, visitor_data: Option<&str>) -> Value {
    let mut client = json!({
        "clientName": profile.client_name,
        "clientVersion": profile.version,
        "userAgent": profile.user_agent,
        "hl": "en",
        "gl": "US",
    });
    if let Some(os) = profile.os_name {
        client["osName"] = json!(os);
    }
    if let Some(osv) = profile.os_version {
        client["osVersion"] = json!(osv);
    }
    if let Some(make) = profile.device_make {
        client["deviceMake"] = json!(make);
    }
    if let Some(model) = profile.device_model {
        client["deviceModel"] = json!(model);
    }
    if let Some(sdk) = profile.android_sdk {
        client["androidSdkVersion"] = json!(sdk);
    }
    if let Some(vd) = visitor_data {
        client["visitorData"] = json!(vd);
    }
    json!({ "client": client })
}

fn add_api_headers(
    req: reqwest::RequestBuilder,
    profile: &ClientProfile,
    visitor_data: Option<&str>,
    auth: Option<&str>,
) -> reqwest::RequestBuilder {
    let mut r = req
        .header("User-Agent", profile.user_agent)
        .header("X-YouTube-Client-Name", profile.numeric_id)
        .header("X-YouTube-Client-Version", profile.version)
        .header("X-Goog-Api-Format-Version", "2");
    if let Some(vd) = visitor_data {
        r = r.header("X-Goog-Visitor-Id", vd);
    }
    if let Some(a) = auth {
        r = r.header("Authorization", a);
    }
    if let Some(referer) = profile.referer {
        r = r.header("Referer", referer);
    }
    if let Some(origin) = profile.origin {
        r = r.header("Origin", origin);
    }
    r
}

pub async fn player_request(
    http: &reqwest::Client,
    profile: &ClientProfile,
    video_id: &str,
    visitor_data: Option<&str>,
    sig_timestamp: Option<u32>,
    auth: Option<&str>,
) -> AnyResult<PlayerResponse> {
    let context = build_context(profile, visitor_data);
    let mut payload = json!({
        "context": context,
        "videoId": video_id,
        "contentCheckOk": true,
        "racyCheckOk": true,
    });
    if let Some(sts) = sig_timestamp {
        payload["playbackContext"] = json!({
            "contentPlaybackContext": { "signatureTimestamp": sts }
        });
    }
    let url = "https://www.youtube.com/youtubei/v1/player?prettyPrint=false";
    let req = add_api_headers(http.post(url), profile, visitor_data, auth);
    let res = req.json(&payload).send().await?;
    let status = res.status();
    if !status.is_success() {
        let body = res.text().await.unwrap_or_default();
        return Err(format!("InnerTube player {} ({}): {}", status, profile.label, body).into());
    }
    Ok(res.json::<PlayerResponse>().await?)
}

pub async fn search_request(
    http: &reqwest::Client,
    profile: &ClientProfile,
    query: &str,
    params: Option<&str>,
    visitor_data: Option<&str>,
    auth: Option<&str>,
) -> AnyResult<Value> {
    let context = build_context(profile, visitor_data);
    let mut payload = json!({ "context": context, "query": query });
    if let Some(p) = params {
        payload["params"] = json!(p);
    }
    let url = if profile.client_name == "WEB_REMIX" {
        "https://music.youtube.com/youtubei/v1/search?prettyPrint=false"
    } else {
        "https://www.youtube.com/youtubei/v1/search?prettyPrint=false"
    };
    let req = add_api_headers(http.post(url), profile, visitor_data, auth);
    let res = req.json(&payload).send().await?;
    let status = res.status();
    if !status.is_success() {
        let body = res.text().await.unwrap_or_default();
        return Err(format!("InnerTube search {} ({}): {}", status, profile.label, body).into());
    }
    Ok(res.json::<Value>().await?)
}

pub async fn browse_playlist_request(
    http: &reqwest::Client,
    profile: &ClientProfile,
    playlist_id: &str,
    visitor_data: Option<&str>,
    auth: Option<&str>,
) -> AnyResult<Value> {
    let context = build_context(profile, visitor_data);
    let browse_id = format!(
        "VL{}",
        playlist_id.strip_prefix("VL").unwrap_or(playlist_id)
    );
    let payload = json!({ "context": context, "browseId": browse_id });
    let url = "https://www.youtube.com/youtubei/v1/browse?prettyPrint=false";
    let req = add_api_headers(http.post(url), profile, visitor_data, auth);
    let res = req.json(&payload).send().await?;
    let status = res.status();
    if !status.is_success() {
        let body = res.text().await.unwrap_or_default();
        return Err(format!("InnerTube browse {} ({}): {}", status, profile.label, body).into());
    }
    Ok(res.json::<Value>().await?)
}

pub async fn next_request(
    http: &reqwest::Client,
    profile: &ClientProfile,
    video_id: &str,
    playlist_id: &str,
    visitor_data: Option<&str>,
    auth: Option<&str>,
) -> AnyResult<Value> {
    let context = build_context(profile, visitor_data);
    let payload = json!({
        "context": context,
        "videoId": video_id,
        "playlistId": playlist_id,
    });
    let url = "https://www.youtube.com/youtubei/v1/next?prettyPrint=false";
    let req = add_api_headers(http.post(url), profile, visitor_data, auth);
    let res = req.json(&payload).send().await?;
    let status = res.status();
    if !status.is_success() {
        let body = res.text().await.unwrap_or_default();
        return Err(format!("InnerTube next {} ({}): {}", status, profile.label, body).into());
    }
    Ok(res.json::<Value>().await?)
}

pub fn check_playability(status: &PlayabilityStatus) -> Result<(), String> {
    if status.status == "OK" {
        return Ok(());
    }
    let reason = status.reason.as_deref().unwrap_or("unknown reason");
    match status.status.as_str() {
        "ERROR" => Err(reason.to_string()),
        "UNPLAYABLE" => {
            if reason == "unknown reason" {
                Err("This video is unplayable.".to_string())
            } else {
                Err(reason.to_string())
            }
        }
        "LOGIN_REQUIRED" => {
            if reason.contains("private") {
                Err("This is a private video.".to_string())
            } else if reason.contains("inappropriate") {
                Err("This video requires age verification.".to_string())
            } else {
                Err("This video requires login.".to_string())
            }
        }
        "CONTENT_CHECK_REQUIRED" => Err(reason.to_string()),
        "LIVE_STREAM_OFFLINE" => Err(reason.to_string()),
        _ => Err("This video cannot be viewed anonymously.".to_string()),
    }
}

pub fn best_audio_format<'a>(data: &'a crate::sources::youtube::innertube::StreamingData) -> Option<&'a Format> {
    let adaptive = data.adaptive_formats.as_deref().unwrap_or(&[]);
    let formats = data.formats.as_deref().unwrap_or(&[]);

    let audio_only: Vec<&Format> = adaptive
        .iter()
        .filter(|f| f.mime_type.starts_with("audio/"))
        .collect();

    if let Some(best) = audio_only
        .iter()
        .copied()
        .max_by_key(|f| f.bitrate)
    {
        return Some(best);
    }

    formats.iter().max_by_key(|f| f.bitrate)
}
