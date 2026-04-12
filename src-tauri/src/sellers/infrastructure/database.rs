use crate::sellers::domain::seller_type::SellerType;
use crate::sellers::infrastructure::entities::SellerRow;

/// Fetch every seller row, ordered alphabetically by name.
pub async fn list_sellers(
    executor: &mut sqlx::SqliteConnection,
) -> Result<Vec<SellerRow>, sqlx::Error> {
    let sql = r#"
        SELECT
            id,
            name,
            type AS seller_type,
            email,
            phone,
            website_url,
            street_address,
            extended_address,
            city,
            state_region,
            postal_code,
            country_code,
            created_at,
            updated_at,
            version
        FROM sellers
        ORDER BY name
    "#;

    sqlx::query_as::<_, SellerRow>(sql)
        .fetch_all(executor)
        .await
}

/// Find a single seller row by its string identifier.
///
/// Returns `None` when no row with that `id` exists.
pub async fn find_seller_by_id(
    executor: &mut sqlx::SqliteConnection,
    id: &str,
) -> Result<Option<SellerRow>, sqlx::Error> {
    let sql = r#"
        SELECT
            id,
            name,
            type AS seller_type,
            email,
            phone,
            website_url,
            street_address,
            extended_address,
            city,
            state_region,
            postal_code,
            country_code,
            created_at,
            updated_at,
            version
        FROM sellers
        WHERE id = ?
    "#;

    sqlx::query_as::<_, SellerRow>(sql)
        .bind(id)
        .fetch_optional(executor)
        .await
}

/// Return only the `created_at` timestamp string for a seller, or `None` if not found.
///
/// Used during upsert to preserve the original creation time.
pub async fn get_seller_created_at(
    executor: &mut sqlx::SqliteConnection,
    id: &str,
) -> Result<Option<String>, sqlx::Error> {
    sqlx::query_scalar("SELECT created_at FROM sellers WHERE id = ?")
        .bind(id)
        .fetch_optional(executor)
        .await
}

/// Insert or update a seller row using SQLite's `ON CONFLICT` upsert.
///
/// The `created_at` column is **never** overwritten on conflict — callers are
/// responsible for passing the correct preserved value (see
/// [`get_seller_created_at`]).
#[allow(clippy::too_many_arguments)]
pub async fn upsert_seller(
    executor: &mut sqlx::SqliteConnection,
    id: &str,
    name: &str,
    seller_type: &SellerType,
    email: Option<&str>,
    phone: Option<&str>,
    website_url: Option<&str>,
    street_address: Option<&str>,
    extended_address: Option<&str>,
    city: Option<&str>,
    state_region: Option<&str>,
    postal_code: Option<&str>,
    country_code: Option<&str>,
    created_at: &str,
    updated_at: &str,
) -> Result<(), sqlx::Error> {
    let sql = r#"
        INSERT INTO sellers (
            id, name, type, email, phone, website_url,
            street_address, extended_address, city, state_region, postal_code, country_code,
            created_at, updated_at
        ) VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?)
        ON CONFLICT(id) DO UPDATE SET
            name             = excluded.name,
            type             = excluded.type,
            email            = excluded.email,
            phone            = excluded.phone,
            website_url      = excluded.website_url,
            street_address   = excluded.street_address,
            extended_address = excluded.extended_address,
            city             = excluded.city,
            state_region     = excluded.state_region,
            postal_code      = excluded.postal_code,
            country_code     = excluded.country_code,
            updated_at       = excluded.updated_at,
            version          = version + 1
    "#;

    sqlx::query(sql)
        .bind(id)
        .bind(name)
        .bind(seller_type)
        .bind(email)
        .bind(phone)
        .bind(website_url)
        .bind(street_address)
        .bind(extended_address)
        .bind(city)
        .bind(state_region)
        .bind(postal_code)
        .bind(country_code)
        .bind(created_at)
        .bind(updated_at)
        .execute(executor)
        .await?;

    Ok(())
}

/// Delete a seller by id, returning the number of rows affected.
pub async fn delete_seller(
    executor: &mut sqlx::SqliteConnection,
    id: &str,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("DELETE FROM sellers WHERE id = ?")
        .bind(id)
        .execute(executor)
        .await?;

    Ok(res.rows_affected())
}
