//! Mapping functions between SQLx row structs and domain/view types.
//!
//! `from_row_*` functions convert a raw database row into either a domain
//! aggregate or a read-optimized view model.

use crate::trains::infrastructure::entities::{
    FormationCategoryRow, FormationElementDetailRow, PrototypeRow, TrainFormationRow,
    TrainFormationSummaryRow,
};

// ── View model structs (specta-typed read DTOs) ───────────────────────────────

/// Post-write response for `create_train_formation` and `update_train_formation`.
#[derive(Debug, Clone, serde::Serialize, specta::Type)]
pub struct TrainFormationView {
    pub id: String,
    pub name: String,
    pub category: Option<FormationCategoryView>,
    pub start_year: Option<i32>,
    pub end_year: Option<i32>,
    pub epoch: Option<String>,
    pub notes: Option<String>,
    pub element_count: i64,
    pub has_traction: bool,
}

/// Summary card for the formation list page.
#[derive(Debug, Clone, serde::Serialize, specta::Type)]
pub struct TrainFormationSummary {
    pub id: String,
    pub name: String,
    pub category: Option<FormationCategoryView>,
    pub epoch: Option<String>,
    pub element_count: i64,
    /// Whether the formation has at least one effective traction slot.
    pub has_traction: bool,
    /// Elements that have an `owned_rolling_stock_id` assigned.
    pub owned_count: i64,
    /// Elements that do not have an `owned_rolling_stock_id` assigned.
    pub planned_count: i64,
}

/// Full detail for the formation builder screen.
#[derive(Debug, Clone, serde::Serialize, specta::Type)]
pub struct TrainFormationDetail {
    pub id: String,
    pub name: String,
    pub category: Option<FormationCategoryView>,
    pub start_year: Option<i32>,
    pub end_year: Option<i32>,
    pub epoch: Option<String>,
    pub notes: Option<String>,
    /// Ordered composition slots.
    pub elements: Vec<FormationElementView>,
    /// Whether the formation has at least one effective traction slot.
    pub has_traction: bool,
}

/// An individual element slot in a formation.
#[derive(Debug, Clone, serde::Serialize, specta::Type)]
pub struct FormationElementView {
    pub id: String,
    pub position_order: i32,
    pub prototype: PrototypeView,
    pub owned_rolling_stock_id: Option<String>,
    /// Snapshotted series code retained even after owned model deletion.
    pub snapshot_series_code: Option<String>,
    /// Snapshotted company name retained even after owned model deletion.
    pub snapshot_company_name: Option<String>,
    /// `true` when `snapshot_series_code` is set but `owned_rolling_stock_id` is `None`.
    pub stock_not_found: bool,
    /// Number of `owned_rolling_stocks` rows matching the same `prototype_id`.
    pub owned_count_for_prototype: i64,
    pub traction_override: i32,
    /// Derived: whether this slot counts as a traction source.
    pub is_traction_slot: bool,
}

/// Prototype in search results.
#[derive(Debug, Clone, serde::Serialize, specta::Type)]
pub struct PrototypeView {
    pub id: String,
    pub railway_company_id: String,
    pub company_name: String,
    pub series_code: String,
    pub car_type: String,
    pub service_level: Option<String>,
    pub category: String,
    pub is_motorized: bool,
    pub default_is_dummy: bool,
    pub is_custom: bool,
}

/// Prototypes grouped by railway company (for the search drawer).
#[derive(Debug, Clone, serde::Serialize, specta::Type)]
pub struct PrototypeGroupView {
    pub railway_company_id: String,
    pub company_name: String,
    pub prototypes: Vec<PrototypeView>,
}

/// A formation category (seeded or custom).
#[derive(Debug, Clone, serde::Serialize, specta::Type)]
pub struct FormationCategoryView {
    pub id: String,
    pub name: String,
    pub is_custom: bool,
}

// ── Mapping functions ─────────────────────────────────────────────────────────

/// Map a [`FormationCategoryRow`] to a [`FormationCategoryView`].
pub fn category_row_to_view(row: FormationCategoryRow) -> FormationCategoryView {
    FormationCategoryView {
        id: row.id,
        name: row.name,
        is_custom: row.is_custom != 0,
    }
}

/// Map a [`PrototypeRow`] to a [`PrototypeView`], given the company name.
pub fn prototype_row_to_view(row: PrototypeRow, company_name: String) -> PrototypeView {
    PrototypeView {
        id: row.id,
        railway_company_id: row.railway_company_id,
        company_name,
        series_code: row.series_code,
        car_type: row.car_type,
        service_level: row.service_level,
        category: row.category,
        is_motorized: row.is_motorized != 0,
        default_is_dummy: row.default_is_dummy != 0,
        is_custom: row.is_custom != 0,
    }
}

/// Compute whether a slot counts as providing traction.
///
/// Logic mirrors [`isTractionSlot`] on the frontend:
/// `(is_motorized && !default_is_dummy && override != -1) || override === 1`
pub fn is_traction_slot(
    is_motorized: bool,
    default_is_dummy: bool,
    traction_override: i32,
) -> bool {
    (is_motorized && !default_is_dummy && traction_override != -1) || traction_override == 1
}

/// Map a [`FormationElementDetailRow`] to a [`FormationElementView`].
pub fn element_detail_row_to_view(row: FormationElementDetailRow) -> FormationElementView {
    let is_motorized = row.proto_is_motorized != 0;
    let default_is_dummy = row.proto_default_is_dummy != 0;
    let traction_slot = is_traction_slot(is_motorized, default_is_dummy, row.traction_override);

    // FR-020: stock_not_found when there is a snapshot but the FK was cleared.
    let stock_not_found =
        row.snapshot_series_code.is_some() && row.owned_rolling_stock_id.is_none();

    FormationElementView {
        id: row.id,
        position_order: row.position_order,
        prototype: PrototypeView {
            id: row.prototype_id,
            railway_company_id: row.proto_railway_company_id,
            company_name: row.proto_company_name,
            series_code: row.proto_series_code,
            car_type: row.proto_car_type,
            service_level: row.proto_service_level,
            category: row.proto_category,
            is_motorized,
            default_is_dummy,
            is_custom: row.proto_is_custom != 0,
        },
        owned_rolling_stock_id: row.owned_rolling_stock_id,
        snapshot_series_code: row.snapshot_series_code,
        snapshot_company_name: row.snapshot_company_name,
        stock_not_found,
        owned_count_for_prototype: row.owned_count_for_prototype,
        traction_override: row.traction_override,
        is_traction_slot: traction_slot,
    }
}

/// Map a [`TrainFormationSummaryRow`] to a [`TrainFormationSummary`].
///
/// `has_traction` in the summary is computed from the element counts; for
/// the summary view we use the flag returned directly in the query.
pub fn summary_row_to_view(
    row: TrainFormationSummaryRow,
    category: Option<FormationCategoryView>,
    has_traction: bool,
) -> TrainFormationSummary {
    let owned_count = row.owned_count;
    let planned_count = row.element_count - row.owned_count;
    TrainFormationSummary {
        id: row.id,
        name: row.name,
        category,
        epoch: row.epoch,
        element_count: row.element_count,
        has_traction,
        owned_count,
        planned_count,
    }
}

/// Map a [`TrainFormationRow`] to a [`TrainFormationView`].
pub fn formation_row_to_view(
    row: TrainFormationRow,
    category: Option<FormationCategoryView>,
    element_count: i64,
    has_traction: bool,
) -> TrainFormationView {
    TrainFormationView {
        id: row.id,
        name: row.name,
        category,
        start_year: row.start_year,
        end_year: row.end_year,
        epoch: row.epoch,
        notes: row.notes,
        element_count,
        has_traction,
    }
}

#[cfg(test)]
mod tests {
    use super::is_traction_slot;

    #[test]
    fn coach_only_does_not_count_as_traction() {
        assert!(!is_traction_slot(false, false, 0));
    }

    #[test]
    fn locomotive_counts_as_traction_by_default() {
        assert!(is_traction_slot(true, false, 0));
    }

    #[test]
    fn power_car_counts_as_traction_by_default() {
        assert!(is_traction_slot(true, false, 0));
    }

    #[test]
    fn dummy_motorized_unit_is_excluded_by_default() {
        assert!(!is_traction_slot(true, true, 0));
    }

    #[test]
    fn override_can_force_include_a_non_motorized_unit() {
        assert!(is_traction_slot(false, false, 1));
    }

    #[test]
    fn override_can_force_exclude_a_locomotive() {
        assert!(!is_traction_slot(true, false, -1));
    }

    #[test]
    fn override_can_force_exclude_a_dummy_unit() {
        assert!(!is_traction_slot(true, true, -1));
    }

    #[test]
    fn empty_like_slot_configuration_stays_without_traction() {
        assert!(!is_traction_slot(false, false, 0));
    }
}
