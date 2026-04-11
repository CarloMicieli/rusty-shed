use crate::catalog::domain::railway_model::coupler_repository::{CouplerRepository, CouplerUowExt};
use crate::catalog::domain::railway_model::coupler_type::CouplerType;
use crate::catalog::domain::railway_model::coupler_type_id::CouplerTypeId;
use crate::catalog::domain::railway_model::coupling_socket::CouplingSocket;
use crate::collecting::domain::OwnedRollingStockId;
use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use sqlx::SqliteConnection;

/// SQLite-backed implementation of `CouplerRepository`.
pub struct SqliteCouplerRepository<'conn> {
    executor: &'conn mut SqliteConnection,
}

impl<'conn> SqliteCouplerRepository<'conn> {
    pub fn new(executor: &'conn mut SqliteConnection) -> Self {
        Self { executor }
    }
}

#[async_trait::async_trait]
impl CouplerRepository for SqliteCouplerRepository<'_> {
    async fn find_all(
        &mut self,
        socket: Option<CouplingSocket>,
    ) -> Result<Vec<CouplerType>, DomainError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            id: String,
            manufacturer: String,
            name: String,
            compatible_socket: CouplingSocket,
        }

        let rows = match socket {
            Some(s) => {
                let s_str = s.to_string();
                sqlx::query_as::<_, Row>(
                    "SELECT id, manufacturer, name, compatible_socket \
                     FROM coupler_types \
                     WHERE compatible_socket = ? \
                     ORDER BY manufacturer, name",
                )
                .bind(s_str)
                .fetch_all(&mut *self.executor)
                .await
            }
            None => {
                sqlx::query_as::<_, Row>(
                    "SELECT id, manufacturer, name, compatible_socket \
                     FROM coupler_types \
                     ORDER BY manufacturer, name",
                )
                .fetch_all(&mut *self.executor)
                .await
            }
        }
        .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        rows.into_iter()
            .map(|r| {
                CouplerTypeId::try_from(r.id.as_str())
                    .map_err(|e| DomainError::Validation(e.to_string()))
                    .map(|id| CouplerType {
                        id,
                        manufacturer: r.manufacturer,
                        name: r.name,
                        compatible_socket: r.compatible_socket,
                    })
            })
            .collect()
    }

    async fn get_current_coupler(
        &mut self,
        owned_rs_id: &OwnedRollingStockId,
    ) -> Result<Option<CouplerTypeId>, DomainError> {
        let row: Option<(String,)> =
            sqlx::query_as("SELECT current_coupler_id FROM owned_rolling_stocks WHERE id = ?")
                .bind(owned_rs_id.as_ref())
                .fetch_optional(&mut *self.executor)
                .await
                .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        row.and_then(|(id,)| {
            if id.is_empty() {
                None
            } else {
                CouplerTypeId::try_from(id.as_str()).ok()
            }
        })
        .map(Ok)
        .transpose()
    }

    async fn set_current_coupler(
        &mut self,
        owned_rs_id: &OwnedRollingStockId,
        coupler_id: Option<CouplerTypeId>,
    ) -> Result<(), DomainError> {
        sqlx::query("UPDATE owned_rolling_stocks SET current_coupler_id = ? WHERE id = ?")
            .bind(coupler_id.as_ref().map(|c| c.as_ref().to_owned()))
            .bind(owned_rs_id.as_ref())
            .execute(&mut *self.executor)
            .await
            .map(|_| ())
            .map_err(|e| DomainError::Infrastructure(e.to_string()))
    }
}

impl CouplerUowExt for SqliteUnitOfWork {
    fn coupler_repository(&mut self) -> Box<dyn CouplerRepository + '_> {
        Box::new(SqliteCouplerRepository::new(&mut self.tx))
    }
}
