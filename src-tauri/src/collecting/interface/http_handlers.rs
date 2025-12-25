use crate::collecting::application::get_collection::GetCollectionUseCase;
use crate::collecting::infrastructure::sqlite_repo::SqliteCollectionRepository;
use crate::db::DB_POOL;
use axum::{Json, response::IntoResponse};
use std::sync::Arc;

pub async fn get_collection_handler() -> impl IntoResponse {
    let pool = match DB_POOL.get() {
        Some(pool) => pool.clone(),
        None => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Database not initialized",
            )
                .into_response();
        }
    };

    let repo = SqliteCollectionRepository::new(pool);
    let use_case = GetCollectionUseCase::new(Arc::new(repo));

    match use_case.execute().await {
        Ok(collection) => Json(collection).into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
