use crate::core::infrastructure::error::CommandError;
use crate::dashboard::application::GetDashboardSummary;
use crate::dashboard::domain::DashboardSummary;
use crate::state::AppState;
use serde::Deserialize;
use tracing::info;

const DEFAULT_RECENT_ITEMS: u8 = 4;

/// Query criteria for retrieving the dashboard summary.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct QueryCriteria {
    /// Number of recent items to retrieve for the dashboard.
    pub number_of_recent_items: Option<u8>,
}

/// Default values for QueryCriteria.
impl Default for QueryCriteria {
    fn default() -> Self {
        Self {
            number_of_recent_items: Some(DEFAULT_RECENT_ITEMS),
        }
    }
}

/// Retrieve the dashboard summary.
pub async fn get_dashboard_summary_inner(
    state: &AppState,
    criteria: Option<QueryCriteria>,
) -> Result<DashboardSummary, CommandError> {
    info!("Fetching dashboard summary");
    let criteria = criteria.unwrap_or_default();
    let number_of_recent_items = criteria
        .number_of_recent_items
        .unwrap_or(DEFAULT_RECENT_ITEMS);
    let mut uow = state.unit_of_work().await?;
    let dashboard_summary = GetDashboardSummary::execute(
        &mut uow,
        number_of_recent_items,
        state.models_dir().to_path_buf(),
    )
    .await?;
    uow.commit().await?;
    Ok(dashboard_summary)
}

/// Tauri command to retrieve the dashboard summary.
#[tauri::command]
#[specta::specta]
pub async fn get_dashboard_summary(
    state: tauri::State<'_, AppState>,
    criteria: Option<QueryCriteria>,
) -> Result<DashboardSummary, CommandError> {
    get_dashboard_summary_inner(&state, criteria).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_uow::testing::{MockAppUow, OneShotFactory};
    use crate::core::domain::domain_error::DomainError;
    use crate::dashboard::domain::{DashboardTotals, MockDashboardRepository};
    use sqlx::SqlitePool;
    use std::sync::Arc;

    fn empty_summary() -> DashboardSummary {
        DashboardSummary {
            totals: DashboardTotals {
                collection_items: 0,
                wishlists: 0,
                maintenance_due: 0,
                total_value: None,
            },
            recent_items: vec![],
            purchase_groups: vec![],
        }
    }

    async fn state_with_repo(mock_repo: MockDashboardRepository) -> AppState {
        let pool = SqlitePool::connect("sqlite::memory:")
            .await
            .expect("in-memory sqlite should connect");
        let uow = MockAppUow::new().with_dashboard(mock_repo);
        let factory = Arc::new(OneShotFactory::new(uow));
        AppState::new_with_factory(pool, factory)
    }

    #[tokio::test]
    async fn get_dashboard_summary_inner_uses_default_recent_items_when_criteria_is_none() {
        let mut repo = MockDashboardRepository::new();
        repo.expect_find_summary()
            .once()
            .withf(|params| {
                params.number_of_recent_items == DEFAULT_RECENT_ITEMS
                    && params.models_dir.as_os_str().is_empty()
            })
            .returning(|_| Ok(empty_summary()));

        let state = state_with_repo(repo).await;

        let result = get_dashboard_summary_inner(&state, None).await;

        assert!(result.is_ok(), "{result:?}");
    }

    #[tokio::test]
    async fn get_dashboard_summary_inner_uses_criteria_value_when_present() {
        let mut repo = MockDashboardRepository::new();
        repo.expect_find_summary()
            .once()
            .withf(|params| params.number_of_recent_items == 9)
            .returning(|_| Ok(empty_summary()));

        let state = state_with_repo(repo).await;
        let criteria = Some(QueryCriteria {
            number_of_recent_items: Some(9),
        });

        let result = get_dashboard_summary_inner(&state, criteria).await;

        assert!(result.is_ok(), "{result:?}");
    }

    #[tokio::test]
    async fn get_dashboard_summary_inner_falls_back_when_criteria_count_is_none() {
        let mut repo = MockDashboardRepository::new();
        repo.expect_find_summary()
            .once()
            .withf(|params| params.number_of_recent_items == DEFAULT_RECENT_ITEMS)
            .returning(|_| Ok(empty_summary()));

        let state = state_with_repo(repo).await;
        let criteria = Some(QueryCriteria {
            number_of_recent_items: None,
        });

        let result = get_dashboard_summary_inner(&state, criteria).await;

        assert!(result.is_ok(), "{result:?}");
    }

    #[tokio::test]
    async fn get_dashboard_summary_inner_maps_domain_infrastructure_error() {
        let mut repo = MockDashboardRepository::new();
        repo.expect_find_summary().once().returning(|_| {
            Err(DomainError::Infrastructure(
                "dashboard query failed".to_string(),
            ))
        });

        let state = state_with_repo(repo).await;

        let result = get_dashboard_summary_inner(&state, None).await;

        assert!(
            matches!(result, Err(CommandError::DatabaseError(ref msg)) if msg.contains("dashboard query failed")),
            "{result:?}"
        );
    }
}
