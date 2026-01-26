/// Input for creating a new railway model.
#[derive(Debug, Clone)]
pub struct CreateRailwayModelInput {
    /// Manufacturer identifier as a string.
    pub manufacturer_id: String,
    /// Display name of the manufacturer.
    pub product_code: String,
    /// Description of the railway model.
    pub description: String,
    /// Additional details about the railway model.
    pub details: Option<String>,
    /// Power method used by the railway model.
    pub power_method: String,
    /// Scale of the railway model.
    pub scale: String,
    /// Epoch of the railway model.
    pub epoch: String,
    /// Category of the railway model.
    pub category: String,
    /// Optional delivery date of the railway model.
    pub delivery_date: Option<String>,
    /// Optional availability status of the railway model.
    pub availability_status: Option<String>,
    /// Rolling stocks associated with the railway model.
    pub rolling_stocks: Vec<CreateRollingStockInput>,
}

/// Input for creating a rolling stock.
///
/// This enum is a tagged union where each variant contains the fields
/// relevant to that rolling stock category. Consumers should supply the
/// variant matching the `category` of the rolling stock being created.
#[derive(Debug, Clone)]
pub enum CreateRollingStockInput {
    /// Locomotive-specific input fields.
    Locomotive {
        railway_company_id: String,
        friendly_name: String,
        series_code: String,
        road_number: String,
        series: Option<String>,
        depot: Option<String>,
        livery: Option<String>,
        locomotive_type: String,
        is_dummy: Option<bool>,
        control: Option<String>,
        dcc_interface: Option<String>,
        length_over_buffers: Option<LengthOverBuffersInput>,
        technical_specifications: Option<TechnicalSpecificationsInput>,
    },
    /// Passenger car-specific input fields.
    PassengerCar {
        railway_company_id: String,
        friendly_name: String,
        series_code: String,
        road_number: Option<String>,
        series: Option<String>,
        livery: Option<String>,
        passenger_car_type: String,
        service_level: Option<String>,
        length_over_buffers: Option<LengthOverBuffersInput>,
        technical_specifications: Option<TechnicalSpecificationsInput>,
    },
    /// Freight car-specific input fields.
    FreightCar {
        railway_company_id: String,
        friendly_name: String,
        series_code: String,
        road_number: Option<String>,
        series: Option<String>,
        livery: Option<String>,
        freight_car_type: Option<String>,
        length_over_buffers: Option<LengthOverBuffersInput>,
        technical_specifications: Option<TechnicalSpecificationsInput>,
    },
    /// Railcar-specific input fields.
    Railcar {
        railway_company_id: String,
        friendly_name: String,
        series_code: String,
        road_number: Option<String>,
        series: Option<String>,
        depot: Option<String>,
        livery: Option<String>,
        railcar_type: Option<String>,
        is_dummy: Option<bool>,
        control: Option<String>,
        dcc_interface: Option<String>,
        length_over_buffers: Option<LengthOverBuffersInput>,
        technical_specifications: Option<TechnicalSpecificationsInput>,
    },
    /// Electric multiple unit-specific input fields.
    ElectricMultipleUnit {
        railway_company_id: String,
        friendly_name: String,
        series_code: String,
        road_number: Option<String>,
        series: Option<String>,
        depot: Option<String>,
        livery: Option<String>,
        electric_multiple_unit_type: String,
        is_dummy: Option<bool>,
        control: Option<String>,
        dcc_interface: Option<String>,
        length_over_buffers: Option<LengthOverBuffersInput>,
        technical_specifications: Option<TechnicalSpecificationsInput>,
    },
}

/// Length measurements over buffers for a rolling stock item.
///
/// Both measurements are optional; callers may provide either or both
/// depending on the available data. Values are represented as floating
/// point numbers (millimeters and inches respectively).
#[derive(Debug, Clone)]
pub struct LengthOverBuffersInput {
    /// Length in millimeters.
    pub millimeters: Option<f64>,
    /// Length in inches.
    pub inches: Option<f64>,
}

/// Optional technical specifications for a rolling stock item.
///
/// Each field is optional and represents a small piece of technical
/// metadata such as minimum recommended curve radius, coupling details,
/// or whether a flywheel is fitted.
#[derive(Debug, Clone)]
pub struct TechnicalSpecificationsInput {
    /// Minimum recommended curve radius in millimetres (if known).
    pub minimum_radius: Option<f64>,
    /// Coupling details (socket type and optional behaviours).
    pub coupling: Option<CouplingInput>,
    /// Whether a flywheel is fitted (free-text or a small enum encoded as string).
    pub flywheel_fitted: Option<String>,
    /// Information about the body shell (material / construction hints).
    pub body_shell: Option<String>,
    /// Chassis description or notes.
    pub chassis: Option<String>,
    /// Interior lighting details.
    pub interior_lights: Option<String>,
    /// Exterior lighting details.
    pub lights: Option<String>,
    /// Whether buffers are sprung (if applicable).
    pub sprung_buffers: Option<String>,
}

#[derive(Debug, Clone)]
/// Coupling configuration details for a rolling stock item.
pub struct CouplingInput {
    /// The coupling socket/type (for example: "NEM", "Kadee", "Generic").
    pub socket: String,
    /// Whether close couplers are fitted or supported (free-text).
    pub close_couplers: Option<String>,
    /// Digital shunting capability details (if any).
    pub digital_shunting: Option<String>,
}
