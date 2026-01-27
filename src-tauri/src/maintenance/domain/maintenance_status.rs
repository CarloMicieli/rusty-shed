use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// Represents the maintenance status for a rolling stock item.
///
/// The status is calculated by comparing the `next_maintenance_date` with
/// the current local date. `UpToDate` means the next maintenance is in the
/// future, `Due` means it's today, `Overdue` means in the past, and
/// `Unknown` indicates missing scheduling information.
#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumString, Display, specta::Type,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MaintenanceStatus {
    /// Next maintenance date is strictly in the future.
    UpToDate,

    /// Next maintenance date is today.
    Due,

    /// Next maintenance date is in the past.
    Overdue,

    /// No scheduling information available.
    Unknown,
}

impl MaintenanceStatus {
    /// Calculate the `MaintenanceStatus` for a given optional `next_date`.
    ///
    /// - If `next_date` is `None` returns `MaintenanceStatus::Unknown`.
    /// - If `next_date` == today returns `Due`.
    /// - If `next_date` > today returns `UpToDate`.
    /// - If `next_date` < today returns `Overdue`.
    pub fn calculate_status(next_date: Option<NaiveDate>) -> Self {
        match next_date {
            None => MaintenanceStatus::Unknown,
            Some(d) => {
                let today = chrono::Local::now().date_naive();
                if d == today {
                    MaintenanceStatus::Due
                } else if d > today {
                    MaintenanceStatus::UpToDate
                } else {
                    MaintenanceStatus::Overdue
                }
            }
        }
    }
}

/// Garde validator for `MaintenanceStatus`.
#[allow(dead_code)]
pub fn validate_maintenance_status(value: &str, _ctx: &()) -> garde::Result {
    if value.parse::<MaintenanceStatus>().is_ok() {
        Ok(())
    } else {
        Err(garde::Error::new("error_invalid_maintenance_status"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    use rstest::rstest;

    #[rstest]
    #[case(0, MaintenanceStatus::Due)]
    #[case(1, MaintenanceStatus::UpToDate)]
    #[case(-1, MaintenanceStatus::Overdue)]
    fn calculate_status_relative(#[case] offset_days: i64, #[case] expected: MaintenanceStatus) {
        let today = chrono::Local::now().date_naive();
        let next = today
            .checked_add_signed(Duration::days(offset_days))
            .unwrap();
        assert_eq!(MaintenanceStatus::calculate_status(Some(next)), expected);
    }

    #[test]
    fn it_should_calculate_status_unknown() {
        assert_eq!(
            MaintenanceStatus::calculate_status(None),
            MaintenanceStatus::Unknown
        );
    }

    #[rstest]
    #[case(MaintenanceStatus::UpToDate, "UP_TO_DATE")]
    #[case(MaintenanceStatus::Due, "DUE")]
    #[case(MaintenanceStatus::Overdue, "OVERDUE")]
    #[case(MaintenanceStatus::Unknown, "UNKNOWN")]
    fn display_and_parse(#[case] status: MaintenanceStatus, #[case] text: &str) {
        // Display uses SCREAMING_SNAKE_CASE per `strum` attributes
        assert_eq!(status.to_string(), text);

        // Parsing is ascii_case_insensitive via `strum` attribute
        let parsed = text.parse::<MaintenanceStatus>().unwrap();
        assert_eq!(parsed, status);

        let lower = text.to_lowercase();
        assert_eq!(lower.parse::<MaintenanceStatus>().unwrap(), status);
    }

    mod validator_tests {
        use super::*;
        use rstest::rstest;

        #[rstest]
        #[case("UP_TO_DATE")]
        #[case("DUE")]
        #[case("OVERDUE")]
        fn validate_maintenance_status_accepts_all(#[case] s: &str) {
            assert!(validate_maintenance_status(s, &()).is_ok());
            assert!(validate_maintenance_status(&s.to_lowercase(), &()).is_ok());
        }

        #[test]
        fn validate_maintenance_status_rejects_invalid() {
            let err = validate_maintenance_status("BAD", &()).unwrap_err();
            assert!(err.to_string().contains("error_invalid_maintenance_status"));
        }
    }
}
