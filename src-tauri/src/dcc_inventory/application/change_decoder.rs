use crate::core::domain::domain_error::DomainError;
use crate::dcc_inventory::domain::{DccInventoryUowExt, DecoderId, DigitalRollingStockId};

/// Use case to change the decoder of a DigitalRollingStock aggregate.
pub struct ChangeDecoderUseCase;

impl ChangeDecoderUseCase {
    /// Execute the use case to change the decoder of a `DigitalRollingStock`.
    ///
    /// # Parameters
    /// - `unit_of_work`: Unit of work providing repository access required by the use case.
    /// - `input`: `ChangeDecoderInput` containing the target `DigitalRollingStock` id and the new `DecoderId`.
    ///
    /// # Returns
    /// - `Ok(())` when the change is persisted successfully.
    /// - `Err(DomainError)` when the target cannot be found or a repository error occurs.
    ///
    /// # Type Parameters
    /// - `U`: Unit-of-work type that implements `DccInventoryUowExt` and `Send`.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: ChangeDecoderInput,
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

        drs.change_decoder(input.decoder_id);

        repo.save(drs).await
    }
}

/// Input for changing a decoder
#[derive(Debug, Clone)]
pub struct ChangeDecoderInput {
    /// The digital rolling stock id whose decoder is to be changed
    pub id: DigitalRollingStockId,
    /// The new decoder id to assign to the digital rolling stock
    pub decoder_id: DecoderId,
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
    async fn it_should_change_decoder_and_emit_event() {
        let mut mock = MockDigitalRollingStockRepository::new();

        let existing = {
            let id = DigitalRollingStockId::from_uuid(Uuid::new_v4());
            let owned = OwnedRollingStockId::from(Uuid::new_v4());
            let addr = DccAddress::new(10).unwrap();
            let decoder = DecoderId::try_from("trn:decoder:old:old").unwrap();
            DigitalRollingStock::reconstitute(id, owned, addr, decoder)
        };

        let new_decoder = DecoderId::try_from("trn:decoder:new:d-1").unwrap();
        let expected_decoder = new_decoder.clone();

        mock.expect_find_by_id()
            .times(1)
            .returning(move |_| Ok(Some(existing.clone())));

        mock.expect_save()
            .times(1)
            .withf(move |drs: &DigitalRollingStock| {
                drs.decoder_id == expected_decoder && drs.pending_events.len() == 1
            })
            .returning(|_| Ok(()));

        let mut uow = FakeUow::new(mock);

        let input = ChangeDecoderInput {
            id: DigitalRollingStockId::from_uuid(Uuid::new_v4()),
            decoder_id: new_decoder.clone(),
        };

        ChangeDecoderUseCase::execute(&mut uow, input)
            .await
            .expect("change decoder should succeed");
    }

    #[tokio::test]
    async fn it_should_return_not_found_when_target_missing() {
        let mut mock = MockDigitalRollingStockRepository::new();

        mock.expect_find_by_id().times(1).returning(|_| Ok(None));

        let mut uow = FakeUow::new(mock);

        let input = ChangeDecoderInput {
            id: DigitalRollingStockId::from_uuid(Uuid::new_v4()),
            decoder_id: DecoderId::try_from("trn:decoder:doesnot:exist").unwrap(),
        };

        let res = ChangeDecoderUseCase::execute(&mut uow, input).await;
        assert!(matches!(res, Err(DomainError::NotFound { .. })));
    }
}
