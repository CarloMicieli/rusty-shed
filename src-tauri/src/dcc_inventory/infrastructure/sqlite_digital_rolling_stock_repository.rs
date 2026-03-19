use std::collections::HashMap;

use crate::catalog::domain::railway_model::{DccInterface, PowerMethod, RollingStockCategory};
use crate::catalog::domain::scale::Scale;
use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::dcc_inventory::application::{
    CheckDuplicateAddressResult, DecoderView, DigitalRollingStockView, DigitalSummary,
    InstallableRollingStockView,
};
use crate::dcc_inventory::domain::{
    DccAddress, DccInventoryUowExt, Decoder, DigitalRollingStock, DigitalRollingStockId,
    DigitalRollingStockRepository,
};
use crate::dcc_inventory::infrastructure::entities::{
    DecoderRow, DigitalRollingStockRow, ManufacturerNameRow,
};
use sqlx::SqliteConnection;

/// SQLite implementation of the `DigitalRollingStockRepository`.
pub struct SqliteDigitalRollingStockRepository<'conn> {
    executor: &'conn mut SqliteConnection,
}

impl<'conn> SqliteDigitalRollingStockRepository<'conn> {
    /// Create a new `SqliteDigitalRollingStockRepository` with the given executor.
    ///
    /// # Parameters
    /// - `executor`: Mutable reference to a `SqliteConnection` to execute queries against.
    ///
    /// # Returns
    /// - New instance of `SqliteDigitalRollingStockRepository`.
    pub fn new(executor: &'conn mut SqliteConnection) -> Self {
        Self { executor }
    }

    async fn select_by_id(
        &mut self,
        id: &DigitalRollingStockId,
    ) -> Result<Option<DigitalRollingStockRow>, DomainError> {
        let sql = r#"
            SELECT id, owned_rolling_stock_id, dcc_address, installed_decoder_id
            FROM digital_rolling_stocks
            WHERE id = ?1
            LIMIT 1
        "#;

        let row = sqlx::query_as::<_, DigitalRollingStockRow>(sql)
            .bind(id)
            .fetch_optional(&mut *self.executor)
            .await
            .map_err(DomainError::from)?;

        Ok(row)
    }
}

#[async_trait::async_trait]
impl<'conn> DigitalRollingStockRepository for SqliteDigitalRollingStockRepository<'conn> {
    async fn find_by_id(
        &mut self,
        id: &DigitalRollingStockId,
    ) -> Result<Option<DigitalRollingStock>, DomainError> {
        if let Some(r) = self.select_by_id(id).await? {
            // Map DB row to domain aggregate
            let dcc_address = DccAddress::new(r.dcc_address)
                .map_err(|e| DomainError::Validation(e.to_string()))?;

            let decoder_id = r.installed_decoder_id.ok_or_else(|| {
                DomainError::Validation("missing decoder for digital rolling stock".to_string())
            })?;

            Ok(Some(DigitalRollingStock::new(
                r.id,
                r.owned_rolling_stock_id,
                dcc_address,
                decoder_id,
            )))
        } else {
            Ok(None)
        }
    }

    async fn save(
        &mut self,
        digital_rolling_stock: DigitalRollingStock,
    ) -> Result<(), DomainError> {
        let sql = r#"
            INSERT INTO digital_rolling_stocks (id, owned_rolling_stock_id, dcc_address, installed_decoder_id)
            VALUES (?1, ?2, ?3, ?4)
            ON CONFLICT(id) DO UPDATE SET
              owned_rolling_stock_id = excluded.owned_rolling_stock_id,
              dcc_address = excluded.dcc_address,
              installed_decoder_id = excluded.installed_decoder_id
        "#;

        sqlx::query(sql)
            .bind(&digital_rolling_stock.id)
            .bind(&digital_rolling_stock.owned_rolling_stock_id)
            .bind(*digital_rolling_stock.dcc_address)
            .bind(digital_rolling_stock.decoder_id)
            .execute(&mut *self.executor)
            .await
            .map_err(DomainError::from)?;

        Ok(())
    }

    async fn find_all_decoders(&mut self) -> Result<Vec<Decoder>, DomainError> {
        let sql = r#"
            SELECT id, manufacturer_id, product_code, decoder_type, protocol, decoder_interface
            FROM decoders
            ORDER BY id
        "#;

        let decoder_rows = sqlx::query_as::<_, DecoderRow>(sql)
            .fetch_all(&mut *self.executor)
            .await
            .map_err(DomainError::from)?;

        let mut out = Vec::with_capacity(decoder_rows.len());
        for r in decoder_rows {
            let decoder = Decoder {
                id: r.id,
                manufacturer_id: r.manufacturer_id,
                product_code: r.product_code,
                decoder_type: r.decoder_type,
                protocol: r.protocol,
                decoder_interface: r.decoder_interface,
            };
            out.push(decoder);
        }

        Ok(out)
    }

    async fn find_all_digital_rolling_stocks(
        &mut self,
    ) -> Result<Vec<DigitalRollingStockView>, DomainError> {
        // Load all decoders and manufacturers once and build lookup maps to avoid N+1 queries.
        let decoders = self.find_all_decoders().await?;
        let mut decoder_map = HashMap::with_capacity(decoders.len());
        for d in decoders {
            decoder_map.insert(d.id.clone(), d);
        }

        let msql = r#"SELECT id, name FROM manufacturers"#;
        let manufacturer_rows = sqlx::query_as::<_, ManufacturerNameRow>(msql)
            .fetch_all(&mut *self.executor)
            .await
            .map_err(DomainError::from)?;

        let mut manufacturer_map = HashMap::with_capacity(manufacturer_rows.len());
        for m in manufacturer_rows {
            manufacturer_map.insert(m.id, m.name);
        }

        // Enhanced query with catalog data JOIN and Function decoder exclusion
        let sql = r#"
            SELECT 
                drs.id,
                drs.owned_rolling_stock_id,
                drs.dcc_address,
                drs.installed_decoder_id,
                rs.category,
                rs.road_number,
                rs.series_code,
                rs.series AS description,
                rc.name AS railway_company_name,
                rm.scale,
                rm.power_method
            FROM digital_rolling_stocks drs
            JOIN decoders d ON drs.installed_decoder_id = d.id
            JOIN owned_rolling_stocks ors ON drs.owned_rolling_stock_id = ors.id
            LEFT JOIN rolling_stocks rs ON ors.rolling_stock_id = rs.id
            LEFT JOIN railway_companies rc ON rs.railway_company_id = rc.id
            LEFT JOIN railway_models rm ON rs.railway_model_id = rm.id
            WHERE d.decoder_type != 'FUNCTION'
            ORDER BY drs.dcc_address ASC
        "#;

        #[derive(sqlx::FromRow)]
        struct EnrichedRow {
            id: DigitalRollingStockId,
            owned_rolling_stock_id: crate::collecting::domain::OwnedRollingStockId,
            dcc_address: i32,
            installed_decoder_id: Option<crate::dcc_inventory::domain::DecoderId>,
            category: Option<String>,
            road_number: Option<String>,
            series_code: Option<String>,
            description: Option<String>,
            railway_company_name: Option<String>,
            scale: Option<String>,
            power_method: Option<String>,
        }

        let rows = sqlx::query_as::<_, EnrichedRow>(sql)
            .fetch_all(&mut *self.executor)
            .await
            .map_err(DomainError::from)?;

        let mut out = Vec::with_capacity(rows.len());

        for row in rows {
            let dcc = DccAddress::new(row.dcc_address.try_into().unwrap_or(1))
                .map_err(|e| DomainError::Validation(e.to_string()))?;

            let decoder_id = row.installed_decoder_id.ok_or_else(|| {
                DomainError::Validation("missing decoder for digital rolling stock".to_string())
            })?;

            let decoder = decoder_map
                .get(&decoder_id)
                .ok_or_else(|| DomainError::NotFound {
                    resource: "Decoder".to_string(),
                    identifier: decoder_id.to_string(),
                })?;

            let decoder_view = DecoderView {
                id: decoder.id.clone(),
                manufacturer: manufacturer_map
                    .get(&decoder.manufacturer_id)
                    .cloned()
                    .unwrap_or_else(|| decoder.manufacturer_id.to_string()),
                product_code: decoder.product_code.clone(),
                decoder_type: decoder.decoder_type.clone(),
                protocol: decoder.protocol.clone(),
                decoder_interface: decoder.decoder_interface,
            };

            // Parse catalog enums from strings
            let category = row
                .category
                .and_then(|c| c.parse::<RollingStockCategory>().ok())
                .unwrap_or(RollingStockCategory::Locomotive);

            let scale = row.scale.as_deref().and_then(|s| Scale::try_from(s).ok());
            let power_method = row
                .power_method
                .as_deref()
                .and_then(|p| PowerMethod::try_from(p).ok());

            out.push(DigitalRollingStockView {
                id: row.id,
                owned_rolling_stock_id: row.owned_rolling_stock_id,
                dcc_address: dcc,
                decoder: decoder_view,
                category,
                railway_company_name: row.railway_company_name,
                scale,
                power_method,
                road_number: row.road_number,
                series_code: row.series_code,
                description: row.description,
            });
        }

        Ok(out)
    }

    async fn get_digital_summary(&mut self) -> Result<DigitalSummary, DomainError> {
        let sql = r#"
            SELECT
                COALESCE(SUM(CASE WHEN rs.is_dummy = 0 OR rs.is_dummy IS NULL THEN 1 ELSE 0 END), 0) as total_non_dummy,
                COALESCE(SUM(
                    CASE
                        WHEN (rs.is_dummy = 0 OR rs.is_dummy IS NULL)
                        AND (rs.control IN ('DCC_SOUND', 'DCC_FITTED') OR drs.id IS NOT NULL)
                        THEN 1
                        ELSE 0
                    END
                ), 0) as digital_count
            FROM owned_rolling_stocks ors
            LEFT JOIN rolling_stocks rs ON ors.rolling_stock_id = rs.id
            LEFT JOIN digital_rolling_stocks drs ON drs.owned_rolling_stock_id = ors.id
            JOIN collection_items ci ON ors.collection_item_id = ci.id
            WHERE ci.removed_date IS NULL
        "#;

        #[derive(sqlx::FromRow)]
        struct SummaryRow {
            total_non_dummy: i64,
            digital_count: i64,
        }

        let row = sqlx::query_as::<_, SummaryRow>(sql)
            .fetch_one(&mut *self.executor)
            .await
            .map_err(DomainError::from)?;

        let total_non_dummy = row.total_non_dummy as u32;
        let digital_count = row.digital_count as u32;
        let percentage = if total_non_dummy > 0 {
            (digital_count as f32 / total_non_dummy as f32) * 100.0
        } else {
            0.0
        };

        Ok(DigitalSummary {
            total_non_dummy,
            digital_count,
            percentage,
        })
    }

    async fn check_address_exists(
        &mut self,
        address: DccAddress,
        exclude_id: Option<DigitalRollingStockId>,
    ) -> Result<CheckDuplicateAddressResult, DomainError> {
        let sql = r#"
            SELECT id
            FROM digital_rolling_stocks
            WHERE dcc_address = ?1
            AND id != COALESCE(?2, '')
            LIMIT 1
        "#;

        let exclude_id_str = exclude_id
            .as_ref()
            .map(|id| id.to_string())
            .unwrap_or_default();

        let row: Option<(DigitalRollingStockId,)> = sqlx::query_as(sql)
            .bind(*address)
            .bind(&exclude_id_str)
            .fetch_optional(&mut *self.executor)
            .await
            .map_err(DomainError::from)?;

        Ok(CheckDuplicateAddressResult {
            is_duplicate: row.is_some(),
            existing_rolling_stock_id: row.map(|(id,)| id),
        })
    }

    async fn find_installable_rolling_stocks(
        &mut self,
    ) -> Result<Vec<InstallableRollingStockView>, DomainError> {
        let sql = r#"
            SELECT
                ors.id AS owned_rolling_stock_id,
                rs.category,
                rs.road_number,
                rs.series_code,
                rc.name AS railway_company_name,
                CASE WHEN drs.id IS NOT NULL THEN 1 ELSE 0 END AS has_decoder,
                rs.dcc_interface
            FROM owned_rolling_stocks ors
            LEFT JOIN rolling_stocks rs ON ors.rolling_stock_id = rs.id
            LEFT JOIN railway_companies rc ON rs.railway_company_id = rc.id
            LEFT JOIN digital_rolling_stocks drs ON drs.owned_rolling_stock_id = ors.id
            JOIN collection_items ci ON ors.collection_item_id = ci.id
            WHERE ci.removed_date IS NULL
            AND (rs.is_dummy = 0 OR rs.is_dummy IS NULL)
            ORDER BY rs.road_number ASC, ors.id ASC
        "#;

        #[derive(sqlx::FromRow)]
        struct InstallableRow {
            owned_rolling_stock_id: crate::collecting::domain::OwnedRollingStockId,
            category: Option<String>,
            road_number: Option<String>,
            series_code: Option<String>,
            railway_company_name: Option<String>,
            has_decoder: i32,
            dcc_interface: Option<DccInterface>,
        }

        let rows = sqlx::query_as::<_, InstallableRow>(sql)
            .fetch_all(&mut *self.executor)
            .await
            .map_err(DomainError::from)?;

        let mut out = Vec::with_capacity(rows.len());

        for row in rows {
            let category = row
                .category
                .and_then(|c| c.parse::<RollingStockCategory>().ok())
                .unwrap_or(RollingStockCategory::Locomotive);

            out.push(InstallableRollingStockView {
                owned_rolling_stock_id: row.owned_rolling_stock_id,
                category,
                railway_company_name: row.railway_company_name,
                road_number: row.road_number,
                series_code: row.series_code,
                has_decoder: row.has_decoder != 0,
                dcc_interface: row.dcc_interface,
            });
        }

        Ok(out)
    }
}

impl<'conn> DccInventoryUowExt for SqliteUnitOfWork<'conn> {
    fn digital_rolling_stocks_repository(&mut self) -> Box<dyn DigitalRollingStockRepository + '_> {
        Box::new(SqliteDigitalRollingStockRepository::new(&mut self.tx))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dcc_inventory::domain::DigitalRollingStockId;
    use sqlx::SqlitePool;

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_dcc_inventory.sql")
    )]
    async fn it_should_find_digital_rolling_stock_and_decoders(pool: SqlitePool) {
        let mut conn = pool.acquire().await.expect("should acquire connection");

        let mut repo = SqliteDigitalRollingStockRepository::new(&mut conn);

        let id = DigitalRollingStockId::try_from(
            "trn:digital-rolling-stock:00000000-0000-0000-0000-000000000001",
        )
        .unwrap();

        let res = repo.find_by_id(&id).await.expect("query should run");
        assert!(res.is_some());

        let drs = res.unwrap();
        assert_eq!(drs.id, id);

        let views = repo
            .find_all_digital_rolling_stocks()
            .await
            .expect("views query");
        assert!(!views.is_empty());
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_dcc_inventory.sql")
    )]
    async fn it_should_save_digital_rolling_stock(pool: SqlitePool) {
        let mut conn = pool.acquire().await.expect("should acquire connection");

        let mut repo = SqliteDigitalRollingStockRepository::new(&mut conn);

        let id = DigitalRollingStockId::try_from(
            "trn:digital-rolling-stock:00000000-0000-0000-0000-000000000001",
        )
        .unwrap();

        // Load existing aggregate, then re-save it to exercise the `save` path.
        let drs = repo
            .find_by_id(&id)
            .await
            .expect("query should run")
            .expect("should exist");

        // Call save (consumes the aggregate) — should upsert without error.
        repo.save(drs).await.expect("save should succeed");

        // Verify it can still be loaded afterwards.
        let res = repo.find_by_id(&id).await.expect("query should run");
        assert!(res.is_some());
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_dcc_inventory.sql")
    )]
    async fn it_should_find_all_decoders(pool: SqlitePool) {
        let mut conn = pool.acquire().await.expect("should acquire connection");

        let mut repo = SqliteDigitalRollingStockRepository::new(&mut conn);

        let decoders = repo.find_all_decoders().await.expect("decoders query");
        assert!(!decoders.is_empty());

        // Basic shape assertions for the first decoder
        let d = &decoders[0];
        assert!(!d.id.to_string().is_empty());
        assert!(!d.manufacturer_id.to_string().is_empty());
    }
}
