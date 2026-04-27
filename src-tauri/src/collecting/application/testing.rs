use crate::catalog::domain::railway_model::RailwayModelUowExt;
use crate::catalog::domain::railway_model::{MockRailwayModelRepository, RailwayModelRepository};
use crate::collecting::domain::{CollectionRepository, CollectionUowExt, MockCollectionRepository};
use std::collections::VecDeque;

#[derive(Default)]
pub struct FakeUow {
    repos: VecDeque<MockCollectionRepository>,
    railway_repos: VecDeque<MockRailwayModelRepository>,
}
impl FakeUow {
    pub fn new(repo: MockCollectionRepository, railway_repo: MockRailwayModelRepository) -> Self {
        Self::default()
            .with_collection_repo(repo)
            .with_railway_repo(railway_repo)
    }

    pub fn with_collection_repo(mut self, repo: MockCollectionRepository) -> Self {
        self.repos.push_back(repo);
        self
    }

    pub fn with_railway_repo(mut self, railway_repo: MockRailwayModelRepository) -> Self {
        self.railway_repos.push_back(railway_repo);
        self
    }
}

impl CollectionUowExt for FakeUow {
    fn collections_repository(&mut self) -> Box<dyn CollectionRepository + '_> {
        Box::new(
            self.repos
                .pop_front()
                .expect("collections repo queue exhausted"),
        )
    }
}

impl RailwayModelUowExt for FakeUow {
    fn railway_model_repository(&mut self) -> Box<dyn RailwayModelRepository + '_> {
        Box::new(
            self.railway_repos
                .pop_front()
                .expect("railway repo queue exhausted"),
        )
    }
}
