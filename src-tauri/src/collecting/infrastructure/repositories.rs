use crate::collecting::domain::Collection;
use crate::collecting::domain::CollectionId;
use crate::collecting::domain::CollectionItemId;
use crate::collecting::domain::CollectionRepository;
use crate::collecting::infrastructure::database;
use crate::collecting::infrastructure::entities::{OwnedRollingStockRow, PurchaseInfoRow};
use crate::collecting::infrastructure::mappers::CollectionMapper;
use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use itertools::Itertools;
use std::collections::HashMap;

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
    async fn get_collection(&mut self) -> Result<Collection, DomainError> {
        // For simplicity and matching the use case "get collection", we assume a single user collection for now
        // or getting the first one found. If none exists, we might need to return a default or error.
        // For this iteration, let's try to fetch the first collection.
        let collection_id = CollectionId::default();

        // Try to fetch the collection, but handle query errors gracefully
        let collection_row = database::get_collection(&mut *self.executor, &collection_id).await?;
        if collection_row.is_none() {
            // If no collection exists, return a default empty collection
            return Ok(Collection::default());
        }

        let collection_row =
            collection_row.expect("Expect collection row to be present after None check");

        let owned_rolling_stock_rows =
            database::get_owned_rolling_stocks(&mut *self.executor, &collection_row.id).await?;
        let owned_rolling_stocks_map: HashMap<CollectionItemId, Vec<OwnedRollingStockRow>> =
            owned_rolling_stock_rows
                .into_iter()
                .map(|owned_rs| (owned_rs.collection_item_id.clone(), owned_rs))
                .into_group_map();

        let purchase_info_rows =
            database::get_purchase_infos(&mut *self.executor, &collection_row.id).await?;
        let purchase_info_map: HashMap<CollectionItemId, Vec<PurchaseInfoRow>> = purchase_info_rows
            .into_iter()
            .map(|purchase_info| (purchase_info.collection_item_id.clone(), purchase_info))
            .into_group_map();

        let collection_item_rows =
            database::get_collection_items(&mut *self.executor, &collection_row.id).await?;
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
    use crate::catalog::domain::railway_model::{RailwayModelId, RollingStockId};
    use crate::collecting::domain::{
        BoxCondition, ModelCondition, OwnedRollingStockId, PurchaseCondition, PurchaseInfo,
    };
    use crate::core::domain::{Currency, MonetaryAmount};

    #[sqlx::test(migrations = "./migrations")]
    async fn it_should_return_the_default_collection_when_not_found(conn: sqlx::SqlitePool) {
        let mut unit_of_work = SqliteUnitOfWork::new(&conn)
            .await
            .expect("should create unit of work");

        let collection = unit_of_work
            .collection_repo()
            .get_collection()
            .await
            .expect("should get collection");

        unit_of_work.commit().await.unwrap();

        assert_eq!(collection.id, CollectionId::default());
        assert_eq!(collection.name, "My Collection");
        assert_eq!(collection.items.len(), 0);
    }

    #[sqlx::test(migrations = "./migrations", fixtures("test_collection"))]
    async fn it_should_return_the_collection_data(conn: sqlx::SqlitePool) {
        let mut unit_of_work = SqliteUnitOfWork::new(&conn)
            .await
            .expect("should create unit of work");

        let collection = unit_of_work
            .collection_repo()
            .get_collection()
            .await
            .expect("should get collection");

        assert_eq!(collection.id, CollectionId::default());
        assert_eq!(collection.summary.locomotives_count, 0);
        assert_eq!(collection.summary.passenger_cars_count, 0);
        assert_eq!(collection.summary.freight_cars_count, 0);
        assert_eq!(collection.summary.train_sets_count, 0);
        assert_eq!(collection.summary.railcars_count, 0);
        assert_eq!(collection.summary.electric_multiple_units_count, 0);

        let expected_total_value = MonetaryAmount::new(0, Currency::EUR);
        assert_eq!(collection.total_value, Some(expected_total_value));

        assert_eq!(collection.items.len(), 1);
        let expected_railway_model_id = RailwayModelId::try_from("trn:railway-model:acme:60100")
            .expect("valid railway model id");
        let collection_item = &collection.items[0];
        assert_eq!(collection_item.railway_model_id, expected_railway_model_id);
        assert_eq!(collection_item.model_condition, Some(ModelCondition::Mint));
        assert_eq!(
            collection_item.box_condition,
            Some(BoxCondition::OriginalMint)
        );
        assert_eq!(
            collection_item.purchase_condition,
            Some(PurchaseCondition::New)
        );
        assert_eq!(
            collection_item.notes,
            Some(String::from("My notes go here"))
        );

        assert_eq!(collection_item.rolling_stocks.len(), 1);
        let rolling_stocks = &collection_item.rolling_stocks[0];

        let expected_owned_rolling_stock_id = OwnedRollingStockId::try_from(
            "trn:owned-rolling-stock:77122924-783e-4f3c-a6b5-f4caec9e695d",
        )
        .expect("valid owned rolling stock id");
        let expected_rolling_stock_id =
            RollingStockId::try_from("trn:rolling-stock:70300b1c-b1df-475f-a7be-291e435b1cf8")
                .expect("valid rolling stock id");
        assert_eq!(rolling_stocks.id, expected_owned_rolling_stock_id);
        assert_eq!(rolling_stocks.rolling_stock_id, expected_rolling_stock_id);
        assert_eq!(
            rolling_stocks.notes,
            Some(String::from("My rolling stock notes go here"))
        );

        assert!(collection_item.purchase_info.is_some());
        let purchase_info = collection_item
            .purchase_info
            .as_ref()
            .expect("should be present");
        match purchase_info {
            PurchaseInfo::Purchased(purchased_info) => {
                assert_eq!(
                    purchased_info.id.to_string(),
                    "trn:purchase:59adc26d-0274-4d6b-8c14-61e598d3fe0e"
                );
                let price = purchased_info.price.as_ref().expect("price present");
                assert_eq!(price.amount, 17500);
                assert_eq!(price.currency, Currency::EUR);
                assert_eq!(purchased_info.seller, None);
            }
            other => panic!("Expected purchase info to be Purchased, got: {:?}", other),
        }
    }
}
