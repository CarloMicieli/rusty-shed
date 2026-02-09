/// Collect export data use case
use crate::export::domain::entity_selection::ExportEntitySelection;
use crate::export::domain::error::ExportError;
use crate::export::infrastructure::media_collector::MediaFile;

/// Collect data for export
pub async fn collect_data(
    _selection: &ExportEntitySelection,
) -> Result<(serde_json::Value, Vec<MediaFile>), ExportError> {
    // This will be implemented in Phase 2
    Ok((serde_json::json!({}), Vec::new()))
}
