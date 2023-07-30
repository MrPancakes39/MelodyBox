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

use serde_json::Value;

use crate::USER_AGENT;

use crate::errors::InfoError;
use crate::structure::{NextEndpoint, PlaylistPanelVideoRenderer, TrackRun};

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

#[derive(Debug, Default)]
pub struct TrackInfo {
    pub video_id: String,
    pub title: String,
    pub duration: Option<String>,
    pub duration_seconds: Option<i32>,
    pub thumbnail: String,
    pub artists: Vec<String>,
    pub album: Option<String>,
    pub year: Option<i32>,
    pub lyrics_id: Option<String>,
}

fn parse_duration(duration: &str) -> Option<i32> {
    let vec = duration
        .split(':')
        .map(|n| n.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()
        .ok()?;
    if vec.len() > 3 {
        panic!("Duration vector has more than 3 splits")
    }
    Some(
        vec.iter()
            .rev()
            .zip([1, 60, 3600])
            .map(|(m, n)| m * n)
            .sum::<i32>(),
    )
}

fn parse_song_runs(ti: &mut TrackInfo, runs: &[TrackRun]) {
    // uneven items are always separators
    for run in runs.iter().step_by(2) {
        let text = &run.text;
        // artist or album
        if let Some(nav) = &run.navigation_endpoint {
            // if this panics then it's my fault
            let id = &nav.browse_endpoint.browse_id;
            if id.starts_with("MPRE") || id.contains("release_detail") {
                ti.album = Some(text.clone());
            } else {
                ti.artists.push(text.clone())
            }
        } else {
            // text is a year
            if run.text.len() == 4 && run.text.chars().all(char::is_numeric) {
                ti.year = text.parse::<i32>().ok();
            // duration skip
            } else if run.text.contains(':') {
                continue; // if length is None this is most likely None
            } else {
                // views: start number alphanum space alphanum end
                let views_pattern = text.len() > 3
                    && text.starts_with(char::is_numeric)
                    && text.chars().filter(|&c| c == ' ').count() == 1
                    && !text.ends_with(' ');
                // text is views
                if views_pattern {
                    continue;
                }
                // text is artist
                ti.artists.push(text.clone());
            }
        }
    }
}

fn parse_watch_track(track: &PlaylistPanelVideoRenderer) -> TrackInfo {
    let mut tmp = TrackInfo {
        video_id: track.video_id.clone(),
        title: track.title.runs[0].text.clone(),
        duration: track.length_text.as_ref().map(|l| l.runs[0].text.clone()),
        ..Default::default()
    };
    tmp.duration_seconds = tmp.duration.as_ref().and_then(|d| parse_duration(d));
    tmp.thumbnail = track
        .thumbnail
        .thumbnails
        .iter()
        .max_by(|x, y| x.width.cmp(&y.width))
        .map(|thumb| thumb.url.clone())
        .unwrap_or_default();
    parse_song_runs(&mut tmp, &track.long_byline_text.runs);
    tmp
}

pub async fn get_track_info(video_id: &str) -> Result<TrackInfo, InfoError> {
    let body = BROWSE_ID_JSON
        .replace("VIDEO_ID", video_id)
        .replace("CONTEXT", get_context().as_str());
    let client = reqwest::Client::new();
    let resp = client
        .post("https://music.youtube.com/youtubei/v1/next?alt=json")
        .body(body)
        .header("User-Agent", USER_AGENT)
        .header("Content-Type", "application/json")
        .send()
        .await?;

    let json = match resp.json::<NextEndpoint>().await {
        Err(_) => return Err(InfoError::ParseError),
        Ok(ne) => ne,
    };

    let watch_next_renderer = json
        .contents
        .single_column_music_watch_next_results_renderer
        .tabbed_renderer
        .watch_next_tabbed_results_renderer;

    let results = match &watch_next_renderer.tabs[0].tab_renderer.content {
        None => panic!("Shouldn't Reach Here"),
        Some(content) => &content.music_queue_renderer.content.playlist_panel_renderer,
    };

    let playlist_renderer = match results.contents[0]
        .playlist_panel_video_wrapper_renderer
        .as_ref()
    {
        Some(ppvwr) => &ppvwr.primary_renderer.playlist_panel_video_renderer,
        None => &results.contents[0].playlist_panel_video_renderer,
    };

    let mut track_info = match playlist_renderer {
        None => panic!("Couldn't find VideoRenderer for Track!"),
        Some(renderer) => parse_watch_track(renderer),
    };

    let tab_renderer = &watch_next_renderer.tabs[1].tab_renderer;
    track_info.lyrics_id = match tab_renderer.unselectable {
        Some(_) => None,
        None => tab_renderer
            .endpoint
            .as_ref()
            .map(|e| e.browse_endpoint.browse_id.clone()),
    };

    Ok(track_info)
}

#[derive(Debug, Default)]
pub struct Lyrics {
    pub lyrics: Option<String>,
    pub source: Option<String>,
}

pub async fn get_lyrics_from_yt(info: &TrackInfo) -> color_eyre::Result<Lyrics> {
    let lyrics_browse_id = match &info.lyrics_id {
        None => return Ok(Default::default()),
        Some(s) => s,
    };

    let body = format!(
        "{{\"browseId\": \"{lyrics_browse_id}\", {}}}",
        get_context()
    );
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
