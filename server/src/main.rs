mod musapi;
mod musictag;
mod prestart;
mod router;

use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    prestart::prechecks()?;
    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    let app = Router::new()
        .merge(router::api_router())
        .route("/hello", get(hello))
        .layer(CorsLayer::permissive());

    let addr: SocketAddr = "[::]:3000".parse()?;
    log::info!("listening on PORT {}", &addr.port());
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn hello() -> Html<&'static str> {
    Html("<h1>Hello World!</h1>")
}

// Taken from: https://github.com/tokio-rs/axum/blob/main/examples/graceful-shutdown/src/main.rs
async fn shutdown_signal() {
    use tokio::signal;

    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    log::info!("signal received, starting graceful shutdown");
}
