use crate::catalog::domain::railway_company::{
    RailwayCompany, RailwayCompanyId, RailwayCompanyUowExt,
};
use crate::core::domain::domain_error::DomainError;

/// Query to retrieve all railway companies from the database.
pub struct GetRailwayCompaniesQuery;

impl GetRailwayCompaniesQuery {
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

/// Query to retrieve a railway company by id from the database.
pub struct GetRailwayCompanyByIdQuery;

impl GetRailwayCompanyByIdQuery {
    /// Execute the query to get a railway company by id
    ///
    /// # Arguments
    /// * `unit_of_work` - The unit of work managing the database transaction.
    /// * `railway_company_id` - The identifier of the railway company to retrieve.
    ///
    /// # Returns
    /// - `Ok(Some(RailwayCompany))` when the railway company is found.
    /// - `Ok(None)` when the railway company is not found.
    /// - `Err(DomainError)` with an error message on failure.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        railway_company_id: RailwayCompanyId,
    ) -> Result<Option<RailwayCompany>, DomainError>
    where
        U: RailwayCompanyUowExt + Send,
    {
        let mut repository = unit_of_work.railway_companies_repo();
        repository.find_by_id(&railway_company_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::application::testing::FakeUow;
    use crate::catalog::domain::railway_company::MockRailwayCompanyRepository;
    use mockall::predicate::eq;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn it_returns_railway_companies() -> Result<(), DomainError> {
        let mut mock = MockRailwayCompanyRepository::new();

        let railway_company = RailwayCompany {
            id: RailwayCompanyId::new("trn:railway-company:test"),
            name: "ACME Models".to_string(),
            registered_company_name: None,
            country_code: None,
            period_of_activity: None,
        };

        mock.expect_find_all().returning(move || {
            let v = vec![railway_company.clone()];
            Ok(v)
        });

        let mut uow = FakeUow::with_railway_companies_repo(mock);
        let result = GetRailwayCompaniesQuery::execute(&mut uow).await?;
        assert_eq!(result.len(), 1);
        Ok(())
    }

    #[tokio::test]
    async fn it_returns_railway_company_by_id() -> Result<(), DomainError> {
        let mut mock = MockRailwayCompanyRepository::new();

        let id = RailwayCompanyId::new("trn:railway-company:test");

        let railway_company = RailwayCompany {
            id: id.clone(),
            name: "ACME Models".to_string(),
            registered_company_name: None,
            country_code: None,
            period_of_activity: None,
        };

        mock.expect_find_by_id()
            .with(eq(id.clone()))
            .returning(move |_| {
                let m = railway_company.clone();
                Ok(Some(m))
            });

        let mut uow = FakeUow::with_railway_companies_repo(mock);
        let result = GetRailwayCompanyByIdQuery::execute(&mut uow, id).await?;
        assert!(result.is_some());
        Ok(())
    }
}
