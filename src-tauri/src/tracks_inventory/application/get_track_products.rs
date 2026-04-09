//! Query to fetch all track products for selection in forms.

use crate::core::domain::domain_error::DomainError;
use crate::core::domain::length::Length;
use crate::core::domain::measure_units::MeasureUnit;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::tracks_inventory::application::views::TrackProductView;
use crate::tracks_inventory::domain::{TrackCode, TrackId, TrackType};
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

/// Query to fetch all track products.
pub struct GetTrackProductsQuery;

impl GetTrackProductsQuery {
    /// Execute the query to get all track products.
    ///
    /// # Arguments
    /// - `unit_of_work`: Transactional unit providing repository access.
    ///
    /// # Returns
    /// * `Vec<TrackProductView>` - List of all track products.
    /// * `DomainError` - On database error.
    pub async fn execute(
        unit_of_work: &mut SqliteUnitOfWork,
    ) -> Result<Vec<TrackProductView>, DomainError> {
        let sql = r#"
            SELECT 
                tp.track_id,
                tp.product_code,
                tp.description,
                tp.track_type,
                tp.track_code,
                tp.with_roadbed,
                tp.length_mm,
                tp.radius_mm,
                m.name as manufacturer_name
            FROM track_products tp
            INNER JOIN manufacturers m ON tp.manufacturer_id = m.id
            ORDER BY m.name, tp.product_code
        "#;

        let rows: Vec<TrackProductRow> = sqlx::query_as(sql)
            .fetch_all(&mut *unit_of_work.tx)
            .await
            .map_err(DomainError::from)?;

        let views = rows
            .into_iter()
            .map(|row| TrackProductView {
                track_id: row.track_id,
                manufacturer_name: row.manufacturer_name,
                product_code: row.product_code,
                description: row.description.unwrap_or_default(),
                track_type: row
                    .track_type
                    .and_then(|t| t.parse::<TrackType>().ok())
                    .unwrap_or(TrackType::Straight),
                track_code: row.track_code.unwrap_or(TrackCode::Code83),
                with_roadbed: row.with_roadbed == 1,
                length: row.length_mm.and_then(|mm| {
                    Length::try_new(Decimal::from_i32(mm).unwrap(), MeasureUnit::Millimeters).ok()
                }),
                radius: row.radius_mm.and_then(|mm| {
                    Length::try_new(Decimal::from_i32(mm).unwrap(), MeasureUnit::Millimeters).ok()
                }),
            })
            .collect();

        Ok(views)
    }
}

#[derive(Debug, sqlx::FromRow)]
struct TrackProductRow {
    track_id: TrackId,
    product_code: String,
    description: Option<String>,
    track_type: Option<String>,
    track_code: Option<TrackCode>,
    with_roadbed: i64,
    length_mm: Option<i32>,
    radius_mm: Option<i32>,
    manufacturer_name: String,
}
