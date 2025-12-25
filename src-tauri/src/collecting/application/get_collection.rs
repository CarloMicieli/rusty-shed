use crate::collecting::domain::collection::{Collection, CollectionRepository};
use anyhow::Result;
use std::sync::Arc;

pub struct GetCollectionUseCase {
    repo: Arc<dyn CollectionRepository>,
}

impl GetCollectionUseCase {
    pub fn new(repo: Arc<dyn CollectionRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self) -> Result<Collection> {
        self.repo.get_collection().await
    }
}
