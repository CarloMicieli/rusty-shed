use crate::{
    catalog::domain::manufacturer::{Manufacturer, ManufacturerUowExt},
    core::domain::domain_error::DomainError,
};

/// Query to retrieve all manufacturers.
pub struct GetManufacturers;

impl GetManufacturers {
    /// Execute the query to get all manufacturers.
    ///
    /// # Arguments
    /// * `unit_of_work` - The unit of work managing the database transaction.
    ///
    /// # Returns
    /// - `Ok(Vec<Manufacturer>)` containing all manufacturers on success.
    /// - `Err(DomainError)` with an error message on failure.
    ///
    /// # Type Parameters
    /// * `U` - The type of the unit of work, which must implement `ManufacturerUowExt` and be `Send`.
    pub async fn execute<U>(unit_of_work: &mut U) -> Result<Vec<Manufacturer>, DomainError>
    where
        U: ManufacturerUowExt + Send,
    {
        let mut repository = unit_of_work.manufacturers_repo();
        repository.find_all().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::application::testing::FakeUow;
    use crate::catalog::domain::manufacturer::{
        ManufacturerId, ManufacturerStatus, MockManufacturerRepository,
    };
    use crate::core::domain::metadata::Metadata;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn it_returns_manufacturers() -> Result<(), DomainError> {
        let mut mock = MockManufacturerRepository::new();

        let manufacturer = Manufacturer {
            id: ManufacturerId::from_name("acme"),
            name: "ACME Models".to_string(),
            registered_company_name: None,
            country_code: None,
            status: ManufacturerStatus::Active,
            metadata: Metadata::default(),
            website_url: None,
        };

        mock.expect_find_all().returning(move || {
            let v = vec![manufacturer.clone()];
            Ok(v)
        });

        let mut uow = FakeUow::with_manufacturers_repo(mock);
        let result = GetManufacturers::execute(&mut uow).await?;
        assert_eq!(result.len(), 1);
        Ok(())
    }
}
