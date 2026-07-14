use crate::{
    config::sources::GoogleTtsConfig,
    protocol::tracks::{LoadResult, Track},
    sources::{SourcePlugin, playable_track::BoxedTrack},
};
use async_trait::async_trait;
use regex::Regex;
use std::sync::Arc;

pub mod api;
pub mod extractor;

pub struct GoogleTtsSource {
    config: GoogleTtsConfig,
    search_prefixes: Vec<String>,
    url_pattern: Regex,
}

impl GoogleTtsSource {
    pub fn new(config: GoogleTtsConfig) -> Self {
        Self {
            config,
            search_prefixes: vec!["gtts:".to_string(), "speak:".to_string()],
            url_pattern: Regex::new(r"(?i)^(gtts://|speak://)").unwrap(),
        }
    }
}

#[async_trait]
impl SourcePlugin for GoogleTtsSource {
    fn name(&self) -> &str {
        "google-tts"
    }

    fn can_handle(&self, identifier: &str) -> bool {
        self.search_prefixes
            .iter()
            .any(|p| identifier.starts_with(p))
            || self.url_pattern.is_match(identifier)
    }

    async fn load(
        &self,
        identifier: &str,
        _routeplanner: Option<Arc<dyn crate::routeplanner::RoutePlanner>>,
    ) -> LoadResult {
        let (language, text) = api::parse_query(&self.search_prefixes, &self.config.language, identifier);
        if text.trim().is_empty() {
            return LoadResult::Empty {};
        }
        let url = api::build_url(&language, &text);
        let info = extractor::build_track_info(&language, &text, self.name(), &url);
        LoadResult::Track(Track::new(info))
    }

    async fn get_track(
        &self,
        identifier: &str,
        routeplanner: Option<Arc<dyn crate::routeplanner::RoutePlanner>>,
    ) -> Option<BoxedTrack> {
        let (language, text) = api::parse_query(&self.search_prefixes, &self.config.language, identifier);
        let url = api::build_url(&language, &text);
        Some(Arc::new(crate::sources::http::HttpTrack {
            url,
            local_addr: routeplanner.and_then(|rp| rp.get_address()),
            proxy: None,
        }))
    }

    fn search_prefixes(&self) -> Vec<&str> {
        self.search_prefixes.iter().map(|s| s.as_str()).collect()
    }
}
