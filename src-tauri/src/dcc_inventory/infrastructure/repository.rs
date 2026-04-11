use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::dcc_inventory::application::{
    CheckDuplicateAddressResult, DigitalRollingStockView, DigitalSummary,
    InstallableRollingStockView,
};
use crate::dcc_inventory::domain::{
    DccAddress, DccInventoryUowExt, Decoder, DigitalRollingStock, DigitalRollingStockEvent,
    DigitalRollingStockId, DigitalRollingStockRepository,
};
use crate::dcc_inventory::infrastructure::{database, mappers};
use sqlx::SqliteConnection;

/// SQLite implementation of the [`DigitalRollingStockRepository`].
///
/// Each instance borrows a single [`SqliteConnection`] for its lifetime,
/// ensuring all operations within a unit of work share the same connection
/// (and therefore the same transaction).
pub struct SqliteDigitalRollingStockRepository<'conn> {
    executor: &'conn mut SqliteConnection,
}

impl<'conn> SqliteDigitalRollingStockRepository<'conn> {
    /// Create a new repository that executes queries against `executor`.
    ///
    /// # Parameters
    /// - `executor`: Mutable reference to an open [`SqliteConnection`].
    pub fn new(executor: &'conn mut SqliteConnection) -> Self {
        Self { executor }
    }

    /// Dispatch a single domain event to the correct SQL operation.
    ///
    /// The match is exhaustive so that adding a new event variant forces the
    /// compiler to require a corresponding persistence branch here.
    ///
    /// # Errors
    /// Returns [`DomainError`] if the underlying SQL call fails.
    async fn handle_event(
        &mut self,
        id: &DigitalRollingStockId,
        event: DigitalRollingStockEvent,
    ) -> Result<(), DomainError> {
        match event {
            DigitalRollingStockEvent::Created {
                owned_rolling_stock_id,
                dcc_address,
                decoder_id,
            } => {
                database::insert_digital_rolling_stock(
                    &mut *self.executor,
                    id,
                    &owned_rolling_stock_id,
                    *dcc_address,
                    decoder_id,
                )
                .await
                .map_err(DomainError::from)?;
            }
            DigitalRollingStockEvent::DecoderChanged { decoder_id } => {
                database::update_digital_rolling_stock_decoder(
                    &mut *self.executor,
                    id,
                    decoder_id,
                )
                .await
                .map_err(DomainError::from)?;
            }
            DigitalRollingStockEvent::DccAddressChanged { dcc_address } => {
                database::update_digital_rolling_stock_address(
                    &mut *self.executor,
                    id,
                    *dcc_address,
                )
                .await
                .map_err(DomainError::from)?;
            }
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl<'conn> DigitalRollingStockRepository for SqliteDigitalRollingStockRepository<'conn> {
    async fn find_by_id(
        &mut self,
        id: &DigitalRollingStockId,
    ) -> Result<Option<DigitalRollingStock>, DomainError> {
        let row = database::find_digital_rolling_stock_by_id(&mut *self.executor, id)
            .await
            .map_err(DomainError::from)?;

        // Transpose Option<Result<…>> into Result<Option<…>>
        row.map(mappers::map_digital_rolling_stock_row).transpose()
    }

    async fn save(
        &mut self,
        mut digital_rolling_stock: DigitalRollingStock,
    ) -> Result<(), DomainError> {
        // Clone required: `id` must be borrowed for each iteration of the loop
        // while `pull_events` mutably borrows the aggregate simultaneously.
        let id = digital_rolling_stock.id.clone();
        for ev in digital_rolling_stock.pull_events() {
            self.handle_event(&id, ev).await?;
        }
        Ok(())
    }

    async fn find_all_decoders(&mut self) -> Result<Vec<Decoder>, DomainError> {
        let rows = database::find_all_decoders(&mut *self.executor)
            .await
            .map_err(DomainError::from)?;

        Ok(rows.into_iter().map(mappers::map_decoder_row).collect())
    }

    async fn find_all_digital_rolling_stocks(
        &mut self,
    ) -> Result<Vec<DigitalRollingStockView>, DomainError> {
        let rows = database::find_all_digital_rolling_stocks_view(&mut *self.executor)
            .await
            .map_err(DomainError::from)?;

        rows.into_iter()
            .map(mappers::map_enriched_row_to_view)
            .collect()
    }

    async fn get_digital_summary(&mut self) -> Result<DigitalSummary, DomainError> {
        let row = database::get_digital_summary(&mut *self.executor)
            .await
            .map_err(DomainError::from)?;

        Ok(mappers::map_summary_row(row))
    }

    async fn check_address_exists(
        &mut self,
        address: DccAddress,
        exclude_id: Option<DigitalRollingStockId>,
    ) -> Result<CheckDuplicateAddressResult, DomainError> {
        let existing_id = database::check_address_exists(
            &mut *self.executor,
            *address,
            exclude_id.as_ref(),
        )
        .await
        .map_err(DomainError::from)?;

        Ok(CheckDuplicateAddressResult {
            is_duplicate: existing_id.is_some(),
            existing_rolling_stock_id: existing_id,
        })
    }

    async fn find_installable_rolling_stocks(
        &mut self,
    ) -> Result<Vec<InstallableRollingStockView>, DomainError> {
        let rows = database::find_installable_rolling_stocks(&mut *self.executor)
            .await
            .map_err(DomainError::from)?;

        Ok(rows.into_iter().map(mappers::map_installable_row).collect())
    }
}

impl DccInventoryUowExt for SqliteUnitOfWork {
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

        // Call save (consumes the aggregate) — should persist without error.
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
