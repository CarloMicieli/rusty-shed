//! Utilities for representing and working with a length-over-buffers value.
//!
//! This module provides the `LengthOverBuffers` value object which stores a
//! rail vehicle's overall length expressed both in inches and in
//! millimeters. The type is careful to keep the two representations in
//! sync and validates inputs via `LengthOverBuffers::new`. Use the
//! convenience constructors `from_inches` and `from_millimeters` when you
//! already have a `Length` value in the desired unit.

use crate::core::domain::length::Length;
use crate::core::domain::measure_units::MeasureUnit;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// The rail vehicle measurement method expressed as the length over buffers
///
/// `LengthOverBuffers` holds an optional length in both inches and
/// millimeters. When both values are provided they must represent the same
/// physical measure (the constructor will validate that). Values must be
/// positive. The type implements `Copy`/`Clone` and (de)serializes with
/// serde using the helpers in `crate::core::domain::length::serde`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct LengthOverBuffers {
    /// the overall length in inches
    #[serde(with = "crate::core::domain::length::serde::inches_option")]
    pub inches: Option<Length>,
    /// the overall length in millimeters
    #[serde(with = "crate::core::domain::length::serde::millimeters_option")]
    pub millimeters: Option<Length>,
}

impl LengthOverBuffers {
    /// Creates a new `LengthOverBuffers` from optional decimal values.
    ///
    /// Validation performed:
    /// - Provided values must be positive (non-zero).
    /// - If both `inches` and `millimeters` are provided they must be
    ///   consistent with each other (i.e. they represent the same physical
    ///   length). Consistency is checked using the `MeasureUnit` conversion
    ///   utilities.
    ///
    /// Parameters:
    /// - `inches`: optional decimal length in inches
    /// - `millimeters`: optional decimal length in millimeters
    ///
    /// Returns `Ok(LengthOverBuffers)` when inputs are valid, otherwise a
    /// `LengthOverBuffersError` describing the failure mode.
    pub fn new(
        inches: Option<Decimal>,
        millimeters: Option<Decimal>,
    ) -> Result<Self, LengthOverBuffersError> {
        match (inches, millimeters) {
            (Some(inches), _) if inches.is_sign_negative() || inches.is_zero() => {
                Err(LengthOverBuffersError::NonPositiveValue)
            }
            (_, Some(mm)) if mm.is_sign_negative() || mm.is_zero() => {
                Err(LengthOverBuffersError::NonPositiveValue)
            }
            (Some(inches), Some(mm))
                if !MeasureUnit::Millimeters.same_as(mm, MeasureUnit::Inches, inches) =>
            {
                Err(LengthOverBuffersError::DifferentValues)
            }
            _ => {
                let inches = inches.map(Length::Inches);
                let millimeters = millimeters.map(Length::Millimeters);
                Ok(LengthOverBuffers {
                    inches,
                    millimeters,
                })
            }
        }
    }

    /// Create a `LengthOverBuffers` value from a `Length` expressed in
    /// millimeters.
    ///
    /// The returned value will contain both millimetres and the converted
    /// inches value.
    pub fn from_millimeters(millimeters: Length) -> Self {
        let inches = MeasureUnit::Millimeters
            .to(MeasureUnit::Inches)
            .convert(millimeters.quantity());
        LengthOverBuffers {
            inches: Some(Length::Inches(inches)),
            millimeters: Some(millimeters),
        }
    }

    /// Create a `LengthOverBuffers` value from a `Length` expressed in
    /// inches.
    ///
    /// The returned value will contain both inches and the converted
    /// millimetres value.
    pub fn from_inches(inches: Length) -> Self {
        let millimeters = MeasureUnit::Inches
            .to(MeasureUnit::Millimeters)
            .convert(inches.quantity());
        LengthOverBuffers {
            inches: Some(inches),
            millimeters: Some(Length::Millimeters(millimeters)),
        }
    }

    /// Returns the optional length over buffers value in inches.
    ///
    /// Consumers that only need a single unit can use this accessor and
    /// fall back to conversions if necessary.
    pub fn inches(&self) -> Option<&Length> {
        self.inches.as_ref()
    }

    /// Returns the optional length over buffers value in millimetres.
    pub fn millimeters(&self) -> Option<&Length> {
        self.millimeters.as_ref()
    }
}

/// Errors that can occur while creating a `LengthOverBuffers`.
#[derive(Debug, PartialEq, Error)]
pub enum LengthOverBuffersError {
    /// Provided inch and millimetre values are not equivalent.
    #[error("the value in millimeters is not matching the one in inches")]
    DifferentValues,
    /// Values must be strictly positive (no zero or negative lengths).
    #[error("The length over buffers must be positive")]
    NonPositiveValue,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod length_over_buffer_tests {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use rust_decimal_macros::dec;

        #[rstest]
        #[case(None, None, Ok(LengthOverBuffers { inches: None, millimeters: None}))]
        #[case(Some(dec!(0.0)), Some(dec!(0.0)), Err(LengthOverBuffersError::NonPositiveValue))]
        #[case(Some(dec!(-0.65)), Some(dec!(-16.5)), Err(LengthOverBuffersError::NonPositiveValue))]
        #[case(Some(dec!(0.65)), Some(dec!(16.2)), Err(LengthOverBuffersError::DifferentValues))]
        fn it_should_create_new_length_over_buffers_values(
            #[case] inches: Option<Decimal>,
            #[case] millimeters: Option<Decimal>,
            #[case] expected: Result<LengthOverBuffers, LengthOverBuffersError>,
        ) {
            let result = LengthOverBuffers::new(inches, millimeters);
            assert_eq!(expected, result);
        }

        #[test]
        fn it_should_create_new_length_over_buffer_from_inches() {
            let inches = Length::Inches(dec!(42));
            let lob = LengthOverBuffers::from_inches(inches);
            assert_eq!(Some(&inches), lob.inches());
            assert_eq!(Some(&Length::Millimeters(dec!(1066.8))), lob.millimeters());
        }

        #[test]
        fn it_should_create_new_length_over_buffer_from_millimeters() {
            let millimeters = Length::Millimeters(dec!(42));
            let lob = LengthOverBuffers::from_millimeters(millimeters);
            assert_eq!(Some(&millimeters), lob.millimeters());
            assert_eq!(Some(&Length::Inches(dec!(1.6535442))), lob.inches());
        }

        #[test]
        fn it_should_serialize_as_json() {
            let inches = dec!(0.65);
            let millimeters = dec!(16.5);
            let value = TestStruct {
                length_over_buffers: LengthOverBuffers::new(Some(inches), Some(millimeters))
                    .expect("invalid length over buffers"),
            };

            let json = serde_json::to_string(&value).expect("invalid JSON value");

            let expected = r#"{"length_over_buffers":{"inches":0.65,"millimeters":16.5}}"#;
            assert_eq!(expected, json);
        }

        #[test]
        fn it_should_deserialize_from_json() {
            let inches = dec!(0.65);
            let millimeters = dec!(16.5);

            let json = r#"{"length_over_buffers":{"inches":0.65,"millimeters":16.5}}"#;

            let test_struct: TestStruct = serde_json::from_str(json).expect("Invalid test struct");

            assert_eq!(
                Some(inches),
                test_struct.length_over_buffers.inches.map(|l| l.quantity())
            );
            assert_eq!(
                Some(millimeters),
                test_struct
                    .length_over_buffers
                    .millimeters
                    .map(|l| l.quantity())
            );
        }

        #[test]
        fn it_should_deserialize_from_json_length_over_buffers_with_only_the_inches_value() {
            let inches = dec!(0.65);

            let json = r#"{"length_over_buffers":{"inches":0.65}}"#;

            let test_struct: TestStruct = serde_json::from_str(json).expect("Invalid test struct");

            assert_eq!(
                Some(inches),
                test_struct.length_over_buffers.inches.map(|l| l.quantity())
            );
            assert_eq!(None, test_struct.length_over_buffers.millimeters);
        }

        #[test]
        fn it_should_deserialize_from_json_length_over_buffers_with_only_the_millimeters_value() {
            let millimeters = dec!(16.5);

            let json = r#"{"length_over_buffers":{"millimeters":16.5}}"#;

            let test_struct: TestStruct = serde_json::from_str(json).expect("Invalid test struct");

            assert_eq!(None, test_struct.length_over_buffers.inches);
            assert_eq!(
                Some(millimeters),
                test_struct
                    .length_over_buffers
                    .millimeters
                    .map(|l| l.quantity())
            );
        }

        #[derive(Serialize, Deserialize)]
        struct TestStruct {
            length_over_buffers: LengthOverBuffers,
        }
    }
}
