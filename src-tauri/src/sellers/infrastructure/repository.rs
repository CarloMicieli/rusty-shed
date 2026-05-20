use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::core::infrastructure::usage_queries::canonical_party_usage_count;
use crate::sellers::domain::seller::Seller;
use crate::sellers::domain::seller_event::SellerEvent;
use crate::sellers::domain::seller_id::SellerId;
use crate::sellers::domain::{SellersRepository, SellersUowExt};
use crate::sellers::infrastructure::database;
use chrono::Utc;

pub struct SqliteSellersRepository<'conn> {
    executor: &'conn mut sqlx::SqliteConnection,
}

impl<'conn> SqliteSellersRepository<'conn> {
    pub fn new(executor: &'conn mut sqlx::SqliteConnection) -> Self {
        Self { executor }
    }
}

#[async_trait::async_trait]
impl<'conn> SellersRepository for SqliteSellersRepository<'conn> {
    async fn list(&mut self) -> Result<Vec<Seller>, DomainError> {
        let rows = database::list_sellers(&mut *self.executor)
            .await
            .map_err(|e| DomainError::Infrastructure(e.to_string()))?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn get(&mut self, id: &SellerId) -> Result<Option<Seller>, DomainError> {
        let row = database::find_seller_by_id(&mut *self.executor, &id.0)
            .await
            .map_err(|e| DomainError::Infrastructure(e.to_string()))?;
        Ok(row.map(Into::into))
    }

    async fn find_seller_view_by_id(
        &mut self,
        id: &SellerId,
    ) -> Result<Option<crate::sellers::application::seller_view::SellerView>, DomainError> {
        let row = database::find_seller_by_id(&mut *self.executor, &id.0)
            .await
            .map_err(|e| DomainError::Infrastructure(e.to_string()))?;
        Ok(row.map(|r| {
            let s: Seller = r.into();
            crate::sellers::application::seller_view::SellerView::from(s)
        }))
    }

    async fn upsert(&mut self, seller: &Seller) -> Result<(), DomainError> {
        // Preserve the original created_at when the row already exists.
        let existing_created_at =
            database::get_seller_created_at(&mut *self.executor, &seller.id.0)
                .await
                .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        let created_at_to_use =
            existing_created_at.unwrap_or_else(|| seller.metadata.created_at.to_rfc3339());
        let updated_at = Utc::now().to_rfc3339();

        // Decompose address into flat column values.
        let (street, extended, city, region, postal, country) = match &seller.address {
            Some(addr) => (
                Some(addr.street_address().to_string()),
                addr.extended_address().map(|s| s.to_string()),
                Some(addr.city().to_string()),
                addr.region().map(|s| s.to_string()),
                Some(addr.postal_code().to_string()),
                Some(addr.country_code().to_string()),
            ),
            None => (None, None, None, None, None, None),
        };

        database::upsert_seller(
            &mut *self.executor,
            &seller.id.0,
            &seller.name,
            &seller.seller_type,
            seller.email.as_deref(),
            seller.phone.as_deref(),
            seller.website_url.as_deref(),
            street.as_deref(),
            extended.as_deref(),
            city.as_deref(),
            region.as_deref(),
            postal.as_deref(),
            country.as_deref(),
            &created_at_to_use,
            &updated_at,
        )
        .await
        .map_err(|e| DomainError::Infrastructure(e.to_string()))
    }

    async fn delete(&mut self, id: &SellerId) -> Result<u64, DomainError> {
        database::delete_seller(&mut *self.executor, &id.0)
            .await
            .map_err(|e| DomainError::Infrastructure(e.to_string()))
    }

    async fn save(&mut self, seller: &mut Seller) -> Result<(), DomainError> {
        for ev in seller.pull_events() {
            match ev {
                SellerEvent::Created {
                    aggregate_id,
                    name,
                    seller_type,
                    email,
                    phone,
                    website_url,
                    address,
                    metadata,
                }
                | SellerEvent::Updated {
                    aggregate_id,
                    name,
                    seller_type,
                    email,
                    phone,
                    website_url,
                    address,
                    metadata,
                } => {
                    // Preserve the original created_at when the row already exists.
                    let existing_created_at =
                        database::get_seller_created_at(&mut *self.executor, &aggregate_id.0)
                            .await
                            .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

                    let created_at_to_use =
                        existing_created_at.unwrap_or_else(|| metadata.created_at.to_rfc3339());
                    let updated_at_to_use = metadata.updated_at.to_rfc3339();

                    let (street, extended, city, region, postal, country) = match &address {
                        Some(addr) => (
                            Some(addr.street_address().to_string()),
                            addr.extended_address().map(|s| s.to_string()),
                            Some(addr.city().to_string()),
                            addr.region().map(|s| s.to_string()),
                            Some(addr.postal_code().to_string()),
                            Some(addr.country_code().to_string()),
                        ),
                        None => (None, None, None, None, None, None),
                    };

                    database::upsert_seller(
                        &mut *self.executor,
                        &aggregate_id.0,
                        &name,
                        &seller_type,
                        email.as_deref(),
                        phone.as_deref(),
                        website_url.as_deref(),
                        street.as_deref(),
                        extended.as_deref(),
                        city.as_deref(),
                        region.as_deref(),
                        postal.as_deref(),
                        country.as_deref(),
                        &created_at_to_use,
                        &updated_at_to_use,
                    )
                    .await
                    .map_err(|e| DomainError::Infrastructure(e.to_string()))?;
                }
                SellerEvent::Deleted { aggregate_id } => {
                    database::delete_seller(&mut *self.executor, &aggregate_id.0)
                        .await
                        .map_err(|e| DomainError::Infrastructure(e.to_string()))?;
                }
            }
        }

        Ok(())
    }

    async fn find_seeded_and_name(
        &mut self,
        id: &SellerId,
    ) -> Result<Option<(String, bool)>, DomainError> {
        database::find_seller_seeded_and_name(&mut *self.executor, &id.0)
            .await
            .map_err(|e| DomainError::Infrastructure(e.to_string()))
    }

    async fn find_usage_count(&mut self, id: &SellerId) -> Result<i64, DomainError> {
        canonical_party_usage_count(&mut *self.executor, id.as_ref())
            .await
            .map_err(|e| DomainError::Infrastructure(e.to_string()))
    }
}

impl SellersUowExt for SqliteUnitOfWork {
    /// Links the SQLite-specific repository to the Unit of Work.
    ///
    /// It re-borrows the internal transaction (`&mut *self.tx`) to provide
    /// the repository with a mutable executor without transferring ownership.
    fn sellers_repository(&mut self) -> Box<dyn SellersRepository + '_> {
        Box::new(SqliteSellersRepository::new(&mut self.tx))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
    use crate::sellers::domain::seller_type::SellerType;

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_seller.sql")
    )]
    async fn list_returns_seeded(pool: sqlx::SqlitePool) {
        let mut unit_of_work = SqliteUnitOfWork::new(&pool)
            .await
            .expect("Couldn't create database connection");
        let mut repo = unit_of_work.sellers_repository();

        let sellers = repo.list().await.expect("list failed");

        let first = sellers.first().expect("first failed");

        assert_eq!(
            first.id,
            SellerId::try_from("trn:seller:model-train-shop").unwrap()
        );
        assert_eq!(first.name, "Model Train Shop");
        assert_eq!(first.seller_type, SellerType::Shop);
    }
}
