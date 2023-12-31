use once_cell::sync::Lazy;
use reqwest::StatusCode;

use axum::{
    extract::Path,
    http::header,
    response::{AppendHeaders, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::musapi::MusicApiClient;
use crate::musictag::{create_tag_info, write_tags};

static CLIENT: Lazy<MusicApiClient> = Lazy::new(MusicApiClient::new);

async fn info_handler(Path(sid): Path<String>) -> impl IntoResponse {
    log::debug!("SongID = {:?}", &sid);
    match CLIENT.validate_video_id(&sid).await {
        Err(err) => return err.into_response(),
        Ok(false) => {
            return (StatusCode::BAD_REQUEST, "The id provided is not valid.").into_response()
        }
        _ => {}
    };
    let info = match CLIENT.get_track_info(&sid).await {
        Err(err) => return err.into_response(),
        Ok(info) => info,
    };
    (StatusCode::OK, Json(info)).into_response()
}

async fn lyrics_handler(Path(lid): Path<String>) -> impl IntoResponse {
    log::debug!("LyricsID = {:?}", &lid);
    let lyrics = match CLIENT.get_lyrics(&lid).await {
        Err(err) => return err.into_response(),
        Ok(lyrics) => lyrics,
    };
    (StatusCode::OK, Json(lyrics)).into_response()
}

#[derive(Debug, serde::Deserialize)]
struct GroupedInfo {
    info: crate::musapi::TrackInfo,
    lyrics: crate::musapi::Lyrics,
}

async fn download_handler(
    Path(sid): Path<String>,
    Json(body): Json<GroupedInfo>,
) -> impl IntoResponse {
    log::debug!("SongID = {:?}", &sid);
    match CLIENT.validate_video_id(&sid).await {
        Err(err) => return err.into_response(),
        Ok(false) => {
            return (StatusCode::BAD_REQUEST, "The id provided is not valid.").into_response()
        }
        _ => {}
    };

    let (file_path, org_name) = match CLIENT.download_song(&sid, true).await {
        Err(err) => return err.into_response(),
        Ok(path) => path,
    };

    let cover_img = match CLIENT.download_cover(&body.info.thumbnail).await {
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Couldn't download cover image for song",
            )
                .into_response()
        }
        Ok(data) => data,
    };

    let tag = create_tag_info(
        body.info.title,
        body.info.artists.join(", "),
        body.info.album,
        body.lyrics.lyrics.map(|text| ("eng", text)),
        Some(cover_img),
        Some(format!(
            "This song was downloaded from: https://music.youtube.com/watch?v={sid}."
        )),
    );

    if let Err(e) = write_tags(&file_path, tag) {
        log::error!("{:?}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let content_disp = format!("attachment; filename=\"{org_name}\"");
    let headers = AppendHeaders([
        (header::CONTENT_TYPE, "audio/mpeg"),
        (header::CONTENT_DISPOSITION, &content_disp),
    ]);

    let mut f = File::open(file_path).await.unwrap();
    let mut content = Vec::<u8>::new();
    f.read_to_end(&mut content).await.unwrap();

    (headers, content).into_response()
}

pub fn api_router() -> Router {
    Router::new()
        .route("/api/info/:sid", get(info_handler))
        .route("/api/lyrics/:lid", get(lyrics_handler))
        .route("/api/download/:sid", post(download_handler))
}
