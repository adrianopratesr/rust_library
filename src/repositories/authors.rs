use crate::domain::{
    authors::{Author, CreateAuthor},
    errors::Result,
};

use super::SqlxRepository;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait AuthorRepository {
    async fn create_author(&self, author: CreateAuthor) -> Result<Author>;
}

#[async_trait::async_trait]
impl AuthorRepository for SqlxRepository {
    async fn create_author(&self, author: CreateAuthor) -> Result<Author> {
        let author = sqlx::query_as!(
            Author,
            r#"
            INSERT INTO
                authors(
                        author_id,
                        name,
                        date_of_birth
                )  VALUES ($1, $2, $3)
            returning * 
            "#,
            Uuid::new_v4(),
            author.name,
            author.date_of_birth
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(author)
    }
}
