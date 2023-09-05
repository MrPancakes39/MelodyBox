use once_cell::sync::Lazy;
use crate::musapi::MusicApiClient;
use axum::{extract::Query, response::Html, routing::get, Router};
use serde::Deserialize;

const CLIENT: Lazy<MusicApiClient> = Lazy::new(|| MusicApiClient::new());
#[derive(Debug, Deserialize)]
struct InfoParams {
    id: String,
}

async fn info_handler(Query(query): Query<InfoParams>) -> Html<&'static str> {
    log::info!("VideoID = {:?}", &query.id);
    Html("test")
}

pub fn api_router() -> Router {
    Router::new().route("/api/info", get(info_handler))
}
