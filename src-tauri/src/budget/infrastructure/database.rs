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

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test(migrations = "./migrations")]
    async fn get_budget_config_returns_none_when_not_configured(pool: sqlx::SqlitePool) {
        let mut conn = pool.acquire().await.expect("acquire connection");

        let config = get_budget_config(&mut conn)
            .await
            .expect("query should succeed");

        assert!(config.is_none());
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn save_budget_config_upserts_existing_row(pool: sqlx::SqlitePool) {
        let mut conn = pool.acquire().await.expect("acquire connection");

        save_budget_config(
            &mut conn,
            1,
            "MONTHLY",
            100_000,
            "EUR",
            2026,
            "2026-01-01T00:00:00Z",
            "2026-01-01T00:00:00Z",
            0,
        )
        .await
        .expect("initial save should succeed");

        save_budget_config(
            &mut conn,
            1,
            "YEARLY",
            1_200_000,
            "USD",
            2027,
            "2026-01-01T00:00:00Z",
            "2027-01-01T00:00:00Z",
            1,
        )
        .await
        .expect("upsert should succeed");

        let row = get_budget_config(&mut conn)
            .await
            .expect("query should succeed")
            .expect("config row should exist");

        assert_eq!(row.mode, "YEARLY");
        assert_eq!(row.base_amount, 1_200_000);
        assert_eq!(row.currency, "USD");
        assert_eq!(row.last_reset_year, 2027);
        assert_eq!(row.version, 1);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn get_extra_budgets_filters_by_year_and_orders_by_month(pool: sqlx::SqlitePool) {
        let mut conn = pool.acquire().await.expect("acquire connection");

        add_extra_budget(
            &mut conn,
            "trn:extra-budget:11111111-1111-1111-1111-111111111111",
            2026,
            11,
            1000,
            "EUR",
            Some("November"),
            "2026-11-01T00:00:00Z",
            0,
        )
        .await
        .expect("insert november extra budget");

        add_extra_budget(
            &mut conn,
            "trn:extra-budget:22222222-2222-2222-2222-222222222222",
            2026,
            2,
            2000,
            "EUR",
            Some("February"),
            "2026-02-01T00:00:00Z",
            0,
        )
        .await
        .expect("insert february extra budget");

        add_extra_budget(
            &mut conn,
            "trn:extra-budget:33333333-3333-3333-3333-333333333333",
            2025,
            12,
            3000,
            "EUR",
            Some("Previous year"),
            "2025-12-01T00:00:00Z",
            0,
        )
        .await
        .expect("insert previous year extra budget");

        let rows = get_extra_budgets(&mut conn, 2026)
            .await
            .expect("query should succeed");

        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].month, 2);
        assert_eq!(rows[1].month, 11);
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_collection.sql")
    )]
    async fn get_multi_year_spending_respects_range_and_currency(pool: sqlx::SqlitePool) {
        let mut conn = pool.acquire().await.expect("acquire connection");

        sqlx::query(
            "INSERT INTO purchase_infos (
                id, collection_item_id, purchase_type, purchase_date, purchased_price_amount, purchased_price_currency
             ) VALUES (?1, ?2, 'PURCHASED', ?3, ?4, ?5)",
        )
        .bind("trn:purchase:aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa")
        .bind("trn:collection-item:d20a1a95-1ae4-4970-9e87-b4c84676e730")
        .bind("2024-01-15")
        .bind(5000_i64)
        .bind("EUR")
        .execute(&mut *conn)
        .await
        .expect("insert 2024 EUR purchase info");

        sqlx::query(
            "INSERT INTO purchase_infos (
                id, collection_item_id, purchase_type, purchase_date, purchased_price_amount, purchased_price_currency
             ) VALUES (?1, ?2, 'PURCHASED', ?3, ?4, ?5)",
        )
        .bind("trn:purchase:bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb")
        .bind("trn:collection-item:d20a1a95-1ae4-4970-9e87-b4c84676e730")
        .bind("2023-01-15")
        .bind(7000_i64)
        .bind("USD")
        .execute(&mut *conn)
        .await
        .expect("insert out-of-range USD purchase info");

        let eur_rows = get_multi_year_monthly_spending(&mut conn, 2024, 2025, "EUR")
            .await
            .expect("multi year query should succeed");

        assert_eq!(eur_rows, vec![(2024, 1, 5000), (2025, 12, 17_500)]);

        let usd_rows = get_multi_year_monthly_spending(&mut conn, 2024, 2025, "USD")
            .await
            .expect("currency-filtered query should succeed");

        assert!(usd_rows.is_empty());
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_collection.sql")
    )]
    async fn get_quarterly_spending_handles_boundaries_and_category_aggregation(
        pool: sqlx::SqlitePool,
    ) {
        let mut conn = pool.acquire().await.expect("acquire connection");

        sqlx::query(
            "INSERT INTO collection_items (id, collection_id, railway_model_id, added_date)
             VALUES (?1, ?2, ?3, ?4)",
        )
        .bind("trn:collection-item:aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa")
        .bind("trn:collection:1")
        .bind("trn:railway-model:rivarossi:hr4315")
        .bind("2025-01-01")
        .execute(&mut *conn)
        .await
        .expect("insert passenger collection item");

        let rows_to_insert = vec![
            // Q1 boundary (March) for locomotives
            (
                "trn:purchase:11111111-1111-1111-1111-111111111111",
                "trn:collection-item:d20a1a95-1ae4-4970-9e87-b4c84676e730",
                "2025-03-10",
                100_i64,
                "EUR",
            ),
            // Q2 boundaries (April + June) for locomotives
            (
                "trn:purchase:22222222-2222-2222-2222-222222222222",
                "trn:collection-item:d20a1a95-1ae4-4970-9e87-b4c84676e730",
                "2025-04-10",
                200_i64,
                "EUR",
            ),
            (
                "trn:purchase:33333333-3333-3333-3333-333333333333",
                "trn:collection-item:d20a1a95-1ae4-4970-9e87-b4c84676e730",
                "2025-06-10",
                300_i64,
                "EUR",
            ),
            // Q3 boundary (July) for locomotives
            (
                "trn:purchase:44444444-4444-4444-4444-444444444444",
                "trn:collection-item:d20a1a95-1ae4-4970-9e87-b4c84676e730",
                "2025-07-10",
                400_i64,
                "EUR",
            ),
            // Q4 boundary (October) for locomotives
            (
                "trn:purchase:55555555-5555-5555-5555-555555555555",
                "trn:collection-item:d20a1a95-1ae4-4970-9e87-b4c84676e730",
                "2025-10-10",
                500_i64,
                "EUR",
            ),
            // Q2 passenger car category (same quarter different category)
            (
                "trn:purchase:66666666-6666-6666-6666-666666666666",
                "trn:collection-item:aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa",
                "2025-06-11",
                600_i64,
                "EUR",
            ),
            // Different year should be excluded
            (
                "trn:purchase:77777777-7777-7777-7777-777777777777",
                "trn:collection-item:d20a1a95-1ae4-4970-9e87-b4c84676e730",
                "2024-06-11",
                900_i64,
                "EUR",
            ),
            // Different currency should be excluded
            (
                "trn:purchase:88888888-8888-8888-8888-888888888888",
                "trn:collection-item:d20a1a95-1ae4-4970-9e87-b4c84676e730",
                "2025-06-12",
                999_i64,
                "USD",
            ),
        ];

        for (id, item_id, date, amount, currency) in rows_to_insert {
            sqlx::query(
                "INSERT INTO purchase_infos (
                    id, collection_item_id, purchase_type, purchase_date, purchased_price_amount, purchased_price_currency
                 ) VALUES (?1, ?2, 'PURCHASED', ?3, ?4, ?5)",
            )
            .bind(id)
            .bind(item_id)
            .bind(date)
            .bind(amount)
            .bind(currency)
            .execute(&mut *conn)
            .await
            .expect("insert quarterly purchase info");
        }

        let quarterly = get_quarterly_spending_by_category(&mut conn, 2025, "EUR")
            .await
            .expect("quarterly query should succeed");

        // Fixture contributes 17_500 EUR in December for LOCOMOTIVES (Q4).
        // Added rows contribute:
        // Q1 LOCOMOTIVES = 100
        // Q2 LOCOMOTIVES = 200 + 300 = 500
        // Q2 PASSENGER_CARS = 600
        // Q3 LOCOMOTIVES = 400
        // Q4 LOCOMOTIVES = 500 + 17_500(fixture) = 18_000
        assert!(quarterly.contains(&(1, "LOCOMOTIVES".to_string(), 100_i64)));
        assert!(quarterly.contains(&(2, "LOCOMOTIVES".to_string(), 500_i64)));
        assert!(quarterly.contains(&(2, "PASSENGER_CARS".to_string(), 600_i64)));
        assert!(quarterly.contains(&(3, "LOCOMOTIVES".to_string(), 400_i64)));
        assert!(quarterly.contains(&(4, "LOCOMOTIVES".to_string(), 18_000_i64)));
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_collection.sql")
    )]
    async fn get_quarterly_spending_keeps_unexpected_legacy_category_values(
        pool: sqlx::SqlitePool,
    ) {
        let mut conn = pool.acquire().await.expect("acquire connection");

        // Insert two legacy-category railway models that may exist in old persisted datasets.
        sqlx::query(
            "INSERT INTO railway_models (
                id, manufacturer_id, product_code, power_method, scale, epoch, category, availability_status
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        )
        .bind("trn:railway-model:acme:legacy-1")
        .bind("trn:manufacturer:acme")
        .bind("LEG-1")
        .bind("DC")
        .bind("H0")
        .bind("IV")
        .bind("LEGACY_LOCO")
        .bind("AVAILABLE")
        .execute(&mut *conn)
        .await
        .expect("insert legacy model 1");

        sqlx::query(
            "INSERT INTO railway_models (
                id, manufacturer_id, product_code, power_method, scale, epoch, category, availability_status
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        )
        .bind("trn:railway-model:acme:legacy-2")
        .bind("trn:manufacturer:acme")
        .bind("LEG-2")
        .bind("DC")
        .bind("H0")
        .bind("IV")
        .bind("legacy_loco")
        .bind("AVAILABLE")
        .execute(&mut *conn)
        .await
        .expect("insert legacy model 2");

        sqlx::query(
            "INSERT INTO collection_items (id, collection_id, railway_model_id, added_date)
             VALUES (?1, ?2, ?3, ?4)",
        )
        .bind("trn:collection-item:legacy-11111111-1111-1111-1111-111111111111")
        .bind("trn:collection:1")
        .bind("trn:railway-model:acme:legacy-1")
        .bind("2025-04-01")
        .execute(&mut *conn)
        .await
        .expect("insert collection item for legacy model 1");

        sqlx::query(
            "INSERT INTO collection_items (id, collection_id, railway_model_id, added_date)
             VALUES (?1, ?2, ?3, ?4)",
        )
        .bind("trn:collection-item:legacy-22222222-2222-2222-2222-222222222222")
        .bind("trn:collection:1")
        .bind("trn:railway-model:acme:legacy-2")
        .bind("2025-04-01")
        .execute(&mut *conn)
        .await
        .expect("insert collection item for legacy model 2");

        // Both purchases in Q2/2025, same currency, distinct legacy category strings.
        sqlx::query(
            "INSERT INTO purchase_infos (
                id, collection_item_id, purchase_type, purchase_date, purchased_price_amount, purchased_price_currency
             ) VALUES (?1, ?2, 'PURCHASED', ?3, ?4, ?5)",
        )
        .bind("trn:purchase:legacy-11111111-1111-1111-1111-111111111111")
        .bind("trn:collection-item:legacy-11111111-1111-1111-1111-111111111111")
        .bind("2025-04-12")
        .bind(321_i64)
        .bind("EUR")
        .execute(&mut *conn)
        .await
        .expect("insert purchase for legacy category LEGACY_LOCO");

        sqlx::query(
            "INSERT INTO purchase_infos (
                id, collection_item_id, purchase_type, purchase_date, purchased_price_amount, purchased_price_currency
             ) VALUES (?1, ?2, 'PURCHASED', ?3, ?4, ?5)",
        )
        .bind("trn:purchase:legacy-22222222-2222-2222-2222-222222222222")
        .bind("trn:collection-item:legacy-22222222-2222-2222-2222-222222222222")
        .bind("2025-05-12")
        .bind(654_i64)
        .bind("EUR")
        .execute(&mut *conn)
        .await
        .expect("insert purchase for legacy category legacy_loco");

        let quarterly = get_quarterly_spending_by_category(&mut conn, 2025, "EUR")
            .await
            .expect("quarterly query should succeed");

        // Query should preserve unexpected categories as-is and aggregate per distinct value.
        assert!(quarterly.contains(&(2, "LEGACY_LOCO".to_string(), 321_i64)));
        assert!(quarterly.contains(&(2, "legacy_loco".to_string(), 654_i64)));
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_collection.sql")
    )]
    async fn get_quarterly_spending_orders_legacy_and_normal_categories_in_same_quarter(
        pool: sqlx::SqlitePool,
    ) {
        let mut conn = pool.acquire().await.expect("acquire connection");

        // Legacy category model.
        sqlx::query(
            "INSERT INTO railway_models (
                id, manufacturer_id, product_code, power_method, scale, epoch, category, availability_status
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        )
        .bind("trn:railway-model:acme:legacy-mixed")
        .bind("trn:manufacturer:acme")
        .bind("LEG-MIX")
        .bind("DC")
        .bind("H0")
        .bind("IV")
        .bind("LEGACY_MIXED")
        .bind("AVAILABLE")
        .execute(&mut *conn)
        .await
        .expect("insert legacy mixed model");

        // Three items in the same quarter/currency across legacy + normal categories.
        sqlx::query(
            "INSERT INTO collection_items (id, collection_id, railway_model_id, added_date)
             VALUES (?1, ?2, ?3, ?4)",
        )
        .bind("trn:collection-item:mix-11111111-1111-1111-1111-111111111111")
        .bind("trn:collection:1")
        .bind("trn:railway-model:acme:legacy-mixed")
        .bind("2025-04-01")
        .execute(&mut *conn)
        .await
        .expect("insert legacy collection item");

        sqlx::query(
            "INSERT INTO collection_items (id, collection_id, railway_model_id, added_date)
             VALUES (?1, ?2, ?3, ?4)",
        )
        .bind("trn:collection-item:mix-22222222-2222-2222-2222-222222222222")
        .bind("trn:collection:1")
        .bind("trn:railway-model:acme:60100") // LOCOMOTIVES
        .bind("2025-04-01")
        .execute(&mut *conn)
        .await
        .expect("insert locomotives collection item");

        sqlx::query(
            "INSERT INTO collection_items (id, collection_id, railway_model_id, added_date)
             VALUES (?1, ?2, ?3, ?4)",
        )
        .bind("trn:collection-item:mix-33333333-3333-3333-3333-333333333333")
        .bind("trn:collection:1")
        .bind("trn:railway-model:rivarossi:hr4315") // PASSENGER_CARS
        .bind("2025-04-01")
        .execute(&mut *conn)
        .await
        .expect("insert passenger cars collection item");

        // All in Q2/2025 and EUR.
        sqlx::query(
            "INSERT INTO purchase_infos (
                id, collection_item_id, purchase_type, purchase_date, purchased_price_amount, purchased_price_currency
             ) VALUES (?1, ?2, 'PURCHASED', ?3, ?4, ?5)",
        )
        .bind("trn:purchase:mix-11111111-1111-1111-1111-111111111111")
        .bind("trn:collection-item:mix-11111111-1111-1111-1111-111111111111")
        .bind("2025-04-10")
        .bind(111_i64)
        .bind("EUR")
        .execute(&mut *conn)
        .await
        .expect("insert legacy purchase");

        sqlx::query(
            "INSERT INTO purchase_infos (
                id, collection_item_id, purchase_type, purchase_date, purchased_price_amount, purchased_price_currency
             ) VALUES (?1, ?2, 'PURCHASED', ?3, ?4, ?5)",
        )
        .bind("trn:purchase:mix-22222222-2222-2222-2222-222222222222")
        .bind("trn:collection-item:mix-22222222-2222-2222-2222-222222222222")
        .bind("2025-05-10")
        .bind(222_i64)
        .bind("EUR")
        .execute(&mut *conn)
        .await
        .expect("insert locomotives purchase");

        sqlx::query(
            "INSERT INTO purchase_infos (
                id, collection_item_id, purchase_type, purchase_date, purchased_price_amount, purchased_price_currency
             ) VALUES (?1, ?2, 'PURCHASED', ?3, ?4, ?5)",
        )
        .bind("trn:purchase:mix-33333333-3333-3333-3333-333333333333")
        .bind("trn:collection-item:mix-33333333-3333-3333-3333-333333333333")
        .bind("2025-06-10")
        .bind(333_i64)
        .bind("EUR")
        .execute(&mut *conn)
        .await
        .expect("insert passenger cars purchase");

        let quarterly = get_quarterly_spending_by_category(&mut conn, 2025, "EUR")
            .await
            .expect("quarterly query should succeed");

        let quarter_two_rows: Vec<(i32, String, i64)> = quarterly
            .into_iter()
            .filter(|(quarter, _, _)| *quarter == 2)
            .collect();

        // ORDER BY clause is quarter ASC, category ASC.
        assert_eq!(
            quarter_two_rows,
            vec![
                (2, "LEGACY_MIXED".to_string(), 111_i64),
                (2, "LOCOMOTIVES".to_string(), 222_i64),
                (2, "PASSENGER_CARS".to_string(), 333_i64),
            ]
        );
    }
}
