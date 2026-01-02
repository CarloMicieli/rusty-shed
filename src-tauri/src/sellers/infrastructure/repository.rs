use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::sellers::domain::seller::Seller;
use crate::sellers::domain::seller_id::SellerId;
use crate::sellers::infrastructure::entities::SellerRow;
use anyhow::Context;
use chrono::Utc;

pub struct SqliteSellersRepository<'conn> {
    executor: &'conn mut sqlx::SqliteConnection,
}

impl<'conn> SqliteSellersRepository<'conn> {
    pub fn new(executor: &'conn mut sqlx::SqliteConnection) -> Self {
        Self { executor }
    }

    pub async fn list(&mut self) -> anyhow::Result<Vec<Seller>> {
        let sql = r#"
        SELECT
          seller_id,
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
            .with_context(|| "listing sellers")?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    pub async fn get(&mut self, id: &SellerId) -> anyhow::Result<Option<Seller>> {
        let sql = r#"
        SELECT
          seller_id,
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
        WHERE seller_id = ?
        "#;
        let row = sqlx::query_as::<_, SellerRow>(sql)
            .bind(&id.0)
            .fetch_optional(&mut *self.executor)
            .await
            .with_context(|| format!("getting seller {}", id))?;
        Ok(row.map(Into::into))
    }

    pub async fn upsert(&mut self, seller: &Seller) -> anyhow::Result<()> {
        // Keep existing created_at if row exists; otherwise use provided created_at.
        let existing_created_at: Option<String> =
            sqlx::query_scalar("SELECT created_at FROM sellers WHERE seller_id = ?")
                .bind(&seller.id.0)
                .fetch_optional(&mut *self.executor)
                .await
                .with_context(|| format!("fetching seller created_at for {}", seller.id))?;

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
              seller_id, name, type, email, phone, website_url,
              street_address, extended_address, city, state_region, postal_code, country_code,
              created_at, updated_at
            ) VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?)
            ON CONFLICT(seller_id) DO UPDATE SET
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
            .with_context(|| format!("upserting seller {}", seller.id))?;
        Ok(())
    }

    pub async fn delete(&mut self, id: &SellerId) -> anyhow::Result<u64> {
        let sql = "DELETE FROM sellers WHERE seller_id = ?";
        let res = sqlx::query(sql)
            .bind(&id.0)
            .execute(&mut *self.executor)
            .await
            .with_context(|| format!("deleting seller {}", id))?;
        Ok(res.rows_affected())
    }
}

pub trait SellersUowExt {
    fn sellers_repo(&mut self) -> SqliteSellersRepository<'_>;
}

impl<'conn> SellersUowExt for SqliteUnitOfWork<'conn> {
    fn sellers_repo(&mut self) -> SqliteSellersRepository<'_> {
        SqliteSellersRepository::new(&mut self.tx)
    }
}
