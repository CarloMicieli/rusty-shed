use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::dashboard::domain::{
    DashboardRecentItem, DashboardRepository, DashboardSummary, DashboardUowExt, QueryParams,
};
use crate::dashboard::infrastructure::entities::{
    DashboardRecentItemRow, DashboardTotalsRow, ModelCardRow, PurchaseGroupRow,
};
use crate::media::infrastructure::ImageRepository;
use sqlx::SqliteConnection;
use std::path::Path;

/// An SQLite-specific implementation of the `DashboardRepository`.
///
/// It holds a mutable reference to a connection, which in this architecture
/// is provided by the `SqliteUnitOfWork`'s active transaction.
pub struct SqliteDashboardRepository<'conn> {
    /// A mutable reference to the database connection/executor.
    executor: &'conn mut SqliteConnection,
}

impl<'conn> SqliteDashboardRepository<'conn> {
    /// Creates a new instance of the `SqliteDashboardRepository`.
    ///
    /// # Arguments
    /// * `executor` - A mutable reference to the database connection/executor.
    ///
    /// # Returns
    /// A new `SqliteManufacturerRepository` instance.
    pub fn new(executor: &'conn mut SqliteConnection) -> Self {
        Self { executor }
    }

    /// Fetches aggregated totals for the user's dashboard.
    ///
    /// It retrieves various summary statistics about the user's collection.
    ///
    /// # Returns
    /// - A `DashboardTotalsRow` instance on success.
    /// - A `DomainError` if the query fails.
    async fn find_dashboard_totals(&mut self) -> Result<DashboardTotalsRow, DomainError> {
        let sql = r#"
            SELECT
                locomotives_count
                + passenger_cars_count
                + freight_cars_count 
                + train_sets_count 
                + railcars_count 
                + electric_multiple_units_count AS collection_items,
                0 AS wishlists,
                0 AS maintenance_due,
                c.total_value_amount,
                c.total_value_currency
            FROM collections AS c
        "#;

        let row = sqlx::query_as::<_, DashboardTotalsRow>(sql)
            .fetch_optional(&mut *self.executor)
            .await?;

        Ok(row.unwrap_or_default())
    }

    /// Fetches the most recently added item to the user's depot.
    ///
    /// It retrieves the latest item based on the creation timestamp, limited by the specified number of items.
    ///
    /// # Arguments
    /// * `number_of_items` - The number of recent items to retrieve.
    ///
    /// # Returns
    /// - A vector of `DashboardRecentItemRow` instances on success.
    /// - A `DomainError` if the query fails.
    async fn find_recent_item(
        &mut self,
        number_of_items: u8,
    ) -> Result<Vec<DashboardRecentItemRow>, DomainError> {
        let sql = r#"
            SELECT
                rm.id,
                rm.category AS title,
                t.description AS subtitle,
                'COLLECTION' AS source,
                rm.created_at
            FROM railway_models rm
            LEFT JOIN railway_model_translations t ON t.railway_model_id = rm.id AND t.language_code = 'en'
            ORDER BY rm.created_at DESC
            LIMIT ?1
        "#;

        let rows = sqlx::query_as::<_, DashboardRecentItemRow>(sql)
            .bind(number_of_items)
            .fetch_all(&mut *self.executor)
            .await?;

        Ok(rows)
    }

    /// Fetches purchase groups (models grouped by purchase date + seller).
    ///
    /// This query groups collection items by their purchase date and seller,
    /// limiting to the 3 most recent purchase events.
    ///
    /// # Returns
    /// - A vector of tuples (PurchaseGroupRow, Vec<ModelCardRow>) on success.
    /// - A `DomainError` if the query fails.
    async fn fetch_purchase_groups(
        &mut self,
        models_dir: &Path,
    ) -> Result<Vec<(PurchaseGroupRow, Vec<ModelCardRow>)>, DomainError> {
        // First, get the top 3 purchase groups
        let groups_sql = r#"
            SELECT
                pi.purchase_date,
                pi.seller_id,
                s.name AS seller_name,
                ci.notes,
                COUNT(DISTINCT ci.id) AS model_count
            FROM purchase_infos pi
            JOIN collection_items ci ON pi.collection_item_id = ci.id
            LEFT JOIN sellers s ON pi.seller_id = s.id
            WHERE ci.removed_date IS NULL
            GROUP BY pi.purchase_date, pi.seller_id
            ORDER BY pi.purchase_date DESC
            LIMIT 3
        "#;

        let groups = sqlx::query_as::<
            _,
            crate::dashboard::infrastructure::entities::PurchaseGroupRow,
        >(groups_sql)
        .fetch_all(&mut *self.executor)
        .await?;

        let mut result = Vec::new();

        // For each group, fetch up to 3 model cards
        for group in groups {
            let models_sql = r#"
                SELECT
                    ci.id AS collection_item_id,
                    rm.id AS railway_model_id,
                    m.name AS manufacturer_name,
                    rm.product_code,
                    COALESCE(t.description, '') AS description,
                    NULL AS image_path,
                    ci.purchase_condition,
                    rm.category,
                    rm.power_method,
                    rm.scale,
                    rm.epoch AS era,
                    pi.purchased_price_amount AS price_amount,
                    pi.purchased_price_currency AS price_currency
                FROM purchase_infos pi
                JOIN collection_items ci ON pi.collection_item_id = ci.id
                JOIN railway_models rm ON ci.railway_model_id = rm.id
                JOIN manufacturers m ON rm.manufacturer_id = m.id
                LEFT JOIN railway_model_translations t ON t.railway_model_id = rm.id AND t.language_code = 'en'
                WHERE pi.purchase_date = ?1
                  AND (
                    (pi.seller_id IS NULL AND ?2 IS NULL)
                    OR (pi.seller_id = ?2)
                  )
                  AND ci.removed_date IS NULL
                ORDER BY ci.added_date DESC
                LIMIT 3
            "#;

            let mut models = sqlx::query_as::<_, ModelCardRow>(models_sql)
                .bind(&group.purchase_date)
                .bind(&group.seller_id)
                .fetch_all(&mut *self.executor)
                .await?;

            // Resolve images
            let image_repo = ImageRepository;
            for model in &mut models {
                if let Ok(path) = image_repo
                    .find_image(&model.railway_model_id, models_dir)
                    .await
                {
                    model.image_path = path.to_str().map(String::from);
                }
            }

            result.push((group, models));
        }

        Ok(result)
    }
}

#[async_trait::async_trait]
impl<'conn> DashboardRepository for SqliteDashboardRepository<'conn> {
    async fn find_summary(&mut self, params: QueryParams) -> Result<DashboardSummary, DomainError> {
        let totals = self.find_dashboard_totals().await?.try_into()?;

        let recent_items: Vec<DashboardRecentItem> = self
            .find_recent_item(params.number_of_recent_items)
            .await?
            .into_iter()
            .map(|row| row.try_into())
            .collect::<Result<Vec<DashboardRecentItem>, DomainError>>()?;

        let purchase_groups: Vec<crate::dashboard::domain::PurchaseGroup> = self
            .fetch_purchase_groups(&params.models_dir)
            .await?
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(DashboardSummary {
            totals,
            recent_items,
            purchase_groups,
        })
    }
}

impl DashboardUowExt for SqliteUnitOfWork {
    /// Links the SQLite-specific repository to the Unit of Work.
    ///
    /// It re-borrows the internal transaction (`&mut *self.tx`) to provide
    /// the repository with a mutable executor without transferring ownership.
    fn dashboard_repository(&mut self) -> Box<dyn DashboardRepository + '_> {
        Box::new(SqliteDashboardRepository::new(&mut self.tx))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collecting::domain::CollectionItemId;
    use crate::core::domain::Currency;
    use pretty_assertions::assert_eq;

    #[sqlx::test(migrations = "./migrations")]
    async fn it_should_retrieve_empty_dashboard_summary_when_database_is_empty(
        pool: sqlx::SqlitePool,
    ) {
        let mut conn = pool.acquire().await.expect("should acquire connection");

        let mut repo = SqliteDashboardRepository::new(&mut conn);
        let params = QueryParams {
            number_of_recent_items: 1,
            models_dir: std::path::PathBuf::from("/tmp"),
        };

        let summary = repo
            .find_summary(params)
            .await
            .expect("should retrieve summary");

        let totals = summary.totals;
        assert_eq!(totals.collection_items, 0);
        assert_eq!(totals.wishlists, 0);
        assert_eq!(totals.maintenance_due, 0);
        assert_eq!(totals.total_value.is_none(), true);

        let recent_items = summary.recent_items;
        assert_eq!(recent_items.len(), 0);
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_dashboard.sql")
    )]
    async fn it_should_retrieve_dashboard_summary(pool: sqlx::SqlitePool) {
        let mut conn = pool.acquire().await.expect("should acquire connection");

        let mut repo = SqliteDashboardRepository::new(&mut conn);
        let params = QueryParams {
            number_of_recent_items: 1,
            models_dir: std::path::PathBuf::from("/tmp"),
        };

        let summary = repo
            .find_summary(params)
            .await
            .expect("should retrieve summary");

        let totals = summary.totals;
        assert_eq!(totals.collection_items, 2);
        assert_eq!(totals.wishlists, 0);
        assert_eq!(totals.maintenance_due, 0);
        assert_eq!(totals.total_value.is_some(), true);

        let total_value = totals.total_value.unwrap();
        assert_eq!(total_value.amount, 20000);
        assert_eq!(total_value.currency, Currency::EUR);

        let recent_items = summary.recent_items;
        assert_eq!(recent_items.len(), 1);

        let purchase_groups = summary.purchase_groups;
        assert_eq!(purchase_groups.len(), 2);

        let first_group = &purchase_groups[0];
        assert_eq!(first_group.model_cards.len(), 1);

        let expected_collection_item_id =
            CollectionItemId::try_from("trn:collection-item:a6d749a5-f3e9-44e1-9963-35ad90d4b83a")
                .expect("fixture should contain a valid collection item id");
        assert_eq!(first_group.model_cards[0].id, expected_collection_item_id);
    }
}
