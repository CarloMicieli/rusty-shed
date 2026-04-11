//! Repository trait and UoW extension for the trains bounded context.
//!
//! `TrainsRepository` is the abstract port the application layer programs
//! against.  The only implementation is `SqlxTrainFormationRepository` in
//! `infrastructure::train_formation_repo`.  During tests the trait is
//! auto-mocked via `mockall`.

use crate::core::domain::domain_error::DomainError;
use crate::trains::domain::formation::formation_element::FormationElement;
use crate::trains::domain::formation::train_formation::TrainFormation;
use crate::trains::domain::views::{
    FormationCategoryView, FormationElementView, PrototypeGroupView, PrototypeView,
    TrainFormationDetail, TrainFormationSummary, TrainFormationView,
};
use async_trait::async_trait;

// ---------------------------------------------------------------------------
// Input type for prototype creation (owned, mockall-compatible)
// ---------------------------------------------------------------------------

/// Owned input for creating a new prototype; replaces the lifetime-bound
/// `SavePrototypeParams<'a>` when crossing the application/infrastructure
/// boundary.
#[derive(Debug, Clone)]
pub struct CreatePrototypeInput {
    pub id: String,
    pub railway_company_id: String,
    pub series_code: String,
    pub friendly_name: Option<String>,
    pub is_motorized: bool,
    pub default_is_dummy: bool,
    pub notes: Option<String>,
    /// Specification discriminator: `LOCOMOTIVE` | `PASSENGER_CAR` | `FREIGHT_CAR` |
    /// `RAILCAR` | `ELECTRIC_MULTIPLE_UNIT`
    pub specification_type: String,
    pub locomotive_type: Option<String>,
    pub locomotive_series: Option<String>,
    pub service_level: Option<String>,
    pub passenger_car_type: Option<String>,
    pub freight_car_type: Option<String>,
    pub railcar_type: Option<String>,
    pub electric_multiple_unit_type: Option<String>,
    pub elements_count: Option<i64>,
    pub is_permanently_coupled: Option<bool>,
}

// ---------------------------------------------------------------------------
// Repository trait
// ---------------------------------------------------------------------------

/// Abstract repository for all train-formation persistence operations.
///
/// Method names are intentionally distinct from the concrete
/// `SqlxTrainFormationRepository` methods to avoid Rust name-resolution
/// ambiguity when the concrete type implements this trait.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait TrainsRepository: Send + Sync {
    // ── Formation CRUD ────────────────────────────────────────────────────

    /// Load a formation aggregate by its unique ID.
    ///
    /// Returns [`DomainError::NotFound`] when no matching record exists.
    async fn find_formation_by_id(&mut self, id: &str) -> Result<TrainFormation, DomainError>;

    /// Persist (upsert) a formation aggregate.
    async fn save_formation(&mut self, formation: &TrainFormation) -> Result<(), DomainError>;

    /// Delete a formation and cascade-delete all its elements.
    async fn delete_formation(&mut self, id: &str) -> Result<(), DomainError>;

    // ── Formation read views ──────────────────────────────────────────────

    /// Return a lightweight post-write view of a formation.
    async fn get_formation_view(&mut self, id: &str) -> Result<TrainFormationView, DomainError>;

    /// Return the full detail view (with elements) for a formation.
    async fn get_formation_detail(&mut self, id: &str)
    -> Result<TrainFormationDetail, DomainError>;

    /// Return summary cards for all formations.
    async fn get_all_formation_summaries(
        &mut self,
    ) -> Result<Vec<TrainFormationSummary>, DomainError>;

    // ── Element operations ────────────────────────────────────────────────

    /// Append a new element slot to a formation.
    ///
    /// Takes ownership of `element` so that mockall can generate a mock
    /// without lifetime parameters.
    async fn add_formation_element(
        &mut self,
        formation_id: &str,
        element: FormationElement,
    ) -> Result<(), DomainError>;

    /// Remove an element by ID and shift subsequent positions down.
    async fn remove_formation_element(&mut self, element_id: &str) -> Result<(), DomainError>;

    /// Bulk-reorder elements within a formation.
    ///
    /// `element_ids` must contain exactly all IDs currently in the formation.
    async fn reorder_formation_elements(
        &mut self,
        formation_id: &str,
        element_ids: Vec<String>,
    ) -> Result<(), DomainError>;

    /// Return the detailed view of a single element.
    async fn get_formation_element_view(
        &mut self,
        element_id: &str,
    ) -> Result<FormationElementView, DomainError>;

    /// Assign or unassign an owned rolling-stock model to an element.
    ///
    /// Pass `None` to unassign.
    async fn assign_rolling_stock_to_element(
        &mut self,
        element_id: &str,
        owned_rolling_stock_id: Option<String>,
    ) -> Result<FormationElementView, DomainError>;

    /// Override the traction status of an element.
    ///
    /// `0` = use prototype default, `1` = force include, `-1` = force exclude.
    async fn set_element_traction_override(
        &mut self,
        element_id: &str,
        traction_override: i32,
    ) -> Result<FormationElementView, DomainError>;

    // ── Prototype operations ──────────────────────────────────────────────

    /// Search prototypes by an optional free-text query, grouped by company.
    async fn find_prototypes_by_query(
        &mut self,
        query: Option<String>,
    ) -> Result<Vec<PrototypeGroupView>, DomainError>;

    /// Persist a new custom prototype and return its view.
    async fn create_prototype(
        &mut self,
        input: CreatePrototypeInput,
    ) -> Result<PrototypeView, DomainError>;

    // ── Category operations ───────────────────────────────────────────────

    /// Return all formation categories.
    async fn get_all_categories(&mut self) -> Result<Vec<FormationCategoryView>, DomainError>;

    /// Create a new custom formation category.
    async fn create_formation_category(
        &mut self,
        id: &str,
        name: &str,
    ) -> Result<FormationCategoryView, DomainError>;
}

// ---------------------------------------------------------------------------
// UoW extension trait
// ---------------------------------------------------------------------------

/// Extension trait that exposes a [`TrainsRepository`] through the Unit of Work.
pub trait TrainsUowExt: Send {
    /// Obtain a fresh trains repository bound to the current transaction.
    fn trains_repo(&mut self) -> Box<dyn TrainsRepository + '_>;
}
