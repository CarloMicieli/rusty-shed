use crate::catalog::application::{GetManufacturerById, GetManufacturers};
use crate::catalog::domain::manufacturer::{
    Manufacturer as DomainManufacturer, ManufacturerId, ManufacturerStatus,
};
use crate::catalog::infrastructure::entities::ManufacturerRow;
use crate::core::domain::identifiers::Identifier;
use crate::core::infrastructure::error::CommandError;
use crate::state::AppState;
use garde::Validate;
use serde::{Deserialize, Serialize};
use tracing::info;
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Manufacturer {
    pub id: ManufacturerId,
    pub name: String,
    pub registered_company_name: Option<String>,
    pub country_code: Option<String>,
    pub status: ManufacturerStatus,
    pub website_url: Option<Url>,
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
        }
    }
}

/// Retrieve all manufacturers from the database.
pub async fn get_manufacturers_inner(state: &AppState) -> Result<Vec<Manufacturer>, CommandError> {
    info!("Fetching all manufacturers from the database.");
    let mut uow = state.unit_of_work().await?;
    let manufacturers = GetManufacturers::execute(&mut uow).await?;
    uow.commit().await?;
    Ok(manufacturers.into_iter().map(Manufacturer::from).collect())
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
    let manufacturer = GetManufacturerById::execute(&mut uow, manufacturer_id).await?;
    uow.commit().await?;
    Ok(manufacturer.map(Manufacturer::from))
}

/// Tauri command to retrieve a manufacturer by its identifier.
#[tauri::command]
#[specta::specta]
pub async fn get_manufacturer_by_id(
    state: tauri::State<'_, AppState>,
    manufacturer_id: ManufacturerId,
) -> Result<Option<Manufacturer>, CommandError> {
    get_manufacturer_by_id_inner(&state, manufacturer_id).await
}

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

fn normalize_optional(value: Option<String>) -> Option<String> {
    value.and_then(|raw| {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    })
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

    let id = ManufacturerId::new_from_parts(&[name.as_str()]);
    let mut tx = state.db_pool().begin().await?;

    let insert_result = sqlx::query(
        r#"
        INSERT INTO manufacturers (id, name, status, country_code, website_url)
        VALUES (?1, ?2, 'ACTIVE', ?3, ?4)
        "#,
    )
    .bind(id.as_ref())
    .bind(&name)
    .bind(&country_code)
    .bind(&website_url)
    .execute(&mut *tx)
    .await;

    if let Err(err) = insert_result {
        if let sqlx::Error::Database(db_err) = &err
            && db_err.is_unique_violation()
        {
            return Err(CommandError::Conflict(
                "A manufacturer with this name already exists".to_string(),
            ));
        }
        return Err(CommandError::DatabaseError(err.to_string()));
    }

    let row = sqlx::query_as::<_, ManufacturerRow>(
        r#"
        SELECT id, name, registered_company_name, status, country_code, website_url, created_at, updated_at, version
        FROM manufacturers
        WHERE id = ?1
        LIMIT 1
        "#,
    )
    .bind(id.as_ref())
    .fetch_one(&mut *tx)
    .await
    .map_err(CommandError::from)?;

    tx.commit().await.map_err(CommandError::from)?;

    let domain = DomainManufacturer::try_from(row).map_err(CommandError::from)?;
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

#[cfg(test)]
mod tests {
    use super::*;
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
}
