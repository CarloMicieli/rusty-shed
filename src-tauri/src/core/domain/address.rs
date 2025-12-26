use isocountry::CountryCode;
use serde::{Deserialize, Serialize};

/// It represents a physical street address
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub struct Address {
    /// the street address
    pub street_address: String,
    /// the (optional) extended information for the address
    pub extended_address: Option<String>,
    /// the city/town
    pub city: String,
    /// the region code; for example, the state or province.
    pub region: Option<String>,
    /// the postal code (ZIP code)
    pub postal_code: String,
    /// the ISO country code (ISO 3166-1 alpha-3)
    #[specta(type = String)]
    pub country: CountryCode,
}

impl Address {
    /// the street address
    pub fn street_address(&self) -> &str {
        &self.street_address
    }

    /// the (optional) extended information for the address
    pub fn extended_address(&self) -> Option<&str> {
        self.extended_address.as_deref()
    }

    /// the city/town
    pub fn city(&self) -> &str {
        &self.city
    }

    /// the region code; for example, the state or province.
    pub fn region(&self) -> Option<&str> {
        self.region.as_deref()
    }

    /// the postal code (ZIP code)
    pub fn postal_code(&self) -> &str {
        &self.postal_code
    }

    /// the ISO country code (ISO 3166-1 alpha-3)
    pub fn country_code(&self) -> CountryCode {
        self.country
    }

    /// Creates a new address builder
    pub fn builder() -> AddressBuilder {
        AddressBuilder::default()
    }
}

/// Builder for `Address`.
///
/// Construct an `Address` using the fluent-style builder API. Required fields are:
/// - street address
/// - city
/// - postal code
/// - country
///
/// Call `build()` to validate and obtain the final `Address` or an error indicating
/// which required field is missing.
#[derive(Default)]
pub struct AddressBuilder {
    street_address: Option<String>,
    extended_address: Option<String>,
    city: Option<String>,
    region: Option<String>,
    postal_code: Option<String>,
    country_code: Option<CountryCode>,
}

impl AddressBuilder {
    /// Set the street address.
    ///
    /// Example: `"22 Acacia Avenue"`.
    pub fn street_address(mut self, street_address: &str) -> AddressBuilder {
        self.street_address = Some(street_address.to_owned());
        self
    }

    /// Set the optional extended address information.
    ///
    /// Example: `"Apt. 999"`.
    pub fn extended_address(mut self, extended_address: &str) -> AddressBuilder {
        self.extended_address = Some(extended_address.to_owned());
        self
    }

    /// Set the city / town for the address.
    pub fn city(mut self, city: &str) -> AddressBuilder {
        self.city = Some(city.to_owned());
        self
    }

    /// Set the region (state/province) for the address.
    pub fn region(mut self, region: &str) -> AddressBuilder {
        self.region = Some(region.to_owned());
        self
    }

    /// Set the postal code (ZIP code) for the address.
    pub fn postal_code(mut self, postal_code: &str) -> AddressBuilder {
        self.postal_code = Some(postal_code.to_owned());
        self
    }

    /// Set the country using a `CountryCode` value (ISO 3166-1 alpha-3).
    ///
    /// Example: `CountryCode::GBR`.
    pub fn country(mut self, country_code: CountryCode) -> AddressBuilder {
        self.country_code = Some(country_code);
        self
    }

    /// Set the country using an ISO 3166-1 alpha-2 code string.
    ///
    /// Accepts a two-letter country code (for example, `"GB"`, `"IT"`). If the provided
    /// string cannot be parsed into a known country the builder's country remains `None`.
    /// This method does not return an error; validation occurs when `build()` is called.
    pub fn country_code(mut self, country_code: &str) -> AddressBuilder {
        self.country_code = CountryCode::for_alpha2(country_code).ok();
        self
    }

    /// Validate and build the `Address`.
    ///
    /// Returns `Ok(Address)` when all required fields are present. If a required field is
    /// missing, returns an `AddressBuilderError` indicating which field is missing.
    pub fn build(self) -> Result<Address, AddressBuilderError> {
        let street_address = self
            .street_address
            .ok_or(AddressBuilderError::MissingStreetAddress)?;
        let extended_address = self.extended_address;
        let city = self.city.ok_or(AddressBuilderError::MissingCity)?;
        let region = self.region;
        let postal_code = self
            .postal_code
            .ok_or(AddressBuilderError::MissingPostalCode)?;
        let country_code = self
            .country_code
            .ok_or(AddressBuilderError::MissingCountry)?;

        Ok(Address {
            street_address,
            extended_address,
            city,
            region,
            postal_code,
            country: country_code,
        })
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq, Copy, Clone)]
/// Errors returned when `AddressBuilder::build()` is called and a required field is missing.
pub enum AddressBuilderError {
    /// The street address was not provided.
    #[error("street address is required")]
    MissingStreetAddress,

    /// The city was not provided.
    #[error("city is required")]
    MissingCity,

    /// The postal code was not provided.
    #[error("postal code is required")]
    MissingPostalCode,

    /// The country was not provided.
    #[error("country is required")]
    MissingCountry,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod addresses {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;

        #[test]
        fn it_should_create_new_addresses() {
            let address = Address::builder()
                .street_address("22 acacia avenue")
                .extended_address("Apt. 999")
                .region("Essex")
                .city("London")
                .country(CountryCode::GBR)
                .postal_code("123456")
                .build()
                .unwrap();

            assert_eq!("22 acacia avenue", address.street_address());
            assert_eq!(Some("Apt. 999"), address.extended_address());
            assert_eq!(Some("Essex"), address.region());
            assert_eq!("London", address.city());
            assert_eq!(CountryCode::GBR, address.country_code());
            assert_eq!("123456", address.postal_code());
        }

        #[rstest]
        #[case(
            None,
            Some("postal_code"),
            Some("city"),
            Some(CountryCode::ITA),
            Err(AddressBuilderError::MissingStreetAddress)
        )]
        #[case(
            Some("street_address"),
            None,
            Some("city"),
            Some(CountryCode::ITA),
            Err(AddressBuilderError::MissingPostalCode)
        )]
        #[case(
            Some("street_address"),
            Some("postal_code"),
            None,
            Some(CountryCode::ITA),
            Err(AddressBuilderError::MissingCity)
        )]
        #[case(
            Some("street_address"),
            Some("postal_code"),
            Some("city"),
            None,
            Err(AddressBuilderError::MissingCountry)
        )]
        fn it_should_validate_the_required_value(
            #[case] street_address: Option<&str>,
            #[case] postal_code: Option<&str>,
            #[case] city: Option<&str>,
            #[case] country: Option<CountryCode>,
            #[case] expected: Result<Address, AddressBuilderError>,
        ) {
            let mut address_builder = Address::builder();

            if let Some(street_address) = street_address {
                address_builder = address_builder.street_address(street_address);
            }
            if let Some(postal_code) = postal_code {
                address_builder = address_builder.postal_code(postal_code);
            }
            if let Some(city) = city {
                address_builder = address_builder.city(city);
            }
            if let Some(country) = country {
                address_builder = address_builder.country(country);
            }

            let result = address_builder.build();
            assert_eq!(expected, result);
        }
    }
}
