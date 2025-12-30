use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, OnceLock};
use uuid::Uuid;

use crate::core::infrastructure::error::CommandError;

/// Core collection item representation surfaced to the frontend.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct CollectionItemLite {
    pub id: String,
    pub brand: String,
    pub catalog_number: String,
    pub title: String,
    pub scale: String,
    pub power_system: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub created_at: DateTime<Utc>,
}

/// Payload for creating a collection item.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct CreateCollectionItemInput {
    pub brand: String,
    pub catalog_number: String,
    pub title: String,
    pub scale: String,
    pub power_system: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
}

/// Payload for updating a collection item.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCollectionItemInput {
    pub id: String,
    pub brand: String,
    pub catalog_number: String,
    pub title: String,
    pub scale: String,
    pub power_system: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
}

static COLLECTION_STORE: OnceLock<Mutex<Vec<CollectionItemLite>>> = OnceLock::new();

fn store() -> &'static Mutex<Vec<CollectionItemLite>> {
    COLLECTION_STORE.get_or_init(|| Mutex::new(Vec::new()))
}

fn normalize(text: &str) -> String {
    text.to_lowercase()
}

fn to_command_error(message: impl Into<String>) -> CommandError {
    CommandError::Unknown(message.into())
}

#[tauri::command]
#[specta::specta]
pub async fn list_collection_items(search: Option<String>) -> Result<Vec<CollectionItemLite>, CommandError> {
    let guard = store().lock().map_err(|_| to_command_error("collection store poisoned"))?;
    let items = guard.clone();

    if let Some(query) = search.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()) {
        let q = normalize(query);
        let filtered = items
            .into_iter()
            .filter(|item| {
                let haystack = format!(
                    "{} {} {} {} {}",
                    item.brand,
                    item.catalog_number,
                    item.title,
                    item.description.clone().unwrap_or_default(),
                    item.tags.join(" ")
                )
                .to_lowercase();
                haystack.contains(&q)
            })
            .collect();
        return Ok(filtered);
    }

    Ok(items)
}

#[tauri::command]
#[specta::specta]
pub async fn create_collection_item(
    input: CreateCollectionItemInput,
) -> Result<CollectionItemLite, CommandError> {
    let mut guard = store().lock().map_err(|_| to_command_error("collection store poisoned"))?;

    let item = CollectionItemLite {
        id: Uuid::new_v4().to_string(),
        brand: input.brand,
        catalog_number: input.catalog_number,
        title: input.title,
        scale: input.scale,
        power_system: input.power_system,
        description: input.description,
        tags: input.tags,
        created_at: Utc::now(),
    };

    guard.push(item.clone());
    Ok(item)
}

#[tauri::command]
#[specta::specta]
pub async fn update_collection_item(
    input: UpdateCollectionItemInput,
) -> Result<CollectionItemLite, CommandError> {
    let mut guard = store().lock().map_err(|_| to_command_error("collection store poisoned"))?;
    let pos = guard
        .iter()
        .position(|item| item.id == input.id)
        .ok_or_else(|| to_command_error("item not found"))?;

    let updated = CollectionItemLite {
        id: input.id,
        brand: input.brand,
        catalog_number: input.catalog_number,
        title: input.title,
        scale: input.scale,
        power_system: input.power_system,
        description: input.description,
        tags: input.tags,
        created_at: guard[pos].created_at,
    };

    guard[pos] = updated.clone();
    Ok(updated)
}

#[tauri::command]
#[specta::specta]
pub async fn delete_collection_item(id: String) -> Result<(), CommandError> {
    let mut guard = store().lock().map_err(|_| to_command_error("collection store poisoned"))?;
    let len_before = guard.len();
    guard.retain(|item| item.id != id);

    if guard.len() == len_before {
        return Err(to_command_error("item not found"));
    }

    Ok(())
}
