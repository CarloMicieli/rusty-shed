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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_management::domain::ExportEntitySelection;

    #[tokio::test]
    async fn collect_data_returns_empty_payload_and_media_list() {
        let selection = ExportEntitySelection {
            include_railway_models: true,
            include_collection_items: true,
            include_sellers: true,
            include_maintenance_logs: true,
            include_dcc_roster: true,
            include_orphaned_images: true,
            include_track_inventory: true,
            include_train_formations: true,
            include_wishlists: true,
        };

        let result = collect_data(&selection).await;

        assert!(result.is_ok(), "{result:?}");
        let (payload, media) = result.expect("collect_data should succeed");
        assert_eq!(payload, serde_json::json!({}));
        assert!(media.is_empty());
    }
}
