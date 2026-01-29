use crate::core::domain::domain_error::DomainError;
use crate::dcc_inventory::domain::{DccAddress, DccInventoryUowExt, DigitalRollingStockId};

/// Use case to change the DCC address of a DigitalRollingStock aggregate.
pub struct ChangeDccAddressUseCase;

impl ChangeDccAddressUseCase {
    /// Execute the use case to change the DCC address of a `DigitalRollingStock`.
    ///
    /// # Parameters
    /// - `unit_of_work`: Unit of work providing repository access required by the use case.
    /// - `input`: `ChangeDccAddressInput` containing the target `DigitalRollingStock` id and the new DCC address.
    ///
    /// # Returns
    /// - `Ok(())` when the change is persisted successfully.
    /// - `Err(DomainError)` when the target cannot be found or a repository error occurs.
    ///
    /// # Type Parameters
    /// - `U`: Unit-of-work type that implements `DccInventoryUowExt` and `Send`.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: ChangeDccAddressInput,
    ) -> Result<(), DomainError>
    where
        U: DccInventoryUowExt + Send,
    {
        let mut repo = unit_of_work.digital_rolling_stocks_repository();

        let maybe = repo.find_by_id(&input.id).await?;
        let mut drs = maybe.ok_or_else(|| DomainError::NotFound {
            resource: "DigitalRollingStock".to_string(),
            identifier: input.id.to_string(),
        })?;

        drs.change_dcc_address(input.new_dcc_address);

        repo.save(drs).await
    }
}

/// Input for changing DCC address
#[derive(Debug, Clone)]
pub struct ChangeDccAddressInput {
    /// The digital rolling stock id whose DCC address is to be changed
    pub id: DigitalRollingStockId,
    /// The new DCC address to assign to the digital rolling stock
    pub new_dcc_address: DccAddress,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collecting::domain::OwnedRollingStockId;
    use crate::dcc_inventory::domain::MockDigitalRollingStockRepository;
    use crate::dcc_inventory::domain::{DccAddress, DecoderId};
    use crate::dcc_inventory::domain::{DigitalRollingStock, DigitalRollingStockId};
    use uuid::Uuid;

    use crate::dcc_inventory::application::testing::FakeUow;

    #[tokio::test]
    async fn it_should_change_dcc_address_and_emit_event() {
        let mut mock = MockDigitalRollingStockRepository::new();

        let existing = {
            let id = DigitalRollingStockId::from_uuid(Uuid::new_v4());
            let owned = OwnedRollingStockId::from(Uuid::new_v4());
            let addr = DccAddress::new(10).unwrap();
            let decoder = DecoderId::try_from("trn:decoder:old:old").unwrap();
            DigitalRollingStock::new(id, owned, addr, decoder)
        };

        let new_addr = DccAddress::new(500).unwrap();
        let expected_addr = new_addr;

        mock.expect_find_by_id()
            .times(1)
            .returning(move |_| Ok(Some(existing.clone())));

        mock.expect_save()
            .times(1)
            .withf(move |drs: &DigitalRollingStock| {
                drs.dcc_address == expected_addr && drs.pending_events.len() == 1
            })
            .returning(|_| Ok(()));

        let mut uow = FakeUow::new(mock);

        let input = ChangeDccAddressInput {
            id: DigitalRollingStockId::from_uuid(Uuid::new_v4()),
            new_dcc_address: new_addr,
        };

        ChangeDccAddressUseCase::execute(&mut uow, input)
            .await
            .expect("change address should succeed");
    }

    #[tokio::test]
    async fn it_should_return_not_found_when_target_missing() {
        let mut mock = MockDigitalRollingStockRepository::new();

        mock.expect_find_by_id().times(1).returning(|_| Ok(None));

        let mut uow = FakeUow::new(mock);

        let input = ChangeDccAddressInput {
            id: DigitalRollingStockId::from_uuid(Uuid::new_v4()),
            new_dcc_address: DccAddress::new(1).unwrap(),
        };

        let res = ChangeDccAddressUseCase::execute(&mut uow, input).await;
        assert!(matches!(res, Err(DomainError::NotFound { .. })));
    }
}
