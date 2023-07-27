const LYRICS_ID_JSON: &str = r#"{
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
    },
    "context": {
        "client": {
            "clientName": "WEB_REMIX",
            "clientVersion": "1.20230727.01.00",
            "hl": "en"
        },
        "user": {}
    }
}"#;

use crate::USER_AGENT;

// struct

pub async fn get_lyrics_browse_id(video_id: &str) -> color_eyre::Result<()> {
    let client = reqwest::Client::new();
    let resp = client
        .post("https://music.youtube.com/youtubei/v1/next?alt=json")
        .body(LYRICS_ID_JSON.replace("VIDEO_ID", video_id))
        .header("User-Agent", USER_AGENT)
        .header("Content-Type", "application/json")
        .send()
        .await?;
    let text = resp.text().await?;

    let y: serde_json::Value = serde_json::from_str(&text)?;
    dbg!(y);

    Ok(())
}
