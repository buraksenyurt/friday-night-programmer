mod dom;
mod tests;
mod controls;
mod handlers;

use axum::Router;
use axum::routing::get;
use log::{info};
use tokio::io;
use crate::handlers::index_handler;

#[tokio::main]
async fn main() -> io::Result<()> {
    setup_log();

    let app = Router::new().route("/", get(index_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:1903").await?;
    info!("Server is listening on: 0.0.0.0:1903");
    axum::serve(listener, app).await?;

    Ok(())
}

fn setup_log() {
    dotenvy::dotenv().ok();
    let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".into());
    unsafe {
        std::env::set_var("RUST_LOG", &log_level);
    }
    env_logger::init();
}

