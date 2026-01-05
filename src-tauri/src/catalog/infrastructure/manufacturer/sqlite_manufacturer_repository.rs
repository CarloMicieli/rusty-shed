use crate::catalog::domain::manufacturer::{
    Manufacturer, ManufacturerId, ManufacturerRepository, ManufacturerUowExt,
};
use crate::catalog::infrastructure::entities::ManufacturerRow;
use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use sqlx::SqliteConnection;

/// An SQLite-specific implementation of the `ManufacturerRepository`.
///
/// It holds a mutable reference to a connection, which in this architecture
/// is provided by the `SqliteUnitOfWork`'s active transaction.
pub struct SqliteManufacturerRepository<'conn> {
    /// A mutable reference to the database connection/executor.
    executor: &'conn mut SqliteConnection,
}

impl<'conn> SqliteManufacturerRepository<'conn> {
    /// Creates a new instance of the `SqliteManufacturerRepository`.
    ///
    /// # Arguments
    /// * `executor` - A mutable reference to the database connection/executor.
    ///
    /// # Returns
    /// A new `SqliteManufacturerRepository` instance.
    pub fn new(executor: &'conn mut SqliteConnection) -> Self {
        Self { executor }
    }
}

#[async_trait::async_trait]
impl<'conn> ManufacturerRepository for SqliteManufacturerRepository<'conn> {
    /// Retrieves all Railway Model manufacturers from the database.
    ///
    /// # Arguments
    /// none
    ///
    /// # Returns
    /// - Returns the list of all [`Manufacturer`]s in the database.
    /// - Returns a [`DomainError::DatabaseError`] if the query fails.
    async fn find_all(&mut self) -> Result<Vec<Manufacturer>, DomainError> {
        let sql = r#"
            SELECT id, name, registered_company_name, status, country_code, website_url, created_at, updated_at 
            FROM manufacturers
            ORDER BY name
        "#;

        let row = sqlx::query_as::<_, ManufacturerRow>(sql)
            .fetch_all(&mut *self.executor)
            .await?;

        row.into_iter()
            .map(Manufacturer::try_from)
            .collect::<Result<Vec<Manufacturer>, DomainError>>()
    }

    /// Fetch a manufacturer row by its ID.
    ///
    /// This function executes a simple SELECT query against the `manufacturers` table
    /// and returns the matching `ManufacturerRow` if present.
    ///
    /// # Arguments
    /// - `id` - The manufacturer identifier to look up.
    ///
    /// # Returns
    /// - `Ok(Some(ManufacturerRow))` when a matching row is found
    /// - `Ok(None)` when no row matches the provided `id`
    /// - `Err(DomainError)` if the query fails.
    async fn find_by_id(
        &mut self,
        id: &ManufacturerId,
    ) -> Result<Option<Manufacturer>, DomainError> {
        let sql = r#"
            SELECT id, name, registered_company_name, status, country_code, website_url, created_at, updated_at 
            FROM manufacturers
            WHERE id = ?1 
            LIMIT 1"#;

        let row = sqlx::query_as::<_, ManufacturerRow>(sql)
            .bind(id)
            .fetch_optional(&mut *self.executor)
            .await?;

        match row {
            Some(r) => {
                let manufacturer = Manufacturer::try_from(r)?;
                Ok(Some(manufacturer))
            }
            None => Ok(None),
        }
    }
}

impl<'conn> ManufacturerUowExt for SqliteUnitOfWork<'conn> {
    /// Links the SQLite-specific repository to the Unit of Work.
    ///
    /// It re-borrows the internal transaction (`&mut *self.tx`) to provide
    /// the repository with a mutable executor without transferring ownership.
    fn manufacturers_repo(&mut self) -> Box<dyn ManufacturerRepository + '_> {
        Box::new(SqliteManufacturerRepository::new(&mut self.tx))
    }
}

#[cfg(test)]
mod tests {
    /*
        use super::*;
        use crate::catalog::domain::manufacturer::ManufacturerId;
        use pretty_assertions::assert_eq;
        use url::Url;

        #[sqlx::test(migrations = "./migrations", fixtures("test_manufacturer"))]
        async fn it_should_retrieve_manufacturers_from_db(pool: sqlx::SqlitePool) {
            let mut conn = pool.acquire().await.expect("should acquire connection");

            let id = ManufacturerId::try_from("trn:manufacturer:acme").unwrap();
            let result = get_manufacturer_by_id(&mut conn, &id)
                .await
                .expect("should run query without errors");

            assert!(result.is_some());

            let manufacturer = result.unwrap();
            assert_eq!(manufacturer.id, id);
            assert_eq!(manufacturer.name, "ACME");
            assert_eq!(
                manufacturer.registered_company_name,
                Some("ACME Corporation".to_string())
            );
            assert_eq!(manufacturer.country_code, Some("IT".to_string()));
            assert_eq!(
                manufacturer.website_url,
                Some(Url::parse("https://www.acmetreni.com").unwrap())
            );
        }
    */
}
