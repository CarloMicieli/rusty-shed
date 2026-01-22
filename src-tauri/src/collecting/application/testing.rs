use crate::catalog::domain::railway_model::RailwayModelUowExt;
use crate::catalog::domain::railway_model::{MockRailwayModelRepository, RailwayModelRepository};
use crate::collecting::domain::{CollectionRepository, CollectionUowExt, MockCollectionRepository};

#[derive(Default)]
pub struct FakeUow {
    repo: Option<MockCollectionRepository>,
    railway_repo: Option<MockRailwayModelRepository>,
}

impl FakeUow {
    pub fn new(repo: MockCollectionRepository, railway_repo: MockRailwayModelRepository) -> Self {
        Self {
            repo: Some(repo),
            railway_repo: Some(railway_repo),
        }
    }
}

impl CollectionUowExt for FakeUow {
    fn collections_repository(&mut self) -> Box<dyn CollectionRepository + '_> {
        struct BorrowedCollectionRepo<'a> {
            inner: &'a mut MockCollectionRepository,
        }

        #[async_trait::async_trait]
        impl<'a> CollectionRepository for BorrowedCollectionRepo<'a> {
            async fn find_view(
                &mut self,
            ) -> Result<
                crate::collecting::domain::CollectionView,
                crate::core::domain::domain_error::DomainError,
            > {
                self.inner.find_view().await
            }

            async fn find_by_id(
                &mut self,
                id: &crate::collecting::domain::CollectionId,
            ) -> Result<
                Option<crate::collecting::domain::Collection>,
                crate::core::domain::domain_error::DomainError,
            > {
                self.inner.find_by_id(id).await
            }

            async fn save(
                &mut self,
                collection: &mut crate::collecting::domain::Collection,
            ) -> Result<(), crate::core::domain::domain_error::DomainError> {
                self.inner.save(collection).await
            }

            async fn find_depot_view(
                &mut self,
            ) -> Result<
                crate::collecting::domain::DepotView,
                crate::core::domain::domain_error::DomainError,
            > {
                self.inner.find_depot_view().await
            }
        }

        Box::new(BorrowedCollectionRepo {
            inner: self.repo.as_mut().expect("collections repo missing"),
        })
    }
}

impl RailwayModelUowExt for FakeUow {
    fn railway_model_repository(&mut self) -> Box<dyn RailwayModelRepository + '_> {
        struct BorrowedRailwayRepo<'a> {
            inner: &'a mut MockRailwayModelRepository,
        }

        #[async_trait::async_trait]
        impl<'a> crate::catalog::domain::railway_model::RailwayModelRepository for BorrowedRailwayRepo<'a> {
            async fn find_by_id(
                &mut self,
                id: &crate::catalog::domain::railway_model::RailwayModelId,
            ) -> Result<
                Option<crate::catalog::domain::railway_model::RailwayModel>,
                crate::core::domain::domain_error::DomainError,
            > {
                self.inner.find_by_id(id).await
            }

            async fn create(
                &mut self,
                params: &crate::catalog::domain::railway_model::RailwayModelParams,
            ) -> Result<
                crate::catalog::domain::railway_model::RailwayModelId,
                crate::core::domain::domain_error::DomainError,
            > {
                self.inner.create(params).await
            }

            async fn save(
                &mut self,
                aggregate: &mut crate::catalog::domain::railway_model::RailwayModel,
            ) -> Result<(), crate::core::domain::domain_error::DomainError> {
                self.inner.save(aggregate).await
            }
        }

        Box::new(BorrowedRailwayRepo {
            inner: self
                .railway_repo
                .as_mut()
                .expect("railway models repo missing"),
        })
    }
}
