use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::usage_queries::canonical_party_usage_count;
use crate::sellers::domain::seller_id::SellerId;

pub struct DeleteSellerWithLock;

impl DeleteSellerWithLock {
    pub async fn ensure_deletable(
        executor: &mut sqlx::SqliteConnection,
        id: &SellerId,
    ) -> Result<(), DomainError> {
        let row = sqlx::query_as::<_, (String, i64)>(
            r#"
            SELECT name, is_system_seeded
            FROM sellers
            WHERE id = ?1
            LIMIT 1
            "#,
        )
        .bind(id.as_ref())
        .fetch_optional(&mut *executor)
        .await
        .map_err(|e| DomainError::Infrastructure(e.to_string()))?
        .ok_or_else(|| DomainError::NotFound {
            resource: "Seller".to_string(),
            identifier: id.to_string(),
        })?;

        let (_name, is_seeded) = row;
        if is_seeded != 0 {
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
