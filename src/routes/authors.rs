use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch, post},
    Json, Router,
};
use uuid::Uuid;

use crate::{
    domain::authors::{self, Author, CreateAuthor, UpdateAuthor},
    repositories::{self, authors::AuthorRepository},
};

pub(super) fn configure_routes() -> Router<Arc<dyn AuthorRepository + Send + Sync>> {
    Router::new().nest(
        "/authors",
        Router::new()
            .route("/", post(create_author))
            .route("/:author_id", get(get_author))
            .route("/:author_id", patch(update_author)),
    )
}

async fn create_author(
    State(repository): State<Arc<dyn AuthorRepository + Send + Sync>>,
    Json(author): Json<CreateAuthor>,
) -> impl IntoResponse {
    if let Ok(author) = repository.create_author(author).await {
        (StatusCode::CREATED, Json::from(Some(author)))
    } else {
        (StatusCode::UNPROCESSABLE_ENTITY, Json::from(None))
    }
}

async fn get_author(Path(author_id): Path<Uuid>) -> impl IntoResponse {
    (StatusCode::OK, format!("author_id = {author_id}"))
}

async fn update_author(
    Path(author_id): Path<Uuid>,
    Json(author): Json<UpdateAuthor>,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        format!("Update author {author:?} {}", author_id),
    )
}
