//! Use case: create a new train formation.

use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::trains::domain::formation::train_formation::TrainFormation;
use crate::trains::infrastructure::mappers::TrainFormationView;
use crate::trains::infrastructure::train_formation_repo::SqlxTrainFormationRepository;
use crate::trains::interface::command_args::CreateTrainFormationArgs;

pub struct CreateTrainFormationUseCase;

impl CreateTrainFormationUseCase {
    pub async fn execute(
        uow: &mut SqliteUnitOfWork<'_>,
        args: CreateTrainFormationArgs,
    ) -> Result<TrainFormationView, DomainError> {
        let id = uuid::Uuid::new_v4().to_string();
        let mut formation = TrainFormation::create(id.clone(), args.name)?;

        // Set optional metadata fields
        formation.update_metadata(
            args.category_id,
            args.start_year,
            args.end_year,
            args.epoch,
            args.notes,
        )?;

        let mut repo = SqlxTrainFormationRepository::new(&mut uow.tx);
        repo.save(&formation).await?;
        repo.get_view(&id).await
    }
}
