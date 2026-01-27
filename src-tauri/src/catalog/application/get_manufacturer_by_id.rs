use crate::catalog::domain::manufacturer::{Manufacturer, ManufacturerId, ManufacturerUowExt};
use crate::core::domain::domain_error::DomainError;

/// Query to retrieve a manufacturer by id.
pub struct GetManufacturerById;

impl GetManufacturerById {
    /// Execute the query to get a manufacturer by id
    ///
    /// # Arguments
    /// * `unit_of_work` - The unit of work managing the database transaction.
    /// * `manufacturer_id` - The identifier of the manufacturer to retrieve.
    ///
    /// # Returns
    /// - `Ok(Option<Manufacturer>)` containing the manufacturer on success.
    /// - `Err(DomainError)` with an error message on failure.
    ///
    /// # Type Parameters
    /// * `U` - The type of the unit of work, which must implement `ManufacturerUowExt` and be `Send`.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        manufacturer_id: ManufacturerId,
    ) -> Result<Option<Manufacturer>, DomainError>
    where
        U: ManufacturerUowExt + Send,
    {
        let mut repository = unit_of_work.manufacturers_repo();
        repository.find_by_id(&manufacturer_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::application::testing::FakeUow;
    use crate::catalog::domain::manufacturer::{ManufacturerStatus, MockManufacturerRepository};
    use crate::core::domain::identifiers::Identifier;
    use crate::core::domain::metadata::Metadata;
    use mockall::predicate::eq;

    #[tokio::test]
    async fn it_returns_manufacturer_by_id() -> Result<(), DomainError> {
        let mut mock = MockManufacturerRepository::new();

        let id = ManufacturerId::new_from_parts(&["test"]);

        let manufacturer = Manufacturer {
            id: id.clone(),
            name: "ACME Models".to_string(),
            registered_company_name: None,
            country_code: None,
            status: ManufacturerStatus::Active,
            metadata: Metadata::default(),
            website_url: None,
        };

        mock.expect_find_by_id()
            .with(eq(id.clone()))
            .returning(move |_| {
                let m = manufacturer.clone();
                Ok(Some(m))
            });

        let mut uow = FakeUow::with_manufacturers_repo(mock);
        let result = GetManufacturerById::execute(&mut uow, id).await?;
        assert!(result.is_some());
        Ok(())
    }
}
