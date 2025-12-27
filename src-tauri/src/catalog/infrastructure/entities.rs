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
