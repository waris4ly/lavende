use crate::protocol::tracks::{Track, TrackInfo};
use serde_json::Value;

pub fn build_artwork_url(json: &Value) -> Option<String> {
    let art_id = json["coverArt"]
        .as_str()
        .or_else(|| json["AlbumArt"].as_str())
        .or_else(|| json["cover"].as_str())
        .filter(|s| !s.is_empty())?;
    Some(format!(
        "https://artwork.anghcdn.co/?id={}&size=640",
        art_id
    ))
}

pub fn parse_track(json: &Value) -> Option<Track> {
    let id = json["id"]
        .as_str()
        .map(|s| s.to_owned())
        .or_else(|| json["id"].as_i64().map(|n| n.to_string()))
        .filter(|s| !s.is_empty())?;
    let title = json["title"]
        .as_str()
        .or_else(|| json["name"].as_str())
        .filter(|s| !s.is_empty())?
        .to_owned();
    let author = json["artist"]
        .as_str()
        .or_else(|| json["artistName"].as_str())
        .unwrap_or("Unknown Artist")
        .to_owned();
    let duration_secs = json["duration"]
        .as_f64()
        .or_else(|| {
            json["duration"]
                .as_str()
                .and_then(|s| s.parse::<f64>().ok())
        })
        .unwrap_or(0.0);
    let length = (duration_secs * 1000.0).round() as u64;
    let artwork_url = build_artwork_url(json);
    let uri = format!("https://play.anghami.com/song/{}", id);
    Some(Track::new(TrackInfo {
        identifier: id,
        is_seekable: true,
        author,
        length,
        is_stream: false,
        position: 0,
        title,
        uri: Some(uri),
        artwork_url,
        isrc: None,
        source_name: "anghami".to_owned(),
    }))
}

pub fn collection_title(body: &Value, type_hint: &str, default: &str) -> String {
    let mut candidates = vec![
        &body["title"],
        &body["name"],
        &body["playlist_name"],
        &body["album_name"],
        &body["albumTitle"],
        &body["playlistTitle"],
        &body["album_info"]["title"],
        &body["playlist_info"]["title"],
    ];
    for t in &["album", "playlist", type_hint] {
        candidates.push(&body[*t]["title"]);
        candidates.push(&body[*t]["name"]);
        candidates.push(&body[*t]["album_name"]);
        candidates.push(&body[*t]["playlist_name"]);
        candidates.push(&body[*t]["albumTitle"]);
        candidates.push(&body[*t]["playlistTitle"]);
        candidates.push(&body[*t]["_attributes"]["title"]);
        candidates.push(&body[*t]["_attributes"]["name"]);
    }
    candidates.push(&body["_attributes"]["title"]);
    candidates.push(&body["_attributes"]["name"]);
    if let Some(title) = candidates
        .into_iter()
        .find_map(|v| v.as_str().filter(|s| !s.is_empty()))
    {
        return title.to_owned();
    }
    if let Some(sections) = body["sections"].as_array() {
        for sec in sections {
            if let Some(t) = sec["title"]
                .as_str()
                .or_else(|| sec["name"].as_str())
                .filter(|s| !s.is_empty())
            {
                return t.to_owned();
            }
        }
    }
    default.to_owned()
}
