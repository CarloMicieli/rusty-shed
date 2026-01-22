use crate::collecting::application::RemoveCollectionItemInput;
use crate::collecting::domain::CollectionSummary;
use crate::collecting::domain::collection_events::OwnedRollingStockIds;
use crate::collecting::domain::{
    CollectionEvent, CollectionId, CollectionItemId, OwnedRollingStock, OwnedRollingStockId,
    PurchaseInfo, PurchasedInfo,
};
use crate::collecting::domain::{CollectionItem, NewCollectionItem};
use crate::core::domain::EventEnvelope;
use crate::core::domain::MonetaryAmount;
use crate::core::domain::metadata::Metadata;

type CollectionDomainEvent = EventEnvelope<CollectionEvent>;

/// Represents a user-owned collection of railway models.
///
/// A `Collection` contains identifying information, a few aggregated summary
/// values and the list of `CollectionItem` entries that make up the
/// collection. It is intentionally lightweight to keep IPC payloads small.
///
/// Default behavior:
/// - `Collection::default()` returns an empty collection with a generated id,
///   the name "My Collection", a `CollectionSummary::default()` and no
///   `total_value` (i.e. `None`). This mirrors previous code paths that
///   returned a default when no database row existed.
#[derive(Debug, Clone)]
pub struct Collection {
    /// Unique identifier for the collection (typically a UUID stored as a string).
    pub id: CollectionId,

    /// Display name for this collection.
    pub name: String,

    /// Precomputed summary counts (e.g. total items, tracked vs untracked).
    pub summary: CollectionSummary,

    /// Optional total monetary value of the collection. Use `MonetaryAmount`
    /// to preserve currency and decimal precision.
    pub total_value: Option<MonetaryAmount>,

    /// The list of items contained in this collection.
    pub items: Vec<CollectionItem>,

    /// Pending events that have occurred in this collection but not yet processed.
    pub pending_events: Vec<CollectionDomainEvent>,

    /// Metadata about the collection (creation date, last modified, etc.).
    pub metadata: Metadata,
}

impl Collection {
    /// Returns the total number of items in the collection.
    pub fn size(&self) -> u16 {
        self.items.len() as u16
    }

    /// Add a new item to the collection, updating summary and total value.
    /// Also generates a `CollectionEvent::RailwayModelAdded` event.
    pub fn add_item(&mut self, new_item: NewCollectionItem) -> CollectionItemId {
        // Build event rolling stock entries from the supplied RailwayModel's rolling stocks
        let owned_rolling_stock = new_item
            .railway_model
            .rolling_stocks
            .iter()
            .map(|rs| OwnedRollingStockIds {
                owned_rolling_stock_id: OwnedRollingStockId::default(),
                rolling_stock_id: rs.id_as_ref().clone(),
                installed_decoder_id: None,
            })
            .collect();

        let event = CollectionEvent::RailwayModelAdded {
            aggregate_id: self.id.clone(),
            collection_item_id: new_item.collection_item_id.clone(),
            category: new_item.railway_model.category,
            railway_model_id: new_item.railway_model.id.clone(),
            added_date: new_item.added_date,
            rolling_stock: owned_rolling_stock,
            price: new_item.price,
            seller_id: new_item.seller_id,
            purchase_info_id: new_item.purchase_info_id,
            purchase_date: new_item.purchase_date,
            purchase_condition: new_item.purchase_condition,
            model_condition: new_item.model_condition,
            box_condition: new_item.box_condition,
            notes: new_item.notes,
        };

        self.apply(&event);
        self.pending_events.push(CollectionDomainEvent::new(event));

        new_item.collection_item_id
    }

    /// Remove an item from the collection by marking it removed and emitting
    /// a `RailwayModelRemoved` event. The event contains the item id and the
    /// removed_date as provided by the caller.
    pub fn remove_item(&mut self, remove_collection_item: RemoveCollectionItemInput) {
        let event = CollectionEvent::RailwayModelRemoved {
            aggregate_id: self.id.clone(),
            collection_item_id: remove_collection_item.collection_item_id,
            removed_date: remove_collection_item.removed_date,
            category: remove_collection_item.category,
        };

        self.apply(&event);
        self.pending_events.push(CollectionDomainEvent::new(event));
    }

    /// Pulls and returns pending events, clearing the internal buffer.
    pub fn pull_events(&mut self) -> Vec<CollectionDomainEvent> {
        std::mem::take(&mut self.pending_events)
    }

    /// Apply a `CollectionEvent` to the current state of the `Collection`.
    /// This method mutates the collection based on the event type.
    ///
    /// # Arguments
    /// - `event`: The `CollectionEvent` to apply.
    pub fn apply(&mut self, event: &CollectionEvent) {
        match event {
            CollectionEvent::CollectionCreated { .. } => {
                // No state change needed for creation event
            }
            CollectionEvent::RailwayModelAdded {
                collection_item_id,
                railway_model_id,
                added_date,
                rolling_stock,
                price,
                seller_id,
                purchase_info_id,
                purchase_date,
                purchase_condition,
                model_condition,
                box_condition,
                notes,
                category,
                ..
            } => {
                // --- 1. Update Summary & Totals ---
                self.summary.update_count(*category, 1u16);

                let current_total = self.total_value.take().unwrap_or_default();
                // TODO: handle the unwrap safely or log the error
                self.total_value = Some(current_total.add_same_currency(price).unwrap());

                // --- 2. Reconstruct Internal Objects ---
                // We map from the event data back into our internal structs
                let internal_rolling_stocks = rolling_stock
                    .iter()
                    .map(|rs| OwnedRollingStock {
                        id: rs.owned_rolling_stock_id.clone(),
                        rolling_stock_id: rs.rolling_stock_id.clone(),
                        notes: None,
                        installed_decoder_id: rs.installed_decoder_id.clone(),
                    })
                    .collect();

                let purchase_info = PurchaseInfo::Purchased(PurchasedInfo {
                    id: purchase_info_id.clone(),
                    purchase_date: *purchase_date,
                    price: Some(price.clone()),
                    seller: seller_id.clone(),
                });

                let new_item = CollectionItem {
                    id: collection_item_id.clone(),
                    railway_model_id: railway_model_id.clone(),
                    added_date: *added_date,
                    removed_date: None,
                    purchase_condition: *purchase_condition,
                    model_condition: *model_condition,
                    box_condition: *box_condition,
                    notes: notes.clone(),
                    rolling_stocks: internal_rolling_stocks,
                    purchase_info: Some(purchase_info),
                };

                // --- 3. Final State Mutation ---
                self.items.push(new_item);
            }
            CollectionEvent::RailwayModelRemoved {
                collection_item_id,
                removed_date,
                category,
                ..
            } => {
                // Set removed_date on the item, decrement the summary and
                // subtract the purchase price from total_value when available.
                if let Some(item) = self.items.iter_mut().find(|i| &i.id == collection_item_id) {
                    item.removed_date = Some(*removed_date);

                    // Decrement summary counts for the category
                    self.summary.decrement_count(*category, 1u16);

                    // Subtract purchase price from total_value when present
                    if let Some(purchase_info) = &item.purchase_info
                        && let PurchaseInfo::Purchased(pi) = purchase_info
                        && let Some(price) = &pi.price
                        && let Some(current_total) = self.total_value.take()
                    {
                        // Create a negative amount to subtract
                        let neg = MonetaryAmount::new(-price.amount, price.currency);
                        if let Ok(new_total) = current_total.add_same_currency(&neg) {
                            self.total_value = Some(new_total);
                        } else {
                            // currency mismatch or overflow: restore old total
                            self.total_value = Some(current_total);
                        }
                    }
                }
            }
            _ => todo!("Handle other event types"),
        }
    }
}

impl Default for Collection {
    /// Returns a sensible default `Collection` matching existing code paths
    /// that expect a default when no collection is present in the database.
    fn default() -> Self {
        let create_collection_event = CollectionEvent::CollectionCreated {
            aggregate_id: CollectionId::default(),
            name: "My Collection".to_string(),
        };

        Collection {
            id: CollectionId::default(),
            name: "My Collection".to_string(),
            summary: CollectionSummary::default(),
            total_value: None,
            items: Vec::new(),
            pending_events: vec![EventEnvelope::new(create_collection_event)],
            metadata: Metadata::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::domain::manufacturer::ManufacturerId;
    use crate::catalog::domain::railway_model::{
        Category, PowerMethod, ProductCode, RailwayModel, RailwayModelId, RailwayModelManufacturer,
    };
    use crate::catalog::domain::scale::Scale;
    use crate::collecting::domain::{
        BoxCondition, ModelCondition, PurchaseCondition, PurchaseInfoId,
    };
    use crate::core::domain::Currency;
    use crate::sellers::domain::seller_id::SellerId;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_default_collection_has_expected_values() {
        let collection = Collection::default();

        assert_eq!(collection.name, "My Collection");
        assert!(collection.items.is_empty());
        assert!(collection.total_value.is_none());
        assert_eq!(collection.summary, CollectionSummary::default());
        assert_eq!(collection.items.len(), 0);
        assert_eq!(collection.pending_events.len(), 1);

        let event = collection
            .pending_events
            .first()
            .expect("No pending events");
        match &**event {
            CollectionEvent::CollectionCreated { aggregate_id, .. } => {
                assert_eq!(aggregate_id, &collection.id);
            }
            _ => panic!("Expected CollectionCreated event"),
        }
    }

    #[test]
    fn it_should_add_item_updates_collection_correctly() {
        let mut collection = Collection::default();
        assert_eq!(collection.items.len(), 0);

        let railway_model_id = RailwayModelId::try_from("trn:railway-model:acme:60100")
            .expect("valid railway model id");

        let railway_model = RailwayModel {
            id: railway_model_id.clone(),
            manufacturer: RailwayModelManufacturer {
                manufacturer_id: ManufacturerId::new("not-a-trn"),
                display: "Test Manufacturer".to_string(),
            },
            product_code: ProductCode::try_from("P100").unwrap(),
            description: "Test model".to_string(),
            details: None,
            power_method: PowerMethod::DC,
            scale: Scale::H0,
            epoch: "IV".into(),
            category: Category::Locomotives,
            delivery_date: None,
            availability_status: None,
            rolling_stocks: vec![],
            pending_events: Vec::new(),
        };

        let seller_id = SellerId::try_from("trn:seller:foo").unwrap();

        let new_item = NewCollectionItem {
            collection_item_id: CollectionItemId::default(),
            purchase_info_id: PurchaseInfoId::default(),
            railway_model: railway_model.clone(),
            price: MonetaryAmount::new(1000, Currency::USD),
            seller_id: Some(seller_id.clone()),
            added_date: chrono::NaiveDate::from_ymd_opt(2024, 6, 15).unwrap(),
            purchase_date: chrono::NaiveDate::from_ymd_opt(2024, 6, 10).unwrap(),
            purchase_condition: Some(PurchaseCondition::New),
            model_condition: Some(ModelCondition::Mint),
            box_condition: Some(BoxCondition::OriginalMint),
            notes: Some("Test addition".to_string()),
        };

        let _item_id = collection.add_item(new_item);

        assert_eq!(collection.items.len(), 1);
        assert_eq!(collection.pending_events.len(), 2);

        let summary = &collection.summary;
        assert_eq!(summary.locomotives_count, 1);
        assert_eq!(summary.passenger_cars_count, 0);
        assert_eq!(summary.freight_cars_count, 0);
        assert_eq!(summary.train_sets_count, 0);
        assert_eq!(summary.railcars_count, 0);
        assert_eq!(summary.electric_multiple_units_count, 0);
        assert_eq!(summary.starter_sets_count, 0);

        let total_value = collection.total_value.expect("Total value should be set");
        assert_eq!(total_value, MonetaryAmount::new(1000, Currency::USD));
    }

    #[test]
    fn it_should_remove_item_updates_collection_correctly() {
        let mut collection = Collection::default();

        let railway_model_id = RailwayModelId::try_from("trn:railway-model:acme:60100")
            .expect("valid railway model id");

        let railway_model = RailwayModel {
            id: railway_model_id.clone(),
            manufacturer: RailwayModelManufacturer {
                manufacturer_id: ManufacturerId::new("not-a-trn"),
                display: "Test Manufacturer".to_string(),
            },
            product_code: ProductCode::try_from("P100").unwrap(),
            description: "Test model".to_string(),
            details: None,
            power_method: PowerMethod::DC,
            scale: Scale::H0,
            epoch: "IV".into(),
            category: Category::Locomotives,
            delivery_date: None,
            availability_status: None,
            rolling_stocks: vec![],
            pending_events: Vec::new(),
        };

        let seller_id = SellerId::try_from("trn:seller:foo").unwrap();

        let new_item = NewCollectionItem {
            collection_item_id: CollectionItemId::default(),
            purchase_info_id: PurchaseInfoId::default(),
            railway_model: railway_model.clone(),
            price: MonetaryAmount::new(1000, Currency::USD),
            seller_id: Some(seller_id.clone()),
            added_date: chrono::NaiveDate::from_ymd_opt(2024, 6, 15).unwrap(),
            purchase_date: chrono::NaiveDate::from_ymd_opt(2024, 6, 10).unwrap(),
            purchase_condition: Some(PurchaseCondition::New),
            model_condition: Some(ModelCondition::Mint),
            box_condition: Some(BoxCondition::OriginalMint),
            notes: Some("Test addition".to_string()),
        };

        let _item_id = collection.add_item(new_item);

        assert_eq!(collection.items.len(), 1);
        let item_id = collection.items[0].id.clone();

        let removed_date = chrono::NaiveDate::from_ymd_opt(2025, 1, 2).unwrap();

        let remove_cmd = RemoveCollectionItemInput {
            collection_item_id: item_id.clone(),
            category: Category::Locomotives,
            removed_date,
        };

        collection.remove_item(remove_cmd);

        // The removed date was set on the item
        let item = collection
            .items
            .iter()
            .find(|i| i.id == item_id)
            .expect("item present");
        assert_eq!(item.removed_date, Some(removed_date));

        // Summary decremented
        assert_eq!(collection.summary.locomotives_count, 0);

        // Total value decreased by the price amount (back to zero)
        let total = collection.total_value.expect("total present");
        assert_eq!(total.amount, 0);
    }
}
