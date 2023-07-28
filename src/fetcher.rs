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
            "clientVersion": "1.DATE.01.00",
            "hl": "en"
        },
        "user": {}
    }
}"#;

use serde_json::Value;

use crate::USER_AGENT;

fn get_body(video_id: &str) -> String {
    use chrono::Utc;
    LYRICS_ID_JSON
        .replace("VIDEO_ID", video_id)
        .replace("DATE", Utc::now().format("%Y%m%d").to_string().as_str())
}

fn nav(json: Value, array: &[&str]) -> Option<Value> {
    let mut item = &json;
    for k in array {
        item = item.get(k)?;
    }
    Some(item.clone())
}

fn get_tab_browse_id(watch_next_renderer: Value, tab_id: usize) -> Option<String> {
    let tmp = &watch_next_renderer["tabs"][tab_id]["tabRenderer"];
    if tmp.get("unselectable").is_none() {
        Some(serde_json::from_value(tmp["endpoint"]["browseEndpoint"]["browseId"].clone()).unwrap())
    } else {
        None
    }
}

async fn json_way(resp: reqwest::Response) -> color_eyre::Result<()> {
    let text = resp.text().await?;

    let y: serde_json::Value = serde_json::from_str(&text)?;
    // dbg!(&y);
    let watch_next_renderer = nav(
        y,
        &[
            "contents",
            "singleColumnMusicWatchNextResultsRenderer",
            "tabbedRenderer",
            "watchNextTabbedResultsRenderer",
        ],
    );
    // dbg!(&watch_next_renderer);
    // let lyrics_browse_id = get_tab_browse_id(watch_next_renderer.unwrap(), 1);
    // dbg!(&lyrics_browse_id);

    Ok(())
}

use crate::structure::NextEndpoint;

async fn serde_way(resp: reqwest::Response) -> Option<String> {
    let json = resp.json::<NextEndpoint>().await.ok()?;
    let watch = json
        .contents
        .single_column_music_watch_next_results_renderer
        .tabbed_renderer
        .watch_next_tabbed_results_renderer;
    let tab = watch.tabs[1].clone();
    Some(tab.tab_renderer.endpoint?.browse_endpoint.browse_id)
}

pub async fn get_lyrics_browse_id(video_id: &str) -> color_eyre::Result<()> {
    let body = get_body(video_id);
    let client = reqwest::Client::new();
    let resp = client
        .post("https://music.youtube.com/youtubei/v1/next?alt=json")
        .body(body)
        .header("User-Agent", USER_AGENT)
        .header("Content-Type", "application/json")
        .send()
        .await?;

    Ok(())
}
