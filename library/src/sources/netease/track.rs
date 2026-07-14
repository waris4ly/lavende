use crate::{
    config::HttpProxyConfig,
    sources::{
        http::HttpTrack,
        playable_track::{PlayableTrack, ResolvedTrack},
    },
};
use async_trait::async_trait;
use std::net::IpAddr;
use tracing::debug;

pub struct NeteaseTrack {
    pub stream_url: String,
    pub local_addr: Option<IpAddr>,
    pub proxy: Option<HttpProxyConfig>,
}

#[async_trait]
impl PlayableTrack for NeteaseTrack {
    async fn resolve(&self) -> Result<ResolvedTrack, String> {
        let url = self.stream_url.clone();
        debug!("Netease playback URL: {url}");
        HttpTrack {
            url,
            local_addr: self.local_addr,
            proxy: None,
        }
        .resolve()
        .await
    }
}
