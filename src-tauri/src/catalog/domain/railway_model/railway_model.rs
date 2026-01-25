use crate::catalog::domain::manufacturer::ManufacturerId;
use crate::catalog::domain::railway_model::RailwayModelEvent;
use crate::catalog::domain::railway_model::{
    AvailabilityStatus, Category, DeliveryDate, Epoch, PowerMethod, ProductCode, RailwayModelId,
    RollingStock,
};
use crate::catalog::domain::railway_model::{RollingStockId, RollingStockParams};
use crate::catalog::domain::scale::Scale;
use serde_json::json;

/// A `RailwayModel` represents a manufactured model product in the catalog.
///
/// It contains metadata about the product (manufacturer, product code,
/// scale, epoch, etc.) and a list of `RollingStock` instances that correspond
/// to specific owned or catalogued items of this model.
#[derive(Debug, Clone)]
pub struct RailwayModel {
    /// Unique identifier for the railway model.
    pub id: RailwayModelId,

    /// Reference to the manufacturer id of the model (e.g. Bachmann, Märklin).
    pub manufacturer_id: ManufacturerId,

    /// Manufacturer-assigned product code.
    pub product_code: ProductCode,

    /// Human-readable description of the model.
    pub description: String,

    /// Additional details about the model (e.g. special features, variations).
    pub details: Option<String>,

    /// The power method used by this model (e.g. Diesel, Electric, None for non-powered models).
    pub power_method: PowerMethod,

    /// The scale of the model (e.g. HO, N).
    pub scale: Scale,

    /// The historical epoch the model belongs to.
    pub epoch: Epoch,

    /// Classification category for the model (e.g. locomotive, freight car).
    pub category: Category,

    /// Delivery or release date information for the product.
    pub delivery_date: Option<DeliveryDate>,

    /// the availability status
    pub availability_status: Option<AvailabilityStatus>,

    /// Rolling stock instances (specific vehicles) that correspond to this model.
    pub rolling_stocks: Vec<RollingStock>,

    /// Pending domain events produced by operations on this aggregate.
    pub pending_events: Vec<RailwayModelEvent>,
}

impl RailwayModel {
    /// Returns and clears pending events for this aggregate.
    pub fn pull_events(&mut self) -> Vec<RailwayModelEvent> {
        std::mem::take(&mut self.pending_events)
    }

    /// Push a domain event onto the pending events list.
    pub fn push_event(&mut self, ev: RailwayModelEvent) {
        self.pending_events.push(ev);
    }

    /// Update the description and emit a RailwayModelUpdated event.
    pub fn update_description(&mut self, description: String) {
        self.description = description.clone();
        let changed = json!({ "description": description });
        let ev = RailwayModelEvent::RailwayModelUpdated {
            event_id: uuid::Uuid::new_v4(),
            railway_model_id: self.id.clone(),
            timestamp: chrono::Utc::now().naive_utc(),
            changed,
        };
        self.push_event(ev);
    }

    /// Update details and emit a RailwayModelUpdated event.
    pub fn update_details(&mut self, details: Option<String>) {
        self.details = details.clone();
        let changed = json!({ "details": details });
        let ev = RailwayModelEvent::RailwayModelUpdated {
            event_id: uuid::Uuid::new_v4(),
            railway_model_id: self.id.clone(),
            timestamp: chrono::Utc::now().naive_utc(),
            changed,
        };
        self.push_event(ev);
    }

    /// Update availability status and emit a RailwayModelUpdated event.
    pub fn set_availability_status(&mut self, status: Option<AvailabilityStatus>) {
        self.availability_status = status;
        let availability = status.as_ref().map(|s| s.to_string());
        let changed = json!({ "availability_status": availability });
        let ev = RailwayModelEvent::RailwayModelUpdated {
            event_id: uuid::Uuid::new_v4(),
            railway_model_id: self.id.clone(),
            timestamp: chrono::Utc::now().naive_utc(),
            changed,
        };
        self.push_event(ev);
    }

    /// Emit RollingStockAdded event for a new rolling stock. Returns the generated id.
    pub fn add_rolling_stock(&mut self, params: RollingStockParams) -> RollingStockId {
        let rs_id = RollingStockId::new();
        let ev = RailwayModelEvent::RollingStockAdded {
            event_id: uuid::Uuid::new_v4(),
            railway_model_id: self.id.clone(),
            timestamp: chrono::Utc::now().naive_utc(),
            rolling_stock_id: rs_id.clone(),
            rolling_stock_params: params,
        };
        self.push_event(ev);
        rs_id
    }

    /// Emit RollingStockRemoved event and remove from in-memory collection if present.
    pub fn remove_rolling_stock(&mut self, id: &RollingStockId) {
        self.rolling_stocks.retain(|rs| rs.id_as_ref() != id);
        let ev = RailwayModelEvent::RollingStockRemoved {
            event_id: uuid::Uuid::new_v4(),
            railway_model_id: self.id.clone(),
            timestamp: chrono::Utc::now().naive_utc(),
            rolling_stock_id: id.clone(),
        };
        self.push_event(ev);
    }
}
