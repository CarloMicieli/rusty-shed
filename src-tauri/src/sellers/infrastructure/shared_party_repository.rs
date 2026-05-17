use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::usage_queries;

pub async fn canonical_party_usage_count(
    executor: &mut sqlx::SqliteConnection,
    party_id: &str,
) -> Result<i64, DomainError> {
    usage_queries::canonical_party_usage_count(executor, party_id)
        .await
        .map_err(|e| DomainError::Infrastructure(e.to_string()))
}
