use crate::{
    config::AppConfig,
    protocol::tracks::{LoadResult, PlaylistData, PlaylistInfo, Track},
    sources::{SourcePlugin, playable_track::BoxedTrack},
};
use async_trait::async_trait;
use regex::Regex;
use serde_json::{Value, json};
use std::sync::Arc;

pub mod api;
pub mod extractor;
pub mod reader;

pub struct AnghamiSource {
    client: Arc<reqwest::Client>,
    udid: String,
    search_limit: usize,
    url_regex: Regex,
}

impl AnghamiSource {
    pub fn new(config: &AppConfig, client: Arc<reqwest::Client>) -> Result<Self, String> {
        let ag_config = config.sources.anghami.clone().unwrap_or_default();
        let udid = uuid::Uuid::new_v4().simple().to_string();
        Ok(Self {
            client,
            udid,
            search_limit: ag_config.search_limit,
            url_regex: Regex::new(
                r"^https?://(?:play\.|www\.)?anghami\.com/(?P<type>song|album|playlist|artist)/(?P<id>[0-9]+)",
            )
            .unwrap(),
        })
    }

    async fn api_request(&self, params: Vec<(&str, &str)>) -> Option<Value> {
        api::api_request(&self.client, &self.udid, params).await
    }

    fn parse_track(&self, json: &Value) -> Option<Track> {
        extractor::parse_track(json)
    }

    fn extract_tracks(&self, body: &Value) -> Vec<Track> {
        if let Some(songbuffers) = body["songbuffers"].as_array() {
            let mut song_map = std::collections::HashMap::new();
            for buffer_base64 in songbuffers {
                if let Some(s) = buffer_base64.as_str() {
                    if let Ok(decoded) = base64::Engine::decode(&base64::prelude::BASE64_STANDARD, s.as_bytes()) {
                        let songs = reader::decode_song_batch(&decoded);
                        for (id, track_info) in songs {
                            song_map.insert(id, Track::new(track_info));
                        }
                    }
                }
            }
            if !song_map.is_empty() {
                if let Some(order) = self.get_song_order(body) {
                    let mut order_tracks = Vec::new();
                    for id in order.split(',') {
                        if let Some(track) = song_map.remove(id.trim()) {
                            order_tracks.push(track);
                        }
                    }
                    if !order_tracks.is_empty() {
                        return order_tracks;
                    }
                }
                return song_map.into_values().collect();
            }
        }
        if let Some(sections) = body["sections"].as_array() {
            for section in sections {
                let type_ = section["type"].as_str().unwrap_or("");
                let group = section["group"].as_str().unwrap_or("");
                if type_ == "song" || group == "songs" || group == "album_songs" {
                    if let Some(data) = section["data"].as_array() {
                        let tracks: Vec<Track> =
                            data.iter().filter_map(|s| self.parse_track(s)).collect();
                        if !tracks.is_empty() {
                            return tracks;
                        }
                    }
                }
            }
        }
        for path in &["songs", "playlist/songs", "album/songs"] {
            let parts: Vec<&str> = path.split('/').collect();
            let mut current = body;
            for part in parts {
                current = &current[part];
            }
            if current.is_null() {
                continue;
            }
            let mut song_map = std::collections::HashMap::new();
            if let Some(obj) = current.as_object() {
                for v in obj.values() {
                    let s = if !v["_attributes"].is_null() {
                        &v["_attributes"]
                    } else {
                        v
                    };
                    if let Some(track) = self.parse_track(s) {
                        song_map.insert(track.info.identifier.clone(), track);
                    }
                }
            }
            if song_map.is_empty() {
                continue;
            }
            if let Some(order) = self.get_song_order(body) {
                let mut order_tracks = Vec::new();
                for id in order.split(',') {
                    if let Some(track) = song_map.remove(id.trim()) {
                        order_tracks.push(track);
                    }
                }
                if !order_tracks.is_empty() {
                    return order_tracks;
                }
            } else {
                return song_map.into_values().collect();
            }
        }
        body["data"]
            .as_array()
            .map(|data| data.iter().filter_map(|s| self.parse_track(s)).collect())
            .unwrap_or_default()
    }

    fn get_song_order<'a>(&self, body: &'a Value) -> Option<&'a str> {
        [
            &body["songorder"],
            &body["_attributes"]["songorder"],
            &body["playlist"]["songorder"],
            &body["album"]["songorder"],
        ]
        .iter()
        .find_map(|v| v.as_str())
    }

    async fn get_search(&self, query: &str) -> LoadResult {
        if query.is_empty() {
            return LoadResult::Empty {};
        }
        let body = match self
            .api_request(vec![
                ("type", "GETtabsearch"),
                ("query", query),
                ("web2", "true"),
                ("language", "en"),
                ("output", "json"),
            ])
            .await
        {
            Some(b) => b,
            None => return LoadResult::Empty {},
        };
        let tracks: Vec<Track> = body["sections"]
            .as_array()
            .and_then(|secs| {
                secs.iter().find(|s| {
                    s["type"].as_str() == Some("genericitem")
                        && s["group"].as_str() == Some("songs")
                })
            })
            .and_then(|s| s["data"].as_array())
            .map(|data| {
                data.iter()
                    .take(self.search_limit)
                    .filter_map(|item| self.parse_track(item))
                    .collect()
            })
            .unwrap_or_default();
        if tracks.is_empty() {
            LoadResult::Empty {}
        } else {
            LoadResult::Search(tracks)
        }
    }

    async fn get_song(&self, id: &str) -> LoadResult {
        if let Some(body) = self
            .api_request(vec![
                ("type", "GETsongdata"),
                ("songId", id),
                ("output", "jsonhp"),
            ])
            .await
        {
            if body["status"].as_str() == Some("ok") {
                if let Some(track) = self.parse_track(&body) {
                    return LoadResult::Track(track);
                }
            }
        }
        if let Some(body) = self
            .api_request(vec![
                ("type", "GETtabsearch"),
                ("query", id),
                ("web2", "true"),
                ("language", "en"),
                ("output", "json"),
            ])
            .await
        {
            if let Some(sections) = body["sections"].as_array() {
                for section in sections {
                    if let Some(data) = section["data"].as_array() {
                        let song = data.iter().find(|s| {
                            s["id"].as_str() == Some(id)
                                || s["id"].as_i64().map(|n| n.to_string()).as_deref() == Some(id)
                        });
                        if let Some(s) = song {
                            if let Some(track) = self.parse_track(s) {
                                return LoadResult::Track(track);
                            }
                        }
                    }
                }
            }
        }
        LoadResult::Empty {}
    }

    async fn get_album(&self, id: &str) -> LoadResult {
        for buffered in &[false, true] {
            let mut params = vec![
                ("type", "GETalbumdata"),
                ("albumId", id),
                ("web2", "true"),
                ("language", "en"),
                ("output", "json"),
            ];
            if *buffered {
                params.push(("buffered", "1"));
            }
            let body = match self.api_request(params).await {
                Some(b) if b["error"].is_null() => b,
                _ => continue,
            };
            let tracks = self.extract_tracks(&body);
            if tracks.is_empty() {
                continue;
            }
            let mut name = extractor::collection_title(&body, "album", "Unknown Album");
            if name == "Unknown Album" {
                if let Some(first_album) = body["data"]
                    .as_array()
                    .and_then(|a| a.first())
                    .and_then(|t| t["album"].as_str().or_else(|| t["albumName"].as_str()))
                {
                    name = first_album.to_owned();
                } else if let Some(sections) = body["sections"].as_array() {
                    for sec in sections {
                        if let Some(first_album) = sec["data"]
                            .as_array()
                            .and_then(|a| a.first())
                            .and_then(|t| {
                                t["album"].as_str().or_else(|| t["albumName"].as_str())
                            })
                        {
                            name = first_album.to_owned();
                            break;
                        }
                    }
                }
            }
            let artwork_url = tracks.first().and_then(|t| t.info.artwork_url.clone());
            let author = tracks.first().map(|t| t.info.author.clone());
            return LoadResult::Playlist(PlaylistData {
                info: PlaylistInfo {
                    name,
                    selected_track: -1,
                },
                plugin_info: json!({
                  "type": "album",
                  "url": format!("https://play.anghami.com/album/{}", id),
                  "artworkUrl": artwork_url,
                  "author": author,
                  "totalTracks": tracks.len()
                }),
                tracks,
            });
        }
        LoadResult::Empty {}
    }

    async fn get_playlist(&self, id: &str) -> LoadResult {
        for buffered in &[false, true] {
            let mut params = vec![
                ("type", "GETplaylistdata"),
                ("playlistId", id),
                ("web2", "true"),
                ("language", "en"),
                ("output", "json"),
            ];
            if *buffered {
                params.push(("buffered", "1"));
            }
            let body = match self.api_request(params).await {
                Some(b) if b["error"].is_null() => b,
                _ => continue,
            };
            let tracks = self.extract_tracks(&body);
            if tracks.is_empty() {
                continue;
            }
            let mut name = extractor::collection_title(&body, "playlist", "Unknown Playlist");
            if name == "Unknown Playlist" {
                if let Some(alt_name) = body["playlist"]["name"]
                    .as_str()
                    .or_else(|| body["playlist"]["title"].as_str())
                {
                    name = alt_name.to_owned();
                }
            }
            let artwork_url = tracks.first().and_then(|t| t.info.artwork_url.clone());
            return LoadResult::Playlist(PlaylistData {
                info: PlaylistInfo {
                    name,
                    selected_track: -1,
                },
                plugin_info: json!({
                  "type": "playlist",
                  "url": format!("https://play.anghami.com/playlist/{}", id),
                  "artworkUrl": artwork_url,
                  "totalTracks": tracks.len()
                }),
                tracks,
            });
        }
        LoadResult::Empty {}
    }

    async fn get_artist(&self, id: &str) -> LoadResult {
        let body = match self
            .api_request(vec![
                ("type", "GETartistprofile"),
                ("artistId", id),
                ("web2", "true"),
                ("language", "en"),
                ("output", "json"),
            ])
            .await
        {
            Some(b) => b,
            None => return LoadResult::Empty {},
        };
        let tracks: Vec<Track> = if let Some(sections) = body["sections"].as_array() {
            sections
                .iter()
                .find(|s| {
                    s["group"].as_str() == Some("songs") || s["type"].as_str() == Some("song")
                })
                .and_then(|s| s["data"].as_array())
                .map(|data| data.iter().filter_map(|s| self.parse_track(s)).collect())
                .unwrap_or_default()
        } else {
            body["data"]
                .as_array()
                .map(|data| data.iter().filter_map(|s| self.parse_track(s)).collect())
                .unwrap_or_default()
        };
        if tracks.is_empty() {
            return LoadResult::Empty {};
        }
        let name = body["name"]
            .as_str()
            .or_else(|| body["title"].as_str())
            .unwrap_or("Unknown Artist")
            .to_owned();
        let artwork_url = tracks.first().and_then(|t| t.info.artwork_url.clone());
        LoadResult::Playlist(PlaylistData {
            info: PlaylistInfo {
                name: format!("{}'s Top Tracks", name),
                selected_track: -1,
            },
            plugin_info: json!({
              "type": "artist",
              "url": format!("https://play.anghami.com/artist/{}", id),
              "artworkUrl": artwork_url,
              "author": name,
              "totalTracks": tracks.len()
            }),
            tracks,
        })
    }
}

#[async_trait]
impl SourcePlugin for AnghamiSource {
    fn name(&self) -> &str {
        "anghami"
    }

    fn can_handle(&self, identifier: &str) -> bool {
        self.search_prefixes()
            .into_iter()
            .any(|p| identifier.starts_with(p))
            || self.url_regex.is_match(identifier)
    }

    fn search_prefixes(&self) -> Vec<&str> {
        vec!["agsearch:"]
    }

    fn is_mirror(&self) -> bool {
        true
    }

    async fn load(
        &self,
        identifier: &str,
        _routeplanner: Option<Arc<dyn crate::routeplanner::RoutePlanner>>,
    ) -> LoadResult {
        if let Some(prefix) = self
            .search_prefixes()
            .into_iter()
            .find(|p| identifier.starts_with(p))
        {
            return self.get_search(&identifier[prefix.len()..]).await;
        }
        if let Some(caps) = self.url_regex.captures(identifier) {
            let type_ = caps.name("type").map(|m| m.as_str()).unwrap_or("");
            let id = caps.name("id").map(|m| m.as_str()).unwrap_or("");
            return match type_ {
                "song" => self.get_song(id).await,
                "album" => self.get_album(id).await,
                "playlist" => self.get_playlist(id).await,
                "artist" => self.get_artist(id).await,
                _ => LoadResult::Empty {},
            };
        }
        LoadResult::Empty {}
    }

    async fn get_track(
        &self,
        _identifier: &str,
        _routeplanner: Option<Arc<dyn crate::routeplanner::RoutePlanner>>,
    ) -> Option<BoxedTrack> {
        None
    }
}
