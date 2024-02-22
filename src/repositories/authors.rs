use crate::domain::{
    authors::{Author, CreateAuthor},
    errors::Result,
};

#[async_trait::async_trait]
pub trait AuthorRepository {
    async fn create_author(&self, author: CreateAuthor) -> Result<Author>;
}
