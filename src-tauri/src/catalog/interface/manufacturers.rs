use crate::catalog::application::{
    CreateManufacturer, CreateManufacturerInput, DeleteManufacturer, GetManufacturerById,
    GetManufacturers, MergeManufacturers, UpdateManufacturer, UpdateManufacturerInput,
};
use crate::catalog::domain::manufacturer::{
    Manufacturer as DomainManufacturer, ManufacturerId, ManufacturerStatus,
};
use crate::core::infrastructure::error::CommandError;
use crate::state::AppState;
use garde::Validate;
use serde::{Deserialize, Serialize};
use tracing::info;
use url::Url;

/// Manufacturer DTO exposed by Tauri command handlers.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Manufacturer {
    pub id: ManufacturerId,
    pub name: String,
    pub registered_company_name: Option<String>,
    pub country_code: Option<String>,
    pub status: ManufacturerStatus,
    pub website_url: Option<Url>,
    pub is_system_seeded: bool,
    pub usage_count: i64,
}

impl From<DomainManufacturer> for Manufacturer {
    fn from(value: DomainManufacturer) -> Self {
        Self {
            id: value.id,
            name: value.name,
            registered_company_name: value.registered_company_name,
            country_code: value.country_code,
            status: value.status,
            website_url: value.website_url,
            is_system_seeded: false,
            usage_count: 0,
        }
    }
}

/// Retrieve all manufacturers from the database.
pub async fn get_manufacturers_inner(state: &AppState) -> Result<Vec<Manufacturer>, CommandError> {
    info!("Fetching all manufacturers from the database.");
    let mut uow = state.unit_of_work().await?;
    let domain_manufacturers = GetManufacturers::execute(&mut uow).await?;

    let mut manufacturers = Vec::with_capacity(domain_manufacturers.len());
    for m in domain_manufacturers {
        let id = m.id.clone();
        let mut dto = Manufacturer::from(m);
        let mut repo = uow.manufacturers_repo();
        dto.is_system_seeded = repo
            .find_is_system_seeded(&id)
            .await
            .map_err(CommandError::from)?
            .unwrap_or(false);
        dto.usage_count = repo
            .find_usage_count(&id)
            .await
            .map_err(CommandError::from)?;
        drop(repo);
        manufacturers.push(dto);
    }

    uow.commit().await?;
    Ok(manufacturers)
}

/// Tauri command to retrieve all manufacturers.
#[tauri::command]
#[specta::specta]
pub async fn get_manufacturers(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<Manufacturer>, CommandError> {
    get_manufacturers_inner(&state).await
}

/// Retrieve a manufacturer by its identifier.
pub async fn get_manufacturer_by_id_inner(
    state: &AppState,
    manufacturer_id: ManufacturerId,
) -> Result<Option<Manufacturer>, CommandError> {
    info!(
        "Fetching manufacturer {} from the database.",
        manufacturer_id
    );
    let mut uow = state.unit_of_work().await?;
    let domain = GetManufacturerById::execute(&mut uow, manufacturer_id).await?;

    let mut dto = domain.map(Manufacturer::from);
    if let Some(value) = dto.as_mut() {
        let id = value.id.clone();
        let mut repo = uow.manufacturers_repo();
        value.is_system_seeded = repo
            .find_is_system_seeded(&id)
            .await
            .map_err(CommandError::from)?
            .unwrap_or(false);
        value.usage_count = repo
            .find_usage_count(&id)
            .await
            .map_err(CommandError::from)?;
        drop(repo);
    }

    uow.commit().await?;
    Ok(dto)
}

#[tauri::command]
#[specta::specta]
pub async fn get_manufacturer_by_id(
    state: tauri::State<'_, AppState>,
    manufacturer_id: ManufacturerId,
) -> Result<Option<Manufacturer>, CommandError> {
    get_manufacturer_by_id_inner(&state, manufacturer_id).await
}

fn normalize_optional(value: Option<String>) -> Option<String> {
    value
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
}

/// Input payload for creating a manufacturer.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct CreateManufacturerArgs {
    #[garde(length(min = 1, max = 200))]
    pub name: String,
    pub website_url: Option<String>,
    #[garde(length(min = 2, max = 2))]
    pub country_code: Option<String>,
}

pub async fn create_manufacturer_inner(
    state: &AppState,
    args: CreateManufacturerArgs,
) -> Result<Manufacturer, CommandError> {
    args.validate().map_err(CommandError::from)?;

    let name = args.name.trim().to_string();
    if name.is_empty() {
        return Err(CommandError::validation_field("name", "Name is required"));
    }

    let website_url = normalize_optional(args.website_url);
    if let Some(url) = &website_url {
        Url::parse(url).map_err(|e| CommandError::validation_field("websiteUrl", e.to_string()))?;
    }

    let country_code = normalize_optional(args.country_code).map(|value| value.to_uppercase());

    let mut uow = state.unit_of_work().await?;
    let domain = CreateManufacturer::execute(
        &mut uow,
        CreateManufacturerInput {
            name,
            country_code,
            website_url,
        },
    )
    .await
    .map_err(CommandError::from)?;
    uow.commit().await?;
    Ok(Manufacturer::from(domain))
}

#[tauri::command]
#[specta::specta]
pub async fn create_manufacturer(
    state: tauri::State<'_, AppState>,
    args: CreateManufacturerArgs,
) -> Result<Manufacturer, CommandError> {
    create_manufacturer_inner(&state, args).await
}

/// Input payload for updating a manufacturer.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct UpdateManufacturerArgs {
    pub id: ManufacturerId,
    #[garde(length(min = 1, max = 200))]
    pub name: String,
    pub website_url: Option<String>,
    #[garde(length(min = 2, max = 2))]
    pub country_code: Option<String>,
}

pub async fn update_manufacturer_inner(
    state: &AppState,
    args: UpdateManufacturerArgs,
) -> Result<Manufacturer, CommandError> {
    args.validate().map_err(CommandError::from)?;

    let name = args.name.trim().to_string();
    if name.is_empty() {
        return Err(CommandError::validation_field("name", "Name is required"));
    }

    let website_url = normalize_optional(args.website_url);
    if let Some(url) = &website_url {
        Url::parse(url).map_err(|e| CommandError::validation_field("websiteUrl", e.to_string()))?;
    }

    let country_code = normalize_optional(args.country_code).map(|value| value.to_uppercase());

    let mut uow = state.unit_of_work().await?;
    let domain = UpdateManufacturer::execute(
        &mut uow,
        UpdateManufacturerInput {
            id: args.id,
            name,
            country_code,
            website_url,
        },
    )
    .await
    .map_err(CommandError::from)?;

    let id = domain.id.clone();
    let mut result = Manufacturer::from(domain);
    let mut repo = uow.manufacturers_repo();
    result.is_system_seeded = repo
        .find_is_system_seeded(&id)
        .await
        .map_err(CommandError::from)?
        .unwrap_or(false);
    result.usage_count = repo
        .find_usage_count(&id)
        .await
        .map_err(CommandError::from)?;
    drop(repo);
    uow.commit().await?;
    Ok(result)
}

#[tauri::command]
#[specta::specta]
pub async fn update_manufacturer(
    state: tauri::State<'_, AppState>,
    args: UpdateManufacturerArgs,
) -> Result<Manufacturer, CommandError> {
    update_manufacturer_inner(&state, args).await
}

pub async fn delete_manufacturer_inner(
    state: &AppState,
    id: ManufacturerId,
) -> Result<(), CommandError> {
    let mut uow = state.unit_of_work().await?;
    DeleteManufacturer::execute(&mut uow, &id)
        .await
        .map_err(CommandError::from)?;
    uow.commit().await?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn delete_manufacturer(
    state: tauri::State<'_, AppState>,
    id: ManufacturerId,
) -> Result<(), CommandError> {
    delete_manufacturer_inner(&state, id).await
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct MergeManufacturerArgs {
    pub source_id: ManufacturerId,
    pub target_id: ManufacturerId,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ManufacturerMergeResult {
    pub source_id: String,
    pub target_id: String,
    pub relinked_count: i64,
}

pub async fn merge_manufacturers_inner(
    state: &AppState,
    args: MergeManufacturerArgs,
) -> Result<ManufacturerMergeResult, CommandError> {
    let mut uow = state.unit_of_work().await?;

    let relinked_count = MergeManufacturers::execute(&mut uow, &args.source_id, &args.target_id)
        .await
        .map_err(CommandError::from)?;

    uow.commit().await?;

    Ok(ManufacturerMergeResult {
        source_id: args.source_id.to_string(),
        target_id: args.target_id.to_string(),
        relinked_count,
    })
}

#[tauri::command]
#[specta::specta]
pub async fn merge_manufacturers(
    state: tauri::State<'_, AppState>,
    args: MergeManufacturerArgs,
) -> Result<ManufacturerMergeResult, CommandError> {
    merge_manufacturers_inner(&state, args).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::identifiers::Identifier;
    use sqlx::SqlitePool;

    fn app_state(pool: SqlitePool) -> AppState {
        AppState::for_test(pool)
    }

    fn minimal_args(name: &str) -> CreateManufacturerArgs {
        CreateManufacturerArgs {
            name: name.to_string(),
            website_url: None,
            country_code: None,
        }
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn create_manufacturer_empty_name_returns_validation_error(pool: SqlitePool) {
        let state = app_state(pool);
        let result = create_manufacturer_inner(&state, minimal_args(" ")).await;
        assert!(
            matches!(result, Err(CommandError::ValidationError(_))),
            "Expected ValidationError, got: {:?}",
            result
        );
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn create_manufacturer_duplicate_name_returns_conflict(pool: SqlitePool) {
        let state = app_state(pool);
        let first = create_manufacturer_inner(&state, minimal_args("ACME")).await;
        assert!(
            first.is_ok(),
            "Expected first insert to succeed, got: {first:?}"
        );

        let second = create_manufacturer_inner(&state, minimal_args("acme")).await;
        assert!(
            matches!(second, Err(CommandError::Conflict(_))),
            "Expected Conflict, got: {:?}",
            second
        );
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn create_manufacturer_success_returns_inserted_row(pool: SqlitePool) {
        let state = app_state(pool);
        let result = create_manufacturer_inner(
            &state,
            CreateManufacturerArgs {
                name: "Roco".to_string(),
                website_url: Some("https://www.roco.cc".to_string()),
                country_code: Some("at".to_string()),
            },
        )
        .await
        .expect("manufacturer should be created");

        assert_eq!(result.name, "Roco");
        assert_eq!(result.country_code.as_deref(), Some("AT"));
        assert_eq!(
            result.website_url.as_ref().map(Url::as_str),
            Some("https://www.roco.cc/")
        );
        assert_eq!(result.status, ManufacturerStatus::Active);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn update_manufacturer_blocks_name_change_for_system_seeded(pool: SqlitePool) {
        let state = app_state(pool.clone());
        let manufacturer_id = ManufacturerId::new_from_parts(&["seeded-maker"]);

        sqlx::query(
            r#"
            INSERT INTO manufacturers (
                id,
                name,
                status,
                country_code,
                website_url,
                created_at,
                updated_at,
                version,
                is_system_seeded
            )
            VALUES (?1, ?2, 'ACTIVE', NULL, NULL, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 1, 1)
            "#,
        )
        .bind(manufacturer_id.as_ref())
        .bind("Seeded Manufacturer")
        .execute(&pool)
        .await
        .expect("seed manufacturer should insert");

        let result = update_manufacturer_inner(
            &state,
            UpdateManufacturerArgs {
                id: manufacturer_id,
                name: "Renamed Manufacturer".to_string(),
                website_url: None,
                country_code: None,
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
    async fn get_manufacturers_inner_enriches_seeded_and_usage_metadata(pool: SqlitePool) {
        let state = app_state(pool.clone());
        let manufacturer_id = ManufacturerId::new_from_parts(&["seeded-maker"]);

        sqlx::query(
            r#"
            INSERT INTO manufacturers (
                id,
                name,
                status,
                country_code,
                website_url,
                created_at,
                updated_at,
                version,
                is_system_seeded
            )
            VALUES (?1, ?2, 'ACTIVE', NULL, NULL, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 1, 1)
            "#,
        )
        .bind(manufacturer_id.as_ref())
        .bind("Seeded Manufacturer")
        .execute(&pool)
        .await
        .expect("seed manufacturer should insert");

        let manufacturers = get_manufacturers_inner(&state)
            .await
            .expect("query should succeed");

        let seeded = manufacturers
            .into_iter()
            .find(|m| m.id == manufacturer_id)
            .expect("seeded manufacturer should be present");

        assert!(seeded.is_system_seeded);
        assert_eq!(seeded.usage_count, 0);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn get_manufacturer_by_id_inner_returns_none_when_missing(pool: SqlitePool) {
        let state = app_state(pool);
        let missing_id = ManufacturerId::new_from_parts(&["missing-maker"]);

        let manufacturer = get_manufacturer_by_id_inner(&state, missing_id)
            .await
            .expect("query should succeed");

        assert!(manufacturer.is_none());
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn get_manufacturer_by_id_inner_enriches_seeded_and_usage_metadata(pool: SqlitePool) {
        let state = app_state(pool.clone());
        let manufacturer_id = ManufacturerId::new_from_parts(&["seeded-maker"]);

        sqlx::query(
            r#"
            INSERT INTO manufacturers (
                id,
                name,
                status,
                country_code,
                website_url,
                created_at,
                updated_at,
                version,
                is_system_seeded
            )
            VALUES (?1, ?2, 'ACTIVE', NULL, NULL, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 1, 1)
            "#,
        )
        .bind(manufacturer_id.as_ref())
        .bind("Seeded Manufacturer")
        .execute(&pool)
        .await
        .expect("seed manufacturer should insert");

        let manufacturer = get_manufacturer_by_id_inner(&state, manufacturer_id)
            .await
            .expect("query should succeed")
            .expect("manufacturer should exist");

        assert!(manufacturer.is_system_seeded);
        assert_eq!(manufacturer.usage_count, 0);
    }
}
