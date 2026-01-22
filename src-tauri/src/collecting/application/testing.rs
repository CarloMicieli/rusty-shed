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
        Box::new(self.repo.take().expect("collections repo already taken"))
    }
}

impl RailwayModelUowExt for FakeUow {
    fn railway_model_repository(&mut self) -> Box<dyn RailwayModelRepository + '_> {
        Box::new(
            self.railway_repo
                .take()
                .expect("railway repo already taken"),
        )
    }
}
