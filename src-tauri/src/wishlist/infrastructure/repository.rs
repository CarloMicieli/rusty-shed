use crate::core::domain::currency::Currency;
use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::WithDomainContext;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::wishlist::domain::repository::{WishlistRepository, WishlistUowExt};
use crate::wishlist::domain::wishlist::Wishlist;
use crate::wishlist::domain::wishlist_event::WishlistEvent;
use crate::wishlist::domain::wishlist_id::WishlistId;
use crate::wishlist::domain::wishlist_item::WishlistItem;
use crate::wishlist::domain::wishlist_item_id::WishlistItemId;
use crate::wishlist::domain::wishlist_preview::WishlistPreview;
use crate::wishlist::domain::wishlist_priority::WishlistPriority;
use crate::wishlist::domain::wishlist_status::WishlistStatus;
use crate::wishlist::infrastructure::database;
use crate::wishlist::infrastructure::entities::{
    WishlistItemRow, WishlistPreviewAggregateRow, WishlistRow,
};
use anyhow::Error as AnyhowError;
use std::collections::HashMap;
use std::convert::TryFrom;

pub struct SqliteWishlistRepository<'conn> {
    /// A mutable reference to the database connection/executor.
    executor: &'conn mut sqlx::SqliteConnection,
}

impl<'conn> SqliteWishlistRepository<'conn> {
    /// Creates a new repository instance using the provided executor.
    pub fn new(executor: &'conn mut sqlx::SqliteConnection) -> Self {
        Self { executor }
    }

    fn priority_to_db(priority: &WishlistPriority) -> &'static str {
        match priority {
            WishlistPriority::Low => "LOW",
            WishlistPriority::Normal => "NORMAL",
            WishlistPriority::High => "HIGH",
        }
    }

    fn status_to_db(status: &WishlistStatus) -> &'static str {
        match status {
            WishlistStatus::Wanted => "WANTED",
            WishlistStatus::OnOrder => "ON_ORDER",
            WishlistStatus::Purchased => "PURCHASED",
            WishlistStatus::Ignored => "IGNORED",
        }
    }

    fn map_price_update(
        desired_price: &Option<Option<crate::core::domain::MonetaryAmount>>,
    ) -> database::PriceUpdate {
        match desired_price {
            None => database::PriceUpdate {
                mode: 0,
                amount: None,
                currency: None,
            },
            Some(None) => database::PriceUpdate {
                mode: 1,
                amount: None,
                currency: None,
            },
            Some(Some(ma)) => database::PriceUpdate {
                mode: 2,
                amount: Some(ma.amount),
                currency: Some(ma.currency.to_code().to_string()),
            },
        }
    }

    fn parse_totals_by_currency(
        totals_by_currency: Option<&str>,
    ) -> Result<HashMap<Currency, i64>, DomainError> {
        let mut totals = HashMap::with_capacity(2);

        let Some(raw) = totals_by_currency else {
            return Ok(totals);
        };

        for pair in raw.split('|') {
            let mut parts = pair.splitn(2, ':');
            let currency_code = parts.next().unwrap_or_default();
            let amount_str = parts.next().unwrap_or_default();

            if currency_code.is_empty() || amount_str.is_empty() {
                continue;
            }

            let amount = amount_str
                .parse::<i64>()
                .map_err(|e| DomainError::Validation(e.to_string()))?;

            let currency = Currency::from_code(currency_code)
                .map_err(|e| DomainError::Validation(e.to_string()))?;

            totals.insert(currency, amount);
        }

        Ok(totals)
    }

    /// Handle a single `WishlistEvent` by performing the corresponding
    /// repository/database operation. This uses an exhaustive match to
    /// guarantee every event variant is handled.
    pub async fn handle_event(
        &mut self,
        wishlist_id: &WishlistId,
        event: &WishlistEvent,
    ) -> Result<(), DomainError> {
        match event {
            WishlistEvent::Created {
                name,
                notes,
                is_default,
            } => {
                let affected = database::update_wishlist_details(
                    &mut *self.executor,
                    wishlist_id,
                    name,
                    notes.as_deref(),
                )
                .await
                .with_domain_context("Error applying wishlist creation event")?;

                if affected == 0 {
                    return Err(DomainError::NotFound {
                        resource: "Wishlist".to_string(),
                        identifier: wishlist_id.to_string(),
                    });
                }

                if *is_default {
                    database::set_default_wishlist(&mut *self.executor, wishlist_id)
                        .await
                        .with_domain_context(
                            "Error setting default wishlist from creation event",
                        )?;
                } else {
                    database::unset_default_wishlist(&mut *self.executor, wishlist_id)
                        .await
                        .with_domain_context(
                            "Error unsetting default wishlist from creation event",
                        )?;
                }

                Ok(())
            }
            WishlistEvent::Renamed { name } => {
                let affected =
                    database::update_wishlist_name(&mut *self.executor, wishlist_id, name)
                        .await
                        .with_domain_context("Error renaming wishlist from event")?;
                if affected == 0 {
                    return Err(DomainError::NotFound {
                        resource: "Wishlist".to_string(),
                        identifier: wishlist_id.to_string(),
                    });
                }
                Ok(())
            }
            WishlistEvent::ItemAdded { item } => self.add_item(wishlist_id, item).await,
            WishlistEvent::ItemRemoved { item_id } => self.remove_item(item_id).await,
            WishlistEvent::ItemMoved {
                item_id,
                destination,
            } => self.move_item(item_id, destination).await,
            WishlistEvent::ItemPurchased {
                item_id,
                purchased_price,
            } => {
                let affected =
                    database::mark_item_purchased(&mut *self.executor, item_id, purchased_price)
                        .await
                        .with_domain_context("Error marking wishlist item as purchased")?;
                if affected == 0 {
                    return Err(DomainError::NotFound {
                        resource: "WishlistItem".to_string(),
                        identifier: item_id.to_string(),
                    });
                }
                Ok(())
            }
            WishlistEvent::MarkedDefault { is_default } => {
                if *is_default {
                    database::set_default_wishlist(&mut *self.executor, wishlist_id)
                        .await
                        .with_domain_context("Error setting default wishlist from event")?;
                } else {
                    database::unset_default_wishlist(&mut *self.executor, wishlist_id)
                        .await
                        .with_domain_context("Error unsetting default wishlist from event")?;
                }
                Ok(())
            }
            WishlistEvent::ItemUpdated {
                item_id,
                priority,
                status,
                desired_price,
                added_date,
            } => {
                let priority_str = priority
                    .as_ref()
                    .map(|p| Self::priority_to_db(p).to_string());
                let status_str = status.as_ref().map(|s| Self::status_to_db(s).to_string());
                let price = Self::map_price_update(desired_price);

                let affected = database::update_wishlist_item_fields(
                    &mut *self.executor,
                    item_id,
                    priority_str,
                    status_str,
                    price,
                    *added_date,
                )
                .await
                .with_domain_context("Error updating wishlist item fields")?;

                if affected == 0 {
                    return Err(DomainError::NotFound {
                        resource: "WishlistItem".to_string(),
                        identifier: item_id.to_string(),
                    });
                }
                Ok(())
            }
        }
    }
}

#[async_trait::async_trait]
impl<'conn> WishlistRepository for SqliteWishlistRepository<'conn> {
    /// Executes the SQLite-specific logic to fetch a wishlist by its ID.
    async fn find_by_id(&mut self, id: &WishlistId) -> Result<Option<Wishlist>, DomainError> {
        let wishlist_row = database::find_wishlist_by_id(&mut *self.executor, id)
            .await
            .with_domain_context("Error finding wishlist by id")?;

        if wishlist_row.is_none() {
            return Ok(None);
        }

        let wishlist_item_rows = database::find_wishlist_items_by_id(&mut *self.executor, id)
            .await
            .with_domain_context("Error finding wishlist items by wishlist id")?;

        let mut wishlist = Wishlist::try_from(wishlist_row.unwrap())
            .map_err(|e: AnyhowError| DomainError::Validation(e.to_string()))?;

        for item_row in wishlist_item_rows {
            let item = WishlistItem::try_from(item_row)
                .map_err(|e: AnyhowError| DomainError::Validation(e.to_string()))?;
            wishlist.add_item(item);
        }

        // Hydration via add_item generates ItemAdded events; discard them so
        // that save_wishlist only processes real domain events emitted after load.
        wishlist.drain_events();

        Ok(Some(wishlist))
    }

    async fn find_wishlists(&mut self) -> Result<Vec<WishlistPreview>, DomainError> {
        let rows: Vec<WishlistPreviewAggregateRow> =
            database::find_wishlist_previews(&mut *self.executor)
                .await
                .with_domain_context("Error fetching wishlist previews")?;

        let mut previews = Vec::with_capacity(rows.len());

        for row in rows {
            let wishlist_id = WishlistId::try_from(row.wishlist_id.as_str())
                .map_err(|e| DomainError::Validation(e.to_string()))?;

            previews.push(WishlistPreview {
                id: wishlist_id,
                name: row.name,
                notes: row.notes,
                is_default: row.is_default != 0,
                count: row.item_count,
                updated_at: row.updated_at,
                total_value: Self::parse_totals_by_currency(row.totals_by_currency.as_deref())?,
            });
        }

        Ok(previews)
    }

    async fn create_wishlist(&mut self, wishlist: &Wishlist) -> Result<(), DomainError> {
        // Use wishlist.metadata values for deterministic timestamps and version
        let row = WishlistRow {
            id: wishlist.id.to_string(),
            name: wishlist.name.clone(),
            notes: wishlist.notes.clone(),
            is_default: if wishlist.is_default { 1 } else { 0 },
            version: wishlist.metadata.version as i64,
            created_at: wishlist.metadata.created_at.naive_utc(),
            updated_at: wishlist.metadata.updated_at.naive_utc(),
        };

        database::insert_wishlist(&mut *self.executor, row)
            .await
            .with_domain_context("Error inserting wishlist")?;

        if wishlist.is_default {
            self.handle_event(
                &wishlist.id,
                &WishlistEvent::MarkedDefault { is_default: true },
            )
            .await?;
        }

        // Apply any pending domain events emitted by the aggregate.
        for ev in &wishlist.pending_events {
            self.handle_event(&wishlist.id, ev).await?;
        }
        Ok(())
    }

    async fn save_wishlist(&mut self, wishlist: &Wishlist) -> Result<(), DomainError> {
        let exists = database::wishlist_exists(&mut *self.executor, &wishlist.id)
            .await
            .with_domain_context("Error checking wishlist existence")?;

        if !exists {
            return Err(DomainError::NotFound {
                resource: "Wishlist".to_string(),
                identifier: wishlist.id.to_string(),
            });
        }

        // Process emitted events from aggregate
        for ev in &wishlist.pending_events {
            self.handle_event(&wishlist.id, ev).await?;
        }

        Ok(())
    }

    async fn rename_wishlist(&mut self, id: &WishlistId, name: &str) -> Result<(), DomainError> {
        let affected = database::update_wishlist_name(&mut *self.executor, id, name)
            .await
            .with_domain_context("Error renaming wishlist")?;
        if affected == 0 {
            return Err(DomainError::NotFound {
                resource: "Wishlist".to_string(),
                identifier: id.to_string(),
            });
        }
        Ok(())
    }

    async fn delete_wishlist(&mut self, id: &WishlistId) -> Result<(), DomainError> {
        let affected = database::delete_wishlist(&mut *self.executor, id)
            .await
            .with_domain_context("Error deleting wishlist")?;
        if affected == 0 {
            return Err(DomainError::NotFound {
                resource: "Wishlist".to_string(),
                identifier: id.to_string(),
            });
        }
        Ok(())
    }

    async fn set_default_wishlist(&mut self, id: &WishlistId) -> Result<(), DomainError> {
        database::set_default_wishlist(&mut *self.executor, id)
            .await
            .with_domain_context("Error setting default wishlist")?;
        Ok(())
    }

    async fn add_item(
        &mut self,
        wishlist_id: &WishlistId,
        item: &WishlistItem,
    ) -> Result<(), DomainError> {
        let (desired_amount, desired_currency) = item
            .desired_price
            .as_ref()
            .map(|p| (Some(p.amount), Some(p.currency.to_code().to_string())))
            .unwrap_or((None, None));

        let (purchased_amount, purchased_currency) = item
            .purchased_price
            .as_ref()
            .map(|p| (Some(p.amount), Some(p.currency.to_code().to_string())))
            .unwrap_or((None, None));

        let priority_str = Self::priority_to_db(&item.priority).to_string();
        let status_str = Self::status_to_db(&item.status).to_string();

        let row = WishlistItemRow {
            id: item.id.to_string(),
            wishlist_id: wishlist_id.to_string(),
            railway_model_id: item.railway_model_id.to_string(),
            priority: priority_str,
            status: status_str,
            desired_price_amount: desired_amount,
            desired_price_currency: desired_currency,
            added_date: item.added_date,
            removed_date: item.removed_date,
            notes: item.notes.clone(),
            purchased_at: None,
            purchased_price_amount: purchased_amount,
            purchased_price_currency: purchased_currency,
        };

        database::insert_wishlist_item(&mut *self.executor, row)
            .await
            .with_domain_context("Error inserting wishlist item")?;
        database::touch_wishlist_updated_at(&mut *self.executor, wishlist_id)
            .await
            .with_domain_context("Error updating wishlist timestamp after item insert")?;
        Ok(())
    }

    async fn remove_item(&mut self, item_id: &WishlistItemId) -> Result<(), DomainError> {
        // Touch the parent wishlist timestamp before deleting (after deletion the
        // item row is gone and the parent id can no longer be looked up).
        database::touch_wishlist_updated_at_for_item(&mut *self.executor, item_id)
            .await
            .with_domain_context("Error updating wishlist timestamp before item delete")?;
        let affected = database::delete_wishlist_item(&mut *self.executor, item_id)
            .await
            .with_domain_context("Error deleting wishlist item")?;
        if affected == 0 {
            return Err(DomainError::NotFound {
                resource: "WishlistItem".to_string(),
                identifier: item_id.to_string(),
            });
        }
        Ok(())
    }

    async fn move_item(
        &mut self,
        item_id: &WishlistItemId,
        destination_wishlist: &WishlistId,
    ) -> Result<(), DomainError> {
        let source_wishlist_id = database::move_wishlist_item_with_source(
            &mut *self.executor,
            item_id,
            destination_wishlist,
        )
        .await
        .with_domain_context("Error moving wishlist item")?;

        let Some(source_wishlist_id) = source_wishlist_id else {
            return Err(DomainError::NotFound {
                resource: "WishlistItem".to_string(),
                identifier: item_id.to_string(),
            });
        };

        let source_wishlist = WishlistId::try_from(source_wishlist_id.as_str())
            .map_err(|e| DomainError::Validation(e.to_string()))?;

        database::touch_wishlist_updated_at(&mut *self.executor, &source_wishlist)
            .await
            .with_domain_context("Error updating source wishlist timestamp after item move")?;

        if source_wishlist != *destination_wishlist {
            database::touch_wishlist_updated_at(&mut *self.executor, destination_wishlist)
                .await
                .with_domain_context(
                    "Error updating destination wishlist timestamp after item move",
                )?;
        }

        Ok(())
    }
}

impl<'conn> WishlistUowExt for SqliteUnitOfWork<'conn> {
    /// Links the SQLite-specific repository to the Unit of Work.
    ///
    /// It re-borrows the internal transaction (`&mut *self.tx`) to provide
    /// the repository with a mutable executor without transferring ownership.
    fn wishlist_repository(&mut self) -> Box<dyn WishlistRepository + '_> {
        Box::new(SqliteWishlistRepository::new(&mut self.tx))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::Currency;
    use crate::wishlist::domain::wishlist_id::WishlistId;
    use crate::wishlist::domain::wishlist_priority::WishlistPriority;
    use crate::wishlist::domain::wishlist_status::WishlistStatus;
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use sqlx::SqlitePool;

    #[sqlx::test(migrations = "./migrations")]
    async fn get_wishlist_repo_returns_none(conn: SqlitePool) -> Result<()> {
        let mut unit_of_work = SqliteUnitOfWork::new(&conn).await?;
        let mut repo = unit_of_work.wishlist_repository();

        let id = WishlistId::default();
        let result = repo.find_by_id(&id).await?;
        assert!(result.is_none());

        Ok(())
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_wishlist.sql")
    )]
    async fn get_wishlist_repo_returns_some(conn: SqlitePool) -> Result<()> {
        let mut unit_of_work = SqliteUnitOfWork::new(&conn).await?;
        let mut repo = unit_of_work.wishlist_repository();

        let id = WishlistId::try_from("trn:wishlist:58fb6f1d-d838-44b5-b65c-21e5388ca4c9")?;
        let result = repo.find_by_id(&id).await?;

        assert!(result.is_some());
        let wishlist = result.unwrap();
        assert_eq!(
            wishlist.id.to_string(),
            "trn:wishlist:58fb6f1d-d838-44b5-b65c-21e5388ca4c9"
        );
        assert_eq!(wishlist.items.len(), 1);

        let item = &wishlist.items[0];
        assert_eq!(
            item.id.to_string(),
            "trn:wishlist-item:2af7578c-8857-4894-8c93-0be4b579ff25"
        );
        assert_eq!(
            item.railway_model_id.to_string(),
            "trn:railway-model:acme:60100".to_string()
        );
        assert_eq!(
            item.desired_price.as_ref().map(|p| p.amount),
            Some(12345i64)
        );
        assert_eq!(
            item.desired_price.as_ref().map(|p| p.currency),
            Some(Currency::EUR)
        );
        assert_eq!(item.priority, WishlistPriority::Normal);
        assert_eq!(item.status, WishlistStatus::Wanted);
        assert_eq!(item.notes, Some("Fixture item notes".to_string()));
        assert_eq!(
            item.added_date,
            chrono::NaiveDate::from_ymd_opt(2025, 12, 26).unwrap()
        );

        Ok(())
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn list_wishlist_previews_returns_empty(conn: SqlitePool) -> Result<()> {
        let mut unit_of_work = SqliteUnitOfWork::new(&conn).await?;
        let mut repo = unit_of_work.wishlist_repository();

        let previews = repo.find_wishlists().await?;
        assert_eq!(previews.len(), 0);

        Ok(())
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_wishlists.sql")
    )]
    async fn list_wishlist_previews_returns_preview(conn: SqlitePool) -> Result<()> {
        let mut unit_of_work = SqliteUnitOfWork::new(&conn).await?;
        let mut repo = unit_of_work.wishlist_repository();

        let previews = repo.find_wishlists().await?;
        assert!(!previews.is_empty());

        // Find the preview for our fixture wishlist id/name
        let maybe = previews.iter().find(|p| {
            p.name == "Test Wishlist"
                || p.id.to_string() == "trn:wishlist:58fb6f1d-d838-44b5-b65c-21e5388ca4c9"
                || p.id.to_string() == "trn:wishlist:11111111-1111-1111-1111-111111111111"
        });
        assert!(
            maybe.is_some(),
            "expected at least one preview matching fixture"
        );
        let preview = maybe.unwrap();

        assert_eq!(preview.count, 2);
        let eur_total = preview
            .total_value
            .get(&crate::core::domain::currency::Currency::EUR)
            .cloned()
            .unwrap_or(0);
        assert_eq!(eur_total, 17500 + 15000);

        let first_wishlist = &previews[0];
        assert_eq!(first_wishlist.name, "Test Wishlist 1");
        assert_eq!(
            first_wishlist.id.to_string(),
            "trn:wishlist:58fb6f1d-d838-44b5-b65c-21e5388ca4c9"
        );
        assert_eq!(first_wishlist.count, 2);
        assert_eq!(first_wishlist.notes, Some("Notes".to_string()));
        assert_eq!(first_wishlist.is_default, false);
        assert_eq!(first_wishlist.total_value.get(&Currency::EUR), Some(&32500));

        let second_wishlist = &previews[1];
        assert_eq!(second_wishlist.name, "Test Wishlist 2");
        assert_eq!(
            second_wishlist.id.to_string(),
            "trn:wishlist:c9950910-96e1-47ae-8097-cd0ebbaa83f5"
        );
        assert_eq!(second_wishlist.count, 2);
        assert_eq!(second_wishlist.notes, Some("Notes".to_string()));
        assert_eq!(second_wishlist.is_default, true);
        assert_eq!(
            second_wishlist.total_value.get(&Currency::EUR),
            Some(&15000)
        );
        assert_eq!(
            second_wishlist.total_value.get(&Currency::USD),
            Some(&17500)
        );

        Ok(())
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_wishlist.sql")
    )]
    async fn item_updated_event_persists_changed_columns(conn: SqlitePool) -> Result<()> {
        use crate::wishlist::domain::wishlist_event::WishlistEvent;

        let wishlist_id =
            WishlistId::try_from("trn:wishlist:58fb6f1d-d838-44b5-b65c-21e5388ca4c9")?;
        let item_id =
            WishlistItemId::try_from("trn:wishlist-item:2af7578c-8857-4894-8c93-0be4b579ff25")?;

        let mut unit_of_work = SqliteUnitOfWork::new(&conn).await?;
        let mut repo = unit_of_work.wishlist_repository();

        // Load the wishlist; drain the ItemAdded events generated during load so
        // that save_wishlist only processes our ItemUpdated event.
        let mut wishlist = repo
            .find_by_id(&wishlist_id)
            .await?
            .expect("wishlist exists");
        let _ = wishlist.drain_events(); // clear load-time ItemAdded events

        let event = WishlistEvent::ItemUpdated {
            item_id: item_id.clone(),
            priority: Some(WishlistPriority::High),
            status: None,              // unchanged
            desired_price: Some(None), // clear
            added_date: Some(chrono::NaiveDate::from_ymd_opt(2024, 6, 1).unwrap()),
        };
        wishlist.pending_events.push(event);

        repo.save_wishlist(&wishlist).await?;

        // Re-read and verify changed columns
        let updated = repo
            .find_by_id(&wishlist_id)
            .await?
            .expect("wishlist still exists");
        let item = updated
            .items
            .iter()
            .find(|i| i.id == item_id)
            .expect("item still exists");

        assert_eq!(item.priority, WishlistPriority::High); // changed
        assert_eq!(item.status, WishlistStatus::Wanted); // unchanged
        assert!(item.desired_price.is_none()); // cleared
        assert_eq!(
            item.added_date,
            chrono::NaiveDate::from_ymd_opt(2024, 6, 1).unwrap()
        ); // changed

        Ok(())
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_wishlist.sql")
    )]
    async fn item_updated_event_leaves_unchanged_columns_intact(conn: SqlitePool) -> Result<()> {
        use crate::wishlist::domain::wishlist_event::WishlistEvent;

        let wishlist_id =
            WishlistId::try_from("trn:wishlist:58fb6f1d-d838-44b5-b65c-21e5388ca4c9")?;
        let item_id =
            WishlistItemId::try_from("trn:wishlist-item:2af7578c-8857-4894-8c93-0be4b579ff25")?;

        let mut unit_of_work = SqliteUnitOfWork::new(&conn).await?;
        let mut repo = unit_of_work.wishlist_repository();

        // Load and drain load-time events
        let mut wishlist = repo
            .find_by_id(&wishlist_id)
            .await?
            .expect("wishlist exists");
        let _ = wishlist.drain_events();

        // Only update status; all other fields should remain intact
        let event = WishlistEvent::ItemUpdated {
            item_id: item_id.clone(),
            priority: None,
            status: Some(WishlistStatus::OnOrder),
            desired_price: None, // unchanged — price stays 12345 EUR
            added_date: None,
        };
        wishlist.pending_events.push(event);

        repo.save_wishlist(&wishlist).await?;

        let updated = repo
            .find_by_id(&wishlist_id)
            .await?
            .expect("wishlist still exists");
        let item = updated
            .items
            .iter()
            .find(|i| i.id == item_id)
            .expect("item still exists");

        assert_eq!(item.priority, WishlistPriority::Normal); // unchanged
        assert_eq!(item.status, WishlistStatus::OnOrder); // changed
        assert_eq!(
            item.desired_price.as_ref().map(|p| p.amount),
            Some(12345i64)
        ); // unchanged
        assert_eq!(
            item.desired_price.as_ref().map(|p| p.currency),
            Some(Currency::EUR)
        ); // unchanged

        Ok(())
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_wishlists.sql")
    )]
    async fn save_wishlist_replays_root_events_only(conn: SqlitePool) -> Result<()> {
        use crate::wishlist::domain::wishlist_event::WishlistEvent;

        let first_id = WishlistId::try_from("trn:wishlist:58fb6f1d-d838-44b5-b65c-21e5388ca4c9")?;
        let second_id = WishlistId::try_from("trn:wishlist:c9950910-96e1-47ae-8097-cd0ebbaa83f5")?;

        let mut unit_of_work = SqliteUnitOfWork::new(&conn).await?;
        let mut repo = unit_of_work.wishlist_repository();

        let mut wishlist = repo.find_by_id(&first_id).await?.expect("wishlist exists");
        let _ = wishlist.drain_events();

        wishlist.pending_events.push(WishlistEvent::Renamed {
            name: "Renamed Through Event".to_string(),
        });
        wishlist
            .pending_events
            .push(WishlistEvent::MarkedDefault { is_default: true });

        repo.save_wishlist(&wishlist).await?;

        let updated_first = repo
            .find_by_id(&first_id)
            .await?
            .expect("first wishlist exists");
        let updated_second = repo
            .find_by_id(&second_id)
            .await?
            .expect("second wishlist exists");

        assert_eq!(updated_first.name, "Renamed Through Event");
        assert!(updated_first.is_default);
        assert!(!updated_second.is_default);

        Ok(())
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_wishlists.sql")
    )]
    async fn move_item_not_found_does_not_touch_wishlist_timestamps(
        conn: SqlitePool,
    ) -> Result<()> {
        let destination =
            WishlistId::try_from("trn:wishlist:c9950910-96e1-47ae-8097-cd0ebbaa83f5")?;
        let missing_item =
            WishlistItemId::try_from("trn:wishlist-item:aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa")?;

        let mut unit_of_work = SqliteUnitOfWork::new(&conn).await?;
        let mut repo = unit_of_work.wishlist_repository();

        let before = repo.find_wishlists().await?;
        let before_by_id: std::collections::HashMap<String, chrono::NaiveDateTime> = before
            .iter()
            .map(|preview| (preview.id.to_string(), preview.updated_at))
            .collect();

        let err = repo
            .move_item(&missing_item, &destination)
            .await
            .expect_err("missing item should fail");

        match err {
            DomainError::NotFound { resource, .. } => {
                assert_eq!(resource, "WishlistItem");
            }
            other => panic!("unexpected error: {other:?}"),
        }

        let after = repo.find_wishlists().await?;
        for preview in after {
            let before_ts = before_by_id
                .get(&preview.id.to_string())
                .expect("wishlist should exist before and after");
            assert_eq!(*before_ts, preview.updated_at);
        }

        Ok(())
    }
}
