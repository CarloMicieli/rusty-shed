#[async_trait::async_trait]
pub trait CollectionRepository: Send + Sync {
    async fn get_collection(
        &self,
    ) -> anyhow::Result<crate::collecting::domain::collection::Collection>;
}
