use crate::catalog::domain::railway_model::{CouplerType, CouplerUowExt, CouplingSocket};
use crate::core::domain::domain_error::DomainError;

/// Input for [`GetCouplerTypes::execute`].
pub struct GetCouplerTypesInput {
    /// When `Some`, only couplers compatible with this socket are returned.
    pub socket: Option<CouplingSocket>,
}

/// Query use-case that returns the coupler type catalogue, optionally filtered by socket.
pub struct GetCouplerTypes;

impl GetCouplerTypes {
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: GetCouplerTypesInput,
    ) -> Result<Vec<CouplerType>, DomainError>
    where
        U: CouplerUowExt + Send,
    {
        let mut repo = unit_of_work.coupler_repository();
        repo.find_all(input.socket).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::application::testing::FakeUow;
    use crate::catalog::domain::railway_model::{
        CouplerTypeId, CouplingSocket, MockCouplerRepository,
    };
    use crate::core::domain::identifiers::Identifier;

    fn make_coupler(socket: CouplingSocket) -> CouplerType {
        CouplerType {
            id: CouplerTypeId::from_string_unchecked("trn:coupler:test:abc".to_string()),
            manufacturer: "Test".to_string(),
            name: "Test coupler".to_string(),
            compatible_socket: socket,
        }
    }

    #[tokio::test]
    async fn it_returns_all_coupler_types_when_no_filter() {
        let mut mock = MockCouplerRepository::new();
        mock.expect_find_all()
            .times(1)
            .returning(|_| Ok(vec![make_coupler(CouplingSocket::Nem362)]));

        let mut uow = FakeUow::with_coupler_repo(mock);
        let result = GetCouplerTypes::execute(&mut uow, GetCouplerTypesInput { socket: None })
            .await
            .expect("should return coupler types");

        assert_eq!(result.len(), 1);
    }

    #[tokio::test]
    async fn it_passes_socket_filter_to_repository() {
        let socket = CouplingSocket::Nem362;
        let mut mock = MockCouplerRepository::new();
        mock.expect_find_all()
            .withf(|s| *s == Some(CouplingSocket::Nem362))
            .times(1)
            .returning(|_| Ok(vec![make_coupler(CouplingSocket::Nem362)]));

        let mut uow = FakeUow::with_coupler_repo(mock);
        let result = GetCouplerTypes::execute(
            &mut uow,
            GetCouplerTypesInput {
                socket: Some(socket),
            },
        )
        .await
        .expect("should return filtered coupler types");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].compatible_socket, CouplingSocket::Nem362);
    }

    #[tokio::test]
    async fn it_returns_empty_vec_when_none_found() {
        let mut mock = MockCouplerRepository::new();
        mock.expect_find_all().times(1).returning(|_| Ok(vec![]));

        let mut uow = FakeUow::with_coupler_repo(mock);
        let result = GetCouplerTypes::execute(&mut uow, GetCouplerTypesInput { socket: None })
            .await
            .expect("should return empty list");

        assert!(result.is_empty());
    }
}
