use chrono::NaiveDateTime;

/// Row mapping for the `manufacturers` table.
///
/// Represents a single row returned from queries against the `manufacturers`
/// table. Field names correspond to the table columns.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ManufacturerRow {
    /// Primary identifier for the manufacturer (e.g. UUID or database ID).
    pub id: String,

    /// Human-friendly name of the manufacturer.
    pub name: String,

    /// Optional registered (legal) company name, when different from `name`.
    pub registered_company_name: Option<String>,

    /// Status of the manufacturer (for example: "active", "inactive").
    pub status: String,

    /// Optional ISO 3166-1 alpha-2 country code for the manufacturer's country.
    pub country_code: Option<String>,

    /// Timestamp when the row was created.
    pub created_at: NaiveDateTime,

    /// Timestamp when the row was last updated.
    pub updated_at: NaiveDateTime,
}

/// Row mapping for the `railway_companies` table.
///
/// Represents a single row returned from queries against the `railway_companies`
/// table. Field names correspond to the table columns.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct RailwayCompanyRow {
    /// Primary identifier for the railway company.
    pub id: String,

    /// Human-friendly name of the railway company.
    pub name: String,

    /// Optional registered (legal) company name, when different from `name`.
    pub registered_company_name: Option<String>,

    /// Optional ISO 3166-1 alpha-2 country code for the company's country.
    pub country_code: Option<String>,

    /// Optional status field (kept as a string in the DB).
    pub status: Option<String>,

    /// Date when the railway began operation (YYYY-MM-DD string).
    pub operating_since: Option<String>,

    /// Date when the railway ended operation (YYYY-MM-DD string).
    pub operating_until: Option<String>,

    /// Timestamp when the row was created.
    pub created_at: NaiveDateTime,

    /// Timestamp when the row was last updated.
    pub updated_at: NaiveDateTime,
}
