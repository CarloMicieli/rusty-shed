use crate::collecting::domain::{CollectionRepository, CollectionUowExt, MockCollectionRepository};

#[derive(Default)]
pub struct FakeUow {
    repo: Option<MockCollectionRepository>,
}

impl FakeUow {
    pub fn new(repo: MockCollectionRepository) -> Self {
        Self { repo: Some(repo) }
    }
}

impl CollectionUowExt for FakeUow {
    fn collections_repository(&mut self) -> Box<dyn CollectionRepository + '_> {
        Box::new(self.repo.take().expect("collections repo already taken"))
    }
}
