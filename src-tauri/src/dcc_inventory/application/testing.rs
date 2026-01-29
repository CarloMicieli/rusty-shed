use crate::dcc_inventory::domain::DccInventoryUowExt;
use crate::dcc_inventory::domain::MockDigitalRollingStockRepository;

/// Test helper unit-of-work that returns a `MockDigitalRollingStockRepository`.
pub struct FakeUow {
    pub repo: Option<MockDigitalRollingStockRepository>,
}

impl FakeUow {
    pub fn new(repo: MockDigitalRollingStockRepository) -> Self {
        Self { repo: Some(repo) }
    }
}

impl DccInventoryUowExt for FakeUow {
    fn digital_rolling_stocks_repository(
        &mut self,
    ) -> Box<dyn crate::dcc_inventory::domain::DigitalRollingStockRepository + '_> {
        Box::new(self.repo.take().expect("repo already taken"))
    }
}
