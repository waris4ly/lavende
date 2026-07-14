use crate::{
    common::types::{AnyResult, AudioFormat},
    config::HttpProxyConfig,
    sources::{
        playable_track::{PlayableTrack, ResolvedTrack},
        youtube::{
            cipher::YouTubeCipherManager,
            innertube::{
                best_audio_format, check_playability, player_request, Format,
            },
            oauth::YouTubeOAuth,
            stream::create_reader,
            innertube::ClientProfile,
        },
    },
};
use async_trait::async_trait;
use std::net::IpAddr;
use std::sync::Arc;
use tracing::{debug, error, info, warn};

pub struct YoutubeTrack {
    pub identifier: String,
    pub clients: Vec<&'static ClientProfile>,
    pub oauth: Arc<YouTubeOAuth>,
    pub cipher_manager: Arc<YouTubeCipherManager>,
    pub visitor_data: Option<String>,
    pub local_addr: Option<IpAddr>,
    pub proxy: Option<HttpProxyConfig>,
    pub http: Arc<reqwest::Client>,
}

#[async_trait]
impl PlayableTrack for YoutubeTrack {
    fn supports_seek(&self) -> bool {
        true
    }

    async fn resolve(&self) -> Result<ResolvedTrack, String> {
        let mut last_error = String::from("No clients available");

        for client in &self.clients {
            let name = client.label;
            match self.resolve_url_for_client(client).await {
                Ok(Some(url)) => {
                    info!("YoutubeTrack: resolved '{}' using '{}'", self.identifier, name);
                    let is_hls = url.contains(".m3u8") || url.contains("/playlist");
                    let hint = Some(detect_audio_kind(&url, is_hls));

                    match create_reader(
                        &url,
                        client.client_name,
                        self.local_addr,
                        self.proxy.clone(),
                        self.cipher_manager.clone(),
                    )
                    .await
                    {
                        Ok(reader) => return Ok(ResolvedTrack::new(reader, hint)),
                        Err(e) => {
                            warn!("YoutubeTrack: reader failed for '{}': {} -- trying next client", name, e);
                            last_error = e.to_string();
                        }
                    }
                }
                Ok(None) => {
                    debug!("YoutubeTrack: client '{}' returned no stream URL for '{}'", name, self.identifier);
                }
                Err(e) => {
                    warn!("YoutubeTrack: client '{}' failed for '{}': {}", name, self.identifier, e);
                    last_error = e.to_string();
                }
            }
        }

        error!("YoutubeTrack: all clients failed for '{}': {}", self.identifier, last_error);
        Err(format!("All clients failed: {}", last_error))
    }
}

impl YoutubeTrack {
    async fn resolve_url_for_client(&self, client: &ClientProfile) -> AnyResult<Option<String>> {
        let sig_timestamp = self.cipher_manager.get_signature_timestamp().await.ok();
        let auth = if client.can_search || client.client_name == "ANDROID" || client.client_name == "IOS" {
            self.oauth.get_auth_header().await
        } else {
            None
        };

        let response = player_request(
            &self.http,
            client,
            &self.identifier,
            self.visitor_data.as_deref(),
            sig_timestamp,
            auth.as_deref(),
        )
        .await?;

        if let Err(e) = check_playability(&response.playability_status) {
            warn!("{} player: video {} not playable: {}", client.label, self.identifier, e);
            return Err(e.into());
        }

        let sd = match response.streaming_data {
            Some(sd) => sd,
            None => {
                error!("{} player: no streamingData for {}", client.label, self.identifier);
                return Ok(None);
            }
        };

        if let Some(hls) = sd.hls_manifest_url.as_ref() {
            debug!("{} player: using HLS manifest for {}", client.label, self.identifier);
            return Ok(Some(hls.to_string()));
        }

        if let Some(best) = best_audio_format(&sd) {
            let player_page_url = format!("https://www.youtube.com/watch?v={}", self.identifier);
            match self.resolve_format_url(best, &player_page_url).await {
                Ok(Some(url)) => return Ok(Some(url)),
                Ok(None) => {
                    warn!("{} player: best format had no resolvable URL for {}", client.label, self.identifier);
                }
                Err(e) => {
                    error!("{} player: cipher resolution failed for {}: {}", client.label, self.identifier, e);
                    return Err(e);
                }
            }
        }

        Ok(None)
    }

    async fn resolve_format_url(&self, format: &Format, player_page_url: &str) -> AnyResult<Option<String>> {
        if let Some(url) = format.url.as_ref() {
            let n_param = url
                .split("&n=")
                .nth(1)
                .or_else(|| url.split("?n=").nth(1))
                .and_then(|s| s.split('&').next());

            if n_param.is_none() {
                return Ok(Some(url.to_string()));
            }

            let resolved = self
                .cipher_manager
                .resolve_url(url, player_page_url, n_param, None)
                .await?;
            return Ok(Some(resolved));
        }

        let cipher_str = format
            .signature_cipher
            .as_ref()
            .or(format.cipher.as_ref());

        if let Some(cipher_str) = cipher_str {
            if let Some((url, sig)) = decode_signature_cipher(cipher_str) {
                let n_param = url
                    .split("&n=")
                    .nth(1)
                    .or_else(|| url.split("?n=").nth(1))
                    .and_then(|s| s.split('&').next());

                let resolved = self
                    .cipher_manager
                    .resolve_url(&url, player_page_url, n_param, Some(&sig))
                    .await?;
                return Ok(Some(resolved));
            }
        }

        Ok(None)
    }
}

fn decode_signature_cipher(cipher_str: &str) -> Option<(String, String)> {
    let mut url = None;
    let mut sig = None;
    for part in cipher_str.split('&') {
        if let Some((k, v)) = part.split_once('=') {
            let decoded = urlencoding::decode(v).ok()?.to_string();
            match k {
                "url" => url = Some(decoded),
                "s" => sig = Some(decoded),
                _ => {}
            }
        }
    }
    match (url, sig) {
        (Some(u), Some(s)) => Some((u, s)),
        _ => None,
    }
}

pub fn detect_audio_kind(url: &str, is_hls: bool) -> AudioFormat {
    if is_hls {
        AudioFormat::Aac
    } else {
        AudioFormat::from_url(url)
    }
}
