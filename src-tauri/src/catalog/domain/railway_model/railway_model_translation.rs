use crate::catalog::domain::railway_model::RailwayModelId;
use serde::Serialize;

/// All stored translations for a single railway model.
/// Used by the edit form to pre-populate language-specific input fields.
#[derive(Debug, Clone, Serialize, specta::Type)]
pub struct RailwayModelTranslations {
    pub railway_model_id: RailwayModelId,
    pub en: Option<RailwayModelTranslationEntry>,
    pub it: Option<RailwayModelTranslationEntry>,
}

/// A single language entry with optional description and details.
#[derive(Debug, Clone, Serialize, specta::Type)]
pub struct RailwayModelTranslationEntry {
    pub description: Option<String>,
    pub details: Option<String>,
}
