use once_cell::sync::Lazy;
use crate::musapi::MusicApiClient;
use axum::{extract::Query, response::Html, routing::get, Router};
use serde::Deserialize;

const CLIENT: Lazy<MusicApiClient> = Lazy::new(|| MusicApiClient::new());
#[derive(Debug, Deserialize)]
struct InfoParams {
    #[allow(non_snake_case)]
    videoId: String,
}

async fn info_handler(Query(query): Query<InfoParams>) -> Html<&'static str> {
    log::info!("VideoID = {:?}", &query.videoId);
    Html("test")
}

pub fn api_router() -> Router {
    Router::new().route("/info", get(info_handler))
}
