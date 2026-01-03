use specta::specta;
use crate::catalog::domain::{Category, DeliveryDate, Epoch, PowerMethod, ProductCode, RollingStock, Scale, ServiceLevel};
use crate::catalog::domain::availability_status::AvailabilityStatus;
use crate::catalog::domain::category::{ElectricMultipleUnitType, FreightCarType, LocomotiveType, PassengerCarType, RailcarType};
use crate::catalog::domain::control::Control;
use crate::catalog::domain::dcc_interface::DccInterface;
use crate::catalog::domain::length_over_buffers::LengthOverBuffers;
use crate::catalog::domain::manufacturer_id::ManufacturerId;
use crate::catalog::domain::railway_company_id::RailwayCompanyId;
use crate::catalog::domain::technical_specifications::TechnicalSpecifications;

/// Represents the data required to create a new Railway model within the system.
///
/// In Clean Architecture, this acts as the "NewData" input for the 
/// [`CatalogRepository`](crate::catalog::domain::repository::CatalogRepository).
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

#[derive(Debug, Clone)]
pub enum RollingStockParams {
    /// an electric multiple unit rolling stock
    ElectricMultipleUnitParams {
        railway_company_id: RailwayCompanyId,
        /// the livery description
        livery: Option<String>,
        /// the overall length
        length_over_buffer: Option<LengthOverBuffers>,
        /// the technical specifications
        technical_specifications: Option<TechnicalSpecifications>,
        /// the electric multiple unit friendly name
        friendly_name: String,
        /// the series code (eg. a short code identifying the series)
        series_code: Option<String>,
        /// the identification marking for this electric multiple unit
        road_number: Option<String>,
        /// the prototype series information
        series: Option<String>,
        /// the depot name
        depot: Option<String>,
        /// the electric multiple unit type
        electric_multiple_unit_type: ElectricMultipleUnitType,
        /// the dcc interface
        dcc_interface: Option<DccInterface>,
        /// the control
        control: Option<Control>,
        /// indicate whether the rolling stock has a motor or not
        is_dummy: bool,
    },
    /// a freight car rolling stock
    FreightCarParams {
        railway_company_id: RailwayCompanyId,
        /// the livery description
        livery: Option<String>,
        /// the overall length
        length_over_buffer: Option<LengthOverBuffers>,
        /// the technical specifications
        technical_specifications: Option<TechnicalSpecifications>,
        /// the freight car friendly name
        friendly_name: String,
        /// the series code
        series_code: Option<String>,
        /// the identification marking for this freight car
        road_number: Option<String>,
        /// the freight car type
        freight_car_type: Option<FreightCarType>,
    },
    /// a locomotive rolling stock
    LocomotiveParams {
        railway_company_id: RailwayCompanyId,
        /// the livery description
        livery: Option<String>,
        /// the overall length
        length_over_buffer: Option<LengthOverBuffers>,
        /// the technical specification
        technical_specifications: Option<TechnicalSpecifications>,
        /// the locomotive friendly name
        friendly_name: String,
        /// the series code
        series_code: Option<String>,
        /// the identification marking for this locomotive
        road_number: String,
        /// the prototype series information
        series: Option<String>,
        /// the depot name
        depot: Option<String>,
        /// the locomotive type
        locomotive_type: LocomotiveType,
        /// the dcc interface
        dcc_interface: Option<DccInterface>,
        /// the control
        control: Option<Control>,
        /// indicate whether the rolling stock has a motor or not
        is_dummy: bool,
    },
    /// a passenger car rolling stock
    PassengerCarParams {
        railway_company_id: RailwayCompanyId,
        /// the livery description
        livery: Option<String>,
        /// the overall length
        length_over_buffer: Option<LengthOverBuffers>,
        /// the technical specifications
        technical_specifications: Option<TechnicalSpecifications>,
        /// the passenger car friendly name
        friendly_name: String,
        /// the series code
        series_code: Option<String>,
        /// the identification marking for this passenger car
        road_number: Option<String>,
        /// the prototype series information
        series: Option<String>,
        /// the passenger car type
        passenger_car_type: Option<PassengerCarType>,
        /// the travel class for this passenger car. Passenger cars can have multiple service
        /// levels (ie, '1st/2nd')
        service_level: Option<ServiceLevel>,
    },
    /// a railcar rolling stock
    RailcarParams {
        /// the railway for this rolling stock
        railway_company_id: RailwayCompanyId,
        /// the livery description
        livery: Option<String>,
        /// the overall length
        length_over_buffer: Option<LengthOverBuffers>,
        /// the technical specifications
        technical_specifications: Option<TechnicalSpecifications>,
        /// the railcar friendly name
        friendly_name: String,
        /// the series code
        series_code: Option<String>,
        /// the identification marking for this railcar
        road_number: Option<String>,
        /// the railcar series
        series: Option<String>,
        /// the depot name
        depot: Option<String>,
        /// the railcar type
        railcar_type: RailcarType,
        /// the dcc interface
        dcc_interface: Option<DccInterface>,
        /// the control
        control: Option<Control>,
        /// indicate whether the rolling stock has a motor or not
        is_dummy: bool,
    },
}
