use crate::protocol::tracks::TrackInfo;

pub fn build_track_info(text: &str, identifier: &str, url: &str, source_name: &str) -> TrackInfo {
    let title_text = if text.len() > 50 {
        format!("{}...", &text[..47])
    } else {
        text.to_string()
    };
    TrackInfo {
        identifier: identifier.to_string(),
        is_seekable: true,
        author: "Flowery TTS".to_string(),
        length: 0,
        is_stream: false,
        position: 0,
        title: title_text,
        uri: Some(url.to_string()),
        source_name: source_name.to_string(),
        artwork_url: None,
        isrc: None,
    }
}
