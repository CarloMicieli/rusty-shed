use crate::catalog::domain::manufacturer::{ManufacturerId, ManufacturerStatus};
use crate::catalog::domain::railway_company::{RailwayCompanyId, RailwayStatus};
use crate::catalog::domain::railway_model::{
    AvailabilityStatus, BodyShellType, Category, ChassisType, Control, DccInterface, DeliveryDate,
    ElectricMultipleUnitType, Epoch, FreightCarType, LocomotiveType, PassengerCarType, PowerMethod,
    ProductCode, RailcarType, RailwayModelId, RollingStockCategory, RollingStockId, ServiceLevel,
};
use crate::catalog::domain::scale::Scale;
use chrono::{NaiveDate, NaiveDateTime};

/// Row mapping for the `manufacturers` table.
///
/// Represents a single row returned from queries against the `manufacturers`
/// table. Field names correspond to the table columns.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ManufacturerRow {
    /// Primary identifier for the manufacturer (e.g. UUID or database ID).
    pub id: ManufacturerId,

    /// Human-friendly name of the manufacturer.
    pub name: String,

    /// Optional registered (legal) company name, when different from `name`.
    pub registered_company_name: Option<String>,

    /// Status of the manufacturer (for example: "active", "inactive").
    pub status: ManufacturerStatus,

    /// Optional ISO 3166-1 alpha-2 country code for the manufacturer's country.
    pub country_code: Option<String>,

    /// Optional website URL (stored as TEXT in the DB).
    pub website_url: Option<String>,

    /// Timestamp when the row was created.
    pub created_at: NaiveDateTime,

    /// Timestamp when the row was last updated.
    pub updated_at: NaiveDateTime,

    /// Row version for optimistic concurrency control.
    pub version: i64,
}

/// Row mapping for the `railway_companies` table.
///
/// Represents a single row returned from queries against the `railway_companies`
/// table. Field names correspond to the table columns.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct RailwayCompanyRow {
    /// Primary identifier for the railway company.
    pub id: RailwayCompanyId,

    /// Human-friendly name of the railway company.
    pub name: String,

    /// Optional registered (legal) company name, when different from `name`.
    pub registered_company_name: Option<String>,

    /// Optional ISO 3166-1 alpha-2 country code for the company's country.
    pub country_code: Option<String>,

    /// Optional status field (kept as a string in the DB).
    pub status: Option<RailwayStatus>,

    /// Date when the railway began operation (YYYY-MM-DD string).
    pub operating_since: Option<NaiveDate>,

    /// Date when the railway ended operation (YYYY-MM-DD string).
    pub operating_until: Option<NaiveDate>,

    /// Timestamp when the row was created.
    pub created_at: NaiveDateTime,

    /// Timestamp when the row was last updated.
    pub updated_at: NaiveDateTime,
    /// Row version for optimistic concurrency control.
    pub version: i64,
}

/// Row mapping for railway model queries with language-resolved text.
///
/// Returned by COALESCE double-LEFT-JOIN queries that resolve description/details
/// to the requested language with EN fallback.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct RailwayModelRow {
    /// Primary identifier for the railway model.
    pub id: RailwayModelId,

    /// Foreign key to the manufacturer.
    pub manufacturer_id: ManufacturerId,

    /// Denormalized manufacturer display name.
    pub manufacturer_name: String,

    /// Product code assigned by manufacturer (SKU or catalogue code).
    pub product_code: ProductCode,

    /// The language code that was actually resolved (may be "en" fallback).
    pub resolved_lang: String,

    /// Short textual description of the model in the resolved language.
    /// `None` if both requested and EN translations are absent.
    pub description: Option<String>,

    /// Extended details or notes about the model in the resolved language.
    pub details: Option<String>,

    /// Power method of the model (e.g., electric, steam, diesel).
    pub power_method: PowerMethod,

    /// Model scale (e.g., H0, N).
    pub scale: Scale,

    /// Historical epoch the model represents.
    pub epoch: Epoch,

    /// Category of the model (e.g., locomotive, passenger, freight).
    pub category: Category,

    /// Expected or actual delivery date, when available.
    pub delivery_date: Option<DeliveryDate>,

    /// Availability status (preorder, discontinued, etc.), optional.
    pub availability_status: Option<AvailabilityStatus>,

    /// Timestamp when the row was created.
    pub created_at: NaiveDateTime,

    /// Timestamp when the row was last updated.
    pub updated_at: NaiveDateTime,
    /// Row version for optimistic concurrency control.
    pub version: i64,
}

/// Row mapping for the `rolling_stocks` table.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct RollingStockRow {
    /// Primary identifier for the rolling stock row.
    pub id: RollingStockId,

    /// Foreign key to the associated railway model.
    pub railway_model_id: RailwayModelId,

    /// Category of the rolling stock (e.g., locomotive, passenger, freight).
    pub category: RollingStockCategory,

    /// Foreign key to the owning railway company.
    pub railway_company_id: RailwayCompanyId,

    /// Name of the railway company (denormalized for query convenience).
    pub railway_company_name: String,

    /// Optional ISO 3166-1 alpha-2 country code of the railway company.
    pub railway_company_country_code: Option<String>,

    /// Optional prototype this rolling stock is linked to (FK to `prototypes.id`).
    pub prototype_id: Option<String>,

    /// Livery description, if available (paint scheme / decoration).
    pub livery: Option<String>,

    /// Length in inches as stored in the DB.
    pub length_inches: Option<f64>,

    /// Length in millimeters as stored in the DB.
    pub length_millimeters: Option<f64>,

    /// Technical minimum radius in millimeters.
    pub technical_minimum_radius_mm: Option<f64>,

    /// Coupling socket type or description.
    pub technical_coupling_socket: Option<String>,

    /// Whether close-couplers are used / details about them.
    pub technical_coupling_close_couplers: Option<String>,

    /// Digital shunting coupling support details.
    pub technical_coupling_digital_shunting: Option<String>,

    /// Whether a flywheel is fitted (stored as text flag/description).
    pub technical_flywheel_fitted: Option<String>,

    /// Body shell type, when available.
    pub technical_body_shell: Option<BodyShellType>,

    /// Chassis type, when available.
    pub technical_chassis: Option<ChassisType>,

    /// Interior lighting details.
    pub technical_interior_lights: Option<String>,

    /// Exterior lighting details.
    pub technical_lights: Option<String>,

    /// Whether sprung buffers are present.
    pub technical_sprung_buffers: Option<String>,

    /// Canonical series code for the rolling stock.
    pub series_code: String,

    /// Friendly display name for UI, optional.
    pub friendly_name: Option<String>,

    /// Road number or identifier, optional.
    pub road_number: Option<String>,

    /// Series string (e.g., batch or family), optional.
    pub series: Option<String>,

    /// Depot location code or name, optional.
    pub depot: Option<String>,

    /// Specific electric multiple unit type, when applicable.
    pub electric_multiple_unit_type: Option<ElectricMultipleUnitType>,

    /// Freight car type, when applicable.
    pub freight_car_type: Option<FreightCarType>,

    /// Locomotive type, when applicable.
    pub locomotive_type: Option<LocomotiveType>,

    /// Passenger car type, when applicable.
    pub passenger_car_type: Option<PassengerCarType>,

    /// Railcar type, when applicable.
    pub railcar_type: Option<RailcarType>,

    /// Service level (e.g., standard, premium), optional.
    pub service_level: Option<ServiceLevel>,

    /// DCC interface type, when applicable.
    pub dcc_interface: Option<DccInterface>,

    /// Control method (e.g., analogue, digital), optional.
    pub control: Option<Control>,

    /// Flag indicating whether this record is a dummy/placeholder.
    pub is_dummy: bool,
}
