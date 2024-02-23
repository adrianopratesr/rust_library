use std::sync::Arc;
pub mod authors;
use crate::repositories::authors::AuthorRepository;

#[derive(Clone)]
pub struct Handler {
    author_repository: Arc<dyn AuthorRepository + Send + Sync>,
}

impl Handler {
    pub const fn new(author_repository: Arc<dyn AuthorRepository + Send + Sync>) -> Self {
        Self { author_repository }
    }
}
