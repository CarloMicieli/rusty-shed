use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::dashboard::domain::{
    DashboardDepotEntry, DashboardRecentItem, DashboardRepository, DashboardSummary,
    DashboardUowExt, QueryParams,
};
use crate::dashboard::infrastructure::DashboardDepotEntryRow;
use crate::dashboard::infrastructure::entities::{DashboardRecentItemRow, DashboardTotalsRow};
use sqlx::SqliteConnection;

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

    /// Fetches a list of dashboard depot entries limited by the specified number of entries.
    ///
    /// It should retrieve some pseudo-random selection of depot entries for display on the dashboard.
    /// If the number of elements in the depot is less than the requested number, it returns all available entries.
    ///
    /// # Arguments
    /// * `number_of_entries` - The maximum number of depot entries to retrieve.
    ///
    /// # Returns
    /// - A vector of `DashboardDepotEntry` instances on success.
    /// - A `DomainError` if the query fails.
    async fn find_depot_entries(
        &mut self,
        number_of_entries: u8,
    ) -> Result<Vec<DashboardDepotEntryRow>, DomainError> {
        let sql = r#"
            SELECT DISTINCT
                rm.id,
                rm.manufacturer_id,
                m.name AS manufacturer_name,
                rm.product_code,
                rm.category,
                rm.scale,
                rm.epoch,
                rs.railway_company_id,
                rc.name AS railway_company_name,
                rc.country_code AS railway_company_country_code,
                rm.description,
                rm.power_method
            FROM railway_models rm
            JOIN rolling_stocks rs ON rm.id = rs.railway_model_id
            JOIN manufacturers m ON rm.manufacturer_id = m.id
            JOIN railway_companies rc ON rs.railway_company_id = rc.id
            ORDER BY RANDOM()
            LIMIT ?1
        "#;

        let rows = sqlx::query_as::<_, DashboardDepotEntryRow>(sql)
            .bind(number_of_entries)
            .fetch_all(&mut *self.executor)
            .await?;

        Ok(rows)
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
                rm.description AS subtitle,
                'COLLECTION' AS source,
                rm.created_at
            FROM railway_models rm
            ORDER BY rm.created_at DESC
            LIMIT ?1
        "#;

        let rows = sqlx::query_as::<_, DashboardRecentItemRow>(sql)
            .bind(number_of_items)
            .fetch_all(&mut *self.executor)
            .await?;

        Ok(rows)
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

        let depot_items = self
            .find_depot_entries(params.number_of_depot_entries)
            .await?
            .into_iter()
            .map(|row| row.try_into())
            .collect::<Result<Vec<DashboardDepotEntry>, DomainError>>()?;

        Ok(DashboardSummary {
            totals,
            recent_items,
            depot_items,
        })
    }
}

impl<'conn> DashboardUowExt for SqliteUnitOfWork<'conn> {
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
            number_of_depot_entries: 1,
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

        let depot_items = summary.depot_items;
        assert_eq!(depot_items.len(), 0);
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
            number_of_depot_entries: 1,
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

        let depot_items = summary.depot_items;
        assert_eq!(depot_items.len(), 1);
    }
}
