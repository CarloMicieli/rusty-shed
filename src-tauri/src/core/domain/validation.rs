use crate::core::domain::domain_error::DomainError;
use chrono::Local;
use serde::Serialize;
use std::borrow::Cow;
use std::collections::HashMap;
use std::str::FromStr;

/// A validation error returned by application use-cases.
///
/// The structure contains:
/// - `code`: a stable machine-readable identifier for the kind of validation error.
/// - `message`: an optional human-facing message (ideally localized at the caller).
/// - `params`: a map of parameter names to `ValidationErrorParam` values used to
///   provide structured additional data about the failure (for example, an
///   invalid numeric range or the name of a missing field).
///
/// Usage notes:
/// - `ValidationError.code` should be a stable machine identifier (for
///   example `required_field`, `invalid_format`, ...).
/// - `ValidationError.message` is an optional, human-facing text (localized
///   on the caller side when possible).
/// - `ValidationError.params` contains structured values that provide extra
///   details about the validation failure (numbers or text).
#[derive(Debug, PartialEq, Clone, Serialize, specta::Type)]
pub struct ValidationError {
    /// A stable machine-readable code identifying the validation error type.
    pub code: Cow<'static, str>,
    /// An optional human-facing message describing the validation error.
    pub message: Option<Cow<'static, str>>,
    /// A map of parameter names to values providing additional context.
    pub params: HashMap<Cow<'static, str>, ValidationErrorParam>,
}

/// Parameter values for a validation error.
///
/// The enum is tagged via serde so it serializes as an object with `type` and
/// `value` keys (useful for interop with TypeScript and the frontend).
#[derive(Debug, PartialEq, Clone, Serialize, specta::Type)]
#[serde(tag = "type", content = "value")]
#[specta(tag = "type", content = "value")]
pub enum ValidationErrorParam {
    /// A numeric parameter (for example: a failing boundary value).
    Number(i64),
    /// A textual parameter (for example: a field name or explanatory text).
    Text(Cow<'static, str>),
}

/// Context for collecting validation errors during input processing.
///     
/// This struct allows accumulating multiple validation errors associated
/// with different fields. It provides methods to collect results and add errors,
/// and finally to convert the collected errors into a `CommandError` if any exist.
#[derive(Debug, Default)]
pub struct ValidationContext {
    errors: HashMap<String, Vec<ValidationError>>,
}

impl ValidationContext {
    /// Collects a Result, adding an `invalid_format` error to the context if it is Err.
    /// Returns Some(T) if Ok, None if Err.
    pub fn collect<T, E: ToString>(&mut self, field: &str, res: Result<T, E>) -> Option<T> {
        match res {
            Ok(val) => Some(val),
            Err(e) => {
                self.push_error(field, "invalid_format", e.to_string());
                None
            }
        }
    }

    /// Adds a validation error for a specific field.
    pub fn push_error(
        &mut self,
        field: &str,
        code: impl Into<Cow<'static, str>>,
        message: impl Into<Cow<'static, str>>,
    ) {
        let err = ValidationError {
            code: code.into(),
            message: Some(message.into()),
            params: HashMap::new(),
        };

        self.errors.entry(field.to_string()).or_default().push(err);
    }

    /// Converts the context into a Result.
    /// If errors exist, returns Err(CommandError::ValidationError).
    pub fn finish(self) -> Result<(), DomainError> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(DomainError::ValidationError(self.errors))
        }
    }

    /// Convenience: try to parse an optional string into T using FromStr.
    /// If `value` is Some and parse fails, records an error and returns None.
    pub fn validate_opt_parse<T>(&mut self, field: &str, value: Option<String>) -> Option<T>
    where
        T: FromStr,
        <T as FromStr>::Err: ToString,
    {
        value.and_then(|s| self.collect(field, s.parse::<T>()))
    }

    /// Convenience: try to parse a required string into T using FromStr.
    /// Records an error on failure and returns None.
    pub fn validate_parse<T>(&mut self, field: &str, value: String) -> Option<T>
    where
        T: FromStr,
        <T as FromStr>::Err: ToString,
    {
        self.collect(field, value.parse::<T>())
    }

    /// Convenience: try to convert a required string into T using TryFrom<&str>.
    /// Records an error on failure and returns None.
    pub fn validate_try_from<T>(&mut self, field: &str, value: String) -> Option<T>
    where
        for<'a> T: TryFrom<&'a str>,
        for<'a> <T as TryFrom<&'a str>>::Error: ToString,
    {
        match T::try_from(value.as_str()) {
            Ok(v) => Some(v),
            Err(e) => {
                self.push_error(field, "invalid_format", e.to_string());
                None
            }
        }
    }

    /// Convenience: try to convert an optional string into T using TryFrom<&str>.
    pub fn validate_opt_try_from<T>(&mut self, field: &str, value: Option<String>) -> Option<T>
    where
        for<'a> T: TryFrom<&'a str>,
        for<'a> <T as TryFrom<&'a str>>::Error: ToString,
    {
        value.and_then(|s| self.validate_try_from::<T>(field, s))
    }

    /// Convenience: parse a vector of strings into Vec<T> using TryFrom<&str>, collecting
    /// successes and pushing per-item errors into the context.
    pub fn validate_vec_try_from<T>(&mut self, field: &str, values: Vec<String>) -> Vec<T>
    where
        for<'a> T: TryFrom<&'a str>,
        for<'a> <T as TryFrom<&'a str>>::Error: ToString,
    {
        let mut out = Vec::with_capacity(values.len());
        for v in values.into_iter() {
            if let Some(parsed) = self.validate_try_from::<T>(field, v) {
                out.push(parsed);
            }
        }
        out
    }
}

/// Garde validator: rejects a `NaiveDate` that is strictly in the future (after today).
pub fn validate_not_future_date(value: &chrono::NaiveDate, _: &()) -> garde::Result {
    let today = Local::now().date_naive();
    if *value > today {
        Err(garde::Error::new("error_date_in_future"))
    } else {
        Ok(())
    }
}

/// Garde validator: rejects an `Option<NaiveDate>` whose inner value is strictly in the future.
pub fn validate_opt_not_future_date(value: &Option<chrono::NaiveDate>, _: &()) -> garde::Result {
    match value {
        Some(d) => validate_not_future_date(d, &()),
        None => Ok(()),
    }
}

/// Garde validator: rejects a `&str` ISO date (`YYYY-MM-DD`) that is strictly in the future.
pub fn validate_not_future_iso_date(value: &str, _: &()) -> garde::Result {
    let d = chrono::NaiveDate::parse_from_str(value, "%Y-%m-%d")
        .map_err(|_| garde::Error::new("error_invalid_date_format"))?;
    validate_not_future_date(&d, &())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_should_serialize_validation_error_with_params() {
        let mut params = HashMap::new();
        params.insert(Cow::Borrowed("min_value"), ValidationErrorParam::Number(10));
        params.insert(
            Cow::Borrowed("field_name"),
            ValidationErrorParam::Text(Cow::Borrowed("age")),
        );

        let error = ValidationError {
            code: Cow::Borrowed("value_too_low"),
            message: Some(Cow::Borrowed("The value is below the minimum allowed.")),
            params,
        };

        let serialized = serde_json::to_value(&error).unwrap();
        let expected = json!({
            "code": "value_too_low",
            "message": "The value is below the minimum allowed.",
            "params": {
                "min_value": { "type": "Number", "value": 10 },
                "field_name": { "type": "Text", "value": "age" }
            }
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn it_should_validation_context_collect_and_finish() {
        let mut validation_context = ValidationContext::default();

        let value: Option<i32> = validation_context.collect("age", "not a number".parse::<i32>());

        assert!(value.is_none());

        let result = validation_context.finish();
        assert!(result.is_err());
        if let Err(DomainError::ValidationError(errors)) = result {
            assert!(errors.contains_key("age"));
            let age_errors = &errors["age"];
            assert_eq!(age_errors.len(), 1);
            assert_eq!(age_errors[0].code, "invalid_format");
        } else {
            panic!("Expected ValidationError");
        }
    }

    #[test]
    fn validate_not_future_date_accepts_past_date() {
        let past = chrono::NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        assert!(validate_not_future_date(&past, &()).is_ok());
    }

    #[test]
    fn validate_not_future_date_rejects_future_date() {
        let future = chrono::Local::now().naive_local().date() + chrono::Duration::days(1);
        let err = validate_not_future_date(&future, &()).unwrap_err();
        assert_eq!(err.to_string(), "error_date_in_future");
    }

    #[test]
    fn validate_opt_not_future_date_accepts_none() {
        assert!(validate_opt_not_future_date(&None, &()).is_ok());
    }

    #[test]
    fn validate_opt_not_future_date_accepts_past_some() {
        let past = chrono::NaiveDate::from_ymd_opt(2020, 6, 15).unwrap();
        assert!(validate_opt_not_future_date(&Some(past), &()).is_ok());
    }

    #[test]
    fn validate_opt_not_future_date_rejects_future_some() {
        let future = chrono::Local::now().naive_local().date() + chrono::Duration::days(1);
        let err = validate_opt_not_future_date(&Some(future), &()).unwrap_err();
        assert_eq!(err.to_string(), "error_date_in_future");
    }

    #[test]
    fn validate_not_future_iso_date_accepts_valid_past_string() {
        assert!(validate_not_future_iso_date("2020-01-01", &()).is_ok());
    }

    #[test]
    fn validate_not_future_iso_date_rejects_invalid_format() {
        let err = validate_not_future_iso_date("not-a-date", &()).unwrap_err();
        assert_eq!(err.to_string(), "error_invalid_date_format");
    }

    #[test]
    fn validate_not_future_iso_date_rejects_future_string() {
        let future = chrono::Local::now().naive_local().date() + chrono::Duration::days(1);
        let future_str = future.format("%Y-%m-%d").to_string();
        let err = validate_not_future_iso_date(&future_str, &()).unwrap_err();
        assert_eq!(err.to_string(), "error_date_in_future");
    }
}
