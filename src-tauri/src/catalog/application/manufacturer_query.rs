use crate::catalog::domain::manufacturer::{Manufacturer, ManufacturerId, ManufacturerUowExt};
use crate::core::domain::domain_error::DomainError;

/// Query to retrieve all manufacturers from the database.
pub struct GetManufacturersQuery;

impl GetManufacturersQuery {
    /// Execute the query to get all manufacturers.
    ///
    /// # Arguments
    /// * `unit_of_work` - The unit of work managing the database transaction.
    ///
    /// # Returns
    /// - `Ok(Vec<Manufacturer>)` containing all manufacturers on success.
    /// - `Err(DomainError)` with an error message on failure.
    pub async fn execute<U>(unit_of_work: &mut U) -> Result<Vec<Manufacturer>, DomainError>
    where
        U: ManufacturerUowExt + Send,
    {
        let mut repository = unit_of_work.manufacturers_repo();
        repository.find_all().await
    }
}

/// Query to retrieve all manufacturers from the database.
pub struct GetManufacturerByIdQuery;

impl GetManufacturerByIdQuery {
    /// Execute the query to get a manufacturer by id
    ///
    /// # Arguments
    /// * `unit_of_work` - The unit of work managing the database transaction.
    /// * `manufacturer_id` - The identifier of the manufacturer to retrieve.
    ///
    /// # Returns
    /// - `Ok(Option<Manufacturer>)` containing the manufacturer on success.
    /// - `Err(DomainError)` with an error message on failure.
    pub async fn execute(
        unit_of_work: &mut dyn ManufacturerUowExt,
        manufacturer_id: ManufacturerId,
    ) -> Result<Option<Manufacturer>, DomainError> {
        let mut repository = unit_of_work.manufacturers_repo();
        repository.find_by_id(&manufacturer_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::application::testing::FakeUow;
    use crate::catalog::domain::manufacturer::{ManufacturerStatus, MockManufacturerRepository};
    use mockall::predicate::eq;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn it_returns_manufacturers() -> Result<(), DomainError> {
        let mut mock = MockManufacturerRepository::new();

        let manufacturer = Manufacturer {
            id: ManufacturerId::new("trn:manufacturer:mn-test"),
            name: "ACME Models".to_string(),
            registered_company_name: None,
            country_code: None,
            status: ManufacturerStatus::Active,
            website_url: None,
        };

        mock.expect_find_all().returning(move || {
            let v = vec![manufacturer.clone()];
            Ok(v)
        });

        let mut uow = FakeUow::with_manufacturers_repo(mock);
        let result = GetManufacturersQuery::execute(&mut uow).await?;
        assert_eq!(result.len(), 1);
        Ok(())
    }

    #[tokio::test]
    async fn it_returns_manufacturer_by_id() -> Result<(), DomainError> {
        let mut mock = MockManufacturerRepository::new();

        let id = ManufacturerId::new("trn:manufacturer:mn-test");

        let manufacturer = Manufacturer {
            id: id.clone(),
            name: "ACME Models".to_string(),
            registered_company_name: None,
            country_code: None,
            status: ManufacturerStatus::Active,
            website_url: None,
        };

        mock.expect_find_by_id()
            .with(eq(id.clone()))
            .returning(move |_| {
                let m = manufacturer.clone();
                Ok(Some(m))
            });

        let mut uow = FakeUow::with_manufacturers_repo(mock);
        let result = GetManufacturerByIdQuery::execute(&mut uow, id).await?;
        assert!(result.is_some());
        Ok(())
    }
}
