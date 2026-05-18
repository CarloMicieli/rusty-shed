/// Shared usage-count helpers for entity-management flows.
pub async fn manufacturer_usage_count(
    executor: &mut sqlx::SqliteConnection,
    manufacturer_id: &str,
) -> sqlx::Result<i64> {
    sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)
        FROM railway_models
        WHERE manufacturer_id = ?1
        "#,
    )
    .bind(manufacturer_id)
    .fetch_one(executor)
    .await
}

pub async fn canonical_party_usage_count(
    executor: &mut sqlx::SqliteConnection,
    party_id: &str,
) -> sqlx::Result<i64> {
    sqlx::query_scalar::<_, i64>(
        r#"
        SELECT
            (
                SELECT COUNT(*) FROM purchase_infos WHERE seller_id = ?1
            ) + (
                SELECT COUNT(*) FROM purchase_infos WHERE buyer_id = ?1
            )
        "#,
    )
    .bind(party_id)
    .fetch_one(executor)
    .await
}
