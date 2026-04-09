//! Use case: update metadata on an existing train formation.

use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::trains::infrastructure::mappers::TrainFormationView;
use crate::trains::infrastructure::train_formation_repo::SqlxTrainFormationRepository;
use crate::trains::interface::command_args::UpdateTrainFormationArgs;

pub struct UpdateTrainFormationUseCase;

impl UpdateTrainFormationUseCase {
    pub async fn execute(
        uow: &mut SqliteUnitOfWork,
        id: String,
        args: UpdateTrainFormationArgs,
    ) -> Result<TrainFormationView, DomainError> {
        let mut repo = SqlxTrainFormationRepository::new(&mut uow.tx);
        let mut formation = repo.find_by_id_raw(&id).await?;

        if let Some(name) = args.name {
            formation.rename(name)?;
        }

        formation.update_metadata(
            args.category_id,
            args.start_year,
            args.end_year,
            args.epoch,
            args.notes,
        )?;

        repo.save(&formation).await?;
        repo.get_view(&id).await
    }
}
