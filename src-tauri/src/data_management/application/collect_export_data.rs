/// Collect export data use case
use crate::data_management::domain::{ExportEntitySelection, ExportError};
use crate::data_management::infrastructure::media_collector::MediaFile;

/// Collect data for export
pub async fn collect_data(
    _selection: &ExportEntitySelection,
) -> Result<(serde_json::Value, Vec<MediaFile>), ExportError> {
    // This will be implemented in Phase 2
    Ok((serde_json::json!({}), Vec::new()))
}
