use crate::catalog::domain::manufacturer::ManufacturerId;
use crate::catalog::domain::railway_company::RailwayCompanyId;
use crate::catalog::domain::railway_model::RailwayModelEvent;
use crate::catalog::domain::railway_model::RollingStockCategory;
use crate::catalog::domain::railway_model::localized_field::LocalizedField;
use crate::catalog::domain::railway_model::rolling_stock::{
    RollingStockDccPatch, RollingStockSpecPatch,
};
use crate::catalog::domain::railway_model::{
    AvailabilityStatus, Category, DeliveryDate, Epoch, PowerMethod, ProductCode, RailwayModelId,
    RollingStock,
};
use crate::catalog::domain::railway_model::{RollingStockId, RollingStockParams};
use crate::catalog::domain::scale::Scale;
use crate::core::domain::{Language, domain_error::DomainError};
use serde_json::json;
use uuid::Uuid;

/// A `RailwayModel` represents a manufactured model product in the catalog.
///
/// It contains metadata about the product (manufacturer, product code,
/// scale, epoch, etc.) and a list of `RollingStock` instances that correspond
/// to specific owned or catalogued items of this model.
#[derive(Debug, Clone)]
pub struct RailwayModel {
    /// Unique identifier for the railway model.
    pub id: RailwayModelId,

    /// Reference to the manufacturer id of the model (e.g. Bachmann, Märklin).
    pub manufacturer_id: ManufacturerId,

    /// Manufacturer-assigned product code.
    pub product_code: ProductCode,

    /// Human-readable description of the model (localized).
    pub description: LocalizedField,

    /// Additional details about the model, localized (e.g. special features, variations).
    pub details: Option<LocalizedField>,

    /// The power method used by this model (e.g. Diesel, Electric, None for non-powered models).
    pub power_method: PowerMethod,

    /// The scale of the model (e.g. HO, N).
    pub scale: Scale,

    /// The historical epoch the model belongs to.
    pub epoch: Epoch,

    /// Classification category for the model (e.g. locomotive, freight car).
    pub category: Category,

    /// Delivery or release date information for the product.
    pub delivery_date: Option<DeliveryDate>,

    /// the availability status
    pub availability_status: Option<AvailabilityStatus>,

    /// Rolling stock instances (specific vehicles) that correspond to this model.
    pub rolling_stocks: Vec<RollingStock>,

    /// Pending domain events produced by operations on this aggregate.
    pub pending_events: Vec<RailwayModelEvent>,
}

impl RailwayModel {
    /// Returns and clears pending events for this aggregate.
    pub fn pull_events(&mut self) -> Vec<RailwayModelEvent> {
        std::mem::take(&mut self.pending_events)
    }

    /// Push a domain event onto the pending events list.
    pub fn push_event(&mut self, ev: RailwayModelEvent) {
        self.pending_events.push(ev);
    }

    /// Upsert a translation for the given language and emit a `TranslationUpserted` event.
    ///
    /// At least one of `description` or `details` should be `Some` with a non-empty value;
    /// when both resolve to `None` the repository will delete the translation row.
    pub fn upsert_translation(
        &mut self,
        lang: Language,
        description: Option<String>,
        details: Option<String>,
    ) {
        let ev = RailwayModelEvent::TranslationUpserted {
            event_id: Uuid::new_v4(),
            railway_model_id: self.id.clone(),
            timestamp: chrono::Utc::now().naive_utc(),
            lang,
            description,
            details,
        };
        self.push_event(ev);
    }

    /// Update the description and emit a `TranslationUpserted` event for the aggregate's current language.
    ///
    /// Returns `Err(DomainError::Validation)` when `description` is empty after trimming.
    pub fn update_description(&mut self, description: String) -> Result<(), DomainError> {
        let trimmed = description.trim().to_string();
        if trimmed.is_empty() {
            return Err(DomainError::Validation(
                "description must not be empty".to_string(),
            ));
        }
        let lang = self.description.lang;
        self.description = LocalizedField {
            lang,
            value: trimmed.clone(),
        };
        self.upsert_translation(lang, Some(trimmed), None);
        Ok(())
    }

    /// Update details and emit a `TranslationUpserted` event for the aggregate's current language.
    pub fn update_details(&mut self, details: Option<String>) {
        let lang = self.description.lang;
        self.details = details.clone().map(|v| LocalizedField { lang, value: v });
        self.upsert_translation(lang, None, details);
    }

    /// Update scale and emit a RailwayModelUpdated event.
    pub fn update_scale(&mut self, scale: Scale) {
        self.scale = scale.clone();
        let changed = json!({ "scale": scale.as_code() });
        let ev = RailwayModelEvent::RailwayModelUpdated {
            event_id: uuid::Uuid::new_v4(),
            railway_model_id: self.id.clone(),
            timestamp: chrono::Utc::now().naive_utc(),
            changed,
        };
        self.push_event(ev);
    }

    /// Update epoch and emit a RailwayModelUpdated event.
    pub fn update_epoch(&mut self, epoch: Epoch) {
        self.epoch = epoch.clone();
        let changed = json!({ "epoch": epoch.0 });
        let ev = RailwayModelEvent::RailwayModelUpdated {
            event_id: uuid::Uuid::new_v4(),
            railway_model_id: self.id.clone(),
            timestamp: chrono::Utc::now().naive_utc(),
            changed,
        };
        self.push_event(ev);
    }

    /// Update the railway company of a rolling stock and emit a RollingStockUpdated event.
    ///
    /// Returns `Err(DomainError::NotFound)` when no rolling stock with `rolling_stock_id` exists.
    pub fn update_rolling_stock_railway_company(
        &mut self,
        rolling_stock_id: &RollingStockId,
        company_id: RailwayCompanyId,
    ) -> Result<(), DomainError> {
        let rs = self
            .rolling_stocks
            .iter_mut()
            .find(|rs| rs.id_as_ref() == rolling_stock_id)
            .ok_or_else(|| DomainError::NotFound {
                resource: "RollingStock".to_string(),
                identifier: rolling_stock_id.to_string(),
            })?;

        let changed = rs.apply_railway_company(company_id);

        let ev = RailwayModelEvent::RollingStockUpdated {
            event_id: Uuid::new_v4(),
            railway_model_id: self.id.clone(),
            rolling_stock_id: rolling_stock_id.clone(),
            timestamp: chrono::Utc::now().naive_utc(),
            changed,
        };
        self.push_event(ev);
        Ok(())
    }

    /// Change the category (variant) of a rolling stock and emit a RollingStockUpdated event.
    ///
    /// Returns `Err(DomainError::NotFound)` when no rolling stock with `rolling_stock_id` exists.
    pub fn update_rolling_stock_category(
        &mut self,
        rolling_stock_id: &RollingStockId,
        new_category: RollingStockCategory,
    ) -> Result<(), DomainError> {
        let rs = self
            .rolling_stocks
            .iter_mut()
            .find(|rs| rs.id_as_ref() == rolling_stock_id)
            .ok_or_else(|| DomainError::NotFound {
                resource: "RollingStock".to_string(),
                identifier: rolling_stock_id.to_string(),
            })?;

        let changed = rs.apply_category(new_category);

        let ev = RailwayModelEvent::RollingStockUpdated {
            event_id: Uuid::new_v4(),
            railway_model_id: self.id.clone(),
            rolling_stock_id: rolling_stock_id.clone(),
            timestamp: chrono::Utc::now().naive_utc(),
            changed,
        };
        self.push_event(ev);
        Ok(())
    }

    /// Change the subcategory (type field) of a rolling stock and emit a RollingStockUpdated event.
    ///
    /// Returns `Err(DomainError::NotFound)` when no rolling stock with `rolling_stock_id` exists.
    /// Returns `Err(DomainError::Validation)` when `subcategory` is invalid for the current category.
    pub fn update_rolling_stock_subcategory(
        &mut self,
        rolling_stock_id: &RollingStockId,
        subcategory: String,
    ) -> Result<(), DomainError> {
        let rs = self
            .rolling_stocks
            .iter_mut()
            .find(|rs| rs.id_as_ref() == rolling_stock_id)
            .ok_or_else(|| DomainError::NotFound {
                resource: "RollingStock".to_string(),
                identifier: rolling_stock_id.to_string(),
            })?;

        let changed = rs.apply_subcategory(subcategory)?;

        let ev = RailwayModelEvent::RollingStockUpdated {
            event_id: Uuid::new_v4(),
            railway_model_id: self.id.clone(),
            rolling_stock_id: rolling_stock_id.clone(),
            timestamp: chrono::Utc::now().naive_utc(),
            changed,
        };
        self.push_event(ev);
        Ok(())
    }

    /// Change the service level of a rolling stock and emit a RollingStockUpdated event.
    ///
    /// Returns `Err(DomainError::NotFound)` when no rolling stock with `rolling_stock_id` exists.
    pub fn update_rolling_stock_service_level(
        &mut self,
        rolling_stock_id: &RollingStockId,
        service_level: Option<crate::catalog::domain::railway_model::ServiceLevel>,
    ) -> Result<(), DomainError> {
        let rs = self
            .rolling_stocks
            .iter_mut()
            .find(|rs| rs.id_as_ref() == rolling_stock_id)
            .ok_or_else(|| DomainError::NotFound {
                resource: "RollingStock".to_string(),
                identifier: rolling_stock_id.to_string(),
            })?;

        let changed = rs.apply_service_level(service_level);

        let ev = RailwayModelEvent::RollingStockUpdated {
            event_id: Uuid::new_v4(),
            railway_model_id: self.id.clone(),
            rolling_stock_id: rolling_stock_id.clone(),
            timestamp: chrono::Utc::now().naive_utc(),
            changed,
        };
        self.push_event(ev);
        Ok(())
    }

    /// Update the identification fields (series_code, road_number, livery, depot) of a rolling
    /// stock and emit a RollingStockUpdated event.
    ///
    /// Returns `Err(DomainError::Validation)` when `series_code` is empty.
    /// Returns `Err(DomainError::NotFound)` when no rolling stock with `rolling_stock_id` exists.
    pub fn update_rolling_stock_identification(
        &mut self,
        rolling_stock_id: &RollingStockId,
        series_code: String,
        road_number: Option<String>,
        livery: Option<String>,
        depot: Option<String>,
    ) -> Result<(), DomainError> {
        let trimmed_series = series_code.trim().to_string();
        if trimmed_series.is_empty() {
            return Err(DomainError::Validation(
                "series_code must not be empty".to_string(),
            ));
        }

        let rs = self
            .rolling_stocks
            .iter_mut()
            .find(|rs| rs.id_as_ref() == rolling_stock_id)
            .ok_or_else(|| DomainError::NotFound {
                resource: "RollingStock".to_string(),
                identifier: rolling_stock_id.to_string(),
            })?;

        let changed = rs.apply_identification_patch(trimmed_series, road_number, livery, depot);

        let ev = RailwayModelEvent::RollingStockUpdated {
            event_id: Uuid::new_v4(),
            railway_model_id: self.id.clone(),
            rolling_stock_id: rolling_stock_id.clone(),
            timestamp: chrono::Utc::now().naive_utc(),
            changed,
        };
        self.push_event(ev);
        Ok(())
    }

    /// Update the full technical specifications of a rolling stock and emit a RollingStockUpdated event.
    ///
    /// Returns `Err(DomainError::Validation)` when `spec.series_code` is empty.
    /// Returns `Err(DomainError::NotFound)` when no rolling stock with `rolling_stock_id` exists.
    pub fn update_rolling_stock_specifications(
        &mut self,
        rolling_stock_id: &RollingStockId,
        spec: RollingStockSpecPatch,
    ) -> Result<(), DomainError> {
        if spec.series_code.trim().is_empty() {
            return Err(DomainError::Validation(
                "series_code must not be empty".to_string(),
            ));
        }

        let rs = self
            .rolling_stocks
            .iter_mut()
            .find(|rs| rs.id_as_ref() == rolling_stock_id)
            .ok_or_else(|| DomainError::NotFound {
                resource: "RollingStock".to_string(),
                identifier: rolling_stock_id.to_string(),
            })?;

        let changed = rs.apply_specifications(spec);

        let ev = RailwayModelEvent::RollingStockUpdated {
            event_id: Uuid::new_v4(),
            railway_model_id: self.id.clone(),
            rolling_stock_id: rolling_stock_id.clone(),
            timestamp: chrono::Utc::now().naive_utc(),
            changed,
        };
        self.push_event(ev);
        Ok(())
    }

    /// Update only the control type, DCC interface, and length of a single rolling stock unit.
    ///
    /// Unlike [`update_rolling_stock_specifications`], this method only touches these three
    /// fields and leaves all other technical specification fields unchanged.
    ///
    /// Returns `Err(DomainError::NotFound)` when no rolling stock with `rolling_stock_id` exists.
    pub fn update_rolling_stock_dcc(
        &mut self,
        rolling_stock_id: &RollingStockId,
        patch: RollingStockDccPatch,
    ) -> Result<(), DomainError> {
        let rs = self
            .rolling_stocks
            .iter_mut()
            .find(|rs| rs.id_as_ref() == rolling_stock_id)
            .ok_or_else(|| DomainError::NotFound {
                resource: "RollingStock".to_string(),
                identifier: rolling_stock_id.to_string(),
            })?;

        let changed = rs.apply_dcc(patch);

        let ev = RailwayModelEvent::RollingStockUpdated {
            event_id: Uuid::new_v4(),
            railway_model_id: self.id.clone(),
            rolling_stock_id: rolling_stock_id.clone(),
            timestamp: chrono::Utc::now().naive_utc(),
            changed,
        };
        self.push_event(ev);
        Ok(())
    }

    /// Update category and emit a RailwayModelUpdated event.
    pub fn update_category(&mut self, category: Category) {
        self.category = category;
        let changed = json!({ "category": category.to_string() });
        let ev = RailwayModelEvent::RailwayModelUpdated {
            event_id: uuid::Uuid::new_v4(),
            railway_model_id: self.id.clone(),
            timestamp: chrono::Utc::now().naive_utc(),
            changed,
        };
        self.push_event(ev);
    }

    /// Update delivery date and emit a RailwayModelUpdated event.
    pub fn update_delivery_date(&mut self, delivery_date: Option<DeliveryDate>) {
        self.delivery_date = delivery_date.clone();
        let date_str = delivery_date.as_ref().map(|d| d.to_string());
        let changed = json!({ "delivery_date": date_str });
        let ev = RailwayModelEvent::RailwayModelUpdated {
            event_id: uuid::Uuid::new_v4(),
            railway_model_id: self.id.clone(),
            timestamp: chrono::Utc::now().naive_utc(),
            changed,
        };
        self.push_event(ev);
    }

    /// Update availability status and emit a RailwayModelUpdated event.
    pub fn set_availability_status(&mut self, status: Option<AvailabilityStatus>) {
        self.availability_status = status;
        let availability = status.as_ref().map(|s| s.to_string());
        let changed = json!({ "availability_status": availability });
        let ev = RailwayModelEvent::RailwayModelUpdated {
            event_id: uuid::Uuid::new_v4(),
            railway_model_id: self.id.clone(),
            timestamp: chrono::Utc::now().naive_utc(),
            changed,
        };
        self.push_event(ev);
    }

    /// Emit RollingStockAdded event for a new rolling stock. Returns the generated id.
    pub fn add_rolling_stock(&mut self, params: RollingStockParams) -> RollingStockId {
        let rs_id = RollingStockId::from_uuid(&Uuid::new_v4());
        let ev = RailwayModelEvent::RollingStockAdded {
            event_id: uuid::Uuid::new_v4(),
            railway_model_id: self.id.clone(),
            timestamp: chrono::Utc::now().naive_utc(),
            rolling_stock_id: rs_id.clone(),
            rolling_stock_params: params,
        };
        self.push_event(ev);
        rs_id
    }

    /// Emit RollingStockRemoved event and remove from in-memory collection if present.
    pub fn remove_rolling_stock(&mut self, id: &RollingStockId) {
        self.rolling_stocks.retain(|rs| rs.id_as_ref() != id);
        let ev = RailwayModelEvent::RollingStockRemoved {
            event_id: uuid::Uuid::new_v4(),
            railway_model_id: self.id.clone(),
            timestamp: chrono::Utc::now().naive_utc(),
            rolling_stock_id: id.clone(),
        };
        self.push_event(ev);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::domain::railway_model::category::LocomotiveType;
    use crate::catalog::domain::railway_model::rolling_stock::RollingStockSpecPatch;
    use crate::catalog::domain::railway_model::{
        Category, Epoch, PowerMethod, ProductCode, RollingStock,
    };
    use pretty_assertions::assert_eq;

    fn make_test_model() -> RailwayModel {
        let manufacturer_id =
            crate::catalog::domain::manufacturer::ManufacturerId::try_from("trn:manufacturer:acme")
                .unwrap();
        let product_code = ProductCode::try_from("TEST-001").unwrap();
        let id = RailwayModelId::new(&manufacturer_id, "TEST-001").unwrap();
        RailwayModel {
            id,
            manufacturer_id,
            product_code,
            description: LocalizedField {
                lang: Language::English,
                value: "A test model".to_string(),
            },
            details: Some(LocalizedField {
                lang: Language::English,
                value: "Some details".to_string(),
            }),
            power_method: PowerMethod::DC,
            scale: Scale::H0,
            epoch: Epoch::from("IV"),
            category: Category::Locomotives,
            delivery_date: None,
            availability_status: None,
            rolling_stocks: vec![],
            pending_events: vec![],
        }
    }

    fn make_test_locomotive(id: RollingStockId) -> RollingStock {
        let railway = RailwayCompanyId::try_from("trn:railway-company:fs").unwrap();
        RollingStock::Locomotive {
            id,
            railway_id: railway,
            livery: None,
            length_over_buffer: None,
            technical_specifications: None,
            friendly_name: None,
            series_code: "SC-1".to_string(),
            road_number: Some("100".to_string()),
            series: None,
            depot: None,
            locomotive_type: LocomotiveType::ElectricLocomotive,
            dcc_interface: None,
            control: None,
            is_dummy: false,
        }
    }

    // --- T005: US1 tests (update_description, update_details) ---

    #[test]
    fn update_description_emits_translation_upserted_event() {
        let mut model = make_test_model();
        let result = model.update_description("new description".to_string());
        assert!(result.is_ok());
        assert_eq!(model.description.value, "new description");
        let events = model.pull_events();
        assert_eq!(events.len(), 1);
        match &events[0] {
            RailwayModelEvent::TranslationUpserted {
                description, lang, ..
            } => {
                assert_eq!(description.as_deref(), Some("new description"));
                assert_eq!(*lang, Language::English);
            }
            _ => panic!("expected TranslationUpserted"),
        }
    }

    #[test]
    fn update_description_empty_returns_error() {
        let mut model = make_test_model();
        let result = model.update_description("".to_string());
        assert!(result.is_err());
        let events = model.pull_events();
        assert_eq!(events.len(), 0, "no event should be emitted on error");
    }

    #[test]
    fn update_details_none_emits_translation_upserted_event() {
        let mut model = make_test_model();
        model.update_details(None);
        let events = model.pull_events();
        assert_eq!(events.len(), 1);
        match &events[0] {
            RailwayModelEvent::TranslationUpserted { details, .. } => {
                assert!(details.is_none());
            }
            _ => panic!("expected TranslationUpserted"),
        }
    }

    // --- T013: US2 tests (update_rolling_stock_identification) ---

    #[test]
    fn update_rolling_stock_identification_emits_event() {
        let mut model = make_test_model();
        let rs_id = RollingStockId::from_uuid(&uuid::Uuid::new_v4());
        let loco = make_test_locomotive(rs_id.clone());
        model.rolling_stocks.push(loco);

        let result = model.update_rolling_stock_identification(
            &rs_id,
            "SC-NEW".to_string(),
            Some("456".to_string()),
            None,
            None,
        );
        assert!(result.is_ok());
        let events = model.pull_events();
        assert_eq!(events.len(), 1);
        match &events[0] {
            RailwayModelEvent::RollingStockUpdated {
                changed,
                rolling_stock_id,
                ..
            } => {
                assert_eq!(*rolling_stock_id, rs_id);
                assert_eq!(changed["series_code"], "SC-NEW");
                assert_eq!(changed["road_number"], "456");
            }
            _ => panic!("expected RollingStockUpdated"),
        }
    }

    #[test]
    fn update_rolling_stock_identification_empty_series_code_returns_error() {
        let mut model = make_test_model();
        let rs_id = RollingStockId::from_uuid(&uuid::Uuid::new_v4());
        let loco = make_test_locomotive(rs_id.clone());
        model.rolling_stocks.push(loco);

        let result =
            model.update_rolling_stock_identification(&rs_id, "".to_string(), None, None, None);
        assert!(result.is_err());
    }

    // --- T022: US3 tests (update_scale, update_epoch) ---

    #[test]
    fn update_scale_emits_event() {
        let mut model = make_test_model();
        model.update_scale(Scale::N);
        let events = model.pull_events();
        assert_eq!(events.len(), 1);
        match &events[0] {
            RailwayModelEvent::RailwayModelUpdated { changed, .. } => {
                assert_eq!(changed["scale"], "N");
            }
            _ => panic!("expected RailwayModelUpdated"),
        }
    }

    #[test]
    fn update_epoch_emits_event() {
        let mut model = make_test_model();
        model.update_epoch(Epoch::from("IV"));
        let events = model.pull_events();
        assert_eq!(events.len(), 1);
        match &events[0] {
            RailwayModelEvent::RailwayModelUpdated { changed, .. } => {
                assert_eq!(changed["epoch"], "IV");
            }
            _ => panic!("expected RailwayModelUpdated"),
        }
    }

    // --- T022: US3 tests (update_rolling_stock_railway_company) ---

    #[test]
    fn update_rolling_stock_railway_company_emits_event() {
        let mut model = make_test_model();
        let rs_id = RollingStockId::from_uuid(&uuid::Uuid::new_v4());
        let loco = make_test_locomotive(rs_id.clone());
        model.rolling_stocks.push(loco);

        let new_company = RailwayCompanyId::try_from("trn:railway-company:sncf").unwrap();
        let result = model.update_rolling_stock_railway_company(&rs_id, new_company);
        assert!(result.is_ok());
        let events = model.pull_events();
        assert_eq!(events.len(), 1);
        match &events[0] {
            RailwayModelEvent::RollingStockUpdated {
                changed,
                rolling_stock_id,
                ..
            } => {
                assert_eq!(*rolling_stock_id, rs_id);
                assert_eq!(changed["railway_company_id"], "trn:railway-company:sncf");
            }
            _ => panic!("expected RollingStockUpdated"),
        }
    }

    #[test]
    fn update_rolling_stock_railway_company_not_found_returns_error() {
        let mut model = make_test_model();
        let missing_id = RollingStockId::from_uuid(&uuid::Uuid::new_v4());
        let company = RailwayCompanyId::try_from("trn:railway-company:db").unwrap();
        let result = model.update_rolling_stock_railway_company(&missing_id, company);
        assert!(matches!(result, Err(DomainError::NotFound { .. })));
    }

    // --- T034: US4 tests (update_rolling_stock_specifications) ---

    #[test]
    fn update_rolling_stock_specifications_emits_event() {
        let mut model = make_test_model();
        let rs_id = RollingStockId::from_uuid(&uuid::Uuid::new_v4());
        let loco = make_test_locomotive(rs_id.clone());
        model.rolling_stocks.push(loco);

        let spec = RollingStockSpecPatch {
            series_code: "SC-FULL".to_string(),
            road_number: Some("999".to_string()),
            livery: None,
            depot: None,
            series: None,
            friendly_name: None,
            flywheel_fitted: None,
            body_shell: None,
            chassis: None,
            interior_lights: None,
            lights: None,
            dcc_interface: None,
            control: None,
            coupling_socket: None,
            close_couplers: None,
            digital_shunting: None,
        };

        let result = model.update_rolling_stock_specifications(&rs_id, spec);
        assert!(result.is_ok());
        let events = model.pull_events();
        assert_eq!(events.len(), 1);
        match &events[0] {
            RailwayModelEvent::RollingStockUpdated { changed, .. } => {
                assert_eq!(changed["series_code"], "SC-FULL");
                assert_eq!(changed["road_number"], "999");
            }
            _ => panic!("expected RollingStockUpdated"),
        }
    }

    #[test]
    fn update_rolling_stock_specifications_empty_series_code_returns_error() {
        let mut model = make_test_model();
        let rs_id = RollingStockId::from_uuid(&uuid::Uuid::new_v4());
        let loco = make_test_locomotive(rs_id.clone());
        model.rolling_stocks.push(loco);

        let spec = RollingStockSpecPatch {
            series_code: "".to_string(),
            road_number: None,
            livery: None,
            depot: None,
            series: None,
            friendly_name: None,
            flywheel_fitted: None,
            body_shell: None,
            chassis: None,
            interior_lights: None,
            lights: None,
            dcc_interface: None,
            control: None,
            coupling_socket: None,
            close_couplers: None,
            digital_shunting: None,
        };

        let result = model.update_rolling_stock_specifications(&rs_id, spec);
        assert!(result.is_err());
    }

    #[test]
    fn update_rolling_stock_specifications_all_optional_none_is_valid() {
        let mut model = make_test_model();
        let rs_id = RollingStockId::from_uuid(&uuid::Uuid::new_v4());
        let loco = make_test_locomotive(rs_id.clone());
        model.rolling_stocks.push(loco);

        let spec = RollingStockSpecPatch {
            series_code: "SC-VALID".to_string(),
            road_number: None,
            livery: None,
            depot: None,
            series: None,
            friendly_name: None,
            flywheel_fitted: None,
            body_shell: None,
            chassis: None,
            interior_lights: None,
            lights: None,
            dcc_interface: None,
            control: None,
            coupling_socket: None,
            close_couplers: None,
            digital_shunting: None,
        };

        let result = model.update_rolling_stock_specifications(&rs_id, spec);
        assert!(result.is_ok());
    }
}
