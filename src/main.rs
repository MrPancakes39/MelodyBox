const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/115.0";
const API_BASE: &str = "https://piped-api.privacy.com.de";

use color_eyre::Result;
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

async fn download_song(video_id: &str) -> Result<()> {
    let url = format!("{API_BASE}/streams/{video_id}");
    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .header("User-Agent", USER_AGENT)
        .send()
        .await?;
    let json = resp.json::<PipedResponse>().await?;
    println!("{:#?}", json);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    download_song("HoBGWhapaho").await?;
    Ok(())
}
