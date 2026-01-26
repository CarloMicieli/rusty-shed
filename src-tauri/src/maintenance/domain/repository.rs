use crate::collecting::domain::OwnedRollingStockId;
use crate::core::domain::domain_error::DomainError;
use crate::maintenance::domain::MaintenanceCard;
use async_trait::async_trait;

/// Repository abstraction for maintenance operations.
///
/// The `MaintenanceRepository` trait defines the persistence contract for the
/// maintenance bounded context. Implementations are responsible for:
/// - mapping persistence rows into domain types (`MaintenanceCard`,
///   `MaintenanceEvent`) before returning to callers,
/// - performing any multi-statement changes that must be atomic inside a
///   single database transaction, and
/// - returning rich `DomainError` values so the application layer can decide
///   how to present failures to the user.
///
/// Important guidelines for implementors:
/// - Methods return `Result<_, DomainError>`; implementations should wrap
///   lower-level SQL errors in the `DomainError::Infrastructure` variant and
///   return `DomainError::Validation` for mapping/validation failures.
/// - Do not leak infrastructure-specific types (e.g. DB row structs) to
///   callers — always return domain types. The repository is the layer that
///   performs infra -> domain translation.
/// - Where operations require atomicity (for example, inserting a
///   `maintenance_events` row and updating the `maintenance_cards` projection),
///   they must be executed inside a single transaction. The `SqliteMaintenanceRepository`
///   demonstrates this pattern via the `save` method which consumes pending
///   domain events and persists them transactionally.
/// - Date-only semantics are expected for maintenance scheduling: compare
///   `NaiveDate` values (YYYY-MM-DD) rather than full datetimes when
///   computing due/overdue state.
///
/// Concurrency & lifecycle:
/// - Repository instances are typically short-lived and bound to a database
///   connection or transaction. Acquire a repository from a Unit of Work for a
///   single logical operation and avoid reusing it across unrelated work.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait MaintenanceRepository {
    /// Find the maintenance card by `OwnedRollingStockId`.
    ///
    /// Returns the domain `MaintenanceCard` when present. Implementations
    /// must map persistence rows into the domain model before returning and
    /// should not expose infrastructure types to callers.
    async fn find_by_rolling_stock_id(
        &mut self,
        owned_rolling_stock_id: &OwnedRollingStockId,
    ) -> Result<Option<MaintenanceCard>, DomainError>;

    /// Find a maintenance card by its `MaintenanceCardId`.
    ///
    /// Implementations should accept the strongly-typed `MaintenanceCardId`
    /// (a TRN wrapper) and return the corresponding domain `MaintenanceCard`
    /// if present. Mapping or parse errors should be reported using
    /// `DomainError::Validation` and infrastructure failures using
    /// `DomainError::Infrastructure`.
    async fn find_by_id(
        &mut self,
        id: &crate::maintenance::domain::MaintenanceCardId,
    ) -> Result<Option<MaintenanceCard>, DomainError>;

    /// Persist changes for a maintenance card.
    ///
    /// The implementation SHOULD consume any pending events present on the
    /// provided `MaintenanceCard` and perform the corresponding persistence
    /// operations (insert into `maintenance_events` and update the
    /// `maintenance_cards` projection) inside a single transaction so the
    /// system remains consistent. The repository is responsible for translating
    /// domain events into persistence-side rows and updating projections.
    async fn save(&mut self, maintenance_card: MaintenanceCard) -> Result<(), DomainError>;

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
    /// Returned values are domain `MaintenanceCard` instances that match the
    /// due/overdue criteria. Any database or mapping errors should be returned
    /// as a `DomainError` so callers can react appropriately.
    async fn list_due_cards(&mut self) -> Result<Vec<MaintenanceCard>, DomainError>;
}

/// Extension trait to attach the maintenance repository to the Unit of Work.
pub trait MaintenanceUowExt: Send {
    /// Returns a boxed maintenance repository tied to the Unit of Work's transaction.
    fn maintenance_repository(&mut self) -> Box<dyn MaintenanceRepository + Send + '_>;
}
