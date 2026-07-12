pub mod player;

use lavende_core;
use napi_derive::napi;

#[napi]
pub fn set_config_path(path: Option<String>) {
    lavende_core::set_config_path(path);
}

#[napi]
pub async fn resolve_jiosaavn(url_or_token: String) -> napi::Result<String> {
    lavende_core::resolve_jiosaavn(url_or_token)
        .await
        .map_err(|e| napi::Error::from_reason(e))
}

#[napi]
pub async fn resolve_youtube(url_or_id: String) -> napi::Result<String> {
    lavende_core::resolve_youtube(url_or_id)
        .await
        .map_err(|e| napi::Error::from_reason(e))
}

#[napi]
pub async fn load(identifier: String) -> napi::Result<String> {
    lavende_core::load(identifier)
        .await
        .map_err(|e| napi::Error::from_reason(e))
}

#[napi]
pub async fn load_lyrics(encoded_track: String, skip_track_source: bool) -> napi::Result<String> {
    lavende_core::load_lyrics(encoded_track, skip_track_source)
        .await
        .map_err(|e| napi::Error::from_reason(e))
}

#[napi]
pub async fn load_lyrics_by_search(title: String, artist: String) -> napi::Result<String> {
    lavende_core::load_lyrics_by_search(title, artist)
        .await
        .map_err(|e| napi::Error::from_reason(e))
}
