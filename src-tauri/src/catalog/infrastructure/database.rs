use crate::catalog::infrastructure::entities::ManufacturerRow;
use sqlx::sqlite::SqliteConnection;

/// Fetch a manufacturer row by its ID.
///
/// This function executes a simple SELECT query against the `manufacturers` table
/// and returns the matching `ManufacturerRow` if present.
///
/// # Arguments
///
/// * `executor` - A mutable reference to an open `SqliteConnection` used to run the query.
/// * `id` - The manufacturer identifier to look up.
///
/// # Returns
///
/// Returns `Ok(Some(ManufacturerRow))` when a matching row is found, `Ok(None)` when no
/// row matches the provided `id`, or `Err(sqlx::Error)` if the query fails.
///
/// # Errors
///
/// Errors from `sqlx` (preparing or executing the query) are propagated to the caller.
pub async fn get_manufacturer_by_id(
    executor: &mut SqliteConnection,
    id: &str,
) -> Result<Option<ManufacturerRow>, sqlx::Error> {
    let sql = "SELECT id, name, registered_company_name, status, country_code, created_at, updated_at FROM manufacturers WHERE id = ?1 LIMIT 1";

    let row = sqlx::query_as::<_, ManufacturerRow>(sql)
        .bind(id)
        .fetch_optional(&mut *executor)
        .await?;

    Ok(row)
}
