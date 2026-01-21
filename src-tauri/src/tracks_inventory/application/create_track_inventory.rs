use crate::core::domain::domain_error::DomainError;
use crate::core::domain::metadata::Metadata;
use crate::tracks_inventory::domain::TracksInventoryUowExt;
use crate::tracks_inventory::domain::{TrackInventory, TrackInventoryEvent, TrackInventoryId};

/// Use case to create a new `TrackInventory` aggregate.
#[allow(dead_code)]
pub struct CreateTrackInventoryUseCase;

impl CreateTrackInventoryUseCase {
    pub async fn execute<U>(
        unit_of_work: &mut U,
        name: String,
        description: Option<String>,
    ) -> Result<TrackInventoryId, DomainError>
    where
        U: TracksInventoryUowExt + Send,
    {
        let mut repo = unit_of_work.track_inventories_repo();

        let id = TrackInventoryId::default();

        let mut aggregate = TrackInventory {
            id: id.clone(),
            name: name.clone(),
            description: description.clone(),
            inventory: std::collections::HashMap::new(),
            purchase_history: Vec::new(),
            metadata: Metadata::default(),
            pending_events: Vec::new(),
        };

        aggregate.push_event(TrackInventoryEvent::Created);
        aggregate.push_event(TrackInventoryEvent::Renamed { name });
        aggregate.push_event(TrackInventoryEvent::DescriptionUpdated { description });

        repo.save(aggregate).await.map(|_| id)
    }
}
