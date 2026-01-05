use super::database;
use crate::catalog::domain::railway_model::RailwayModelId;
use crate::catalog::domain::railway_model::RailwayModelRepository;
use crate::catalog::domain::railway_model::RollingStockCategory;
use crate::catalog::domain::railway_model::RollingStockId;
use crate::catalog::domain::railway_model::{RailwayModel, RollingStock};
use crate::catalog::domain::railway_model::{RailwayModelParams, RollingStockParams};
use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use anyhow::Context;
use sqlx::sqlite::SqliteConnection;
use std::collections::HashMap;

/// An SQLite-specific implementation of the `RailwayModelRepository`.
///
/// It holds a mutable reference to a connection, which in this architecture
/// is provided by the `SqliteUnitOfWork`'s active transaction.
pub struct SqliteRailwayModelRepository<'conn> {
    /// A mutable reference to the database connection/executor.
    executor: &'conn mut SqliteConnection,
}

impl<'conn> SqliteRailwayModelRepository<'conn> {
    /// Creates a new repository instance using the provided executor.
    pub fn new(executor: &'conn mut SqliteConnection) -> Self {
        Self { executor }
    }

    pub async fn insert_railway_model(
        &mut self,
        railway_model: &RailwayModelParams,
    ) -> Result<RailwayModelId, DomainError> {
        let insert_cmd = r#"
        INSERT INTO railway_models (
            id, manufacturer_id, product_code, description, details, 
            power_method, scale, epoch, category, delivery_date, availability_status, created_at, updated_at) 
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
        "#;

        //TODO: fix me
        let id = RailwayModelId::new(&railway_model.manufacturer_id, &railway_model.product_code)
            .map_err(|e| DomainError::Validation(e.to_string()))?;

        sqlx::query(insert_cmd)
            .bind(&id)
            .bind(&railway_model.manufacturer_id)
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
            .map_err(DomainError::from)?;

        Ok(id)
    }

    pub async fn insert_rolling_stock(
        &mut self,
        railway_model_id: &RailwayModelId,
        rolling_stock: &RollingStockParams,
    ) -> Result<(), DomainError> {
        // Bind parameters and execute the insert command here...
        let id = RollingStockId::new();

        let (
            technical_coupling,
            technical_flywheel_fitted,
            technical_body_shell,
            technical_chassis,
            technical_interior_lights,
            technical_lights,
            technical_sprung_buffers,
        ) = if let Some(tech_specs) = rolling_stock.technical_specifications() {
            (
                tech_specs.coupling,
                tech_specs.flywheel_fitted,
                tech_specs.body_shell,
                tech_specs.chassis,
                tech_specs.interior_lights,
                tech_specs.lights,
                tech_specs.sprung_buffers,
            )
        } else {
            (None, None, None, None, None, None, None)
        };

        let (
            technical_coupling_socket,
            technical_coupling_close_couplers,
            technical_coupling_digital_shunting,
        ) = if let Some(coupling) = technical_coupling {
            (
                coupling.socket(),
                coupling.close_couplers(),
                coupling.digital_shunting(),
            )
        } else {
            (None, None, None)
        };

        match rolling_stock {
            RollingStockParams::ElectricMultipleUnitParams {
                railway_company_id,
                livery,
                length_over_buffers: _,
                technical_specifications: _,
                friendly_name,
                series_code,
                road_number,
                series,
                depot,
                electric_multiple_unit_type,
                dcc_interface,
                control,
                is_dummy,
            } => {
                let insert_cmd = r#"                
                    INSERT INTO rolling_stocks (
                        id, railway_model_id, category, railway_company_id, livery,
                        length_inches, length_millimeters, technical_minimum_radius_mm, 
                        technical_coupling_socket, technical_coupling_close_couplers, technical_coupling_digital_shunting,
                        technical_flywheel_fitted, technical_body_shell, technical_chassis, technical_interior_lights,
                        technical_lights, technical_sprung_buffers, series_code, friendly_name,
                        road_number, series, depot, electric_multiple_unit_type, dcc_interface, control, is_dummy
                    ) VALUES (
                        ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26
                    );"#;

                sqlx::query(insert_cmd)
                    .bind(id)
                    .bind(railway_model_id)
                    .bind(RollingStockCategory::ElectricMultipleUnit)
                    .bind(railway_company_id)
                    .bind(livery)
                    .bind(None::<f64>) //length_inches
                    .bind(None::<f64>) //length_inches
                    .bind(None::<f64>) //technical_minimum_radius_mm
                    .bind(technical_coupling_socket)
                    .bind(technical_coupling_close_couplers)
                    .bind(technical_coupling_digital_shunting)
                    .bind(technical_flywheel_fitted)
                    .bind(technical_body_shell)
                    .bind(technical_chassis)
                    .bind(technical_interior_lights)
                    .bind(technical_lights)
                    .bind(technical_sprung_buffers)
                    .bind(series_code)
                    .bind(friendly_name)
                    .bind(road_number)
                    .bind(series)
                    .bind(depot)
                    .bind(electric_multiple_unit_type)
                    .bind(dcc_interface)
                    .bind(control)
                    .bind(is_dummy)
                    .execute(&mut *self.executor)
                    .await
                    .map_err(DomainError::from)?;
            }
            RollingStockParams::LocomotiveParams {
                railway_company_id,
                livery,
                length_over_buffers: _,
                technical_specifications: _,
                friendly_name,
                series_code,
                road_number,
                series,
                depot,
                locomotive_type,
                dcc_interface,
                control,
                is_dummy,
            } => {
                let insert_cmd = r#"
                    INSERT INTO rolling_stocks (
                        id, railway_model_id, category, railway_company_id, livery,
                        length_inches, length_millimeters, technical_minimum_radius_mm, 
                        technical_coupling_socket, technical_coupling_close_couplers, technical_coupling_digital_shunting,
                        technical_flywheel_fitted, technical_body_shell, technical_chassis, technical_interior_lights,
                        technical_lights, technical_sprung_buffers, series_code, friendly_name,
                        road_number, series, depot, locomotive_type, dcc_interface, control, is_dummy
                    ) VALUES (
                        ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26
                    );"#;
                sqlx::query(insert_cmd)
                    .bind(id)
                    .bind(railway_model_id)
                    .bind(RollingStockCategory::Locomotive)
                    .bind(railway_company_id)
                    .bind(livery)
                    .bind(None::<f64>) //length_inches
                    .bind(None::<f64>) //length_inches
                    .bind(None::<f64>) //technical_minimum_radius_mm
                    .bind(technical_coupling_socket)
                    .bind(technical_coupling_close_couplers)
                    .bind(technical_coupling_digital_shunting)
                    .bind(technical_flywheel_fitted)
                    .bind(technical_body_shell)
                    .bind(technical_chassis)
                    .bind(technical_interior_lights)
                    .bind(technical_lights)
                    .bind(technical_sprung_buffers)
                    .bind(series_code)
                    .bind(friendly_name)
                    .bind(road_number)
                    .bind(series)
                    .bind(depot)
                    .bind(locomotive_type)
                    .bind(dcc_interface)
                    .bind(control)
                    .bind(is_dummy)
                    .execute(&mut *self.executor)
                    .await
                    .map_err(DomainError::from)?;
            }
            RollingStockParams::PassengerCarParams {
                railway_company_id,
                livery,
                length_over_buffers: _,
                technical_specifications: _,
                friendly_name,
                series_code,
                road_number,
                series,
                passenger_car_type,
                service_level,
            } => {
                let insert_cmd = r#"
                    INSERT INTO rolling_stocks (
                        id, railway_model_id, category, railway_company_id, livery,
                        length_inches, length_millimeters, technical_minimum_radius_mm, 
                        technical_coupling_socket, technical_coupling_close_couplers, technical_coupling_digital_shunting,
                        technical_flywheel_fitted, technical_body_shell, technical_chassis, technical_interior_lights,
                        technical_lights, technical_sprung_buffers, series_code, friendly_name,
                        road_number, series, passenger_car_type, service_level
                    ) VALUES (
                        ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23
                    );"#;
                sqlx::query(insert_cmd)
                    .bind(id)
                    .bind(railway_model_id)
                    .bind(RollingStockCategory::PassengerCar)
                    .bind(railway_company_id)
                    .bind(livery)
                    .bind(None::<f64>) //length_inches
                    .bind(None::<f64>) //length_inches
                    .bind(None::<f64>) //technical_minimum_radius_mm
                    .bind(technical_coupling_socket)
                    .bind(technical_coupling_close_couplers)
                    .bind(technical_coupling_digital_shunting)
                    .bind(technical_flywheel_fitted)
                    .bind(technical_body_shell)
                    .bind(technical_chassis)
                    .bind(technical_interior_lights)
                    .bind(technical_lights)
                    .bind(technical_sprung_buffers)
                    .bind(series_code)
                    .bind(friendly_name)
                    .bind(road_number)
                    .bind(series)
                    .bind(passenger_car_type)
                    .bind(service_level)
                    .execute(&mut *self.executor)
                    .await
                    .map_err(DomainError::from)?;
            }
            RollingStockParams::FreightCarParams {
                railway_company_id,
                livery,
                length_over_buffers: _,
                technical_specifications: _,
                friendly_name,
                series_code,
                series,
                road_number,
                freight_car_type,
            } => {
                let insert_cmd = r#"
                    INSERT INTO rolling_stocks (
                        id, railway_model_id, category, railway_company_id, livery,
                        length_inches, length_millimeters, technical_minimum_radius_mm,
                        technical_coupling_socket, technical_coupling_close_couplers, technical_coupling_digital_shunting,
                        technical_flywheel_fitted, technical_body_shell, technical_chassis, technical_interior_lights,
                        technical_lights, technical_sprung_buffers, series_code, series, friendly_name,
                        road_number, freight_car_type
                    ) VALUES (
                        ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22
                    );"#;
                sqlx::query(insert_cmd)
                    .bind(id)
                    .bind(railway_model_id)
                    .bind(RollingStockCategory::PassengerCar)
                    .bind(railway_company_id)
                    .bind(livery)
                    .bind(None::<f64>) //length_inches
                    .bind(None::<f64>) //length_inches
                    .bind(None::<f64>) //technical_minimum_radius_mm
                    .bind(technical_coupling_socket)
                    .bind(technical_coupling_close_couplers)
                    .bind(technical_coupling_digital_shunting)
                    .bind(technical_flywheel_fitted)
                    .bind(technical_body_shell)
                    .bind(technical_chassis)
                    .bind(technical_interior_lights)
                    .bind(technical_lights)
                    .bind(technical_sprung_buffers)
                    .bind(series_code)
                    .bind(series)
                    .bind(friendly_name)
                    .bind(road_number)
                    .bind(freight_car_type)
                    .execute(&mut *self.executor)
                    .await
                    .map_err(DomainError::from)?;
            }
            RollingStockParams::RailcarParams {
                railway_company_id,
                livery,
                length_over_buffers: _,
                technical_specifications: _,
                friendly_name,
                series_code,
                road_number,
                series,
                depot,
                railcar_type,
                dcc_interface,
                control,
                is_dummy,
            } => {
                let insert_cmd = r#"
                INSERT INTO rolling_stocks (
                    id, railway_model_id, category, railway_company_id, livery,
                    length_inches, length_millimeters, technical_minimum_radius_mm, 
                    technical_coupling_socket, technical_coupling_close_couplers, technical_coupling_digital_shunting,
                    technical_flywheel_fitted, technical_body_shell, technical_chassis, technical_interior_lights,
                    technical_lights, technical_sprung_buffers, series_code, friendly_name,
                    road_number, series, depot, railcar_type, dcc_interface, control, is_dummy
                ) VALUES (
                    ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26
                );"#;
                sqlx::query(insert_cmd)
                    .bind(id)
                    .bind(railway_model_id)
                    .bind(RollingStockCategory::Railcar)
                    .bind(railway_company_id)
                    .bind(livery)
                    .bind(None::<f64>) //length_inches
                    .bind(None::<f64>) //length_inches
                    .bind(None::<f64>) //technical_minimum_radius_mm
                    .bind(technical_coupling_socket)
                    .bind(technical_coupling_close_couplers)
                    .bind(technical_coupling_digital_shunting)
                    .bind(technical_flywheel_fitted)
                    .bind(technical_body_shell)
                    .bind(technical_chassis)
                    .bind(technical_interior_lights)
                    .bind(technical_lights)
                    .bind(technical_sprung_buffers)
                    .bind(series_code)
                    .bind(friendly_name)
                    .bind(road_number)
                    .bind(series)
                    .bind(depot)
                    .bind(railcar_type)
                    .bind(dcc_interface)
                    .bind(control)
                    .bind(is_dummy)
                    .execute(&mut *self.executor)
                    .await
                    .map_err(DomainError::from)?;
            }
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl<'conn> RailwayModelRepository for SqliteRailwayModelRepository<'conn> {
    async fn create(&mut self, params: &RailwayModelParams) -> Result<RailwayModelId, DomainError> {
        let railway_model_id = self.insert_railway_model(params).await?;

        for rs in params.rolling_stocks.iter() {
            self.insert_rolling_stock(&railway_model_id, rs).await?;
        }

        Ok(railway_model_id)
    }
}

/// An extension trait that provides access to the `RailwayModelRepository`.
///
/// This follows the **Interface Segregation Principle**. By using extension traits,
/// we avoid a "God Object" where one struct knows about every repository in the
/// system. Instead, repositories are grouped by domain logic.
pub trait CatalogUowExt {
    /// Returns a trait object for interacting with railway model data.
    ///
    /// The repository is bound to the lifetime of the Unit of Work to ensure
    /// it cannot outlive the transaction it relies on.
    fn railway_models(&mut self) -> Box<dyn RailwayModelRepository + '_>;
}

impl<'conn> CatalogUowExt for SqliteUnitOfWork<'conn> {
    /// Links the SQLite-specific repository to the Unit of Work.
    ///
    /// It re-borrows the internal transaction (`&mut *self.tx`) to provide
    /// the repository with a mutable executor without transferring ownership.
    fn railway_models(&mut self) -> Box<dyn RailwayModelRepository + '_> {
        Box::new(SqliteRailwayModelRepository::new(&mut self.tx))
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

    mod railway_model_repo_tests {
        use super::*;
        use crate::catalog::domain::railway_model::Category;
        use pretty_assertions::assert_eq;

        #[sqlx::test(migrations = "./migrations", fixtures("test_railway_model"))]
        async fn it_should_retrieve_railway_model_from_db(pool: sqlx::SqlitePool) {
            let mut conn = pool.acquire().await.expect("should acquire connection");

            let id = RailwayModelId::try_from("trn:railway-model:acme:60100").unwrap();
            let result = get_railway_model_by_id(&mut conn, &id)
                .await
                .expect("should run query without errors");

            println!("{:#?}", result);
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

    mod model_railway_repository {
        use super::*;
        use crate::catalog::domain::railway_model::AvailabilityStatus;
        use crate::catalog::domain::railway_model::Control;
        use crate::catalog::domain::railway_model::DccInterface;
        use crate::catalog::domain::railway_model::RailwayModelId;
        use crate::catalog::domain::railway_model::{
            Category, DeliveryDate, Epoch, PowerMethod, ProductCode, ServiceLevel,
        };
        use crate::catalog::domain::railway_model::{
            ElectricMultipleUnitType, FreightCarType, LocomotiveType, PassengerCarType, RailcarType,
        };
        use crate::catalog::domain::scale::Scale;
        use pretty_assertions::assert_eq;
        use sqlx::Row;
        use crate::catalog::domain::manufacturer::ManufacturerId;
        use crate::catalog::domain::railway_company::RailwayCompanyId;

        const TEST_RAILWAY_MODEL_ID: &str = "trn:railway-model:acme:1234";
        const RAILWAY_MODEL_QUERY: &str = r#"
                SELECT *
                FROM railway_models
                WHERE id = ?1
            "#;
        const ROLLING_STOCK_QUERY: &str = r#"
                SELECT *
                FROM rolling_stocks
                WHERE railway_model_id = ?1
            "#;

        #[sqlx::test(migrations = "./migrations", fixtures("test_railway_model"))]
        async fn it_should_insert_railway_models(pool: sqlx::SqlitePool) {
            let mut conn = pool.acquire().await.expect("should acquire connection");

            let params = RailwayModelParams {
                manufacturer_id: ManufacturerId::try_from("trn:manufacturer:acme").unwrap(),
                product_code: ProductCode("9999".to_string()),
                description: "Test Model".to_string(),
                details: None,
                power_method: PowerMethod::DC,
                scale: Scale::H0,
                epoch: Epoch::from("IV"),
                category: Category::Locomotives,
                delivery_date: Some(DeliveryDate::Year(2023)),
                availability_status: Some(AvailabilityStatus::Available),
                rolling_stocks: vec![],
            };
            let mut repo = SqliteRailwayModelRepository::new(&mut conn);
            let result = repo.insert_railway_model(&params).await;

            assert!(result.is_ok(), "should insert railway model without errors");

            let railway_model_id = result.unwrap();

            let row = sqlx::query(RAILWAY_MODEL_QUERY)
                .bind(&railway_model_id)
                .fetch_one(&mut *conn)
                .await
                .expect("should fetch one railway model row");

            let manufacturer_id: String = row.get("manufacturer_id");
            let product_code: String = row.get("product_code");
            let description: String = row.get("description");
            let scale: String = row.get("scale");
            let power_method: String = row.get("power_method");
            let epoch: String = row.get("epoch");
            let category: String = row.get("category");
            let delivery_date: Option<String> = row.get("delivery_date");
            let availability_status: Option<String> = row.get("availability_status");

            assert_eq!(manufacturer_id, "trn:manufacturer:acme");
            assert_eq!(product_code, "9999");
            assert_eq!(description, "Test Model");
            assert_eq!(scale, "H0 (1:87)");
            assert_eq!(power_method, "DC");
            assert_eq!(epoch, "IV");
            assert_eq!(category, "LOCOMOTIVES");
            assert_eq!(delivery_date, Some("2023".to_string()));
            assert_eq!(availability_status, Some("AVAILABLE".to_string()));
        }

        #[sqlx::test(migrations = "./migrations", fixtures("test_railway_model"))]
        async fn it_should_insert_locomotive_rolling_stocks(pool: sqlx::SqlitePool) {
            let railway_model_id = RailwayModelId::try_from(TEST_RAILWAY_MODEL_ID)
                .expect("should parse railway model id");
            let mut conn = pool.acquire().await.expect("should acquire connection");

            let params = RollingStockParams::LocomotiveParams {
                railway_company_id: RailwayCompanyId::try_from("trn:railway-company:fs").unwrap(),
                livery: Some("Blue".to_string()),
                length_over_buffers: None,
                technical_specifications: None,
                friendly_name: "Blue Loco".to_string(),
                series_code: Some("SC".to_string()),
                road_number: "RN100".to_string(),
                series: Some("S1".to_string()),
                depot: Some("Depot".to_string()),
                locomotive_type: LocomotiveType::ElectricLocomotive,
                dcc_interface: Some(DccInterface::Nem651),
                control: Some(Control::DccReady),
                is_dummy: false,
            };

            let mut repo = SqliteRailwayModelRepository::new(&mut conn);
            let result = repo.insert_rolling_stock(&railway_model_id, &params).await;

            assert!(result.is_ok(), "should insert rolling stock without errors");

            let row = sqlx::query(ROLLING_STOCK_QUERY)
                .bind(&railway_model_id)
                .fetch_one(&mut *conn)
                .await
                .expect("should fetch one rolling stock row");

            let company: String = row.get("railway_company_id");
            let livery: Option<String> = row.get("livery");
            let series_code: Option<String> = row.get("series_code");
            let series: Option<String> = row.get("series");
            let depot: Option<String> = row.get("depot");
            let friendly_name: String = row.get("friendly_name");
            let road_number: Option<String> = row.get("road_number");
            let locomotive_type: Option<String> = row.get("locomotive_type");
            let is_dummy: i64 = row.get("is_dummy");
            let dcc_interface: Option<String> = row.get("dcc_interface");
            let control: Option<String> = row.get("control");

            assert_eq!(company, "trn:railway-company:fs");
            assert_eq!(series_code, Some("SC".to_string()));
            assert_eq!(series, Some("S1".to_string()));
            assert_eq!(friendly_name, "Blue Loco");
            assert_eq!(depot, Some("Depot".to_string()));
            assert_eq!(road_number, Some("RN100".to_string()));
            assert_eq!(locomotive_type, Some("ELECTRIC_LOCOMOTIVE".to_string()));
            assert_eq!(livery, Some("Blue".to_string()));
            assert_eq!(is_dummy, 0);
            assert_eq!(dcc_interface, Some("NEM_651".to_string()));
            assert_eq!(control, Some("DCC_READY".to_string()));
        }

        #[sqlx::test(migrations = "./migrations", fixtures("test_railway_model"))]
        async fn it_should_insert_freight_car_rolling_stocks(pool: sqlx::SqlitePool) {
            let railway_model_id = RailwayModelId::try_from(TEST_RAILWAY_MODEL_ID)
                .expect("should parse railway model id");
            let mut conn = pool.acquire().await.expect("should acquire connection");

            let params = RollingStockParams::FreightCarParams {
                railway_company_id: RailwayCompanyId::try_from("trn:railway-company:fs").unwrap(),
                livery: Some("Blue".to_string()),
                length_over_buffers: None,
                technical_specifications: None,
                friendly_name: "Blue Loco".to_string(),
                series_code: Some("SC".to_string()),
                road_number: Some("RN100".to_string()),
                series: Some("S1".to_string()),
                freight_car_type: Some(FreightCarType::AutoTransportCars),
            };

            let mut repo = SqliteRailwayModelRepository::new(&mut conn);
            let result = repo.insert_rolling_stock(&railway_model_id, &params).await;

            assert!(result.is_ok(), "should insert rolling stock without errors");

            let row = sqlx::query(ROLLING_STOCK_QUERY)
                .bind(&railway_model_id)
                .fetch_one(&mut *conn)
                .await
                .expect("should fetch one rolling stock row");

            let company: String = row.get("railway_company_id");
            let livery: Option<String> = row.get("livery");
            let series_code: Option<String> = row.get("series_code");
            let series: Option<String> = row.get("series");
            let friendly_name: String = row.get("friendly_name");
            let road_number: Option<String> = row.get("road_number");
            let freight_car_type: Option<String> = row.get("freight_car_type");
            let is_dummy: i64 = row.get("is_dummy");

            assert_eq!(company, "trn:railway-company:fs");
            assert_eq!(series_code, Some("SC".to_string()));
            assert_eq!(series, Some("S1".to_string()));
            assert_eq!(friendly_name, "Blue Loco");
            assert_eq!(road_number, Some("RN100".to_string()));
            assert_eq!(freight_car_type, Some("AUTO_TRANSPORT_CARS".to_string()));
            assert_eq!(livery, Some("Blue".to_string()));
            assert_eq!(is_dummy, 0);
        }

        #[sqlx::test(migrations = "./migrations", fixtures("test_railway_model"))]
        async fn it_should_insert_passenger_car_rolling_stocks(pool: sqlx::SqlitePool) {
            let railway_model_id = RailwayModelId::try_from(TEST_RAILWAY_MODEL_ID)
                .expect("should parse railway model id");
            let mut conn = pool.acquire().await.expect("should acquire connection");

            let params = RollingStockParams::PassengerCarParams {
                railway_company_id: RailwayCompanyId::try_from("trn:railway-company:fs").unwrap(),
                livery: Some("Blue".to_string()),
                length_over_buffers: None,
                technical_specifications: None,
                friendly_name: "Blue Loco".to_string(),
                series_code: Some("SC".to_string()),
                road_number: Some("RN100".to_string()),
                series: Some("S1".to_string()),
                passenger_car_type: Some(PassengerCarType::OpenCoach),
                service_level: Some(ServiceLevel::First),
            };

            let mut repo = SqliteRailwayModelRepository::new(&mut conn);
            let result = repo.insert_rolling_stock(&railway_model_id, &params).await;

            println!("Insert result: {:?}", result);
            assert!(result.is_ok(), "should insert rolling stock without errors");

            let row = sqlx::query(ROLLING_STOCK_QUERY)
                .bind(&railway_model_id)
                .fetch_one(&mut *conn)
                .await
                .expect("should fetch one rolling stock row");

            let company: String = row.get("railway_company_id");
            let livery: Option<String> = row.get("livery");
            let series_code: Option<String> = row.get("series_code");
            let series: Option<String> = row.get("series");
            let friendly_name: String = row.get("friendly_name");
            let road_number: Option<String> = row.get("road_number");
            let passenger_car_type: Option<String> = row.get("passenger_car_type");
            let service_level: Option<String> = row.get("service_level");
            let is_dummy: i64 = row.get("is_dummy");

            assert_eq!(company, "trn:railway-company:fs");
            assert_eq!(series_code, Some("SC".to_string()));
            assert_eq!(series, Some("S1".to_string()));
            assert_eq!(friendly_name, "Blue Loco");
            assert_eq!(road_number, Some("RN100".to_string()));
            assert_eq!(passenger_car_type, Some("OPEN_COACH".to_string()));
            assert_eq!(livery, Some("Blue".to_string()));
            assert_eq!(service_level, Some("FIRST".to_string()));
            assert_eq!(is_dummy, 0);
        }

        #[sqlx::test(migrations = "./migrations", fixtures("test_railway_model"))]
        async fn it_should_insert_railcar_rolling_stocks(pool: sqlx::SqlitePool) {
            let railway_model_id = RailwayModelId::try_from(TEST_RAILWAY_MODEL_ID)
                .expect("should parse railway model id");
            let mut conn = pool.acquire().await.expect("should acquire connection");

            let params = RollingStockParams::RailcarParams {
                railway_company_id: RailwayCompanyId::try_from("trn:railway-company:fs").unwrap(),
                livery: Some("Blue".to_string()),
                length_over_buffers: None,
                technical_specifications: None,
                friendly_name: "Blue Loco".to_string(),
                series_code: Some("SC".to_string()),
                road_number: Some("RN100".to_string()),
                series: Some("S1".to_string()),
                depot: Some("Depot".to_string()),
                railcar_type: RailcarType::PowerCar,
                dcc_interface: Some(DccInterface::Nem651),
                control: Some(Control::DccReady),
                is_dummy: false,
            };

            let mut repo = SqliteRailwayModelRepository::new(&mut conn);
            let result = repo.insert_rolling_stock(&railway_model_id, &params).await;

            assert!(result.is_ok(), "should insert rolling stock without errors");

            let row = sqlx::query(ROLLING_STOCK_QUERY)
                .bind(&railway_model_id)
                .fetch_one(&mut *conn)
                .await
                .expect("should fetch one rolling stock row");

            let company: String = row.get("railway_company_id");
            let livery: Option<String> = row.get("livery");
            let series_code: Option<String> = row.get("series_code");
            let series: Option<String> = row.get("series");
            let depot: Option<String> = row.get("depot");
            let friendly_name: String = row.get("friendly_name");
            let road_number: Option<String> = row.get("road_number");
            let railcar_type: Option<String> = row.get("railcar_type");
            let is_dummy: i64 = row.get("is_dummy");
            let dcc_interface: Option<String> = row.get("dcc_interface");
            let control: Option<String> = row.get("control");

            assert_eq!(company, "trn:railway-company:fs");
            assert_eq!(series_code, Some("SC".to_string()));
            assert_eq!(series, Some("S1".to_string()));
            assert_eq!(friendly_name, "Blue Loco");
            assert_eq!(depot, Some("Depot".to_string()));
            assert_eq!(road_number, Some("RN100".to_string()));
            assert_eq!(railcar_type, Some("POWER_CAR".to_string()));
            assert_eq!(livery, Some("Blue".to_string()));
            assert_eq!(is_dummy, 0);
            assert_eq!(dcc_interface, Some("NEM_651".to_string()));
            assert_eq!(control, Some("DCC_READY".to_string()));
        }

        #[sqlx::test(migrations = "./migrations", fixtures("test_railway_model"))]
        async fn it_should_insert_emu_rolling_stocks(pool: sqlx::SqlitePool) {
            let railway_model_id = RailwayModelId::try_from(TEST_RAILWAY_MODEL_ID)
                .expect("should parse railway model id");
            let mut conn = pool.acquire().await.expect("should acquire connection");

            let params = RollingStockParams::ElectricMultipleUnitParams {
                railway_company_id: RailwayCompanyId::try_from("trn:railway-company:fs").unwrap(),
                livery: Some("Blue".to_string()),
                length_over_buffers: None,
                technical_specifications: None,
                friendly_name: "Blue Loco".to_string(),
                series_code: Some("SC".to_string()),
                road_number: Some("RN100".to_string()),
                series: Some("S1".to_string()),
                depot: Some("Depot".to_string()),
                electric_multiple_unit_type: ElectricMultipleUnitType::DrivingCar,
                dcc_interface: Some(DccInterface::Nem651),
                control: Some(Control::DccReady),
                is_dummy: true,
            };

            let mut repo = SqliteRailwayModelRepository::new(&mut conn);
            let result = repo.insert_rolling_stock(&railway_model_id, &params).await;

            assert!(result.is_ok(), "should insert rolling stock without errors");

            let row = sqlx::query(ROLLING_STOCK_QUERY)
                .bind(&railway_model_id)
                .fetch_one(&mut *conn)
                .await
                .expect("should fetch one rolling stock row");

            let company: String = row.get("railway_company_id");
            let livery: Option<String> = row.get("livery");
            let series_code: Option<String> = row.get("series_code");
            let series: Option<String> = row.get("series");
            let depot: Option<String> = row.get("depot");
            let friendly_name: String = row.get("friendly_name");
            let road_number: Option<String> = row.get("road_number");
            let electric_multiple_unit_type: Option<String> =
                row.get("electric_multiple_unit_type");
            let is_dummy: i64 = row.get("is_dummy");
            let dcc_interface: Option<String> = row.get("dcc_interface");
            let control: Option<String> = row.get("control");

            assert_eq!(company, "trn:railway-company:fs");
            assert_eq!(series_code, Some("SC".to_string()));
            assert_eq!(series, Some("S1".to_string()));
            assert_eq!(friendly_name, "Blue Loco");
            assert_eq!(depot, Some("Depot".to_string()));
            assert_eq!(road_number, Some("RN100".to_string()));
            assert_eq!(electric_multiple_unit_type, Some("DRIVING_CAR".to_string()));
            assert_eq!(livery, Some("Blue".to_string()));
            assert_eq!(is_dummy, 1);
            assert_eq!(dcc_interface, Some("NEM_651".to_string()));
            assert_eq!(control, Some("DCC_READY".to_string()));
        }
    }
}
