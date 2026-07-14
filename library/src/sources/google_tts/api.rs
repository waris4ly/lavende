pub fn build_url(language: &str, text: &str) -> String {
    let encoded_text = urlencoding::encode(text);
    format!(
        "https://translate.google.com/translate_tts?ie=UTF-8&q={}&tl={}&total=1&idx=0&textlen={}&client=gtx",
        encoded_text,
        language,
        text.len()
    )
}

pub fn parse_query(
    search_prefixes: &[String],
    default_language: &str,
    identifier: &str,
) -> (String, String) {
    let mut path = identifier;
    for prefix in search_prefixes {
        if path.starts_with(prefix) {
            path = path.trim_start_matches(prefix);
            break;
        }
    }
    if path.starts_with("//") {
        path = &path[2..];
    }
    if let Some(split_idx) = path.find(':') {
        let lang = &path[..split_idx];
        let actual_text = &path[split_idx + 1..];
        if !lang.is_empty() {
            return (lang.to_string(), actual_text.to_string());
        }
    }
    (default_language.to_string(), path.to_string())
}
