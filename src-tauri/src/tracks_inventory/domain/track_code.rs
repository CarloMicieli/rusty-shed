use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

/// Rail profile code for a track product.
///
/// This enum lists the common model railway rail heights ("Code" values)
/// used to describe the rail profile. It derives `specta::Type` so it can be
/// emitted into TypeScript bindings and `EnumString` to allow parsing from
/// textual representations.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, specta::Type, sqlx::Type,
)]
#[sqlx(type_name = "TEXT")]
#[strum(ascii_case_insensitive)]
pub enum TrackCode {
    /// 70-series rail profile (lightweight rail, often used for smaller scales).
    #[serde(rename = "CODE_70")]
    #[sqlx(rename = "CODE_70")]
    #[strum(serialize = "CODE_70")]
    Code70,

    /// 75-series rail profile.
    #[serde(rename = "CODE_75")]
    #[sqlx(rename = "CODE_75")]
    #[strum(serialize = "CODE_75")]
    Code75,

    /// 83-series rail profile (common medium-weight rail for many layouts).
    #[serde(rename = "CODE_83")]
    #[sqlx(rename = "CODE_83")]
    #[strum(serialize = "CODE_83")]
    Code83,

    /// 100-series rail profile (heavy-duty rail profile).
    #[serde(rename = "CODE_100")]
    #[sqlx(rename = "CODE_100")]
    #[strum(serialize = "CODE_100")]
    Code100,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json;
    use std::str::FromStr;

    #[test]
    fn serde_serialization_tokens() {
        assert_eq!(
            serde_json::to_string(&TrackCode::Code70).unwrap(),
            "\"CODE_70\""
        );
        assert_eq!(
            serde_json::to_string(&TrackCode::Code75).unwrap(),
            "\"CODE_75\""
        );
        assert_eq!(
            serde_json::to_string(&TrackCode::Code83).unwrap(),
            "\"CODE_83\""
        );
        assert_eq!(
            serde_json::to_string(&TrackCode::Code100).unwrap(),
            "\"CODE_100\""
        );
    }

    #[test]
    fn serde_deserialization_tokens() {
        assert_eq!(
            serde_json::from_str::<TrackCode>("\"CODE_70\"").unwrap(),
            TrackCode::Code70
        );
        assert_eq!(
            serde_json::from_str::<TrackCode>("\"CODE_75\"").unwrap(),
            TrackCode::Code75
        );
        assert_eq!(
            serde_json::from_str::<TrackCode>("\"CODE_83\"").unwrap(),
            TrackCode::Code83
        );
        assert_eq!(
            serde_json::from_str::<TrackCode>("\"CODE_100\"").unwrap(),
            TrackCode::Code100
        );
    }

    #[test]
    fn fromstr_ascii_case_insensitive() {
        assert_eq!(TrackCode::from_str("code_70").unwrap(), TrackCode::Code70);
        assert_eq!(TrackCode::from_str("Code_75").unwrap(), TrackCode::Code75);
        assert_eq!(TrackCode::from_str("cOdE_83").unwrap(), TrackCode::Code83);
        assert_eq!(TrackCode::from_str("CODE_100").unwrap(), TrackCode::Code100);
    }

    #[test]
    fn fromstr_invalid() {
        assert!(TrackCode::from_str("NOT_A_CODE").is_err());
    }
}
