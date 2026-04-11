/// Application-level Unit of Work abstraction.
///
/// `AppUnitOfWork` is a mega-supertrait combining all domain `*UowExt` traits
/// plus a `commit` lifecycle method.  By storing the factory as
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
use std::future::Future;
use std::pin::Pin;

use crate::budget::domain::BudgetUowExt;
use crate::catalog::domain::manufacturer::ManufacturerUowExt;
use crate::catalog::domain::railway_company::RailwayCompanyUowExt;
use crate::catalog::domain::railway_model::RailwayModelUowExt;
use crate::catalog::domain::railway_model::coupler_repository::CouplerUowExt;
use crate::collecting::domain::CollectionUowExt;
use crate::core::infrastructure::error::CommandError;
use crate::dashboard::domain::DashboardUowExt;
use crate::dcc_inventory::domain::DccInventoryUowExt;
use crate::maintenance::domain::MaintenanceUowExt;
use crate::search::domain::GlobalSearchUowExt;
use crate::sellers::domain::SellersUowExt;
use crate::tracks_inventory::domain::{TrackProductUowExt, TracksInventoryUowExt};
use crate::trains::domain::{TrainsRepository, TrainsUowExt};
use crate::wishlist::domain::WishlistUowExt;

// Repository trait imports (needed for Box<dyn AppUnitOfWork> forwarding impls)
use crate::budget::domain::BudgetRepository;
use crate::catalog::domain::manufacturer::ManufacturerRepository;
use crate::catalog::domain::railway_company::RailwayCompanyRepository;
use crate::catalog::domain::railway_model::RailwayModelRepository;
use crate::catalog::domain::railway_model::coupler_repository::CouplerRepository;
use crate::collecting::domain::CollectionRepository;
use crate::dashboard::domain::DashboardRepository;
use crate::dcc_inventory::domain::DigitalRollingStockRepository;
use crate::maintenance::domain::MaintenanceRepository;
use crate::search::domain::GlobalSearchRepository;
use crate::sellers::domain::SellersRepository;
use crate::tracks_inventory::domain::{TrackInventoryRepository, TrackProductRepository};
use crate::wishlist::domain::repository::WishlistRepository;

/// A composite Unit of Work trait covering all 12 bounded contexts, plus a
/// `commit` lifecycle method that replaces the direct `sqlx::Error`-returning
/// `SqliteUnitOfWork::commit`.
///
/// The `commit` method takes `self: Box<Self>` so that:
/// 1. The type remains object-safe (no generic return).
/// 2. The transaction is consumed after commit (no double-commit).
pub trait AppUnitOfWork:
    BudgetUowExt
    + CollectionUowExt
    + CouplerUowExt
    + DashboardUowExt
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
    fn commit(
        self: Box<Self>,
    ) -> Pin<Box<dyn Future<Output = Result<(), CommandError>> + Send + 'static>>;
}

// ---------------------------------------------------------------------------
// Forwarding impls – enable use cases bound on `U: XxxUowExt` to accept
// `&mut Box<dyn AppUnitOfWork>` as the concrete type `U`.
// ---------------------------------------------------------------------------

impl BudgetUowExt for Box<dyn AppUnitOfWork> {
    fn budget_repo(&mut self) -> Box<dyn BudgetRepository + '_> {
        (**self).budget_repo()
    }
}

impl CouplerUowExt for Box<dyn AppUnitOfWork> {
    fn coupler_repository(&mut self) -> Box<dyn CouplerRepository + '_> {
        (**self).coupler_repository()
    }
}

impl CollectionUowExt for Box<dyn AppUnitOfWork> {
    fn collections_repository(&mut self) -> Box<dyn CollectionRepository + '_> {
        (**self).collections_repository()
    }
}

impl DashboardUowExt for Box<dyn AppUnitOfWork> {
    fn dashboard_repository(&mut self) -> Box<dyn DashboardRepository + '_> {
        (**self).dashboard_repository()
    }
}

impl DccInventoryUowExt for Box<dyn AppUnitOfWork> {
    fn digital_rolling_stocks_repository(&mut self) -> Box<dyn DigitalRollingStockRepository + '_> {
        (**self).digital_rolling_stocks_repository()
    }
}

impl GlobalSearchUowExt for Box<dyn AppUnitOfWork> {
    fn global_search_repo(&mut self) -> Box<dyn GlobalSearchRepository + '_> {
        (**self).global_search_repo()
    }
}

impl MaintenanceUowExt for Box<dyn AppUnitOfWork> {
    fn maintenance_repository(&mut self) -> Box<dyn MaintenanceRepository + Send + '_> {
        (**self).maintenance_repository()
    }
}

impl ManufacturerUowExt for Box<dyn AppUnitOfWork> {
    fn manufacturers_repo(&mut self) -> Box<dyn ManufacturerRepository + '_> {
        (**self).manufacturers_repo()
    }
}

impl RailwayCompanyUowExt for Box<dyn AppUnitOfWork> {
    fn railway_companies_repo(&mut self) -> Box<dyn RailwayCompanyRepository + '_> {
        (**self).railway_companies_repo()
    }
}

impl RailwayModelUowExt for Box<dyn AppUnitOfWork> {
    fn railway_model_repository(&mut self) -> Box<dyn RailwayModelRepository + '_> {
        (**self).railway_model_repository()
    }
}

impl SellersUowExt for Box<dyn AppUnitOfWork> {
    fn sellers_repository(&mut self) -> Box<dyn SellersRepository + '_> {
        (**self).sellers_repository()
    }
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
/// Not a supertrait of `AppUnitOfWork` (to avoid ambiguity with
/// `TracksInventoryUowExt::track_products_repo`), but provided here so that
/// use cases bound only on `U: TrackProductUowExt` compile against
/// `Box<dyn AppUnitOfWork>`.
impl TrackProductUowExt for Box<dyn AppUnitOfWork> {
    fn track_products_repo(&mut self) -> Box<dyn TrackProductRepository + '_> {
        TracksInventoryUowExt::track_products_repo(self)
    }
}

impl TrainsUowExt for Box<dyn AppUnitOfWork> {
    fn trains_repo(&mut self) -> Box<dyn TrainsRepository + '_> {
        (**self).trains_repo()
    }
}

impl WishlistUowExt for Box<dyn AppUnitOfWork> {
    fn wishlist_repository(&mut self) -> Box<dyn WishlistRepository + '_> {
        (**self).wishlist_repository()
    }
}

// ---------------------------------------------------------------------------
// Factory trait
// ---------------------------------------------------------------------------

/// Creates a fresh `AppUnitOfWork` per command-handler invocation.
///
/// The production implementation (`SqliteUowFactory`) starts a SQLite
/// transaction. Tests inject a mock via `AppState::new_with_factory`.
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

    /// A hand-rolled mock `AppUnitOfWork` for interface-layer tests.
    ///
    /// Each domain repo slot defaults to `None`; calling `with_xxx_repo`
    /// configures a pre-built mock.  Accessing a slot that is `None` (or has
    /// already been taken) panics with a descriptive message.
    ///
    /// `commit` is a no-op that returns `Ok(())`.
    #[derive(Default)]
    pub struct MockAppUow {
        budget: Option<Box<dyn BudgetRepository + Send>>,
        collection: Option<Box<dyn CollectionRepository + Send>>,
        coupler: Option<Box<dyn CouplerRepository + Send>>,
        dashboard: Option<Box<dyn DashboardRepository + Send>>,
        dcc_inventory: Option<Box<dyn DigitalRollingStockRepository + Send>>,
        global_search: Option<Box<dyn GlobalSearchRepository + Send>>,
        maintenance: Option<Box<dyn MaintenanceRepository + Send>>,
        manufacturer: Option<Box<dyn ManufacturerRepository + Send>>,
        railway_company: Option<Box<dyn RailwayCompanyRepository + Send>>,
        railway_model: Option<Box<dyn RailwayModelRepository + Send>>,
        sellers: Option<Box<dyn SellersRepository + Send>>,
        track_product: Option<Box<dyn TrackProductRepository + Send>>,
        track_inventory: Option<Box<dyn TrackInventoryRepository + Send>>,
        trains: Option<Box<dyn TrainsRepository + Send>>,
        wishlist: Option<Box<dyn WishlistRepository + Send>>,
    }

    impl MockAppUow {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_budget(mut self, r: impl BudgetRepository + 'static) -> Self {
            self.budget = Some(Box::new(r));
            self
        }
        pub fn with_collection(mut self, r: impl CollectionRepository + 'static) -> Self {
            self.collection = Some(Box::new(r));
            self
        }
        pub fn with_coupler(mut self, r: impl CouplerRepository + 'static) -> Self {
            self.coupler = Some(Box::new(r));
            self
        }
        pub fn with_dashboard(mut self, r: impl DashboardRepository + 'static) -> Self {
            self.dashboard = Some(Box::new(r));
            self
        }
        pub fn with_dcc_inventory(
            mut self,
            r: impl DigitalRollingStockRepository + 'static,
        ) -> Self {
            self.dcc_inventory = Some(Box::new(r));
            self
        }
        pub fn with_global_search(mut self, r: impl GlobalSearchRepository + 'static) -> Self {
            self.global_search = Some(Box::new(r));
            self
        }
        pub fn with_maintenance(mut self, r: impl MaintenanceRepository + Send + 'static) -> Self {
            self.maintenance = Some(Box::new(r));
            self
        }
        pub fn with_manufacturer(mut self, r: impl ManufacturerRepository + 'static) -> Self {
            self.manufacturer = Some(Box::new(r));
            self
        }
        pub fn with_railway_company(mut self, r: impl RailwayCompanyRepository + 'static) -> Self {
            self.railway_company = Some(Box::new(r));
            self
        }
        pub fn with_railway_model(mut self, r: impl RailwayModelRepository + 'static) -> Self {
            self.railway_model = Some(Box::new(r));
            self
        }
        pub fn with_sellers(mut self, r: impl SellersRepository + 'static) -> Self {
            self.sellers = Some(Box::new(r));
            self
        }
        pub fn with_track_product(mut self, r: impl TrackProductRepository + 'static) -> Self {
            self.track_product = Some(Box::new(r));
            self
        }
        pub fn with_track_inventory(mut self, r: impl TrackInventoryRepository + 'static) -> Self {
            self.track_inventory = Some(Box::new(r));
            self
        }
        pub fn with_trains_repo(mut self, r: impl TrainsRepository + 'static) -> Self {
            self.trains = Some(Box::new(r));
            self
        }
        pub fn with_wishlist(mut self, r: impl WishlistRepository + 'static) -> Self {
            self.wishlist = Some(Box::new(r));
            self
        }
    }

    macro_rules! take_or_panic {
        ($opt:expr, $name:literal) => {
            $opt.take()
                .unwrap_or_else(|| panic!("{} not configured or already taken", $name))
        };
    }

    impl BudgetUowExt for MockAppUow {
        fn budget_repo(&mut self) -> Box<dyn BudgetRepository + '_> {
            take_or_panic!(self.budget, "MockAppUow::budget_repo")
        }
    }
    impl CollectionUowExt for MockAppUow {
        fn collections_repository(&mut self) -> Box<dyn CollectionRepository + '_> {
            take_or_panic!(self.collection, "MockAppUow::collections_repository")
        }
    }
    impl CouplerUowExt for MockAppUow {
        fn coupler_repository(&mut self) -> Box<dyn CouplerRepository + '_> {
            take_or_panic!(self.coupler, "MockAppUow::coupler_repository")
        }
    }
    impl DashboardUowExt for MockAppUow {
        fn dashboard_repository(&mut self) -> Box<dyn DashboardRepository + '_> {
            take_or_panic!(self.dashboard, "MockAppUow::dashboard_repository")
        }
    }
    impl DccInventoryUowExt for MockAppUow {
        fn digital_rolling_stocks_repository(
            &mut self,
        ) -> Box<dyn DigitalRollingStockRepository + '_> {
            take_or_panic!(
                self.dcc_inventory,
                "MockAppUow::digital_rolling_stocks_repository"
            )
        }
    }
    impl GlobalSearchUowExt for MockAppUow {
        fn global_search_repo(&mut self) -> Box<dyn GlobalSearchRepository + '_> {
            take_or_panic!(self.global_search, "MockAppUow::global_search_repo")
        }
    }
    impl MaintenanceUowExt for MockAppUow {
        fn maintenance_repository(&mut self) -> Box<dyn MaintenanceRepository + Send + '_> {
            take_or_panic!(self.maintenance, "MockAppUow::maintenance_repository")
        }
    }
    impl ManufacturerUowExt for MockAppUow {
        fn manufacturers_repo(&mut self) -> Box<dyn ManufacturerRepository + '_> {
            take_or_panic!(self.manufacturer, "MockAppUow::manufacturers_repo")
        }
    }
    impl RailwayCompanyUowExt for MockAppUow {
        fn railway_companies_repo(&mut self) -> Box<dyn RailwayCompanyRepository + '_> {
            take_or_panic!(self.railway_company, "MockAppUow::railway_companies_repo")
        }
    }
    impl RailwayModelUowExt for MockAppUow {
        fn railway_model_repository(&mut self) -> Box<dyn RailwayModelRepository + '_> {
            take_or_panic!(self.railway_model, "MockAppUow::railway_model_repository")
        }
    }
    impl SellersUowExt for MockAppUow {
        fn sellers_repository(&mut self) -> Box<dyn SellersRepository + '_> {
            take_or_panic!(self.sellers, "MockAppUow::sellers_repository")
        }
    }
    impl TracksInventoryUowExt for MockAppUow {
        fn track_products_repo(&mut self) -> Box<dyn TrackProductRepository + '_> {
            take_or_panic!(self.track_product, "MockAppUow::track_products_repo")
        }

        fn track_inventories_repo(&mut self) -> Box<dyn TrackInventoryRepository + '_> {
            take_or_panic!(self.track_inventory, "MockAppUow::track_inventories_repo")
        }
    }
    impl TrackProductUowExt for MockAppUow {
        fn track_products_repo(&mut self) -> Box<dyn TrackProductRepository + '_> {
            take_or_panic!(self.track_product, "MockAppUow::track_products_repo")
        }
    }
    impl TrainsUowExt for MockAppUow {
        fn trains_repo(&mut self) -> Box<dyn TrainsRepository + '_> {
            take_or_panic!(self.trains, "MockAppUow::trains_repo")
        }
    }
    impl WishlistUowExt for MockAppUow {
        fn wishlist_repository(&mut self) -> Box<dyn WishlistRepository + '_> {
            take_or_panic!(self.wishlist, "MockAppUow::wishlist_repository")
        }
    }
    impl AppUnitOfWork for MockAppUow {
        fn commit(
            self: Box<Self>,
        ) -> Pin<Box<dyn Future<Output = Result<(), CommandError>> + Send + 'static>> {
            Box::pin(async { Ok(()) })
        }
    }

    /// A single-use factory that returns the pre-built `MockAppUow`.
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
