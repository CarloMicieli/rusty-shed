use crate::catalog::domain::manufacturer::{ManufacturerId, ManufacturerStatus};
use serde::{Deserialize, Serialize};
use url::Url;

/// A manufacturer (maker of railway models).
///
/// Fields reflect the `manufacturers` table in the database. Optional fields
/// correspond to nullable DB columns.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Manufacturer {
    /// Unique identifier for the manufacturer.
    pub id: ManufacturerId,

    /// The common name of the manufacturer (not null).
    pub name: String,

    /// The legally registered company name (nullable).
    pub registered_company_name: Option<String>,

    /// The ISO 3166-1 alpha-2 country code where the company is registered
    /// (nullable). Example: `"IT"` for Italy.
    pub country_code: Option<String>,

    /// The lifecycle status of the manufacturer. Defaults to `Active`.
    pub status: ManufacturerStatus,

    /// Optional website URL for the manufacturer.
    pub website_url: Option<Url>,
}
