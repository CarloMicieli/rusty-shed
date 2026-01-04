// src-tauri/src/catalog/domain/radius.rs
use crate::core::domain::length::Length;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;
use thiserror::Error;

/// Domain types for handling minimum drivable radii.
///
/// A `Radius` wraps a `Length` expressed in millimeters and enforces
/// that the value is non-negative.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize, specta::Type)]
pub struct Radius(#[serde(with = "crate::core::domain::length::serde::millimeters")] Length);

impl Radius {
    /// Create a new `Radius` from a millimeters value.
    ///
    /// The provided `value` is interpreted as millimeters. Returns
    /// `Ok(Radius)` if `value` is positive, otherwise returns
    /// `Err(RadiusError::NegativeRadius)`.
    ///
    /// # Errors
    ///
    /// Returns `RadiusError::NegativeRadius` when `value` is negative.
    pub fn from_millimeters(value: Decimal) -> Result<Self, RadiusError> {
        if value.is_sign_positive() {
            Ok(Radius(Length::Millimeters(value)))
        } else {
            Err(RadiusError::NegativeRadius)
        }
    }

    /// Return the underlying `Length` for this radius.
    ///
    /// The returned `Length` is expressed in millimeters.
    pub fn value(&self) -> Length {
        self.0
    }
}

impl fmt::Display for Radius {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Errors that can occur when creating a `Radius`.
#[derive(Debug, Eq, PartialEq, Error)]
pub enum RadiusError {
    #[error("radius cannot be negative")]
    NegativeRadius,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::measure_units::MeasureUnit;
    use pretty_assertions::assert_eq;
    use rust_decimal_macros::dec;

    #[test]
    fn it_should_create_a_new_radius_in_millimeters() {
        let radius = Radius::from_millimeters(dec!(360.0)).expect("unable to create the radius");
        assert_eq!(
            Length::new(dec!(360), MeasureUnit::Millimeters),
            radius.value()
        );
    }

    #[test]
    fn it_should_fail_to_create_negative_radius() {
        let result = Radius::from_millimeters(dec!(-1.0));
        assert_eq!(Err(RadiusError::NegativeRadius), result);
    }

    #[test]
    fn it_should_display_a_radius() {
        let radius = Radius::from_millimeters(dec!(360.0)).unwrap();
        assert_eq!("360.0 mm", radius.to_string());
    }

    #[test]
    fn it_should_serialize_radius_as_json() {
        let value = TestStruct {
            radius: Radius::from_millimeters(dec!(360.0)).unwrap(),
        };

        let json = serde_json::to_string(&value).expect("Invalid JSON value");

        assert_eq!(r#"{"radius":360.0}"#, json);
    }

    #[test]
    fn it_should_deserialize_radius_from_json() {
        let json = r#"{"radius":360.0}"#;
        let value = TestStruct {
            radius: Radius::from_millimeters(dec!(360.0)).unwrap(),
        };

        let deserialize_value: TestStruct = serde_json::from_str(json).expect("Invalid JSON value");

        assert_eq!(value, deserialize_value);
    }

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct TestStruct {
        radius: Radius,
    }
}
