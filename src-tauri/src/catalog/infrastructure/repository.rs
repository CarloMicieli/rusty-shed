use super::database;
use crate::catalog::domain::manufacturer::Manufacturer;
use crate::catalog::domain::manufacturer_id::ManufacturerId;
use crate::catalog::domain::railway_company::RailwayCompany;
use crate::catalog::domain::railway_company_id::RailwayCompanyId;
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
        let rc = RailwayCompany::try_from(row).map_err(
            |e| {
                anyhow::anyhow!(format!(
                    "mapping RailwayCompanyRow -> RailwayCompany: {}",
                    e
                ))
            },
        )?;
        Ok(Some(rc))
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
}
