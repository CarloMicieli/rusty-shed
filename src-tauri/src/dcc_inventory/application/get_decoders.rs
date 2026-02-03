use crate::core::domain::domain_error::DomainError;
use crate::dcc_inventory::domain::{DccInventoryUowExt, Decoder};

/// Use case to fetch all available decoders.
pub struct GetDecodersUseCase;

impl GetDecodersUseCase {
    /// Execute the use case to fetch all decoder master records.
    ///
    /// # Parameters
    /// - `unit_of_work`: Unit of work providing repository access required by the query.
    ///
    /// # Returns
    /// - `Ok(Vec<Decoder>)` containing all decoders on success.
    /// - `Err(DomainError)` when the repository query fails.
    ///
    /// # Type Parameters
    /// - `U`: Unit-of-work type that implements `DccInventoryUowExt` and `Send`.
    pub async fn execute<U>(unit_of_work: &mut U) -> Result<Vec<Decoder>, DomainError>
    where
        U: DccInventoryUowExt + Send,
    {
        let mut repo = unit_of_work.digital_rolling_stocks_repository();
        repo.find_all_decoders().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::domain::manufacturer::ManufacturerId;
    use crate::catalog::domain::railway_model::DccInterface;
    use crate::dcc_inventory::application::testing::FakeUow;
    use crate::dcc_inventory::domain::{
        DecoderId, DecoderType, DigitalProtocol, MockDigitalRollingStockRepository,
    };

    #[tokio::test]
    async fn it_should_return_all_decoders() {
        let mut mock = MockDigitalRollingStockRepository::new();

        let decoder = crate::dcc_inventory::domain::Decoder {
            id: DecoderId::try_from("trn:decoder:acme:d-100").unwrap(),
            manufacturer_id: ManufacturerId::try_from("trn:manufacturer:acme").unwrap(),
            product_code: "d-100".to_string(),
            decoder_type: DecoderType::Plain,
            protocol: DigitalProtocol::Dcc,
            decoder_interface: DccInterface::Nem651,
        };

        mock.expect_find_all_decoders()
            .times(1)
            .returning(move || Ok(vec![decoder.clone()]));

        let mut uow = FakeUow::new(mock);

        let result = GetDecodersUseCase::execute(&mut uow)
            .await
            .expect("query should succeed");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].product_code, "d-100");
    }
}
