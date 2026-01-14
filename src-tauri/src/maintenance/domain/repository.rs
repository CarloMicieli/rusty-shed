use crate::core::domain::domain_error::DomainError;
use crate::maintenance::infrastructure::entities::{MaintenanceCardRow, MaintenanceEventRow};
use crate::maintenance::infrastructure::repository::NewMaintenanceEvent;
use async_trait::async_trait;

/// Repository abstraction for maintenance operations.
///
/// This trait defines the persistence API for maintenance-related data.
/// Implementations are responsible for mapping database rows to the
/// corresponding `entities` types and for correctly handling transactional
/// semantics where required.
///
/// Key responsibilities and expectations:
/// - Provide CRUD-like accessors for maintenance cards and maintenance events.
/// - `record_event_transaction` MUST be implemented to perform the insert of a
///   maintenance event and the related update to the maintenance card atomically
///   (i.e. inside a single database transaction) so the system remains in a
///   consistent state.
/// - Date fields are treated as date-only values (YYYY-MM-DD). Any logic that
///   evaluates whether a card is due/overdue should compare dates without time
///   components (SQLite's `date('now')` is suitable for SQLite-based
///   implementations).
/// - Methods return `anyhow::Result` so SQL execution errors and mapping errors
///   can be propagated and enriched by callers. Implementations should avoid
///   swallowing errors and instead return informative failures.
///
/// Concurrency & lifecycle:
/// - Repositories are typically short-lived and bound to a specific database
///   connection or transaction (see `SqliteMaintenanceRepository`). Callers
///   should acquire a repository instance as part of a unit-of-work or
///   transaction scope and not reuse it across unrelated transactions.
///
/// Error handling:
/// - SQL errors, constraint violations, and mapping/parsing problems are
///   surfaced via the `anyhow::Error` value returned by methods. Callers can
///   inspect or log these errors as needed.
#[async_trait]
pub trait MaintenanceRepository {
    /// Fetch the maintenance card by owned rolling stock id.
    async fn get_card_by_stock_id(
        &mut self,
        owned_rolling_stock_id: &str,
    ) -> Result<Option<MaintenanceCardRow>, DomainError>;

    /// Record an event and update the maintenance card within the same transaction.
    async fn record_event_transaction(
        &mut self,
        new_event: NewMaintenanceEvent,
    ) -> Result<(), DomainError>;

    /// List events for a maintenance card.
    async fn list_events_for_card(
        &mut self,
        maintenance_card_id: &str,
    ) -> Result<Vec<MaintenanceEventRow>, DomainError>;

    /// List maintenance cards that are due or overdue.
    ///
    /// A card is considered "due" when either:
    /// - `next_maintenance_date` is present and is less than or equal to the
    ///   current local date (comparison performed using SQLite's `date('now')`),
    ///   or
    /// - `next_maintenance_date` is NULL and `last_maintenance_date` is present
    ///   and less than or equal to the current date (i.e. the card has been
    ///   maintained before but no next date was scheduled and it is now overdue).
    ///
    /// Returned rows are mapped to `MaintenanceCardRow`. Any database or mapping
    /// errors are returned as `anyhow::Error`.
    async fn list_due_cards(&mut self) -> Result<Vec<MaintenanceCardRow>, DomainError>;
}

/// Extension trait to attach the maintenance repository to the Unit of Work.
pub trait MaintenanceUowExt: Send {
    /// Returns a boxed maintenance repository tied to the Unit of Work's transaction.
    fn maintenance_repository(&mut self) -> Box<dyn MaintenanceRepository + Send + '_>;
}
