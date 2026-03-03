use crate::catalog::domain::railway_model::{Control, RollingStockId};
use crate::collecting::domain::{DigitalSetup, OwnedRollingStockId};
use serde::Serialize;

/// A lightweight view of rolling stock that references catalog model data.
///
/// This struct intentionally contains only the minimal information needed by
/// the collecting domain to reference a catalog `RollingStock` and basic
/// provenance. Detailed model information lives in the catalog domain and
/// should not be duplicated here. Fields like railway and epoch are no longer
/// stored on the owned_rolling_stocks table and should be obtained from the
/// catalog when needed.
#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct OwnedRollingStockView {
    /// Unique identifier for this owned rolling stock record (e.g. UUID in the DB).
    pub id: OwnedRollingStockId,

    /// Identifier of the related rolling stock in the catalog (or the owned rolling stock id when catalog id is not available).
    pub rolling_stock_id: RollingStockId,

    /// Free-form notes associated with this owned instance.
    /// Use this for short owner notes or a brief textual label.
    pub notes: Option<String>,

    /// Series derived from the catalog rolling stock data.
    pub series: Option<String>,

    /// Road number derived from the catalog rolling stock data.
    pub road_number: Option<String>,

    /// Livery derived from the catalog rolling stock data.
    pub livery: Option<String>,

    /// Control system derived from the catalog rolling stock data.
    pub control: Option<Control>,

    /// Railway company name derived from the catalog rolling stock data.
    pub railway_company_name: Option<String>,

    /// Optional digital setup information if a decoder is installed.
    pub digital: Option<DigitalSetup>,

    /// Depot name derived from the catalog rolling stock data.
    pub depot: Option<String>,
}
