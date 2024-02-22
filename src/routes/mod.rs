use std::sync::Arc;

use axum::Router;

use crate::repositories::authors::AuthorRepository;
mod authors;

pub(super) fn configure_routes() -> Router<Arc<dyn AuthorRepository + Send + Sync>> {
    authors::configure_routes()
}
