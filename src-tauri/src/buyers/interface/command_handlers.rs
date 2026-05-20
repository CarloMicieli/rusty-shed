use crate::buyers::application::merge_buyer::MergeBuyer;
use crate::core::infrastructure::error::CommandError;
use crate::sellers::application::seller_view::SellerView;
use crate::sellers::domain::seller_id::SellerId;
use crate::sellers::interface::command_handlers as sellers_command_handlers;
use crate::sellers::interface::{CreateSellerPayload, Seller, UpdateSellerPayload};
use crate::state::AppState;
use serde::{Deserialize, Serialize};

/// Tauri command that lists all buyers.
#[tauri::command]
#[specta::specta]
pub async fn get_buyers(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<SellerView>, CommandError> {
    sellers_command_handlers::get_sellers(state).await
}

/// Tauri command that returns one buyer by identifier.
#[tauri::command]
#[specta::specta]
pub async fn get_buyer_by_id(
    state: tauri::State<'_, AppState>,
    id: SellerId,
) -> Result<Option<SellerView>, CommandError> {
    sellers_command_handlers::get_seller_by_id(state, id).await
}

/// Tauri command that creates a buyer.
#[tauri::command]
#[specta::specta]
pub async fn create_buyer(
    state: tauri::State<'_, AppState>,
    payload: CreateSellerPayload,
) -> Result<Seller, CommandError> {
    sellers_command_handlers::create_seller(state, payload).await
}

#[tauri::command]
#[specta::specta]
pub async fn update_buyer(
    state: tauri::State<'_, AppState>,
    payload: UpdateSellerPayload,
) -> Result<Seller, CommandError> {
    sellers_command_handlers::update_seller(state, payload).await
}

/// Tauri command that deletes a buyer by identifier.
#[tauri::command]
#[specta::specta]
pub async fn delete_buyer(
    state: tauri::State<'_, AppState>,
    id: SellerId,
) -> Result<(), CommandError> {
    sellers_command_handlers::delete_seller(state, id).await
}

/// Input payload for buyer-merge operations.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct MergeBuyerArgs {
    pub source_id: SellerId,
    pub target_id: SellerId,
}

/// Result payload returned after a successful buyer merge.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct BuyerMergeResult {
    pub source_id: String,
    pub target_id: String,
    pub relinked_count: i64,
}

/// Executes the buyer merge transaction and returns a summary payload.
pub async fn merge_buyers_inner(
    state: &AppState,
    args: MergeBuyerArgs,
) -> Result<BuyerMergeResult, CommandError> {
    let mut tx = state.db_pool().begin().await.map_err(CommandError::from)?;

    let relinked_count = MergeBuyer::execute(&mut tx, &args.source_id, &args.target_id)
        .await
        .map_err(CommandError::from)?;

    tx.commit().await.map_err(CommandError::from)?;

    Ok(BuyerMergeResult {
        source_id: args.source_id.to_string(),
        target_id: args.target_id.to_string(),
        relinked_count,
    })
}

#[tauri::command]
#[specta::specta]
pub async fn merge_buyers(
    state: tauri::State<'_, AppState>,
    args: MergeBuyerArgs,
) -> Result<BuyerMergeResult, CommandError> {
    merge_buyers_inner(&state, args).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use sqlx::SqlitePool;

    fn app_state(pool: SqlitePool) -> AppState {
        AppState::for_test(pool)
    }

    fn merge_args(source_id: &str, target_id: &str) -> MergeBuyerArgs {
        MergeBuyerArgs {
            source_id: SellerId::try_from(source_id).expect("source id should be valid"),
            target_id: SellerId::try_from(target_id).expect("target id should be valid"),
        }
    }

    async fn insert_seller(
        pool: &SqlitePool,
        id: &str,
        name: &str,
        seeded: bool,
    ) -> Result<(), sqlx::Error> {
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            r#"
            INSERT INTO sellers (id, name, type, created_at, updated_at, version, is_system_seeded)
            VALUES (?1, ?2, 'SHOP', ?3, ?4, 1, ?5)
            "#,
        )
        .bind(id)
        .bind(name)
        .bind(&now)
        .bind(&now)
        .bind(if seeded { 1 } else { 0 })
        .execute(pool)
        .await
        .map(|_| ())
    }

    async fn insert_collection_item_graph(
        pool: &SqlitePool,
        manufacturer_id: &str,
        railway_model_id: &str,
        collection_id: &str,
        collection_item_id: &str,
    ) -> Result<(), sqlx::Error> {
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            r#"
            INSERT INTO manufacturers (id, name, created_at, updated_at, version)
            VALUES (?1, ?2, ?3, ?4, 1)
            "#,
        )
        .bind(manufacturer_id)
        .bind("Merge Test Manufacturer")
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;

        sqlx::query(
            r#"
            INSERT INTO railway_models (
                id,
                manufacturer_id,
                product_code,
                power_method,
                scale,
                epoch,
                category,
                created_at,
                updated_at,
                version
            )
            VALUES (?1, ?2, 'MT-001', 'ANALOG_DC', 'H0', 'V', 'LOCOMOTIVE', ?3, ?4, 1)
            "#,
        )
        .bind(railway_model_id)
        .bind(manufacturer_id)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;

        sqlx::query(
            r#"
            INSERT INTO collections (id, name, created_at, updated_at, version)
            VALUES (?1, 'Merge Test Collection', ?2, ?3, 1)
            "#,
        )
        .bind(collection_id)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;

        sqlx::query(
            r#"
            INSERT INTO collection_items (id, collection_id, railway_model_id, added_date)
            VALUES (?1, ?2, ?3, '2026-01-01')
            "#,
        )
        .bind(collection_item_id)
        .bind(collection_id)
        .bind(railway_model_id)
        .execute(pool)
        .await
        .map(|_| ())
    }

    async fn insert_purchase_info(
        pool: &SqlitePool,
        id: &str,
        collection_item_id: &str,
        seller_id: Option<&str>,
        buyer_id: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO purchase_infos (id, collection_item_id, purchase_date, seller_id, buyer_id)
            VALUES (?1, ?2, '2026-01-02', ?3, ?4)
            "#,
        )
        .bind(id)
        .bind(collection_item_id)
        .bind(seller_id)
        .bind(buyer_id)
        .execute(pool)
        .await
        .map(|_| ())
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn merge_buyers_inner_relinks_seller_and_buyer_and_deletes_source(pool: SqlitePool) {
        let state = app_state(pool.clone());
        let source_id = "trn:seller:merge-source";
        let target_id = "trn:seller:merge-target";
        let manufacturer_id = "trn:manufacturer:merge-tests";
        let railway_model_id = "trn:railway-model:merge-tests";
        let collection_id = "trn:collection:merge-tests";
        let collection_item_id = "trn:collection-item:merge-tests";

        insert_seller(&pool, source_id, "Merge Source", false)
            .await
            .expect("source should insert");
        insert_seller(&pool, target_id, "Merge Target", false)
            .await
            .expect("target should insert");
        insert_collection_item_graph(
            &pool,
            manufacturer_id,
            railway_model_id,
            collection_id,
            collection_item_id,
        )
        .await
        .expect("purchase graph should insert");
        insert_purchase_info(
            &pool,
            "trn:purchase:merge-seller-ref",
            collection_item_id,
            Some(source_id),
            None,
        )
        .await
        .expect("seller purchase info should insert");
        insert_purchase_info(
            &pool,
            "trn:purchase:merge-buyer-ref",
            collection_item_id,
            None,
            Some(source_id),
        )
        .await
        .expect("buyer purchase info should insert");

        let result = merge_buyers_inner(&state, merge_args(source_id, target_id))
            .await
            .expect("merge should succeed");

        assert_eq!(result.source_id, source_id);
        assert_eq!(result.target_id, target_id);
        assert_eq!(result.relinked_count, 2);

        let source_exists =
            sqlx::query_scalar::<_, i64>("SELECT COUNT(1) FROM sellers WHERE id = ?1")
                .bind(source_id)
                .fetch_one(&pool)
                .await
                .expect("source count query should work");
        assert_eq!(source_exists, 0, "source seller should be deleted");

        let source_references = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(1) FROM purchase_infos WHERE seller_id = ?1 OR buyer_id = ?1",
        )
        .bind(source_id)
        .fetch_one(&pool)
        .await
        .expect("source reference count query should work");
        assert_eq!(source_references, 0, "source references should be relinked");

        let target_references = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(1) FROM purchase_infos WHERE seller_id = ?1 OR buyer_id = ?1",
        )
        .bind(target_id)
        .fetch_one(&pool)
        .await
        .expect("target reference count query should work");
        assert_eq!(
            target_references, 2,
            "target should receive both references"
        );
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn merge_buyers_inner_same_id_returns_business_rule(pool: SqlitePool) {
        let state = app_state(pool);
        let shared_id = "trn:seller:merge-shared";

        let result = merge_buyers_inner(&state, merge_args(shared_id, shared_id)).await;

        assert!(
            matches!(result, Err(CommandError::BusinessRule(_))),
            "Expected BusinessRule, got: {:?}",
            result
        );
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn merge_buyers_inner_missing_source_returns_not_found(pool: SqlitePool) {
        let state = app_state(pool.clone());
        let source_id = "trn:seller:missing-source";
        let target_id = "trn:seller:existing-target";

        insert_seller(&pool, target_id, "Existing Target", false)
            .await
            .expect("target should insert");

        let result = merge_buyers_inner(&state, merge_args(source_id, target_id)).await;

        assert!(
            matches!(result, Err(CommandError::NotFound(_))),
            "Expected NotFound, got: {:?}",
            result
        );
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn merge_buyers_inner_missing_target_returns_not_found(pool: SqlitePool) {
        let state = app_state(pool.clone());
        let source_id = "trn:seller:existing-source";
        let target_id = "trn:seller:missing-target";

        insert_seller(&pool, source_id, "Existing Source", false)
            .await
            .expect("source should insert");

        let result = merge_buyers_inner(&state, merge_args(source_id, target_id)).await;

        assert!(
            matches!(result, Err(CommandError::NotFound(_))),
            "Expected NotFound, got: {:?}",
            result
        );
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn merge_buyers_inner_seeded_entity_returns_business_rule(pool: SqlitePool) {
        let state = app_state(pool.clone());
        let source_id = "trn:seller:seeded-source";
        let target_id = "trn:seller:normal-target";

        insert_seller(&pool, source_id, "Seeded Source", true)
            .await
            .expect("seeded source should insert");
        insert_seller(&pool, target_id, "Normal Target", false)
            .await
            .expect("target should insert");

        let result = merge_buyers_inner(&state, merge_args(source_id, target_id)).await;

        assert!(
            matches!(result, Err(CommandError::BusinessRule(_))),
            "Expected BusinessRule, got: {:?}",
            result
        );
    }
}
