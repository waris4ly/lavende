use crate::protocol::tracks::TrackInfo;

pub fn build_track_info(
    language: &str,
    text: &str,
    source_name: &str,
    api_url: &str,
) -> TrackInfo {
    let title_text = if text.len() > 50 {
        format!("{}...", &text[..47])
    } else {
        text.to_string()
    };
    TrackInfo {
        identifier: format!("gtts://{}:{}", language, text),
        is_seekable: true,
        author: "Google TTS".to_string(),
        length: 0,
        is_stream: false,
        position: 0,
        title: format!("TTS: {}", title_text),
        uri: Some(api_url.to_string()),
        source_name: source_name.to_string(),
        artwork_url: None,
        isrc: None,
    }
}
