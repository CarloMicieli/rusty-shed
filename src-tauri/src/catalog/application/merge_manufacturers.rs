use crate::catalog::domain::manufacturer::ManufacturerId;
use crate::core::domain::domain_error::DomainError;

pub struct MergeManufacturers;

impl MergeManufacturers {
    pub async fn execute(
        tx: &mut sqlx::SqliteConnection,
        source_id: &ManufacturerId,
        target_id: &ManufacturerId,
    ) -> Result<i64, DomainError> {
        if source_id == target_id {
            return Err(DomainError::BusinessRule(
                "Source and target must be different".to_string(),
            ));
        }

        let source_seeded = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT is_system_seeded
            FROM manufacturers
            WHERE id = ?1
            LIMIT 1
            "#,
        )
        .bind(source_id.as_ref())
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| DomainError::Infrastructure(e.to_string()))?
        .ok_or_else(|| DomainError::NotFound {
            resource: "Manufacturer".to_string(),
            identifier: source_id.to_string(),
        })?;

        let target_seeded = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT is_system_seeded
            FROM manufacturers
            WHERE id = ?1
            LIMIT 1
            "#,
        )
        .bind(target_id.as_ref())
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| DomainError::Infrastructure(e.to_string()))?
        .ok_or_else(|| DomainError::NotFound {
            resource: "Manufacturer".to_string(),
            identifier: target_id.to_string(),
        })?;

        if source_seeded != 0 || target_seeded != 0 {
            return Err(DomainError::BusinessRule(
                "Protected entities cannot be merged".to_string(),
            ));
        }

        let relinked_count = sqlx::query(
            r#"
            UPDATE railway_models
            SET manufacturer_id = ?2
            WHERE manufacturer_id = ?1
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
            DELETE FROM manufacturers
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
                resource: "Manufacturer".to_string(),
                identifier: source_id.to_string(),
            });
        }

        Ok(relinked_count)
    }
}
