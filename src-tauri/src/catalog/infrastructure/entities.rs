use crate::catalog::domain::railway_model::{
    AvailabilityStatus, BodyShellType, Category, ChassisType, Control, DccInterface,
    ElectricMultipleUnitType, FreightCarType, LocomotiveType, PassengerCarType, PowerMethod,
    RailcarType, RollingStockCategory, ServiceLevel,
};
use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use sqlx::types::Text;

/// Row mapping for the `manufacturers` table.
///
/// Represents a single row returned from queries against the `manufacturers`
/// table. Field names correspond to the table columns.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ManufacturerRow {
    /// Primary identifier for the manufacturer (e.g. UUID or database ID).
    pub id: String,

    /// Human-friendly name of the manufacturer.
    pub name: String,

    /// Optional registered (legal) company name, when different from `name`.
    pub registered_company_name: Option<String>,

    /// Status of the manufacturer (for example: "active", "inactive").
    pub status: String,

    /// Optional ISO 3166-1 alpha-2 country code for the manufacturer's country.
    pub country_code: Option<String>,

    /// Optional website URL (stored as TEXT in the DB).
    pub website_url: Option<String>,

    /// Timestamp when the row was created.
    pub created_at: NaiveDateTime,

    /// Timestamp when the row was last updated.
    pub updated_at: NaiveDateTime,
}

/// Row mapping for the `railway_companies` table.
///
/// Represents a single row returned from queries against the `railway_companies`
/// table. Field names correspond to the table columns.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct RailwayCompanyRow {
    /// Primary identifier for the railway company.
    pub id: String,

    /// Human-friendly name of the railway company.
    pub name: String,

    /// Optional registered (legal) company name, when different from `name`.
    pub registered_company_name: Option<String>,

    /// Optional ISO 3166-1 alpha-2 country code for the company's country.
    pub country_code: Option<String>,

    /// Optional status field (kept as a string in the DB).
    pub status: Option<String>,

    /// Date when the railway began operation (YYYY-MM-DD string).
    pub operating_since: Option<String>,

    /// Date when the railway ended operation (YYYY-MM-DD string).
    pub operating_until: Option<String>,

    /// Timestamp when the row was created.
    pub created_at: NaiveDateTime,

    /// Timestamp when the row was last updated.
    pub updated_at: NaiveDateTime,
}

/// Row mapping for the `railway_models` table.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct RailwayModelRow {
    pub id: String,
    pub manufacturer_id: String,
    pub product_code: String,
    pub description: String,
    pub details: Option<String>,
    pub power_method: PowerMethod,
    pub scale: String,
    pub epoch: String,
    pub category: Category,
    pub delivery_date: Option<String>,
    pub availability_status: Option<AvailabilityStatus>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Row mapping for the `rolling_stocks` table.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct RollingStockRow {
    pub id: String,
    pub railway_model_id: String,
    pub category: RollingStockCategory,
    pub railway_company_id: String,
    pub livery: Option<String>,
    pub length_inches: Option<Text<Decimal>>,
    pub length_millimeters: Option<Text<Decimal>>,
    pub technical_minimum_radius_mm: Option<Text<Decimal>>,
    pub technical_coupling_socket: Option<String>,
    pub technical_coupling_close_couplers: Option<String>,
    pub technical_coupling_digital_shunting: Option<String>,
    pub technical_flywheel_fitted: Option<String>,
    pub technical_body_shell: Option<BodyShellType>,
    pub technical_chassis: Option<ChassisType>,
    pub technical_interior_lights: Option<String>,
    pub technical_lights: Option<String>,
    pub technical_sprung_buffers: Option<String>,
    pub series_code: String,
    pub friendly_name: Option<String>,
    pub road_number: Option<String>,
    pub series: Option<String>,
    pub depot: Option<String>,
    pub electric_multiple_unit_type: Option<ElectricMultipleUnitType>,
    pub freight_car_type: Option<FreightCarType>,
    pub locomotive_type: Option<LocomotiveType>,
    pub passenger_car_type: Option<PassengerCarType>,
    pub railcar_type: Option<RailcarType>,
    pub service_level: Option<ServiceLevel>,
    pub dcc_interface: Option<DccInterface>,
    pub control: Option<Control>,
    pub is_dummy: i64,
}
