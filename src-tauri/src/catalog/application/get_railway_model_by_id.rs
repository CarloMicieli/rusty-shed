use crate::catalog::domain::railway_model::RailwayModelView;
use crate::catalog::domain::railway_model::{RailwayModel, RailwayModelId, RailwayModelUowExt};
use crate::core::domain::domain_error::DomainError;
#[allow(unused)]
use crate::core::domain::Language;

/// Query to retrieve a railway model by id.
pub struct GetRailwayModelById;

impl GetRailwayModelById {
    /// Execute the query to retrieve a railway model by id.
    ///
    /// # Arguments
    /// * `unit_of_work` - The unit of work managing the database transaction.
    /// * `railway_model_id` - The identifier of the railway model to retrieve.
    /// * `lang` - The preferred language code ("en" or "it").
    ///
    /// # Returns
    /// - `Ok(Some(RailwayModel))` when the railway model is found.
    /// - `Ok(None)` when the railway model is not found.
    /// - `Err(DomainError)` with an error message on failure.
    ///
    /// # Type Parameters
    /// * `U` - The type of the unit of work, which must implement `RailwayModelUowExt` and be `Send`.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        railway_model_id: &RailwayModelId,
        lang: &str,
    ) -> Result<Option<RailwayModel>, DomainError>
    where
        U: RailwayModelUowExt + Send,
    {
        let mut repository = unit_of_work.railway_model_repository();
        repository.find_by_id(railway_model_id, lang).await
    }
}

/// Query to retrieve a railway model view (UI) by id.
pub struct GetRailwayModelViewById;

impl GetRailwayModelViewById {
    /// Execute the query to retrieve a railway model view (UI) by id.
    ///
    /// # Arguments
    /// * `unit_of_work` - The unit of work managing the database transaction.
    /// * `railway_model_id` - The identifier of the railway model to retrieve the view for.
    /// * `lang` - The preferred language code ("en" or "it").
    ///
    /// # Returns
    /// - `Ok(Some(RailwayModelView))` when the railway model view is found.
    /// - `Ok(None)` when the railway model view is not found.
    /// - `Err(DomainError)` with an error message on failure.
    ///
    /// # Type Parameters
    /// * `U` - The type of the unit of work, which must implement `RailwayModelUowExt` and be `Send`.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        railway_model_id: &RailwayModelId,
        lang: &str,
    ) -> Result<Option<RailwayModelView>, DomainError>
    where
        U: RailwayModelUowExt + Send,
    {
        let mut repository = unit_of_work.railway_model_repository();
        repository.find_view_by_id(railway_model_id, lang).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::application::testing::FakeUow;
    use crate::catalog::domain::manufacturer::ManufacturerId;
    use crate::catalog::domain::railway_model::RailwayModelManufacturer;
    use crate::catalog::domain::railway_model::RailwayModelView;
    use crate::catalog::domain::railway_model::{
        Category, MockRailwayModelRepository, PowerMethod, ProductCode,
    };
    use crate::catalog::domain::scale::Scale;
    use crate::core::domain::identifiers::Identifier;
    use crate::core::domain::metadata::Metadata;

    #[tokio::test]
    async fn it_returns_railway_model_by_id() {
        let mut mock = MockRailwayModelRepository::new();
        let railway_model_id = RailwayModelId::try_from("trn:railway-model:model-x:1234").unwrap();
        let railway_model = RailwayModelView {
            id: railway_model_id.clone(),
            manufacturer: RailwayModelManufacturer {
                manufacturer_id: ManufacturerId::from_string_unchecked(
                    "trn:manufacturer:mn-test".to_string(),
                ),
                display: "Test Manufacturer".to_string(),
            },
            product_code: ProductCode::try_from("12345").unwrap(),
            description: "A test railway model".to_string(),
            description_lang: Language::English,
            details: None,
            details_lang: None,
            power_method: PowerMethod::DC,
            scale: Scale::H0,
            epoch: "IV".into(),
            category: Category::Locomotives,
            delivery_date: None,
            availability_status: None,
            rolling_stock: vec![],
            metadata: Metadata::default(),
        };

        mock.expect_find_view_by_id()
            .times(1)
            .returning(move |_, _| Ok(Some(railway_model.clone())));
        let mut fake_uow = FakeUow::with_railway_models_repo(mock);

        let result = GetRailwayModelViewById::execute(&mut fake_uow, &railway_model_id, "en")
            .await
            .expect("it should return");

        assert!(result.is_some());
    }
}
