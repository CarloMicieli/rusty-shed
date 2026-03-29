//! T081 — Use-case boundary tests for the train-formations application layer.
//!
//! These tests verify that each use case handles input validation, not-found
//! errors, and cross-layer contracts correctly against a real in-memory SQLite
//! database.

// ─────────────────────────────────────────────────────────────────────────────
// create_train_formation
// ─────────────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod create_train_formation {
    use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
    use crate::trains::application::create_train_formation::CreateTrainFormationUseCase;
    use crate::trains::interface::command_args::CreateTrainFormationArgs;
    use garde::Validate;

    /// An empty `name` field must fail `garde` validation before the use case
    /// is ever executed (mirrors the command-handler validation gate).
    #[test]
    fn test_create_train_formation_empty_name_fails_validation() {
        let args = CreateTrainFormationArgs {
            name: String::new(),
            category_id: None,
            start_year: None,
            end_year: None,
            epoch: None,
            notes: None,
        };
        assert!(
            args.validate().is_err(),
            "empty name must fail garde validation"
        );
    }

    /// Valid args must produce a `TrainFormationView` with the correct name.
    #[sqlx::test(migrations = "./migrations")]
    async fn test_create_train_formation_valid_args(pool: sqlx::SqlitePool) {
        let mut uow = SqliteUnitOfWork::new(&pool).await.expect("create uow");

        let args = CreateTrainFormationArgs {
            name: "EuroCity Gottardo".into(),
            category_id: None,
            start_year: Some(1975),
            end_year: Some(1985),
            epoch: Some("IV".into()),
            notes: Some("Scenic alpine route".into()),
        };

        let result = CreateTrainFormationUseCase::execute(&mut uow, args).await;

        assert!(result.is_ok(), "valid args must succeed: {result:?}");
        let view = result.unwrap();
        assert_eq!(view.name, "EuroCity Gottardo");
        assert_eq!(view.epoch, Some("IV".into()));
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// update_train_formation
// ─────────────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod update_train_formation {
    use crate::core::domain::domain_error::DomainError;
    use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
    use crate::trains::application::update_train_formation::UpdateTrainFormationUseCase;
    use crate::trains::interface::command_args::UpdateTrainFormationArgs;

    /// Updating a non-existent formation must return `DomainError::NotFound`.
    #[sqlx::test(migrations = "./migrations")]
    async fn test_update_train_formation_not_found(pool: sqlx::SqlitePool) {
        let mut uow = SqliteUnitOfWork::new(&pool).await.expect("create uow");

        let result = UpdateTrainFormationUseCase::execute(
            &mut uow,
            "non-existent-id".into(),
            UpdateTrainFormationArgs {
                name: Some("New Name".into()),
                category_id: None,
                start_year: None,
                end_year: None,
                epoch: None,
                notes: None,
            },
        )
        .await;

        assert!(
            matches!(result, Err(DomainError::NotFound { .. })),
            "missing ID must return NotFound, got {result:?}"
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// delete_train_formation
// ─────────────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod delete_train_formation {
    use crate::core::domain::domain_error::DomainError;
    use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
    use crate::trains::application::delete_train_formation::DeleteTrainFormationUseCase;

    /// Deleting a non-existent formation must return `DomainError::NotFound`.
    #[sqlx::test(migrations = "./migrations")]
    async fn test_delete_train_formation_not_found(pool: sqlx::SqlitePool) {
        let mut uow = SqliteUnitOfWork::new(&pool).await.expect("create uow");

        let result = DeleteTrainFormationUseCase::execute(&mut uow, "non-existent-id".into()).await;

        assert!(
            matches!(result, Err(DomainError::NotFound { .. })),
            "missing ID must return NotFound, got {result:?}"
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// add_formation_element
// ─────────────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod add_formation_element {
    use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
    use crate::trains::application::add_formation_element::AddFormationElementUseCase;
    use crate::trains::application::create_train_formation::CreateTrainFormationUseCase;
    use crate::trains::interface::command_args::{
        AddFormationElementArgs, CreateTrainFormationArgs,
    };

    /// Adding an element whose `prototype_id` does not exist must return an error
    /// (FK RESTRICT constraint at the DB level).
    #[sqlx::test(migrations = "./migrations")]
    async fn test_add_formation_element_invalid_prototype_id(pool: sqlx::SqlitePool) {
        let mut uow = SqliteUnitOfWork::new(&pool).await.expect("create uow");

        // Create a formation to attach the element to
        let formation = CreateTrainFormationUseCase::execute(
            &mut uow,
            CreateTrainFormationArgs {
                name: "Element Test Formation".into(),
                category_id: None,
                start_year: None,
                end_year: None,
                epoch: None,
                notes: None,
            },
        )
        .await
        .expect("create formation");

        let result = AddFormationElementUseCase::execute(
            &mut uow,
            formation.id.clone(),
            AddFormationElementArgs {
                prototype_id: "non-existent-prototype".into(),
                owned_rolling_stock_id: None,
            },
        )
        .await;

        assert!(
            result.is_err(),
            "adding an element with a non-existent prototype_id must fail"
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// reorder_formation_elements
// ─────────────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod reorder_formation_elements {
    use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
    use crate::trains::application::add_formation_element::AddFormationElementUseCase;
    use crate::trains::application::create_train_formation::CreateTrainFormationUseCase;
    use crate::trains::application::reorder_formation_elements::ReorderFormationElementsUseCase;
    use crate::trains::interface::command_args::{
        AddFormationElementArgs, CreateTrainFormationArgs, ReorderFormationElementsArgs,
    };
    use chrono::Utc;

    /// A valid reorder must succeed and the returned detail must reflect the
    /// new element order.
    #[sqlx::test(migrations = "./migrations")]
    async fn test_reorder_formation_elements_valid(pool: sqlx::SqlitePool) {
        // Insert a railway company and prototype (committed to pool before UoW)
        sqlx::query(
            "INSERT OR IGNORE INTO railway_companies
             (id, name, registered_company_name, country_code, status, operating_since)
             VALUES ('trn:railway-company:fs', 'FS', 'Ferrovie dello Stato', 'IT', 'ACTIVE', '1905-07-01')",
        )
        .execute(&pool)
        .await
        .expect("insert company");

        let now = Utc::now().to_rfc3339();
        sqlx::query(
            "INSERT INTO prototypes
             (id, railway_company_id, series_code, car_type, service_level,
              category, is_motorized, default_is_dummy, is_custom, created_at, updated_at, version)
             VALUES ('proto-reorder-1', 'trn:railway-company:fs', 'Re 4/4 II',
                     'Locomotive', NULL, 'Locomotive', 1, 0, 0, ?, ?, 0)",
        )
        .bind(&now)
        .bind(&now)
        .execute(&pool)
        .await
        .expect("insert prototype");

        let mut uow = SqliteUnitOfWork::new(&pool).await.expect("create uow");

        let formation = CreateTrainFormationUseCase::execute(
            &mut uow,
            CreateTrainFormationArgs {
                name: "Reorder Test Formation".into(),
                category_id: None,
                start_year: None,
                end_year: None,
                epoch: None,
                notes: None,
            },
        )
        .await
        .expect("create formation");

        let el1 = AddFormationElementUseCase::execute(
            &mut uow,
            formation.id.clone(),
            AddFormationElementArgs {
                prototype_id: "proto-reorder-1".into(),
                owned_rolling_stock_id: None,
            },
        )
        .await
        .expect("add el1");

        let el2 = AddFormationElementUseCase::execute(
            &mut uow,
            formation.id.clone(),
            AddFormationElementArgs {
                prototype_id: "proto-reorder-1".into(),
                owned_rolling_stock_id: None,
            },
        )
        .await
        .expect("add el2");

        // Reverse the order — el2 first, el1 second
        let result = ReorderFormationElementsUseCase::execute(
            &mut uow,
            formation.id.clone(),
            ReorderFormationElementsArgs {
                element_ids: vec![el2.id.clone(), el1.id.clone()],
            },
        )
        .await;

        assert!(result.is_ok(), "valid reorder must succeed: {result:?}");
        let detail = result.unwrap();
        assert_eq!(
            detail.elements[0].id, el2.id,
            "el2 must be first after reorder"
        );
        assert_eq!(
            detail.elements[1].id, el1.id,
            "el1 must be second after reorder"
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// create_custom_prototype
// ─────────────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod create_custom_prototype {
    use crate::core::domain::domain_error::DomainError;
    use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
    use crate::trains::application::create_custom_prototype::CreateCustomPrototypeUseCase;
    use crate::trains::interface::command_args::CreateCustomPrototypeArgs;
    use garde::Validate;

    /// An unrecognised `car_type` must fail `garde` validation at the boundary.
    #[test]
    fn test_create_custom_prototype_invalid_car_type_fails_validation() {
        let args = CreateCustomPrototypeArgs {
            railway_company_id: "trn:railway-company:fs".into(),
            series_code: "Test 1".into(),
            car_type: "FlyingSaucer".into(), // not in the allowed enum
            service_level: None,
            category: "Locomotive".into(),
            is_motorized: true,
            default_is_dummy: false,
            notes: None,
        };
        assert!(
            args.validate().is_err(),
            "invalid car_type must fail garde validation"
        );
    }

    /// Supplying an unknown `railway_company_id` must return an error from the
    /// use case (the repo fails to resolve the company name).
    #[sqlx::test(migrations = "./migrations")]
    async fn test_create_custom_prototype_unknown_company_returns_error(pool: sqlx::SqlitePool) {
        let mut uow = SqliteUnitOfWork::new(&pool).await.expect("create uow");

        let result = CreateCustomPrototypeUseCase::execute(
            &mut uow,
            CreateCustomPrototypeArgs {
                railway_company_id: "trn:railway-company:non-existent".into(),
                series_code: "E.444 Custom".into(),
                car_type: "Locomotive".into(),
                service_level: None,
                category: "Locomotive".into(),
                is_motorized: true,
                default_is_dummy: false,
                notes: None,
            },
        )
        .await;

        assert!(
            matches!(result, Err(DomainError::NotFound { .. })),
            "unknown railway_company_id must return NotFound, got {result:?}"
        );
    }
}
