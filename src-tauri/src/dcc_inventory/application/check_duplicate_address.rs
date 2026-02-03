use crate::core::domain::domain_error::DomainError;
use crate::dcc_inventory::application::CheckDuplicateAddressResult;
use crate::dcc_inventory::domain::{DccAddress, DccInventoryUowExt, DigitalRollingStockId};

/// Use case to check if a DCC address is already in use.
pub struct CheckDuplicateAddressUseCase;

impl CheckDuplicateAddressUseCase {
    /// Execute the use case to check for duplicate DCC addresses.
    ///
    /// # Parameters
    /// - `unit_of_work`: Unit of work providing repository access required by the query.
    /// - `address`: The DCC address to check.
    /// - `exclude_id`: Optional ID to exclude from the check (for edit scenarios).
    ///
    /// # Returns
    /// - `Ok(CheckDuplicateAddressResult)` with duplicate information on success.
    /// - `Err(DomainError)` when the repository query fails.
    ///
    /// # Type Parameters
    /// - `U`: Unit-of-work type that implements `DccInventoryUowExt` and `Send`.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        address: DccAddress,
        exclude_id: Option<DigitalRollingStockId>,
    ) -> Result<CheckDuplicateAddressResult, DomainError>
    where
        U: DccInventoryUowExt + Send,
    {
        let mut repo = unit_of_work.digital_rolling_stocks_repository();
        repo.check_address_exists(address, exclude_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dcc_inventory::application::CheckDuplicateAddressResult;
    use crate::dcc_inventory::application::testing::FakeUow;
    use crate::dcc_inventory::domain::MockDigitalRollingStockRepository;
    use uuid::Uuid;

    #[tokio::test]
    async fn it_should_detect_duplicate_address() {
        let mut mock = MockDigitalRollingStockRepository::new();
        let address = DccAddress::new(42).unwrap();
        let existing_id = DigitalRollingStockId::from_uuid(Uuid::new_v4());

        let result = CheckDuplicateAddressResult {
            is_duplicate: true,
            existing_rolling_stock_id: Some(existing_id.clone()),
        };

        mock.expect_check_address_exists()
            .withf(move |addr, exclude| *addr == address && exclude.is_none())
            .times(1)
            .returning(move |_, _| Ok(result.clone()));

        let mut uow = FakeUow::new(mock);

        let res = CheckDuplicateAddressUseCase::execute(&mut uow, address, None)
            .await
            .expect("query should succeed");

        assert!(res.is_duplicate);
        assert_eq!(res.existing_rolling_stock_id, Some(existing_id));
    }

    #[tokio::test]
    async fn it_should_not_detect_duplicate_when_excluding_self() {
        let mut mock = MockDigitalRollingStockRepository::new();
        let address = DccAddress::new(42).unwrap();
        let self_id = DigitalRollingStockId::from_uuid(Uuid::new_v4());

        let result = CheckDuplicateAddressResult {
            is_duplicate: false,
            existing_rolling_stock_id: None,
        };

        mock.expect_check_address_exists()
            .times(1)
            .returning(move |_, _| Ok(result.clone()));

        let mut uow = FakeUow::new(mock);

        let res = CheckDuplicateAddressUseCase::execute(&mut uow, address, Some(self_id.clone()))
            .await
            .expect("query should succeed");

        assert!(!res.is_duplicate);
    }
}
