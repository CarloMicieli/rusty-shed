use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;

use once_cell::sync::Lazy;
use regex::Regex;

/// Quarter of a year for delivery date representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Quarter {
    Q1,
    Q2,
    Q3,
    Q4,
}

impl fmt::Display for Quarter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Quarter::Q1 => write!(f, "Q1"),
            Quarter::Q2 => write!(f, "Q2"),
            Quarter::Q3 => write!(f, "Q3"),
            Quarter::Q4 => write!(f, "Q4"),
        }
    }
}

impl FromStr for Quarter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "Q1" => Ok(Quarter::Q1),
            "Q2" => Ok(Quarter::Q2),
            "Q3" => Ok(Quarter::Q3),
            "Q4" => Ok(Quarter::Q4),
            other => Err(format!("invalid quarter: {}", other)),
        }
    }
}

/// Delivery date for a model. The representation can be one of:
///
/// - `Year(year)` — only the year is known (e.g. `2026`)
/// - `YearMonth { year, month }` — year and numeric month (displayed as "yyyy/mm")
/// - `YearQuarter { year, quarter }` — year and quarter (displayed as "yyyy/Qn")
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeliveryDate {
    Year(i32),
    YearMonth { year: i32, month: u8 },
    YearQuarter { year: i32, quarter: Quarter },
}

impl fmt::Display for DeliveryDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeliveryDate::Year(y) => write!(f, "{:04}", y),
            DeliveryDate::YearMonth { year, month } => write!(f, "{:04}/{:02}", year, month),
            DeliveryDate::YearQuarter { year, quarter } => write!(f, "{:04}/{}", year, quarter),
        }
    }
}

// Regular expressions used by the parser. Compiled once for efficiency.
static RE_YEAR: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(?P<year>\d{4})$").expect("invalid RE_YEAR regex"));
static RE_YM: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?P<year>\d{4})/(?P<month>\d{1,2})$").expect("invalid RE_YM regex"));
// Case-insensitive quarter match (e.g. Q1 or q1)
static RE_YQ: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)^(?P<year>\d{4})/Q(?P<q>[1-4])$").expect("invalid RE_YQ regex"));

impl DeliveryDate {
    /// Parse a delivery date from a string. Accepted formats:
    /// - `YYYY` (year)
    /// - `YYYY/MM` or `YYYY/M` (month 1-12)
    /// - `YYYY/Qn` where n is 1-4 (quarter)
    pub fn parse(s: &str) -> Result<Self, String> {
        let s = s.trim();
        if s.is_empty() {
            return Err("empty delivery date".to_string());
        }

        // Year-only: match with regex
        if let Some(caps) = RE_YEAR.captures(s)
            && let Some(year_str) = caps.name("year")
            && let Ok(year) = year_str.as_str().parse::<i32>()
            && (1000..=9999).contains(&year)
        {
            return Ok(DeliveryDate::Year(year));
        }

        // Year/Quarter (case-insensitive Q)
        if let Some(caps) = RE_YQ.captures(s) {
            let year_str = caps.name("year").expect("regex matched but 'year' capture missing").as_str();
            let q_str = caps.name("q").expect("regex matched but 'q' capture missing").as_str();
            if let Ok(year) = year_str.parse::<i32>()
                && let Ok(qn) = q_str.parse::<u8>()
            {
                let quarter = match qn {
                    1 => Quarter::Q1,
                    2 => Quarter::Q2,
                    3 => Quarter::Q3,
                    4 => Quarter::Q4,
                    _ => return Err(format!("invalid quarter number: {}", qn)),
                };
                return Ok(DeliveryDate::YearQuarter { year, quarter });
            }
        }

        // Year/Month
        if let Some(caps) = RE_YM.captures(s) {
            let year_str = caps.name("year").expect("regex matched but 'year' capture missing").as_str();
            let month_str = caps.name("month").expect("regex matched but 'month' capture missing").as_str();
            if let Ok(year) = year_str.parse::<i32>()
                && let Ok(month) = month_str.parse::<u8>()
                && (1..=12).contains(&month)
            {
                return Ok(DeliveryDate::YearMonth { year, month });
            }
        }

        Err(format!("could not parse delivery date: {}", s))
    }
}

// Serde support: serialize as string using Display, deserialize by parsing string
impl Serialize for DeliveryDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for DeliveryDate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DeliveryDate::parse(&s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use serde_json;

    #[rstest]
    #[case("2026", DeliveryDate::Year(2026))]
    #[case("2026/7", DeliveryDate::YearMonth { year: 2026, month: 7 })]
    #[case("2026/07", DeliveryDate::YearMonth { year: 2026, month: 7 })]
    #[case("2026/Q3", DeliveryDate::YearQuarter { year: 2026, quarter: Quarter::Q3 })]
    #[case("2026/q1", DeliveryDate::YearQuarter { year: 2026, quarter: Quarter::Q1 })]
    fn parse_ok(#[case] input: &str, #[case] expected: DeliveryDate) {
        let d = DeliveryDate::parse(input).expect("should parse");
        assert_eq!(d, expected);
    }

    #[rstest]
    #[case("2026/1", DeliveryDate::YearMonth { year: 2026, month: 1 })]
    #[case("2026/12", DeliveryDate::YearMonth { year: 2026, month: 12 })]
    #[case("2026/Q4", DeliveryDate::YearQuarter { year: 2026, quarter: Quarter::Q4 })]
    #[case(" 2026 ", DeliveryDate::Year(2026))]
    fn parse_edge_ok(#[case] input: &str, #[case] expected: DeliveryDate) {
        let d = DeliveryDate::parse(input).expect("should parse edge case");
        assert_eq!(d, expected);
    }

    #[rstest]
    #[case("")]
    #[case("20")]
    #[case("abcd")]
    #[case("2026/13")]
    #[case("2026/Q5")]
    #[case("2026/0")]
    #[case("10000")] // year out of allowed range (1000..=9999)
    #[case("0999")] // year below allowed range
    #[case("-2026")] // negative year
    #[case("2026/ 7")] // whitespace after slash -> invalid because parts are not trimmed
    #[case("2026 /07")] // whitespace before slash -> invalid
    fn parse_err(#[case] input: &str) {
        assert!(
            DeliveryDate::parse(input).is_err(),
            "{} should be invalid",
            input
        );
    }

    #[rstest]
    #[case(DeliveryDate::Year(2026), "2026")]
    #[case(DeliveryDate::YearMonth { year: 2026, month: 1 }, "2026/01")]
    #[case(DeliveryDate::YearMonth { year: 2026, month: 12 }, "2026/12")]
    #[case(DeliveryDate::YearQuarter { year: 2026, quarter: Quarter::Q4 }, "2026/Q4")]
    fn display_cases(#[case] value: DeliveryDate, #[case] expected: &str) {
        assert_eq!(value.to_string(), expected);
    }

    #[rstest]
    #[case(DeliveryDate::Year(1000))]
    #[case(DeliveryDate::YearMonth { year: 2026, month: 1 })]
    #[case(DeliveryDate::YearMonth { year: 2026, month: 12 })]
    #[case(DeliveryDate::YearQuarter { year: 2026, quarter: Quarter::Q4 })]
    fn serde_roundtrip(#[case] orig: DeliveryDate) {
        let json = serde_json::to_string(&orig).expect("serialize");
        let de: DeliveryDate = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(orig, de);
    }
}
