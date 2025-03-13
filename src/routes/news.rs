use crate::{
    db,
    error::ApiError,
    models::news::{CreateNewsPost, NewsPost},
};
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post},
};
use sqlx::PgPool;

pub fn news_routes() -> Router<PgPool> {
    Router::new()
        .route("/news", get(get_news))
        .route("/news", post(create_news)) // Dodajte POST rutu
        .route("/news/{id}", get(get_news_by_id))
}

// Postojeća GET funkcija
async fn get_news(State(pool): State<PgPool>) -> Result<Json<Vec<NewsPost>>, ApiError> {
    let news = db::news::get_news(&pool).await?;
    Ok(Json(news))
}
async fn get_news_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<NewsPost>, ApiError> {
    let news = db::news::get_news_by_id(&pool, id).await?;
    Ok(Json(news))
}
// Nova POST funkcija
async fn create_news(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateNewsPost>,
) -> Result<Json<NewsPost>, ApiError> {
    let news = db::news::create_news(&pool, payload).await?;
    Ok(Json(news))
}
