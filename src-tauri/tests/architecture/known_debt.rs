use crate::support::{DependencyDebt, observed_outward_dependencies, production_sources};
use std::collections::BTreeSet;

const KNOWN_OUTWARD_DEPENDENCIES: &[(&str, &str)] = &[
    // ADR 0004 domain isolation debt. Keep this list shrinking; resolved entries
    // make the test fail until they are removed here.
    (
        "src/catalog/domain/railway_model/rolling_stock_params.rs",
        "crate::catalog::application",
    ),
    (
        "src/collecting/domain/collection.rs",
        "crate::collecting::application",
    ),
    (
        "src/data_management/domain/backup_errors.rs",
        "crate::core::infrastructure",
    ),
    (
        "src/dcc_inventory/domain/repositories.rs",
        "crate::dcc_inventory::application",
    ),
    (
        "src/sellers/domain/repository.rs",
        "crate::sellers::application",
    ),
    // ADR 0004 / ADR 0011 application isolation debt. Infrastructure and Tauri
    // access is frozen to these observed edges while the modules migrate.
    (
        "src/cloud_backup/application/connect_google.rs",
        "crate::cloud_backup::infrastructure",
    ),
    (
        "src/cloud_backup/application/connect_google.rs",
        "external::tauri",
    ),
    (
        "src/cloud_backup/application/disconnect_google.rs",
        "crate::cloud_backup::infrastructure",
    ),
    (
        "src/cloud_backup/application/get_connection_status.rs",
        "crate::cloud_backup::infrastructure",
    ),
    (
        "src/cloud_backup/application/list_backups.rs",
        "crate::cloud_backup::infrastructure",
    ),
    (
        "src/cloud_backup/application/restore_backup.rs",
        "crate::cloud_backup::infrastructure",
    ),
    (
        "src/cloud_backup/application/restore_backup.rs",
        "external::tauri",
    ),
    (
        "src/cloud_backup/application/sync_backup.rs",
        "crate::cloud_backup::infrastructure",
    ),
    (
        "src/cloud_backup/application/sync_backup.rs",
        "crate::data_management::interface",
    ),
    (
        "src/cloud_backup/application/sync_backup.rs",
        "external::sqlx",
    ),
    (
        "src/cloud_backup/application/sync_backup.rs",
        "external::tauri",
    ),
    (
        "src/data_management/application/analyze_package.rs",
        "crate::data_management::infrastructure",
    ),
    (
        "src/data_management/application/execute_export.rs",
        "crate::data_management::infrastructure",
    ),
    (
        "src/data_management/application/export_database.rs",
        "external::sqlx",
    ),
    (
        "src/data_management/application/import_database.rs",
        "crate::data_management::infrastructure",
    ),
    (
        "src/data_management/application/ports.rs",
        "crate::data_management::infrastructure",
    ),
    (
        "src/data_management/application/preview_export.rs",
        "external::sqlx",
    ),
    (
        "src/data_management/application/preview_import.rs",
        "crate::data_management::infrastructure",
    ),
    (
        "src/media/application/delete_model_image.rs",
        "crate::media::infrastructure",
    ),
    (
        "src/media/application/get_railway_model_image.rs",
        "crate::media::infrastructure",
    ),
    (
        "src/media/application/upload_model_image.rs",
        "crate::media::infrastructure",
    ),
    (
        "src/sellers/application/delete_seller_with_lock.rs",
        "crate::core::infrastructure",
    ),
    (
        "src/sellers/application/delete_seller_with_lock.rs",
        "external::sqlx",
    ),
    ("src/sellers/application/merge_seller.rs", "external::sqlx"),
    (
        "src/settings/application/get_settings.rs",
        "crate::settings::infrastructure",
    ),
    (
        "src/settings/application/get_settings.rs",
        "external::tauri",
    ),
    (
        "src/settings/application/initialize_settings.rs",
        "crate::settings::infrastructure",
    ),
    (
        "src/settings/application/initialize_settings.rs",
        "external::tauri",
    ),
    (
        "src/settings/application/update_settings.rs",
        "crate::settings::infrastructure",
    ),
    (
        "src/settings/application/update_settings.rs",
        "external::tauri",
    ),
    (
        "src/tracks_inventory/application/create_track_product.rs",
        "crate::core::infrastructure",
    ),
    (
        "src/tracks_inventory/application/delete_track_product.rs",
        "crate::core::infrastructure",
    ),
    (
        "src/tracks_inventory/application/delete_track_product_translation.rs",
        "crate::core::infrastructure",
    ),
    (
        "src/tracks_inventory/application/update_track_product.rs",
        "crate::core::infrastructure",
    ),
    (
        "src/tracks_inventory/application/upsert_track_product_translation.rs",
        "crate::core::infrastructure",
    ),
];

#[test]
fn inner_layer_dependency_debt_cannot_grow() {
    let expected: BTreeSet<_> = KNOWN_OUTWARD_DEPENDENCIES
        .iter()
        .map(|(source, dependency)| DependencyDebt {
            source: (*source).to_string(),
            dependency: (*dependency).to_string(),
        })
        .collect();
    let observed = observed_outward_dependencies(&production_sources());
    let new_dependencies: Vec<_> = observed.difference(&expected).collect();
    let stale_dependencies: Vec<_> = expected.difference(&observed).collect();

    assert!(
        new_dependencies.is_empty() && stale_dependencies.is_empty(),
        concat!(
            "Inner-layer dependency ratchet changed (ADR 0004, ADR 0011).\n",
            "New forbidden dependencies:\n{new_dependencies:#?}\n",
            "Resolved dependencies; remove these stale ratchet entries:\n{stale_dependencies:#?}"
        ),
        new_dependencies = new_dependencies,
        stale_dependencies = stale_dependencies,
    );
}
