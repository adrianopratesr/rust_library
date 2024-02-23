use axum::{http::StatusCode, response::IntoResponse, Router};

use crate::{domain::errors::Error, handler::Handler};
mod authors;

pub(super) fn configure_routes() -> Router<Handler> {
    authors::configure_routes()
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::AuthorNotFound => (StatusCode::NOT_FOUND, "Author not found".to_string()),
            Self::DatabaseError(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{err:?}")),
        }
        .into_response()
    }
}
