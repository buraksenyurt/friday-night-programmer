mod bindables;
mod controls;
mod dom;
mod handlers;
mod models;
mod tests;
mod utils;
mod views;

use crate::handlers::*;
use axum::Router;
use axum::routing::{get, post};
use log::info;
use tokio::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    utils::setup_log();

    let app = Router::new()
        .route("/", get(handle_create_user_view))
        .route("/create-new-user", post(handle_create_user_post));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:1903").await?;
    info!("Server is listening on: 0.0.0.0:1903");
    axum::serve(listener, app).await?;

    Ok(())
}
