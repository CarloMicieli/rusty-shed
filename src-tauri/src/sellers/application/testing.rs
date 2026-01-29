use crate::sellers::domain::{SellersRepository, SellersUowExt};

#[derive(Default)]
pub struct FakeUow {
    sellers_repo: Option<Box<dyn SellersRepository + 'static>>,
}

impl FakeUow {
    pub fn with_sellers_repo(sellers_repo: Box<dyn SellersRepository + 'static>) -> Self {
        Self {
            sellers_repo: Some(sellers_repo),
        }
    }
}

impl SellersUowExt for FakeUow {
    fn sellers_repository(&mut self) -> Box<dyn SellersRepository + '_> {
        self.sellers_repo
            .take()
            .expect("sellers repository already taken")
    }
}
