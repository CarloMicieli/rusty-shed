use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// Types of maintenance tasks commonly performed on rolling stock.
#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type, Display, EnumString, Default,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MaintenanceType {
    /// Cleaning the electrical contact surfaces of the wheels to ensure steady power pickup.
    WheelCleaning,
    /// Removing dust, oxidation, or "black gunk" from the rails of the layout.
    TrackCleaning,
    /// Cleaning internal electrical wipers or brass pick-ups that transfer power from wheels to the motor.
    ContactCleaning,

    /// Applying light plastic-safe oil to axles, bearings, or motor shafts.
    Lubrication,
    /// Applying heavy-duty grease to gear towers and worm gears within the drivetrain.
    GearGrease,
    /// Replacing the carbon brushes and springs within a DC motor to restore performance.
    MotorBrushReplacement,
    /// Replacing the rubber traction tires on driving wheels to restore pulling power.
    TractionTireReplacement,

    /// Installing a new DCC (Digital Command Control) decoder, including hard-wiring or plug-and-play.
    DecoderInstall,
    /// Updating the internal software/firmware of a digital decoder via a programmer.
    FirmwareUpdate,
    /// Replacing or upgrading the speaker or enclosure for sound-enabled locomotives.
    SpeakerRepair,
    /// Installing capacitors (PowerPacks) to prevent stalling over dirty track or insulated frogs.
    StayAliveInstall,

    /// Adjusting coupler height, centering springs, or replacing trip pins for reliable switching.
    CouplerAdjustment,
    /// Re-attaching or replacing fine scale details like handrails, whistles, or air hoses.
    DetailRepair,
    /// Applying powders, airbrushing, or washes to simulate real-world grime and age.
    Weathering,

    /// A standard "check-up" involving a visual inspection and a short test run.
    GeneralInspection,
    /// Any maintenance task not covered by the standard categories.
    #[default]
    Other,
}

// rust
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(MaintenanceType::WheelCleaning, "WHEEL_CLEANING")]
    #[case(MaintenanceType::TrackCleaning, "TRACK_CLEANING")]
    #[case(MaintenanceType::ContactCleaning, "CONTACT_CLEANING")]
    #[case(MaintenanceType::Lubrication, "LUBRICATION")]
    #[case(MaintenanceType::GearGrease, "GEAR_GREASE")]
    #[case(MaintenanceType::MotorBrushReplacement, "MOTOR_BRUSH_REPLACEMENT")]
    #[case(MaintenanceType::TractionTireReplacement, "TRACTION_TIRE_REPLACEMENT")]
    #[case(MaintenanceType::DecoderInstall, "DECODER_INSTALL")]
    #[case(MaintenanceType::FirmwareUpdate, "FIRMWARE_UPDATE")]
    #[case(MaintenanceType::SpeakerRepair, "SPEAKER_REPAIR")]
    #[case(MaintenanceType::StayAliveInstall, "STAY_ALIVE_INSTALL")]
    #[case(MaintenanceType::CouplerAdjustment, "COUPLER_ADJUSTMENT")]
    #[case(MaintenanceType::DetailRepair, "DETAIL_REPAIR")]
    #[case(MaintenanceType::Weathering, "WEATHERING")]
    #[case(MaintenanceType::GeneralInspection, "GENERAL_INSPECTION")]
    #[case(MaintenanceType::Other, "OTHER")]
    fn display_and_parse(#[case] typ: MaintenanceType, #[case] text: &str) {
        // Display produces SCREAMING_SNAKE_CASE
        assert_eq!(typ.to_string(), text);

        // Parsing is case-insensitive
        assert_eq!(text.parse::<MaintenanceType>().unwrap(), typ);
        assert_eq!(text.to_lowercase().parse::<MaintenanceType>().unwrap(), typ);
    }

    #[test]
    fn default_is_other() {
        assert_eq!(MaintenanceType::default(), MaintenanceType::Other);
    }

    #[test]
    fn parse_unknown_is_err() {
        assert!(
            "INVALID_MAINTENANCE_TYPE"
                .parse::<MaintenanceType>()
                .is_err()
        );
    }
}
