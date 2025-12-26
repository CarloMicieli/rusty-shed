use once_cell::sync::Lazy;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize, Serializer};
use std::cmp;
use std::convert;
use std::fmt;
use std::ops;

/// Represents the ratio between a model railway scale and the real-world
/// prototype size.
///
/// The type wraps a `Decimal` and enforces validation rules when created via
/// `TryFrom<Decimal>`: the value must be strictly positive (>= 1) and within
/// an allowed maximum (220). Internally it is a transparent wrapper so it
/// serializes as a single numeric value.
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, specta::Type)]
#[specta(transparent)]
#[serde(transparent)]
pub struct Ratio(Decimal);

/// Conversion from `Decimal` performs validation and returns a `Ratio` on
/// success.
///
/// Validation rules:
/// - value must be positive and non-zero,
/// - value must be inside the inclusive range [1, 220].
impl convert::TryFrom<Decimal> for Ratio {
    type Error = RatioError;

    fn try_from(value: Decimal) -> Result<Self, Self::Error> {
        match value {
            _ if value.is_sign_negative() => Err(RatioError::NonPositiveValue(value)),
            _ if value.is_zero() => Err(RatioError::NonPositiveValue(value)),
            _ if value > dec!(220) => Err(RatioError::OutsideAllowedRange),
            _ if value < Decimal::ONE => Err(RatioError::OutsideAllowedRange),
            _ => Ok(Ratio(value)),
        }
    }
}

/// `Ratio` is serialized as a floating numeric value (not as a structured
/// object). We rely on rust_decimal's float serializer so JSON output will be
/// a plain number (e.g. `43.5`).
impl Serialize for Ratio {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        rust_decimal::serde::float::serialize(&self.0, serializer)
    }
}

/// Errors that may occur when creating a `Ratio`.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum RatioError {
    /// The provided ratio value is not positive (negative or zero).
    #[error("scale ratios must be positive (value: {0})")]
    NonPositiveValue(Decimal),

    /// The provided ratio is outside the allowed bounds (1..=220).
    #[error("scale ratios must be included in the 1-220 range")]
    OutsideAllowedRange,
}

impl fmt::Display for Ratio {
    /// Display a `Ratio` using the conventional `1:ratio` notation.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "1:{}", self.0)
    }
}

impl ops::Deref for Ratio {
    type Target = Decimal;

    /// Deref yields access to the inner `Decimal` value.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl convert::AsRef<Decimal> for Ratio {
    /// Obtain a reference to the underlying `Decimal` value.
    fn as_ref(&self) -> &Decimal {
        &self.0
    }
}

impl cmp::PartialOrd for Ratio {
    /// Partial ordering for `Ratio` delegates to `Ord` (total order).
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for Ratio {
    /// Note: ordering is implemented so that smaller numeric denominators are
    /// considered "greater" in the model domain (so that `1:87 > 1:160`).
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

impl Ratio {
    /// Convenience constructor for the common 1:87 scale ratio.
    pub fn r87() -> Self {
        Ratio(dec!(87))
    }

    /// Convenience constructor for the common 1:160 scale ratio (N scale).
    pub fn r160() -> Self {
        Ratio(dec!(160))
    }

    /// Convenience constructor for the common 1:120 scale ratio (TT scale).
    pub fn r120() -> Self {
        Ratio(dec!(120))
    }

    /// Convenience constructor for the common 1:220 scale ratio (Z scale).
    pub fn r220() -> Self {
        Ratio(dec!(220))
    }

    /// Convenience constructor for the garden G scale (1:22.5).
    pub fn r22_5() -> Self {
        Ratio(dec!(22.5))
    }

    /// Convenience constructor for scale 1 (1:32).
    pub fn r32() -> Self {
        Ratio(dec!(32))
    }

    /// Convenience constructor for 0 scale (1:43.5).
    pub fn r43_5() -> Self {
        Ratio(dec!(43.5))
    }

    /// Convenience constructor for 00 (1:76.2).
    pub fn r76_2() -> Self {
        Ratio(dec!(76.2))
    }
}

/// Common, shared `Ratio` values as thread-safe statics.
///
/// We expose these as `Lazy<Ratio>` statics because `Decimal` cannot be
/// constructed in a `const` context. These are cheap to initialize and safe
/// for concurrent access.
pub static R87: Lazy<Ratio> = Lazy::new(|| Ratio(dec!(87)));
pub static R160: Lazy<Ratio> = Lazy::new(|| Ratio(dec!(160)));
pub static R120: Lazy<Ratio> = Lazy::new(|| Ratio(dec!(120)));
pub static R220: Lazy<Ratio> = Lazy::new(|| Ratio(dec!(220)));
pub static R22_5: Lazy<Ratio> = Lazy::new(|| Ratio(dec!(22.5)));
pub static R32: Lazy<Ratio> = Lazy::new(|| Ratio(dec!(32)));
pub static R43_5: Lazy<Ratio> = Lazy::new(|| Ratio(dec!(43.5)));
pub static R76_2: Lazy<Ratio> = Lazy::new(|| Ratio(dec!(76.2)));

#[cfg(test)]
mod tests {
    use super::*;

    mod ratios {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use rust_decimal_macros::dec;

        #[test]
        fn it_should_create_new_ratios() {
            let value = dec!(87);
            assert_eq!(Ok(Ratio(value)), Ratio::try_from(value));
        }

        #[test]
        fn it_should_dereference_ratios() {
            let value = dec!(87);
            let ratio = Ratio(value);

            assert_eq!(&value, ratio.as_ref());
            assert_eq!(value, *ratio);
        }

        #[rstest]
        #[case(dec!(0))]
        #[case(dec!(-1))]
        fn it_should_only_allow_non_negative_ratios(#[case] input: Decimal) {
            let result = Ratio::try_from(input);
            assert_eq!(Err(RatioError::NonPositiveValue(input)), result);
        }

        #[rstest]
        #[case(dec!(0.9), Err(RatioError::OutsideAllowedRange))]
        #[case(dec!(1.0), Ok(Ratio(dec!(1.0))))]
        #[case(dec!(220.0), Ok(Ratio(dec!(220.0))))]
        #[case(dec!(221.0), Err(RatioError::OutsideAllowedRange))]
        fn it_should_check_if_the_input_is_inside_the_allowed_range(
            #[case] input: Decimal,
            #[case] expected: Result<Ratio, RatioError>,
        ) {
            let result = Ratio::try_from(input);
            assert_eq!(expected, result);
        }

        #[test]
        fn it_should_display_ratios() {
            let ratio1 = Ratio::try_from(dec!(87));
            assert_eq!("1:87", ratio1.unwrap().to_string());
        }

        #[test]
        fn it_should_compare_two_ratios() {
            let ratio1 = Ratio::try_from(dec!(87)).unwrap();
            let ratio2 = Ratio::try_from(dec!(160)).unwrap();

            assert!(ratio1 > ratio2, "1:87 > 1:160 must hold true");
            assert!(ratio2 < ratio1, "1:160 < 1:87 must hold true");
        }
    }

    mod ratio_serialization {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_serialize_ratios() {
            let value = TestStruct {
                ratio: Ratio::try_from(dec!(43.5)).unwrap(),
            };

            let json = serde_json::to_string(&value).expect("invalid json value");

            assert_eq!(r#"{"ratio":43.5}"#, json);
        }

        #[derive(Debug, Serialize)]
        struct TestStruct {
            ratio: Ratio,
        }
    }
}
