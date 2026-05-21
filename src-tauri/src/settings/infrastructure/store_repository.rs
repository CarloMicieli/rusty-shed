use crate::settings::domain::user_settings::UserSettings;
use serde_json::Value;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

const STORE_FILE: &str = "settings.json";
const SETTINGS_KEY: &str = "user_settings";

/// Repository trait for settings persistence
pub trait SettingsRepository {
    fn load(&self, app: &AppHandle) -> Result<UserSettings, String>;
    fn save(&self, app: &AppHandle, settings: &UserSettings) -> Result<(), String>;
}

/// Implementation using tauri-plugin-store
pub struct StoreSettingsRepository;

impl StoreSettingsRepository {
    pub fn new() -> Self {
        StoreSettingsRepository
    }
}

impl SettingsRepository for StoreSettingsRepository {
    fn load(&self, app: &AppHandle) -> Result<UserSettings, String> {
        let store = app
            .store(STORE_FILE)
            .map_err(|e| format!("Failed to access store: {}", e))?;

        // Try to load settings from store
        match store.get(SETTINGS_KEY) {
            Some(mut value) => {
                // One-time compatibility migration:
                // legacy `first_run=true` means onboarding incomplete,
                // while canonical `has_completed_onboarding=true` means complete.
                if let Value::Object(obj) = &mut value
                    && !obj.contains_key("has_completed_onboarding")
                {
                    let legacy_first_run = obj
                        .get("first_run")
                        .and_then(Value::as_bool)
                        .or_else(|| obj.get("firstRun").and_then(Value::as_bool));

                    if let Some(first_run) = legacy_first_run {
                        obj.insert(
                            "has_completed_onboarding".to_string(),
                            Value::Bool(!first_run),
                        );
                        obj.remove("first_run");
                        obj.remove("firstRun");
                    }
                }

                // Deserialize from JSON value
                serde_json::from_value(value.clone())
                    .map_err(|e| format!("Failed to deserialize settings: {}", e))
            }
            None => {
                // Settings not found, return defaults
                Ok(UserSettings::default())
            }
        }
    }

    fn save(&self, app: &AppHandle, settings: &UserSettings) -> Result<(), String> {
        // Validate before saving
        settings.validate()?;

        let store = app
            .store(STORE_FILE)
            .map_err(|e| format!("Failed to access store: {}", e))?;

        // Serialize settings to JSON value
        let value = serde_json::to_value(settings)
            .map_err(|e| format!("Failed to serialize settings: {}", e))?;

        // Set value in store (returns (), not Result)
        store.set(SETTINGS_KEY, value);

        // Save store to disk
        store
            .save()
            .map_err(|e| format!("Failed to save store to disk: {}", e))?;

        Ok(())
    }
}

impl Default for StoreSettingsRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_repository_creation() {
        let _repo = StoreSettingsRepository::new();
        // Just verify it compiles and creates - no assertion needed
    }
}
