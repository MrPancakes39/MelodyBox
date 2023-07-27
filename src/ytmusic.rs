const YTM_BASE_API: &str = "https://music.youtube.com/youtubei/v1";

use crate::USER_AGENT;

use reqwest::header::HeaderMap;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
struct YtmConfig {
    has_persistent_playlist_panel: bool,
    music_video_type: String,
}

#[derive(Debug, Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
struct YtmWatchConfig {
    watch_endpoint_music_config: YtmConfig,
}

type YtmContext = HashMap<String, HashMap<String, String>>;

#[derive(Debug, Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
struct YtmBody {
    enable_persistent_playlist_panel: bool,
    is_audio_only: bool,
    tuner_setting_value: String,
    video_id: String,
    playlist_id: String,
    watch_endpoint_music_supported_configs: YtmWatchConfig,
    context: YtmContext,
}

#[derive(Debug)]
struct YTMusic {
    body: YtmBody,
    headers: HeaderMap,
}

impl Default for YTMusic {
    fn default() -> Self {
        let mut body = YtmBody {
            enable_persistent_playlist_panel: true,
            is_audio_only: true,
            tuner_setting_value: String::from("AUTOMIX_SETTING_NORMAL"),
            watch_endpoint_music_supported_configs: YtmWatchConfig {
                watch_endpoint_music_config: YtmConfig {
                    has_persistent_playlist_panel: true,
                    music_video_type: String::from("MUSIC_VIDEO_TYPE_ATV"),
                },
            },
            ..Default::default()
        };
        init_context(&mut body.context);

        let mut headers = HeaderMap::new();
        headers.insert("User-Agent", USER_AGENT.parse().unwrap());
        headers.insert("Accept", "*/*".parse().unwrap());
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("Content-Encoding", "gzip".parse().unwrap());
        headers.insert("Origin", "https://music.youtube.com".parse().unwrap());

        Self { body, headers }
    }
}

// "context": {
//   "user": {},
//   "client": {
//     "clientName": "WEB_REMIX",
//     "clientVersion": "1." + "%Y%m%d" + '.01.00'
//     "hl": "en"
//   }
// }
fn init_context(context: &mut YtmContext) {
    use chrono::Utc;
    context.insert("user".to_string(), HashMap::new());
    let mut tmp = HashMap::new();
    tmp.insert("clientName".to_string(), "WEB_REMIX".to_string());
    tmp.insert(
        "clientVersion".to_string(),
        format!("1.{}.01.00", Utc::now().format("%Y%m%d").to_string()),
    );
    tmp.insert("hl".to_string(), "en".to_string());
    context.insert("client".to_string(), tmp);
}

// async fn get_visitor_id(yt: &YTMusic) -> color_eyre::Result<String> {
//     // response = self._session.get(url,
//     // params=params,
//     // headers=self.headers,
//     // proxies=self.proxies,
//     // cookies=self.cookies)
//     // return response
//     // response = request_func(YTM_DOMAIN) // "https://music.youtube.com"
//     // matches = re.findall(r'ytcfg\.set\s*\(\s*({.+?})\s*\)\s*;', response.text)
//     // visitor_id = ""
//     // if len(matches) > 0:
//     // ytcfg = json.loads(matches[0])
//     // visitor_id = ytcfg.get('VISITOR_DATA')
//     // return {'X-Goog-Visitor-Id': visitor_id
//     let client = reqwest::Client::new();
//     let resp = client.get("https://music.youtube.com").send().await?;
//     let text = resp.text().await?;
//     dbg!(text);

//     Ok(String::new())
// }

// impl YTMusic {
//     async fn new() -> color_eyre::Result<Self> {
//         let mut tmp = YTMusic::default();
//         // let visitor_id = get_visitor_id(&tmp);
//         // tmp.headers
//         // .insert("X-Goog-Visitor-Id", visitor_id.parse().unwrap());
//         Ok(tmp)
//     }
// }

pub async fn ytfunc() -> color_eyre::Result<()> {
    // let mut x = YTMusic::default();
    // x.body.video_id = "HoBGWhapaho".to_string();
    // x.body.playlist_id = "RDAMVMHoBGWhapaho".to_string();
    // dbg!(&x);

    // let y = serde_json::to_string_pretty(&x.body)?;
    // println!("{}", y);
    // dbg!(&y);

    let y = r#"{
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
    let z = y.replace("VIDEO_ID", "HoBGWhapaho");
    println!("{}", z);

    let client = reqwest::Client::new();
    let resp = client
        .post("https://music.youtube.com/youtubei/v1/next?alt=json")
        .body(z)
        .header("User-Agent", USER_AGENT)
        .header("Content-Type", "application/json")
        // .json()
        // .headers(x.headers)
        .send()
        .await?;

    let h = resp.text().await?;
    dbg!(h);
    // let m = resp.json::<serde_json::Value>().await?;
    // let text = resp.bytes().await?;
    // let mut f = std::fs::File::create("./tmp.dat")?;
    // use std::io::Write;
    // f.write_all(&text)?;

    // dbg!();
    // dbg!(m);

    Ok(())
}
