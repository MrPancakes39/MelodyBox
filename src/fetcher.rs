const BROWSE_ID_JSON: &str = r#"{
    "enablePersistentPlaylistPanel": true,
    "isAudioOnly": true,
    "tunerSettingValue": "AUTOMIX_SETTING_NORMAL",
    "videoId": "VIDEO_ID",
    "playlistId": "RDAMVMVIDEO_ID",
    "watchEndpointMusicSupportedConfigs": {
        "watchEndpointMusicConfig": {
            "hasPersistentPlaylistPanel": true,
            "musicVideoType": "MUSIC_VIDEO_TYPE_ATV"
        }
    }, CONTEXT
}"#;

use crate::USER_AGENT;

use crate::errors::IdError;
use crate::structure::NextEndpoint;
use serde_json::Value;

fn get_context() -> String {
    use chrono::Utc;
    r#"
    "context": {
        "client": {
            "clientName": "WEB_REMIX",
            "clientVersion": "1.DATE.01.00",
            "hl": "en"
        },
        "user": {}
    }"#
    .replace("DATE", Utc::now().format("%Y%m%d").to_string().as_str())
}

async fn get_lyrics_browse_id(video_id: &str, context: &str) -> Result<Option<String>, IdError> {
    let body = BROWSE_ID_JSON
        .replace("VIDEO_ID", video_id)
        .replace("CONTEXT", context);
    let client = reqwest::Client::new();
    let resp = client
        .post("https://music.youtube.com/youtubei/v1/next?alt=json")
        .body(body)
        .header("User-Agent", USER_AGENT)
        .header("Content-Type", "application/json")
        .send()
        .await?;
    let json = match resp.json::<NextEndpoint>().await {
        Err(_) => return Err(IdError::ParseError),
        Ok(ne) => ne,
    };

    let watch_next_renderer = json
        .contents
        .single_column_music_watch_next_results_renderer
        .tabbed_renderer
        .watch_next_tabbed_results_renderer;
    let tab_renderer = &watch_next_renderer.tabs[1].tab_renderer;
    let lyrics_browse_id = match tab_renderer.unselectable {
        Some(_) => None,
        None => tab_renderer
            .endpoint
            .as_ref()
            .map(|e| e.browse_endpoint.browse_id.clone()),
    };

    Ok(lyrics_browse_id)
}

#[derive(Debug, Default)]
pub struct Lyrics {
    pub lyrics: Option<String>,
    pub source: Option<String>,
}

pub async fn get_lyrics_from_yt(video_id: &str) -> color_eyre::Result<Lyrics> {
    let context = get_context();
    let lyrics_browse_id = match get_lyrics_browse_id(video_id, &context).await? {
        None => return Ok(Default::default()),
        Some(s) => s,
    };
    let body = format!("{{\"browseId\": \"{lyrics_browse_id}\", {context}}}");
    let client = reqwest::Client::new();
    let resp = client
        .post("https://music.youtube.com/youtubei/v1/browse?alt=json")
        .body(body)
        .header("User-Agent", USER_AGENT)
        .header("Content-Type", "application/json")
        .send()
        .await?;
    let json: serde_json::Value = resp.json().await?;
    let tmp =
        &json["contents"]["sectionListRenderer"]["contents"][0]["musicDescriptionShelfRenderer"];

    let mut ret: Lyrics = Default::default();
    ret.lyrics = match &tmp["description"]["runs"][0]["text"] {
        Value::String(s) => Some(s.clone()),
        _ => None,
    };
    ret.source = match &tmp["footer"]["runs"][0]["text"] {
        Value::String(s) => Some(s.clone()),
        _ => None,
    };

    Ok(ret)
}
