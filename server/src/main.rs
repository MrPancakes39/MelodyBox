mod musapi;
mod prestart;

use axum::{routing::get, Router};
use std::net::SocketAddr;

use musapi::MusicApiClient;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    prestart::prechecks()?;

    let client = MusicApiClient::new();
    println!("{:?}", client);
    // let info = client.get_track_info("FJX0JPXD2nM").await?;
    // dbg!(&info);
    // let lyrics = client.get_lyrics(&info).await?;
    // dbg!(&lyrics);
    // let path = client.download_song(&info.video_id).await?;
    // println!("Path: {path:#}");

    let app = Router::new().route("/", get(|| async { "Hello, world!" }));

    let addr: SocketAddr = "[::]:3000".parse().unwrap();
    println!("listening on {}", &addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
