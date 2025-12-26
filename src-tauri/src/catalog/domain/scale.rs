use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

// bring Gauge into scope so Scale::gauge can return it
use crate::catalog::domain::scale_gauge::Gauge;

/// Model railway scales supported by the application.
///
/// Each variant corresponds to a commonly used hobbyist scale name (for example
/// `H0` or `00`). Use `Scale::ratio()` to obtain the numeric ratio that follows
/// the `1:` notation (e.g. `Scale::H0` -> `87.0` meaning `1:87`). The `Display`
/// implementation produces a human-friendly string such as `H0 (1:87)`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub enum Scale {
    /// H0 scale (1:87)
    H0,
    /// H0 narrow/metric (1:87)
    H0m,
    /// H0e (1:87)
    H0e,
    /// N scale (1:160)
    N,
    /// TT scale (1:120)
    TT,
    /// Z scale (1:220)
    Z,
    /// G scale (garden) (1:22.5)
    G,
    /// 1 scale (1:32)
    Scale1,
    /// 0 scale (1:43.5)
    Scale0,
    /// 00 (double-zero) scale (1:76.2)
    Scale00,
}

impl Scale {
    /// Returns the numeric ratio (the denominator in `1:ratio`).
    ///
    /// Examples: `Scale::H0` -> `87.0`, `Scale::G` -> `22.5`.
    pub fn ratio(&self) -> f32 {
        match self {
            Scale::H0 => 87.0,
            Scale::H0m => 87.0,
            Scale::H0e => 87.0,
            Scale::N => 160.0,
            Scale::TT => 120.0,
            Scale::Z => 220.0,
            Scale::G => 22.5,
            Scale::Scale1 => 32.0,
            Scale::Scale0 => 43.5,
            Scale::Scale00 => 76.2,
        }
    }

    /// Returns the `Gauge` associated with this modeling `Scale`.
    ///
    /// This maps each `Scale` variant to the corresponding `Gauge` constant
    /// defined in `scale_gauge.rs` (for example `Scale::H0` -> `Gauge::H0`,
    /// `Scale::Scale1` -> `Gauge::ONE`). The returned value is a value copy
    /// of the associated constant and can be used directly by callers.
    pub fn gauge(&self) -> Gauge {
        match self {
            Scale::H0 => Gauge::H0,
            Scale::H0m => Gauge::H0M,
            Scale::H0e => Gauge::H0E,
            Scale::N => Gauge::N,
            Scale::TT => Gauge::TT,
            Scale::Z => Gauge::Z,
            Scale::G => Gauge::G,
            Scale::Scale1 => Gauge::ONE,
            Scale::Scale0 => Gauge::ZERO,
            Scale::Scale00 => Gauge::DOUBLE_ZERO,
        }
    }
}

impl fmt::Display for Scale {
    /// Format the scale as `LABEL (1:RATIO)`, for example `H0 (1:87)`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Scale::H0 => "H0",
            Scale::H0m => "H0m",
            Scale::H0e => "H0e",
            Scale::N => "N",
            Scale::TT => "TT",
            Scale::Z => "Z",
            Scale::G => "G",
            Scale::Scale1 => "1",
            Scale::Scale0 => "0",
            Scale::Scale00 => "00",
        };

        let ratio = self.ratio();
        // Format ratio without trailing `.0` when it's an integer, otherwise with 1 decimal
        let ratio_str = if (ratio - ratio.trunc()).abs() < f32::EPSILON {
            format!("{}", ratio as i32)
        } else {
            // show one decimal place for common fractions like 22.5 or 43.5
            format!("{:.1}", ratio)
        };

        write!(f, "{} (1:{})", label, ratio_str)
    }
}

// Static error message used when parsing fails
/// Error message used when parsing a string into a `Scale` fails.
const INVALID_SCALE: &str = "invalid scale";

impl Scale {
    /// Parse a short scale label into `Scale`.
    ///
    /// Accepts the compact form such as `"H0"`, `"N"`, `"00"`, `"1"` or `"0"`.
    fn from_short(short: &str) -> Result<Self, anyhow::Error> {
        match short {
            "H0" => Ok(Scale::H0),
            "H0m" => Ok(Scale::H0m),
            "H0e" => Ok(Scale::H0e),
            "N" => Ok(Scale::N),
            "TT" => Ok(Scale::TT),
            "Z" => Ok(Scale::Z),
            "G" => Ok(Scale::G),
            "1" => Ok(Scale::Scale1),
            "0" => Ok(Scale::Scale0),
            "00" => Ok(Scale::Scale00),
            _ => Err(anyhow::anyhow!(INVALID_SCALE)),
        }
    }
}

impl TryFrom<&str> for Scale {
    type Error = anyhow::Error;

    /// Attempts to parse a `Scale` from a string. Accepts either the short label
    /// (e.g. `"H0"`, `"00"`) or the full Display form such as `"H0 (1:87)"`.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let s = value.trim();
        // Try direct short label match first
        let short = s;

        if let Ok(scale) = Self::from_short(short) {
            return Ok(scale);
        }

        // If not matched, attempt to extract the leading label from Display-like input
        // e.g. "H0 (1:87)" or "1 (1:32)" -> take the substring before first space or '('
        let leading = s.split([' ', '(']).next().unwrap_or("").trim();

        Self::from_short(leading)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case(Scale::H0, "H0 (1:87)")]
    #[case(Scale::H0m, "H0m (1:87)")]
    #[case(Scale::H0e, "H0e (1:87)")]
    #[case(Scale::N, "N (1:160)")]
    #[case(Scale::TT, "TT (1:120)")]
    #[case(Scale::Z, "Z (1:220)")]
    #[case(Scale::G, "G (1:22.5)")]
    #[case(Scale::Scale1, "1 (1:32)")]
    #[case(Scale::Scale0, "0 (1:43.5)")]
    #[case(Scale::Scale00, "00 (1:76.2)")]
    fn display_variants(#[case] scale: Scale, #[case] expected: &str) {
        assert_eq!(scale.to_string(), expected);
    }

    #[rstest]
    #[case("H0", Scale::H0)]
    #[case("H0m", Scale::H0m)]
    #[case("H0e", Scale::H0e)]
    #[case("N", Scale::N)]
    #[case("TT", Scale::TT)]
    #[case("Z", Scale::Z)]
    #[case("G", Scale::G)]
    #[case("1", Scale::Scale1)]
    #[case("0", Scale::Scale0)]
    #[case("00", Scale::Scale00)]
    // also accept the Display output forms
    #[case("H0 (1:87)", Scale::H0)]
    #[case("H0m (1:87)", Scale::H0m)]
    #[case("H0e (1:87)", Scale::H0e)]
    #[case("N (1:160)", Scale::N)]
    #[case("TT (1:120)", Scale::TT)]
    #[case("Z (1:220)", Scale::Z)]
    #[case("G (1:22.5)", Scale::G)]
    #[case("1 (1:32)", Scale::Scale1)]
    #[case("0 (1:43.5)", Scale::Scale0)]
    #[case("00 (1:76.2)", Scale::Scale00)]
    fn try_from_valid_values(#[case] input: &str, #[case] expected: Scale) {
        let parsed = Scale::try_from(input).expect("should parse");
        assert_eq!(parsed, expected);
    }

    #[test]
    fn try_from_invalid_returns_error() {
        let err = Scale::try_from("unknown");
        assert!(err.is_err());
        let err = err.unwrap_err();
        assert!(format!("{}", err).contains(INVALID_SCALE));
    }

    // New test: ensure Scale::gauge maps each enum variant to the correct Gauge constant
    #[rstest]
    #[case(Scale::H0, Gauge::H0)]
    #[case(Scale::H0m, Gauge::H0M)]
    #[case(Scale::H0e, Gauge::H0E)]
    #[case(Scale::N, Gauge::N)]
    #[case(Scale::TT, Gauge::TT)]
    #[case(Scale::Z, Gauge::Z)]
    #[case(Scale::G, Gauge::G)]
    #[case(Scale::Scale1, Gauge::ONE)]
    #[case(Scale::Scale0, Gauge::ZERO)]
    #[case(Scale::Scale00, Gauge::DOUBLE_ZERO)]
    fn gauge_mappings(#[case] scale: Scale, #[case] expected: Gauge) {
        assert_eq!(scale.gauge(), expected);
    }
}
