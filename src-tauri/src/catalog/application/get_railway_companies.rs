use crate::catalog::domain::railway_company::{RailwayCompany, RailwayCompanyUowExt};
use crate::core::domain::domain_error::DomainError;

/// Query to retrieve all railway companies.
pub struct GetRailwayCompanies;

impl GetRailwayCompanies {
    /// Execute the query to retrieve all railway companies.
    ///
    /// # Arguments
    /// * `unit_of_work` - The unit of work managing the database transaction.
    ///
    /// # Returns
    /// - `Ok(Vec<RailwayCompany>)` containing all railway companies on success.
    /// - `Err(DomainError)` with an error message on failure.
    ///
    /// # Type Parameters
    /// * `U` - The type of the unit of work, which must implement `RailwayCompanyUowExt` and be `Send`.
    pub async fn execute<U>(unit_of_work: &mut U) -> Result<Vec<RailwayCompany>, DomainError>
    where
        U: RailwayCompanyUowExt + Send,
    {
        let mut repository = unit_of_work.railway_companies_repo();
        repository.find_all().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::application::testing::FakeUow;
    use crate::catalog::domain::railway_company::MockRailwayCompanyRepository;
    use crate::catalog::domain::railway_company::RailwayCompanyId;
    use crate::core::domain::identifiers::Identifier;
    use crate::core::domain::metadata::Metadata;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn it_returns_railway_companies() -> Result<(), DomainError> {
        let mut mock = MockRailwayCompanyRepository::new();

        let railway_company = RailwayCompany {
            id: RailwayCompanyId::from_string_unchecked("trn:railway-company:test".to_string()),
            name: "ACME Models".to_string(),
            registered_company_name: None,
            country_code: None,
            metadata: Metadata::default(),
            period_of_activity: None,
        };

        mock.expect_find_all().returning(move || {
            let v = vec![railway_company.clone()];
            Ok(v)
        });

        let mut uow = FakeUow::with_railway_companies_repo(mock);
        let result = GetRailwayCompanies::execute(&mut uow).await?;
        assert_eq!(result.len(), 1);
        Ok(())
    }
}
