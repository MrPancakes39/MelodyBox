const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/115.0";
const API_BASE: &str = "https://piped-api.privacy.com.de";
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AudioStream {
    pub url: String,
    pub format: String,
    pub bitrate: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SongInfo {
    title: String,
    audio_streams: Vec<AudioStream>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum PipedResponse {
    Success(SongInfo),
    Error { error: String },
}

#[derive(Debug, thiserror::Error)]
pub enum DownloadErrors {
    #[error("There was an Error in doing the Request")]
    RequestError(reqwest::Error),
    #[error("Couldn't Parse JSON")]
    ParseError,
    #[error("API returned an Error")]
    ApiError(String),
}

pub async fn download_song(video_id: &str) -> Result<(), DownloadErrors> {
    let url = format!("{API_BASE}/streams/{video_id}");
    let client = reqwest::Client::new();
    let resp = match client
        .get(url)
        .header("User-Agent", USER_AGENT)
        .send()
        .await
    {
        Err(err) => return Err(DownloadErrors::RequestError(err)),
        Ok(res) => res,
    };
    let json = match resp.json::<PipedResponse>().await {
        Err(_) => return Err(DownloadErrors::ParseError),
        Ok(res) => res,
    };
    let song_info = match json {
        PipedResponse::Error { error } => return Err(DownloadErrors::ApiError(error)),
        PipedResponse::Success(info) => info,
    };
    println!("{:#?}", song_info);
    Ok(())
}
