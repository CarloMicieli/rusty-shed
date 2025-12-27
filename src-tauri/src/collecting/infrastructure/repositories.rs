use crate::collecting::domain::collection::Collection;
use crate::collecting::domain::collection_id::CollectionId;
use crate::collecting::domain::collection_item_id::CollectionItemId;
use crate::collecting::domain::repository::CollectionRepository;
use crate::collecting::infrastructure::database;
use crate::collecting::infrastructure::mappers::CollectionMapper;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use anyhow::anyhow;
use itertools::Itertools;

/// An SQLite-specific implementation of the `CollectionRepository`.
///
/// It holds a mutable reference to a connection, which in this architecture
/// is provided by the `SqliteUnitOfWork`'s active transaction.
pub struct SqliteCollectionRepository<'conn> {
    /// A mutable reference to the database connection/executor.
    executor: &'conn mut sqlx::SqliteConnection,
}

impl<'conn> SqliteCollectionRepository<'conn> {
    /// Creates a new repository instance using the provided executor.
    pub fn new(executor: &'conn mut sqlx::SqliteConnection) -> Self {
        Self { executor }
    }
}

#[async_trait::async_trait]
impl<'conn> CollectionRepository for SqliteCollectionRepository<'conn> {
    /// Executes the SQLite-specific logic to fetch a collection.
    async fn get_collection(&mut self) -> anyhow::Result<Collection> {
        // For simplicity and matching the use case "get collection", we assume a single user collection for now
        // or getting the first one found. If none exists, we might need to return a default or error.
        // For this iteration, let's try to fetch the first collection.
        let collection_id = CollectionId::default();

        let collection_row = database::get_collection(&mut *self.executor, &collection_id).await?;
        if collection_row.is_none() {
            // Return an empty collection structure if no DB entry exists yet
            return Ok(Collection::default());
        }

        let collection_row =
            collection_row.expect("Expect collection row to be present after None check");
        let collection_id = CollectionId::try_from(&collection_row.id).map_err(|e| anyhow!(e))?;
        let collection_item_rows =
            database::get_collection_items(&mut *self.executor, &collection_id).await?;

        let owned_rolling_stock_rows =
            database::get_owned_rolling_stocks(&mut *self.executor, &collection_id).await?;
        let owned_rolling_stocks_map = owned_rolling_stock_rows
            .into_iter()
            .map(|owned_rs| {
                (
                    CollectionItemId::try_from(&owned_rs.collection_item_id).unwrap(),
                    owned_rs,
                )
            })
            .into_group_map();

        let purchase_info_rows =
            database::get_purchase_infos(&mut *self.executor, &collection_id).await?;
        let purchase_info_map = purchase_info_rows
            .into_iter()
            .map(|purchase_info| {
                (
                    CollectionItemId::try_from(&purchase_info.collection_item_id).unwrap(),
                    purchase_info,
                )
            })
            .into_group_map();

        let mut collection_items = Vec::new();
        for collection_item_row in collection_item_rows {
            let item = CollectionMapper::row_to_collection_item(
                collection_item_row,
                &owned_rolling_stocks_map,
                &purchase_info_map,
            )?;
            collection_items.push(item);
        }

        CollectionMapper::row_to_collection(collection_row, collection_items)
    }
}

/// An extension trait that provides access to the `CollectionRepository`.
///
/// This follows the **Interface Segregation Principle**. By using extension traits,
/// we avoid a "God Object" where one struct knows about every repository in the
/// system. Instead, repositories are grouped by domain logic.
pub trait CollectingUowExt {
    /// Returns a trait object for interacting with collection data.
    ///
    /// The repository is bound to the lifetime of the Unit of Work to ensure
    /// it cannot outlive the transaction it relies on.
    fn collection_repo(&mut self) -> Box<dyn CollectionRepository + '_>;
}

impl<'conn> CollectingUowExt for SqliteUnitOfWork<'conn> {
    /// Links the SQLite-specific repository to the Unit of Work.
    ///
    /// It re-borrows the internal transaction (`&mut *self.tx`) to provide
    /// the repository with a mutable executor without transferring ownership.
    fn collection_repo(&mut self) -> Box<dyn CollectionRepository + '_> {
        Box::new(SqliteCollectionRepository::new(&mut self.tx))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collecting::domain::purchase_info::PurchaseInfo;
    use crate::core::domain::Currency;

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_collection_empty(conn: sqlx::SqlitePool) {
        let mut uow = SqliteUnitOfWork::new(&conn).await.unwrap();

        let collection = uow.collection_repo().get_collection().await.unwrap();

        uow.commit().await.unwrap();

        assert_eq!(collection.name, "My Collection");
        assert_eq!(collection.items.len(), 0);
    }

    #[sqlx::test(migrations = "./migrations", fixtures("test_collection"))]
    async fn it_should_get_collection_with_data(conn: sqlx::SqlitePool) {
        let mut uow = SqliteUnitOfWork::new(&conn).await.unwrap();

        let collection = uow.collection_repo().get_collection().await.unwrap();

        uow.commit().await.unwrap();

        assert_eq!(collection.id, CollectionId::default());
        assert_eq!(collection.summary.locomotives_count, 0);
        assert_eq!(collection.summary.passenger_cars_count, 0);
        assert_eq!(collection.summary.freight_cars_count, 0);
        assert_eq!(collection.summary.train_sets_count, 0);
        assert_eq!(collection.summary.railcars_count, 0);
        assert_eq!(collection.summary.electric_multiple_units_count, 0);
        assert!(collection.total_value.is_some());
        assert_eq!(collection.items.len(), 1);
        assert_eq!(
            collection.items[0].railway_model_id,
            "trn:railway-model:acme:60100".to_string()
        );

        assert_eq!(collection.items[0].rolling_stocks.len(), 1);
        assert_eq!(
            collection.items[0].rolling_stocks[0].rolling_stock_id,
            "rs-001".to_string()
        );

        assert!(collection.items[0].purchase_info.is_some());
        let purchase_info = collection.items[0].purchase_info.as_ref().unwrap();
        match purchase_info {
            PurchaseInfo::Purchased(purchased_info) => {
                assert_eq!(purchased_info.id, "59adc26d-0274-4d6b-8c14-61e598d3fe0e");
                let price = purchased_info.price.as_ref().expect("price present");
                assert_eq!(price.amount, 17500);
                assert_eq!(price.currency, Currency::EUR);
                assert_eq!(purchased_info.seller, None);
            }
            other => panic!("Expected purchase info to be Purchased, got: {:?}", other),
        }
    }
}
