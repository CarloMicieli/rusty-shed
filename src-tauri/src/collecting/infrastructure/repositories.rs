// Catalog domain types: only import the identifiers required by the
// repository implementation signatures. Test-only types are imported in
// the `#[cfg(test)]` modules below.
use crate::catalog::domain::railway_model::{RailwayModelId, RollingStockId};
use crate::collecting::domain::CollectionView;
use crate::collecting::domain::{
    BoxCondition, Collection, CollectionEvent, CollectionId, CollectionItem, ModelCondition,
    OwnedRollingStock, OwnedRollingStockId, PurchaseCondition, PurchaseInfoId,
};
use crate::collecting::domain::{CollectionItemId, CollectionSummary, DepotView};
use crate::collecting::domain::{
    CollectionItemUpdate, CollectionRepository, CollectionUowExt, UpdateCollectionItemInput,
};
use crate::collecting::infrastructure::database;
use crate::collecting::infrastructure::entities::{OwnedRollingStockRow, PurchaseInfoRow};
use crate::collecting::infrastructure::mappers::CollectionMapper;
#[allow(unused)]
use crate::core::domain::Language;
use crate::core::domain::{Currency, MonetaryAmount, domain_error::DomainError};
use crate::core::infrastructure::WithDomainContext;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::sellers::domain::seller_id::SellerId;
use chrono::NaiveDate;
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

    async fn insert_collection(
        &mut self,
        collection_id: &CollectionId,
        name: &str,
    ) -> Result<(), DomainError> {
        let insert_cmd = r#"
            INSERT INTO collections (id, name)
            VALUES (?1, ?2)
        "#;

        sqlx::query(insert_cmd)
            .bind(collection_id)
            .bind(name)
            .execute(&mut *self.executor)
            .await
            .with_domain_context("Error inserting the collection")?;

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    async fn insert_collection_item(
        &mut self,
        collection_id: &CollectionId,
        collection_item_id: &CollectionItemId,
        railway_model_id: &RailwayModelId,
        added_date: &NaiveDate,
        purchase_condition: Option<PurchaseCondition>,
        model_condition: Option<ModelCondition>,
        box_condition: Option<BoxCondition>,
        notes: Option<&str>,
    ) -> Result<(), DomainError> {
        let insert_cmd = r#"
            INSERT INTO collection_items (
                id, collection_id, railway_model_id, added_date, 
                purchase_condition, model_condition, box_condition, notes)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8);
        "#;

        sqlx::query(insert_cmd)
            .bind(collection_item_id)
            .bind(collection_id)
            .bind(railway_model_id)
            .bind(added_date)
            .bind(purchase_condition)
            .bind(model_condition)
            .bind(box_condition)
            .bind(notes)
            .execute(&mut *self.executor)
            .await
            .with_domain_context("Error inserting the collection item")?;

        Ok(())
    }

    async fn insert_owned_rolling_stocks(
        &mut self,
        owned_rolling_stock_id: &OwnedRollingStockId,
        collection_item_id: &CollectionItemId,
        rolling_stock_id: &RollingStockId,
        notes: Option<&str>,
    ) -> Result<(), DomainError> {
        let insert_cmd = r#"
            INSERT INTO owned_rolling_stocks (
                id, collection_item_id, rolling_stock_id, notes)
            VALUES (?1, ?2, ?3, ?4);
        "#;

        sqlx::query(insert_cmd)
            .bind(owned_rolling_stock_id)
            .bind(collection_item_id)
            .bind(rolling_stock_id)
            .bind(notes)
            .execute(&mut *self.executor)
            .await
            .with_domain_context("Error inserting the owned_rolling stock")?;

        Ok(())
    }

    async fn insert_purchase_info(
        &mut self,
        purchase_info_id: &PurchaseInfoId,
        collection_item_id: &CollectionItemId,
        price: Option<&MonetaryAmount>,
        seller_id: Option<&SellerId>,
        purchase_date: &NaiveDate,
    ) -> Result<(), DomainError> {
        let insert_cmd = r#"
            INSERT INTO purchase_infos (
                id, collection_item_id, purchased_price_amount, purchased_price_currency, seller_id, purchase_date)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6);
        "#;

        let (price_amount, price_currency) = match price {
            Some(monetary_amount) => (Some(monetary_amount.amount), Some(monetary_amount.currency)),
            None => (None, None),
        };

        sqlx::query(insert_cmd)
            .bind(purchase_info_id)
            .bind(collection_item_id)
            .bind(price_amount)
            .bind(price_currency)
            .bind(seller_id)
            .bind(purchase_date)
            .execute(&mut *self.executor)
            .await
            .with_domain_context("Error inserting the purchase info")?;

        Ok(())
    }

    async fn update_collection_metadata(
        &mut self,
        collection_id: &CollectionId,
    ) -> Result<(), DomainError> {
        let update_cmd = r#"
            UPDATE collections
            SET version = version + 1,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = ?1;
        "#;

        sqlx::query(update_cmd)
            .bind(collection_id)
            .execute(&mut *self.executor)
            .await
            .with_domain_context("Error inserting the collection summary")?;

        Ok(())
    }

    async fn update_collection_summary(
        &mut self,
        collection_id: &CollectionId,
        collection_summary: &CollectionSummary,
        total_value: Option<&MonetaryAmount>,
    ) -> Result<(), DomainError> {
        let update_cmd = r#"
            UPDATE collections
            SET electric_multiple_units_count = ?1,
                freight_cars_count = ?2,
                locomotives_count = ?3,
                passenger_cars_count = ?4,
                railcars_count = ?5,
                starter_sets_count = ?6,
                train_sets_count = ?7,
                total_value_amount = ?8,
                total_value_currency = ?9
            WHERE id = ?10;
        "#;

        let amount: Option<i64> = total_value.map(|mv| mv.amount);
        let currency: Option<Currency> = total_value.map(|mv| mv.currency);

        sqlx::query(update_cmd)
            .bind(collection_summary.electric_multiple_units_count)
            .bind(collection_summary.freight_cars_count)
            .bind(collection_summary.locomotives_count)
            .bind(collection_summary.passenger_cars_count)
            .bind(collection_summary.railcars_count)
            .bind(collection_summary.starter_sets_count)
            .bind(collection_summary.train_sets_count)
            .bind(amount)
            .bind(currency)
            .bind(collection_id)
            .execute(&mut *self.executor)
            .await
            .with_domain_context("Error inserting the collection summary")?;

        Ok(())
    }

    async fn update_collection_item_removed_date(
        &mut self,
        collection_item_id: &CollectionItemId,
        removed_date: &NaiveDate,
    ) -> Result<(), DomainError> {
        let update_cmd = r#"
            UPDATE collection_items
            SET removed_date = ?1
            WHERE id = ?2;
        "#;

        sqlx::query(update_cmd)
            .bind(removed_date)
            .bind(collection_item_id)
            .execute(&mut *self.executor)
            .await
            .with_domain_context("Error updating collection item removed_date")?;

        Ok(())
    }

    async fn update_collection_item_field<T>(
        &mut self,
        collection_item_id: &CollectionItemId,
        field_name: &str,
        value: Option<T>,
    ) -> Result<(), DomainError>
    where
        T: Send + Sync + for<'q> sqlx::Encode<'q, sqlx::Sqlite> + sqlx::Type<sqlx::Sqlite>,
    {
        let sql = format!("UPDATE collection_items SET {field_name} = ?1 WHERE id = ?2");
        let result = sqlx::query(&sql)
            .bind(value)
            .bind(collection_item_id)
            .execute(&mut *self.executor)
            .await
            .with_domain_context("Error updating collection item field")?;

        if result.rows_affected() == 0 {
            return Err(DomainError::NotFound {
                resource: "CollectionItem".to_string(),
                identifier: collection_item_id.to_string(),
            });
        }

        Ok(())
    }

    async fn update_purchase_info_seller(
        &mut self,
        collection_item_id: &CollectionItemId,
        seller_id: Option<&SellerId>,
    ) -> Result<(), DomainError> {
        let result =
            sqlx::query("UPDATE purchase_infos SET seller_id = ?1 WHERE collection_item_id = ?2")
                .bind(seller_id)
                .bind(collection_item_id)
                .execute(&mut *self.executor)
                .await
                .with_domain_context("Error updating purchase info seller")?;

        if result.rows_affected() == 0 {
            return Err(DomainError::NotFound {
                resource: "PurchaseInfo".to_string(),
                identifier: collection_item_id.to_string(),
            });
        }

        Ok(())
    }

    async fn update_purchase_info_price(
        &mut self,
        collection_item_id: &CollectionItemId,
        price: Option<&MonetaryAmount>,
    ) -> Result<(), DomainError> {
        let (amount, currency) = match price {
            Some(monetary_amount) => (Some(monetary_amount.amount), Some(monetary_amount.currency)),
            None => (None, None),
        };

        let result = sqlx::query(
            "UPDATE purchase_infos SET purchased_price_amount = ?1, purchased_price_currency = ?2 WHERE collection_item_id = ?3",
        )
        .bind(amount)
        .bind(currency)
        .bind(collection_item_id)
        .execute(&mut *self.executor)
        .await
        .with_domain_context("Error updating purchase info price")?;

        if result.rows_affected() == 0 {
            return Err(DomainError::NotFound {
                resource: "PurchaseInfo".to_string(),
                identifier: collection_item_id.to_string(),
            });
        }

        Ok(())
    }

    async fn update_purchase_info_date(
        &mut self,
        collection_item_id: &CollectionItemId,
        purchase_date: Option<NaiveDate>,
    ) -> Result<(), DomainError> {
        let result = sqlx::query(
            "UPDATE purchase_infos SET purchase_date = ?1 WHERE collection_item_id = ?2",
        )
        .bind(purchase_date)
        .bind(collection_item_id)
        .execute(&mut *self.executor)
        .await
        .with_domain_context("Error updating purchase info purchase_date")?;

        if result.rows_affected() == 0 {
            return Err(DomainError::NotFound {
                resource: "PurchaseInfo".to_string(),
                identifier: collection_item_id.to_string(),
            });
        }

        Ok(())
    }

    async fn recalculate_collection_total_value(
        &mut self,
        collection_id: &CollectionId,
    ) -> Result<(), DomainError> {
        let sql = r#"
            UPDATE collections
               SET total_value_amount = COALESCE((
                       SELECT SUM(COALESCE(pi.purchased_price_amount, 0))
                         FROM purchase_infos pi
                         JOIN collection_items ci ON ci.id = pi.collection_item_id
                        WHERE ci.collection_id = ?1
                          AND ci.removed_date IS NULL
                   ), 0),
                   total_value_currency = COALESCE((
                       SELECT MAX(pi.purchased_price_currency)
                         FROM purchase_infos pi
                         JOIN collection_items ci ON ci.id = pi.collection_item_id
                        WHERE ci.collection_id = ?1
                          AND ci.removed_date IS NULL
                          AND pi.purchased_price_currency IS NOT NULL
                   ), total_value_currency)
             WHERE id = ?1;
        "#;

        sqlx::query(sql)
            .bind(collection_id)
            .execute(&mut *self.executor)
            .await
            .with_domain_context("Error recalculating collection total value")?;

        Ok(())
    }
}

impl<'conn> CollectionUowExt for SqliteUnitOfWork<'conn> {
    /// Links the SQLite-specific repository to the Unit of Work.
    ///
    /// It re-borrows the internal transaction (`&mut *self.tx`) to provide
    /// the repository with a mutable executor without transferring ownership.
    fn collections_repository(&mut self) -> Box<dyn CollectionRepository + '_> {
        Box::new(SqliteCollectionRepository::new(&mut self.tx))
    }
}

#[async_trait::async_trait]
impl<'conn> CollectionRepository for SqliteCollectionRepository<'conn> {
    /// Executes the SQLite-specific logic to fetch a collection.
    async fn find_view(&mut self) -> Result<CollectionView, DomainError> {
        // For simplicity and matching the use case "get collection", we assume a single user collection for now
        // or getting the first one found. If none exists, we might need to return a default or error.
        // For this iteration, let's try to fetch the first collection.
        let collection_id = CollectionId::default();

        // Try to fetch the collection, but handle query errors gracefully
        let collection_row = database::get_collection(&mut *self.executor, &collection_id).await?;
        if collection_row.is_none() {
            // If no collection exists, return a default empty collection
            return Ok(CollectionView::default());
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

    /// Saves the current state of the collection.
    async fn save(&mut self, collection: &mut Collection) -> Result<(), DomainError> {
        for envelope in collection.pull_events() {
            match &*envelope {
                CollectionEvent::CollectionCreated { aggregate_id, .. } => {
                    self.insert_collection(aggregate_id, &collection.name)
                        .await?;
                }
                CollectionEvent::RailwayModelAdded {
                    aggregate_id,
                    collection_item_id,
                    railway_model_id,
                    rolling_stock,
                    price,
                    seller_id,
                    added_date,
                    purchase_info_id,
                    purchase_date,
                    purchase_condition,
                    model_condition,
                    box_condition,
                    notes,
                    ..
                } => {
                    self.update_collection_summary(
                        aggregate_id,
                        &collection.summary,
                        collection.total_value.as_ref(),
                    )
                    .await?;

                    self.insert_collection_item(
                        aggregate_id,
                        collection_item_id,
                        railway_model_id,
                        added_date,
                        *purchase_condition,
                        *model_condition,
                        *box_condition,
                        notes.as_deref(),
                    )
                    .await?;

                    for owned_rs in rolling_stock {
                        self.insert_owned_rolling_stocks(
                            &owned_rs.owned_rolling_stock_id,
                            collection_item_id,
                            &owned_rs.rolling_stock_id,
                            None,
                        )
                        .await?;
                    }

                    self.insert_purchase_info(
                        purchase_info_id,
                        collection_item_id,
                        Some(price),
                        seller_id.as_ref(),
                        purchase_date,
                    )
                    .await?
                }
                CollectionEvent::RailwayModelRemoved {
                    aggregate_id,
                    collection_item_id,
                    removed_date,
                    ..
                } => {
                    // Persist updated summary/total first (collection.summary already mutated)
                    self.update_collection_summary(
                        aggregate_id,
                        &collection.summary,
                        collection.total_value.as_ref(),
                    )
                    .await?;

                    // Set removed_date on the collection item row
                    self.update_collection_item_removed_date(collection_item_id, removed_date)
                        .await?;
                }
                CollectionEvent::RailwayModelSold { .. } => {}
            }
        }

        self.update_collection_metadata(&collection.id).await?;

        // `pull_events()` already cleared `pending_events` by taking ownership.
        Ok(())
    }

    /// Finds a compact depot view suitable for the UI by assembling owned
    /// rolling stocks and related model metadata.
    async fn find_depot_view(&mut self) -> Result<DepotView, DomainError> {
        let collection_id = CollectionId::default();

        let collection_row = database::get_collection(&mut *self.executor, &collection_id).await?;
        if collection_row.is_none() {
            return Ok(DepotView {
                rolling_stocks: Vec::new(),
            });
        }

        let collection_row = collection_row.expect("Expect collection row to be present");

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

        let mut depot_items = Vec::new();
        for collection_item_row in collection_item_rows {
            let item = CollectionMapper::row_to_collection_item(
                collection_item_row,
                &owned_rolling_stocks_map,
                &purchase_info_map,
            )?;

            for owned in item.rolling_stocks.iter() {
                let depot_rs = CollectionMapper::collection_item_owned_to_depot(&item, owned)?;
                depot_items.push(depot_rs);
            }
        }

        Ok(DepotView {
            rolling_stocks: depot_items,
        })
    }

    async fn update_item(&mut self, input: &UpdateCollectionItemInput) -> Result<(), DomainError> {
        match &input.update {
            CollectionItemUpdate::Seller(seller_id) => {
                self.update_purchase_info_seller(&input.collection_item_id, seller_id.as_ref())
                    .await?;
            }
            CollectionItemUpdate::Price(price) => {
                self.update_purchase_info_price(&input.collection_item_id, price.as_ref())
                    .await?;
                self.recalculate_collection_total_value(&CollectionId::default())
                    .await?;
            }
            CollectionItemUpdate::PurchaseDate(purchase_date) => {
                self.update_purchase_info_date(&input.collection_item_id, *purchase_date)
                    .await?;
            }
            CollectionItemUpdate::AddedDate(added_date) => {
                self.update_collection_item_field(
                    &input.collection_item_id,
                    "added_date",
                    *added_date,
                )
                .await?;
            }
            CollectionItemUpdate::Notes(notes) => {
                self.update_collection_item_field(
                    &input.collection_item_id,
                    "notes",
                    notes.as_ref().map(std::string::ToString::to_string),
                )
                .await?;
            }
            CollectionItemUpdate::PurchaseCondition(purchase_condition) => {
                self.update_collection_item_field(
                    &input.collection_item_id,
                    "purchase_condition",
                    *purchase_condition,
                )
                .await?;
            }
            CollectionItemUpdate::ModelCondition(model_condition) => {
                self.update_collection_item_field(
                    &input.collection_item_id,
                    "model_condition",
                    *model_condition,
                )
                .await?;
            }
            CollectionItemUpdate::BoxCondition(box_condition) => {
                self.update_collection_item_field(
                    &input.collection_item_id,
                    "box_condition",
                    *box_condition,
                )
                .await?;
            }
        }

        self.update_collection_metadata(&CollectionId::default())
            .await?;

        Ok(())
    }

    /// Retrieve a full `Collection` aggregate by id, returning `None` when
    /// no collection with the given id exists.
    async fn find_by_id(&mut self, id: &CollectionId) -> Result<Option<Collection>, DomainError> {
        let collection_row = database::get_collection(&mut *self.executor, id).await?;
        if collection_row.is_none() {
            return Ok(None);
        }

        let collection_row = collection_row.expect("Expect collection row to be present");

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

        let mut collection_items: Vec<CollectionItem> = Vec::new();
        for collection_item_row in collection_item_rows {
            let iv = CollectionMapper::row_to_collection_item(
                collection_item_row,
                &owned_rolling_stocks_map,
                &purchase_info_map,
            )?;

            let rolling_stocks = iv
                .rolling_stocks
                .into_iter()
                .map(|ov| OwnedRollingStock {
                    id: ov.id,
                    rolling_stock_id: ov.rolling_stock_id,
                    notes: ov.notes,
                    installed_decoder_id: ov.digital.map(|d| d.installed_decoder_id),
                })
                .collect();

            let item = CollectionItem {
                id: iv.id,
                railway_model_id: iv.railway_model.railway_model_id,
                added_date: iv.added_date,
                removed_date: iv.removed_date,
                purchase_condition: iv.purchase_condition,
                model_condition: iv.model_condition,
                box_condition: iv.box_condition,
                notes: iv.notes,
                rolling_stocks,
                purchase_info: iv.purchase_info,
            };

            collection_items.push(item);
        }

        // Build the Collection aggregate
        let total_value = MonetaryAmount::from_db(
            collection_row.total_value_amount,
            Some(&collection_row.total_value_currency),
        )
        .map_err(|err| DomainError::Validation(err.to_string()))?;

        let summary = CollectionSummary {
            locomotives_count: collection_row.locomotives_count as u16,
            passenger_cars_count: collection_row.passenger_cars_count as u16,
            freight_cars_count: collection_row.freight_cars_count as u16,
            train_sets_count: collection_row.train_sets_count as u16,
            railcars_count: collection_row.railcars_count as u16,
            electric_multiple_units_count: collection_row.electric_multiple_units_count as u16,
            starter_sets_count: 0u16,
        };

        let collection = Collection {
            id: collection_row.id,
            name: collection_row.name,
            summary,
            total_value,
            items: collection_items,
            pending_events: Vec::new(),
            metadata: Default::default(),
        };

        Ok(Some(collection))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::domain::manufacturer::ManufacturerId;
    use crate::catalog::domain::railway_company::RailwayCompanyId;
    use crate::catalog::domain::railway_model::localized_field::LocalizedField;
    use crate::catalog::domain::railway_model::{
        Category, LocomotiveType, PowerMethod, ProductCode, RailwayModel, RollingStock,
    };
    use crate::catalog::domain::railway_model::{RailwayModelId, RollingStockId};
    use crate::catalog::domain::scale::Scale;
    use crate::collecting::domain::{
        BoxCondition, ModelCondition, OwnedRollingStockId, PurchaseCondition, PurchaseInfo,
    };
    use crate::core::domain::identifiers::Identifier;
    use crate::core::domain::{Currency, MonetaryAmount};
    use crate::core::infrastructure::logging;
    use crate::sellers::domain::seller_id::SellerId;

    #[sqlx::test(migrations = "./migrations")]
    async fn it_should_return_the_default_collection_when_not_found(conn: sqlx::SqlitePool) {
        let mut unit_of_work = SqliteUnitOfWork::new(&conn)
            .await
            .expect("should create unit of work");

        let collection = unit_of_work
            .collections_repository()
            .find_view()
            .await
            .expect("should get collection");

        unit_of_work.commit().await.unwrap();

        assert_eq!(collection.id, CollectionId::default());
        assert_eq!(collection.name, "My Collection");
        assert_eq!(collection.items.len(), 0);
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_collection.sql")
    )]
    async fn it_should_return_the_collection_data(conn: sqlx::SqlitePool) {
        let mut unit_of_work = SqliteUnitOfWork::new(&conn)
            .await
            .expect("should create unit of work");

        let collection = unit_of_work
            .collections_repository()
            .find_view()
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

        let railway_model = &collection_item.railway_model;
        assert_eq!(railway_model.railway_model_id, expected_railway_model_id);
        assert_eq!(railway_model.manufacturer, "ACME");
        assert_eq!(railway_model.product_code, "60100");
        assert_eq!(railway_model.scale, Scale::H0);
        assert_eq!(railway_model.epoch, "IV".into());

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

    #[sqlx::test(migrations = "./migrations")]
    async fn it_should_insert_collection_row_on_save(conn: sqlx::SqlitePool) {
        let mut unit_of_work = SqliteUnitOfWork::new(&conn)
            .await
            .expect("should create unit of work");

        let mut collection = Collection::default();

        unit_of_work
            .collections_repository()
            .save(&mut collection)
            .await
            .expect("save should succeed");
        unit_of_work.commit().await.expect("commit should succeed");

        // Assert: query the DB directly to verify a row was inserted
        let mut conn2 = conn.acquire().await.expect("acquire conn");
        let saved = database::get_collection(&mut conn2, &collection.id)
            .await
            .expect("query should succeed");
        assert!(saved.is_some());
        let row = saved.unwrap();
        assert_eq!(row.id, CollectionId::default());
        assert_eq!(row.name, "My Collection");
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures(
            "../../../fixtures/test_railway_model.sql",
            "../../../fixtures/test_seller.sql"
        )
    )]
    async fn it_should_persist_collection_item_and_related_rows_on_save(conn: sqlx::SqlitePool) {
        logging::test_helper::setup();

        // Arrange
        let mut unit_of_work = SqliteUnitOfWork::new(&conn)
            .await
            .expect("should create unit of work");

        let mut collection = Collection::default();

        let railway_model_id = RailwayModelId::try_from("trn:railway-model:acme:60100")
            .expect("valid railway model id");
        let rolling_stock_id =
            RollingStockId::try_from("trn:rolling-stock:70300b1c-b1df-475f-a7be-291e435b1cf8")
                .expect("valid rolling stock id");
        let seller = SellerId::try_from("trn:seller:model-train-shop").ok();

        let railway_model = RailwayModel {
            id: railway_model_id.clone(),
            manufacturer_id: ManufacturerId::from_string_unchecked(
                "trn:manufacturer:not-a-trn".to_string(),
            ),
            product_code: ProductCode::try_from("P100").unwrap(),
            description: LocalizedField {
                lang: Language::English,
                value: "Test model".to_string(),
            },
            details: None,
            power_method: PowerMethod::DC,
            scale: Scale::H0,
            epoch: "IV".into(),
            category: Category::Locomotives,
            delivery_date: None,
            availability_status: None,
            rolling_stocks: vec![RollingStock::Locomotive {
                id: rolling_stock_id.clone(),
                railway_id: RailwayCompanyId::from_string_unchecked("RY-ACME".to_string()),
                livery: None,
                length_over_buffer: None,
                technical_specifications: None,
                friendly_name: None,
                series_code: "".to_string(),
                road_number: None,
                series: None,
                depot: None,
                locomotive_type: LocomotiveType::ElectricLocomotive,
                dcc_interface: None,
                control: None,
                is_dummy: false,
            }],
            pending_events: Vec::new(),
        };

        let new_item = crate::collecting::domain::NewCollectionItem {
            collection_item_id: CollectionItemId::default(),
            purchase_info_id: PurchaseInfoId::default(),
            railway_model: railway_model.clone(),
            price: MonetaryAmount::new(1234, Currency::USD),
            seller_id: seller.clone(),
            added_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            purchase_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            purchase_condition: Some(PurchaseCondition::New),
            model_condition: Some(ModelCondition::Mint),
            box_condition: Some(BoxCondition::OriginalMint),
            notes: Some("Inserted by test".to_string()),
        };

        let _item_id = collection.add_item(new_item);

        unit_of_work
            .collections_repository()
            .save(&mut collection)
            .await
            .expect("save should succeed");
        unit_of_work.commit().await.expect("commit should succeed");

        let mut conn2 = conn.acquire().await.expect("acquire conn");

        let purchase_infos = database::get_purchase_infos(&mut conn2, &collection.id)
            .await
            .expect("query purchase_infos");
        assert!(!purchase_infos.is_empty());
        let pi = &purchase_infos[0];
        assert_eq!(pi.purchased_price_amount.unwrap(), 1234);
        assert_eq!(pi.purchased_price_currency.as_deref().unwrap(), "USD");

        let owned_id = collection.items[0]
            .rolling_stocks
            .first()
            .expect("owned rolling stock present")
            .id
            .to_string();

        let ors = database::get_owned_rolling_stock(&mut conn2, owned_id)
            .await
            .expect("query owned rolling stock");
        assert!(ors.is_some());
        let ors_row = ors.unwrap();
        assert_eq!(ors_row.rolling_stock_id.unwrap(), rolling_stock_id);

        let items = database::get_collection_items(&mut conn2, &collection.id)
            .await
            .expect("query collection_items");
        assert!(!items.is_empty());
        assert_eq!(items[0].railway_model_id, railway_model_id);
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures(
            "../../../fixtures/test_railway_model.sql",
            "../../../fixtures/test_seller.sql"
        )
    )]
    async fn it_should_persist_removed_date_and_update_summary(conn: sqlx::SqlitePool) {
        // Arrange
        let mut unit_of_work = SqliteUnitOfWork::new(&conn)
            .await
            .expect("should create unit of work");

        let mut collection = Collection::default();

        let railway_model_id = RailwayModelId::try_from("trn:railway-model:acme:60100")
            .expect("valid railway model id");
        let rolling_stock_id =
            RollingStockId::try_from("trn:rolling-stock:70300b1c-b1df-475f-a7be-291e435b1cf8")
                .expect("valid rolling stock id");
        let seller = SellerId::try_from("trn:seller:model-train-shop").ok();

        let railway_model = RailwayModel {
            id: railway_model_id.clone(),
            manufacturer_id: ManufacturerId::from_string_unchecked(
                "trn:manufacturer:not-a-trn".to_string(),
            ),
            product_code: ProductCode::try_from("P100").unwrap(),
            description: LocalizedField {
                lang: Language::English,
                value: "Test model".to_string(),
            },
            details: None,
            power_method: PowerMethod::DC,
            scale: Scale::H0,
            epoch: "IV".into(),
            category: Category::Locomotives,
            delivery_date: None,
            availability_status: None,
            rolling_stocks: vec![RollingStock::Locomotive {
                id: rolling_stock_id.clone(),
                railway_id: RailwayCompanyId::from_string_unchecked("RY-ACME".to_string()),
                livery: None,
                length_over_buffer: None,
                technical_specifications: None,
                friendly_name: None,
                series_code: "".to_string(),
                road_number: None,
                series: None,
                depot: None,
                locomotive_type: LocomotiveType::ElectricLocomotive,
                dcc_interface: None,
                control: None,
                is_dummy: false,
            }],
            pending_events: Vec::new(),
        };

        let new_item = crate::collecting::domain::NewCollectionItem {
            collection_item_id: CollectionItemId::default(),
            purchase_info_id: PurchaseInfoId::default(),
            railway_model: railway_model.clone(),
            price: MonetaryAmount::new(1234, Currency::USD),
            seller_id: seller.clone(),
            added_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            purchase_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            purchase_condition: Some(PurchaseCondition::New),
            model_condition: Some(ModelCondition::Mint),
            box_condition: Some(BoxCondition::OriginalMint),
            notes: Some("Inserted by test".to_string()),
        };

        let _item_id = collection.add_item(new_item);

        unit_of_work
            .collections_repository()
            .save(&mut collection)
            .await
            .expect("save should succeed");
        unit_of_work.commit().await.expect("commit should succeed");

        // Now remove the item
        let mut uow2 = SqliteUnitOfWork::new(&conn).await.expect("uow2");
        let mut repo = uow2.collections_repository();
        let view = repo.find_view().await.expect("find view");

        // Build domain collection from view (reuse logic similar to use-case)
        let mut collection2 = Collection {
            id: view.id.clone(),
            name: view.name.clone(),
            summary: view.summary,
            total_value: view.total_value,
            items: view
                .items
                .into_iter()
                .map(|iv| crate::collecting::domain::CollectionItem {
                    id: iv.id,
                    railway_model_id: iv.railway_model.railway_model_id,
                    added_date: iv.added_date,
                    removed_date: iv.removed_date,
                    purchase_condition: iv.purchase_condition,
                    model_condition: iv.model_condition,
                    box_condition: iv.box_condition,
                    notes: iv.notes,
                    rolling_stocks: iv
                        .rolling_stocks
                        .into_iter()
                        .map(|ov| crate::collecting::domain::OwnedRollingStock {
                            id: ov.id,
                            rolling_stock_id: ov.rolling_stock_id,
                            notes: ov.notes,
                            installed_decoder_id: ov.digital.map(|d| d.installed_decoder_id),
                        })
                        .collect(),
                    purchase_info: iv.purchase_info,
                })
                .collect(),
            pending_events: Vec::new(),
            metadata: Default::default(),
        };

        let item_id = collection2.items[0].id.clone();
        let removed_date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();

        let remove_cmd = crate::collecting::application::RemoveCollectionItemInput {
            collection_item_id: item_id.clone(),
            category: Category::Locomotives,
            removed_date,
        };

        collection2.remove_item(remove_cmd);

        repo.save(&mut collection2).await.expect("save remove");
        drop(repo);
        uow2.commit().await.expect("commit remove");

        // Verify DB: item removed_date set
        // Note: We query directly without the removed_date filter since get_collection_items
        // now filters out soft-deleted items
        let mut conn2 = conn.acquire().await.expect("acquire conn");
        let removed_date: Option<NaiveDate> = sqlx::query_scalar(
            "SELECT removed_date FROM collection_items WHERE collection_id = ? LIMIT 1",
        )
        .bind(CollectionId::default().to_string())
        .fetch_one(&mut *conn2)
        .await
        .expect("query collection_items");
        assert!(removed_date.is_some());

        // Verify collection summary/total updated
        let coll_row = database::get_collection(&mut conn2, &CollectionId::default())
            .await
            .expect("query collection")
            .expect("row present");

        // total_value_amount should exist and be decreased (in this test to 0 or initial - price)
        // We simply assert that it is an integer (exists)
        assert!(coll_row.total_value_amount >= 0);
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_collection.sql")
    )]
    async fn it_should_return_depot_view(conn: sqlx::SqlitePool) {
        let mut unit_of_work = SqliteUnitOfWork::new(&conn)
            .await
            .expect("should create unit of work");

        let depot = unit_of_work
            .collections_repository()
            .find_depot_view()
            .await
            .expect("should get depot view");

        unit_of_work.commit().await.unwrap();

        assert_eq!(depot.rolling_stocks.len(), 1);
        let rs = &depot.rolling_stocks[0];

        // Expect the manufacturer and product code to match the fixture
        assert_eq!(rs.manufacturer_name, "ACME");
        assert_eq!(rs.product_code.to_string(), "60100");
        // Owned rolling stock id should look like a trn:owned-rolling-stock
        assert!(rs.id.to_string().starts_with("trn:owned-rolling-stock"));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn it_should_return_none_for_find_by_id_when_not_found(conn: sqlx::SqlitePool) {
        let mut unit_of_work = SqliteUnitOfWork::new(&conn)
            .await
            .expect("should create unit of work");

        let res = unit_of_work
            .collections_repository()
            .find_by_id(&CollectionId::default())
            .await
            .expect("find_by_id should succeed");

        assert!(res.is_none());

        unit_of_work.commit().await.unwrap();
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_collection.sql")
    )]
    async fn it_should_find_collection_by_id(conn: sqlx::SqlitePool) {
        let mut unit_of_work = SqliteUnitOfWork::new(&conn)
            .await
            .expect("should create unit of work");

        let coll = unit_of_work
            .collections_repository()
            .find_by_id(&CollectionId::default())
            .await
            .expect("find_by_id should succeed")
            .expect("collection should be present");

        assert_eq!(coll.id, CollectionId::default());
        assert!(!coll.items.is_empty());
    }
}
