mod error;
mod handlers;
mod models;

use std::net::SocketAddr;

use axum::{
    routing::{get, patch},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "todo_api=debug,tower_http=debug".to_string()),
        )
        .init();

    let database_url = std::env::var("DATABASE_URL").expect("Missing DATABASE_URL");

    tracing::info!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    tracing::info!("Executing migrations...");
    sqlx::migrate!("./migrations").run(&pool).await?;

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/todos/completed", get(handlers::get_completed_todos))
        .route("/api/todos/incomplete", get(handlers::get_incomplete_todos))
        .route("/api/todos/overdue", patch(handlers::update_overdue_todos))
        .route(
            "/api/todos",
            get(handlers::get_all_todos).post(handlers::create_todo),
        )
        .route(
            "/api/todos/{id}",
            get(handlers::get_todo_by_id)
                .put(handlers::update_todo)
                .delete(handlers::delete_todo),
        )
        .with_state(pool)
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Starting web server: http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
