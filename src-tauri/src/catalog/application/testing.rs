use crate::catalog::domain::manufacturer::{
    ManufacturerRepository, ManufacturerUowExt, MockManufacturerRepository,
};
use crate::catalog::domain::railway_company::{
    MockRailwayCompanyRepository, RailwayCompanyRepository, RailwayCompanyUowExt,
};
use crate::catalog::domain::railway_model::{
    MockRailwayModelRepository, RailwayModelRepository, RailwayModelUowExt,
};

#[derive(Default)]
pub struct FakeUow {
    manufacturers_repo: Option<MockManufacturerRepository>,
    railway_companies_repo: Option<MockRailwayCompanyRepository>,
    railway_models_repo: Option<MockRailwayModelRepository>,
}

impl FakeUow {
    pub fn with_manufacturers_repo(manufacturers_repo: MockManufacturerRepository) -> Self {
        Self {
            manufacturers_repo: Some(manufacturers_repo),
            ..Default::default()
        }
    }

    pub fn with_railway_companies_repo(
        railway_companies_repo: MockRailwayCompanyRepository,
    ) -> Self {
        Self {
            railway_companies_repo: Some(railway_companies_repo),
            ..Default::default()
        }
    }

    pub fn with_railway_models_repo(railway_models_repo: MockRailwayModelRepository) -> Self {
        Self {
            railway_models_repo: Some(railway_models_repo),
            ..Default::default()
        }
    }

    pub fn with_company_and_model_repos(
        railway_companies_repo: MockRailwayCompanyRepository,
        railway_models_repo: MockRailwayModelRepository,
    ) -> Self {
        Self {
            manufacturers_repo: None,
            railway_companies_repo: Some(railway_companies_repo),
            railway_models_repo: Some(railway_models_repo),
        }
    }
}

impl ManufacturerUowExt for FakeUow {
    fn manufacturers_repo(&mut self) -> Box<dyn ManufacturerRepository + '_> {
        Box::new(
            self.manufacturers_repo
                .take()
                .expect("manufacturer repository already taken"),
        )
    }
}

impl RailwayCompanyUowExt for FakeUow {
    fn railway_companies_repo(&mut self) -> Box<dyn RailwayCompanyRepository + '_> {
        Box::new(
            self.railway_companies_repo
                .take()
                .expect("railway company repository already taken"),
        )
    }
}

impl RailwayModelUowExt for FakeUow {
    fn railway_model_repository(&mut self) -> Box<dyn RailwayModelRepository + '_> {
        Box::new(
            self.railway_models_repo
                .take()
                .expect("railway model repository already taken"),
        )
    }
}
