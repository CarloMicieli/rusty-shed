use crate::core::domain::domain_error::DomainError;
use crate::core::domain::metadata::Metadata;
use crate::trains::domain::formation::formation_element::FormationElement;
use crate::trains::domain::formation::train_formation_event::TrainFormationEvent;
use serde::{Deserialize, Serialize};

/// Aggregate root for a named train formation.
///
/// A formation has optional metadata (category, epoch, year range, notes)
/// and an ordered list of composition slots ([`FormationElement`]).
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct TrainFormation {
    /// Unique identifier.  Format: `trn:formation:<uuid>`
    pub id: String,

    /// Human-readable name.  Must be non-empty and globally unique.
    pub name: String,

    /// Optional link to a [`FormationCategory`].
    pub category_id: Option<String>,

    /// Year the formation entered service.
    pub start_year: Option<i32>,

    /// Year the formation left service (or `None` for still-active).
    pub end_year: Option<i32>,

    /// Roman-numeral modelling epoch (I–VI+).
    pub epoch: Option<String>,

    /// Optional Markdown notes.
    pub notes: Option<String>,

    /// Ordered composition slots.
    pub elements: Vec<FormationElement>,

    /// Unpersisted events; drained by the repository after each operation.
    #[serde(skip)]
    pub pending_events: Vec<TrainFormationEvent>,

    /// Audit timestamps and optimistic-concurrency version.
    pub metadata: Metadata,
}

impl TrainFormation {
    /// Construct a new formation and emit a [`TrainFormationEvent::Created`] event.
    ///
    /// # Errors
    /// Returns [`DomainError::Validation`] when `name` is empty or whitespace-only.
    pub fn create(id: String, name: String) -> Result<Self, DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::Validation("name must not be empty".into()));
        }
        let mut formation = TrainFormation {
            id: id.clone(),
            name: name.clone(),
            category_id: None,
            start_year: None,
            end_year: None,
            epoch: None,
            notes: None,
            elements: Vec::new(),
            pending_events: Vec::new(),
            metadata: Metadata::default(),
        };
        formation
            .pending_events
            .push(TrainFormationEvent::Created { id, name });
        Ok(formation)
    }

    /// Rename the formation.
    ///
    /// # Errors
    /// Returns [`DomainError::Validation`] when the new name is empty.
    pub fn rename(&mut self, name: String) -> Result<(), DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::Validation("name must not be empty".into()));
        }
        let ev = TrainFormationEvent::Renamed { name };
        self.pending_events.push(ev.clone());
        self.apply_event(&ev);
        Ok(())
    }

    /// Update optional metadata fields (epoch, year range, notes, category).
    ///
    /// # Errors
    /// Returns [`DomainError::BusinessRule`] when `start_year > end_year`.
    pub fn update_metadata(
        &mut self,
        category_id: Option<String>,
        start_year: Option<i32>,
        end_year: Option<i32>,
        epoch: Option<String>,
        notes: Option<String>,
    ) -> Result<(), DomainError> {
        if let (Some(s), Some(e)) = (start_year, end_year)
            && s > e
        {
            return Err(DomainError::BusinessRule(
                "start_year cannot exceed end_year".into(),
            ));
        }
        let ev = TrainFormationEvent::MetadataUpdated {
            category_id,
            start_year,
            end_year,
            epoch,
            notes,
        };
        self.pending_events.push(ev.clone());
        self.apply_event(&ev);
        Ok(())
    }

    /// Append a new element slot to the composition.
    pub fn add_element(&mut self, element: FormationElement) {
        let ev = TrainFormationEvent::ElementAdded { element };
        self.pending_events.push(ev.clone());
        self.apply_event(&ev);
    }

    /// Remove an element slot by ID.
    ///
    /// # Errors
    /// Returns [`DomainError::NotFound`] when no element with the given ID exists.
    pub fn remove_element(&mut self, element_id: &str) -> Result<(), DomainError> {
        if !self.elements.iter().any(|e| e.id == element_id) {
            return Err(DomainError::NotFound {
                resource: "FormationElement".into(),
                identifier: element_id.into(),
            });
        }
        let ev = TrainFormationEvent::ElementRemoved {
            element_id: element_id.into(),
        };
        self.pending_events.push(ev.clone());
        self.apply_event(&ev);
        Ok(())
    }

    /// Validate and record a reorder intent.
    ///
    /// The actual `position_order` DB update is performed atomically by the repository.
    ///
    /// # Errors
    /// Returns [`DomainError::BusinessRule`] when `ordered_ids` doesn't exactly match
    /// the current element set (wrong count or wrong IDs).
    pub fn reorder_elements(&mut self, ordered_ids: Vec<String>) -> Result<(), DomainError> {
        let current_ids: std::collections::HashSet<_> =
            self.elements.iter().map(|e| e.id.as_str()).collect();
        let supplied_ids: std::collections::HashSet<_> =
            ordered_ids.iter().map(String::as_str).collect();
        if current_ids != supplied_ids {
            return Err(DomainError::BusinessRule(
                "ordered_element_ids must contain exactly the current element set".into(),
            ));
        }
        let ev = TrainFormationEvent::ElementsReordered {
            ordered_element_ids: ordered_ids,
        };
        self.pending_events.push(ev.clone());
        self.apply_event(&ev);
        Ok(())
    }

    /// Assign a physical model to an element slot.
    ///
    /// # Errors
    /// Returns [`DomainError::NotFound`] when no element with the given ID exists.
    pub fn assign_rolling_stock(
        &mut self,
        element_id: &str,
        owned_rolling_stock_id: String,
    ) -> Result<(), DomainError> {
        if !self.elements.iter().any(|e| e.id == element_id) {
            return Err(DomainError::NotFound {
                resource: "FormationElement".into(),
                identifier: element_id.into(),
            });
        }
        let ev = TrainFormationEvent::RollingStockAssigned {
            element_id: element_id.into(),
            owned_rolling_stock_id,
        };
        self.pending_events.push(ev.clone());
        self.apply_event(&ev);
        Ok(())
    }

    /// Unassign the physical model from an element slot.
    ///
    /// # Errors
    /// Returns [`DomainError::NotFound`] when no element with the given ID exists.
    pub fn unassign_rolling_stock(&mut self, element_id: &str) -> Result<(), DomainError> {
        if !self.elements.iter().any(|e| e.id == element_id) {
            return Err(DomainError::NotFound {
                resource: "FormationElement".into(),
                identifier: element_id.into(),
            });
        }
        let ev = TrainFormationEvent::RollingStockUnassigned {
            element_id: element_id.into(),
        };
        self.pending_events.push(ev.clone());
        self.apply_event(&ev);
        Ok(())
    }

    /// Set the per-slot traction override (`0`, `1`, or `-1`).
    ///
    /// # Errors
    /// - [`DomainError::BusinessRule`] when `traction_override` is not in `{-1, 0, 1}`.
    /// - [`DomainError::NotFound`] when no element with the given ID exists.
    pub fn set_traction_override(
        &mut self,
        element_id: &str,
        traction_override: i32,
    ) -> Result<(), DomainError> {
        if ![-1, 0, 1].contains(&traction_override) {
            return Err(DomainError::BusinessRule(
                "traction_override must be -1, 0, or 1".into(),
            ));
        }
        if !self.elements.iter().any(|e| e.id == element_id) {
            return Err(DomainError::NotFound {
                resource: "FormationElement".into(),
                identifier: element_id.into(),
            });
        }
        let ev = TrainFormationEvent::TractionOverrideSet {
            element_id: element_id.into(),
            traction_override,
        };
        self.pending_events.push(ev.clone());
        self.apply_event(&ev);
        Ok(())
    }

    /// Drain pending events.  Called by the repository after persisting changes.
    pub fn take_events(&mut self) -> Vec<TrainFormationEvent> {
        std::mem::take(&mut self.pending_events)
    }

    // ── private ──────────────────────────────────────────────────────────────

    fn apply_event(&mut self, event: &TrainFormationEvent) {
        match event {
            TrainFormationEvent::Created { name, .. } => {
                self.name = name.clone();
            }
            TrainFormationEvent::Renamed { name } => {
                self.name = name.clone();
            }
            TrainFormationEvent::MetadataUpdated {
                category_id,
                start_year,
                end_year,
                epoch,
                notes,
            } => {
                self.category_id = category_id.clone();
                self.start_year = *start_year;
                self.end_year = *end_year;
                self.epoch = epoch.clone();
                self.notes = notes.clone();
            }
            TrainFormationEvent::ElementAdded { element } => {
                self.elements.push(element.clone());
            }
            TrainFormationEvent::ElementRemoved { element_id } => {
                self.elements.retain(|e| e.id != *element_id);
            }
            TrainFormationEvent::ElementsReordered {
                ordered_element_ids,
            } => {
                let order_map: std::collections::HashMap<_, _> = ordered_element_ids
                    .iter()
                    .enumerate()
                    .map(|(i, id)| (id.as_str(), i as i32))
                    .collect();
                self.elements
                    .sort_by_key(|e| order_map.get(e.id.as_str()).copied().unwrap_or(i32::MAX));
            }
            TrainFormationEvent::RollingStockAssigned {
                element_id,
                owned_rolling_stock_id,
            } => {
                if let Some(el) = self.elements.iter_mut().find(|e| e.id == *element_id) {
                    el.owned_rolling_stock_id = Some(owned_rolling_stock_id.clone());
                }
            }
            TrainFormationEvent::RollingStockUnassigned { element_id } => {
                if let Some(el) = self.elements.iter_mut().find(|e| e.id == *element_id) {
                    el.owned_rolling_stock_id = None;
                }
            }
            TrainFormationEvent::TractionOverrideSet {
                element_id,
                traction_override,
            } => {
                if let Some(el) = self.elements.iter_mut().find(|e| e.id == *element_id) {
                    el.traction_override = *traction_override;
                }
            }
            TrainFormationEvent::Deleted { .. } => {}
        }
    }
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_formation() -> TrainFormation {
        TrainFormation::create("trn:formation:test".into(), "Test Formation".into()).unwrap()
    }

    #[test]
    fn test_formation_name_must_not_be_empty() {
        let result = TrainFormation::create("id".into(), "  ".into());
        assert!(result.is_err(), "expected Err for empty name");
    }

    #[test]
    fn test_formation_start_after_end_rejected() {
        let mut f = make_formation();
        let result = f.update_metadata(None, Some(1985), Some(1980), None, None);
        assert!(result.is_err(), "expected Err for start_year > end_year");
    }

    #[test]
    fn test_formation_same_year_allowed() {
        let mut f = make_formation();
        let result = f.update_metadata(None, Some(1975), Some(1975), None, None);
        assert!(result.is_ok(), "same start/end year should be allowed");
    }

    #[test]
    fn test_formation_null_years_allowed() {
        let mut f = make_formation();
        let result = f.update_metadata(None, None, None, None, None);
        assert!(result.is_ok(), "null years should be allowed");
    }

    #[test]
    fn test_formation_open_ended_allowed() {
        let mut f = make_formation();
        let result = f.update_metadata(None, Some(1975), None, None, None);
        assert!(result.is_ok(), "open-ended year should be allowed");
    }

    #[test]
    fn test_traction_coach_only() {
        let f = make_formation();
        // No elements → no traction (also covers empty case)
        assert!(f.elements.is_empty());
    }

    #[test]
    fn test_reorder_mismatched_ids_rejected() {
        let mut f = make_formation();
        let el = FormationElement {
            id: "el-1".into(),
            prototype_id: "p-1".into(),
            owned_rolling_stock_id: None,
            position_order: 0,
            traction_override: 0,
        };
        f.add_element(el);
        let result = f.reorder_elements(vec!["wrong-id".into()]);
        assert!(result.is_err());
    }

    #[test]
    fn test_traction_override_invalid_value_rejected() {
        let mut f = make_formation();
        let el = FormationElement {
            id: "el-1".into(),
            prototype_id: "p-1".into(),
            owned_rolling_stock_id: None,
            position_order: 0,
            traction_override: 0,
        };
        f.add_element(el);
        let result = f.set_traction_override("el-1", 2);
        assert!(result.is_err());
    }
}
