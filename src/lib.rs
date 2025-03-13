pub mod config;
pub mod db;
pub mod error;
pub mod models;
pub mod routes;

use axum::{Router, routing::get};
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};

pub fn create_app(pool: PgPool) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/health", get(routes::health::health_check))
        .nest("/api", routes::news::news_routes()) // Ovo mora biti dodano
        .with_state(pool)
        .layer(cors)
}
