const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/115.0";

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Stream {
    pub url: String,
    pub format: String,
    pub quality: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JSONRes {
    audio_streams: Vec<Stream>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://piped-api.privacy.com.de/streams/HoBGWhapaho";
    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .header("User-Agent", USER_AGENT)
        .send()
        .await?;
    let json = resp.json::<JSONRes>().await?;
    let best_audio = json
        .audio_streams
        .iter()
        .filter(|stream| stream.format == "M4A")
        .max_by(|s1, s2| {
            let n1 = s1
                .quality
                .split(" ")
                .next()
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let n2 = s2
                .quality
                .split(" ")
                .next()
                .unwrap()
                .parse::<i32>()
                .unwrap();
            n1.cmp(&n2)
        });
    let best_url = &best_audio.unwrap().url;
    println!("{:#?}", best_url);
    Ok(())
}
