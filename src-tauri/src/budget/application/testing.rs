use crate::budget::domain::repository::MockBudgetRepository;
use crate::budget::domain::{
    BudgetConfiguration, BudgetMode, BudgetRepository, BudgetUowExt, ExtraBudgetEntry,
    ExtraBudgetId,
};
use crate::core::domain::Currency;
use crate::core::domain::calendar::{Month, Year};
use crate::core::domain::monetary_amount::MonetaryAmount;
use std::collections::VecDeque;

/// A minimal UoW for budget use-case unit tests.
///
/// `BudgetUowExt::budget_repo()` may be called more than once per use-case
/// execution (e.g., one call to read, one to save). This struct holds a queue
/// of mocks and pops one per call, so each test simply enqueues one
/// `MockBudgetRepository` per expected invocation.
#[derive(Default)]
pub struct FakeBudgetUow {
    repos: VecDeque<Box<dyn BudgetRepository + Send>>,
}

impl FakeBudgetUow {
    /// Create an empty UoW.
    pub fn new() -> Self {
        Self {
            repos: VecDeque::new(),
        }
    }

    /// Enqueue a mock repository. Mocks are consumed in FIFO order.
    pub fn with_repo(mut self, repo: MockBudgetRepository) -> Self {
        self.repos.push_back(Box::new(repo));
        self
    }
}

impl BudgetUowExt for FakeBudgetUow {
    fn budget_repo(&mut self) -> Box<dyn BudgetRepository + '_> {
        self.repos
            .pop_front()
            .expect("FakeBudgetUow: no mock repo left in queue — did you enqueue enough mocks?")
    }
}

// ─── Shared test fixtures ────────────────────────────────────────────────────

/// Returns a valid `BudgetConfiguration` suitable for use in tests.
pub fn sample_budget_config() -> BudgetConfiguration {
    BudgetConfiguration::new(
        BudgetMode::Monthly,
        MonetaryAmount::new(100_000, Currency::EUR),
    )
}

/// Returns a valid `ExtraBudgetEntry` suitable for use in tests.
pub fn sample_extra_budget_entry() -> ExtraBudgetEntry {
    ExtraBudgetEntry {
        id: ExtraBudgetId::default(),
        year: Year::try_from(2026).unwrap(),
        month: Month::try_from(4).unwrap(),
        amount: MonetaryAmount::new(5_000, Currency::EUR),
        reason: Some("Birthday gift".to_string()),
        created_at: chrono::Utc::now(),
        version: 0,
    }
}

/// Returns a valid `ExtraBudgetId` suitable for use in tests.
pub fn sample_extra_budget_id() -> ExtraBudgetId {
    ExtraBudgetId::default()
}
