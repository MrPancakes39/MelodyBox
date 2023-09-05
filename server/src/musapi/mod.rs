mod downloader;
mod errors;
mod fetcher;
mod structure;

const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/115.0";
const PIPED_BASE_API: &str = "https://piped-api.lunar.icu";

use reqwest::Client;

use self::downloader::*;
use self::errors::*;
use self::fetcher::*;

#[derive(Debug)]
pub struct MusicApiClient {
    client: Client,
}

impl MusicApiClient {
    pub fn new() -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        MusicApiClient {
            client: Client::builder()
                .user_agent(USER_AGENT)
                .default_headers(headers)
                .build()
                .unwrap(),
        }
    }

    pub async fn get_track_info(&self, video_id: &str) -> Result<TrackInfo, RequestorError> {
        get_track_info(&self.client, video_id).await
    }

    pub async fn get_lyrics(&self, info: &TrackInfo) -> Result<Lyrics, RequestorError> {
        get_lyrics_from_yt(&self.client, info).await
    }

    pub async fn download_song(&self, video_id: &str) -> Result<String, RequestorError> {
        download_song(&self.client, video_id).await
    }

    pub async fn validate_video_id(&self, video_id: &str) -> Result<bool, RequestorError> {
        #[derive(Debug, serde::Deserialize)]
        struct StreamResponse {
            error: Option<String>,
        }

        let url = format!("{PIPED_BASE_API}/streams/{video_id}");
        let resp = self.client.get(url).send().await?;
        let json = match resp.json::<StreamResponse>().await {
            Err(_) => return Err(RequestorError::ParseError),
            Ok(res) => res,
        };
        Ok(match json.error {
            None => true,
            Some(_) => false,
        })
    }
}
