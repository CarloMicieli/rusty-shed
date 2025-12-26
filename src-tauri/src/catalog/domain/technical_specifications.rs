use serde::{Deserialize, Serialize};

pub use crate::catalog::domain::body_shell_type::BodyShellType;
pub use crate::catalog::domain::chassis_type::ChassisType;
pub use crate::catalog::domain::coupling::Coupling;
pub use crate::catalog::domain::coupling_socket::CouplingSocket;
pub use crate::catalog::domain::feature_flag::FeatureFlag;
pub use crate::catalog::domain::radius::{Radius, RadiusError};

/// The technical specification data for a rolling stock model
#[derive(Debug, Eq, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct TechnicalSpecifications {
    /// the minimum drivable radius
    pub minimum_radius: Option<Radius>,
    /// the coupling
    pub coupling: Option<Coupling>,
    /// has a flywheel fitted
    pub flywheel_fitted: Option<FeatureFlag>,
    /// body shell type
    pub body_shell: Option<BodyShellType>,
    /// chassis type
    pub chassis: Option<ChassisType>,
    /// has interior lighting
    pub interior_lights: Option<FeatureFlag>,
    /// has lights
    pub lights: Option<FeatureFlag>,
    /// has sprung buffers
    pub sprung_buffers: Option<FeatureFlag>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TechnicalSpecificationsBuilder {
    minimum_radius: Option<Radius>,
    coupling: Option<Coupling>,
    flywheel_fitted: Option<FeatureFlag>,
    chassis: Option<ChassisType>,
    body_shell: Option<BodyShellType>,
    interior_lights: Option<FeatureFlag>,
    lights: Option<FeatureFlag>,
    sprung_buffers: Option<FeatureFlag>,
}

impl TechnicalSpecificationsBuilder {
    /// with the minimum radius
    pub fn with_minimum_radius(mut self, radius: Radius) -> Self {
        self.minimum_radius = Some(radius);
        self
    }

    /// with the coupling shaft
    pub fn with_coupling(mut self, coupling: Coupling) -> Self {
        self.coupling = Some(coupling);
        self
    }

    /// with flywheel fitted
    pub fn with_flywheel_fitted(mut self) -> Self {
        self.flywheel_fitted = Some(FeatureFlag::Yes);
        self
    }

    /// with body shell type
    pub fn with_body_shell(mut self, body_shell_types: BodyShellType) -> Self {
        self.body_shell = Some(body_shell_types);
        self
    }

    /// with chassis type
    pub fn with_chassis(mut self, chassis_types: ChassisType) -> Self {
        self.chassis = Some(chassis_types);
        self
    }

    /// with interior lights
    pub fn with_interior_lights(mut self) -> Self {
        self.interior_lights = Some(FeatureFlag::Yes);
        self
    }

    /// with headlights
    pub fn with_lights(mut self) -> Self {
        self.lights = Some(FeatureFlag::Yes);
        self
    }

    /// with sprung buffers
    pub fn with_sprung_buffers(mut self) -> Self {
        self.sprung_buffers = Some(FeatureFlag::Yes);
        self
    }

    /// Build a new technical specifications value
    pub fn build(self) -> TechnicalSpecifications {
        TechnicalSpecifications {
            minimum_radius: self.minimum_radius,
            coupling: self.coupling,
            flywheel_fitted: self.flywheel_fitted,
            body_shell: self.body_shell,
            chassis: self.chassis,
            interior_lights: self.interior_lights,
            lights: self.lights,
            sprung_buffers: self.sprung_buffers,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use rust_decimal_macros::dec;

    #[test]
    fn it_should_create_tech_specs() {
        let coupling = Coupling::new(CouplingSocket::Nem362, FeatureFlag::Yes, FeatureFlag::No);

        let radius = Radius::from_millimeters(dec!(360)).unwrap();
        let tech_specs = TechnicalSpecificationsBuilder::default()
            .with_coupling(coupling)
            .with_chassis(ChassisType::Plastic)
            .with_body_shell(BodyShellType::MetalDieCast)
            .with_minimum_radius(radius)
            .with_interior_lights()
            .with_lights()
            .with_sprung_buffers()
            .with_flywheel_fitted()
            .build();

        assert_eq!(Some(coupling), tech_specs.coupling);
        assert_eq!(Some(radius), tech_specs.minimum_radius);
        assert_eq!(Some(ChassisType::Plastic), tech_specs.chassis);
        assert_eq!(Some(BodyShellType::MetalDieCast), tech_specs.body_shell);
        assert_eq!(Some(FeatureFlag::Yes), tech_specs.interior_lights);
        assert_eq!(Some(FeatureFlag::Yes), tech_specs.lights);
        assert_eq!(Some(FeatureFlag::Yes), tech_specs.sprung_buffers);
        assert_eq!(Some(FeatureFlag::Yes), tech_specs.flywheel_fitted);
    }
}
