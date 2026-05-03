use crate::catalog::domain::manufacturer::{ManufacturerId, ManufacturerStatus};
use crate::core::domain::metadata::Metadata;
use serde::{Deserialize, Serialize};
use url::Url;

/// A manufacturer (maker of railway models).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

    /// Metadata about the manufacturer (creation date, version, last updated).
    #[serde(skip)]
    pub metadata: Metadata,
}
