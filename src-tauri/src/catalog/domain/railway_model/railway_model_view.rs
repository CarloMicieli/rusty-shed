use crate::catalog::domain::manufacturer::ManufacturerId;
use crate::catalog::domain::railway_company::RailwayCompanyId;
use crate::catalog::domain::railway_model::{
    AvailabilityStatus, Category, Control, DccInterface, DeliveryDate, ElectricMultipleUnitType,
    Epoch, FreightCarType, LengthOverBuffers, LocomotiveType, PassengerCarType, PowerMethod,
    ProductCode, RailcarType, RailwayModelId, RollingStockId, ServiceLevel,
    TechnicalSpecifications,
};
use crate::catalog::domain::scale::Scale;
use crate::core::domain::metadata::Metadata;
use serde::Serialize;

/// A UI-focused view of a railway model used by the frontend.
#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct RailwayModelView {
    /// Unique identifier for the railway model.
    pub id: RailwayModelId,

    /// Manufacturer metadata (id and display name).
    pub manufacturer: RailwayModelManufacturer,

    /// Manufacturer-assigned product code.
    pub product_code: ProductCode,

    /// Human-readable description of the model.
    pub description: String,

    /// Optional longer details or notes about the model.
    pub details: Option<String>,

    /// Power method used by the model (e.g. DC, AC).
    pub power_method: PowerMethod,

    /// Scale of the model (e.g. H0, N).
    pub scale: Scale,

    /// Historical epoch the model represents.
    pub epoch: Epoch,

    /// Classification category for the model (locomotive, freight car, etc.).
    pub category: Category,

    /// Optional delivery or release date information.
    pub delivery_date: Option<DeliveryDate>,

    /// Optional availability status for the model.
    pub availability_status: Option<AvailabilityStatus>,

    /// Metadata for the resource (versioning and timestamps).
    pub metadata: Metadata,

    /// Rolling stock instances (UI views) associated with this model.
    pub rolling_stock: Vec<RollingStockView>,
}

impl PartialEq for RailwayModelView {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for RailwayModelView {}

/// Lightweight manufacturer information used by the UI view layer.
#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct RailwayModelManufacturer {
    /// The unique identifier of the manufacturer.
    pub manufacturer_id: ManufacturerId,

    /// The manufacturer display name (e.g. Bachmann, Märklin).
    pub display: String,
}

/// Lightweight railway/company info for `RollingStockView` UI shapes.
#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct RollingStockRailway {
    /// The railway company's unique identifier.
    pub railway_company_id: RailwayCompanyId,

    /// The railway display name used in UIs.
    pub display: String,
}

/// A UI-focused view of a rolling stock item used by the frontend.
#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum RollingStockView {
    ElectricMultipleUnit {
        /// Identifier for this rolling stock instance.
        id: RollingStockId,

        /// Railway/company information for this rolling stock (id + display).
        railway: RollingStockRailway,

        /// Optional livery name or color scheme.
        livery: Option<String>,

        /// Optional length over buffers measurement.
        length_over_buffer: Option<LengthOverBuffers>,

        /// Optional technical specifications summary.
        technical_specifications: Option<TechnicalSpecifications>,

        /// Optional friendly/display name for the vehicle.
        friendly_name: Option<String>,

        /// Series or product code used for this rolling stock instance.
        series_code: String,

        /// Optional road number assigned to this vehicle.
        road_number: Option<String>,

        /// Optional series identifier.
        series: Option<String>,

        /// Optional depot/location assignment.
        depot: Option<String>,

        /// Specific EMU subtype.
        electric_multiple_unit_type: ElectricMultipleUnitType,

        /// Optional DCC interface configuration.
        dcc_interface: Option<DccInterface>,

        /// Optional control type information.
        control: Option<Control>,

        /// Marker indicating a placeholder/dummy instance.
        is_dummy: bool,
    },
    Locomotive {
        /// Identifier for this rolling stock instance.
        id: RollingStockId,

        /// Railway/company information for this rolling stock (id + display).
        railway: RollingStockRailway,

        /// Optional livery name or color scheme.
        livery: Option<String>,

        /// Optional length over buffers measurement.
        length_over_buffer: Option<LengthOverBuffers>,

        /// Optional technical specifications summary.
        technical_specifications: Option<TechnicalSpecifications>,

        /// Optional friendly/display name for the vehicle.
        friendly_name: Option<String>,

        /// Series or product code used for this rolling stock instance.
        series_code: String,

        /// Optional road number assigned to this vehicle.
        road_number: Option<String>,

        /// Optional series identifier.
        series: Option<String>,

        /// Optional depot/location assignment.
        depot: Option<String>,

        /// Specific locomotive subtype.
        locomotive_type: LocomotiveType,

        /// Optional DCC interface configuration.
        dcc_interface: Option<DccInterface>,

        /// Optional control type information.
        control: Option<Control>,

        /// Marker indicating a placeholder/dummy instance.
        is_dummy: bool,
    },
    FreightCar {
        /// Identifier for this rolling stock instance.
        id: RollingStockId,

        /// Railway/company information for this rolling stock (id + display).
        railway: RollingStockRailway,

        /// Optional livery name or color scheme.
        livery: Option<String>,

        /// Optional length over buffers measurement.
        length_over_buffer: Option<LengthOverBuffers>,

        /// Optional technical specifications summary.
        technical_specifications: Option<TechnicalSpecifications>,

        /// Optional friendly/display name for the vehicle.
        friendly_name: Option<String>,

        /// Series or product code used for this rolling stock instance.
        series_code: String,

        /// Optional road number assigned to this vehicle.
        road_number: Option<String>,

        /// Optional freight car type classification.
        freight_car_type: Option<FreightCarType>,
    },
    PassengerCar {
        /// Identifier for this rolling stock instance.
        id: RollingStockId,

        /// Railway/company information for this rolling stock (id + display).
        railway: RollingStockRailway,

        /// Optional livery name or color scheme.
        livery: Option<String>,

        /// Optional length over buffers measurement.
        length_over_buffer: Option<LengthOverBuffers>,

        /// Optional technical specifications summary.
        technical_specifications: Option<TechnicalSpecifications>,

        /// Optional friendly/display name for the vehicle.
        friendly_name: Option<String>,

        /// Series or product code used for this rolling stock instance.
        series_code: String,

        /// Optional road number assigned to this vehicle.
        road_number: Option<String>,

        /// Optional series identifier.
        series: Option<String>,

        /// Optional passenger car type classification.
        passenger_car_type: Option<PassengerCarType>,

        /// Optional service level (e.g., first class, standard).
        service_level: Option<ServiceLevel>,
    },
    Railcar {
        /// Identifier for this rolling stock instance.
        id: RollingStockId,

        /// Railway/company information for this rolling stock (id + display).
        railway: RollingStockRailway,

        /// Optional livery name or color scheme.
        livery: Option<String>,

        /// Optional length over buffers measurement.
        length_over_buffer: Option<LengthOverBuffers>,

        /// Optional technical specifications summary.
        technical_specifications: Option<TechnicalSpecifications>,

        /// Optional friendly/display name for the vehicle.
        friendly_name: Option<String>,

        /// Series or product code used for this rolling stock instance.
        series_code: String,

        /// Optional road number assigned to this vehicle.
        road_number: Option<String>,

        /// Optional series identifier.
        series: Option<String>,

        /// Optional depot/location assignment.
        depot: Option<String>,

        /// Specific railcar subtype.
        railcar_type: RailcarType,

        /// Optional DCC interface configuration.
        dcc_interface: Option<DccInterface>,

        /// Optional control type information.
        control: Option<Control>,

        /// Marker indicating a placeholder/dummy instance.
        is_dummy: bool,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::domain::manufacturer::ManufacturerId;
    use crate::catalog::domain::railway_model::Category;
    use crate::catalog::domain::railway_model::Epoch;
    use crate::catalog::domain::railway_model::PowerMethod;
    use crate::catalog::domain::railway_model::ProductCode;
    use crate::catalog::domain::railway_model::RailwayModelId;
    use crate::catalog::domain::scale::Scale;
    use pretty_assertions::{assert_eq, assert_ne};

    fn sample_rm_id(s: &str) -> RailwayModelId {
        RailwayModelId::try_from(s).unwrap()
    }

    fn sample_manufacturer() -> RailwayModelManufacturer {
        RailwayModelManufacturer {
            manufacturer_id: ManufacturerId::try_from("trn:manufacturer:bachmann").unwrap(),
            display: "Bachmann".to_string(),
        }
    }

    #[test]
    fn railway_model_view_eq_same_id() {
        let id = sample_rm_id("trn:railway-model:mn-1:ACME-100");
        let a = RailwayModelView {
            id: id.clone(),
            manufacturer: sample_manufacturer(),
            product_code: ProductCode::try_from("PC-1").unwrap(),
            description: "desc".into(),
            details: None,
            power_method: PowerMethod::DC,
            scale: Scale::H0,
            epoch: Epoch::from("IV"),
            category: Category::Locomotives,
            delivery_date: None,
            availability_status: None,
            metadata: Metadata::default(),
            rolling_stock: vec![],
        };
        let b = RailwayModelView {
            id: id.clone(),
            ..a.clone()
        };
        assert_eq!(a, b);
    }

    #[test]
    fn railway_model_view_eq_different_id() {
        let a = RailwayModelView {
            id: sample_rm_id("trn:railway-model:mn-1:ACME-101"),
            manufacturer: sample_manufacturer(),
            product_code: ProductCode::try_from("PC-1").unwrap(),
            description: "desc".into(),
            details: None,
            power_method: PowerMethod::DC,
            scale: Scale::H0,
            epoch: Epoch::from("IV"),
            category: Category::Locomotives,
            delivery_date: None,
            availability_status: None,
            metadata: Metadata::default(),
            rolling_stock: vec![],
        };
        let mut b = a.clone();
        b.id = sample_rm_id("trn:railway-model:mn-2:ACME-102");
        assert_ne!(a, b);
    }
}
