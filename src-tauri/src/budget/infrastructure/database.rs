// Budget Database Queries
// SQL queries for budget_config and extra_budgets tables

use crate::budget::infrastructure::entities::{BudgetConfigRow, ExtraBudgetRow};
use crate::core::domain::domain_error::DomainError;
use sqlx::SqliteConnection;

/// Get the budget configuration (singleton).
pub async fn get_budget_config(
    executor: &mut SqliteConnection,
) -> Result<Option<BudgetConfigRow>, DomainError> {
    let sql = r#"
        SELECT id, mode, base_amount, currency, last_reset_year,
               created_at, updated_at, version
        FROM budget_config
        WHERE id = 1
    "#;

    let row = sqlx::query_as::<_, BudgetConfigRow>(sql)
        .fetch_optional(executor)
        .await?;

    Ok(row)
}

/// Insert or update the budget configuration.
#[allow(clippy::too_many_arguments)]
pub async fn save_budget_config(
    executor: &mut SqliteConnection,
    id: i32,
    mode: &str,
    base_amount: i64,
    currency: &str,
    last_reset_year: i32,
    created_at: &str,
    updated_at: &str,
    version: i32,
) -> Result<(), DomainError> {
    let sql = r#"
        INSERT INTO budget_config (id, mode, base_amount, currency, last_reset_year, 
                                   created_at, updated_at, version)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
        ON CONFLICT(id) DO UPDATE SET
            mode = ?2,
            base_amount = ?3,
            currency = ?4,
            last_reset_year = ?5,
            updated_at = ?7,
            version = ?8
    "#;

    sqlx::query(sql)
        .bind(id)
        .bind(mode)
        .bind(base_amount)
        .bind(currency)
        .bind(last_reset_year)
        .bind(created_at)
        .bind(updated_at)
        .bind(version)
        .execute(executor)
        .await?;

    Ok(())
}

/// Get all extra budget entries for a specific year.
pub async fn get_extra_budgets(
    executor: &mut SqliteConnection,
    year: i32,
) -> Result<Vec<ExtraBudgetRow>, DomainError> {
    let sql = r#"
        SELECT id, year, month, amount, currency, reason, created_at, version
        FROM extra_budgets
        WHERE year = ?1
        ORDER BY month ASC
    "#;

    let rows = sqlx::query_as::<_, ExtraBudgetRow>(sql)
        .bind(year)
        .fetch_all(executor)
        .await?;

    Ok(rows)
}

/// Get a specific extra budget entry by ID.
pub async fn get_extra_budget_by_id(
    executor: &mut SqliteConnection,
    id: &str,
) -> Result<Option<ExtraBudgetRow>, DomainError> {
    let sql = r#"
        SELECT id, year, month, amount, currency, reason, created_at, version
        FROM extra_budgets
        WHERE id = ?1
    "#;

    let row = sqlx::query_as::<_, ExtraBudgetRow>(sql)
        .bind(id)
        .fetch_optional(executor)
        .await?;

    Ok(row)
}

/// Insert a new extra budget entry.
#[allow(clippy::too_many_arguments)]
pub async fn add_extra_budget(
    executor: &mut SqliteConnection,
    id: &str,
    year: i32,
    month: i32,
    amount: i64,
    currency: &str,
    reason: Option<&str>,
    created_at: &str,
    version: i32,
) -> Result<(), DomainError> {
    let sql = r#"
        INSERT INTO extra_budgets (id, year, month, amount, currency, reason, created_at, version)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
    "#;

    sqlx::query(sql)
        .bind(id)
        .bind(year)
        .bind(month)
        .bind(amount)
        .bind(currency)
        .bind(reason)
        .bind(created_at)
        .bind(version)
        .execute(executor)
        .await?;

    Ok(())
}

/// Delete an extra budget entry by ID.
pub async fn remove_extra_budget(
    executor: &mut SqliteConnection,
    id: &str,
) -> Result<(), DomainError> {
    let sql = r#"
        DELETE FROM extra_budgets
        WHERE id = ?1
    "#;

    sqlx::query(sql).bind(id).execute(executor).await?;

    Ok(())
}

/// Get monthly spending for a range of years in a single query.
///
/// Returns `(year, month, total_amount)` triples for all months in `[start_year, end_year]`
/// that have at least one purchase in the given `currency`.  Months with no spending are omitted.
///
/// Prefer this over calling [`get_monthly_spending`] in a loop when multiple years of data
/// are needed at once (e.g., the 5-year heatmap in the budget dashboard).
pub async fn get_multi_year_monthly_spending(
    executor: &mut SqliteConnection,
    start_year: i32,
    end_year: i32,
    currency: &str,
) -> Result<Vec<(i32, i32, i64)>, DomainError> {
    let sql = r#"
        SELECT
            CAST(strftime('%Y', pi.purchase_date) AS INTEGER) AS year,
            CAST(strftime('%m', pi.purchase_date) AS INTEGER) AS month,
            SUM(pi.purchased_price_amount) AS total_amount
        FROM collection_items ci
        JOIN purchase_infos pi ON ci.id = pi.collection_item_id
        WHERE pi.purchase_date IS NOT NULL
            AND CAST(strftime('%Y', pi.purchase_date) AS INTEGER) BETWEEN ?1 AND ?2
            AND pi.purchased_price_currency = ?3
        GROUP BY year, month
        ORDER BY year ASC, month ASC
    "#;

    let rows: Vec<(i32, i32, i64)> = sqlx::query_as(sql)
        .bind(start_year)
        .bind(end_year)
        .bind(currency)
        .fetch_all(executor)
        .await?;

    Ok(rows)
}

/// Get monthly spending aggregated from collection_items.purchase_info.
/// Returns list of (month, total_amount) pairs for a given year.
pub async fn get_monthly_spending(
    executor: &mut SqliteConnection,
    year: i32,
    currency: &str,
) -> Result<Vec<(i32, i64)>, DomainError> {
    let sql = r#"
        SELECT
            CAST(strftime('%m', pi.purchase_date) AS INTEGER) AS month,
            SUM(pi.purchased_price_amount) AS total_amount
        FROM collection_items ci
        JOIN purchase_infos pi ON ci.id = pi.collection_item_id
        WHERE pi.purchase_date IS NOT NULL
            AND strftime('%Y', pi.purchase_date) = ?1
            AND pi.purchased_price_currency = ?2
        GROUP BY month
        ORDER BY month ASC
    "#;

    let rows: Vec<(i32, i64)> = sqlx::query_as(sql)
        .bind(year.to_string())
        .bind(currency)
        .fetch_all(executor)
        .await?;

    Ok(rows)
}

/// Get quarterly spending broken down by category.
/// Returns list of (quarter, category, total_amount) tuples for a given year.
pub async fn get_quarterly_spending_by_category(
    executor: &mut SqliteConnection,
    year: i32,
    currency: &str,
) -> Result<Vec<(i32, String, i64)>, DomainError> {
    let sql = r#"
        SELECT
            CASE
                WHEN CAST(strftime('%m', pi.purchase_date) AS INTEGER) BETWEEN 1 AND 3 THEN 1
                WHEN CAST(strftime('%m', pi.purchase_date) AS INTEGER) BETWEEN 4 AND 6 THEN 2
                WHEN CAST(strftime('%m', pi.purchase_date) AS INTEGER) BETWEEN 7 AND 9 THEN 3
                ELSE 4
            END AS quarter,
            rm.category,
            SUM(pi.purchased_price_amount) AS total_amount
        FROM collection_items ci
        JOIN purchase_infos pi ON ci.id = pi.collection_item_id
        JOIN railway_models rm ON ci.railway_model_id = rm.id
        WHERE pi.purchase_date IS NOT NULL
            AND strftime('%Y', pi.purchase_date) = ?1
            AND pi.purchased_price_currency = ?2
        GROUP BY quarter, rm.category
        ORDER BY quarter ASC, rm.category ASC
    "#;

    let rows: Vec<(i32, String, i64)> = sqlx::query_as(sql)
        .bind(year.to_string())
        .bind(currency)
        .fetch_all(executor)
        .await?;

    Ok(rows)
}
