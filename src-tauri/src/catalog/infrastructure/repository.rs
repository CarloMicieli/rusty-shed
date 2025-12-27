use super::database;
use crate::catalog::domain::manufacturer::Manufacturer;
use crate::catalog::domain::manufacturer_id::ManufacturerId;
use crate::catalog::domain::railway_company::RailwayCompany;
use crate::catalog::domain::railway_company_id::RailwayCompanyId;
use crate::catalog::domain::railway_model_id::RailwayModelId;
use crate::catalog::domain::{RailwayModel, RollingStock};
use anyhow::Context;
use sqlx::sqlite::SqliteConnection;

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
            let res = get_railway_model_by_id(
                &mut *conn,
                &crate::catalog::domain::railway_model_id::RailwayModelId::try_from("RM-1")
                    .unwrap(),
            )
            .await
            .expect("query failed");
            assert!(res.is_some());
            let rm = res.unwrap();
            assert_eq!(&*rm.id, "RM-1");
            assert_eq!(rm.rolling_stocks.len(), 0);

            // insert a rolling stock
            let rs_id = uuid::Uuid::new_v4().to_string();
            let insert_rs = format!(
                "INSERT INTO rolling_stocks (id, railway_model_id, category, railway_company_id, type_name, class_name, road_number, locomotive_type, is_dummy) VALUES ('{}','RM-1','LOCOMOTIVE','RC-1','T','C','1','DIESEL_LOCOMOTIVE',0)",
                rs_id
            );
            conn.execute(insert_rs.as_str())
                .await
                .expect("insert rolling stock");

            // Case 2: with rolling stocks
            let res2 = get_railway_model_by_id(
                &mut *conn,
                &crate::catalog::domain::railway_model_id::RailwayModelId::try_from("RM-1")
                    .unwrap(),
            )
            .await
            .expect("query failed");
            assert!(res2.is_some());
            let rm2 = res2.unwrap();
            assert_eq!(rm2.rolling_stocks.len(), 1);
        }
    }
}
