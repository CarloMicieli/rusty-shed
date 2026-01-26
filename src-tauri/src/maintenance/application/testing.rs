use crate::maintenance::domain::{
    MaintenanceRepository, MaintenanceUowExt, MockMaintenanceRepository,
};

#[derive(Default)]
pub struct FakeUow {
    repo: Option<MockMaintenanceRepository>,
}

impl FakeUow {
    pub fn new(repo: MockMaintenanceRepository) -> Self {
        Self { repo: Some(repo) }
    }

    pub fn with_repo(repo: MockMaintenanceRepository) -> Self {
        Self { repo: Some(repo) }
    }
}

impl MaintenanceUowExt for FakeUow {
    fn maintenance_repository(&mut self) -> Box<dyn MaintenanceRepository + Send + '_> {
        Box::new(
            self.repo
                .take()
                .expect("maintenance repository already taken"),
        )
    }
}
