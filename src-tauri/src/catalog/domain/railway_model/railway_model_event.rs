use chrono::NaiveDateTime;
// serde derives removed: domain events are not serialized/deserialized
use uuid::Uuid;

use crate::catalog::domain::railway_model::{
    RailwayModelId, RailwayModelParams, RollingStockId, RollingStockParams,
};
use crate::core::domain::Language;

/// Domain events for `RailwayModel` aggregate.
#[derive(Debug, Clone)]
pub enum RailwayModelEvent {
    RailwayModelCreated {
        event_id: Uuid,
        railway_model_id: RailwayModelId,
        timestamp: NaiveDateTime,
        params: RailwayModelParams,
    },
    RailwayModelUpdated {
        event_id: Uuid,
        railway_model_id: RailwayModelId,
        timestamp: NaiveDateTime,
        /// Minimal patch describing changed fields (object).
        changed: serde_json::Value,
    },
    RollingStockAdded {
        event_id: Uuid,
        railway_model_id: RailwayModelId,
        timestamp: NaiveDateTime,
        rolling_stock_id: RollingStockId,
        rolling_stock_params: RollingStockParams,
    },
    RollingStockRemoved {
        event_id: Uuid,
        railway_model_id: RailwayModelId,
        timestamp: NaiveDateTime,
        rolling_stock_id: RollingStockId,
    },
    /// Emitted when a rolling stock's fields are updated.
    RollingStockUpdated {
        event_id: Uuid,
        railway_model_id: RailwayModelId,
        rolling_stock_id: RollingStockId,
        timestamp: NaiveDateTime,
        /// Minimal patch describing changed fields (same convention as RailwayModelUpdated.changed).
        changed: serde_json::Value,
    },
    /// Emitted when a translation is created or updated for a specific language.
    TranslationUpserted {
        event_id: Uuid,
        railway_model_id: RailwayModelId,
        timestamp: NaiveDateTime,
        /// The language code being upserted.
        lang: Language,
        /// New description for this language. `None` means leave unchanged; `Some("")` clears it.
        description: Option<String>,
        /// New details for this language. `None` means leave unchanged; `Some("")` clears it.
        details: Option<String>,
    },
}
