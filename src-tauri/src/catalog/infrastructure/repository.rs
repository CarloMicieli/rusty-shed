use super::database;
use crate::catalog::application::create_railway_model::CreateRollingStockInput;
use crate::catalog::domain::manufacturer::Manufacturer;
use crate::catalog::domain::manufacturer_id::ManufacturerId;
use crate::catalog::domain::railway_company::RailwayCompany;
use crate::catalog::domain::railway_company_id::RailwayCompanyId;
use crate::catalog::domain::railway_model_id::RailwayModelId;
use crate::catalog::domain::rolling_stock_id::RollingStockId;
use crate::catalog::domain::{RailwayModel, RollingStock};
use anyhow::Context;
use sqlx::sqlite::SqliteConnection;
use std::collections::HashMap;

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
    let row_opt = database::get_manufacturer_by_id(executor, &id.to_string())
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

/// Insert a new railway model into the database.
pub async fn insert_railway_model(
    executor: &mut SqliteConnection,
    railway_model: &RailwayModel,
) -> anyhow::Result<()> {
    let sql = "INSERT INTO railway_models (id, manufacturer_id, product_code, description, details, \
        power_method, scale, epoch, category, delivery_date, availability_status, created_at, updated_at) \
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)";

    sqlx::query(sql)
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
        .execute(&mut *executor)
        .await
        .context("inserting railway model")?;

    Ok(())
}

/// Insert a new rolling stock into the database.
pub async fn insert_rolling_stock(
    executor: &mut SqliteConnection,
    railway_model_id: &RailwayModelId,
    rolling_stock_id: &RollingStockId,
    input: CreateRollingStockInput,
) -> anyhow::Result<()> {
    let sql = "INSERT INTO rolling_stocks (id, railway_model_id, category, railway_company_id, livery, \
        length_inches, length_millimeters, technical_minimum_radius_mm, technical_coupling, \
        technical_flywheel_fitted, technical_body_shell, technical_chassis, technical_interior_lights, \
        technical_lights, technical_sprung_buffers, series_code, friendly_name, road_number, series, depot, \
        electric_multiple_unit_type, freight_car_type, locomotive_type, passenger_car_type, railcar_type, \
        service_level, dcc_interface, control, is_dummy) \
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29)";

    let (
        category,
        friendly_name,
        series_code,
        road_number,
        series,
        depot,
        livery,
        locomotive_type,
        passenger_car_type,
        freight_car_type,
        electric_multiple_unit_type,
        service_level,
        control,
        dcc_interface,
        is_dummy,
        length_inches,
        length_millimeters,
        minimum_radius,
        coupling,
        flywheel_fitted,
        body_shell,
        chassis,
        interior_lights,
        lights,
        sprung_buffers,
        railway_company_id,
    ) = match input {
        CreateRollingStockInput::Locomotive {
            railway_company_id,
            friendly_name,
            series_code,
            road_number,
            series,
            depot,
            livery,
            locomotive_type,
            is_dummy,
            control,
            dcc_interface,
            length_over_buffers,
            technical_specifications,
        } => (
            "LOCOMOTIVE",
            Some(friendly_name),
            series_code,
            Some(road_number),
            series,
            depot,
            livery,
            Some(locomotive_type),
            None,
            None,
            None,
            None,
            control,
            dcc_interface,
            is_dummy.unwrap_or(false),
            length_over_buffers.as_ref().and_then(|l| l.inches),
            length_over_buffers.as_ref().and_then(|l| l.millimeters),
            technical_specifications
                .as_ref()
                .and_then(|t| t.minimum_radius),
            technical_specifications.as_ref().and_then(|t| {
                t.coupling
                    .as_ref()
                    .map(|c| format!("{{\"socket\":\"{}\"}}", c.socket))
            }),
            technical_specifications
                .as_ref()
                .and_then(|t| t.flywheel_fitted.clone()),
            technical_specifications
                .as_ref()
                .and_then(|t| t.body_shell.clone()),
            technical_specifications
                .as_ref()
                .and_then(|t| t.chassis.clone()),
            technical_specifications
                .as_ref()
                .and_then(|t| t.interior_lights.clone()),
            technical_specifications
                .as_ref()
                .and_then(|t| t.lights.clone()),
            technical_specifications
                .as_ref()
                .and_then(|t| t.sprung_buffers.clone()),
            railway_company_id,
        ),
        CreateRollingStockInput::PassengerCar {
            railway_company_id,
            friendly_name,
            series_code,
            road_number,
            series,
            depot,
            livery,
            passenger_car_type,
            service_level,
            length_over_buffers,
            technical_specifications,
        } => (
            "PASSENGER_CAR",
            Some(friendly_name),
            series_code,
            road_number,
            series,
            depot,
            livery,
            None,
            Some(passenger_car_type),
            None,
            None,
            service_level,
            None,
            None,
            false,
            length_over_buffers.as_ref().and_then(|l| l.inches),
            length_over_buffers.as_ref().and_then(|l| l.millimeters),
            technical_specifications
                .as_ref()
                .and_then(|t| t.minimum_radius),
            technical_specifications.as_ref().and_then(|t| {
                t.coupling
                    .as_ref()
                    .map(|c| format!("{{\"socket\":\"{}\"}}", c.socket))
            }),
            technical_specifications
                .as_ref()
                .and_then(|t| t.flywheel_fitted.clone()),
            technical_specifications
                .as_ref()
                .and_then(|t| t.body_shell.clone()),
            technical_specifications
                .as_ref()
                .and_then(|t| t.chassis.clone()),
            technical_specifications
                .as_ref()
                .and_then(|t| t.interior_lights.clone()),
            technical_specifications
                .as_ref()
                .and_then(|t| t.lights.clone()),
            technical_specifications
                .as_ref()
                .and_then(|t| t.sprung_buffers.clone()),
            railway_company_id,
        ),
        CreateRollingStockInput::FreightCar {
            railway_company_id,
            friendly_name,
            series_code,
            road_number,
            series,
            depot,
            livery,
            freight_car_type,
            length_over_buffers,
            technical_specifications,
        } => (
            "FREIGHT_CAR",
            Some(friendly_name),
            series_code,
            road_number,
            series,
            depot,
            livery,
            None,
            None,
            freight_car_type,
            None,
            None,
            None,
            None,
            false,
            length_over_buffers.as_ref().and_then(|l| l.inches),
            length_over_buffers.as_ref().and_then(|l| l.millimeters),
            technical_specifications
                .as_ref()
                .and_then(|t| t.minimum_radius),
            technical_specifications.as_ref().and_then(|t| {
                t.coupling
                    .as_ref()
                    .map(|c| format!("{{\"socket\":\"{}\"}}", c.socket))
            }),
            technical_specifications
                .as_ref()
                .and_then(|t| t.flywheel_fitted.clone()),
            technical_specifications
                .as_ref()
                .and_then(|t| t.body_shell.clone()),
            technical_specifications
                .as_ref()
                .and_then(|t| t.chassis.clone()),
            technical_specifications
                .as_ref()
                .and_then(|t| t.interior_lights.clone()),
            technical_specifications
                .as_ref()
                .and_then(|t| t.lights.clone()),
            technical_specifications
                .as_ref()
                .and_then(|t| t.sprung_buffers.clone()),
            railway_company_id,
        ),
        CreateRollingStockInput::Railcar {
            railway_company_id,
            friendly_name,
            series_code,
            road_number,
            series,
            depot,
            livery,
            control,
            dcc_interface,
            length_over_buffers,
            technical_specifications,
        } => (
            "RAILCAR",
            Some(friendly_name),
            series_code,
            road_number,
            series,
            depot,
            livery,
            None,
            None,
            None,
            None,
            None,
            control,
            dcc_interface,
            false,
            length_over_buffers.as_ref().and_then(|l| l.inches),
            length_over_buffers.as_ref().and_then(|l| l.millimeters),
            technical_specifications
                .as_ref()
                .and_then(|t| t.minimum_radius),
            technical_specifications.as_ref().and_then(|t| {
                t.coupling
                    .as_ref()
                    .map(|c| format!("{{\"socket\":\"{}\"}}", c.socket))
            }),
            technical_specifications
                .as_ref()
                .and_then(|t| t.flywheel_fitted.clone()),
            technical_specifications
                .as_ref()
                .and_then(|t| t.body_shell.clone()),
            technical_specifications
                .as_ref()
                .and_then(|t| t.chassis.clone()),
            technical_specifications
                .as_ref()
                .and_then(|t| t.interior_lights.clone()),
            technical_specifications
                .as_ref()
                .and_then(|t| t.lights.clone()),
            technical_specifications
                .as_ref()
                .and_then(|t| t.sprung_buffers.clone()),
            railway_company_id,
        ),
        CreateRollingStockInput::ElectricMultipleUnit {
            railway_company_id,
            friendly_name,
            series_code,
            road_number,
            series,
            depot,
            livery,
            electric_multiple_unit_type,
            is_dummy,
            control,
            dcc_interface,
            length_over_buffers,
            technical_specifications,
        } => (
            "ELECTRIC_MULTIPLE_UNIT",
            Some(friendly_name),
            series_code,
            road_number,
            series,
            depot,
            livery,
            None,
            None,
            None,
            Some(electric_multiple_unit_type),
            None,
            control,
            dcc_interface,
            is_dummy.unwrap_or(false),
            length_over_buffers.as_ref().and_then(|l| l.inches),
            length_over_buffers.as_ref().and_then(|l| l.millimeters),
            technical_specifications
                .as_ref()
                .and_then(|t| t.minimum_radius),
            technical_specifications.as_ref().and_then(|t| {
                t.coupling
                    .as_ref()
                    .map(|c| format!("{{\"socket\":\"{}\"}}", c.socket))
            }),
            technical_specifications
                .as_ref()
                .and_then(|t| t.flywheel_fitted.clone()),
            technical_specifications
                .as_ref()
                .and_then(|t| t.body_shell.clone()),
            technical_specifications
                .as_ref()
                .and_then(|t| t.chassis.clone()),
            technical_specifications
                .as_ref()
                .and_then(|t| t.interior_lights.clone()),
            technical_specifications
                .as_ref()
                .and_then(|t| t.lights.clone()),
            technical_specifications
                .as_ref()
                .and_then(|t| t.sprung_buffers.clone()),
            railway_company_id,
        ),
    };

    sqlx::query(sql)
        .bind(rolling_stock_id.to_string())
        .bind(railway_model_id.to_string())
        .bind(category)
        .bind(railway_company_id)
        .bind(livery)
        .bind(length_inches)
        .bind(length_millimeters)
        .bind(minimum_radius)
        .bind(coupling)
        .bind(flywheel_fitted)
        .bind(body_shell)
        .bind(chassis)
        .bind(interior_lights)
        .bind(lights)
        .bind(sprung_buffers)
        .bind(series_code)
        .bind(friendly_name)
        .bind(road_number)
        .bind(series)
        .bind(depot)
        .bind(electric_multiple_unit_type)
        .bind(freight_car_type)
        .bind(locomotive_type)
        .bind(passenger_car_type)
        .bind(None::<String>) // railcar_type placeholder
        .bind(service_level)
        .bind(dcc_interface)
        .bind(control)
        .bind(is_dummy as i32)
        .execute(&mut *executor)
        .await
        .context("inserting rolling stock")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    mod manufacturer_repo_tests {
        use super::*;
        use crate::catalog::domain::manufacturer_id::ManufacturerId;
        use pretty_assertions::assert_eq;

        #[sqlx::test(migrations = "./migrations", fixtures("test_manufacturer"))]
        async fn repo_gets_manufacturer_by_id(pool: sqlx::SqlitePool) {
            let mut conn = pool.acquire().await.expect("acquire conn");

            let res = get_manufacturer_by_id(&mut conn, &ManufacturerId::try_from("MN-1").unwrap())
                .await
                .expect("query should run");

            assert!(res.is_some());
            let m = res.unwrap();
            assert_eq!(&*m.id, "MN-1");
            assert_eq!(m.name, "ACME Models");
        }
    }

    mod railway_repo_tests {
        use super::*;
        use crate::catalog::domain::railway_company_id::RailwayCompanyId;
        use pretty_assertions::assert_eq;

        #[sqlx::test(migrations = "./migrations", fixtures("test_railway_company"))]
        async fn repo_gets_railway_by_id(pool: sqlx::SqlitePool) {
            let mut conn = pool.acquire().await.expect("acquire conn");

            let res =
                get_railway_company_by_id(&mut conn, &RailwayCompanyId::try_from("RC-1").unwrap())
                    .await
                    .expect("query should run");

            assert!(res.is_some());
            let r = res.unwrap();
            assert_eq!(&*r.id, "RC-1");
            assert_eq!(r.name, "Ferrovie dello Stato");
        }
    }

    mod railway_model_repo_tests {
        use super::*;
        use pretty_assertions::assert_eq;
        use sqlx::{Executor, SqlitePool};

        async fn setup_db() -> SqlitePool {
            let pool = SqlitePool::connect("sqlite::memory:")
                .await
                .expect("create pool");
            let mut conn = pool.acquire().await.expect("acquire conn");

            // create required tables (minimal subset)
            let schema = include_str!(
                "../../../migrations/0001_create_railway_models_and_rolling_stocks.sql"
            );
            conn.execute(schema).await.expect("create schema");

            pool
        }

        #[tokio::test]
        async fn repo_gets_railway_model_with_and_without_children() {
            let pool = setup_db().await;
            let mut conn = pool.acquire().await.expect("acquire conn");

            // insert manufacturer required by FK
            conn.execute("INSERT INTO manufacturers (id, name, status, created_at, updated_at) VALUES ('MN-1','ACME','ACTIVE',CURRENT_TIMESTAMP,CURRENT_TIMESTAMP)")
                .await
                .expect("insert manufacturer");

            // insert railway company required by FK
            conn.execute("INSERT INTO railway_companies (id, name, created_at, updated_at) VALUES ('RC-1','RC',CURRENT_TIMESTAMP,CURRENT_TIMESTAMP)")
                .await
                .expect("insert railway company");

            // insert a railway model
            conn.execute("INSERT INTO railway_models (id, manufacturer_id, product_code, description, power_method, scale, epoch, category, created_at, updated_at) VALUES ('RM-1','MN-1','ACME-100','Test', 'DC', 'H0', 'III', 'LOCOMOTIVES', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
                .await
                .expect("insert railway model");

            // Case 1: no rolling stocks -> should return model with empty list
            let res =
                get_railway_model_by_id(&mut conn, &RailwayModelId::try_from("RM-1").unwrap())
                    .await
                    .expect("query failed");
            assert!(res.is_some());
            let rm = res.unwrap();
            assert_eq!(&*rm.id, "RM-1");
            assert_eq!(rm.rolling_stocks.len(), 0);

            // insert a rolling stock
            let rs_id = uuid::Uuid::new_v4().to_string();
            let insert_rs = format!(
                "INSERT INTO rolling_stocks (id, railway_model_id, category, railway_company_id, series_code, friendly_name, road_number, locomotive_type, is_dummy) VALUES ('{}','RM-1','LOCOMOTIVE','RC-1','123','Class X','1','DIESEL_LOCOMOTIVE',0)",
                rs_id
            );
            conn.execute(insert_rs.as_str())
                .await
                .expect("insert rolling stock");

            // Case 2: with rolling stocks
            let res2 =
                get_railway_model_by_id(&mut conn, &RailwayModelId::try_from("RM-1").unwrap())
                    .await
                    .expect("query failed");
            assert!(res2.is_some());
            let rm2 = res2.unwrap();
            assert_eq!(rm2.rolling_stocks.len(), 1);
        }
    }
}
