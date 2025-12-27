use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// The NMRA and NEM Connectors for digital control (DCC)
///
/// # Description
/// The NMRA and NEM adopted standard mechanical and electrical interfaces to connect Multifunction
/// Decoders to a locomotive's electrical system. These plugs and sockets make it simpler to install
/// a decoder into a suitably equipped locomotive.
///
/// In many cases a blanking plug must be removed before installing the decoder. If a locomotive
/// is not DCC-Ready it will lack an interface and must use a Hardwired Decoder or a drop-in
/// replacement DCC control board (if available) for that specific model.
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, EnumString, Display, Serialize, Deserialize, specta::Type,
)]
#[strum(ascii_case_insensitive)]
pub enum DccInterface {
    /// 6 Pin standard mechanical and electrical interfaces (NMRA Small)
    #[serde(rename = "NEM_651")]
    #[strum(serialize = "NEM_651")]
    Nem651,

    /// 8 Pin standard mechanical and electrical interfaces (NMRA Medium)
    #[serde(rename = "NEM_652")]
    #[strum(serialize = "NEM_652")]
    Nem652,

    /// 4 Pin standard mechanical and electrical interfaces (NMRA Large)
    #[serde(rename = "NEM_654")]
    #[strum(serialize = "NEM_654")]
    Nem654,

    /// The PluX8 connector consists of two rows of 4 pins.
    #[serde(rename = "PLUX_8")]
    #[strum(serialize = "PLUX_8")]
    Plux8,

    #[serde(rename = "PLUX_12")]
    #[strum(serialize = "PLUX_12")]
    Plux12,

    /// The PluX16 connector consists of two rows of 8 pins.
    #[serde(rename = "PLUX_16")]
    #[strum(serialize = "PLUX_16")]
    Plux16,

    /// The PluX22 connector consists of two rows of 11 pins.
    #[serde(rename = "PLUX_22")]
    #[strum(serialize = "PLUX_22")]
    Plux22,

    /// standard connector for extremely tight applications, such as TT and N scale locomotives (NEM 662)
    #[serde(rename = "NEXT_18")]
    #[strum(serialize = "NEXT_18")]
    Next18,

    #[serde(rename = "NEXT_18_S")]
    #[strum(serialize = "NEXT_18_S")]
    Next18S,

    /// 21MTC Connector interface is a standard adopted by both the NMRA and NEM (NEM 660).
    /// Its name comes from 21 pin Marklin/Trix Connector, developed by Marklin and ESU.
    #[serde(rename = "MTC_21")]
    #[strum(serialize = "MTC_21")]
    Mtc21,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use strum::ParseError;

    #[rstest]
    #[case("NEM_651", Ok(DccInterface::Nem651))]
    #[case("NEM_652", Ok(DccInterface::Nem652))]
    #[case("NEM_654", Ok(DccInterface::Nem654))]
    #[case("PLUX_8", Ok(DccInterface::Plux8))]
    #[case("PLUX_12", Ok(DccInterface::Plux12))]
    #[case("PLUX_16", Ok(DccInterface::Plux16))]
    #[case("PLUX_22", Ok(DccInterface::Plux22))]
    #[case("NEXT_18", Ok(DccInterface::Next18))]
    #[case("NEXT_18_S", Ok(DccInterface::Next18S))]
    #[case("MTC_21", Ok(DccInterface::Mtc21))]
    fn parse_shouting_case(
        #[case] input: &str,
        #[case] expected: Result<DccInterface, ParseError>,
    ) {
        let result = input.parse::<DccInterface>();
        assert_eq!(expected, result);
    }

    #[test]
    fn parse_invalid_returns_error() {
        let result = "NO_SUCH_INTERFACE".parse::<DccInterface>();
        assert_eq!(Err(ParseError::VariantNotFound), result);
    }

    #[test]
    fn parse_lowercase() {
        // ascii_case_insensitive should accept lowercase
        let result = "nem_651".parse::<DccInterface>();
        assert_eq!(Ok(DccInterface::Nem651), result);
    }

    #[rstest]
    #[case(DccInterface::Nem651, "NEM_651")]
    #[case(DccInterface::Nem652, "NEM_652")]
    #[case(DccInterface::Nem654, "NEM_654")]
    #[case(DccInterface::Plux8, "PLUX_8")]
    #[case(DccInterface::Plux12, "PLUX_12")]
    #[case(DccInterface::Plux16, "PLUX_16")]
    #[case(DccInterface::Plux22, "PLUX_22")]
    #[case(DccInterface::Next18, "NEXT_18")]
    #[case(DccInterface::Next18S, "NEXT_18_S")]
    #[case(DccInterface::Mtc21, "MTC_21")]
    fn display_variants(#[case] input: DccInterface, #[case] expected: &str) {
        assert_eq!(expected, input.to_string());
    }
}
