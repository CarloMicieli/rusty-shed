use crate::catalog::domain::railway_model::RailwayModelEvent;
use crate::catalog::domain::railway_model::railway_model_translation::{
    RailwayModelTranslationEntry, RailwayModelTranslations,
};
use crate::catalog::domain::railway_model::{
    RailwayModel, RailwayModelId, RailwayModelParams, RailwayModelRepository, RailwayModelUowExt,
    RollingStock, RollingStockCategory, RollingStockId, RollingStockParams,
};
use crate::catalog::domain::railway_model::{
    RailwayModelView, RollingStockRailway, RollingStockView,
};
use crate::catalog::domain::scale::Scale;
use crate::catalog::infrastructure::entities::{RailwayModelRow, RollingStockRow};
use crate::core::domain::{domain_error::DomainError, Language};
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::search::infrastructure::rebuild_search_index;
use chrono::TimeZone;
use sqlx::Row;
use sqlx::SqliteConnection;
use uuid::Uuid;

/// An SQLite-specific implementation of the `RailwayModelRepository`.
///
/// It holds a mutable reference to a connection, which in this architecture
/// is provided by the `SqliteUnitOfWork`'s active transaction.
pub struct SqliteRailwayModelRepository<'conn> {
    /// A mutable reference to the database connection/executor.
    executor: &'conn mut SqliteConnection,
}

impl<'conn> SqliteRailwayModelRepository<'conn> {
    /// Creates a new instance of the `SqliteRailwayModelRepository`.
    ///
    /// # Arguments
    /// * `executor` - A mutable reference to the database connection/executor.
    ///
    /// # Returns
    /// A new `SqliteRailwayModelRepository` instance.
    pub fn new(executor: &'conn mut SqliteConnection) -> Self {
        Self { executor }
    }

    /// Fetch a railway model row by its ID with language-aware text resolution.
    ///
    /// Uses COALESCE double-LEFT-JOIN to resolve description/details in `lang`,
    /// falling back to the English translation when the requested language is absent.
    async fn select_railway_model_by_id(
        &mut self,
        id: &RailwayModelId,
        lang: &str,
    ) -> Result<Option<RailwayModelRow>, DomainError> {
        let sql = r#"
            SELECT
                rm.id,
                rm.manufacturer_id,
                m.name AS manufacturer_name,
                rm.product_code,
                COALESCE(t_req.language_code, t_en.language_code, 'en') AS resolved_lang,
                COALESCE(t_req.description,  t_en.description)          AS description,
                COALESCE(t_req.details,      t_en.details)               AS details,
                rm.power_method,
                rm.scale,
                rm.epoch,
                rm.category,
                rm.delivery_date,
                rm.availability_status,
                rm.created_at,
                rm.updated_at,
                rm.version
            FROM railway_models AS rm
            JOIN manufacturers AS m ON m.id = rm.manufacturer_id
            LEFT JOIN railway_model_translations AS t_req
                ON t_req.railway_model_id = rm.id AND t_req.language_code = ?1
            LEFT JOIN railway_model_translations AS t_en
                ON t_en.railway_model_id = rm.id AND t_en.language_code = 'en'
            WHERE rm.id = ?2
            LIMIT 1"#;

        let row = sqlx::query_as::<_, RailwayModelRow>(sql)
            .bind(lang)
            .bind(id)
            .fetch_optional(&mut *self.executor)
            .await
            .map_err(DomainError::from)?;

        Ok(row)
    }

    /// Fetch rolling stocks for a given railway model id.
    async fn select_rolling_stocks_by_id(
        &mut self,
        railway_model_id: &RailwayModelId,
    ) -> Result<Vec<RollingStockRow>, DomainError> {
        let sql = r#"
            SELECT 
                rs.id, 
                rs.railway_model_id, 
                rs.category, 
                rs.railway_company_id,
                rc.name as railway_company_name, 
                rs.livery, 
                rs.length_inches, 
                rs.length_millimeters, 
                rs.technical_minimum_radius_mm, 
                rs.technical_coupling_socket, 
                rs.technical_coupling_close_couplers, 
                rs.technical_coupling_digital_shunting, 
                rs.technical_flywheel_fitted, 
                rs.technical_body_shell, 
                rs.technical_chassis, 
                rs.technical_interior_lights, 
                rs.technical_lights, 
                rs.technical_sprung_buffers, 
                rs.series_code, 
                rs.friendly_name, 
                rs.road_number, 
                rs.series, 
                rs.depot, 
                rs.electric_multiple_unit_type, 
                rs.freight_car_type, 
                rs.locomotive_type, 
                rs.passenger_car_type, 
                rs.railcar_type, 
                rs.service_level, 
                rs.dcc_interface, 
                rs.control, 
                rs.is_dummy 
            FROM rolling_stocks AS rs
            JOIN railway_companies AS rc ON rs.railway_company_id = rc.id
            WHERE rs.railway_model_id = ?1"#;

        let rows = sqlx::query_as::<_, RollingStockRow>(sql)
            .bind(railway_model_id)
            .fetch_all(&mut *self.executor)
            .await
            .map_err(DomainError::from)?;

        Ok(rows)
    }

    /// Apply a minimal patch update to a rolling stock row from a `changed` JSON object.
    ///
    /// Detects the patch type from the keys present in `changed`:
    /// - Only `railway_company_id` → railway company patch (1 column)
    /// - `series_code` but no `flywheel_fitted` → identification patch (4 columns)
    /// - `series_code` with `flywheel_fitted` → full specifications patch (14 columns)
    async fn update_rolling_stock_from_patch(
        &mut self,
        rolling_stock_id: &RollingStockId,
        changed: &serde_json::Value,
    ) -> Result<(), DomainError> {
        let map = match changed {
            serde_json::Value::Object(m) => m,
            _ => return Ok(()),
        };

        if map.contains_key("railway_company_id") {
            // Railway company patch
            let company_id = map
                .get("railway_company_id")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            sqlx::query("UPDATE rolling_stocks SET railway_company_id = ?1 WHERE id = ?2")
                .bind(company_id)
                .bind(rolling_stock_id)
                .execute(&mut *self.executor)
                .await
                .map_err(DomainError::from)?;
        } else if map.contains_key("series_code") && map.contains_key("flywheel_fitted") {
            // Full specifications patch (14 columns)
            let series_code = map
                .get("series_code")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let road_number = map.get("road_number").and_then(|v| v.as_str());
            let livery = map.get("livery").and_then(|v| v.as_str());
            let depot = map.get("depot").and_then(|v| v.as_str());
            let flywheel = map.get("flywheel_fitted").and_then(|v| v.as_str());
            let body_shell = map.get("body_shell").and_then(|v| v.as_str());
            let chassis = map.get("chassis").and_then(|v| v.as_str());
            let interior_lights = map.get("interior_lights").and_then(|v| v.as_str());
            let lights = map.get("lights").and_then(|v| v.as_str());
            let dcc_interface = map.get("dcc_interface").and_then(|v| v.as_str());
            let control = map.get("control").and_then(|v| v.as_str());
            let coupling_socket = map.get("coupling_socket").and_then(|v| v.as_str());
            let close_couplers = map.get("close_couplers").and_then(|v| v.as_str());
            let digital_shunting = map.get("digital_shunting").and_then(|v| v.as_str());

            sqlx::query(
                r#"
                UPDATE rolling_stocks
                SET series_code = ?1,
                    road_number = ?2,
                    livery = ?3,
                    depot = ?4,
                    technical_flywheel_fitted = ?5,
                    technical_body_shell = ?6,
                    technical_chassis = ?7,
                    technical_interior_lights = ?8,
                    technical_lights = ?9,
                    dcc_interface = ?10,
                    control = ?11,
                    technical_coupling_socket = ?12,
                    technical_coupling_close_couplers = ?13,
                    technical_coupling_digital_shunting = ?14
                WHERE id = ?15
            "#,
            )
            .bind(series_code)
            .bind(road_number)
            .bind(livery)
            .bind(depot)
            .bind(flywheel)
            .bind(body_shell)
            .bind(chassis)
            .bind(interior_lights)
            .bind(lights)
            .bind(dcc_interface)
            .bind(control)
            .bind(coupling_socket)
            .bind(close_couplers)
            .bind(digital_shunting)
            .bind(rolling_stock_id)
            .execute(&mut *self.executor)
            .await
            .map_err(DomainError::from)?;
        } else if map.contains_key("series_code") {
            // Identification patch (4 columns)
            let series_code = map
                .get("series_code")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let road_number = map.get("road_number").and_then(|v| v.as_str());
            let livery = map.get("livery").and_then(|v| v.as_str());
            let depot = map.get("depot").and_then(|v| v.as_str());

            sqlx::query(
                r#"
                UPDATE rolling_stocks
                SET series_code = ?1,
                    road_number = ?2,
                    livery = ?3,
                    depot = ?4
                WHERE id = ?5
            "#,
            )
            .bind(series_code)
            .bind(road_number)
            .bind(livery)
            .bind(depot)
            .bind(rolling_stock_id)
            .execute(&mut *self.executor)
            .await
            .map_err(DomainError::from)?;
        }

        Ok(())
    }

    /// Inserts a new railway model into the database.
    async fn insert_railway_model(
        &mut self,
        railway_model: &RailwayModelParams,
    ) -> Result<RailwayModelId, DomainError> {
        let insert_cmd = r#"
        INSERT INTO railway_models (
            id, manufacturer_id, product_code,
            power_method, scale, epoch, category, delivery_date, availability_status, created_at, updated_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
        "#;

        let id = RailwayModelId::new(&railway_model.manufacturer_id, &railway_model.product_code)
            .map_err(|e| DomainError::Validation(e.to_string()))?;

        sqlx::query(insert_cmd)
            .bind(&id)
            .bind(&railway_model.manufacturer_id)
            .bind(railway_model.product_code.to_string())
            .bind(railway_model.power_method.to_string())
            .bind(&railway_model.scale)
            .bind(&railway_model.epoch.0)
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

        // Insert the EN translation for description/details
        sqlx::query(
            r#"INSERT INTO railway_model_translations (railway_model_id, language_code, description, details)
               VALUES (?1, 'en', ?2, ?3)"#,
        )
        .bind(&id)
        .bind(&railway_model.description)
        .bind(&railway_model.details)
        .execute(&mut *self.executor)
        .await
        .map_err(DomainError::from)?;

        Ok(id)
    }

    /// Inserts a new rolling stock into the database.
    async fn insert_rolling_stock(
        &mut self,
        railway_model_id: &RailwayModelId,
        rolling_stock: &RollingStockParams,
    ) -> Result<(), DomainError> {
        // Bind parameters and execute the insert command here...
        let id = RollingStockId::from_uuid(&Uuid::new_v4());

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

    async fn find_by_id(
        &mut self,
        id: &RailwayModelId,
        lang: &str,
    ) -> Result<Option<RailwayModel>, DomainError> {
        let row_opt = self.select_railway_model_by_id(id, lang).await?;

        if let Some(row) = row_opt {
            let mut rm = RailwayModel::try_from(row)?;

            // fetch rolling stocks
            let child_rows = self.select_rolling_stocks_by_id(id).await?;

            let mut rolling_stocks = Vec::with_capacity(child_rows.len());
            for cr in child_rows {
                let rs = RollingStock::try_from(cr)?;
                rolling_stocks.push(rs);
            }

            rm.rolling_stocks = rolling_stocks;

            Ok(Some(rm))
        } else {
            Ok(None)
        }
    }

    async fn find_view_by_id(
        &mut self,
        id: &RailwayModelId,
        lang: &str,
    ) -> Result<Option<RailwayModelView>, DomainError> {
        let row_opt = self.select_railway_model_by_id(id, lang).await?;

        if let Some(row) = row_opt {
            // fetch rolling stocks
            let child_rows = self.select_rolling_stocks_by_id(id).await?;

            let mut rolling_stock_views = Vec::with_capacity(child_rows.len());
            for cr in child_rows {
                let railway = RollingStockRailway {
                    railway_company_id: cr.railway_company_id.clone(),
                    display: cr.railway_company_name.clone(),
                };

                let view = match cr.category {
                    RollingStockCategory::Locomotive => RollingStockView::Locomotive {
                        id: cr.id,
                        railway,
                        livery: cr.livery,
                        length_over_buffer: None,
                        technical_specifications: None,
                        friendly_name: cr.friendly_name,
                        series_code: cr.series_code,
                        road_number: cr.road_number,
                        series: cr.series,
                        depot: cr.depot,
                        locomotive_type: cr.locomotive_type.unwrap_or_default(),
                        dcc_interface: cr.dcc_interface,
                        control: cr.control,
                        is_dummy: cr.is_dummy,
                    },
                    RollingStockCategory::FreightCar => RollingStockView::FreightCar {
                        id: cr.id,
                        railway,
                        livery: cr.livery,
                        length_over_buffer: None,
                        technical_specifications: None,
                        friendly_name: cr.friendly_name,
                        series_code: cr.series_code,
                        road_number: cr.road_number,
                        freight_car_type: cr.freight_car_type,
                    },
                    RollingStockCategory::PassengerCar => RollingStockView::PassengerCar {
                        id: cr.id,
                        railway,
                        livery: cr.livery,
                        length_over_buffer: None,
                        technical_specifications: None,
                        friendly_name: cr.friendly_name,
                        series_code: cr.series_code,
                        road_number: cr.road_number,
                        series: cr.series,
                        passenger_car_type: cr.passenger_car_type,
                        service_level: cr.service_level,
                    },
                    RollingStockCategory::ElectricMultipleUnit => {
                        RollingStockView::ElectricMultipleUnit {
                            id: cr.id,
                            railway,
                            livery: cr.livery,
                            length_over_buffer: None,
                            technical_specifications: None,
                            friendly_name: cr.friendly_name,
                            series_code: cr.series_code,
                            road_number: cr.road_number,
                            series: cr.series,
                            depot: cr.depot,
                            electric_multiple_unit_type: cr
                                .electric_multiple_unit_type
                                .unwrap_or_default(),
                            dcc_interface: cr.dcc_interface,
                            control: cr.control,
                            is_dummy: cr.is_dummy,
                        }
                    }
                    RollingStockCategory::Railcar => RollingStockView::Railcar {
                        id: cr.id,
                        railway,
                        livery: cr.livery,
                        length_over_buffer: None,
                        technical_specifications: None,
                        friendly_name: cr.friendly_name,
                        series_code: cr.series_code,
                        road_number: cr.road_number,
                        series: cr.series,
                        depot: cr.depot,
                        railcar_type: cr.railcar_type.unwrap_or_default(),
                        dcc_interface: cr.dcc_interface,
                        control: cr.control,
                        is_dummy: cr.is_dummy,
                    },
                };

                rolling_stock_views.push(view);
            }

            let manufacturer = crate::catalog::domain::railway_model::RailwayModelManufacturer {
                manufacturer_id: row.manufacturer_id,
                display: row.manufacturer_name,
            };

            let metadata = crate::core::domain::metadata::Metadata {
                version: row.version as u8,
                created_at: chrono::Utc.from_utc_datetime(&row.created_at),
                updated_at: chrono::Utc.from_utc_datetime(&row.updated_at),
            };

            let lang = Language::try_from(row.resolved_lang.as_str())
                .unwrap_or(Language::English);

            let details_lang = row.details.as_ref().map(|_| lang);

            let view = RailwayModelView {
                id: row.id,
                manufacturer,
                product_code: row.product_code,
                description: row.description.unwrap_or_default(),
                description_lang: lang,
                details: row.details,
                details_lang,
                power_method: row.power_method,
                scale: row.scale,
                epoch: row.epoch,
                category: row.category,
                delivery_date: row.delivery_date,
                availability_status: row.availability_status,
                rolling_stock: rolling_stock_views,
                metadata,
            };

            Ok(Some(view))
        } else {
            Ok(None)
        }
    }

    async fn save(&mut self, aggregate: &mut RailwayModel) -> Result<(), DomainError> {
        // Pull pending events from the aggregate and apply them to the DB using
        // the same executor (the UnitOfWork provides the transaction/executor).
        let events = aggregate.pull_events();

        for ev in events.into_iter() {
            match ev {
                RailwayModelEvent::RailwayModelCreated { params, .. } => {
                    // Reuse existing helpers which bind to the repository's executor.
                    let _id = self.insert_railway_model(&params).await?;
                    for rs in params.rolling_stocks.iter() {
                        self.insert_rolling_stock(&_id, rs).await?;
                    }
                }
                RailwayModelEvent::RailwayModelUpdated {
                    railway_model_id,
                    changed,
                    ..
                } => {
                    // Apply a minimal set of updates for common fields. The
                    // `changed` payload is expected to be a JSON object.
                    if let serde_json::Value::Object(map) = changed {
                        if let Some(serde_json::Value::String(availability)) =
                            map.get("availability_status")
                        {
                            let update_cmd = r#"UPDATE railway_models SET availability_status = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2;"#;
                            sqlx::query(update_cmd)
                                .bind(availability)
                                .bind(&railway_model_id)
                                .execute(&mut *self.executor)
                                .await
                                .map_err(DomainError::from)?;
                        }

                        if let Some(serde_json::Value::String(scale_str)) = map.get("scale") {
                            let scale = Scale::try_from(scale_str.as_str())
                                .map_err(|e| DomainError::Validation(e.to_string()))?;
                            let update_cmd = r#"UPDATE railway_models SET scale = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2;"#;
                            sqlx::query(update_cmd)
                                .bind(&scale)
                                .bind(&railway_model_id)
                                .execute(&mut *self.executor)
                                .await
                                .map_err(DomainError::from)?;
                        }

                        if let Some(serde_json::Value::String(epoch)) = map.get("epoch") {
                            let update_cmd = r#"UPDATE railway_models SET epoch = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2;"#;
                            sqlx::query(update_cmd)
                                .bind(epoch)
                                .bind(&railway_model_id)
                                .execute(&mut *self.executor)
                                .await
                                .map_err(DomainError::from)?;
                        }
                    }
                }
                RailwayModelEvent::TranslationUpserted {
                    railway_model_id,
                    lang,
                    description,
                    details,
                    ..
                } => {
                    // When both fields are None, delete the translation row.
                    // Otherwise upsert using INSERT OR REPLACE.
                    let lang_str = lang.to_string();
                    let desc_empty = description.as_deref().map(|s| s.is_empty()).unwrap_or(true);
                    let details_empty = details.as_deref().map(|s| s.is_empty()).unwrap_or(true);

                    if description.is_none() && details.is_none() {
                        // No-op: neither field was changed (only one side of upsert_translation fired)
                    } else if desc_empty && details_empty {
                        // Both cleared — delete the translation row
                        sqlx::query(
                            r#"DELETE FROM railway_model_translations
                               WHERE railway_model_id = ?1 AND language_code = ?2"#,
                        )
                        .bind(&railway_model_id)
                        .bind(&lang_str)
                        .execute(&mut *self.executor)
                        .await
                        .map_err(DomainError::from)?;
                    } else {
                        // Upsert translation — INSERT OR REPLACE keeps FTS triggers firing
                        sqlx::query(
                            r#"INSERT INTO railway_model_translations
                                   (railway_model_id, language_code, description, details, updated_at)
                               VALUES (?1, ?2, ?3, ?4, CURRENT_TIMESTAMP)
                               ON CONFLICT(railway_model_id, language_code)
                               DO UPDATE SET
                                   description = CASE WHEN ?3 IS NOT NULL THEN ?3 ELSE description END,
                                   details     = CASE WHEN ?4 IS NOT NULL THEN ?4 ELSE details END,
                                   updated_at  = CURRENT_TIMESTAMP"#,
                        )
                        .bind(&railway_model_id)
                        .bind(&lang_str)
                        .bind(&description)
                        .bind(&details)
                        .execute(&mut *self.executor)
                        .await
                        .map_err(DomainError::from)?;
                    }
                }
                RailwayModelEvent::RollingStockAdded {
                    railway_model_id,
                    rolling_stock_params,
                    ..
                } => {
                    // Insert a new rolling stock for the given railway model.
                    self.insert_rolling_stock(&railway_model_id, &rolling_stock_params)
                        .await?;
                }
                RailwayModelEvent::RollingStockRemoved {
                    rolling_stock_id, ..
                } => {
                    let delete_cmd = r#"DELETE FROM rolling_stocks WHERE id = ?1;"#;
                    sqlx::query(delete_cmd)
                        .bind(rolling_stock_id)
                        .execute(&mut *self.executor)
                        .await
                        .map_err(DomainError::from)?;
                }
                RailwayModelEvent::RollingStockUpdated {
                    rolling_stock_id,
                    changed,
                    ..
                } => {
                    self.update_rolling_stock_from_patch(&rolling_stock_id, &changed)
                        .await?;
                }
            }
        }

        // Rebuild the FTS5 search index for this model within the same transaction.
        // This keeps the full-text search index in sync with all domain mutations applied above.
        rebuild_search_index(aggregate.id.as_ref(), self.executor).await?;

        Ok(())
    }

    async fn find_translations(
        &mut self,
        id: &RailwayModelId,
    ) -> Result<Option<RailwayModelTranslations>, DomainError> {
        // Check if the model exists first
        let exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM railway_models WHERE id = ?1)")
                .bind(id)
                .fetch_one(&mut *self.executor)
                .await
                .map_err(DomainError::from)?;

        if !exists {
            return Ok(None);
        }

        let rows = sqlx::query(
            "SELECT language_code, description, details FROM railway_model_translations WHERE railway_model_id = ?1",
        )
        .bind(id)
        .fetch_all(&mut *self.executor)
        .await
        .map_err(DomainError::from)?;

        let mut en: Option<RailwayModelTranslationEntry> = None;
        let mut it: Option<RailwayModelTranslationEntry> = None;

        for row in rows {
            let lang_code: String = row.get("language_code");
            let description: Option<String> = row.get("description");
            let details: Option<String> = row.get("details");
            let entry = RailwayModelTranslationEntry {
                description,
                details,
            };
            match lang_code.as_str() {
                "en" => en = Some(entry),
                "it" => it = Some(entry),
                _ => {}
            }
        }

        Ok(Some(RailwayModelTranslations {
            railway_model_id: id.clone(),
            en,
            it,
        }))
    }

    async fn search(&mut self, query: &str) -> Result<Vec<RailwayModelId>, DomainError> {
        // Use runtime query (not query!) for FTS5 MATCH — see research R-001
        let rows = sqlx::query(
            r#"SELECT DISTINCT railway_model_id
               FROM railway_model_search_idx
               WHERE railway_model_search_idx MATCH ?1
               ORDER BY rank
               LIMIT 200"#,
        )
        .bind(query)
        .fetch_all(&mut *self.executor)
        .await
        .map_err(DomainError::from)?;

        let ids = rows
            .into_iter()
            .map(|row| {
                let id_str: String = row.get("railway_model_id");
                RailwayModelId::try_from(id_str.as_str())
                    .map_err(|e| DomainError::Validation(e.to_string()))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(ids)
    }
}

impl<'conn> RailwayModelUowExt for SqliteUnitOfWork<'conn> {
    /// Links the SQLite-specific repository to the Unit of Work.
    ///
    /// It re-borrows the internal transaction (`&mut *self.tx`) to provide
    /// the repository with a mutable executor without transferring ownership.
    fn railway_model_repository(&mut self) -> Box<dyn RailwayModelRepository + '_> {
        Box::new(SqliteRailwayModelRepository::new(&mut self.tx))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::domain::manufacturer::ManufacturerId;
    use crate::catalog::domain::railway_company::RailwayCompanyId;
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

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../../fixtures/test_railway_model.sql")
    )]
    async fn it_should_insert_railway_models(pool: sqlx::SqlitePool) {
        let mut conn = pool.acquire().await.expect("should acquire connection");

        let params = RailwayModelParams {
            manufacturer_id: ManufacturerId::try_from("trn:manufacturer:acme").unwrap(),
            product_code: ProductCode::try_from("9999").unwrap(),
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
        let scale: String = row.get("scale");
        let power_method: String = row.get("power_method");
        let epoch: String = row.get("epoch");
        let category: String = row.get("category");
        let delivery_date: Option<String> = row.get("delivery_date");
        let availability_status: Option<String> = row.get("availability_status");

        // Verify EN translation was inserted into railway_model_translations
        let translation_row = sqlx::query(
            "SELECT description FROM railway_model_translations WHERE railway_model_id = ?1 AND language_code = 'en'",
        )
        .bind(&railway_model_id)
        .fetch_one(&mut *conn)
        .await
        .expect("should find EN translation");
        let description: String = translation_row.get("description");

        assert_eq!(manufacturer_id, "trn:manufacturer:acme");
        assert_eq!(product_code, "9999");
        assert_eq!(description, "Test Model");
        assert_eq!(scale, "H0"); // SQLx stores the enum variant name, not Display format
        assert_eq!(power_method, "DC");
        assert_eq!(epoch, "IV");
        assert_eq!(category, "LOCOMOTIVES");
        assert_eq!(delivery_date, Some("2023".to_string()));
        assert_eq!(availability_status, Some("AVAILABLE".to_string()));
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../../fixtures/test_railway_model.sql")
    )]
    async fn it_should_insert_locomotive_rolling_stocks(pool: sqlx::SqlitePool) {
        let railway_model_id =
            RailwayModelId::try_from(TEST_RAILWAY_MODEL_ID).expect("should parse railway model id");
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

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../../fixtures/test_railway_model.sql")
    )]
    async fn it_should_insert_freight_car_rolling_stocks(pool: sqlx::SqlitePool) {
        let railway_model_id =
            RailwayModelId::try_from(TEST_RAILWAY_MODEL_ID).expect("should parse railway model id");
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

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../../fixtures/test_railway_model.sql")
    )]
    async fn it_should_insert_passenger_car_rolling_stocks(pool: sqlx::SqlitePool) {
        let railway_model_id =
            RailwayModelId::try_from(TEST_RAILWAY_MODEL_ID).expect("should parse railway model id");
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

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../../fixtures/test_railway_model.sql")
    )]
    async fn it_should_insert_railcar_rolling_stocks(pool: sqlx::SqlitePool) {
        let railway_model_id =
            RailwayModelId::try_from(TEST_RAILWAY_MODEL_ID).expect("should parse railway model id");
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

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../../fixtures/test_railway_model.sql")
    )]
    async fn it_should_insert_emu_rolling_stocks(pool: sqlx::SqlitePool) {
        let railway_model_id =
            RailwayModelId::try_from(TEST_RAILWAY_MODEL_ID).expect("should parse railway model id");
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
        let electric_multiple_unit_type: Option<String> = row.get("electric_multiple_unit_type");
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

/*
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

*/
