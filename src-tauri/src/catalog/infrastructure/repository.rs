use super::database;
use crate::catalog::domain::manufacturer::Manufacturer;
use crate::catalog::domain::manufacturer_id::ManufacturerId;
use crate::catalog::domain::railway_company::RailwayCompany;
use crate::catalog::domain::railway_company_id::RailwayCompanyId;
use crate::catalog::domain::railway_model_id::RailwayModelId;
use crate::catalog::domain::repository::CatalogRepository;
use crate::catalog::domain::{RailwayModel, RollingStock};
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use anyhow::Context;
use sqlx::sqlite::SqliteConnection;
use std::collections::HashMap;

/// An SQLite-specific implementation of the `CollectionRepository`.
///
/// It holds a mutable reference to a connection, which in this architecture
/// is provided by the `SqliteUnitOfWork`'s active transaction.
pub struct SqliteCatalogRepository<'conn> {
    /// A mutable reference to the database connection/executor.
    executor: &'conn mut SqliteConnection,
}

impl<'conn> SqliteCatalogRepository<'conn> {
    /// Creates a new repository instance using the provided executor.
    pub fn new(executor: &'conn mut SqliteConnection) -> Self {
        Self { executor }
    }
}

#[async_trait::async_trait]
impl<'conn> CatalogRepository for SqliteCatalogRepository<'conn> {
    async fn insert_railway_model(&mut self, railway_model: &RailwayModel) -> anyhow::Result<()> {
        let insert_cmd = r#"
        INSERT INTO railway_models (
            id, manufacturer_id, product_code, description, details, \
            power_method, scale, epoch, category, delivery_date, availability_status, created_at, updated_at) \
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        "#;

        sqlx::query(insert_cmd)
            .bind(railway_model.id.to_string())
            .bind(&railway_model.manufacturer)
            .bind(railway_model.product_code.to_string())
            .bind(&railway_model.description)
            .bind(&railway_model.details)
            .bind(railway_model.power_method.to_string())
            .bind(railway_model.scale.to_string())
            .bind(&railway_model.epoch.0) // Access inner String
            .bind(railway_model.category.to_string())
            .bind(railway_model.delivery_date.as_ref().map(|d| d.to_string()))
            .bind(
                railway_model
                    .availability_status
                    .as_ref()
                    .map(|s| s.to_string()),
            )
            .execute(&mut *self.executor)
            .await
            .context("inserting railway model")?;

        Ok(())
    }

    async fn insert_rolling_stock(
        &mut self,
        railway_model_id: &RailwayModelId,
        rolling_stock: &RollingStock,
    ) -> anyhow::Result<()> {
        // Bind parameters and execute the insert command here...
        match rolling_stock {
            RollingStock::ElectricMultipleUnit { id, .. } => {
                let insert_cmd = r#"                
                    INSERT INTO rolling_stocks (
                        id, railway_model_id, category, railway_company_id, livery,
                        length_inches, length_millimeters, technical_minimum_radius_mm, technical_coupling,
                        technical_flywheel_fitted, technical_body_shell, technical_chassis, technical_interior_lights,
                        technical_lights, technical_sprung_buffers, series_code, friendly_name,
                        road_number, series, depot, electric_multiple_unit_type, dcc_interface, control, is_dummy
                    ) VALUES (
                        ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24
                    );"#;
                sqlx::query(insert_cmd)
                    .bind(id)
                    .bind(railway_model_id)
                    .bind(rolling_stock.category())
                    // Bind other parameters...
                    .execute(&mut *self.executor)
                    .await
                    .context("inserting electric multiple unit rolling stock")?;
            }
            RollingStock::Locomotive { id, .. } => {
                let insert_cmd = r#"
                    INSERT INTO rolling_stocks (
                        id, railway_model_id, category, railway_company_id, livery,
                        length_inches, length_millimeters, technical_minimum_radius_mm, technical_coupling,
                        technical_flywheel_fitted, technical_body_shell, technical_chassis, technical_interior_lights,
                        technical_lights, technical_sprung_buffers, series_code, friendly_name,
                        road_number, series, depot, locomotive_type, dcc_interface, control, is_dummy
                    ) VALUES (
                        ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24
                    );"#;
                sqlx::query(insert_cmd)
                    .bind(id)
                    .bind(railway_model_id)
                    .bind(rolling_stock.category())
                    // Bind other parameters...
                    .execute(&mut *self.executor)
                    .await
                    .context("inserting locomotive rolling stock")?;
            }
            RollingStock::PassengerCar { id, .. } => {
                let insert_cmd = r#"
                    INSERT INTO rolling_stocks (
                        id, railway_model_id, category, railway_company_id, livery,
                        length_inches, length_millimeters, technical_minimum_radius_mm, technical_coupling,
                        technical_flywheel_fitted, technical_body_shell, technical_chassis, technical_interior_lights,
                        technical_lights, technical_sprung_buffers, series_code, friendly_name,
                        road_number, series, depot, passenger_car_type, service_level, is_dummy
                    ) VALUES (
                        ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23
                    );"#;
                sqlx::query(insert_cmd)
                    .bind(id)
                    .bind(railway_model_id)
                    .bind(rolling_stock.category())
                    // Bind other parameters...
                    .execute(&mut *self.executor)
                    .await
                    .context("inserting passenger car rolling stock")?;
            }
            RollingStock::FreightCar { id, .. } => {
                let insert_cmd = r#"
                    INSERT INTO rolling_stocks (
                        id, railway_model_id, category, railway_company_id, livery,
                        length_inches, length_millimeters, technical_minimum_radius_mm, technical_coupling,
                        technical_flywheel_fitted, technical_body_shell, technical_chassis, technical_interior_lights,
                        technical_lights, technical_sprung_buffers, series_code, friendly_name,
                        road_number, series, depot, freight_car_type, is_dummy
                    ) VALUES (
                        ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22
                    );"#;
                sqlx::query(insert_cmd)
                    .bind(id)
                    .bind(railway_model_id)
                    .bind(rolling_stock.category())
                    // Bind other parameters...
                    .execute(&mut *self.executor)
                    .await
                    .context("inserting freight car stock")?;
            }
            RollingStock::Railcar { id, .. } => {
                let insert_cmd = r#"
                INSERT INTO rolling_stocks (
                    id, railway_model_id, category, railway_company_id, livery,
                    length_inches, length_millimeters, technical_minimum_radius_mm, technical_coupling,
                    technical_flywheel_fitted, technical_body_shell, technical_chassis, technical_interior_lights,
                    technical_lights, technical_sprung_buffers, series_code, friendly_name,
                    road_number, series, depot, dcc_interface, control, is_dummy
                ) VALUES (
                    ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23
                );"#;
                sqlx::query(insert_cmd)
                    .bind(id)
                    .bind(railway_model_id)
                    .bind(rolling_stock.category())
                    // Bind other parameters...
                    .execute(&mut *self.executor)
                    .await
                    .context("inserting railcar stock")?;
            }
        }
        Ok(())
    }
}

/// An extension trait that provides access to the `CollectionRepository`.
///
/// This follows the **Interface Segregation Principle**. By using extension traits,
/// we avoid a "God Object" where one struct knows about every repository in the
/// system. Instead, repositories are grouped by domain logic.
pub trait CatalogUowExt {
    /// Returns a trait object for interacting with collection data.
    ///
    /// The repository is bound to the lifetime of the Unit of Work to ensure
    /// it cannot outlive the transaction it relies on.
    fn catalog_repository(&mut self) -> Box<dyn CatalogRepository + '_>;
}

impl<'conn> CatalogUowExt for SqliteUnitOfWork<'conn> {
    /// Links the SQLite-specific repository to the Unit of Work.
    ///
    /// It re-borrows the internal transaction (`&mut *self.tx`) to provide
    /// the repository with a mutable executor without transferring ownership.
    fn catalog_repository(&mut self) -> Box<dyn CatalogRepository + '_> {
        Box::new(SqliteCatalogRepository::new(&mut self.tx))
    }
}

/// Retrieve a `Manufacturer` from the database by its `ManufacturerId`.
///
/// This function queries the `manufacturers` table for the provided `id`, maps
/// the resulting row into the domain `Manufacturer` and returns it if found.
///
/// # Arguments
///
/// * `executor` - A mutable reference to an active `SqliteConnection` used to
///   execute the query.
/// * `id` - The domain `ManufacturerId` to look up.
///
/// # Returns
///
/// Returns `Ok(Some(Manufacturer))` when a matching manufacturer was found and
/// successfully mapped, `Ok(None)` when no row matches the given `id`, or
/// `Err(anyhow::Error)` if the query or mapping fails.
///
/// # Errors
///
/// Any errors from the underlying database query (`sqlx`) or from mapping the
/// database row into the domain model are propagated and wrapped in an
/// `anyhow::Error` with additional context.
pub async fn get_manufacturer_by_id(
    executor: &mut SqliteConnection,
    id: &ManufacturerId,
) -> anyhow::Result<Option<Manufacturer>> {
    let row_opt = database::get_manufacturer_by_id(executor, id)
        .await
        .context("querying manufacturers table")?;

    if let Some(row) = row_opt {
        let manufacturer = Manufacturer::try_from(row)
            .map_err(|e| anyhow::anyhow!("mapping ManufacturerRow -> Manufacturer: {}", e))?;
        Ok(Some(manufacturer))
    } else {
        Ok(None)
    }
}

/// Retrieve a `RailwayCompany` from the database by its `RailwayCompanyId`.
///
/// This mirrors `get_manufacturer_by_id` but for the `railway_companies` table.
pub async fn get_railway_company_by_id(
    executor: &mut SqliteConnection,
    id: &RailwayCompanyId,
) -> anyhow::Result<Option<RailwayCompany>> {
    let row_opt = database::get_railway_company_by_id(executor, &id.to_string())
        .await
        .context("querying railway_companies table")?;

    if let Some(row) = row_opt {
        let rc = RailwayCompany::try_from(row).map_err(|e| {
            anyhow::anyhow!(format!(
                "mapping RailwayCompanyRow -> RailwayCompany: {}",
                e
            ))
        })?;
        Ok(Some(rc))
    } else {
        Ok(None)
    }
}

/// Retrieve a `RailwayModel` by its `RailwayModelId` using a two-query strategy.
///
/// Query 1: load parent `railway_models` row. Query 2: load child `rolling_stocks` rows.
pub async fn get_railway_model_by_id(
    executor: &mut SqliteConnection,
    id: &RailwayModelId,
) -> anyhow::Result<Option<RailwayModel>> {
    let row_opt = database::get_railway_model_by_id(executor, &id.to_string())
        .await
        .context("querying railway_models table")?;

    if let Some(row) = row_opt {
        let mut rm = RailwayModel::try_from(row).map_err(|e| {
            anyhow::anyhow!(format!("mapping RailwayModelRow -> RailwayModel: {}", e))
        })?;

        // fetch rolling stocks
        let child_rows =
            database::get_rolling_stocks_by_railway_model_id(executor, &id.to_string())
                .await
                .context("querying rolling_stocks table")?;

        let mut rolling_stocks = Vec::with_capacity(child_rows.len());
        for cr in child_rows {
            let rs = RollingStock::try_from(cr).map_err(|e| {
                anyhow::anyhow!(format!("mapping RollingStockRow -> RollingStock: {}", e))
            })?;
            rolling_stocks.push(rs);
        }

        rm.rolling_stocks = rolling_stocks;

        Ok(Some(rm))
    } else {
        Ok(None)
    }
}

/// Retrieve multiple `RailwayModel`s by their ids with rolling stocks loaded.
pub async fn get_railway_models_by_ids(
    executor: &mut SqliteConnection,
    ids: &[RailwayModelId],
) -> anyhow::Result<Vec<RailwayModel>> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }

    let id_strings: Vec<String> = ids.iter().map(|id| id.to_string()).collect();

    let rows = database::get_railway_models_by_ids(executor, &id_strings)
        .await
        .context("querying railway_models table")?;

    let mut models: HashMap<String, RailwayModel> = HashMap::with_capacity(rows.len());
    for row in rows {
        let mut model = RailwayModel::try_from(row).map_err(|e| {
            anyhow::anyhow!(format!("mapping RailwayModelRow -> RailwayModel: {}", e))
        })?;
        model.rolling_stocks = Vec::new();
        models.insert(model.id.to_string(), model);
    }

    let child_rows = database::get_rolling_stocks_by_railway_model_ids(executor, &id_strings)
        .await
        .context("querying rolling_stocks table")?;

    for cr in child_rows {
        let rs = RollingStock::try_from(cr.clone()).map_err(|e| {
            anyhow::anyhow!(format!("mapping RollingStockRow -> RollingStock: {}", e))
        })?;

        if let Some(model) = models.get_mut(&cr.railway_model_id) {
            model.rolling_stocks.push(rs);
        }
    }

    // Preserve caller order
    let mut result = Vec::with_capacity(ids.len());
    for id in ids {
        if let Some(model) = models.remove(&id.to_string()) {
            result.push(model);
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod manufacturer_repo_tests {
        use super::*;
        use crate::catalog::domain::manufacturer_id::ManufacturerId;
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
    }

    mod railway_repo_tests {
        use super::*;
        use crate::catalog::domain::railway_company_id::RailwayCompanyId;
        use pretty_assertions::assert_eq;

        #[sqlx::test(migrations = "./migrations", fixtures("test_railway_company"))]
        async fn it_should_retrieve_railway_companies_from_db(pool: sqlx::SqlitePool) {
            let mut conn = pool.acquire().await.expect("should acquire connection");

            let id = RailwayCompanyId::try_from("trn:railway-company:fs").unwrap();
            let result = get_railway_company_by_id(&mut conn, &id)
                .await
                .expect("should run query without errors");

            assert!(result.is_some());

            let railway_company = result.unwrap();
            assert_eq!(railway_company.id, id);
            assert_eq!(railway_company.name, "FS");
            assert_eq!(
                railway_company.registered_company_name,
                Some("Ferrovie dello Stato".to_string())
            );
            assert_eq!(railway_company.country_code, Some("IT".to_string()));
        }
    }

    mod railway_model_repo_tests {
        use super::*;
        use crate::catalog::domain::Category;
        use pretty_assertions::assert_eq;

        #[sqlx::test(migrations = "./migrations", fixtures("test_railway_model"))]
        async fn it_should_retrieve_railway_model_from_db(pool: sqlx::SqlitePool) {
            let mut conn = pool.acquire().await.expect("should acquire connection");

            let id = RailwayModelId::try_from("trn:railway-model:acme:60100").unwrap();
            let result = get_railway_model_by_id(&mut conn, &id)
                .await
                .expect("should run query without errors");

            assert!(result.is_some());

            let railway_model = result.unwrap();
            assert_eq!(railway_model.id, id);
            assert_eq!(
                railway_model.manufacturer.to_string(),
                "trn:manufacturer:acme"
            );
            assert_eq!(railway_model.product_code.to_string(), "60100");
            assert_eq!(
                railway_model.description,
                "Locomotiva elettrica E.444.005 Tartaruga"
            );
            assert_eq!(railway_model.rolling_stocks.len(), 1);
            assert_eq!(railway_model.category, Category::Locomotives)
        }
    }
}
