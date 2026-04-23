use crate::core::domain::domain_error::DomainError;
use crate::trains::domain::{PrototypeGroupView, TrainsUowExt};

pub struct GetPrototypesUseCase;

impl GetPrototypesUseCase {
    pub async fn execute<U: TrainsUowExt + Send>(
        uow: &mut U,
        query: Option<String>,
    ) -> Result<Vec<PrototypeGroupView>, DomainError> {
        uow.trains_repo().find_prototypes_by_query(query).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_uow::testing::MockAppUow;
    use crate::core::domain::domain_error::DomainError;
    use crate::trains::domain::repository::MockTrainsRepository;
    use crate::trains::domain::views::{PrototypeGroupView, PrototypeView};

    fn make_group() -> PrototypeGroupView {
        PrototypeGroupView {
            railway_company_id: "co-1".into(),
            company_name: "SBB".into(),
            prototypes: vec![PrototypeView {
                id: "pt-1".into(),
                railway_company_id: "co-1".into(),
                company_name: "SBB".into(),
                series_code: "Re 460".into(),
                friendly_name: Some("Lok 2000".into()),
                is_motorized: true,
                default_is_dummy: false,
                is_custom: false,
                specification_type: "LOCOMOTIVE".into(),
                locomotive_type: Some("ELECTRIC_LOCOMOTIVE".into()),
                locomotive_series: None,
                service_level: None,
                passenger_car_type: None,
                freight_car_type: None,
                railcar_type: None,
                electric_multiple_unit_type: None,
                elements_count: None,
                is_permanently_coupled: None,
            }],
        }
    }

    #[tokio::test]
    async fn returns_grouped_prototypes() {
        let mut repo = MockTrainsRepository::new();
        repo.expect_find_prototypes_by_query()
            .times(1)
            .return_once(|_| Ok(vec![make_group()]));

        let mut uow = MockAppUow::new().with_trains_repo(repo);
        let result = GetPrototypesUseCase::execute(&mut uow, Some("re".into())).await;

        assert!(result.is_ok(), "{result:?}");
        assert_eq!(result.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn repo_error_propagates() {
        let mut repo = MockTrainsRepository::new();
        repo.expect_find_prototypes_by_query()
            .times(1)
            .returning(|_| Err(DomainError::Infrastructure("db down".into())));

        let mut uow = MockAppUow::new().with_trains_repo(repo);
        let result = GetPrototypesUseCase::execute(&mut uow, None).await;

        assert!(matches!(result, Err(DomainError::Infrastructure(_))));
    }
}
