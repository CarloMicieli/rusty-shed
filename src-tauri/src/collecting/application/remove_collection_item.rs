use crate::collecting::domain::Collection;
use crate::collecting::domain::CollectionItem;
use crate::collecting::domain::CollectionView;
use crate::collecting::domain::OwnedRollingStock;
use crate::collecting::domain::RemoveCollectionItem;
use crate::collecting::infrastructure::repositories::CollectingUowExt;
use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;

pub struct RemoveCollectionItemCommand;

impl RemoveCollectionItemCommand {
    pub async fn execute(
        unit_of_work: &mut SqliteUnitOfWork<'_>,
        remove_cmd: RemoveCollectionItem,
    ) -> Result<CollectionView, DomainError> {
        let mut repo = unit_of_work.collection_repository();

        // Load current view and rehydrate into domain `Collection` so we can
        // apply domain operations and persist resulting events.
        let view = repo.find_view().await?;

        let mut collection = Collection {
            id: view.id.clone(),
            name: view.name.clone(),
            summary: view.summary,
            total_value: view.total_value,
            items: view
                .items
                .into_iter()
                .map(|iv| {
                    let rolling_stocks = iv
                        .rolling_stocks
                        .into_iter()
                        .map(|ov| OwnedRollingStock {
                            id: ov.id,
                            rolling_stock_id: ov.rolling_stock_id,
                            notes: ov.notes,
                            installed_decoder_id: ov.digital.map(|d| d.installed_decoder_id),
                        })
                        .collect();

                    CollectionItem {
                        id: iv.id,
                        railway_model_id: iv.railway_model.railway_model_id,
                        added_date: iv.added_date,
                        removed_date: iv.removed_date,
                        purchase_condition: iv.purchase_condition,
                        model_condition: iv.model_condition,
                        box_condition: iv.box_condition,
                        notes: iv.notes,
                        rolling_stocks,
                        purchase_info: iv.purchase_info,
                    }
                })
                .collect(),
            pending_events: Vec::new(),
            metadata: Default::default(),
        };

        collection.remove_item(remove_cmd);

        repo.save(&mut collection).await?;

        // Return the refreshed view after persistence
        let updated = repo.find_view().await?;
        Ok(updated)
    }
}
