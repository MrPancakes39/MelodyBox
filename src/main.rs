const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/115.0";
const API_BASE: &str = "https://piped-api.privacy.com.de";

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AudioStream {
    pub url: String,
    pub format: String,
    pub quality: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PipedResponse {
    audio_streams: Vec<AudioStream>,
}

async fn download_song(video_id: &str) -> Result<(), Box<dyn std::error::Error>> {
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
async fn main() {
    download_song("HoBGWhapaho").await.unwrap();
}
