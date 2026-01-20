use chrono::NaiveDateTime;
// serde derives removed: domain events are not serialized/deserialized
use uuid::Uuid;

use crate::catalog::domain::railway_model::{
    RailwayModelId, RailwayModelParams, RollingStockId, RollingStockParams,
};

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
}
