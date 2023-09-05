use once_cell::sync::Lazy;
use reqwest::StatusCode;

use crate::musapi::MusicApiClient;
use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};

const CLIENT: Lazy<MusicApiClient> = Lazy::new(|| MusicApiClient::new());

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

pub fn api_router() -> Router {
    Router::new()
        .route("/api/info/:sid", get(info_handler))
        .route("/api/lyrics/:lid", get(lyrics_handler))
}
