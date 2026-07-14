use crate::config::sources::FloweryConfig;
use std::collections::HashMap;

pub fn build_url(
    config: &FloweryConfig,
    text: &str,
    params_override: HashMap<String, String>,
) -> String {
    let mut voice = config.voice.clone();
    let mut translate = config.translate;
    let mut silence = config.silence;
    let mut speed = config.speed;
    if !config.enforce_config {
        if let Some(v) = params_override.get("voice") {
            voice = v.clone();
        }
        if let Some(t) = params_override.get("translate") {
            translate = t.parse().unwrap_or(translate);
        }
        if let Some(s) = params_override.get("silence") {
            silence = s.parse().unwrap_or(silence);
        }
        if let Some(sp) = params_override.get("speed") {
            speed = sp.parse().unwrap_or(speed);
        }
    }
    let encoded_text = urlencoding::encode(text);
    format!(
        "https://api.flowery.pw/v1/tts?voice={}&text={}&translate={}&silence={}&audio_format=mp3&speed={}",
        urlencoding::encode(&voice),
        encoded_text,
        translate,
        silence,
        speed
    )
}

pub fn parse_query(
    search_prefixes: &[String],
    identifier: &str,
) -> (String, HashMap<String, String>) {
    let mut path_and_query = identifier;
    for prefix in search_prefixes {
        if path_and_query.starts_with(prefix) {
            path_and_query = path_and_query.trim_start_matches(prefix);
            break;
        }
    }
    if path_and_query.starts_with("//") {
        path_and_query = &path_and_query[2..];
    }
    let mut params = HashMap::new();
    let text = if let Some(split_idx) = path_and_query.find('?') {
        let decoded_text = urlencoding::decode(&path_and_query[..split_idx])
            .unwrap_or_else(|_| std::borrow::Cow::Borrowed(&path_and_query[..split_idx]))
            .into_owned();
        let query_str = &path_and_query[split_idx + 1..];
        for pair in query_str.split('&') {
            if let Some(eq_idx) = pair.find('=') {
                let key = &pair[..eq_idx];
                let value = &pair[eq_idx + 1..];
                params.insert(
                    urlencoding::decode(key)
                        .unwrap_or(std::borrow::Cow::Borrowed(key))
                        .into_owned(),
                    urlencoding::decode(value)
                        .unwrap_or(std::borrow::Cow::Borrowed(value))
                        .into_owned(),
                );
            } else if !pair.is_empty() {
                params.insert(
                    urlencoding::decode(pair)
                        .unwrap_or(std::borrow::Cow::Borrowed(pair))
                        .into_owned(),
                    "".to_string(),
                );
            }
        }
        decoded_text
    } else {
        urlencoding::decode(path_and_query)
            .unwrap_or(std::borrow::Cow::Borrowed(path_and_query))
            .into_owned()
    };
    (text, params)
}
