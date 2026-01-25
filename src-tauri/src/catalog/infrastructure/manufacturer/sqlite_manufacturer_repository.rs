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
            SELECT id, name, registered_company_name, status, country_code, website_url, created_at, updated_at, version
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
            SELECT id, name, registered_company_name, status, country_code, website_url, created_at, updated_at, version
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
    use super::*;
    use crate::catalog::domain::manufacturer::{ManufacturerId, ManufacturerStatus};
    use pretty_assertions::assert_eq;
    use url::Url;

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../../fixtures/test_manufacturer.sql")
    )]
    async fn it_should_find_manufacturer_by_id(pool: sqlx::SqlitePool) {
        let mut conn = pool.acquire().await.expect("should acquire connection");

        let mut repository = SqliteManufacturerRepository::new(&mut conn);

        let id = ManufacturerId::try_from("trn:manufacturer:acme").unwrap();
        let result = repository
            .find_by_id(&id)
            .await
            .expect("should run query without errors");

        assert!(result.is_some());

        let manufacturer = result.expect("should find a manufacturer");
        assert_eq!(manufacturer.id, id);
        assert_eq!(manufacturer.name, "ACME");
        assert_eq!(
            manufacturer.registered_company_name,
            Some("Anonima Costruzioni Modellistiche Esatte S.r.l.".to_string())
        );
        assert_eq!(manufacturer.country_code, Some("IT".to_string()));
        assert_eq!(
            manufacturer.website_url,
            Some(Url::parse("https://www.acmetreni.com").unwrap())
        );
        assert_eq!(manufacturer.status, ManufacturerStatus::Active);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn it_should_find_no_manufacturer_when_id_is_not_found(pool: sqlx::SqlitePool) {
        let mut conn = pool.acquire().await.expect("should acquire connection");

        let mut repository = SqliteManufacturerRepository::new(&mut conn);

        let id = ManufacturerId::try_from("trn:manufacturer:not-found").unwrap();
        let result = repository
            .find_by_id(&id)
            .await
            .expect("should run query without errors");

        assert!(result.is_none());
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../../fixtures/test_manufacturer.sql")
    )]
    async fn it_should_find_all_manufacturers(pool: sqlx::SqlitePool) {
        let mut conn = pool.acquire().await.expect("should acquire connection");

        let mut repository = SqliteManufacturerRepository::new(&mut conn);

        let result = repository
            .find_all()
            .await
            .expect("should run query without errors");

        assert_eq!(result.len(), 2);

        let manufacturer_1 = result.first().expect("should find a manufacturer");
        let id_1 = ManufacturerId::try_from("trn:manufacturer:acme").unwrap();
        assert_eq!(manufacturer_1.id, id_1);
        assert_eq!(manufacturer_1.name, "ACME");
        assert_eq!(
            manufacturer_1.registered_company_name,
            Some("Anonima Costruzioni Modellistiche Esatte S.r.l.".to_string())
        );
        assert_eq!(manufacturer_1.country_code, Some("IT".to_string()));
        assert_eq!(
            manufacturer_1.website_url,
            Some(Url::parse("https://www.acmetreni.com").unwrap())
        );
        assert_eq!(manufacturer_1.status, ManufacturerStatus::Active);

        let manufacturer_2 = result.get(1).expect("should find a manufacturer");
        let id_2 = ManufacturerId::try_from("trn:manufacturer:roco").unwrap();
        assert_eq!(manufacturer_2.id, id_2);
        assert_eq!(manufacturer_2.name, "Roco");
        assert_eq!(
            manufacturer_2.registered_company_name,
            Some("Modelleisenbahn München GmbH".to_string())
        );
        assert_eq!(manufacturer_2.country_code, Some("AT".to_string()));
        assert_eq!(
            manufacturer_2.website_url,
            Some(Url::parse("https://www.roco.cc").unwrap())
        );
        assert_eq!(manufacturer_2.status, ManufacturerStatus::Active);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn it_should_find_no_manufacturers_when_table_is_empty(pool: sqlx::SqlitePool) {
        let mut conn = pool.acquire().await.expect("should acquire connection");
        let mut repository = SqliteManufacturerRepository::new(&mut conn);

        let result = repository
            .find_all()
            .await
            .expect("should run query without errors");

        assert_eq!(result.len(), 0);
    }
}
