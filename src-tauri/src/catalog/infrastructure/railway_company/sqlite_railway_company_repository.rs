use crate::catalog::domain::railway_company::{
    RailwayCompany, RailwayCompanyId, RailwayCompanyRepository, RailwayCompanyUowExt,
};
use crate::catalog::infrastructure::entities::RailwayCompanyRow;
use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use sqlx::SqliteConnection;

/// An SQLite-specific implementation of the `RailwayCompanyRepository`.
///
/// It holds a mutable reference to a connection, which in this architecture
/// is provided by the `SqliteUnitOfWork`'s active transaction.
pub struct SqliteRailwayCompanyRepository<'conn> {
    /// A mutable reference to the database connection/executor.
    executor: &'conn mut SqliteConnection,
}

impl<'conn> SqliteRailwayCompanyRepository<'conn> {
    /// Creates a new instance of the `SqliteRailwayCompanyRepository`.
    ///
    /// # Arguments
    /// * `executor` - A mutable reference to the database connection/executor.
    ///
    /// # Returns
    /// A new `SqliteRailwayCompanyRepository` instance.
    pub fn new(executor: &'conn mut SqliteConnection) -> Self {
        Self { executor }
    }
}

#[async_trait::async_trait]
impl<'conn> RailwayCompanyRepository for SqliteRailwayCompanyRepository<'conn> {
    /// Retrieves all Railway company from the database.
    ///
    /// # Arguments
    /// * none
    ///
    /// # Returns
    /// - Returns the list of all [`RailwayCompany`]s in the database.
    /// - Returns a [`DomainError::DatabaseError`] if the query fails.
    async fn find_all(&mut self) -> Result<Vec<RailwayCompany>, DomainError> {
        let sql = r#"
            SELECT id, name, registered_company_name, country_code, status, operating_since, 
                   operating_until, created_at, updated_at 
            FROM railway_companies
            ORDER BY name"#;

        let row = sqlx::query_as::<_, RailwayCompanyRow>(sql)
            .fetch_all(&mut *self.executor)
            .await?;

        row.into_iter()
            .map(RailwayCompany::try_from)
            .collect::<Result<Vec<RailwayCompany>, DomainError>>()
    }

    /// Fetch a railway company row by its ID.
    ///
    /// This function executes a simple SELECT query against the `railway_companies` table
    /// and returns the matching `RailwayCompanyRow` if present.
    ///
    /// # Arguments
    /// * `id` - The railway company identifier to look up.
    ///
    /// # Returns
    /// - `Ok(Some(RailwayCompanyRow))` when a matching row is found
    /// - `Ok(None)` when no row matches the provided `id`
    /// - `Err(DomainError)` if the query fails.
    async fn find_by_id(
        &mut self,
        id: &RailwayCompanyId,
    ) -> Result<Option<RailwayCompany>, DomainError> {
        let sql = r#"
            SELECT id, name, registered_company_name, country_code, status, operating_since, 
                operating_until, created_at, updated_at 
            FROM railway_companies 
            WHERE id = ?1 
            LIMIT 1"#;

        let row = sqlx::query_as::<_, RailwayCompanyRow>(sql)
            .bind(id)
            .fetch_optional(&mut *self.executor)
            .await?;

        match row {
            Some(r) => Ok(Some(RailwayCompany::try_from(r)?)),
            None => Ok(None),
        }
    }
}

impl<'conn> RailwayCompanyUowExt for SqliteUnitOfWork<'conn> {
    /// Links the SQLite-specific repository to the Unit of Work.
    ///
    /// It re-borrows the internal transaction (`&mut *self.tx`) to provide
    /// the repository with a mutable executor without transferring ownership.
    fn railway_companies_repo(&mut self) -> Box<dyn RailwayCompanyRepository + '_> {
        Box::new(SqliteRailwayCompanyRepository::new(&mut self.tx))
    }
}

#[cfg(test)]
mod railway_repo_tests {
    // use super::*;
    // use crate::catalog::domain::railway_company::RailwayCompanyId;
    // use pretty_assertions::assert_eq;

    // #[sqlx::test(migrations = "./migrations", fixtures("test_railway_company"))]
    // async fn it_should_retrieve_railway_companies_from_db(pool: sqlx::SqlitePool) {
    //     let mut conn = pool.acquire().await.expect("should acquire connection");
    //
    //     let id = RailwayCompanyId::try_from("trn:railway-company:fs").unwrap();
    //     let result = get_railway_company_by_id(&mut conn, &id)
    //         .await
    //         .expect("should run query without errors");
    //
    //     assert!(result.is_some());
    //
    //     let railway_company = result.unwrap();
    //     assert_eq!(railway_company.id, id);
    //     assert_eq!(railway_company.name, "FS");
    //     assert_eq!(
    //         railway_company.registered_company_name,
    //         Some("Ferrovie dello Stato".to_string())
    //     );
    //     assert_eq!(railway_company.country_code, Some("IT".to_string()));
    // }
}
