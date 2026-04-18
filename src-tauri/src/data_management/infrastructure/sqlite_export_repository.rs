use async_trait::async_trait;
use sqlx::SqlitePool;
use std::path::{Path, PathBuf};

use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::data_management::domain::{
    ExportEntitySelection, ExportError, ExportRepository, ExportUowExt,
};
use crate::data_management::infrastructure::{manifest_builder, media_collector};
use serde_json::Value;

/// SQLite-backed implementation of [`ExportRepository`].
pub struct SqliteExportRepository {
    pool: SqlitePool,
}

impl SqliteExportRepository {
    /// Create a new SQLite export repository.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ExportRepository for SqliteExportRepository {
    async fn build_manifest(
        &self,
        selection: &ExportEntitySelection,
        media_dir: &Path,
    ) -> Result<Value, ExportError> {
        manifest_builder::build_manifest(&self.pool, selection, media_dir).await
    }

    async fn collect_media_files(
        &self,
        selection: &ExportEntitySelection,
        media_dir: &Path,
    ) -> Result<Vec<PathBuf>, ExportError> {
        media_collector::collect_media_files(&self.pool, selection, media_dir).await
    }
}

impl ExportUowExt for SqliteUnitOfWork {
    fn export_repo(&mut self) -> Box<dyn ExportRepository + '_> {
        Box::new(SqliteExportRepository::new(self.pool()))
    }
}
