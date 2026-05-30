use crate::catalog::domain::railway_model::localized_field::LocalizedField;
use crate::catalog::domain::railway_model::{
    Category, ElectricMultipleUnitType, Epoch, FreightCarType, LocomotiveType, PassengerCarType,
    PowerMethod, ProductCode, RailcarType, RollingStockCategory,
};
use crate::catalog::domain::railway_model::{
    RailwayModel, RailwayModelEvent, RailwayModelId, RailwayModelParams, RailwayModelUowExt,
    RollingStock, RollingStockParams,
};
use crate::catalog::domain::scale::Scale;
use crate::catalog::domain::{manufacturer::ManufacturerId, railway_company::RailwayCompanyId};
use crate::core::domain::{Language, domain_error::DomainError};
use chrono::Utc;
use serde::Deserialize;
use serde_json::json;

/// Input for saving (create-or-merge) a simplified railway model.
#[derive(Debug, Clone, Deserialize)]
pub struct SaveRailwayModelInput {
    pub manufacturer_id: String,
    pub product_code: String,
    pub description: String,
    pub category: String,
    pub scale: String,
    pub epoch: String,
    pub power_method: String,
    pub rolling_stocks: Vec<SimplifiedRollingStockInput>,
}

/// Simplified rolling stock input used by the simplified commands.
#[derive(Debug, Clone, Deserialize)]
pub struct SimplifiedRollingStockInput {
    pub railway_company_id: String,
    pub series_code: String,
    pub road_number: Option<String>,
    pub subcategory: Option<String>,
    pub category: String,
}

/// Maps a `SimplifiedRollingStockInput` and its resolved `RailwayCompanyId` into
/// the appropriate `RollingStockParams` variant, returning a `DomainError` for
/// any invalid field values.
fn map_simple_rolling_stock(
    rs: SimplifiedRollingStockInput,
    company_id: RailwayCompanyId,
) -> Result<RollingStockParams, DomainError> {
    match rs.category.parse::<RollingStockCategory>() {
        Ok(RollingStockCategory::Locomotive) => {
            let loco_type = rs.subcategory.ok_or_else(|| {
                DomainError::Validation("subcategory required for locomotive".to_string())
            })?;
            let loco_type = loco_type
                .parse::<LocomotiveType>()
                .map_err(|e| DomainError::Validation(e.to_string()))?;

            Ok(RollingStockParams::LocomotiveParams {
                railway_company_id: company_id,
                livery: None,
                length_over_buffers: None,
                technical_specifications: None,
                friendly_name: "".to_string(),
                series_code: Some(rs.series_code),
                road_number: rs.road_number.unwrap_or_default(),
                series: None,
                depot: None,
                locomotive_type: loco_type,
                dcc_interface: None,
                control: None,
                is_dummy: false,
            })
        }
        Ok(RollingStockCategory::PassengerCar) => {
            let passenger_car_type = rs
                .subcategory
                .as_deref()
                .and_then(|s| s.parse::<PassengerCarType>().ok());
            Ok(RollingStockParams::PassengerCarParams {
                railway_company_id: company_id,
                livery: None,
                length_over_buffers: None,
                technical_specifications: None,
                friendly_name: "".to_string(),
                series_code: Some(rs.series_code),
                road_number: rs.road_number,
                series: None,
                passenger_car_type,
                service_level: None,
            })
        }
        Ok(RollingStockCategory::FreightCar) => {
            let freight_car_type = rs
                .subcategory
                .as_deref()
                .and_then(|s| s.parse::<FreightCarType>().ok());
            Ok(RollingStockParams::FreightCarParams {
                railway_company_id: company_id,
                livery: None,
                length_over_buffers: None,
                technical_specifications: None,
                friendly_name: "".to_string(),
                series_code: Some(rs.series_code),
                road_number: rs.road_number,
                series: None,
                freight_car_type,
            })
        }
        Ok(RollingStockCategory::ElectricMultipleUnit) => {
            let electric_multiple_unit_type = rs
                .subcategory
                .as_deref()
                .and_then(|s| s.parse::<ElectricMultipleUnitType>().ok())
                .unwrap_or_default();
            Ok(RollingStockParams::ElectricMultipleUnitParams {
                railway_company_id: company_id,
                livery: None,
                length_over_buffers: None,
                technical_specifications: None,
                friendly_name: "".to_string(),
                series_code: Some(rs.series_code),
                road_number: rs.road_number,
                series: None,
                depot: None,
                electric_multiple_unit_type,
                dcc_interface: None,
                control: None,
                is_dummy: false,
            })
        }
        Ok(RollingStockCategory::Railcar) => {
            let railcar_type = rs
                .subcategory
                .as_deref()
                .and_then(|s| s.parse::<RailcarType>().ok())
                .unwrap_or_default();
            Ok(RollingStockParams::RailcarParams {
                railway_company_id: company_id,
                livery: None,
                length_over_buffers: None,
                technical_specifications: None,
                friendly_name: "".to_string(),
                series_code: Some(rs.series_code),
                road_number: rs.road_number,
                series: None,
                depot: None,
                railcar_type,
                dcc_interface: None,
                control: None,
                is_dummy: false,
            })
        }
        Err(_) => Err(DomainError::Validation(
            "invalid rolling stock category".to_string(),
        )),
    }
}

/// Use case that saves or merges a simplified railway model into the catalog.
pub struct SaveRailwayModel;

impl SaveRailwayModel {
    /// Save or merge a simplified railway model. If a model already exists
    /// it is patched with the provided values (incoming values win). New
    /// rolling stocks are appended.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: SaveRailwayModelInput,
    ) -> Result<RailwayModelId, DomainError>
    where
        U: RailwayModelUowExt + Send,
    {
        let mut repo = unit_of_work.railway_model_repository();

        // Basic validation / parsing
        let manufacturer_id = ManufacturerId::try_from(&input.manufacturer_id)
            .map_err(|e| DomainError::Validation(e.to_string()))?;

        let product_code = ProductCode::try_from(input.product_code.clone())
            .map_err(|e| DomainError::Validation(e.to_string()))?;

        let power_method = input
            .power_method
            .parse::<PowerMethod>()
            .map_err(|e| DomainError::Validation(e.to_string()))?;

        let scale = Scale::try_from(input.scale.as_str())
            .map_err(|e| DomainError::Validation(e.to_string()))?;

        let category = input
            .category
            .parse::<Category>()
            .map_err(|e| DomainError::Validation(e.to_string()))?;

        let epoch = Epoch::from(input.epoch.as_str());

        let railway_model_id = RailwayModelId::new(&manufacturer_id, &product_code.to_string())
            .map_err(|e| DomainError::Validation(e.to_string()))?;

        // Try to find an existing aggregate
        let existing = repo
            .find_by_id(&railway_model_id, Language::English)
            .await?;

        if let Some(mut aggregate) = existing {
            // Merge: incoming wins for main fields
            let mut changed = serde_json::Map::new();

            if aggregate.description.value != input.description {
                aggregate.description = LocalizedField {
                    lang: aggregate.description.lang,
                    value: input.description.clone(),
                };
                changed.insert("description".to_string(), json!(input.description));
            }

            // power method, scale, category, epoch — treat as replacements
            aggregate.power_method = power_method;
            changed.insert(
                "power_method".to_string(),
                json!(aggregate.power_method.to_string()),
            );

            aggregate.scale = scale;
            changed.insert("scale".to_string(), json!(aggregate.scale.to_string()));

            aggregate.category = category;
            changed.insert(
                "category".to_string(),
                json!(aggregate.category.to_string()),
            );

            aggregate.epoch = epoch;
            changed.insert("epoch".to_string(), json!(aggregate.epoch.0.clone()));

            if !changed.is_empty() {
                let ev = RailwayModelEvent::RailwayModelUpdated {
                    event_id: uuid::Uuid::new_v4(),
                    railway_model_id: railway_model_id.clone(),
                    timestamp: Utc::now().naive_utc(),
                    changed: serde_json::Value::Object(changed),
                };
                aggregate.push_event(ev);
            }

            // Append rolling stocks (but skip duplicates)
            for rs in input.rolling_stocks.into_iter() {
                let company_id = RailwayCompanyId::try_from(&rs.railway_company_id)
                    .map_err(|e| DomainError::Validation(e.to_string()))?;

                // Check if a rolling stock with the same series_code, road_number, and railway_id already exists
                let already_exists = aggregate.rolling_stocks.iter().any(|existing_rs| {
                    // Get series_code from existing rolling stock
                    let existing_series_code = match existing_rs {
                        RollingStock::ElectricMultipleUnit { series_code, .. } => {
                            series_code.as_str()
                        }
                        RollingStock::Locomotive { series_code, .. } => series_code.as_str(),
                        RollingStock::FreightCar { series_code, .. } => series_code.as_str(),
                        RollingStock::PassengerCar { series_code, .. } => series_code.as_str(),
                        RollingStock::Railcar { series_code, .. } => series_code.as_str(),
                    };

                    existing_series_code == rs.series_code
                        && existing_rs.road_number() == rs.road_number.as_deref()
                        && existing_rs.railway_id() == &company_id
                });

                if already_exists {
                    // Skip this rolling stock as it's already in the catalog
                    continue;
                }

                let params = map_simple_rolling_stock(rs, company_id)?;
                aggregate.add_rolling_stock(params);
            }

            // Persist
            repo.save(&mut aggregate).await?;

            Ok(railway_model_id)
        } else {
            // Create new aggregate (reuse AddRailwayModel style)
            let rolling_stocks = input
                .rolling_stocks
                .into_iter()
                .map(|rs| {
                    let company_id = RailwayCompanyId::try_from(&rs.railway_company_id)
                        .map_err(|e| DomainError::Validation(e.to_string()))?;
                    map_simple_rolling_stock(rs, company_id)
                })
                .collect::<Result<Vec<RollingStockParams>, DomainError>>()?;

            let railway_model_params = RailwayModelParams {
                manufacturer_id: manufacturer_id.clone(),
                product_code: product_code.clone(),
                power_method,
                scale,
                category,
                epoch,
                delivery_date: None,
                availability_status: None,
                description: input.description,
                details: None,
                rolling_stocks,
            };

            let mut aggregate = RailwayModel {
                id: railway_model_id.clone(),
                manufacturer_id: railway_model_params.manufacturer_id.clone(),
                product_code: railway_model_params.product_code.clone(),
                description: LocalizedField {
                    lang: Language::English,
                    value: railway_model_params.description.clone(),
                },
                details: railway_model_params
                    .details
                    .clone()
                    .map(|v| LocalizedField {
                        lang: Language::English,
                        value: v,
                    }),
                power_method: railway_model_params.power_method,
                scale: railway_model_params.scale.clone(),
                epoch: railway_model_params.epoch.clone(),
                category: railway_model_params.category,
                delivery_date: railway_model_params.delivery_date.clone(),
                availability_status: railway_model_params.availability_status,
                rolling_stocks: Vec::new(),
                pending_events: Vec::new(),
            };

            let created_event = RailwayModelEvent::RailwayModelCreated {
                event_id: uuid::Uuid::new_v4(),
                railway_model_id: railway_model_id.clone(),
                timestamp: Utc::now().naive_utc(),
                params: railway_model_params,
            };

            aggregate.push_event(created_event);

            repo.save(&mut aggregate).await.map(|_| railway_model_id)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::application::testing::FakeUow;
    use crate::catalog::domain::manufacturer::ManufacturerId;
    use crate::catalog::domain::railway_company::RailwayCompanyId;
    use crate::catalog::domain::railway_model::{
        Category, MockRailwayModelRepository, PowerMethod, ProductCode, RailwayModel,
        RailwayModelEvent, RollingStock, RollingStockId, RollingStockParams,
    };
    use crate::catalog::domain::scale::Scale;
    // chrono::NaiveDate not used in these tests

    fn base_input() -> SaveRailwayModelInput {
        SaveRailwayModelInput {
            manufacturer_id: "trn:manufacturer:acme".to_string(),
            product_code: "P100".to_string(),
            description: "A test model".to_string(),
            category: "Locomotives".to_string(),
            scale: "H0".to_string(),
            epoch: "IV".to_string(),
            power_method: "DC".to_string(),
            rolling_stocks: vec![],
        }
    }

    fn existing_model_with(
        rolling_stocks: Vec<RollingStock>,
        description: &str,
        epoch: &str,
    ) -> RailwayModel {
        let manufacturer = ManufacturerId::try_from("trn:manufacturer:acme").unwrap();
        let product = ProductCode::try_from("P100").unwrap();
        let existing_id =
            crate::catalog::domain::railway_model::RailwayModelId::new(&manufacturer, "P100")
                .unwrap();

        RailwayModel {
            id: existing_id,
            manufacturer_id: manufacturer,
            product_code: product,
            description: LocalizedField {
                lang: Language::English,
                value: description.to_string(),
            },
            details: None,
            power_method: PowerMethod::DC,
            scale: Scale::H0,
            epoch: epoch.into(),
            category: Category::Locomotives,
            delivery_date: None,
            availability_status: None,
            rolling_stocks,
            pending_events: Vec::new(),
        }
    }

    #[tokio::test]
    async fn it_creates_new_railway_model_when_missing() {
        let mut mock = MockRailwayModelRepository::new();
        mock.expect_find_by_id().times(1).returning(|_, _| Ok(None));
        mock.expect_save().times(1).returning(|_| Ok(()));

        let mut uow = FakeUow::with_railway_models_repo(mock);

        let input = base_input();

        let id = SaveRailwayModel::execute(&mut uow, input)
            .await
            .expect("should create railway model");

        // Basic sanity: id contains manufacturer namespace
        assert!(id.to_string().contains("acme"));
    }

    #[tokio::test]
    async fn it_updates_existing_railway_model_and_appends_rolling_stock() {
        let mut mock = MockRailwayModelRepository::new();

        // build an existing minimal aggregate
        let existing = existing_model_with(vec![], "Old desc", "III");
        let existing_id = existing.id.clone();

        mock.expect_find_by_id()
            .times(1)
            .returning(move |_, _| Ok(Some(existing.clone())));
        mock.expect_save().times(1).returning(|_| Ok(()));

        let mut uow = FakeUow::with_railway_models_repo(mock);

        let input = SaveRailwayModelInput {
            manufacturer_id: "trn:manufacturer:acme".to_string(),
            product_code: "P100".to_string(),
            description: "New desc".to_string(),
            category: "Locomotives".to_string(),
            scale: "H0".to_string(),
            epoch: "IV".to_string(),
            power_method: "DC".to_string(),
            rolling_stocks: vec![SimplifiedRollingStockInput {
                railway_company_id: "trn:railway-company:rc1".to_string(),
                series_code: "S1".to_string(),
                road_number: Some("100".to_string()),
                subcategory: Some("STEAM_LOCOMOTIVE".to_string()),
                category: "Locomotive".to_string(),
            }],
        };

        let id = SaveRailwayModel::execute(&mut uow, input)
            .await
            .expect("should update railway model");

        assert_eq!(id, existing_id);
    }

    #[test]
    fn map_simple_rolling_stock_requires_locomotive_subcategory() {
        let company_id =
            RailwayCompanyId::try_from("trn:railway-company:rc1").expect("company id should parse");
        let input = SimplifiedRollingStockInput {
            railway_company_id: "trn:railway-company:rc1".to_string(),
            series_code: "E464".to_string(),
            road_number: Some("001".to_string()),
            subcategory: None,
            category: "Locomotive".to_string(),
        };

        let result = map_simple_rolling_stock(input, company_id);
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn map_simple_rolling_stock_maps_passenger_car_without_subcategory() {
        let company_id =
            RailwayCompanyId::try_from("trn:railway-company:rc1").expect("company id should parse");
        let input = SimplifiedRollingStockInput {
            railway_company_id: "trn:railway-company:rc1".to_string(),
            series_code: "UIC-Z".to_string(),
            road_number: Some("50 83".to_string()),
            subcategory: None,
            category: "PASSENGER_CAR".to_string(),
        };

        let mapped = map_simple_rolling_stock(input, company_id).expect("mapping should succeed");
        match mapped {
            RollingStockParams::PassengerCarParams {
                passenger_car_type, ..
            } => assert!(passenger_car_type.is_none()),
            _ => panic!("expected passenger car params"),
        }
    }

    #[test]
    fn map_simple_rolling_stock_maps_emu_and_railcar_defaults() {
        let company_id =
            RailwayCompanyId::try_from("trn:railway-company:rc1").expect("company id should parse");

        let emu = SimplifiedRollingStockInput {
            railway_company_id: "trn:railway-company:rc1".to_string(),
            series_code: "ETR".to_string(),
            road_number: None,
            subcategory: Some("NOT_A_REAL_EMU_TYPE".to_string()),
            category: "ELECTRIC_MULTIPLE_UNIT".to_string(),
        };
        let railcar = SimplifiedRollingStockInput {
            railway_company_id: "trn:railway-company:rc1".to_string(),
            series_code: "ALn".to_string(),
            road_number: None,
            subcategory: Some("NOT_A_REAL_RAILCAR_TYPE".to_string()),
            category: "RAILCAR".to_string(),
        };

        let emu_mapped =
            map_simple_rolling_stock(emu, company_id.clone()).expect("emu mapping should succeed");
        let railcar_mapped =
            map_simple_rolling_stock(railcar, company_id).expect("railcar mapping should succeed");

        assert!(matches!(
            emu_mapped,
            RollingStockParams::ElectricMultipleUnitParams { .. }
        ));
        assert!(matches!(
            railcar_mapped,
            RollingStockParams::RailcarParams { .. }
        ));
    }

    #[test]
    fn map_simple_rolling_stock_rejects_invalid_category() {
        let company_id =
            RailwayCompanyId::try_from("trn:railway-company:rc1").expect("company id should parse");
        let input = SimplifiedRollingStockInput {
            railway_company_id: "trn:railway-company:rc1".to_string(),
            series_code: "X".to_string(),
            road_number: None,
            subcategory: None,
            category: "UnknownCategory".to_string(),
        };

        let result = map_simple_rolling_stock(input, company_id);
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[tokio::test]
    async fn it_skips_duplicate_rolling_stock_when_merging_existing_model() {
        let mut mock = MockRailwayModelRepository::new();

        let duplicate = RollingStock::Locomotive {
            id: RollingStockId::default(),
            railway_id: RailwayCompanyId::try_from("trn:railway-company:rc1").unwrap(),
            livery: None,
            length_over_buffer: None,
            technical_specifications: None,
            friendly_name: None,
            series_code: "S1".to_string(),
            road_number: Some("100".to_string()),
            series: None,
            depot: None,
            locomotive_type: crate::catalog::domain::railway_model::LocomotiveType::SteamLocomotive,
            dcc_interface: None,
            control: None,
            is_dummy: false,
        };

        let existing = existing_model_with(vec![duplicate], "A test model", "IV");
        let existing_id = existing.id.clone();

        mock.expect_find_by_id()
            .times(1)
            .returning(move |_, _| Ok(Some(existing.clone())));
        mock.expect_save().times(1).returning(|aggregate| {
            assert_eq!(aggregate.pending_events.len(), 1);
            assert!(matches!(
                aggregate.pending_events.first(),
                Some(RailwayModelEvent::RailwayModelUpdated { .. })
            ));
            assert!(
                aggregate
                    .pending_events
                    .iter()
                    .all(|event| !matches!(event, RailwayModelEvent::RollingStockAdded { .. }))
            );
            Ok(())
        });

        let mut uow = FakeUow::with_railway_models_repo(mock);
        let mut input = base_input();
        input.rolling_stocks = vec![SimplifiedRollingStockInput {
            railway_company_id: "trn:railway-company:rc1".to_string(),
            series_code: "S1".to_string(),
            road_number: Some("100".to_string()),
            subcategory: Some("STEAM_LOCOMOTIVE".to_string()),
            category: "Locomotive".to_string(),
        }];

        let id = SaveRailwayModel::execute(&mut uow, input)
            .await
            .expect("merge should succeed");

        assert_eq!(id, existing_id);
    }

    #[tokio::test]
    async fn it_rejects_invalid_nested_railway_company_before_save() {
        let mut mock = MockRailwayModelRepository::new();
        let existing = existing_model_with(vec![], "Old desc", "III");

        mock.expect_find_by_id()
            .times(1)
            .returning(move |_, _| Ok(Some(existing.clone())));
        mock.expect_save().times(0);

        let mut uow = FakeUow::with_railway_models_repo(mock);
        let mut input = base_input();
        input.rolling_stocks = vec![SimplifiedRollingStockInput {
            railway_company_id: "invalid-company-id".to_string(),
            series_code: "S1".to_string(),
            road_number: Some("100".to_string()),
            subcategory: Some("STEAM_LOCOMOTIVE".to_string()),
            category: "Locomotive".to_string(),
        }];

        let result = SaveRailwayModel::execute(&mut uow, input).await;

        assert!(
            matches!(result, Err(DomainError::Validation(_))),
            "{result:?}"
        );
    }
}
