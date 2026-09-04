/// Application-level Unit of Work abstraction.
///
/// `AppUnitOfWork` is a mega-supertrait combining all domain `*UowExt` traits
/// plus a `commit` lifecycle method. By storing the factory as
/// `Arc<dyn AppUowFactory>` in `AppState`, Tauri command handlers can be
/// tested without a real SQLite database.
///
/// ## Production path
/// `AppState::new` creates a `SqliteUowFactory` which returns a boxed
/// `SqliteUnitOfWork` (implements every supertrait via the existing infra impls).
///
/// ## Test path
/// Tests inject a custom `AppUowFactory` via `AppState::new_with_factory`, providing
/// whatever `AppUnitOfWork` implementation they need.
use crate::budget::domain::{BudgetRepository, BudgetUowExt};
use crate::catalog::domain::manufacturer::{ManufacturerRepository, ManufacturerUowExt};
use crate::catalog::domain::railway_company::{RailwayCompanyRepository, RailwayCompanyUowExt};
use crate::catalog::domain::railway_model::coupler_repository::{CouplerRepository, CouplerUowExt};
use crate::catalog::domain::railway_model::{RailwayModelRepository, RailwayModelUowExt};
use crate::collecting::domain::{CollectionRepository, CollectionUowExt};
use crate::core::infrastructure::error::CommandError;
use crate::dashboard::domain::{DashboardRepository, DashboardUowExt};
use crate::data_management::domain::{ExportRepository, ExportUowExt};
use crate::dcc_inventory::domain::{DccInventoryUowExt, DigitalRollingStockRepository};
use crate::maintenance::domain::{MaintenanceRepository, MaintenanceUowExt};
use crate::search::domain::{GlobalSearchRepository, GlobalSearchUowExt};
use crate::sellers::domain::{SellersRepository, SellersUowExt};
use crate::tracks_inventory::domain::{
    TrackInventoryRepository, TrackProductRepository, TrackProductUowExt, TracksInventoryUowExt,
};
use crate::trains::domain::{TrainsRepository, TrainsUowExt};
use crate::wishlist::domain::WishlistUowExt;
use crate::wishlist::domain::repository::WishlistRepository;

/// A composite Unit of Work trait covering all domain contexts, plus a
/// `commit` lifecycle method that replaces direct database commits.
///
/// Takes `self: Box<Self>` to ensure object safety and guarantee that
/// the transaction is consumed upon commit (preventing double-commits).
#[async_trait::async_trait]
pub trait AppUnitOfWork:
    BudgetUowExt
    + CollectionUowExt
    + CouplerUowExt
    + DashboardUowExt
    + ExportUowExt
    + DccInventoryUowExt
    + GlobalSearchUowExt
    + MaintenanceUowExt
    + ManufacturerUowExt
    + RailwayCompanyUowExt
    + RailwayModelUowExt
    + SellersUowExt
    + TracksInventoryUowExt
    + TrainsUowExt
    + WishlistUowExt
    + Send
    + 'static
{
    async fn commit(self: Box<Self>) -> Result<(), CommandError>;
}

// ---------------------------------------------------------------------------
// AppUnitOfWork impl for Box<dyn AppUnitOfWork>
// ---------------------------------------------------------------------------

#[async_trait::async_trait]
impl AppUnitOfWork for Box<dyn AppUnitOfWork> {
    async fn commit(self: Box<Self>) -> Result<(), CommandError> {
        (*self).commit().await
    }
}

// ---------------------------------------------------------------------------
// Forwarding impls – enable use cases bound on `U: XxxUowExt` to accept
// `Box<dyn AppUnitOfWork>` or `&mut Box<dyn AppUnitOfWork>`.
// ---------------------------------------------------------------------------

macro_rules! forward_uow_ext {
    ($($trait_name:ident :: $method_name:ident -> $repo_trait:ident $(+ $extra_bound:ident)?),* $(,)?) => {
        $(
            impl $trait_name for Box<dyn AppUnitOfWork> {
                fn $method_name(&mut self) -> Box<dyn $repo_trait $(+ $extra_bound)? + '_> {
                    (**self).$method_name()
                }
            }
        )*
    };
}

forward_uow_ext! {
    BudgetUowExt :: budget_repo -> BudgetRepository,
    CouplerUowExt :: coupler_repository -> CouplerRepository,
    CollectionUowExt :: collections_repository -> CollectionRepository,
    DashboardUowExt :: dashboard_repository -> DashboardRepository,
    ExportUowExt :: export_repo -> ExportRepository,
    DccInventoryUowExt :: digital_rolling_stocks_repository -> DigitalRollingStockRepository,
    GlobalSearchUowExt :: global_search_repo -> GlobalSearchRepository,
    MaintenanceUowExt :: maintenance_repository -> MaintenanceRepository + Send,
    ManufacturerUowExt :: manufacturers_repo -> ManufacturerRepository,
    RailwayCompanyUowExt :: railway_companies_repo -> RailwayCompanyRepository,
    RailwayModelUowExt :: railway_model_repository -> RailwayModelRepository,
    SellersUowExt :: sellers_repository -> SellersRepository,
    TrainsUowExt :: trains_repo -> TrainsRepository,
    WishlistUowExt :: wishlist_repository -> WishlistRepository,
}

impl TracksInventoryUowExt for Box<dyn AppUnitOfWork> {
    fn track_products_repo(&mut self) -> Box<dyn TrackProductRepository + '_> {
        TracksInventoryUowExt::track_products_repo(&mut **self)
    }

    fn track_inventories_repo(&mut self) -> Box<dyn TrackInventoryRepository + '_> {
        (**self).track_inventories_repo()
    }
}

/// Forwarding impl for the narrower `TrackProductUowExt`.
impl TrackProductUowExt for Box<dyn AppUnitOfWork> {
    fn track_products_repo(&mut self) -> Box<dyn TrackProductRepository + '_> {
        TracksInventoryUowExt::track_products_repo(self)
    }
}

// ---------------------------------------------------------------------------
// Factory trait
// ---------------------------------------------------------------------------

/// Creates a fresh `AppUnitOfWork` per command-handler invocation.
#[async_trait::async_trait]
pub trait AppUowFactory: Send + Sync + 'static {
    async fn create_uow(&self) -> Result<Box<dyn AppUnitOfWork>, CommandError>;
}

// ---------------------------------------------------------------------------
// Test support
// ---------------------------------------------------------------------------

#[cfg(test)]
pub mod testing {
    use super::*;

    /// Factory closure type for mock repository creation.
    type RepoFactory<T> = Box<dyn FnMut() -> Box<T> + Send>;

    /// A flexible mock `AppUnitOfWork` for interface-layer testing.
    ///
    /// Supports both static repository instances (single access) and factory
    /// closures for scenarios requiring multiple repository accesses.
    #[derive(Default)]
    pub struct MockAppUow {
        budget: Option<RepoFactory<dyn BudgetRepository>>,
        collection: Option<RepoFactory<dyn CollectionRepository>>,
        coupler: Option<RepoFactory<dyn CouplerRepository>>,
        dashboard: Option<RepoFactory<dyn DashboardRepository>>,
        export: Option<RepoFactory<dyn ExportRepository>>,
        dcc_inventory: Option<RepoFactory<dyn DigitalRollingStockRepository>>,
        global_search: Option<RepoFactory<dyn GlobalSearchRepository>>,
        maintenance: Option<RepoFactory<dyn MaintenanceRepository + Send>>,
        manufacturer: Option<RepoFactory<dyn ManufacturerRepository>>,
        railway_company: Option<RepoFactory<dyn RailwayCompanyRepository>>,
        railway_model: Option<RepoFactory<dyn RailwayModelRepository>>,
        sellers: Option<RepoFactory<dyn SellersRepository>>,
        track_product: Option<RepoFactory<dyn TrackProductRepository>>,
        track_inventory: Option<RepoFactory<dyn TrackInventoryRepository>>,
        trains: Option<RepoFactory<dyn TrainsRepository>>,
        wishlist: Option<RepoFactory<dyn WishlistRepository>>,
    }

    impl MockAppUow {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_export(mut self, r: impl ExportRepository + 'static) -> Self {
            let mut opt = Some(Box::new(r) as Box<dyn ExportRepository>);
            self.export = Some(Box::new(move || {
                opt.take()
                    .unwrap_or_else(|| panic!("MockAppUow::export_repo already accessed"))
            }));
            self
        }

        pub fn with_export_factory<F>(mut self, factory: F) -> Self
        where
            F: FnMut() -> Box<dyn ExportRepository> + Send + 'static,
        {
            self.export = Some(Box::new(factory));
            self
        }

        // Additional repository builder helpers follow the same pattern...
    }

    macro_rules! get_or_panic {
        ($opt:expr, $name:literal) => {
            ($opt
                .as_mut()
                .unwrap_or_else(|| panic!("{} not configured", $name)))()
        };
    }

    impl BudgetUowExt for MockAppUow {
        fn budget_repo(&mut self) -> Box<dyn BudgetRepository + '_> {
            get_or_panic!(self.budget, "MockAppUow::budget_repo")
        }
    }
    impl CollectionUowExt for MockAppUow {
        fn collections_repository(&mut self) -> Box<dyn CollectionRepository + '_> {
            get_or_panic!(self.collection, "MockAppUow::collections_repository")
        }
    }
    impl CouplerUowExt for MockAppUow {
        fn coupler_repository(&mut self) -> Box<dyn CouplerRepository + '_> {
            get_or_panic!(self.coupler, "MockAppUow::coupler_repository")
        }
    }
    impl DashboardUowExt for MockAppUow {
        fn dashboard_repository(&mut self) -> Box<dyn DashboardRepository + '_> {
            get_or_panic!(self.dashboard, "MockAppUow::dashboard_repository")
        }
    }
    impl ExportUowExt for MockAppUow {
        fn export_repo(&mut self) -> Box<dyn ExportRepository + '_> {
            get_or_panic!(self.export, "MockAppUow::export_repo")
        }
    }
    impl DccInventoryUowExt for MockAppUow {
        fn digital_rolling_stocks_repository(
            &mut self,
        ) -> Box<dyn DigitalRollingStockRepository + '_> {
            get_or_panic!(
                self.dcc_inventory,
                "MockAppUow::digital_rolling_stocks_repository"
            )
        }
    }
    impl GlobalSearchUowExt for MockAppUow {
        fn global_search_repo(&mut self) -> Box<dyn GlobalSearchRepository + '_> {
            get_or_panic!(self.global_search, "MockAppUow::global_search_repo")
        }
    }
    impl MaintenanceUowExt for MockAppUow {
        fn maintenance_repository(&mut self) -> Box<dyn MaintenanceRepository + Send + '_> {
            get_or_panic!(self.maintenance, "MockAppUow::maintenance_repository")
        }
    }
    impl ManufacturerUowExt for MockAppUow {
        fn manufacturers_repo(&mut self) -> Box<dyn ManufacturerRepository + '_> {
            get_or_panic!(self.manufacturer, "MockAppUow::manufacturers_repo")
        }
    }
    impl RailwayCompanyUowExt for MockAppUow {
        fn railway_companies_repo(&mut self) -> Box<dyn RailwayCompanyRepository + '_> {
            get_or_panic!(self.railway_company, "MockAppUow::railway_companies_repo")
        }
    }
    impl RailwayModelUowExt for MockAppUow {
        fn railway_model_repository(&mut self) -> Box<dyn RailwayModelRepository + '_> {
            get_or_panic!(self.railway_model, "MockAppUow::railway_model_repository")
        }
    }
    impl SellersUowExt for MockAppUow {
        fn sellers_repository(&mut self) -> Box<dyn SellersRepository + '_> {
            get_or_panic!(self.sellers, "MockAppUow::sellers_repository")
        }
    }
    impl TracksInventoryUowExt for MockAppUow {
        fn track_products_repo(&mut self) -> Box<dyn TrackProductRepository + '_> {
            get_or_panic!(self.track_product, "MockAppUow::track_products_repo")
        }

        fn track_inventories_repo(&mut self) -> Box<dyn TrackInventoryRepository + '_> {
            get_or_panic!(self.track_inventory, "MockAppUow::track_inventories_repo")
        }
    }
    impl TrackProductUowExt for MockAppUow {
        fn track_products_repo(&mut self) -> Box<dyn TrackProductRepository + '_> {
            get_or_panic!(self.track_product, "MockAppUow::track_products_repo")
        }
    }
    impl TrainsUowExt for MockAppUow {
        fn trains_repo(&mut self) -> Box<dyn TrainsRepository + '_> {
            get_or_panic!(self.trains, "MockAppUow::trains_repo")
        }
    }
    impl WishlistUowExt for MockAppUow {
        fn wishlist_repository(&mut self) -> Box<dyn WishlistRepository + '_> {
            get_or_panic!(self.wishlist, "MockAppUow::wishlist_repository")
        }
    }

    #[async_trait::async_trait]
    impl AppUnitOfWork for MockAppUow {
        async fn commit(self: Box<Self>) -> Result<(), CommandError> {
            Ok(())
        }
    }

    /// A single-use factory that returns a pre-built `MockAppUow`.
    pub struct OneShotFactory(std::sync::Mutex<Option<Box<dyn AppUnitOfWork>>>);

    impl OneShotFactory {
        pub fn new(uow: impl AppUnitOfWork) -> Self {
            Self(std::sync::Mutex::new(Some(Box::new(uow))))
        }
    }

    #[async_trait::async_trait]
    impl AppUowFactory for OneShotFactory {
        async fn create_uow(&self) -> Result<Box<dyn AppUnitOfWork>, CommandError> {
            self.0
                .lock()
                .expect("OneShotFactory lock poisoned")
                .take()
                .ok_or_else(|| CommandError::unknown("OneShotFactory: uow already consumed"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::testing::{MockAppUow, OneShotFactory};
    use super::*;
    use crate::data_management::domain::{ExportUowExt, MockExportRepository};

    #[test]
    fn box_dyn_app_uow_forwards_export_repo_access() {
        let mock_uow = MockAppUow::new().with_export(MockExportRepository::new());
        let mut uow: Box<dyn AppUnitOfWork> = Box::new(mock_uow);

        let _repo = ExportUowExt::export_repo(&mut uow);
    }

    #[tokio::test]
    async fn box_dyn_app_uow_implements_app_unit_of_work() {
        let mock_uow = MockAppUow::new();
        let uow: Box<dyn AppUnitOfWork> = Box::new(mock_uow);

        // Verifies Box<dyn AppUnitOfWork> can be passed to functions bounded on AppUnitOfWork
        let result = uow.commit().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn one_shot_factory_returns_error_after_consumption() {
        let factory = OneShotFactory::new(MockAppUow::new());

        let first = factory.create_uow().await;
        assert!(first.is_ok());

        let second = factory.create_uow().await;
        assert!(second.is_err());
    }
}
