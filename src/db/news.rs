use crate::error::ApiError;
use crate::models::news::{CreateNewsPost, NewsPost};
use sqlx::PgPool;
use sqlx::types::chrono::Utc;
pub async fn get_news(pool: &PgPool) -> Result<Vec<NewsPost>, ApiError> {
    sqlx::query_as!(
        NewsPost,
        r#"
        SELECT
            id,
            title,
            content,
            created_at as "created_at: chrono::DateTime<Utc>"
        FROM news
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(ApiError::DatabaseError)
}

pub async fn create_news(pool: &PgPool, payload: CreateNewsPost) -> Result<NewsPost, ApiError> {
    sqlx::query_as!(
        NewsPost,
        r#"
        INSERT INTO news (title, content)
        VALUES ($1, $2)
        RETURNING
            id,
            title,
            content,
            created_at as "created_at: chrono::DateTime<Utc>"
        "#,
        payload.title,
        payload.content
    )
    .fetch_one(pool)
    .await
    .map_err(ApiError::DatabaseError)
}
pub async fn get_news_by_id(pool: &PgPool, id: i32) -> Result<NewsPost, ApiError> {
    sqlx::query_as!(
        NewsPost,
        r#"
        SELECT id, title, content, created_at as "created_at: chrono::DateTime<Utc>"
        FROM news
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
    .map_err(ApiError::DatabaseError)?
    .ok_or(ApiError::NotFound)
}
