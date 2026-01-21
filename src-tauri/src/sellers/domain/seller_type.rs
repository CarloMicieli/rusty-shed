use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// Represents the type/category of a seller.
///
/// This enum is serialized/deserialized as screaming snake case (e.g. `SHOP`) and
/// persisted to the database as text. It is used across the sellers feature to
/// distinguish shops, private sellers, marketplaces and distributors.
///
/// The type is intentionally strongly typed to avoid stringly-typed code and to
/// provide a small surface for formatting and parsing (via `strum` and `sqlx`).
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    Default,
    specta::Type,
    sqlx::Type,
    EnumString,
    Display,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum SellerType {
    /// A retail shop or online store selling products directly to customers.
    #[default]
    Shop,

    /// A private (individual) seller — non-commercial person selling items.
    Private,

    /// An online marketplace aggregating multiple sellers.
    Marketplace,

    /// A commercial distributor or wholesaler that supplies retailers.
    Distributor,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use std::str::FromStr;

    #[rstest]
    #[case("SHOP", SellerType::Shop)]
    #[case("PRIVATE", SellerType::Private)]
    #[case("MARKETPLACE", SellerType::Marketplace)]
    #[case("DISTRIBUTOR", SellerType::Distributor)]
    fn parse_from_str(#[case] s: &str, #[case] expected: SellerType) {
        let parsed = SellerType::from_str(s).unwrap();
        assert_eq!(parsed, expected);
    }

    #[rstest]
    #[case(SellerType::Shop, "\"SHOP\"")]
    #[case(SellerType::Private, "\"PRIVATE\"")]
    fn serde_serialization(#[case] v: SellerType, #[case] expected_json: &str) {
        let json = serde_json::to_string(&v).unwrap();
        assert_eq!(json, expected_json);
        let de: SellerType = serde_json::from_str(&json).unwrap();
        assert_eq!(de, v);
    }

    #[test]
    fn it_should_display_outputs_variant_name() {
        // Display uses the serialized form (SCREAMING_SNAKE_CASE) as configured by `strum`.
        assert_eq!(format!("{}", SellerType::Marketplace), "MARKETPLACE");
    }
}
