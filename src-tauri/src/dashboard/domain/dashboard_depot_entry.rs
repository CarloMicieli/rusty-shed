use crate::catalog::domain::manufacturer::ManufacturerId;
use crate::catalog::domain::railway_company::RailwayCompanyId;
use crate::catalog::domain::railway_model::{Category, Epoch, PowerMethod, RailwayModelId};
use crate::catalog::domain::scale::Scale;
use serde::Serialize;

/// Represents a railway model entry in the user's dashboard depot view.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct DashboardDepotEntry {
    /// Unique identifier for the railway model.
    pub id: RailwayModelId,
    /// Railway model manufacturer.
    pub manufacturer: DashboardDepotManufacturerEntry,
    /// Railway model product code.
    pub product_code: String,
    /// Railway model category.
    pub category: Category,
    /// Railway model scale.
    pub scale: Scale,
    /// Railway model epoch.
    pub epoch: Epoch,
    /// Railway model railway company.
    pub railway_company: DashboardDepotRailwayCompanyEntry,
    /// Railway model description.
    pub description: String,
    /// Railway model power method.
    pub power_method: PowerMethod,
}

/// Represents a manufacturer entry in the user's dashboard depot view.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct DashboardDepotManufacturerEntry {
    /// Unique identifier for the manufacturer.
    pub manufacturer_id: ManufacturerId,
    /// Manufacturer name.
    pub name: String,
}

/// Represents a railway company entry in the user's dashboard depot view.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct DashboardDepotRailwayCompanyEntry {
    /// Unique identifier for the railway company.
    pub railway_company_id: RailwayCompanyId,
    /// Railway company name.
    pub name: String,
    /// ISO 3166-1 alpha-2 country code where the railway company is registered (nullable).
    pub country_code: Option<String>,
}
