use once_cell::sync::Lazy;
use reqwest::StatusCode;

use crate::musapi::MusicApiClient;
use axum::response::IntoResponse;
use axum::{extract::Path, routing::get, Json, Router};

const CLIENT: Lazy<MusicApiClient> = Lazy::new(|| MusicApiClient::new());

async fn info_handler(Path(id): Path<String>) -> impl IntoResponse {
    log::debug!("VideoID = {:?}", &id);
    match CLIENT.validate_video_id(&id).await {
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        Ok(false) => {
            return (StatusCode::BAD_REQUEST, "The id provided is not valid.").into_response()
        }
        Ok(true) => {}
    };
    let info = match CLIENT.get_track_info(&id).await {
        Err(err) => {
            log::error!("ERROR: FAILED TO FETCH INFO {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch info from Youtube API",
            )
                .into_response();
        }
        Ok(info) => info,
    };
    (StatusCode::OK, Json(info)).into_response()
}

pub fn api_router() -> Router {
    Router::new().route("/api/info/:id", get(info_handler))
}
