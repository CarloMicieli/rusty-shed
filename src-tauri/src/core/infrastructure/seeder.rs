use anyhow::Context;
use chrono::Utc;
use csv::ReaderBuilder;
use slug::slugify;
use sqlx::{QueryBuilder, SqlitePool};

static MANUFACTURES: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/seed/manufacturers.csv"
));
static RAILWAY_COMPANIES: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/seed/railway_companies.csv"
));

const CHUNK_SIZE: usize = 50;

pub async fn seed_manufacturers(pool: &SqlitePool) -> anyhow::Result<()> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(MANUFACTURES.as_bytes());

    let now = Utc::now().to_rfc3339();

    // Collect records into a Vec so we can chunk them
    let records: Vec<_> = rdr
        .records()
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to parse manufacturers CSV records")?;

    // Start a single transaction for the entire operation
    // This ensures that either all manufacturers are updated/inserted, or none are.
    let mut tx = pool.begin().await.context("Failed to start transaction")?;

    // Process in chunks of 50 (safe for SQLite's parameter limits)
    for chunk in records.chunks(CHUNK_SIZE) {
        let mut query_builder: QueryBuilder<sqlx::Sqlite> = QueryBuilder::new(
            "INSERT INTO manufacturers (id, name, registered_company_name, status, country_code, website_url, created_at, updated_at) ",
        );

        query_builder.push_values(chunk, |mut b, record| {
            let name = record.get(0).unwrap_or_default();
            let id = format!("trn:manufacturer:{}", slugify(name));

            b.push_bind(id)
                .push_bind(name.to_string())
                .push_bind(
                    record
                        .get(1)
                        .filter(|s| !s.is_empty())
                        .map(|s| s.to_string()),
                )
                .push_bind(record.get(2).unwrap_or("ACTIVE").to_string())
                .push_bind(
                    record
                        .get(3)
                        .filter(|s| !s.is_empty())
                        .map(|s| s.to_string()),
                )
                .push_bind(
                    record
                        .get(4)
                        .filter(|s| !s.is_empty())
                        .map(|s| s.to_string()),
                )
                .push_bind(&now)
                .push_bind(&now);
        });

        // Add the UPSERT (ON CONFLICT) logic
        // If the ID already exists, we update the existing row instead of failing.
        query_builder.push(" ON CONFLICT(id) DO UPDATE SET ");
        query_builder.push("name = EXCLUDED.name, ");
        query_builder.push("registered_company_name = EXCLUDED.registered_company_name, ");
        query_builder.push("status = EXCLUDED.status, ");
        query_builder.push("country_code = EXCLUDED.country_code, ");
        query_builder.push("website_url = EXCLUDED.website_url, ");
        query_builder.push("updated_at = EXCLUDED.updated_at");

        // Execute this chunk against the transaction
        let query = query_builder.build();
        query
            .execute(&mut *tx)
            .await
            .context("Failed to execute upsert chunk")?;
    }

    // 6. Commit everything
    tx.commit().await.context("Failed to commit transaction")?;

    Ok(())
}

pub async fn seed_railway_companies(pool: &SqlitePool) -> anyhow::Result<()> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_reader(RAILWAY_COMPANIES.as_bytes());

    let records: Vec<_> = rdr.records().collect::<Result<Vec<_>, _>>()?;
    let now = Utc::now().to_rfc3339();

    let mut tx = pool.begin().await.context("Failed to start transaction")?;

    let insert_cmd = r#"
            INSERT INTO railway_companies (
                id, name, registered_company_name, country_code,
                status, operating_since, operating_until,
                created_at, updated_at
            )
        "#;

    for chunk in records.chunks(CHUNK_SIZE) {
        let mut query_builder: QueryBuilder<sqlx::Sqlite> = QueryBuilder::new(insert_cmd);

        query_builder.push_values(chunk, |mut b, record| {
            // Handle CSV rows that may contain stray commas inside the registered_company_name
            // column (e.g. "London, Midland and Scottish Railway" unquoted). The CSV has
            // 6 columns: name, registered_company_name, country_code, status, operating_since, operating_until.
            // If a row has more than 6 fields, join the middle fields into the registered_company_name.
            let len = record.len();
            let name = record.get(0).unwrap_or_default();

            // Reconstruct registered_company_name: everything between the first column and the last 4 columns
            let registered_company_name: Option<String> = if len > 6 {
                // join the middle parts (from index 1 up to len-4)
                let middle_count = len - 5; // number of fields that belong to registered_company_name
                let joined = record
                    .iter()
                    .skip(1)
                    .take(middle_count)
                    .map(|s| s.trim())
                    .collect::<Vec<_>>()
                    .join(",");
                if joined.is_empty() {
                    None
                } else {
                    Some(joined)
                }
            } else {
                record
                    .get(1)
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
            };

            let country_code = if len >= 4 {
                record
                    .get(len - 4)
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
            } else {
                None
            };

            let status = if len >= 3 {
                record
                    .get(len - 3)
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
            } else {
                None
            };

            let operating_since = if len >= 2 {
                record
                    .get(len - 2)
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
            } else {
                None
            };

            let operating_until = if len >= 1 {
                record
                    .get(len - 1)
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
            } else {
                None
            };

            let id = format!("trn:railway-company:{}", slugify(name));

            b.push_bind(id)
                .push_bind(name.to_string())
                .push_bind(registered_company_name)
                .push_bind(country_code)
                .push_bind(status)
                .push_bind(operating_since)
                .push_bind(operating_until)
                .push_bind(&now) // Use reference to avoid cloning inside loop
                .push_bind(&now);
        });

        // Add the UPSERT (ON CONFLICT) logic
        // If the ID already exists, we update the existing row instead of failing.
        query_builder.push(" ON CONFLICT(id) DO UPDATE SET ");
        query_builder.push("name = EXCLUDED.name, ");
        query_builder.push("registered_company_name = EXCLUDED.registered_company_name, ");
        query_builder.push("status = EXCLUDED.status, ");
        query_builder.push("country_code = EXCLUDED.country_code, ");
        query_builder.push("operating_until = EXCLUDED.operating_until, ");
        query_builder.push("updated_at = EXCLUDED.updated_at");

        let query = query_builder.build();
        query
            .execute(&mut *tx) // Use &mut *tx to borrow the transaction for each chunk
            .await
            .context("Failed to execute a batch chunk within transaction")?;
    }

    // 4. Commit the transaction
    tx.commit().await.context("Failed to commit transaction")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[sqlx::test(migrations = "./migrations")]
    async fn seeds_railway_companies(pool: SqlitePool) {
        seed_railway_companies(&pool)
            .await
            .expect("seeder should run without errors");

        let mut conn = pool.acquire().await.expect("acquire conn");

        // Ensure table has rows
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM railway_companies")
            .fetch_one(&mut *conn)
            .await
            .expect("count query should succeed");
        assert!(count > 0, "expected at least one seeded railway company");

        // Check a concrete seeded entry exists (FS -> slugified to `fs`)
        let name: Option<String> =
            sqlx::query_scalar::<_, String>("SELECT name FROM railway_companies WHERE id = ?")
                .bind("trn:railway-company:fs")
                .fetch_optional(&mut *conn)
                .await
                .expect("select name query should succeed");

        assert!(
            name.is_some(),
            "expected a seeded entry for id trn:railway-company:fs"
        );
        assert_eq!(name.unwrap(), "FS");
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn seeds_manufacturers(pool: SqlitePool) {
        // Run the manufacturers seeder
        seed_manufacturers(&pool)
            .await
            .expect("seed_manufacturers should run without errors");

        let mut conn = pool.acquire().await.expect("acquire conn");

        // Ensure table has rows
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM manufacturers")
            .fetch_one(&mut *conn)
            .await
            .expect("count query should succeed");
        assert!(count > 0, "expected at least one seeded manufacturer");

        // Check a concrete seeded entry exists (Atlas Model Railroad Co. -> slugified to `atlas-model-railroad-co`)
        let name: Option<String> =
            sqlx::query_scalar::<_, String>("SELECT name FROM manufacturers WHERE id = ?")
                .bind("trn:manufacturer:atlas-model-railroad-co")
                .fetch_optional(&mut *conn)
                .await
                .expect("select name query should succeed");

        assert!(
            name.is_some(),
            "expected a seeded entry for id mfr:atlas-model-railroad-co"
        );
        assert_eq!(name.unwrap(), "Atlas Model Railroad Co.");
    }
}
