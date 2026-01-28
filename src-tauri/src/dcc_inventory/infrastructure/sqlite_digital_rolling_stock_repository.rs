use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::dcc_inventory::application::DigitalRollingStockView;
use crate::dcc_inventory::domain::{
    DccAddress, DccInventoryUowExt, Decoder, DigitalRollingStock, DigitalRollingStockId,
    DigitalRollingStockRepository,
};
use crate::dcc_inventory::infrastructure::entities::{DecoderRow, DigitalRollingStockRow};
use sqlx::SqliteConnection;

/// SQLite implementation of the `DigitalRollingStockRepository`.
pub struct SqliteDigitalRollingStockRepository<'conn> {
    executor: &'conn mut SqliteConnection,
}

impl<'conn> SqliteDigitalRollingStockRepository<'conn> {
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
            let addr = DccAddress::new(r.dcc_address as u16)
                .map_err(|e| DomainError::Validation(e.to_string()))?;

            let decoder = r.installed_decoder_id.ok_or_else(|| {
                DomainError::Validation("missing decoder for digital rolling stock".to_string())
            })?;

            Ok(Some(DigitalRollingStock::new(
                r.id,
                r.owned_rolling_stock_id,
                addr,
                decoder,
            )))
        } else {
            Ok(None)
        }
    }

    async fn save(&mut self, drs: DigitalRollingStock) -> Result<(), DomainError> {
        let sql = r#"
            INSERT INTO digital_rolling_stocks (id, owned_rolling_stock_id, dcc_address, installed_decoder_id)
            VALUES (?1, ?2, ?3, ?4)
            ON CONFLICT(id) DO UPDATE SET
              owned_rolling_stock_id = excluded.owned_rolling_stock_id,
              dcc_address = excluded.dcc_address,
              installed_decoder_id = excluded.installed_decoder_id
        "#;

        sqlx::query(sql)
            .bind(&drs.id)
            .bind(&drs.owned_rolling_stock_id)
            .bind(drs.dcc_address.value() as i64)
            .bind(drs.decoder_id)
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

        let rows = sqlx::query_as::<_, DecoderRow>(sql)
            .fetch_all(&mut *self.executor)
            .await
            .map_err(DomainError::from)?;

        let mut out = Vec::with_capacity(rows.len());
        for r in rows {
            let decoder = Decoder {
                id: r.id,
                manufacturer_id: r.manufacturer_id,
                product_code: r.product_code.unwrap_or_default(),
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
        let sql = r#"
            SELECT id, owned_rolling_stock_id, dcc_address, installed_decoder_id
            FROM digital_rolling_stocks
            ORDER BY id
        "#;

        let rows = sqlx::query_as::<_, DigitalRollingStockRow>(sql)
            .fetch_all(&mut *self.executor)
            .await
            .map_err(DomainError::from)?;

        let mut out = Vec::with_capacity(rows.len());

        for r in rows {
            let dcc = DccAddress::new(r.dcc_address as u16)
                .map_err(|e| DomainError::Validation(e.to_string()))?;

            let decoder = r.installed_decoder_id.ok_or_else(|| {
                DomainError::Validation("missing decoder for digital rolling stock".to_string())
            })?;

            out.push(DigitalRollingStockView {
                id: r.id,
                owned_rolling_stock_id: r.owned_rolling_stock_id,
                dcc_address: dcc,
                decoder_id: decoder,
            });
        }

        Ok(out)
    }
}

impl<'conn> DccInventoryUowExt for SqliteUnitOfWork<'conn> {
    fn digital_rolling_stocks_repo(
        &mut self,
    ) -> Box<dyn crate::dcc_inventory::domain::DigitalRollingStockRepository + '_> {
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
}
