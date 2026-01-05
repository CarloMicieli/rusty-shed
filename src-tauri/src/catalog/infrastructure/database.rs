use crate::catalog::infrastructure::entities::{RailwayModelRow, RollingStockRow};
use sqlx::sqlite::SqliteConnection;
use sqlx::{QueryBuilder, Sqlite};

/// Fetch a railway model row by its ID.
pub async fn get_railway_model_by_id(
    executor: &mut SqliteConnection,
    id: &str,
) -> Result<Option<RailwayModelRow>, sqlx::Error> {
    let sql = "SELECT id, manufacturer_id, product_code, description, details, power_method, \
        scale, epoch, category, delivery_date, availability_status, created_at, updated_at \
        FROM railway_models WHERE id = ?1 LIMIT 1";

    let row = sqlx::query_as::<_, RailwayModelRow>(sql)
        .bind(id)
        .fetch_optional(&mut *executor)
        .await?;

    Ok(row)
}

/// Fetch rolling stocks for a given railway model id.
pub async fn get_rolling_stocks_by_railway_model_id(
    executor: &mut SqliteConnection,
    railway_model_id: &str,
) -> Result<Vec<RollingStockRow>, sqlx::Error> {
    let sql = "SELECT id, railway_model_id, category, railway_company_id, livery, length_inches, \
        length_millimeters, technical_minimum_radius_mm, technical_coupling_socket, technical_coupling_close_couplers, technical_coupling_digital_shunting, technical_flywheel_fitted, \
        technical_body_shell, technical_chassis, technical_interior_lights, technical_lights, technical_sprung_buffers, \
        series_code, friendly_name, road_number, series, depot, electric_multiple_unit_type, freight_car_type, locomotive_type, \
        passenger_car_type, railcar_type, service_level, dcc_interface, control, is_dummy \
        FROM rolling_stocks WHERE railway_model_id = ?1";

    let rows = sqlx::query_as::<_, RollingStockRow>(sql)
        .bind(railway_model_id)
        .fetch_all(&mut *executor)
        .await?;

    Ok(rows)
}

/// Fetch multiple railway models by their IDs.
pub async fn get_railway_models_by_ids(
    executor: &mut SqliteConnection,
    ids: &[String],
) -> Result<Vec<RailwayModelRow>, sqlx::Error> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }

    let mut qb = QueryBuilder::<Sqlite>::new(
        "SELECT id, manufacturer_id, product_code, description, details, power_method, \
        scale, epoch, category, delivery_date, availability_status, created_at, updated_at \
        FROM railway_models WHERE id IN (",
    );

    qb.push_bind(ids[0].as_str());
    for id in ids.iter().skip(1) {
        qb.push(", ");
        qb.push_bind(id.as_str());
    }
    qb.push(")");

    let query = qb.build_query_as::<RailwayModelRow>();
    let rows = query.fetch_all(&mut *executor).await?;

    Ok(rows)
}

/// Fetch rolling stocks for a set of railway model ids.
pub async fn get_rolling_stocks_by_railway_model_ids(
    executor: &mut SqliteConnection,
    railway_model_ids: &[String],
) -> Result<Vec<RollingStockRow>, sqlx::Error> {
    if railway_model_ids.is_empty() {
        return Ok(Vec::new());
    }

    let mut qb = QueryBuilder::<Sqlite>::new(
        "SELECT id, railway_model_id, category, railway_company_id, livery, length_inches, \
        length_millimeters, technical_minimum_radius_mm, technical_coupling, technical_flywheel_fitted, \
        technical_body_shell, technical_chassis, technical_interior_lights, technical_lights, technical_sprung_buffers, \
        series_code, friendly_name, road_number, series, depot, electric_multiple_unit_type, freight_car_type, locomotive_type, \
        passenger_car_type, railcar_type, service_level, dcc_interface, control, is_dummy \
        FROM rolling_stocks WHERE railway_model_id IN (",
    );

    qb.push_bind(railway_model_ids[0].as_str());
    for id in railway_model_ids.iter().skip(1) {
        qb.push(", ");
        qb.push_bind(id.as_str());
    }
    qb.push(")");

    let query = qb.build_query_as::<RollingStockRow>();
    let rows = query.fetch_all(&mut *executor).await?;

    Ok(rows)
}
