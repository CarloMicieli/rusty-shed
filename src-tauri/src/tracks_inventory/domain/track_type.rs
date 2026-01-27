use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, specta::Type, sqlx::Type,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "TEXT", rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
pub enum TrackType {
    Straight,
    Curve,
    Turnout,
    FlexTrack,
}

/// Garde validator for `TrackType`.
#[allow(dead_code)]
pub fn validate_track_type(value: &str, _ctx: &()) -> garde::Result {
    if value.parse::<TrackType>().is_ok() {
        Ok(())
    } else {
        Err(garde::Error::new("error_invalid_track_type"))
    }
}

#[cfg(test)]
mod validator_tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("STRAIGHT")]
    #[case("CURVE")]
    fn validate_track_type_accepts_all(#[case] s: &str) {
        assert!(validate_track_type(s, &()).is_ok());
        assert!(validate_track_type(&s.to_lowercase(), &()).is_ok());
    }

    #[test]
    fn validate_track_type_rejects_invalid() {
        let err = validate_track_type("XXX", &()).unwrap_err();
        assert!(err.to_string().contains("error_invalid_track_type"));
    }
}
