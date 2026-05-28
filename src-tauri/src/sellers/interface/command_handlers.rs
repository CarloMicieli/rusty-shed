use crate::core::infrastructure::error::CommandError;
use crate::sellers::application::create_seller::{CreateSeller, CreateSellerInput};
use crate::sellers::application::delete_seller::DeleteSeller;
use crate::sellers::application::delete_seller_with_lock::DeleteSellerWithLock;
use crate::sellers::application::get_seller_by_id::GetSellerById;
use crate::sellers::application::get_sellers::GetSellers;
use crate::sellers::application::merge_seller::MergeSeller;
use crate::sellers::application::seller_view::SellerView;
use crate::sellers::application::update_seller::{UpdateSellerInput, UpdateSellerUseCase};
use crate::sellers::domain::seller_id::SellerId;
use crate::sellers::interface::{CreateSellerPayload, Seller, UpdateSellerPayload};
use crate::state::AppState;
use garde::Validate;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use tracing::info;

pub async fn get_sellers_inner(state: &AppState) -> Result<Vec<SellerView>, CommandError> {
    info!("Fetching all sellers");

    let mut unit_of_work = state.unit_of_work().await?;

    let mut sellers = GetSellers::execute(&mut unit_of_work).await?;

    for seller in &mut sellers {
        let mut repo = unit_of_work.sellers_repository();
        let (_, is_seeded) = repo
            .find_seeded_and_name(&seller.id)
            .await
            .map_err(CommandError::from)?
            .unwrap_or_default();
        let usage_count = repo
            .find_usage_count(&seller.id)
            .await
            .map_err(CommandError::from)?;
        drop(repo);
        seller.is_system_seeded = is_seeded;
        seller.usage_count = usage_count;
    }

    unit_of_work.commit().await?;

    Ok(sellers)
}

/// Command handler to retrieve all sellers.
///
/// This handler constructs the repository and query handler, executes the query
/// asynchronously and returns the list of `Seller` on success. On failure, it
/// converts the error into a `CommandError` preserving the error
/// message for logging/debugging.
///
/// Parameters:
/// * `state`: Tauri-managed application state which provides a database pool.
///
/// Returns:
/// - `Ok(Vec<Seller>)` when retrieval succeeds.
/// - `Err(CommandError)` when the use-case returns an error.
#[tauri::command]
#[specta::specta]
pub async fn get_sellers(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<SellerView>, CommandError> {
    get_sellers_inner(&state).await
}

pub async fn get_seller_by_id_inner(
    state: &AppState,
    id: SellerId,
) -> Result<Option<SellerView>, CommandError> {
    info!("Fetching seller with ID: {}", id);

    let mut unit_of_work = state.unit_of_work().await?;

    let result = GetSellerById::execute(&mut unit_of_work, &id)
        .await
        .map_err(CommandError::from)?;

    let mut result = result;
    if let Some(seller) = result.as_mut() {
        let mut repo = unit_of_work.sellers_repository();
        let (_, is_seeded) = repo
            .find_seeded_and_name(&seller.id)
            .await
            .map_err(CommandError::from)?
            .unwrap_or_default();
        let usage_count = repo
            .find_usage_count(&seller.id)
            .await
            .map_err(CommandError::from)?;
        drop(repo);
        seller.is_system_seeded = is_seeded;
        seller.usage_count = usage_count;
    }

    unit_of_work.commit().await?;

    Ok(result)
}

/// Command handler to retrieve a seller by its identifier.
///
/// This handler constructs the repository and query handler, executes the query
/// asynchronously and returns the `Seller` on success. On failure, it
/// converts the error into a `CommandError` preserving the error
/// message for logging/debugging.
///    
/// Parameters:
/// * `state`: Tauri-managed application state which provides a database pool.
/// * `id`: The identifier of the seller to retrieve.
///         
/// Returns:
/// - `Ok(Some(Seller))` when a matching seller exists,
/// - `Ok(None)` when no matching row is found
/// - `Err(CommandError)` when the ID cannot be parsed or a database error occurs.
#[tauri::command]
#[specta::specta]
pub async fn get_seller_by_id(
    state: tauri::State<'_, AppState>,
    id: SellerId,
) -> Result<Option<SellerView>, CommandError> {
    get_seller_by_id_inner(&state, id).await
}

pub async fn create_seller_inner(
    state: &AppState,
    payload: CreateSellerPayload,
) -> Result<Seller, CommandError> {
    info!("Creating new seller {:?}", payload);

    payload.validate().map_err(CommandError::from)?;

    let mut unit_of_work = state.unit_of_work().await?;

    let input = CreateSellerInput {
        name: payload.name,
        seller_type: payload.seller_type,
        email: payload.email,
        phone: payload.phone,
        website_url: payload.website_url,
        street_address: payload.street_address,
        extended_address: payload.extended_address,
        city: payload.city,
        state_region: payload.state_region,
        postal_code: payload.postal_code,
        country_code: payload.country_code,
    };
    let result = CreateSeller::execute(&mut unit_of_work, input)
        .await
        .map_err(CommandError::from)?;

    unit_of_work.commit().await?;

    Ok(result.into())
}

/// Command handler to create a new seller.
///
/// This handler constructs the repository and command handler, executes the command
/// asynchronously and returns the created `Seller` on success. On failure, it
/// converts the error into a `CommandError` preserving the error
/// message for logging/debugging.
///
// Parameters:
/// * `state`: Tauri-managed application state which provides a database pool.
/// * `payload`: The payload containing new seller information.
///
/// Returns:
/// - `Ok(Seller)` when creation succeeds.
/// - `Err(CommandError)` when the use-case returns an error.
#[tauri::command]
#[specta::specta]
pub async fn create_seller(
    state: tauri::State<'_, AppState>,
    payload: CreateSellerPayload,
) -> Result<Seller, CommandError> {
    create_seller_inner(&state, payload).await
}

pub async fn update_seller_inner(
    state: &AppState,
    payload: UpdateSellerPayload,
) -> Result<Seller, CommandError> {
    info!("Updating seller: {:?}", payload);

    payload.validate().map_err(CommandError::from)?;

    let seller_id = SellerId::try_from(payload.id.as_str())
        .map_err(|error| CommandError::validation_field("id", error.to_string()))?;

    let mut unit_of_work = state.unit_of_work().await?;

    let (current_name, is_system_seeded) = {
        let mut repo = unit_of_work.sellers_repository();
        repo.find_seeded_and_name(&seller_id)
            .await
            .map_err(CommandError::from)?
            .ok_or_else(|| CommandError::NotFound(format!("Seller '{}' not found", seller_id)))?
    };

    if is_system_seeded && current_name.trim() != payload.name.trim() {
        return Err(CommandError::BusinessRule(
            "System-seeded seller names cannot be edited".to_string(),
        ));
    }

    let input = UpdateSellerInput::try_from(payload)?;
    let result = UpdateSellerUseCase::execute(&mut unit_of_work, input)
        .await
        .map_err(CommandError::from)?;

    unit_of_work.commit().await?;

    Ok(result.into())
}

/// Command handler to update an existing seller.
///
/// This handler constructs the repository and command handler, executes the command
/// asynchronously and returns the updated `
/// Seller` on success. On failure, it
/// converts the error into a `CommandError` preserving the error
/// message for logging/debugging.
///
/// Parameters:
/// * `state`: Tauri-managed application state which provides a database pool.
/// * `payload`: The payload containing updated seller information.
///
/// Returns:
/// - `Ok(Seller)` when the update succeeds.
/// - `Err(CommandError)` when the use-case returns an error.
#[tauri::command]
#[specta::specta]
pub async fn update_seller(
    state: tauri::State<'_, AppState>,
    payload: UpdateSellerPayload,
) -> Result<Seller, CommandError> {
    update_seller_inner(&state, payload).await
}

pub async fn delete_seller_inner(state: &AppState, id: SellerId) -> Result<(), CommandError> {
    info!("Deleting seller with ID: {}", id);

    {
        let mut conn = state
            .db_pool()
            .acquire()
            .await
            .map_err(CommandError::from)?;
        DeleteSellerWithLock::ensure_deletable(&mut conn, &id)
            .await
            .map_err(CommandError::from)?;
    }

    let mut unit_of_work = state.unit_of_work().await?;

    let _ = DeleteSeller::execute(&mut unit_of_work, &id)
        .await
        .map_err(CommandError::from)?;

    unit_of_work.commit().await?;

    Ok(())
}

/// Command handler to delete a seller by ID.
///
/// This handler constructs the repository and command handler, executes the command
/// asynchronously and returns the number of deleted records on success. On failure, it
/// converts the error into a `CommandError` preserving the error
/// message for logging/debugging.
///
/// Parameters:
/// * `state`: Tauri-managed application state which provides a database pool.
/// * `id`: The identifier of the seller to delete.
///         
/// Returns:
/// - `Ok(())` when the deletion succeeds.
/// - `Err(CommandError)` when the use-case returns an error.
#[tauri::command]
#[specta::specta]
pub async fn delete_seller(
    state: tauri::State<'_, AppState>,
    id: SellerId,
) -> Result<(), CommandError> {
    delete_seller_inner(&state, id).await
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct MergeSellerArgs {
    pub source_id: SellerId,
    pub target_id: SellerId,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SellerMergeResult {
    pub source_id: String,
    pub target_id: String,
    pub relinked_count: i64,
}

pub async fn merge_sellers_inner(
    state: &AppState,
    args: MergeSellerArgs,
) -> Result<SellerMergeResult, CommandError> {
    let mut tx = state.db_pool().begin().await.map_err(CommandError::from)?;

    let relinked_count = MergeSeller::execute(&mut tx, &args.source_id, &args.target_id)
        .await
        .map_err(CommandError::from)?;

    tx.commit().await.map_err(CommandError::from)?;

    Ok(SellerMergeResult {
        source_id: args.source_id.to_string(),
        target_id: args.target_id.to_string(),
        relinked_count,
    })
}

#[tauri::command]
#[specta::specta]
pub async fn merge_sellers(
    state: tauri::State<'_, AppState>,
    args: MergeSellerArgs,
) -> Result<SellerMergeResult, CommandError> {
    merge_sellers_inner(&state, args).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sellers::domain::seller_type::SellerType;
    use chrono::Utc;
    use sqlx::SqlitePool;

    fn app_state(pool: SqlitePool) -> AppState {
        AppState::for_test(pool)
    }

    fn minimal_payload(name: &str) -> CreateSellerPayload {
        CreateSellerPayload {
            name: name.to_string(),
            seller_type: SellerType::Shop,
            email: None,
            phone: None,
            website_url: None,
            street_address: None,
            extended_address: None,
            city: None,
            state_region: None,
            postal_code: None,
            country_code: None,
        }
    }

    // ── create_seller_inner ──────────────────────────────────────────────

    #[sqlx::test(migrations = "./migrations")]
    async fn create_seller_empty_name_returns_validation_error(pool: SqlitePool) {
        let state = app_state(pool);
        let result = create_seller_inner(&state, minimal_payload("")).await;
        assert!(
            matches!(result, Err(CommandError::ValidationError(_))),
            "Expected ValidationError, got: {:?}",
            result
        );
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn create_seller_invalid_country_code_returns_validation_error(pool: SqlitePool) {
        let state = app_state(pool);
        let payload = CreateSellerPayload {
            country_code: Some("USA".to_string()), // Must be exactly 2 chars
            ..minimal_payload("Test Shop")
        };
        let result = create_seller_inner(&state, payload).await;
        assert!(
            matches!(result, Err(CommandError::ValidationError(_))),
            "Expected ValidationError, got: {:?}",
            result
        );
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn create_seller_valid_args_does_not_return_validation_error(pool: SqlitePool) {
        let state = app_state(pool);
        let payload = CreateSellerPayload {
            country_code: Some("IT".to_string()),
            ..minimal_payload("Test Shop")
        };
        let result = create_seller_inner(&state, payload).await;
        assert!(
            !matches!(result, Err(CommandError::ValidationError(_))),
            "Did not expect ValidationError, got: {:?}",
            result
        );
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn update_seller_blocks_name_change_for_system_seeded(pool: SqlitePool) {
        let state = app_state(pool.clone());
        let seller_id = "trn:seller:seeded-shop";
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            r#"
            INSERT INTO sellers (id, name, type, created_at, updated_at, version, is_system_seeded)
            VALUES (?1, ?2, 'SHOP', ?3, ?4, 1, 1)
            "#,
        )
        .bind(seller_id)
        .bind("Seeded Shop")
        .bind(&now)
        .bind(&now)
        .execute(&pool)
        .await
        .expect("seed seller should insert");

        let result = update_seller_inner(
            &state,
            UpdateSellerPayload {
                id: seller_id.to_string(),
                name: "Renamed Shop".to_string(),
                seller_type: SellerType::Shop,
                email: None,
                phone: None,
                website_url: None,
                street_address: None,
                extended_address: None,
                city: None,
                state_region: None,
                postal_code: None,
                country_code: None,
                created_at: None,
            },
        )
        .await;

        assert!(
            matches!(result, Err(CommandError::BusinessRule(_))),
            "Expected BusinessRule, got: {:?}",
            result
        );
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn get_sellers_inner_enriches_seeded_and_usage_metadata(pool: SqlitePool) {
        let state = app_state(pool.clone());
        let seller_id = "trn:seller:seeded-shop";
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            r#"
            INSERT INTO sellers (id, name, type, created_at, updated_at, version, is_system_seeded)
            VALUES (?1, ?2, 'SHOP', ?3, ?4, 1, 1)
            "#,
        )
        .bind(seller_id)
        .bind("Seeded Shop")
        .bind(&now)
        .bind(&now)
        .execute(&pool)
        .await
        .expect("seed seller should insert");

        let sellers = get_sellers_inner(&state)
            .await
            .expect("query should succeed");
        let seeded = sellers
            .into_iter()
            .find(|s| s.id.as_ref() == seller_id)
            .expect("seeded seller should be present");

        assert!(seeded.is_system_seeded);
        assert_eq!(seeded.usage_count, 0);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn get_seller_by_id_inner_returns_none_when_missing(pool: SqlitePool) {
        let state = app_state(pool);
        let missing_id = SellerId::try_from("trn:seller:missing").expect("valid seller id");

        let seller = get_seller_by_id_inner(&state, missing_id)
            .await
            .expect("query should succeed");
        assert!(seller.is_none());
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn get_seller_by_id_inner_enriches_seeded_and_usage_metadata(pool: SqlitePool) {
        let state = app_state(pool.clone());
        let seller_id = "trn:seller:seeded-shop";
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            r#"
            INSERT INTO sellers (id, name, type, created_at, updated_at, version, is_system_seeded)
            VALUES (?1, ?2, 'SHOP', ?3, ?4, 1, 1)
            "#,
        )
        .bind(seller_id)
        .bind("Seeded Shop")
        .bind(&now)
        .bind(&now)
        .execute(&pool)
        .await
        .expect("seed seller should insert");

        let seller_id = SellerId::try_from(seller_id).expect("valid seller id");
        let seller = get_seller_by_id_inner(&state, seller_id)
            .await
            .expect("query should succeed")
            .expect("seller should exist");

        assert!(seller.is_system_seeded);
        assert_eq!(seller.usage_count, 0);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn delete_seller_inner_returns_not_found_for_missing_seller(pool: SqlitePool) {
        let state = app_state(pool);
        let missing_id = SellerId::try_from("trn:seller:missing").expect("valid seller id");

        let result = delete_seller_inner(&state, missing_id).await;
        assert!(
            matches!(result, Err(CommandError::NotFound(_))),
            "Expected NotFound, got: {:?}",
            result
        );
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn delete_seller_inner_removes_existing_seller(pool: SqlitePool) {
        let state = app_state(pool.clone());
        let seller_id = "trn:seller:delete-me";
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            r#"
            INSERT INTO sellers (id, name, type, created_at, updated_at, version, is_system_seeded)
            VALUES (?1, ?2, 'SHOP', ?3, ?4, 1, 0)
            "#,
        )
        .bind(seller_id)
        .bind("Delete Me")
        .bind(&now)
        .bind(&now)
        .execute(&pool)
        .await
        .expect("seller should insert");

        let seller_id = SellerId::try_from(seller_id).expect("valid seller id");
        delete_seller_inner(&state, seller_id)
            .await
            .expect("delete should succeed");

        let remaining: Option<String> = sqlx::query_scalar("SELECT id FROM sellers WHERE id = ?1")
            .bind("trn:seller:delete-me")
            .fetch_optional(&pool)
            .await
            .expect("query should succeed");

        assert!(remaining.is_none(), "seller should have been removed");
    }

    fn merge_args(source_id: &str, target_id: &str) -> MergeSellerArgs {
        MergeSellerArgs {
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
    async fn merge_sellers_inner_relinks_seller_and_buyer_and_deletes_source(pool: SqlitePool) {
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

        let result = merge_sellers_inner(&state, merge_args(source_id, target_id))
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
    async fn merge_sellers_inner_same_id_returns_business_rule(pool: SqlitePool) {
        let state = app_state(pool);
        let shared_id = "trn:seller:merge-shared";

        let result = merge_sellers_inner(&state, merge_args(shared_id, shared_id)).await;

        assert!(
            matches!(result, Err(CommandError::BusinessRule(_))),
            "Expected BusinessRule, got: {:?}",
            result
        );
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn merge_sellers_inner_missing_source_returns_not_found(pool: SqlitePool) {
        let state = app_state(pool.clone());
        let source_id = "trn:seller:missing-source";
        let target_id = "trn:seller:existing-target";

        insert_seller(&pool, target_id, "Existing Target", false)
            .await
            .expect("target should insert");

        let result = merge_sellers_inner(&state, merge_args(source_id, target_id)).await;

        assert!(
            matches!(result, Err(CommandError::NotFound(_))),
            "Expected NotFound, got: {:?}",
            result
        );
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn merge_sellers_inner_missing_target_returns_not_found(pool: SqlitePool) {
        let state = app_state(pool.clone());
        let source_id = "trn:seller:existing-source";
        let target_id = "trn:seller:missing-target";

        insert_seller(&pool, source_id, "Existing Source", false)
            .await
            .expect("source should insert");

        let result = merge_sellers_inner(&state, merge_args(source_id, target_id)).await;

        assert!(
            matches!(result, Err(CommandError::NotFound(_))),
            "Expected NotFound, got: {:?}",
            result
        );
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn merge_sellers_inner_seeded_entity_returns_business_rule(pool: SqlitePool) {
        let state = app_state(pool.clone());
        let source_id = "trn:seller:seeded-source";
        let target_id = "trn:seller:normal-target";

        insert_seller(&pool, source_id, "Seeded Source", true)
            .await
            .expect("seeded source should insert");
        insert_seller(&pool, target_id, "Normal Target", false)
            .await
            .expect("target should insert");

        let result = merge_sellers_inner(&state, merge_args(source_id, target_id)).await;

        assert!(
            matches!(result, Err(CommandError::BusinessRule(_))),
            "Expected BusinessRule, got: {:?}",
            result
        );
    }
}
