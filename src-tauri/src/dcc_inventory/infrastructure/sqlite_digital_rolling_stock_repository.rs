use std::collections::HashMap;

use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::dcc_inventory::application::{DecoderView, DigitalRollingStockView};
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

        let sql = r#"
            SELECT id, owned_rolling_stock_id, dcc_address, installed_decoder_id
            FROM digital_rolling_stocks
            ORDER BY id
        "#;

        let digital_rolling_stock_rows = sqlx::query_as::<_, DigitalRollingStockRow>(sql)
            .fetch_all(&mut *self.executor)
            .await
            .map_err(DomainError::from)?;

        let mut out = Vec::with_capacity(digital_rolling_stock_rows.len());

        for digital_rolling_stock_row in digital_rolling_stock_rows {
            let dcc = DccAddress::new(digital_rolling_stock_row.dcc_address)
                .map_err(|e| DomainError::Validation(e.to_string()))?;

            let decoder_id = digital_rolling_stock_row
                .installed_decoder_id
                .ok_or_else(|| {
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

            out.push(DigitalRollingStockView {
                id: digital_rolling_stock_row.id,
                owned_rolling_stock_id: digital_rolling_stock_row.owned_rolling_stock_id,
                dcc_address: dcc,
                decoder: decoder_view,
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
