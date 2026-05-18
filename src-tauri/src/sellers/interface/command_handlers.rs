use crate::core::infrastructure::error::CommandError;
use crate::core::infrastructure::usage_queries::canonical_party_usage_count;
use crate::sellers::application::create_seller::{CreateSeller, CreateSellerInput};
use crate::sellers::application::delete_seller::DeleteSeller;
use crate::sellers::application::delete_seller_with_lock::DeleteSellerWithLock;
use crate::sellers::application::get_seller_by_id::GetSellerById;
use crate::sellers::application::get_sellers::GetSellers;
use crate::sellers::application::seller_view::SellerView;
use crate::sellers::application::update_seller::{UpdateSellerInput, UpdateSellerUseCase};
use crate::sellers::domain::seller_id::SellerId;
use crate::sellers::interface::{CreateSellerPayload, Seller, UpdateSellerPayload};
use crate::state::AppState;
use garde::Validate;
use std::convert::TryFrom;
use tracing::info;

async fn load_seller_seeded_and_name(
    state: &AppState,
    id: &SellerId,
) -> Result<Option<(String, bool)>, CommandError> {
    let mut conn = state.db_pool().acquire().await.map_err(CommandError::from)?;
    let row = sqlx::query_as::<_, (String, i64)>(
        r#"
        SELECT name, is_system_seeded
        FROM sellers
        WHERE id = ?1
        LIMIT 1
        "#,
    )
    .bind(id.as_ref())
    .fetch_optional(&mut *conn)
    .await
    .map_err(CommandError::from)?;

    Ok(row.map(|(name, seeded)| (name, seeded != 0)))
}

async fn enrich_seller_view(state: &AppState, seller: &mut SellerView) -> Result<(), CommandError> {
    let mut conn = state.db_pool().acquire().await.map_err(CommandError::from)?;

    let seeded = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT is_system_seeded
        FROM sellers
        WHERE id = ?1
        LIMIT 1
        "#,
    )
    .bind(seller.id.as_ref())
    .fetch_optional(&mut *conn)
    .await
    .map_err(CommandError::from)?
    .unwrap_or(0);

    let usage_count = canonical_party_usage_count(&mut conn, seller.id.as_ref())
        .await
        .map_err(CommandError::from)?;

    seller.is_system_seeded = seeded != 0;
    seller.usage_count = usage_count;
    Ok(())
}

pub async fn get_sellers_inner(state: &AppState) -> Result<Vec<SellerView>, CommandError> {
    info!("Fetching all sellers");

    let mut unit_of_work = state.unit_of_work().await?;

    let mut sellers = GetSellers::execute(&mut unit_of_work).await?;
    unit_of_work.commit().await?;

    for seller in &mut sellers {
        enrich_seller_view(state, seller).await?;
    }

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

    unit_of_work.commit().await?;

    let mut result = result;
    if let Some(seller) = result.as_mut() {
        enrich_seller_view(state, seller).await?;
    }

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
    let existing = load_seller_seeded_and_name(state, &seller_id).await?;

    let (current_name, is_system_seeded) = existing
        .ok_or_else(|| CommandError::NotFound(format!("Seller '{}' not found", payload.id)))?;

    if is_system_seeded && current_name.trim() != payload.name.trim() {
        return Err(CommandError::BusinessRule(
            "System-seeded seller names cannot be edited".to_string(),
        ));
    }

    let mut unit_of_work = state.unit_of_work().await?;
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
        let mut conn = state.db_pool().acquire().await.map_err(CommandError::from)?;
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
}
