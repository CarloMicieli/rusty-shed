use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::sellers::domain::seller::Seller;
use crate::sellers::domain::seller_event::SellerEvent;
use crate::sellers::domain::seller_id::SellerId;
use crate::sellers::domain::{SellersRepository, SellersUowExt};
use crate::sellers::infrastructure::entities::SellerRow;
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
        let sql = r#"
        SELECT
            id,
            name,
            type AS seller_type,
            email,
            phone,
            website_url,
            street_address,
            extended_address,
            city,
            state_region,
            postal_code,
            country_code,
            created_at,
            updated_at
        FROM sellers
        ORDER BY name
        "#;
        let rows = sqlx::query_as::<_, SellerRow>(sql)
            .fetch_all(&mut *self.executor)
            .await
            .map_err(DomainError::Infrastructure)?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn get(&mut self, id: &SellerId) -> Result<Option<Seller>, DomainError> {
        let sql = r#"
        SELECT
            id,
            name,
            type AS seller_type,
            email,
            phone,
            website_url,
            street_address,
            extended_address,
            city,
            state_region,
            postal_code,
            country_code,
            created_at,
            updated_at
        FROM sellers
        WHERE id = ?
        "#;
        let row = sqlx::query_as::<_, SellerRow>(sql)
            .bind(&id.0)
            .fetch_optional(&mut *self.executor)
            .await
            .map_err(DomainError::Infrastructure)?;
        Ok(row.map(Into::into))
    }

    async fn find_seller_view_by_id(
        &mut self,
        id: &SellerId,
    ) -> Result<Option<crate::sellers::application::seller_view::SellerView>, DomainError> {
        let sql = r#"
        SELECT
            id,
            name,
            type AS seller_type,
            email,
            phone,
            website_url,
            street_address,
            extended_address,
            city,
            state_region,
            postal_code,
            country_code,
            created_at,
            updated_at
        FROM sellers
        WHERE id = ?
        "#;
        let row = sqlx::query_as::<_, SellerRow>(sql)
            .bind(&id.0)
            .fetch_optional(&mut *self.executor)
            .await
            .map_err(DomainError::Infrastructure)?;
        Ok(row.map(|r| {
            let s: Seller = r.into();
            crate::sellers::application::seller_view::SellerView::from(s)
        }))
    }

    async fn upsert(&mut self, seller: &Seller) -> Result<(), DomainError> {
        // Keep existing created_at if row exists; otherwise use provided created_at.
        let existing_created_at: Option<String> =
            sqlx::query_scalar("SELECT created_at FROM sellers WHERE id = ?")
                .bind(&seller.id.0)
                .fetch_optional(&mut *self.executor)
                .await
                .map_err(DomainError::Infrastructure)?;

        let created_at_to_use =
            existing_created_at.unwrap_or_else(|| seller.created_at.to_rfc3339());
        let updated_at = Utc::now().to_rfc3339();

        // Extract address components for DB binding
        let (street_address, extended_address, city, state_region, postal_code, country_code) =
            match &seller.address {
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

        let sql = r#"
        INSERT INTO sellers (
            id, name, type, email, phone, website_url,
            street_address, extended_address, city, state_region, postal_code, country_code,
            created_at, updated_at
        ) VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?)
                        ON CONFLICT(id) DO UPDATE SET
              name = excluded.name,
              type = excluded.type,
              email = excluded.email,
              phone = excluded.phone,
              website_url = excluded.website_url,
              street_address = excluded.street_address,
              extended_address = excluded.extended_address,
              city = excluded.city,
              state_region = excluded.state_region,
              postal_code = excluded.postal_code,
              country_code = excluded.country_code,
              updated_at = excluded.updated_at
        "#;
        sqlx::query(sql)
            .bind(&seller.id.0)
            .bind(&seller.name)
            .bind(&seller.seller_type)
            .bind(&seller.email)
            .bind(&seller.phone)
            .bind(&seller.website_url)
            .bind(&street_address)
            .bind(&extended_address)
            .bind(&city)
            .bind(&state_region)
            .bind(&postal_code)
            .bind(&country_code)
            .bind(&created_at_to_use)
            .bind(&updated_at)
            .execute(&mut *self.executor)
            .await
            .map_err(DomainError::Infrastructure)?;
        Ok(())
    }

    async fn delete(&mut self, id: &SellerId) -> Result<u64, DomainError> {
        let sql = "DELETE FROM sellers WHERE id = ?";
        let res = sqlx::query(sql)
            .bind(&id.0)
            .execute(&mut *self.executor)
            .await
            .map_err(DomainError::Infrastructure)?;
        Ok(res.rows_affected())
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
                    created_at,
                    updated_at,
                }
                | SellerEvent::Updated {
                    aggregate_id,
                    name,
                    seller_type,
                    email,
                    phone,
                    website_url,
                    address,
                    created_at,
                    updated_at,
                } => {
                    // Keep existing created_at if row exists; otherwise use provided created_at.
                    let existing_created_at: Option<String> =
                        sqlx::query_scalar("SELECT created_at FROM sellers WHERE id = ?")
                            .bind(&aggregate_id.0)
                            .fetch_optional(&mut *self.executor)
                            .await
                            .map_err(DomainError::Infrastructure)?;

                    let created_at_to_use =
                        existing_created_at.unwrap_or_else(|| created_at.to_rfc3339());
                    let updated_at_to_use = updated_at.to_rfc3339();

                    let (
                        street_address,
                        extended_address,
                        city,
                        state_region,
                        postal_code,
                        country_code,
                    ) = match &address {
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

                    let sql = r#"
                    INSERT INTO sellers (
                        id, name, type, email, phone, website_url,
                        street_address, extended_address, city, state_region, postal_code, country_code,
                        created_at, updated_at
                    ) VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?)
                                    ON CONFLICT(id) DO UPDATE SET
                          name = excluded.name,
                          type = excluded.type,
                          email = excluded.email,
                          phone = excluded.phone,
                          website_url = excluded.website_url,
                          street_address = excluded.street_address,
                          extended_address = excluded.extended_address,
                          city = excluded.city,
                          state_region = excluded.state_region,
                          postal_code = excluded.postal_code,
                          country_code = excluded.country_code,
                          updated_at = excluded.updated_at
                    "#;
                    sqlx::query(sql)
                        .bind(&aggregate_id.0)
                        .bind(&name)
                        .bind(&seller_type)
                        .bind(&email)
                        .bind(&phone)
                        .bind(&website_url)
                        .bind(&street_address)
                        .bind(&extended_address)
                        .bind(&city)
                        .bind(&state_region)
                        .bind(&postal_code)
                        .bind(&country_code)
                        .bind(&created_at_to_use)
                        .bind(&updated_at_to_use)
                        .execute(&mut *self.executor)
                        .await
                        .map_err(DomainError::Infrastructure)?;
                }
                SellerEvent::Deleted { aggregate_id } => {
                    let sql = "DELETE FROM sellers WHERE id = ?";
                    sqlx::query(sql)
                        .bind(&aggregate_id.0)
                        .execute(&mut *self.executor)
                        .await
                        .map_err(DomainError::Infrastructure)?;
                }
            }
        }

        Ok(())
    }
}

impl<'conn> SellersUowExt for SqliteUnitOfWork<'conn> {
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
