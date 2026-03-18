/// Integration tests for the tracks_inventory use cases.
///
/// Moved here from the application layer because these tests require a real
/// `SqliteUnitOfWork`, which is an infrastructure concern. Application-layer
/// tests should use mock/fake UoW implementations; tests that need a real
/// database belong in the infrastructure layer.
#[cfg(test)]
mod create_track_inventory {
    use crate::core::domain::test_utils::MockIdProvider;
    use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
    use crate::tracks_inventory::application::{
        CreateTrackInventoryUseCase, NewTrackInventoryInput,
    };
    use crate::tracks_inventory::domain::{TrackInventoryId, TracksInventoryUowExt};

    #[sqlx::test(migrations = "./migrations")]
    async fn it_should_create_inventory(pool: sqlx::SqlitePool) {
        let mut unit_of_work = SqliteUnitOfWork::new(&pool)
            .await
            .expect("should create unit of work");

        let name = "My Inventory".to_string();
        let description = Some("Created by test".to_string());

        let fixed_id =
            TrackInventoryId::try_from("trn:track-inventory:00000000-0000-0000-0000-000000000002")
                .unwrap();
        let id_provider = MockIdProvider::new(fixed_id.clone());

        let input = NewTrackInventoryInput {
            name: name.clone(),
            description: description.clone(),
        };

        let id = CreateTrackInventoryUseCase::execute(&mut unit_of_work, id_provider, input)
            .await
            .expect("create should succeed");

        let mut repo = unit_of_work.track_inventories_repo();
        let reloaded = repo
            .find_by_id(&id)
            .await
            .expect("find")
            .expect("inventory exists");

        assert_eq!(reloaded.name, name);
        assert_eq!(reloaded.description, description);
    }
}

#[cfg(test)]
mod rename_track_inventory {
    use crate::core::domain::IdProvider;
    use crate::core::domain::domain_error::DomainError;
    use crate::core::domain::test_utils::{DefaultMockIdProvider, MockIdProvider};
    use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
    use crate::tracks_inventory::application::{
        RenameTrackInventoryInput, RenameTrackInventoryUseCase,
    };
    use crate::tracks_inventory::domain::{TrackInventoryId, TracksInventoryUowExt};

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_tracks_inventory.sql")
    )]
    async fn it_should_rename_inventory(pool: sqlx::SqlitePool) {
        let mut unit_of_work = SqliteUnitOfWork::new(&pool)
            .await
            .expect("should create unit of work");

        let fixed_id =
            TrackInventoryId::try_from("trn:track-inventory:00000000-0000-0000-0000-000000000001")
                .unwrap();
        let id_provider = MockIdProvider::new(fixed_id.clone());
        let inventory_id = id_provider.next_id();

        let new_name = "Renamed Inventory".to_string();

        let input = RenameTrackInventoryInput {
            id: inventory_id.clone(),
            new_name: new_name.clone(),
        };

        RenameTrackInventoryUseCase::execute(&mut unit_of_work, input)
            .await
            .expect("rename should succeed");

        let mut repo = unit_of_work.track_inventories_repo();
        let reloaded = repo
            .find_by_id(&inventory_id)
            .await
            .expect("find")
            .expect("inventory exists");

        assert_eq!(reloaded.name, new_name);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn it_should_return_not_found_when_inventory_missing(pool: sqlx::SqlitePool) {
        let mut unit_of_work = SqliteUnitOfWork::new(&pool)
            .await
            .expect("should create unit of work");

        let missing_provider = DefaultMockIdProvider::default();
        let missing_id: TrackInventoryId = missing_provider.value();

        let input = RenameTrackInventoryInput {
            id: missing_id.clone(),
            new_name: "No such inventory".to_string(),
        };

        let res = RenameTrackInventoryUseCase::execute(&mut unit_of_work, input).await;

        assert!(matches!(res, Err(DomainError::NotFound { .. })));
    }
}

#[cfg(test)]
mod delete_track_inventory {
    use crate::core::domain::domain_error::DomainError;
    use crate::core::domain::test_utils::MockIdProvider;
    use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
    use crate::tracks_inventory::application::{
        CreateTrackInventoryUseCase, DeleteTrackInventoryUseCase, NewTrackInventoryInput,
    };
    use crate::tracks_inventory::domain::{TrackInventoryId, TracksInventoryUowExt};

    #[sqlx::test(migrations = "./migrations")]
    async fn it_should_delete_inventory(pool: sqlx::SqlitePool) {
        let mut unit_of_work = SqliteUnitOfWork::new(&pool).await.unwrap();

        let fixed_id =
            TrackInventoryId::try_from("trn:track-inventory:00000000-0000-0000-0000-000000000001")
                .unwrap();
        let id_provider = MockIdProvider::new(fixed_id.clone());

        let input = NewTrackInventoryInput {
            name: "Test Inventory".to_string(),
            description: Some("To be deleted".to_string()),
        };

        let id = CreateTrackInventoryUseCase::execute(&mut unit_of_work, id_provider, input)
            .await
            .unwrap();

        unit_of_work.commit().await.unwrap();

        let mut unit_of_work = SqliteUnitOfWork::new(&pool).await.unwrap();
        let result = DeleteTrackInventoryUseCase::execute(&mut unit_of_work, &id).await;
        assert!(result.is_ok());

        unit_of_work.commit().await.unwrap();

        let mut unit_of_work = SqliteUnitOfWork::new(&pool).await.unwrap();
        let mut repo = unit_of_work.track_inventories_repo();
        let reloaded = repo.find_by_id(&id).await.unwrap();
        assert!(reloaded.is_none());
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn it_should_fail_for_nonexistent_inventory(pool: sqlx::SqlitePool) {
        let mut unit_of_work = SqliteUnitOfWork::new(&pool).await.unwrap();

        let fake_id =
            TrackInventoryId::try_from("trn:track-inventory:00000000-0000-0000-0000-999999999999")
                .unwrap();

        let result = DeleteTrackInventoryUseCase::execute(&mut unit_of_work, &fake_id).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DomainError::NotFound { .. }));
    }
}

#[cfg(test)]
mod add_purchase {
    use crate::core::domain::currency::Currency;
    use crate::core::domain::monetary_amount::MonetaryAmount;
    use crate::core::domain::test_utils::MockIdProvider;
    use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
    use crate::sellers::domain::seller_id::SellerId;
    use crate::tracks_inventory::application::{AddTrackPurchaseInput, AddTrackPurchaseUseCase};
    use crate::tracks_inventory::domain::{
        TrackId, TrackInventoryId, TrackPurchaseId, TracksInventoryUowExt,
    };
    use chrono::NaiveDate;
    use pretty_assertions::assert_eq;

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_tracks_inventory.sql")
    )]
    async fn it_should_add_purchase(pool: sqlx::SqlitePool) {
        let mut unit_of_work = SqliteUnitOfWork::new(&pool)
            .await
            .expect("should create unit of work");

        let inventory_id =
            TrackInventoryId::try_from("trn:track-inventory:00000000-0000-0000-0000-000000000001")
                .unwrap();

        let input = AddTrackPurchaseInput {
            id: inventory_id.clone(),
            track_id: TrackId::try_from("trn:track:acme:60100").unwrap(),
            quantity: 2,
            price: MonetaryAmount::new(1234, Currency::EUR),
            seller_id: Some(SellerId::try_from("trn:seller:model-train-shop").unwrap()),
            purchase_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        };

        let fixed_purchase_id =
            TrackPurchaseId::try_from("trn:track-purchase:00000000-0000-0000-0000-000000000009")
                .unwrap();
        let purchase_id_provider = MockIdProvider::new(fixed_purchase_id.clone());

        let returned =
            AddTrackPurchaseUseCase::execute(&mut unit_of_work, purchase_id_provider, input)
                .await
                .expect("execute should succeed");

        assert_eq!(returned, fixed_purchase_id);

        let mut repo = unit_of_work.track_inventories_repo();
        let reloaded = repo
            .find_by_id(&inventory_id)
            .await
            .expect("find")
            .expect("inventory exists");

        assert!(
            reloaded
                .purchase_history
                .iter()
                .any(|p| p.track_id.to_string() == "trn:track:acme:60100" && p.quantity == 2)
        );
    }
}

#[cfg(test)]
mod set_item_quantity {
    use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
    use crate::tracks_inventory::application::{
        SetTrackItemQuantityInput, SetTrackItemQuantityUseCase,
    };
    use crate::tracks_inventory::domain::{TrackId, TrackInventoryId, TracksInventoryUowExt};
    use pretty_assertions::assert_eq;

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_tracks_inventory.sql")
    )]
    async fn it_should_set_item_quantity(pool: sqlx::SqlitePool) {
        let mut unit_of_work = SqliteUnitOfWork::new(&pool)
            .await
            .expect("should create unit of work");

        let inventory_id =
            TrackInventoryId::try_from("trn:track-inventory:00000000-0000-0000-0000-000000000001")
                .unwrap();

        let track_id = TrackId::try_from("trn:track:acme:60100").unwrap();

        let input = SetTrackItemQuantityInput {
            inventory_id: inventory_id.clone(),
            track_id: track_id.clone(),
            quantity: 5,
        };

        SetTrackItemQuantityUseCase::execute(&mut unit_of_work, input)
            .await
            .expect("set quantity should succeed");

        let mut repo = unit_of_work.track_inventories_repo();
        let reloaded = repo
            .find_by_id(&inventory_id)
            .await
            .expect("find")
            .expect("inventory exists");

        let qty = reloaded
            .inventory
            .get(&track_id)
            .map(|t| t.quantity)
            .unwrap_or_default();

        assert_eq!(qty, 5);
    }
}

#[cfg(test)]
mod get_track_inventories {
    use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
    use crate::tracks_inventory::application::GetTrackInventoriesQuery;

    #[sqlx::test(migrations = "./migrations")]
    async fn list_inventories_empty(pool: sqlx::SqlitePool) {
        let mut uow = SqliteUnitOfWork::new(&pool).await.unwrap();
        let result = GetTrackInventoriesQuery::execute(&mut uow).await.unwrap();
        assert!(result.is_empty());
    }
}

#[cfg(test)]
mod get_track_products {
    use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
    use crate::tracks_inventory::application::GetTrackProductsQuery;

    #[sqlx::test(migrations = "./migrations")]
    async fn list_products_empty(pool: sqlx::SqlitePool) {
        let mut uow = SqliteUnitOfWork::new(&pool).await.unwrap();
        let result = GetTrackProductsQuery::execute(&mut uow).await.unwrap();
        assert!(result.is_empty());
    }
}
