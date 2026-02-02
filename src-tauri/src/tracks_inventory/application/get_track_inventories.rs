//! Query to fetch all track inventories with summary information.

use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::tracks_inventory::application::views::TrackInventoryListItem;

/// Query to fetch all track inventories.
pub struct GetTrackInventoriesQuery;

impl GetTrackInventoriesQuery {
    /// Execute the query to get all track inventories with summary data.
    ///
    /// # Arguments
    /// - `unit_of_work`: Transactional unit providing repository access.
    ///
    /// # Returns
    /// * `Vec<TrackInventoryListItem>` - List of inventory summaries.
    /// * `DomainError` - On database or domain error.
    ///
    /// # Type Parameters
    /// - `U`: Unit of work type implementing `TracksInventoryUowExt` and `Send`.
    pub async fn execute(
        unit_of_work: &mut SqliteUnitOfWork<'_>,
    ) -> Result<Vec<TrackInventoryListItem>, DomainError> {
        let sql = r#"
            SELECT 
                ti.id,
                ti.name,
                ti.description,
                COUNT(DISTINCT tii.track_id) as total_items,
                COALESCE(SUM(tii.quantity), 0) as total_quantity
            FROM track_inventories ti
            LEFT JOIN track_inventory_items tii ON ti.id = tii.inventory_id
            GROUP BY ti.id, ti.name, ti.description
            ORDER BY ti.created_at DESC
        "#;

        let rows: Vec<TrackInventoryListItemRow> = sqlx::query_as(sql)
            .fetch_all(&mut *unit_of_work.tx)
            .await
            .map_err(DomainError::from)?;

        let views = rows
            .into_iter()
            .map(|row| TrackInventoryListItem {
                id: row.id,
                name: row.name.unwrap_or_default(),
                description: row.description,
                total_items: row.total_items,
                total_quantity: row.total_quantity,
            })
            .collect();

        Ok(views)
    }
}

/// Row representation for inventory list query.
#[derive(Debug, sqlx::FromRow)]
struct TrackInventoryListItemRow {
    id: crate::tracks_inventory::domain::TrackInventoryId,
    name: Option<String>,
    description: Option<String>,
    total_items: i64,
    total_quantity: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;

    #[sqlx::test(migrations = "./migrations")]
    async fn list_inventories_empty(pool: sqlx::SqlitePool) {
        let mut uow = SqliteUnitOfWork::new(&pool).await.unwrap();
        let result = GetTrackInventoriesQuery::execute(&mut uow).await.unwrap();
        assert!(result.is_empty());
    }
}
