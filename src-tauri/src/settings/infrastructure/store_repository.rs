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

fn migrate_legacy_onboarding_fields(value: &mut Value) {
    // One-time compatibility migration:
    // legacy `first_run=true` means onboarding incomplete,
    // while canonical `has_completed_onboarding=true` means complete.
    if let Value::Object(obj) = value {
        if obj.contains_key("has_completed_onboarding") {
            return;
        }

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
}

impl SettingsRepository for StoreSettingsRepository {
    fn load(&self, app: &AppHandle) -> Result<UserSettings, String> {
        let store = app
            .store(STORE_FILE)
            .map_err(|e| format!("Failed to access store: {}", e))?;

        // Try to load settings from store
        match store.get(SETTINGS_KEY) {
            Some(mut value) => {
                migrate_legacy_onboarding_fields(&mut value);

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
    use serde_json::json;

    #[test]
    fn test_default_repository_creation() {
        let _repo = StoreSettingsRepository::new();
        // Just verify it compiles and creates - no assertion needed
    }

    #[test]
    fn migrate_legacy_first_run_true_sets_onboarding_false() {
        let mut value = json!({"first_run": true});

        migrate_legacy_onboarding_fields(&mut value);

        assert_eq!(value["has_completed_onboarding"], Value::Bool(false));
        assert!(value.get("first_run").is_none());
    }

    #[test]
    fn migrate_legacy_first_run_false_sets_onboarding_true() {
        let mut value = json!({"firstRun": false});

        migrate_legacy_onboarding_fields(&mut value);

        assert_eq!(value["has_completed_onboarding"], Value::Bool(true));
        assert!(value.get("firstRun").is_none());
    }

    #[test]
    fn migration_keeps_existing_canonical_key_unchanged() {
        let mut value = json!({
            "has_completed_onboarding": true,
            "first_run": false
        });

        migrate_legacy_onboarding_fields(&mut value);

        assert_eq!(value["has_completed_onboarding"], Value::Bool(true));
        assert_eq!(value["first_run"], Value::Bool(false));
    }

    #[test]
    fn migration_noops_when_no_legacy_keys_exist() {
        let mut value = json!({"language": "en"});

        migrate_legacy_onboarding_fields(&mut value);

        assert_eq!(value["language"], Value::String("en".to_string()));
        assert!(value.get("has_completed_onboarding").is_none());
    }
}
