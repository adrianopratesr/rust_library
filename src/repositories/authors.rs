use crate::domain::{
    authors::{Author, CreateAuthor, UpdateAuthor},
    errors::Result,
};

use super::SqlxRepository;
use uuid::Uuid;

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait AuthorRepository {
    async fn create_author(&self, author: CreateAuthor) -> Result<Author>;
    async fn get_author(&self, author_id: Uuid) -> Result<Option<Author>>;
    async fn update_author(&self, author_id: Uuid, author: UpdateAuthor) -> Result<Option<Author>>;
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

    async fn get_author(&self, author_id: Uuid) -> Result<Option<Author>> {
        let author = sqlx::query_as!(
            Author,
            r#"
            SELECT * FROM 
                authors 
            WHERE author_id = $1 
            "#,
            author_id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(author)
    }

    async fn update_author(&self, author_id: Uuid, author: UpdateAuthor) -> Result<Option<Author>> {
        let author = sqlx::query_as!(
            Author,
            r#"
                UPDATE authors
                SET 
                name = COALESCE($1, name),
                date_of_birth = COALESCE($2, date_of_birth),
                updated_at = NOW()
                WHERE author_id = $3
                returning *
            "#,
            author.name,
            author.date_of_birth,
            author_id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(author)
    }
}
