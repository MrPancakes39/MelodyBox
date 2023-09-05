// mod musapi;
mod prestart;

use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    prestart::prechecks()?;

    let app = Router::new().route("/hello", get(hello));

    let addr: SocketAddr = "[::]:3000".parse()?;
    println!("listening on PORT {}", &addr.port());
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn hello() -> Html<&'static str> {
    Html("<h1>Hello World!</h1>")
}
