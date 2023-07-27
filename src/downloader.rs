const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/115.0";
const API_BASE: &str = "https://piped-api.privacy.com.de";

use serde::Deserialize;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AudioStream {
    url: String,
    format: String,
    bitrate: i32,
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
    #[error("No M4A Streams Available")]
    StreamError,
}

async fn get_stream_url(video_id: &str) -> Result<(String, String), DownloadErrors> {
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
    let best_stream = song_info
        .audio_streams
        .iter()
        .filter(|s| s.format == "WEBMA_OPUS")
        .max_by(|s1, s2| s1.bitrate.cmp(&s2.bitrate));
    dbg!(&best_stream);
    if best_stream.is_none() {
        return Err(DownloadErrors::StreamError);
    }
    Ok((song_info.title, best_stream.unwrap().url.clone()))
}

fn santize_title(title: &String) -> String {
    // Windows not allowed chars in filename
    let not_allowed = ['/', '<', '>', ':', '"', '\\', '|', '?', '*'];
    title
        .chars()
        .map(|c| {
            if c.is_whitespace() || not_allowed.contains(&c) {
                '_'
            } else {
                c
            }
        })
        .collect::<String>()
}

async fn get_song(path: impl AsRef<Path>, url: String) -> Result<(), DownloadErrors> {
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
    let mut file = File::create(path).expect("Can create file");
    let file_content = resp.bytes().await.expect("To be able to parse as bytes");
    file.write_all(&file_content)
        .expect("To be able to write to file");
    Ok(())
}

pub async fn download_song(video_id: &str) -> Result<(), DownloadErrors> {
    let (title, url) = match get_stream_url(video_id).await {
        Err(err) => return Err(err),
        Ok(tup) => tup,
    };

    let file_path = format!("tmp/{}.webm", santize_title(&title));
    get_song(file_path, url).await?;

    Ok(())
}
