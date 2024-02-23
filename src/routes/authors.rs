use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, patch, post},
    Json, Router,
};
use uuid::Uuid;

use crate::{
    domain::{
        authors::{CreateAuthor, UpdateAuthor},
        errors::Result,
    },
    handler::Handler,
};

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().nest(
        "/authors",
        Router::new()
            .route("/", post(create_author))
            .route("/:author_id", get(get_author))
            .route("/:author_id", patch(update_author)),
    )
}

async fn create_author(
    State(handler): State<Handler>,
    Json(author): Json<CreateAuthor>,
) -> Result<impl IntoResponse> {
    let author = handler.create_author(author).await?;

    Ok(Json(author))
}

async fn get_author(
    State(handler): State<Handler>,
    Path(author_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let author = handler.get_author(author_id).await?;

    Ok(Json(author))
}

async fn update_author(
    State(handler): State<Handler>,
    Path(author_id): Path<Uuid>,
    Json(author): Json<UpdateAuthor>,
) -> Result<impl IntoResponse> {
    let author = handler.update_author(author_id, author).await?;
    Ok(Json(author))
}
