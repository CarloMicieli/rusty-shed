use crate::collecting::domain::collection::Collection;

#[async_trait::async_trait]
pub trait CollectionRepository: Send + Sync {
    async fn get_collection(&self) -> anyhow::Result<Collection>;
}
