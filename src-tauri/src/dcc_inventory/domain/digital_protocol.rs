use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// Digital communication protocols supported by decoders.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    Display,
    EnumString,
    specta::Type,
    sqlx::Type,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE", ascii_case_insensitive)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DigitalProtocol {
    Dcc,
    Mfx,
    Selectrix,
    Motorola,
    Fmz,
    Next18,
}

/// Garde validator for `DigitalProtocol`.
#[allow(dead_code)]
pub fn validate_digital_protocol(value: &str, _ctx: &()) -> garde::Result {
    if value.parse::<DigitalProtocol>().is_ok() {
        Ok(())
    } else {
        Err(garde::Error::new("error_invalid_digital_protocol"))
    }
}

#[cfg(test)]
mod tests {
    use super::DigitalProtocol;
    use rstest::rstest;
    use std::str::FromStr;

    /// Ensure Display (to_string) produces the SCREAMING_SNAKE_CASE form and
    /// that `FromStr` (EnumString) can parse both the canonical uppercase form
    /// and a lowercase variant thanks to `ascii_case_insensitive`.
    #[rstest]
    #[case(DigitalProtocol::Dcc, "DCC")]
    #[case(DigitalProtocol::Mfx, "MFX")]
    #[case(DigitalProtocol::Selectrix, "SELECTRIX")]
    #[case(DigitalProtocol::Motorola, "MOTOROLA")]
    #[case(DigitalProtocol::Fmz, "FMZ")]
    fn display_and_parse(#[case] proto: DigitalProtocol, #[case] expected: &str) {
        // Display -> canonical SCREAMING_SNAKE_CASE
        assert_eq!(proto.to_string(), expected);

        // FromStr parses the canonical form
        assert_eq!(DigitalProtocol::from_str(expected).unwrap(), proto);

        // FromStr is ASCII case-insensitive as configured; ensure lowercase parses
        assert_eq!(
            DigitalProtocol::from_str(&expected.to_lowercase()).unwrap(),
            proto
        );

        // Also try mixed case just to be thorough
        let mixed = expected
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if i % 2 == 0 {
                    c.to_ascii_lowercase()
                } else {
                    c
                }
            })
            .collect::<String>();
        assert_eq!(DigitalProtocol::from_str(&mixed).unwrap(), proto);
    }

    mod validator_tests {
        use crate::dcc_inventory::domain::digital_protocol::validate_digital_protocol;
        use rstest::rstest;

        #[rstest]
        #[case("DCC")]
        #[case("MFX")]
        fn validate_digital_protocol_accepts_all(#[case] s: &str) {
            assert!(validate_digital_protocol(s, &()).is_ok());
            assert!(validate_digital_protocol(&s.to_lowercase(), &()).is_ok());
        }

        #[test]
        fn validate_digital_protocol_rejects_invalid() {
            let err = validate_digital_protocol("NOPE", &()).unwrap_err();
            assert!(err.to_string().contains("error_invalid_digital_protocol"));
        }
    }
}
