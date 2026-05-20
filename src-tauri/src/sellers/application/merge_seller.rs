use crate::core::domain::domain_error::DomainError;
use crate::sellers::domain::seller_id::SellerId;

/// Use case that merges one seller into another canonical seller entity.
pub struct MergeSeller;

impl MergeSeller {
    /// Relinks references from `source_id` to `target_id`, then deletes the source seller.
    ///
    /// Returns the number of relinked references across seller and buyer usage fields.
    pub async fn execute(
        tx: &mut sqlx::SqliteConnection,
        source_id: &SellerId,
        target_id: &SellerId,
    ) -> Result<i64, DomainError> {
        if source_id == target_id {
            return Err(DomainError::BusinessRule(
                "Source and target must be different".to_string(),
            ));
        }

        let source_seeded = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT is_system_seeded
            FROM sellers
            WHERE id = ?1
            LIMIT 1
            "#,
        )
        .bind(source_id.as_ref())
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| DomainError::Infrastructure(e.to_string()))?
        .ok_or_else(|| DomainError::NotFound {
            resource: "Seller".to_string(),
            identifier: source_id.to_string(),
        })?;

        let target_seeded = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT is_system_seeded
            FROM sellers
            WHERE id = ?1
            LIMIT 1
            "#,
        )
        .bind(target_id.as_ref())
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| DomainError::Infrastructure(e.to_string()))?
        .ok_or_else(|| DomainError::NotFound {
            resource: "Seller".to_string(),
            identifier: target_id.to_string(),
        })?;

        if source_seeded != 0 || target_seeded != 0 {
            return Err(DomainError::BusinessRule(
                "Protected entities cannot be merged".to_string(),
            ));
        }

        let seller_relinked = sqlx::query(
            r#"
            UPDATE purchase_infos
            SET seller_id = ?2
            WHERE seller_id = ?1
            "#,
        )
        .bind(source_id.as_ref())
        .bind(target_id.as_ref())
        .execute(&mut *tx)
        .await
        .map_err(|e| DomainError::Infrastructure(e.to_string()))?
        .rows_affected() as i64;

        let buyer_relinked = sqlx::query(
            r#"
            UPDATE purchase_infos
            SET buyer_id = ?2
            WHERE buyer_id = ?1
            "#,
        )
        .bind(source_id.as_ref())
        .bind(target_id.as_ref())
        .execute(&mut *tx)
        .await
        .map_err(|e| DomainError::Infrastructure(e.to_string()))?
        .rows_affected() as i64;

        let deleted = sqlx::query(
            r#"
            DELETE FROM sellers
            WHERE id = ?1
            "#,
        )
        .bind(source_id.as_ref())
        .execute(&mut *tx)
        .await
        .map_err(|e| DomainError::Infrastructure(e.to_string()))?
        .rows_affected();

        if deleted == 0 {
            return Err(DomainError::NotFound {
                resource: "Seller".to_string(),
                identifier: source_id.to_string(),
            });
        }

        Ok(seller_relinked + buyer_relinked)
    }
}
