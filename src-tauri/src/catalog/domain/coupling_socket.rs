use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, EnumString, Display, Default,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
pub enum CouplingSocket {
    #[serde(rename = "NONE")]
    #[strum(serialize = "NONE")]
    #[default]
    None,

    /// Receptacle for Replaceable Coupling Heads in Scales TT and N
    #[serde(rename = "NEM_355")]
    #[strum(serialize = "NEM_355")]
    Nem355,

    /// Coupler Head for Scale N
    #[serde(rename = "NEM_356")]
    #[strum(serialize = "NEM_356")]
    Nem356,

    /// Coupler Head for Scale N
    #[serde(rename = "NEM_357")]
    #[strum(serialize = "NEM_357")]
    Nem357,

    /// Coupler Head for Scale TT
    #[serde(rename = "NEM_359")]
    #[strum(serialize = "NEM_359")]
    Nem359,

    /// Standard Coupling for Scale H0
    #[serde(rename = "NEM_360")]
    #[strum(serialize = "NEM_360")]
    Nem360,

    /// NEM shaft 362 with close coupling mechanism
    #[serde(rename = "NEM_362")]
    #[strum(serialize = "NEM_362")]
    Nem362,

    /// Coupler Head for Scale 0
    #[serde(rename = "NEM_365")]
    #[strum(serialize = "NEM_365")]
    Nem365,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use strum::ParseError;

    #[rstest]
    #[case("NONE", Ok(CouplingSocket::None))]
    #[case("NEM_355", Ok(CouplingSocket::Nem355))]
    #[case("NEM_356", Ok(CouplingSocket::Nem356))]
    #[case("NEM_357", Ok(CouplingSocket::Nem357))]
    #[case("NEM_359", Ok(CouplingSocket::Nem359))]
    #[case("NEM_360", Ok(CouplingSocket::Nem360))]
    #[case("NEM_362", Ok(CouplingSocket::Nem362))]
    #[case("NEM_365", Ok(CouplingSocket::Nem365))]
    #[case("invalid", Err(ParseError::VariantNotFound))]
    fn it_should_parse_strings_as_couplings(
        #[case] input: &str,
        #[case] expected: Result<CouplingSocket, ParseError>,
    ) {
        let coupling = input.parse::<CouplingSocket>();
        assert_eq!(expected, coupling);
    }

    #[rstest]
    #[case(CouplingSocket::None, "NONE")]
    #[case(CouplingSocket::Nem355, "NEM_355")]
    #[case(CouplingSocket::Nem356, "NEM_356")]
    #[case(CouplingSocket::Nem357, "NEM_357")]
    #[case(CouplingSocket::Nem359, "NEM_359")]
    #[case(CouplingSocket::Nem360, "NEM_360")]
    #[case(CouplingSocket::Nem362, "NEM_362")]
    #[case(CouplingSocket::Nem365, "NEM_365")]
    fn it_should_display_couplings(#[case] input: CouplingSocket, #[case] expected: &str) {
        assert_eq!(expected, input.to_string());
    }
}
