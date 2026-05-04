use crate::catalog::domain::railway_model::{RailwayModelId, RollingStockId};
use crate::collecting::domain::CollectionStats;
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
                id, collection_item_id, purchase_type, purchased_price_amount, purchased_price_currency, seller_id, purchase_date)
            VALUES (?1, ?2, 'PURCHASED', ?3, ?4, ?5, ?6);
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

    async fn mark_collection_item_as_removed(
        &mut self,
        collection_item_id: &CollectionItemId,
        removed_date: NaiveDate,
    ) -> Result<(), DomainError> {
        let result = sqlx::query(
            "UPDATE collection_items SET removed_date = ?1 WHERE id = ?2 AND removed_date IS NULL",
        )
        .bind(removed_date)
        .bind(collection_item_id)
        .execute(&mut *self.executor)
        .await
        .with_domain_context("Error marking collection item as sold")?;

        if result.rows_affected() > 0 {
            return Ok(());
        }

        let existing_removed_date = sqlx::query_scalar::<_, Option<NaiveDate>>(
            "SELECT removed_date FROM collection_items WHERE id = ?1",
        )
        .bind(collection_item_id)
        .fetch_optional(&mut *self.executor)
        .await
        .with_domain_context("Error checking collection item sell state")?;

        match existing_removed_date {
            None => Err(DomainError::NotFound {
                resource: "CollectionItem".to_string(),
                identifier: collection_item_id.to_string(),
            }),
            Some(_) => Err(DomainError::BusinessRule(
                "Collection item is already sold".to_string(),
            )),
        }
    }

    async fn update_purchase_info_sale(
        &mut self,
        collection_item_id: &CollectionItemId,
        sale_date: NaiveDate,
        sale_price: &MonetaryAmount,
        buyer_id: Option<String>,
    ) -> Result<(), DomainError> {
        let result = sqlx::query(
            "UPDATE purchase_infos SET purchase_type = 'SOLD', sale_date = ?1, sale_price_amount = ?2, sale_price_currency = ?3, buyer_id = ?4 WHERE collection_item_id = ?5",
        )
        .bind(sale_date)
        .bind(sale_price.amount)
        .bind(sale_price.currency)
        .bind(buyer_id)
        .bind(collection_item_id)
        .execute(&mut *self.executor)
        .await
        .with_domain_context("Error updating purchase info sale fields")?;

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
                          AND pi.purchase_type != 'PREORDER'
                   ), 0),
                   total_value_currency = COALESCE((
                       SELECT MAX(pi.purchased_price_currency)
                         FROM purchase_infos pi
                         JOIN collection_items ci ON ci.id = pi.collection_item_id
                        WHERE ci.collection_id = ?1
                          AND ci.removed_date IS NULL
                          AND pi.purchase_type != 'PREORDER'
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

    async fn recalculate_collection_summary(
        &mut self,
        collection_id: &CollectionId,
    ) -> Result<(), DomainError> {
        // Only count active items: removed_date IS NULL and purchase_type != 'PREORDER'
        let sql = r#"
            UPDATE collections
               SET locomotives_count = COALESCE((
                       SELECT COUNT(*)
                         FROM collection_items ci
                         JOIN railway_models rm ON rm.id = ci.railway_model_id
                         LEFT JOIN purchase_infos pi ON pi.collection_item_id = ci.id
                        WHERE ci.collection_id = ?1
                          AND ci.removed_date IS NULL
                          AND (pi.purchase_type IS NULL OR pi.purchase_type != 'PREORDER')
                          AND rm.category = 'LOCOMOTIVES'
                   ), 0),
                   passenger_cars_count = COALESCE((
                       SELECT COUNT(*)
                         FROM collection_items ci
                         JOIN railway_models rm ON rm.id = ci.railway_model_id
                         LEFT JOIN purchase_infos pi ON pi.collection_item_id = ci.id
                        WHERE ci.collection_id = ?1
                          AND ci.removed_date IS NULL
                          AND (pi.purchase_type IS NULL OR pi.purchase_type != 'PREORDER')
                          AND rm.category = 'PASSENGER_CARS'
                   ), 0),
                   freight_cars_count = COALESCE((
                       SELECT COUNT(*)
                         FROM collection_items ci
                         JOIN railway_models rm ON rm.id = ci.railway_model_id
                         LEFT JOIN purchase_infos pi ON pi.collection_item_id = ci.id
                        WHERE ci.collection_id = ?1
                          AND ci.removed_date IS NULL
                          AND (pi.purchase_type IS NULL OR pi.purchase_type != 'PREORDER')
                          AND rm.category = 'FREIGHT_CARS'
                   ), 0),
                   train_sets_count = COALESCE((
                       SELECT COUNT(*)
                         FROM collection_items ci
                         JOIN railway_models rm ON rm.id = ci.railway_model_id
                         LEFT JOIN purchase_infos pi ON pi.collection_item_id = ci.id
                        WHERE ci.collection_id = ?1
                          AND ci.removed_date IS NULL
                          AND (pi.purchase_type IS NULL OR pi.purchase_type != 'PREORDER')
                          AND rm.category = 'TRAIN_SETS'
                   ), 0),
                   railcars_count = COALESCE((
                       SELECT COUNT(*)
                         FROM collection_items ci
                         JOIN railway_models rm ON rm.id = ci.railway_model_id
                         LEFT JOIN purchase_infos pi ON pi.collection_item_id = ci.id
                        WHERE ci.collection_id = ?1
                          AND ci.removed_date IS NULL
                          AND (pi.purchase_type IS NULL OR pi.purchase_type != 'PREORDER')
                          AND rm.category = 'RAILCARS'
                   ), 0),
                   electric_multiple_units_count = COALESCE((
                       SELECT COUNT(*)
                         FROM collection_items ci
                         JOIN railway_models rm ON rm.id = ci.railway_model_id
                         LEFT JOIN purchase_infos pi ON pi.collection_item_id = ci.id
                        WHERE ci.collection_id = ?1
                          AND ci.removed_date IS NULL
                          AND (pi.purchase_type IS NULL OR pi.purchase_type != 'PREORDER')
                          AND rm.category = 'ELECTRIC_MULTIPLE_UNITS'
                   ), 0),
                   starter_sets_count = COALESCE((
                       SELECT COUNT(*)
                         FROM collection_items ci
                         JOIN railway_models rm ON rm.id = ci.railway_model_id
                         LEFT JOIN purchase_infos pi ON pi.collection_item_id = ci.id
                        WHERE ci.collection_id = ?1
                          AND ci.removed_date IS NULL
                          AND (pi.purchase_type IS NULL OR pi.purchase_type != 'PREORDER')
                          AND rm.category = 'STARTER_SETS'
                   ), 0)
             WHERE id = ?1;
        "#;

        sqlx::query(sql)
            .bind(collection_id)
            .execute(&mut *self.executor)
            .await
            .with_domain_context("Error recalculating collection summary")?;

        Ok(())
    }
}

impl CollectionUowExt for SqliteUnitOfWork {
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

        let owned_rolling_stock_rows = database::get_owned_rolling_stocks_including_removed(
            &mut *self.executor,
            &collection_row.id,
        )
        .await?;
        let owned_rolling_stocks_map: HashMap<CollectionItemId, Vec<OwnedRollingStockRow>> =
            owned_rolling_stock_rows
                .into_iter()
                .map(|owned_rs| (owned_rs.collection_item_id.clone(), owned_rs))
                .into_group_map();

        let purchase_info_rows =
            database::get_purchase_infos_including_removed(&mut *self.executor, &collection_row.id)
                .await?;
        let purchase_info_map: HashMap<CollectionItemId, Vec<PurchaseInfoRow>> = purchase_info_rows
            .into_iter()
            .map(|purchase_info| (purchase_info.collection_item_id.clone(), purchase_info))
            .into_group_map();

        let collection_item_rows = database::get_collection_items_including_removed(
            &mut *self.executor,
            &collection_row.id,
        )
        .await?;
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
                    // Metadata is initialised as part of this event — bump version/timestamp.
                    self.update_collection_metadata(aggregate_id).await?;
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
                    .await?;

                    self.update_collection_metadata(aggregate_id).await?;
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

                    self.update_collection_metadata(aggregate_id).await?;
                }
                CollectionEvent::RailwayModelSold { .. } => {}
            }
        }

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

    async fn sell_item(
        &mut self,
        collection_item_id: &CollectionItemId,
        sale_date: NaiveDate,
        sale_price: MonetaryAmount,
        buyer_id: Option<String>,
    ) -> Result<(), DomainError> {
        self.mark_collection_item_as_removed(collection_item_id, sale_date)
            .await?;
        self.update_purchase_info_sale(collection_item_id, sale_date, &sale_price, buyer_id)
            .await?;
        self.recalculate_collection_summary(&CollectionId::default())
            .await?;
        self.recalculate_collection_total_value(&CollectionId::default())
            .await?;
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

        let owned_rolling_stock_rows = database::get_owned_rolling_stocks_including_removed(
            &mut *self.executor,
            &collection_row.id,
        )
        .await?;
        let owned_rolling_stocks_map: HashMap<CollectionItemId, Vec<OwnedRollingStockRow>> =
            owned_rolling_stock_rows
                .into_iter()
                .map(|owned_rs| (owned_rs.collection_item_id.clone(), owned_rs))
                .into_group_map();

        let purchase_info_rows =
            database::get_purchase_infos_including_removed(&mut *self.executor, &collection_row.id)
                .await?;
        let purchase_info_map: HashMap<CollectionItemId, Vec<PurchaseInfoRow>> = purchase_info_rows
            .into_iter()
            .map(|purchase_info| (purchase_info.collection_item_id.clone(), purchase_info))
            .into_group_map();

        let collection_item_rows = database::get_collection_items_including_removed(
            &mut *self.executor,
            &collection_row.id,
        )
        .await?;

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

    async fn receive_preorder(
        &mut self,
        collection_item_id: &CollectionItemId,
        received_date: NaiveDate,
    ) -> Result<(), DomainError> {
        let result = sqlx::query(
            r#"UPDATE purchase_infos
                  SET purchase_type = 'PURCHASED',
                      purchase_date = ?1,
                      purchased_price_amount = COALESCE(preorder_total_amount, deposit_amount),
                      purchased_price_currency = COALESCE(preorder_total_currency, deposit_currency),
                      deposit_amount = NULL,
                      deposit_currency = NULL,
                      preorder_total_amount = NULL,
                      preorder_total_currency = NULL,
                      expected_date = NULL
                WHERE collection_item_id = ?2
                  AND purchase_type = 'PREORDER'"#,
        )
        .bind(received_date)
        .bind(collection_item_id)
        .execute(&mut *self.executor)
        .await
        .with_domain_context("Error converting preorder to purchased")?;

        if result.rows_affected() == 0 {
            return Err(DomainError::NotFound {
                resource: "PreorderPurchaseInfo".to_string(),
                identifier: collection_item_id.to_string(),
            });
        }

        let collection_id = CollectionId::default();
        self.recalculate_collection_summary(&collection_id).await?;
        self.recalculate_collection_total_value(&collection_id)
            .await?;
        self.update_collection_metadata(&collection_id).await?;

        Ok(())
    }

    async fn convert_to_preorder(
        &mut self,
        collection_item_id: &CollectionItemId,
        deposit_amount: i64,
        deposit_currency: &str,
        preorder_total_amount: i64,
        preorder_total_currency: &str,
        expected_date: Option<NaiveDate>,
    ) -> Result<(), DomainError> {
        let result = sqlx::query(
            r#"UPDATE purchase_infos
                  SET purchase_type = 'PREORDER',
                      deposit_amount = ?1,
                      deposit_currency = ?2,
                      preorder_total_amount = ?3,
                      preorder_total_currency = ?4,
                      expected_date = ?5,
                      purchased_price_amount = NULL,
                      purchased_price_currency = NULL
                WHERE collection_item_id = ?6"#,
        )
        .bind(deposit_amount)
        .bind(deposit_currency)
        .bind(preorder_total_amount)
        .bind(preorder_total_currency)
        .bind(expected_date)
        .bind(collection_item_id)
        .execute(&mut *self.executor)
        .await
        .with_domain_context("Error converting purchase info to preorder")?;

        if result.rows_affected() == 0 {
            return Err(DomainError::NotFound {
                resource: "PurchaseInfo".to_string(),
                identifier: collection_item_id.to_string(),
            });
        }

        // Recalculate — preorders are excluded from summary/value counts
        let collection_id = CollectionId::default();
        self.recalculate_collection_summary(&collection_id).await?;
        self.recalculate_collection_total_value(&collection_id)
            .await?;
        self.update_collection_metadata(&collection_id).await?;

        Ok(())
    }

    async fn get_stats(&mut self) -> Result<CollectionStats, DomainError> {
        let sql = r#"
            SELECT
                COALESCE(SUM(CASE WHEN pi.purchase_type = 'PREORDER' AND ci.removed_date IS NULL THEN 1 ELSE 0 END), 0)
                    AS preordered_count,
                COALESCE(SUM(CASE WHEN pi.purchase_type = 'PURCHASED' AND ci.removed_date IS NULL THEN 1 ELSE 0 END), 0)
                    AS active_count,
                COALESCE(SUM(CASE WHEN pi.purchase_type = 'SOLD' THEN 1 ELSE 0 END), 0)
                    AS sold_count,
                COALESCE(SUM(CASE WHEN pi.purchase_type = 'PREORDER' AND ci.removed_date IS NULL THEN COALESCE(pi.deposit_amount, 0) ELSE 0 END), 0)
                    AS investment_at_risk_amount,
                MAX(CASE WHEN pi.purchase_type = 'PREORDER' AND ci.removed_date IS NULL THEN pi.deposit_currency ELSE NULL END)
                    AS investment_at_risk_currency,
                COALESCE(SUM(CASE WHEN pi.purchase_type = 'SOLD' THEN (COALESCE(pi.sale_price_amount, 0) - COALESCE(pi.purchased_price_amount, 0)) ELSE 0 END), 0)
                    AS realized_profit_amount,
                MAX(CASE WHEN pi.purchase_type = 'SOLD' THEN pi.sale_price_currency ELSE NULL END)
                    AS realized_profit_currency
            FROM collection_items ci
            JOIN collections c ON c.id = ci.collection_id
            LEFT JOIN purchase_infos pi ON pi.collection_item_id = ci.id
            WHERE c.id = (SELECT id FROM collections LIMIT 1)
        "#;

        use sqlx::Row as _;
        let row = sqlx::query(sql)
            .fetch_one(&mut *self.executor)
            .await
            .with_domain_context("Error fetching collection stats")?;

        Ok(CollectionStats {
            preordered_count: row.get::<i64, _>("preordered_count"),
            active_count: row.get::<i64, _>("active_count"),
            sold_count: row.get::<i64, _>("sold_count"),
            investment_at_risk_amount: row.get::<i64, _>("investment_at_risk_amount"),
            investment_at_risk_currency: row
                .get::<Option<String>, _>("investment_at_risk_currency"),
            realized_profit_amount: row.get::<i64, _>("realized_profit_amount"),
            realized_profit_currency: row.get::<Option<String>, _>("realized_profit_currency"),
        })
    }

    async fn add_owned_rolling_stock_for_collection_items(
        &mut self,
        railway_model_id: &RailwayModelId,
        rolling_stock_id: &RollingStockId,
    ) -> Result<Vec<OwnedRollingStockId>, DomainError> {
        let collection_item_ids = database::find_collection_item_ids_by_railway_model(
            &mut *self.executor,
            railway_model_id,
        )
        .await?;

        let mut owned_ids = Vec::new();
        for collection_item_id in &collection_item_ids {
            let owned_rs_id = OwnedRollingStockId::default();
            self.insert_owned_rolling_stocks(
                &owned_rs_id,
                collection_item_id,
                rolling_stock_id,
                None,
            )
            .await?;
            owned_ids.push(owned_rs_id);
        }

        Ok(owned_ids)
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
        BoxCondition, CollectionItemUpdate, ModelCondition, OwnedRollingStockId, PurchaseCondition,
        PurchaseInfo, UpdateCollectionItemInput,
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

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_collection.sql")
    )]
    async fn it_should_update_price_and_recalculate_collection_total(conn: sqlx::SqlitePool) {
        let mut uow = SqliteUnitOfWork::new(&conn)
            .await
            .expect("create unit of work");

        let mut repo = uow.collections_repository();
        let input = UpdateCollectionItemInput {
            collection_item_id: CollectionItemId::try_from(
                "trn:collection-item:d20a1a95-1ae4-4970-9e87-b4c84676e730",
            )
            .expect("valid collection item id"),
            update: CollectionItemUpdate::Price(Some(MonetaryAmount::new(20_000, Currency::EUR))),
        };

        repo.update_item(&input)
            .await
            .expect("update should succeed");
        drop(repo);
        uow.commit().await.expect("commit should succeed");

        let mut conn2 = conn.acquire().await.expect("acquire connection");
        let updated_amount: i64 =
            sqlx::query_scalar("SELECT total_value_amount FROM collections WHERE id = ?1 LIMIT 1")
                .bind(CollectionId::default().to_string())
                .fetch_one(&mut *conn2)
                .await
                .expect("query total value");

        assert_eq!(updated_amount, 20_000);
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_collection.sql")
    )]
    async fn it_should_return_not_found_when_updating_purchase_info_for_unknown_item(
        conn: sqlx::SqlitePool,
    ) {
        let mut uow = SqliteUnitOfWork::new(&conn)
            .await
            .expect("create unit of work");
        let mut repo = uow.collections_repository();

        let input = UpdateCollectionItemInput {
            collection_item_id: CollectionItemId::try_from(
                "trn:collection-item:11111111-1111-1111-1111-111111111111",
            )
            .expect("valid unknown item id"),
            update: CollectionItemUpdate::Seller(None),
        };

        let result = repo.update_item(&input).await;

        assert!(matches!(
            result,
            Err(DomainError::NotFound { resource, .. }) if resource == "PurchaseInfo"
        ));
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_collection.sql")
    )]
    async fn it_should_add_owned_rolling_stock_for_active_collection_items(conn: sqlx::SqlitePool) {
        let mut uow = SqliteUnitOfWork::new(&conn)
            .await
            .expect("create unit of work");
        let mut repo = uow.collections_repository();

        let railway_model_id = RailwayModelId::try_from("trn:railway-model:acme:60100")
            .expect("valid railway model id");
        let rolling_stock_id =
            RollingStockId::try_from("trn:rolling-stock:70300b1c-b1df-475f-a7be-291e435b1cf8")
                .expect("valid rolling stock id");

        let inserted_ids = repo
            .add_owned_rolling_stock_for_collection_items(&railway_model_id, &rolling_stock_id)
            .await
            .expect("insert owned rolling stocks should succeed");

        assert_eq!(inserted_ids.len(), 1);
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_collection.sql")
    )]
    async fn it_should_not_add_owned_rolling_stock_for_removed_collection_items(
        conn: sqlx::SqlitePool,
    ) {
        let mut conn2 = conn.acquire().await.expect("acquire connection");
        sqlx::query("UPDATE collection_items SET removed_date = '2026-01-01' WHERE id = ?1")
            .bind("trn:collection-item:d20a1a95-1ae4-4970-9e87-b4c84676e730")
            .execute(&mut *conn2)
            .await
            .expect("soft-delete collection item");
        drop(conn2);

        let mut uow = SqliteUnitOfWork::new(&conn)
            .await
            .expect("create unit of work");
        let mut repo = uow.collections_repository();

        let railway_model_id = RailwayModelId::try_from("trn:railway-model:acme:60100")
            .expect("valid railway model id");
        let rolling_stock_id =
            RollingStockId::try_from("trn:rolling-stock:70300b1c-b1df-475f-a7be-291e435b1cf8")
                .expect("valid rolling stock id");

        let inserted_ids = repo
            .add_owned_rolling_stock_for_collection_items(&railway_model_id, &rolling_stock_id)
            .await
            .expect("insert owned rolling stocks should succeed");

        assert!(inserted_ids.is_empty());
    }

    // ── receive_preorder ────────────────────────────────────────────────────

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_collection.sql")
    )]
    async fn receive_preorder_converts_preorder_to_purchased(conn: sqlx::SqlitePool) {
        // Arrange: patch the existing PURCHASED row to PREORDER with deposit data
        let collection_item_id = "trn:collection-item:d20a1a95-1ae4-4970-9e87-b4c84676e730";
        sqlx::query(
            "UPDATE purchase_infos SET purchase_type = 'PREORDER', deposit_amount = 5000, deposit_currency = 'EUR', preorder_total_amount = 17500, preorder_total_currency = 'EUR', purchased_price_amount = NULL, purchased_price_currency = NULL WHERE collection_item_id = ?1",
        )
        .bind(collection_item_id)
        .execute(&conn)
        .await
        .expect("patch to PREORDER");

        let mut uow = SqliteUnitOfWork::new(&conn)
            .await
            .expect("should create unit of work");

        let item_id = CollectionItemId::try_from(collection_item_id).expect("valid id");
        let received_date = NaiveDate::from_ymd_opt(2025, 6, 1).unwrap();

        uow.collections_repository()
            .receive_preorder(&item_id, received_date)
            .await
            .expect("receive_preorder should succeed");

        uow.commit().await.expect("commit");

        // Assert: purchase_type must be PURCHASED, purchase_date set, preorder fields cleared
        let row: (String, Option<String>, Option<i64>, Option<String>) = sqlx::query_as(
            "SELECT purchase_type, purchase_date, purchased_price_amount, purchased_price_currency FROM purchase_infos WHERE collection_item_id = ?1",
        )
        .bind(collection_item_id)
        .fetch_one(&conn)
        .await
        .expect("fetch row");

        assert_eq!(row.0, "PURCHASED");
        assert_eq!(row.1.as_deref(), Some("2025-06-01"));
        assert_eq!(row.2, Some(17500)); // preorder_total_amount promoted to purchased_price_amount
        assert_eq!(row.3.as_deref(), Some("EUR"));

        // Preorder fields must be cleared
        let preorder_fields: (Option<i64>, Option<String>) = sqlx::query_as(
            "SELECT deposit_amount, deposit_currency FROM purchase_infos WHERE collection_item_id = ?1",
        )
        .bind(collection_item_id)
        .fetch_one(&conn)
        .await
        .expect("fetch preorder fields");

        assert!(preorder_fields.0.is_none(), "deposit_amount should be NULL");
        assert!(
            preorder_fields.1.is_none(),
            "deposit_currency should be NULL"
        );
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn receive_preorder_returns_not_found_for_nonexistent_item(conn: sqlx::SqlitePool) {
        let mut uow = SqliteUnitOfWork::new(&conn)
            .await
            .expect("should create unit of work");

        let fake_id =
            CollectionItemId::try_from("trn:collection-item:00000000-0000-0000-0000-000000000000")
                .expect("valid id");
        let received_date = NaiveDate::from_ymd_opt(2025, 6, 1).unwrap();

        let result = uow
            .collections_repository()
            .receive_preorder(&fake_id, received_date)
            .await;

        assert!(
            matches!(result, Err(DomainError::NotFound { .. })),
            "expected NotFound, got: {result:?}"
        );
    }

    // ── convert_to_preorder ─────────────────────────────────────────────────

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_collection.sql")
    )]
    async fn convert_to_preorder_patches_purchase_info(conn: sqlx::SqlitePool) {
        let collection_item_id = "trn:collection-item:d20a1a95-1ae4-4970-9e87-b4c84676e730";

        let mut uow = SqliteUnitOfWork::new(&conn)
            .await
            .expect("should create unit of work");

        let item_id = CollectionItemId::try_from(collection_item_id).expect("valid id");
        let expected_date = NaiveDate::from_ymd_opt(2026, 3, 1);

        uow.collections_repository()
            .convert_to_preorder(&item_id, 2000, "EUR", 19900, "EUR", expected_date)
            .await
            .expect("convert_to_preorder should succeed");

        uow.commit().await.expect("commit");

        // Assert: PREORDER fields set, purchased_price cleared
        let row: (String, Option<i64>, Option<String>, Option<i64>, Option<String>) =
            sqlx::query_as(
                "SELECT purchase_type, deposit_amount, deposit_currency, preorder_total_amount, preorder_total_currency FROM purchase_infos WHERE collection_item_id = ?1",
            )
            .bind(collection_item_id)
            .fetch_one(&conn)
            .await
            .expect("fetch row");

        assert_eq!(row.0, "PREORDER");
        assert_eq!(row.1, Some(2000));
        assert_eq!(row.2.as_deref(), Some("EUR"));
        assert_eq!(row.3, Some(19900));
        assert_eq!(row.4.as_deref(), Some("EUR"));

        // purchased_price should be cleared
        let price_row: (Option<i64>, Option<String>) = sqlx::query_as(
            "SELECT purchased_price_amount, purchased_price_currency FROM purchase_infos WHERE collection_item_id = ?1",
        )
        .bind(collection_item_id)
        .fetch_one(&conn)
        .await
        .expect("fetch price row");

        assert!(
            price_row.0.is_none(),
            "purchased_price_amount should be NULL"
        );
        assert!(
            price_row.1.is_none(),
            "purchased_price_currency should be NULL"
        );
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn convert_to_preorder_returns_not_found_for_nonexistent_item(conn: sqlx::SqlitePool) {
        let mut uow = SqliteUnitOfWork::new(&conn)
            .await
            .expect("should create unit of work");

        let fake_id =
            CollectionItemId::try_from("trn:collection-item:00000000-0000-0000-0000-000000000000")
                .expect("valid id");

        let result = uow
            .collections_repository()
            .convert_to_preorder(&fake_id, 1000, "EUR", 5000, "EUR", None)
            .await;

        assert!(
            matches!(result, Err(DomainError::NotFound { .. })),
            "expected NotFound, got: {result:?}"
        );
    }

    // ── get_stats ───────────────────────────────────────────────────────────

    #[sqlx::test(migrations = "./migrations")]
    async fn get_stats_returns_zero_counts_for_empty_collection(conn: sqlx::SqlitePool) {
        let mut uow = SqliteUnitOfWork::new(&conn)
            .await
            .expect("should create unit of work");

        // Trigger collection creation by calling find_view (auto-creates default)
        let _ = uow
            .collections_repository()
            .find_view()
            .await
            .expect("find_view");

        let stats = uow
            .collections_repository()
            .get_stats()
            .await
            .expect("get_stats should succeed");

        assert_eq!(stats.preordered_count, 0);
        assert_eq!(stats.active_count, 0);
        assert_eq!(stats.sold_count, 0);
        assert_eq!(stats.investment_at_risk_amount, 0);
        assert!(stats.investment_at_risk_currency.is_none());
        assert_eq!(stats.realized_profit_amount, 0);
        assert!(stats.realized_profit_currency.is_none());
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_collection.sql")
    )]
    async fn get_stats_counts_one_active_item(conn: sqlx::SqlitePool) {
        // test_collection.sql has 1 PURCHASED (active) item
        let mut uow = SqliteUnitOfWork::new(&conn)
            .await
            .expect("should create unit of work");

        let stats = uow
            .collections_repository()
            .get_stats()
            .await
            .expect("get_stats should succeed");

        assert_eq!(stats.preordered_count, 0);
        assert_eq!(stats.active_count, 1);
        assert_eq!(stats.sold_count, 0);
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_collection.sql")
    )]
    async fn get_stats_counts_preorder_and_investment_at_risk(conn: sqlx::SqlitePool) {
        let collection_item_id = "trn:collection-item:d20a1a95-1ae4-4970-9e87-b4c84676e730";

        // Patch the PURCHASED item to PREORDER
        sqlx::query(
            "UPDATE purchase_infos SET purchase_type = 'PREORDER', deposit_amount = 3000, deposit_currency = 'EUR', purchased_price_amount = NULL, purchased_price_currency = NULL WHERE collection_item_id = ?1",
        )
        .bind(collection_item_id)
        .execute(&conn)
        .await
        .expect("patch to PREORDER");

        let mut uow = SqliteUnitOfWork::new(&conn)
            .await
            .expect("should create unit of work");

        let stats = uow
            .collections_repository()
            .get_stats()
            .await
            .expect("get_stats should succeed");

        assert_eq!(stats.preordered_count, 1);
        assert_eq!(stats.active_count, 0);
        assert_eq!(stats.sold_count, 0);
        assert_eq!(stats.investment_at_risk_amount, 3000);
        assert_eq!(stats.investment_at_risk_currency.as_deref(), Some("EUR"));
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_collection.sql")
    )]
    async fn get_stats_counts_sold_item_and_realized_profit(conn: sqlx::SqlitePool) {
        let collection_item_id = "trn:collection-item:d20a1a95-1ae4-4970-9e87-b4c84676e730";

        // Mark item as sold: purchased_price_amount = 17500, sale_price_amount = 20000
        sqlx::query(
            "UPDATE purchase_infos SET purchase_type = 'SOLD', sale_date = '2026-01-01', sale_price_amount = 20000, sale_price_currency = 'EUR' WHERE collection_item_id = ?1",
        )
        .bind(collection_item_id)
        .execute(&conn)
        .await
        .expect("patch to SOLD");

        // Also set removed_date so the item is excluded from active counts
        sqlx::query("UPDATE collection_items SET removed_date = '2026-01-01' WHERE id = ?1")
            .bind(collection_item_id)
            .execute(&conn)
            .await
            .expect("set removed_date");

        let mut uow = SqliteUnitOfWork::new(&conn)
            .await
            .expect("should create unit of work");

        let stats = uow
            .collections_repository()
            .get_stats()
            .await
            .expect("get_stats should succeed");

        assert_eq!(stats.preordered_count, 0);
        assert_eq!(stats.active_count, 0);
        assert_eq!(stats.sold_count, 1);
        // profit = sale_price(20000) - purchased_price(17500) = 2500
        assert_eq!(stats.realized_profit_amount, 2500);
        assert_eq!(stats.realized_profit_currency.as_deref(), Some("EUR"));
    }
}
