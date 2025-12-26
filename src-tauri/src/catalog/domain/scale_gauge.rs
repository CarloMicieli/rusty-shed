use crate::catalog::domain::track_gauge::TrackGauge;
use crate::core::domain::length::Length;
use crate::core::domain::measure_units::MeasureUnit;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::cmp;

/// Represents the track gauge information for a modelling scale.
///
/// A `Gauge` holds the distance between the rails expressed in both
/// millimeters and inches together with the corresponding `TrackGauge`
/// classification (for example `Standard`, `Narrow`, etc.). The struct
/// preserves both units to make round-trip conversions lossless for
/// presentation and storage.
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, specta::Type)]
pub struct Gauge {
    /// The distance between the rails in millimeters.
    ///
    /// This field is serialized/deserialized using the dedicated `Length`
    /// serde helpers so it can be stored consistently in DB rows.
    #[serde(with = "crate::core::domain::length::serde::millimeters")]
    pub millimeters: Length,
    /// The distance between the rails in inches.
    #[serde(with = "crate::core::domain::length::serde::inches")]
    pub inches: Length,
    /// The track gauge classification (e.g. `Standard`, `Narrow`).
    pub track_gauge: TrackGauge,
}
impl Gauge {
    /// Create a new `Gauge`.
    ///
    /// Validates that both `millimeters` and `inches` are positive and that
    /// the two values represent the same physical distance (using the
    /// configured `MeasureUnit` conversion). If validation succeeds returns
    /// `Ok(Gauge)` otherwise returns a `GaugeError` describing the problem.
    pub fn new(
        track_gauge: TrackGauge,
        millimeters: Decimal,
        inches: Decimal,
    ) -> Result<Self, GaugeError> {
        match (millimeters, inches) {
            (mm, _) if mm.is_sign_negative() || mm.is_zero() => Err(
                GaugeError::NegativeRailsDistance(mm, MeasureUnit::Millimeters),
            ),
            (_, inches) if inches.is_sign_negative() || inches.is_zero() => Err(
                GaugeError::NegativeRailsDistance(inches, MeasureUnit::Inches),
            ),
            (mm, inches) if !MeasureUnit::Millimeters.same_as(mm, MeasureUnit::Inches, inches) => {
                Err(GaugeError::DifferentValues)
            }
            (_, _) => Ok(Gauge {
                millimeters: Length::Millimeters(millimeters),
                inches: Length::Inches(inches),
                track_gauge,
            }),
        }
    }

    /// Create a `Gauge` from an inch measurement.
    ///
    /// This converts the provided `inches` value to millimeters and delegates
    /// to `Gauge::new` for validation. Useful when the source data is in
    /// imperial units.
    pub fn from_inches(track_gauge: TrackGauge, inches: Decimal) -> Result<Self, GaugeError> {
        let millimeters = MeasureUnit::Inches
            .to(MeasureUnit::Millimeters)
            .convert(inches)
            // Round millimeters to 1 decimal to match the stored constants (e.g. 16.5)
            .round_dp(1);
        Gauge::new(track_gauge, millimeters, inches)
    }

    /// Create a `Gauge` from a millimeter measurement.
    ///
    /// This converts the provided `millimeters` value to inches and delegates
    /// to `Gauge::new` for validation. Useful when the source data is in
    /// metric units.
    pub fn from_millimeters(
        track_gauge: TrackGauge,
        millimeters: Decimal,
    ) -> Result<Self, GaugeError> {
        let inches = MeasureUnit::Millimeters
            .to(MeasureUnit::Inches)
            .convert(millimeters)
            // Round inches to 3 decimal places to match the stored constants (e.g. 1.772)
            .round_dp(3);
        Gauge::new(track_gauge, millimeters, inches)
    }

    /// Returns the distance between the rails in millimeters as a `Length`.
    pub fn millimeters(&self) -> Length {
        self.millimeters
    }

    /// Returns the distance between the rails in inches as a `Length`.
    pub fn inches(&self) -> Length {
        self.inches
    }

    /// Returns the `TrackGauge` classification for this gauge.
    pub fn track_gauge(&self) -> TrackGauge {
        self.track_gauge
    }

    /// The gauge for the H0 scale (standard gauge).
    ///
    /// Convenience constant for the commonly-used H0 track gauge: 16.5 mm
    /// (approximately 0.65 in).
    pub const H0: Gauge = Gauge {
        track_gauge: TrackGauge::Standard,
        millimeters: Length::Millimeters(dec!(16.5)),
        inches: Length::Inches(dec!(0.65)),
    };

    /// The gauge for the N scale (standard gauge).
    ///
    /// Convenience constant for the commonly-used N track gauge: 9.0 mm
    /// (approximately 0.354 in).
    pub const N: Gauge = Gauge {
        track_gauge: TrackGauge::Standard,
        millimeters: Length::Millimeters(dec!(9.0)),
        inches: Length::Inches(dec!(0.354)),
    };

    /// H0 narrow/metric variant (commonly modelled on 12 mm track).
    pub const H0M: Gauge = Gauge {
        track_gauge: TrackGauge::Narrow,
        millimeters: Length::Millimeters(dec!(12.0)),
        inches: Length::Inches(dec!(0.472)),
    };

    /// H0e / H0 narrow (uses N gauge track in many modeling systems: 9.0 mm).
    pub const H0E: Gauge = Gauge {
        track_gauge: TrackGauge::Narrow,
        millimeters: Length::Millimeters(dec!(9.0)),
        inches: Length::Inches(dec!(0.354)),
    };

    /// TT scale typical track gauge (12.0 mm).
    pub const TT: Gauge = Gauge {
        track_gauge: TrackGauge::Standard,
        millimeters: Length::Millimeters(dec!(12.0)),
        inches: Length::Inches(dec!(0.472)),
    };

    /// Z scale typical track gauge (6.5 mm).
    pub const Z: Gauge = Gauge {
        track_gauge: TrackGauge::Minimum,
        millimeters: Length::Millimeters(dec!(6.5)),
        inches: Length::Inches(dec!(0.256)),
    };

    /// G / Gauge 1 family (garden) commonly uses 45.0 mm track.
    pub const G: Gauge = Gauge {
        track_gauge: TrackGauge::Broad,
        millimeters: Length::Millimeters(dec!(45.0)),
        inches: Length::Inches(dec!(1.772)),
    };

    /// Scale 1 (1:32) typically uses ~45.0 mm track.
    pub const ONE: Gauge = Gauge {
        track_gauge: TrackGauge::Broad,
        millimeters: Length::Millimeters(dec!(45.0)),
        inches: Length::Inches(dec!(1.772)),
    };

    /// 0 scale (1:43.5) typical model gauge ~33.0 mm.
    pub const ZERO: Gauge = Gauge {
        track_gauge: TrackGauge::Broad,
        millimeters: Length::Millimeters(dec!(33.0)),
        inches: Length::Inches(dec!(1.299)),
    };

    /// 00 (double-zero) uses the same 16.5 mm track as H0.
    pub const DOUBLE_ZERO: Gauge = Gauge {
        track_gauge: TrackGauge::Standard,
        millimeters: Length::Millimeters(dec!(16.5)),
        inches: Length::Inches(dec!(0.65)),
    };
}

impl cmp::PartialOrd for Gauge {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.millimeters.partial_cmp(&other.millimeters)
    }
}

/// Errors returned when constructing or validating a `Gauge`.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum GaugeError {
    /// The provided rails distance is negative or zero. The `Decimal` value is
    /// the supplied magnitude and the `MeasureUnit` indicates the unit used
    /// (millimeters or inches).
    #[error("the distance between rails must be positive ({0} {1})")]
    NegativeRailsDistance(Decimal, MeasureUnit),
    /// The provided millimeter and inch values do not match after unit
    /// conversion — this indicates inconsistent input data.
    #[error("the value in millimeters is not matching the one in inches")]
    DifferentValues,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn millimeters_value(length: Length) -> Decimal {
        match length {
            Length::Millimeters(d) => d,
            _ => panic!("expected millimeters length"),
        }
    }

    fn inches_value(length: Length) -> Decimal {
        match length {
            Length::Inches(d) => d,
            _ => panic!("expected inches length"),
        }
    }

    #[test]
    fn constants_have_expected_values() {
        // core/common scales
        assert_eq!(millimeters_value(Gauge::H0.millimeters()), dec!(16.5));
        assert_eq!(inches_value(Gauge::H0.inches()), dec!(0.65));

        assert_eq!(millimeters_value(Gauge::N.millimeters()), dec!(9.0));
        assert_eq!(inches_value(Gauge::N.inches()), dec!(0.354));

        // H0 variants
        assert_eq!(millimeters_value(Gauge::H0M.millimeters()), dec!(12.0));
        assert_eq!(inches_value(Gauge::H0M.inches()), dec!(0.472));

        assert_eq!(millimeters_value(Gauge::H0E.millimeters()), dec!(9.0));
        assert_eq!(inches_value(Gauge::H0E.inches()), dec!(0.354));

        // other scales
        assert_eq!(millimeters_value(Gauge::TT.millimeters()), dec!(12.0));
        assert_eq!(inches_value(Gauge::TT.inches()), dec!(0.472));

        assert_eq!(millimeters_value(Gauge::Z.millimeters()), dec!(6.5));
        assert_eq!(inches_value(Gauge::Z.inches()), dec!(0.256));

        assert_eq!(millimeters_value(Gauge::G.millimeters()), dec!(45.0));
        assert_eq!(inches_value(Gauge::G.inches()), dec!(1.772));

        // previously added checks
        assert_eq!(millimeters_value(Gauge::ONE.millimeters()), dec!(45.0));
        assert_eq!(inches_value(Gauge::ONE.inches()), dec!(1.772));

        assert_eq!(millimeters_value(Gauge::ZERO.millimeters()), dec!(33.0));
        assert_eq!(inches_value(Gauge::ZERO.inches()), dec!(1.299));

        assert_eq!(
            millimeters_value(Gauge::DOUBLE_ZERO.millimeters()),
            dec!(16.5)
        );
        assert_eq!(inches_value(Gauge::DOUBLE_ZERO.inches()), dec!(0.65));
    }

    #[test]
    fn from_millimeters_roundtrip() {
        let g =
            Gauge::from_millimeters(TrackGauge::Broad, dec!(45.0)).expect("should create gauge");
        // compare millimeters exactly and inches rounded to 3 dp
        assert_eq!(millimeters_value(g.millimeters()), dec!(45.0));
        assert_eq!(inches_value(g.inches()).round_dp(3), dec!(1.772));

        let g0 =
            Gauge::from_millimeters(TrackGauge::Broad, dec!(33.0)).expect("should create gauge");
        assert_eq!(millimeters_value(g0.millimeters()), dec!(33.0));
        assert_eq!(inches_value(g0.inches()).round_dp(3), dec!(1.299));
    }

    #[test]
    fn from_inches_roundtrip_double_zero() {
        let g = Gauge::from_inches(TrackGauge::Standard, dec!(0.65)).expect("should create gauge");
        assert_eq!(millimeters_value(g.millimeters()).round_dp(1), dec!(16.5));
        assert_eq!(inches_value(g.inches()), dec!(0.65));
    }

    #[test]
    fn ordering_by_millimeters() {
        // ONE (45) > ZERO (33) > DOUBLE_ZERO (16.5)
        assert!(Gauge::ONE > Gauge::ZERO);
        assert!(Gauge::ZERO > Gauge::DOUBLE_ZERO);
        assert!(Gauge::ONE > Gauge::DOUBLE_ZERO);
    }
}
