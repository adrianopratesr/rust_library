use axum::Router;
mod authors;

pub(super) fn configure_routes() -> Router {
    authors::configure_routes()
}
