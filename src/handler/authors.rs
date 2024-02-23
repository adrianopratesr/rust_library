use uuid::Uuid;

use crate::domain::{
    authors::{Author, CreateAuthor, UpdateAuthor},
    errors::{Error, Result},
};

use super::Handler;

impl Handler {
    pub async fn create_author(&self, author: CreateAuthor) -> Result<Author> {
        self.author_repository.create_author(author).await
    }
    pub async fn get_author(&self, author_id: Uuid) -> Result<Author> {
        self.author_repository
            .get_author(author_id)
            .await?
            .ok_or(Error::AuthorNotFound(author_id))
    }
    pub async fn update_author(&self, author_id: Uuid, author: UpdateAuthor) -> Result<Author> {
        self.author_repository
            .update_author(author_id, author)
            .await?
            .ok_or(Error::AuthorNotFound(author_id))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::repositories::authors::MockAuthorRepository;

    #[tokio::test]
    async fn test_get_author() {
        let mut repository = MockAuthorRepository::new();
        let author_id = Uuid::default();

        repository.expect_get_author().returning(|_| {
            Err(Error::DatabaseError(sqlx::Error::Protocol(
                Default::default(),
            )))
        });

        let handler = Handler::new(Arc::new(repository));

        let error = handler.get_author(author_id).await.unwrap_err();

        assert!(matches!(error, Error::DatabaseError(..)));
    }
}
