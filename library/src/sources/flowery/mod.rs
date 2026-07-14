use crate::{
    config::sources::FloweryConfig,
    protocol::tracks::{LoadResult, Track},
    sources::{SourcePlugin, playable_track::BoxedTrack},
};
use async_trait::async_trait;
use regex::Regex;
use std::sync::Arc;

pub mod api;
pub mod extractor;

pub struct FlowerySource {
    config: FloweryConfig,
    search_prefixes: Vec<String>,
    url_pattern: Regex,
}

impl FlowerySource {
    pub fn new(config: FloweryConfig) -> Self {
        Self {
            config,
            search_prefixes: vec!["ftts:".to_string()],
            url_pattern: Regex::new(r"(?i)^ftts://").unwrap(),
        }
    }
}

#[async_trait]
impl SourcePlugin for FlowerySource {
    fn name(&self) -> &str {
        "flowery"
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
        let (text, params) = api::parse_query(&self.search_prefixes, identifier);
        if text.trim().is_empty() {
            return LoadResult::Empty {};
        }
        let url = api::build_url(&self.config, &text, params);
        let info = extractor::build_track_info(&text, identifier, &url, self.name());
        LoadResult::Track(Track::new(info))
    }

    async fn get_track(
        &self,
        identifier: &str,
        routeplanner: Option<Arc<dyn crate::routeplanner::RoutePlanner>>,
    ) -> Option<BoxedTrack> {
        let (text, params) = api::parse_query(&self.search_prefixes, identifier);
        let url = api::build_url(&self.config, &text, params);
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
