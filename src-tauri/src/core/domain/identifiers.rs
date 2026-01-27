/// Trait for TRN-based identifiers with automatic validation, parsing, and formatting.
///
/// This trait provides a consistent interface for all domain identifiers, ensuring they:
/// - Follow the TRN format (prefix:part1:part2:...)
/// - Are properly validated
/// - Can be created from parts (with automatic slugification)
/// - Can be parsed from strings
/// - Have proper Display implementation
///
/// # Requirements
///
/// Implementing types must:
/// - Implement `AsRef<str>` to expose the underlying string
/// - Be `Sized` (automatically satisfied for most types)
/// - Define a `PREFIX` constant with the resource type prefix
/// - Provide `from_string_unchecked` to construct the type from a validated string
pub trait Identifier: Sized + AsRef<str> {
    /// The prefix for this identifier type (e.g., "trn:manufacturer").
    ///
    /// This prefix is used for validation and automatically prepended when creating
    /// identifiers from parts.
    const PREFIX: &'static str;

    /// Create an identifier from a string without validation.
    ///
    /// # Arguments
    /// * `s` - A validated identifier string
    ///
    /// # Returns
    ///
    /// A new instance of the implementing type
    ///
    /// # Safety
    ///
    /// This method assumes the input string is already validated and in the correct format.
    /// It should only be called internally by the trait's provided methods after validation.
    fn from_string_unchecked(s: String) -> Self;

    /// Create a new identifier from parts, slugifying each part.
    ///
    /// This method takes a slice of string parts, slugifies each one (converts to lowercase,
    /// replaces spaces with hyphens, removes special characters), and joins them with the
    /// prefix to form a complete identifier.
    ///
    /// # Arguments
    /// * `parts` - A slice of string slices representing the identifier components
    ///
    /// # Returns
    /// A new identifier with slugified parts
    fn new_from_parts(parts: &[&str]) -> Self {
        let slugified: Vec<String> = parts.iter().map(slug::slugify).collect();
        let full_id = format!("{}:{}", Self::PREFIX, slugified.join(":"));
        Self::from_string_unchecked(full_id)
    }

    /// Check if a string is a valid identifier of this type.
    ///
    /// This method validates that:
    /// - The string starts with the correct prefix
    /// - The string has at least one part after the prefix
    /// - The string is not empty
    ///
    /// # Arguments
    ///
    /// * `input` - The string to validate
    ///
    /// # Returns
    ///
    /// `true` if the input is valid, `false` otherwise
    fn is_valid(input: &str) -> bool {
        if input.is_empty() {
            return false;
        }

        // Check prefix
        if !input.starts_with(Self::PREFIX) {
            return false;
        }

        // Ensure there's at least one part after the prefix
        let expected_prefix_with_colon = format!("{}:", Self::PREFIX);
        if !input.starts_with(&expected_prefix_with_colon) {
            return false;
        }

        // Ensure there's content after the prefix
        let remainder = &input[expected_prefix_with_colon.len()..];
        !remainder.is_empty()
    }

    /// Helper function for implementing `TryFrom<&str>`.
    ///
    /// Use this in your `TryFrom` implementations to validate and parse identifiers.
    fn try_from_str(value: &str) -> Result<Self, IdParseError> {
        if !Self::is_valid(value) {
            if value.is_empty() || !value.starts_with(Self::PREFIX) {
                return Err(IdParseError::InvalidPrefix {
                    expected: Self::PREFIX.to_string(),
                    input: value.to_string(),
                });
            }
            return Err(IdParseError::InvalidFormat {
                reason: format!("Invalid identifier format: '{}'", value),
            });
        }

        Ok(Self::from_string_unchecked(value.to_string()))
    }
}

/// Error type for identifier parsing failures.
///
/// This error is returned when attempting to parse a string into an identifier
/// and the string doesn't match the expected format.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum IdParseError {
    /// The identifier has an incorrect prefix.
    ///
    /// Contains the expected prefix and the actual input string.
    #[error("Invalid prefix: expected '{expected}', got '{input}'")]
    InvalidPrefix {
        /// The expected prefix for this identifier type
        expected: String,
        /// The actual input that was provided
        input: String,
    },

    /// The identifier format is invalid (e.g., empty or missing parts).
    ///
    /// Contains a description of what went wrong.
    #[error("Invalid format: {reason}")]
    InvalidFormat {
        /// Description of why the format is invalid
        reason: String,
    },
}

/// Macro to implement TryFrom and Display for identifier types.
#[macro_export]
macro_rules! impl_identifier_traits {
    ($type:ty) => {
        impl std::convert::TryFrom<&str> for $type {
            type Error = $crate::core::domain::identifiers::IdParseError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                <$type as $crate::core::domain::identifiers::Identifier>::try_from_str(value)
            }
        }

        impl std::convert::TryFrom<String> for $type {
            type Error = $crate::core::domain::identifiers::IdParseError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                <$type>::try_from(value.as_str())
            }
        }

        impl std::convert::TryFrom<&String> for $type {
            type Error = $crate::core::domain::identifiers::IdParseError;

            fn try_from(value: &String) -> Result<Self, Self::Error> {
                <$type>::try_from(value.as_str())
            }
        }

        impl std::fmt::Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.as_ref())
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    // Test identifier for unit tests
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(crate) struct TestId(String);

    impl AsRef<str> for TestId {
        fn as_ref(&self) -> &str {
            &self.0
        }
    }

    impl Identifier for TestId {
        const PREFIX: &'static str = "trn:test";

        fn from_string_unchecked(s: String) -> Self {
            TestId(s)
        }
    }

    impl_identifier_traits!(TestId);

    #[test]
    fn test_new_from_parts_single_part() {
        let id = TestId::new_from_parts(&["hello"]);
        assert_eq!(id.as_ref(), "trn:test:hello");
    }

    #[test]
    fn test_new_from_parts_multiple_parts() {
        let id = TestId::new_from_parts(&["hello", "world"]);
        assert_eq!(id.as_ref(), "trn:test:hello:world");
    }

    #[test]
    fn test_new_from_parts_slugifies() {
        let id = TestId::new_from_parts(&["Hello World", "Test 123"]);
        assert_eq!(id.as_ref(), "trn:test:hello-world:test-123");
    }

    #[test]
    fn test_new_from_parts_with_special_chars() {
        let id = TestId::new_from_parts(&["ACME Corp!", "Model #456"]);
        assert_eq!(id.as_ref(), "trn:test:acme-corp:model-456");
    }

    #[test]
    fn test_is_valid_accepts_correct_format() {
        assert!(TestId::is_valid("trn:test:foo"));
        assert!(TestId::is_valid("trn:test:foo:bar"));
        assert!(TestId::is_valid("trn:test:a-b-c"));
    }

    #[test]
    fn test_is_valid_rejects_wrong_prefix() {
        assert!(!TestId::is_valid("trn:other:foo"));
        assert!(!TestId::is_valid("invalid:test:foo"));
    }

    #[test]
    fn test_is_valid_rejects_empty() {
        assert!(!TestId::is_valid(""));
    }

    #[test]
    fn test_is_valid_rejects_prefix_only() {
        assert!(!TestId::is_valid("trn:test"));
        assert!(!TestId::is_valid("trn:test:"));
    }

    #[test]
    fn test_try_from_str_success() {
        let result = TestId::try_from("trn:test:valid");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_ref(), "trn:test:valid");
    }

    #[test]
    fn test_try_from_string_success() {
        let result = TestId::try_from("trn:test:valid".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_ref(), "trn:test:valid");
    }

    #[test]
    fn test_try_from_string_ref_success() {
        let s = "trn:test:valid".to_string();
        let result = TestId::try_from(&s);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_ref(), "trn:test:valid");
    }

    #[test]
    fn test_try_from_invalid_prefix() {
        let result = TestId::try_from("trn:other:invalid");
        assert!(result.is_err());
        match result.unwrap_err() {
            IdParseError::InvalidPrefix { expected, input } => {
                assert_eq!(expected, "trn:test");
                assert_eq!(input, "trn:other:invalid");
            }
            _ => panic!("Expected InvalidPrefix error"),
        }
    }

    #[test]
    fn test_try_from_empty_string() {
        let result = TestId::try_from("");
        assert!(result.is_err());
        match result.unwrap_err() {
            IdParseError::InvalidPrefix { .. } => {}
            _ => panic!("Expected InvalidPrefix error"),
        }
    }

    #[test]
    fn test_display() {
        let id = TestId::new_from_parts(&["display", "test"]);
        assert_eq!(format!("{}", id), "trn:test:display:test");
    }

    #[test]
    fn test_clone() {
        let id1 = TestId::new_from_parts(&["clone", "test"]);
        let id2 = id1.clone();
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_partial_eq() {
        let id1 = TestId::new_from_parts(&["eq", "test"]);
        let id2 = TestId::new_from_parts(&["eq", "test"]);
        let id3 = TestId::new_from_parts(&["different"]);
        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }
}

#[cfg(test)]
mod validator_tests {
    use super::*;
    use crate::core::domain::identifiers::tests::TestId;

    #[test]
    fn validate_id_rejects_invalid() {
        // empty string
        let res = validate_id::<TestId>("", &());
        assert!(res.is_err());
        assert!(
            res.unwrap_err()
                .to_string()
                .contains("error_invalid_identifier")
        );

        // wrong prefix
        let res2 = validate_id::<TestId>("invalid:foo", &());
        assert!(res2.is_err());
    }

    #[test]
    fn validate_id_accepts_valid() {
        let id = TestId::new_from_parts(&["my", "item"]);
        let res = validate_id::<TestId>(id.as_ref(), &());
        assert!(res.is_ok());
    }
}

/// Garde validator for domain identifiers used in command arguments.
///
/// Usage in garde attributes: `#[garde(validate = "core::domain::identifiers::validate_id::<MyId>")]`
pub fn validate_id<T: Identifier>(id: &str, _ctx: &()) -> garde::Result {
    if T::is_valid(id) {
        Ok(())
    } else {
        Err(garde::Error::new("error_invalid_identifier"))
    }
}
