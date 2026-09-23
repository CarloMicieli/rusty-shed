use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::usage_queries::canonical_party_usage_count;
use crate::sellers::domain::seller_id::SellerId;
use crate::sellers::infrastructure::database;

/// Guard use case that validates whether a seller can be safely deleted.
pub struct DeleteSellerWithLock;

impl DeleteSellerWithLock {
    /// Ensures that the seller exists, is not protected, and is not referenced by other
    /// entities.
    pub async fn ensure_deletable(
        executor: &mut sqlx::SqliteConnection,
        id: &SellerId,
    ) -> Result<(), DomainError> {
        let seller = database::find_seller_by_id(&mut *executor, id.as_ref())
            .await
            .map_err(|e| DomainError::Infrastructure(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound {
                resource: "Seller".to_string(),
                identifier: id.to_string(),
            })?;

        if seller.is_system_seeded != 0 {
            return Err(DomainError::BusinessRule(
                "Protected entity cannot be deleted".to_string(),
            ));
        }

        let usage_count = canonical_party_usage_count(&mut *executor, id.as_ref())
            .await
            .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        if usage_count > 0 {
            return Err(DomainError::BusinessRule(format!(
                "Entity is still in use ({usage_count})"
            )));
        }

        Ok(())
    }
}
