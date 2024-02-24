use std::{error::Error, net::SocketAddr};

use axum::{routing::get, Router};

use cesrinfo_web::routes::index;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().init();

    let router = Router::new()
        .route("/", get(index))
        .nest_service("/static", ServeDir::new("static"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3007));

    info!("Starting cesrinfo at {}...", addr);
    let tcp_listener = TcpListener::bind(addr).await?;
    axum::serve(tcp_listener, router.into_make_service()).await?;

    Ok(())
}
