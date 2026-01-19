use crate::catalog::domain::manufacturer::ManufacturerId;
use crate::core::domain::domain_error::DomainError;
use crate::core::domain::length::Length;
use crate::core::domain::measure_units::MeasureUnit;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::tracks_inventory::domain::{
    TrackCode, TrackId, TrackProduct, TrackProductRepository, TrackProductUowExt,
};
use crate::tracks_inventory::infrastructure::entities::TrackProductRow;
use rust_decimal::Decimal;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use sqlx::SqliteConnection;

pub struct SqliteTrackProductRepository<'conn> {
    executor: &'conn mut SqliteConnection,
}

impl<'conn> SqliteTrackProductRepository<'conn> {
    pub fn new(executor: &'conn mut SqliteConnection) -> Self {
        Self { executor }
    }

    fn mm_to_length(v: Option<i32>) -> Option<Length> {
        v.map(|mm| {
            let d = Decimal::from_i64(mm as i64).unwrap_or(Decimal::ZERO);
            // safe: mm >= 0 expected
            Length::try_new(d, MeasureUnit::Millimeters).unwrap_or_default()
        })
    }

    async fn select_by_track_id(
        &mut self,
        id: &TrackId,
    ) -> Result<Option<TrackProduct>, DomainError> {
        let sql = r#"
            SELECT track_id, product_code, manufacturer_id, with_roadbed, length_mm, radius_mm, track_code, description
            FROM track_products
            WHERE track_id = ?1
            LIMIT 1
        "#;

        let row: Option<TrackProductRow> = sqlx::query_as(sql)
            .bind(id)
            .fetch_optional(&mut *self.executor)
            .await
            .map_err(DomainError::from)?;

        match row {
            Some(track_product_row) => {
                let track_product = TrackProduct {
                    track_id: track_product_row.track_id,
                    product_code: track_product_row.product_code,
                    manufacturer_id: ManufacturerId::try_from(track_product_row.manufacturer_id)
                        .map_err(|e| DomainError::Validation(e.to_string()))?,
                    description: track_product_row.description.unwrap_or_default(),
                    with_roadbed: track_product_row.with_roadbed == 1,
                    length: Self::mm_to_length(track_product_row.length_mm),
                    radius: Self::mm_to_length(track_product_row.radius_mm),
                    track_code: track_product_row.track_code.unwrap_or(TrackCode::Code83),
                    metadata: Default::default(),
                };

                Ok(Some(track_product))
            }
            None => Ok(None),
        }
    }
}

#[async_trait::async_trait]
impl<'conn> TrackProductRepository for SqliteTrackProductRepository<'conn> {
    async fn find_by_id(&mut self, id: &TrackId) -> Result<Option<TrackProduct>, DomainError> {
        self.select_by_track_id(id).await
    }

    async fn find_by_product_code(
        &mut self,
        manufacturer_id: &ManufacturerId,
        product_code: &str,
    ) -> Result<Option<TrackProduct>, DomainError> {
        let sql = r#"
            SELECT track_id, product_code, manufacturer_id, with_roadbed, length_mm, radius_mm, track_code
            FROM track_products
            WHERE manufacturer_id = ?1 AND product_code = ?2
            LIMIT 1
        "#;

        let row: Option<TrackProductRow> = sqlx::query_as(sql)
            .bind(manufacturer_id)
            .bind(product_code)
            .fetch_optional(&mut *self.executor)
            .await
            .map_err(DomainError::from)?;

        if let Some(track_product_row) = row {
            let track_id = track_product_row.track_id;
            return self.select_by_track_id(&track_id).await;
        }

        Ok(None)
    }

    async fn save(&mut self, track: TrackProduct) -> Result<(), DomainError> {
        let sql = r#"
            INSERT OR REPLACE INTO track_products (
                id, track_id, manufacturer_id, product_code, with_roadbed, length_mm, 
                radius_mm, track_code, created_at, updated_at, version)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 0)
        "#;

        sqlx::query(sql)
            .bind(&track.track_id)
            .bind(&track.track_id)
            .bind(&track.manufacturer_id)
            .bind(track.product_code)
            .bind(if track.with_roadbed { 1 } else { 0 })
            .bind(track.length.map(|l| l.quantity().to_i32().unwrap_or(0)))
            .bind(track.radius.map(|r| r.quantity().to_i32().unwrap_or(0)))
            .bind(track.track_code)
            .execute(&mut *self.executor)
            .await
            .map_err(DomainError::from)?;

        Ok(())
    }
}

impl<'conn> TrackProductUowExt for SqliteUnitOfWork<'conn> {
    fn track_products_repo(&mut self) -> Box<dyn TrackProductRepository + '_> {
        Box::new(SqliteTrackProductRepository::new(&mut self.tx))
    }
}
