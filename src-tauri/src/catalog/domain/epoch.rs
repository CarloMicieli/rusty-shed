use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

/// Backwards-compatible wrapper used across the codebase and DB rows.
///
/// This preserves the original `Epoch(pub String)` API while providing
/// conversions to the structured `EpochKind` for validation and richer handling.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Epoch(pub String);

impl From<&str> for Epoch {
    fn from(s: &str) -> Self {
        Epoch(s.to_string())
    }
}

/// Structured representation of epochs (parsed form).
///
/// Use `EpochKind::try_from(&str)` to parse and `EpochKind::to_string()` / `Display`
/// to format. Convert to the old `Epoch` wrapper with `Into::<Epoch>::into(kind)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum BaseEpoch {
    I,
    II,
    III,
    IV,
    V,
    VI,
}

impl BaseEpoch {
    pub fn ordinal(&self) -> u8 {
        match self {
            BaseEpoch::I => 1,
            BaseEpoch::II => 2,
            BaseEpoch::III => 3,
            BaseEpoch::IV => 4,
            BaseEpoch::V => 5,
            BaseEpoch::VI => 6,
        }
    }
}

impl fmt::Display for BaseEpoch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            BaseEpoch::I => "I",
            BaseEpoch::II => "II",
            BaseEpoch::III => "III",
            BaseEpoch::IV => "IV",
            BaseEpoch::V => "V",
            BaseEpoch::VI => "VI",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for BaseEpoch {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "I" => Ok(BaseEpoch::I),
            "II" => Ok(BaseEpoch::II),
            "III" => Ok(BaseEpoch::III),
            "IV" => Ok(BaseEpoch::IV),
            "V" => Ok(BaseEpoch::V),
            "VI" => Ok(BaseEpoch::VI),
            _ => Err(()),
        }
    }
}

/// Half epoch marker (a or b)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    A,
    B,
}

impl fmt::Display for Half {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Half::A => write!(f, "a"),
            Half::B => write!(f, "b"),
        }
    }
}

/// Parsed, structured epoch representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EpochKind {
    Single {
        epoch: BaseEpoch,
        half: Option<Half>,
    },
    Range {
        start: BaseEpoch,
        end: BaseEpoch,
    },
    Museum,
}

impl fmt::Display for EpochKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EpochKind::Single { epoch, half } => match half {
                Some(h) => write!(f, "{}{}", epoch, h),
                None => write!(f, "{}", epoch),
            },
            EpochKind::Range { start, end } => write!(f, "{}/{}", start, end),
            EpochKind::Museum => write!(f, "Vm"),
        }
    }
}

// Serde for EpochKind (serialize/deserialize as string form)
impl Serialize for EpochKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for EpochKind {
    fn deserialize<D>(deserializer: D) -> Result<EpochKind, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        EpochKind::try_from(s.as_str()).map_err(de::Error::custom)
    }
}

const INVALID_EPOCH: &str = "invalid epoch";

impl TryFrom<&str> for EpochKind {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let s = value.trim();

        // museum
        if s.eq_ignore_ascii_case("Vm") {
            return Ok(EpochKind::Museum);
        }

        // range of form X/Y
        if let Some((l, r)) = s.split_once('/') {
            let left = BaseEpoch::from_str(l).map_err(|_| anyhow::anyhow!(INVALID_EPOCH))?;
            let right = BaseEpoch::from_str(r).map_err(|_| anyhow::anyhow!(INVALID_EPOCH))?;
            // only contiguous allowed
            return if right.ordinal() == left.ordinal() + 1 {
                Ok(EpochKind::Range {
                    start: left,
                    end: right,
                })
            } else {
                Err(anyhow::anyhow!(INVALID_EPOCH))
            };
        }

        // single with optional half (e.g., Ia, Ib)
        if s.len() >= 2 {
            let last = s.chars().last().unwrap();
            if last == 'a' || last == 'b' || last == 'A' || last == 'B' {
                let (base, half_ch) = s.split_at(s.len() - 1);
                let base_epoch =
                    BaseEpoch::from_str(base).map_err(|_| anyhow::anyhow!(INVALID_EPOCH))?;
                let half = match half_ch.chars().next().unwrap().to_ascii_lowercase() {
                    'a' => Half::A,
                    'b' => Half::B,
                    _ => return Err(anyhow::anyhow!(INVALID_EPOCH)),
                };
                return Ok(EpochKind::Single {
                    epoch: base_epoch,
                    half: Some(half),
                });
            }
        }

        // plain single I..VI
        let base = BaseEpoch::from_str(s).map_err(|_| anyhow::anyhow!(INVALID_EPOCH))?;
        Ok(EpochKind::Single {
            epoch: base,
            half: None,
        })
    }
}

impl From<EpochKind> for Epoch {
    fn from(k: EpochKind) -> Self {
        Epoch(k.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("I", EpochKind::Single { epoch: BaseEpoch::I, half: None })]
    #[case("Ia", EpochKind::Single { epoch: BaseEpoch::I, half: Some(Half::A) })]
    #[case("Ib", EpochKind::Single { epoch: BaseEpoch::I, half: Some(Half::B) })]
    #[case("Vm", EpochKind::Museum)]
    #[case("I/II", EpochKind::Range { start: BaseEpoch::I, end: BaseEpoch::II })]
    #[case("II/III", EpochKind::Range { start: BaseEpoch::II, end: BaseEpoch::III })]
    fn parse_valid(#[case] s: &str, #[case] expected: EpochKind) {
        let parsed = EpochKind::try_from(s).expect("should parse");
        assert_eq!(parsed, expected);
        // ensure symmetry between parsing and display
        assert_eq!(parsed.to_string(), s);

        // also round-trip via the string wrapper
        let wrapper: Epoch = parsed.into();
        assert_eq!(wrapper.0, s);
    }

    #[test]
    fn parse_invalid_non_contiguous_range() {
        let err = EpochKind::try_from("I/III");
        assert!(err.is_err());
    }

    #[test]
    fn parse_invalid_string() {
        let err = EpochKind::try_from("unknown");
        assert!(err.is_err());
    }
}
