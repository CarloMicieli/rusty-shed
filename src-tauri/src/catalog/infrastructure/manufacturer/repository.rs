use crate::catalog::domain::manufacturer::{
    Manufacturer, ManufacturerId, ManufacturerRepository, ManufacturerUowExt,
};
use crate::catalog::infrastructure::entities::ManufacturerRow;
use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::core::infrastructure::usage_queries::manufacturer_usage_count;
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

    async fn find_is_system_seeded(
        &mut self,
        id: &ManufacturerId,
    ) -> Result<Option<bool>, DomainError> {
        let sql = r#"
            SELECT is_system_seeded
            FROM manufacturers
            WHERE id = ?1
            LIMIT 1
        "#;

        let seeded = sqlx::query_scalar::<_, i64>(sql)
            .bind(id.as_ref())
            .fetch_optional(&mut *self.executor)
            .await?
            .map(|value| value != 0);

        Ok(seeded)
    }

    async fn find_seeded_and_name(
        &mut self,
        id: &ManufacturerId,
    ) -> Result<Option<(String, bool)>, DomainError> {
        sqlx::query_as::<_, (String, i64)>(
            r#"SELECT name, is_system_seeded FROM manufacturers WHERE id = ?1 LIMIT 1"#,
        )
        .bind(id.as_ref())
        .fetch_optional(&mut *self.executor)
        .await
        .map(|opt| opt.map(|(name, seeded)| (name, seeded != 0)))
        .map_err(|e| DomainError::Infrastructure(e.to_string()))
    }

    async fn relink_railway_models(
        &mut self,
        source_id: &ManufacturerId,
        target_id: &ManufacturerId,
    ) -> Result<i64, DomainError> {
        let sql = r#"
            UPDATE railway_models
            SET manufacturer_id = ?2
            WHERE manufacturer_id = ?1
        "#;

        let rows = sqlx::query(sql)
            .bind(source_id.as_ref())
            .bind(target_id.as_ref())
            .execute(&mut *self.executor)
            .await?
            .rows_affected() as i64;

        Ok(rows)
    }

    async fn delete_by_id(&mut self, id: &ManufacturerId) -> Result<u64, DomainError> {
        let sql = r#"
            DELETE FROM manufacturers
            WHERE id = ?1
        "#;

        let rows = sqlx::query(sql)
            .bind(id.as_ref())
            .execute(&mut *self.executor)
            .await?
            .rows_affected();

        Ok(rows)
    }

    async fn find_usage_count(&mut self, id: &ManufacturerId) -> Result<i64, DomainError> {
        manufacturer_usage_count(&mut *self.executor, id.as_ref())
            .await
            .map_err(|e| DomainError::Infrastructure(e.to_string()))
    }

    async fn insert(
        &mut self,
        id: &ManufacturerId,
        name: String,
        country_code: Option<String>,
        website_url: Option<String>,
    ) -> Result<Manufacturer, DomainError> {
        let insert_result = sqlx::query(
            r#"INSERT INTO manufacturers (id, name, status, country_code, website_url)
               VALUES (?1, ?2, 'ACTIVE', ?3, ?4)"#,
        )
        .bind(id.as_ref())
        .bind(name)
        .bind(country_code)
        .bind(website_url)
        .execute(&mut *self.executor)
        .await;

        if let Err(err) = insert_result {
            if let sqlx::Error::Database(db_err) = &err
                && db_err.is_unique_violation()
            {
                return Err(DomainError::Conflict(
                    "A manufacturer with this name already exists".to_string(),
                ));
            }
            return Err(DomainError::Infrastructure(err.to_string()));
        }

        let row = sqlx::query_as::<_, ManufacturerRow>(
            r#"SELECT id, name, registered_company_name, status, country_code, website_url,
                      created_at, updated_at, version
               FROM manufacturers
               WHERE id = ?1
               LIMIT 1"#,
        )
        .bind(id.as_ref())
        .fetch_one(&mut *self.executor)
        .await
        .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        Manufacturer::try_from(row)
    }

    async fn update(
        &mut self,
        id: &ManufacturerId,
        name: String,
        country_code: Option<String>,
        website_url: Option<String>,
    ) -> Result<Manufacturer, DomainError> {
        let update_result = sqlx::query(
            r#"UPDATE manufacturers
               SET name = ?2, country_code = ?3, website_url = ?4,
                   updated_at = CURRENT_TIMESTAMP
               WHERE id = ?1"#,
        )
        .bind(id.as_ref())
        .bind(name)
        .bind(country_code)
        .bind(website_url)
        .execute(&mut *self.executor)
        .await;

        if let Err(err) = update_result {
            if let sqlx::Error::Database(db_err) = &err
                && db_err.is_unique_violation()
            {
                return Err(DomainError::Conflict(
                    "A manufacturer with this name already exists".to_string(),
                ));
            }
            return Err(DomainError::Infrastructure(err.to_string()));
        }

        let row = sqlx::query_as::<_, ManufacturerRow>(
            r#"SELECT id, name, registered_company_name, status, country_code, website_url,
                      created_at, updated_at, version
               FROM manufacturers
               WHERE id = ?1
               LIMIT 1"#,
        )
        .bind(id.as_ref())
        .fetch_one(&mut *self.executor)
        .await
        .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        Manufacturer::try_from(row)
    }
}

impl ManufacturerUowExt for SqliteUnitOfWork {
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
    use crate::core::domain::identifiers::Identifier;
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

    #[sqlx::test(migrations = "./migrations")]
    async fn it_should_find_seeded_flag(pool: sqlx::SqlitePool) {
        let mut conn = pool.acquire().await.expect("should acquire connection");

        let id = ManufacturerId::new_from_parts(&["seeded"]);
        sqlx::query(
            r#"
            INSERT INTO manufacturers (id, name, status, created_at, updated_at, version, is_system_seeded)
            VALUES (?1, 'Seeded Manufacturer', 'ACTIVE', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 1, 1)
            "#,
        )
        .bind(id.as_ref())
        .execute(&mut *conn)
        .await
        .expect("manufacturer should insert");

        let mut repository = SqliteManufacturerRepository::new(&mut conn);
        let result = repository
            .find_is_system_seeded(&id)
            .await
            .expect("seeded query should succeed");

        assert_eq!(result, Some(true));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn it_should_relink_models_and_delete_manufacturer(pool: sqlx::SqlitePool) {
        let mut conn = pool.acquire().await.expect("should acquire connection");

        let source_id = ManufacturerId::new_from_parts(&["source"]);
        let target_id = ManufacturerId::new_from_parts(&["target"]);
        let railway_company_id = "trn:railway-company:fs";

        sqlx::query(
            r#"
            INSERT INTO railway_companies (id, name, status, created_at, updated_at, version)
            VALUES (?1, 'FS', 'ACTIVE', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 1)
            "#,
        )
        .bind(railway_company_id)
        .execute(&mut *conn)
        .await
        .expect("railway company should insert");

        sqlx::query(
            r#"
            INSERT INTO manufacturers (id, name, status, created_at, updated_at, version, is_system_seeded)
            VALUES
                (?1, 'Source Manufacturer', 'ACTIVE', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 1, 0),
                (?2, 'Target Manufacturer', 'ACTIVE', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 1, 0)
            "#,
        )
        .bind(source_id.as_ref())
        .bind(target_id.as_ref())
        .execute(&mut *conn)
        .await
        .expect("manufacturers should insert");

        sqlx::query(
            r#"
            INSERT INTO railway_models (
                id,
                manufacturer_id,
                product_code,
                power_method,
                scale,
                epoch,
                category,
                created_at,
                updated_at,
                version
            )
            VALUES
                ('trn:railway-model:source:one', ?1, 'SRC-001', 'ANALOG_DC', 'H0', 'V', 'LOCOMOTIVE', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 1),
                ('trn:railway-model:source:two', ?1, 'SRC-002', 'ANALOG_DC', 'H0', 'V', 'LOCOMOTIVE', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 1)
            "#,
        )
        .bind(source_id.as_ref())
        .execute(&mut *conn)
        .await
        .expect("railway models should insert");

        sqlx::query(
            r#"
            INSERT INTO rolling_stocks (
                id,
                railway_model_id,
                railway_company_id,
                category,
                locomotive_type,
                series_code
            )
            VALUES
                ('trn:rolling-stock:source:one', 'trn:railway-model:source:one', ?1, 'LOCOMOTIVE', 'ELECTRIC', 'E.001'),
                ('trn:rolling-stock:source:two', 'trn:railway-model:source:two', ?1, 'LOCOMOTIVE', 'ELECTRIC', 'E.002')
            "#,
        )
        .bind(railway_company_id)
        .execute(&mut *conn)
        .await
        .expect("rolling stocks should insert");

        let mut repository = SqliteManufacturerRepository::new(&mut conn);

        let relinked = repository
            .relink_railway_models(&source_id, &target_id)
            .await
            .expect("relink should succeed");
        assert_eq!(relinked, 2);

        let deleted = repository
            .delete_by_id(&source_id)
            .await
            .expect("delete should succeed");
        assert_eq!(deleted, 1);
    }
}
