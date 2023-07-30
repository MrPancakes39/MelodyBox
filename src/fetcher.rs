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

pub fn get_context() -> String {
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

fn parse_duration(duration: Option<&String>) -> Option<i32> {
    let duration = duration?;
    let vec = duration
        .split(':')
        .map(|n| n.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()
        .ok()?;
    if vec.len() > 3 {
        return None;
    }
    let secs = vec
        .iter()
        .rev()
        .zip([1, 60, 3600])
        .map(|(m, n)| m * n)
        .sum::<i32>();
    Some(secs)
}

fn parse_watch_track(data: &Value) {
    let video_id = &data["videoId"];
    let title = &data["title"]["runs"][0]["text"];
    let length = &data["lengthText"]["runs"][0]["text"];
    let thumbnails = &data["thumbnail"]["thumbnails"];
    let video_type = &data["navigationEndpoint"]["watchEndpoint"]
        ["watchEndpointMusicSupportedConfigs"]["watchEndpointMusicConfig"]["musicVideoType"];
    let runs = match &data["longBylineText"]["runs"] {
        Value::Array(a) => a,
        _ => panic!("Err"),
    };

    #[derive(Debug)]
    struct Artist {
        name: String,
        id: Option<String>,
    }

    let mut artists = Vec::<Artist>::new();
    let mut album = None;
    let mut duration = None;
    let mut duration_seconds = None;
    let mut year = None;
    for run in runs.iter().step_by(2) {
        let text = match &run["text"] {
            Value::String(s) => s,
            _ => panic!("Err: text"),
        };
        // dbg!(text);
        if let Some(nav) = run.get("navigationEndpoint") {
            let id_val = &run["navigationEndpoint"]["browseEndpoint"]["browseId"];
            let id = match id_val {
                Value::String(s) => Some(s),
                _ => None,
            };
            if id.is_some()
                && (id.unwrap().starts_with("MPRE") || id_val.get("release_detail").is_some())
            {
                album = Some(text.clone());
            } else {
                artists.push(Artist {
                    name: text.clone(),
                    id: id.cloned(),
                });
            }
        } else {
            // \d\d\d\d
            if text.len() == 4 && text.chars().all(char::is_numeric) {
                year = text.parse::<i32>().ok()
            } else if text.contains(':') {
                duration = Some(text.clone());
                duration_seconds = parse_duration(duration.as_ref());
            } else {
                let views_pattern = text.len() > 3
                    && text.chars().next().unwrap().is_numeric()
                    && text.chars().filter(|c| c == &' ').count() == 1
                    && text.chars().last().unwrap() != ' ';
                if !views_pattern {
                    artists.push(Artist {
                        name: text.clone(),
                        id: None,
                    });
                }
            }
        }
    }

    dbg!(album);
    dbg!(artists);
    dbg!(duration);
    dbg!(duration_seconds);
    dbg!(length);
    dbg!(thumbnails);
    dbg!(title);
    dbg!(video_id);
    dbg!(video_type);
    dbg!(year);
}

pub async fn get_track_info(video_id: &str, context: &str) -> Result<Option<String>, IdError> {
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
    type X = serde_json::Value;
    // type X = NextEndpoint;
    let json = match resp.json::<X>().await {
        Err(_) => return Err(IdError::ParseError),
        Ok(ne) => ne,
    };

    // dbg!(&json["contents"]);
    let watch_next_renderer = &json["contents"]["singleColumnMusicWatchNextResultsRenderer"]
        ["tabbedRenderer"]["watchNextTabbedResultsRenderer"];

    // dbg!(watch_next_renderer);

    let results = &watch_next_renderer["tabs"][0]["tabRenderer"]["content"]["musicQueueRenderer"]
        ["content"]["playlistPanelRenderer"];
    // dbg!(results);
    let mut result = &results["contents"][0];
    if let Some(ppvwr) = result.get("playlistPanelVideoWrapperRenderer") {
        result = &ppvwr["primaryRenderer"];
    }
    let track = result
        .get("playlistPanelVideoRenderer")
        .map(|data| parse_watch_track(data));

    // let watch_next_renderer = json
    //     .contents
    //     .single_column_music_watch_next_results_renderer
    //     .tabbed_renderer
    //     .watch_next_tabbed_results_renderer;
    // let tab_renderer = &watch_next_renderer.tabs[1].tab_renderer;
    // let lyrics_browse_id = match tab_renderer.unselectable {
    //     Some(_) => None,
    //     None => tab_renderer
    //         .endpoint
    //         .as_ref()
    //         .map(|e| e.browse_endpoint.browse_id.clone()),
    // };

    // Ok(lyrics_browse_id)
    Ok(None)
}

// #[derive(Debug, Default)]
// pub struct Lyrics {
//     pub lyrics: Option<String>,
//     pub source: Option<String>,
// }

// pub async fn get_lyrics_from_yt(video_id: &str) -> color_eyre::Result<Lyrics> {
//     let context = get_context();
//     let lyrics_browse_id = match get_lyrics_browse_id(video_id, &context).await? {
//         None => return Ok(Default::default()),
//         Some(s) => s,
//     };
//     let body = format!("{{\"browseId\": \"{lyrics_browse_id}\", {context}}}");
//     let client = reqwest::Client::new();
//     let resp = client
//         .post("https://music.youtube.com/youtubei/v1/browse?alt=json")
//         .body(body)
//         .header("User-Agent", USER_AGENT)
//         .header("Content-Type", "application/json")
//         .send()
//         .await?;
//     let json: serde_json::Value = resp.json().await?;
//     let tmp =
//         &json["contents"]["sectionListRenderer"]["contents"][0]["musicDescriptionShelfRenderer"];

//     let mut ret: Lyrics = Default::default();
//     ret.lyrics = match &tmp["description"]["runs"][0]["text"] {
//         Value::String(s) => Some(s.clone()),
//         _ => None,
//     };
//     ret.source = match &tmp["footer"]["runs"][0]["text"] {
//         Value::String(s) => Some(s.clone()),
//         _ => None,
//     };

//     Ok(ret)
// }
