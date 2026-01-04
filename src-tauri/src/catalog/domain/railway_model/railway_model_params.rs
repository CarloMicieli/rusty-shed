use crate::catalog::domain::manufacturer::ManufacturerId;
use crate::catalog::domain::railway_model::{
    AvailabilityStatus, Category, DeliveryDate, Epoch, PowerMethod, ProductCode, RollingStockParams,
};
use crate::catalog::domain::scale::Scale;

/// Represents the data required to create a new Railway model within the system.
///
/// In Clean Architecture, this acts as the "NewData" input for the
/// [`CatalogRepository`](crate::catalog::domain::repository::RailwayModelRepository).
///
/// ### Business Rules
///
/// ### Lifecycle
/// This struct is typically mapped from an [`create_railway_model`](crate::catalog::interface::command_handlers::create_railway_model)
/// after the raw input strings have been validated and converted into
/// Domain Value Objects.
#[derive(Debug, Clone)]
pub struct RailwayModelParams {
    /// The manufacturer of the model (e.g. Bachmann, Märklin).
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
    pub rolling_stocks: Vec<RollingStockParams>,
}
