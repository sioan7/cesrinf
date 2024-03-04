use std::{error::Error, net::SocketAddr, process::exit};

use axum::{routing::get, Router};

use cesrinf_web::routes::index;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().init();

    let router = Router::new()
        .route("/", get(index))
        .nest_service("/static", ServeDir::new("static"));

    let port = std::env::var("PORT").unwrap_or_else(|_| "80".to_string());
    let port: u16 = port.parse().unwrap_or_else(|e| {
        eprintln!("Invalid port number {}; parsing failed: {}", port, e);
        exit(1);
    });
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    info!("Starting cesrinf at {}...", addr);
    let tcp_listener = TcpListener::bind(addr).await?;
    axum::serve(tcp_listener, router.into_make_service()).await?;

    Ok(())
}
