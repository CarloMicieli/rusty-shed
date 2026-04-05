use crate::catalog::domain::railway_model::CouplerTypeId;
use crate::core::domain::identifiers::{Identifier, slugify_entity_name};
use crate::dcc_inventory::domain::DecoderId;
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
static DECODERS: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/seed/decoders.csv"));
static SELLERS: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/seed/sellers.csv"));
static TRACK_PRODUCTS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/seed/track_products.csv"
));
static TRAIN_CATEGORIES: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/seed/train_categories.csv"
));
static PROTOTYPES: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/seed/prototypes.csv"));
static COUPLERS: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/seed/couplers.csv"));

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
            let id = format!("trn:manufacturer:{}", slugify_entity_name(name));

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

            let id = format!("trn:railway-company:{}", slugify_entity_name(name));

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

pub async fn seed_decoders(pool: &SqlitePool) -> anyhow::Result<()> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(DECODERS.as_bytes());

    let records: Vec<_> = rdr.records().collect::<Result<Vec<_>, _>>()?;

    let mut tx = pool.begin().await.context("Failed to start transaction")?;

    let insert_cmd = r#"
        INSERT INTO decoders (
            id, manufacturer_id, product_code, decoder_type, protocol, decoder_interface
        )
    "#;

    for chunk in records.chunks(CHUNK_SIZE) {
        let mut query_builder: QueryBuilder<sqlx::Sqlite> = QueryBuilder::new(insert_cmd);

        query_builder.push_values(chunk, |mut b, record| {
            let manufacturer = record.get(0).unwrap_or_default();
            let product_code = record.get(1).unwrap_or_default();
            let decoder_type = record.get(2).unwrap_or_default();
            let protocol = record.get(3).unwrap_or_default();
            let decoder_interface = record.get(4).unwrap_or_default();

            // manufacturer id stored in DB references manufacturers.id which uses the
            // `trn:manufacturer:{slug}` format. The CSV provides the slug-like short id
            // (e.g. `esu`), so build the full TRN here.
            let manufacturer_id = format!("trn:manufacturer:{}", slugify(manufacturer));

            // Build decoder id using the same normalization as DecoderId::new_from_parts
            let id = DecoderId::new_from_parts(&[manufacturer, product_code]).to_string();

            b.push_bind(id)
                .push_bind(manufacturer_id)
                .push_bind(product_code.to_string())
                .push_bind(decoder_type.to_uppercase())
                .push_bind(protocol.to_uppercase())
                .push_bind(decoder_interface.to_uppercase());
        });

        // Upsert logic: update fields when id already present
        query_builder.push(" ON CONFLICT(id) DO UPDATE SET ");
        query_builder.push("manufacturer_id = EXCLUDED.manufacturer_id, ");
        query_builder.push("product_code = EXCLUDED.product_code, ");
        query_builder.push("decoder_type = EXCLUDED.decoder_type, ");
        query_builder.push("protocol = EXCLUDED.protocol, ");
        query_builder.push("decoder_interface = EXCLUDED.decoder_interface");

        let query = query_builder.build();
        query
            .execute(&mut *tx)
            .await
            .context("Failed to execute decoder upsert chunk")?;
    }

    tx.commit().await.context("Failed to commit transaction")?;
    Ok(())
}

pub async fn seed_sellers(pool: &SqlitePool) -> anyhow::Result<()> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(SELLERS.as_bytes());

    let records: Vec<_> = rdr
        .records()
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to parse sellers CSV records")?;

    let now = Utc::now().to_rfc3339();
    let mut tx = pool.begin().await.context("Failed to start transaction")?;

    let insert_cmd = r#"
        INSERT INTO sellers (
            id, name, type, email, phone, website_url,
            street_address, city, state_region, postal_code, country_code,
            created_at, updated_at
        )
    "#;

    for chunk in records.chunks(CHUNK_SIZE) {
        let mut query_builder: QueryBuilder<sqlx::Sqlite> = QueryBuilder::new(insert_cmd);

        query_builder.push_values(chunk, |mut b, record| {
            let name = record.get(0).unwrap_or_default();
            let seller_type = record.get(1).unwrap_or("SHOP");

            // Optional fields (convert empty strings to None)
            let email = record
                .get(2)
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string());

            let phone = record
                .get(3)
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string());

            let website_url = record
                .get(4)
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string());

            let street_address = record
                .get(5)
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string());

            let city = record
                .get(6)
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string());

            // Map CSV `region` -> DB `state_region` (Option A)
            let state_region = record
                .get(7)
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string());

            let postal_code = record
                .get(8)
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string());

            let country_code = record
                .get(9)
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string());

            // Seller id is derived via slug from the name using slugify
            let seller_id = format!("trn:seller:{}", slugify_entity_name(name));

            b.push_bind(seller_id)
                .push_bind(name.to_string())
                .push_bind(seller_type.to_string())
                .push_bind(email)
                .push_bind(phone)
                .push_bind(website_url)
                .push_bind(street_address)
                .push_bind(city)
                .push_bind(state_region)
                .push_bind(postal_code)
                .push_bind(country_code)
                .push_bind(&now)
                .push_bind(&now);
        });

        query_builder.push(" ON CONFLICT(id) DO UPDATE SET ");
        query_builder.push("name = EXCLUDED.name, ");
        query_builder.push("type = EXCLUDED.type, ");
        query_builder.push("email = EXCLUDED.email, ");
        query_builder.push("phone = EXCLUDED.phone, ");
        query_builder.push("website_url = EXCLUDED.website_url, ");
        query_builder.push("street_address = EXCLUDED.street_address, ");
        query_builder.push("city = EXCLUDED.city, ");
        query_builder.push("state_region = EXCLUDED.state_region, ");
        query_builder.push("postal_code = EXCLUDED.postal_code, ");
        query_builder.push("country_code = EXCLUDED.country_code, ");
        query_builder.push("updated_at = EXCLUDED.updated_at");

        let query = query_builder.build();
        query
            .execute(&mut *tx)
            .await
            .context("Failed to execute seller upsert chunk")?;
    }

    tx.commit().await.context("Failed to commit transaction")?;
    Ok(())
}

pub async fn seed_track_products(pool: &SqlitePool) -> anyhow::Result<()> {
    use crate::tracks_inventory::domain::TrackId;

    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_reader(TRACK_PRODUCTS.as_bytes());

    let records: Vec<_> = rdr
        .records()
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to parse track_products CSV")?;

    let now = Utc::now().to_rfc3339();
    let mut tx = pool.begin().await.context("Failed to start transaction")?;

    let insert_cmd = r#"
        INSERT INTO track_products (
            id, track_id, manufacturer_id, product_code, with_roadbed,
            length_mm, radius_mm, track_code, track_type, description,
            created_at, updated_at, version
        )
    "#;

    for chunk in records.chunks(CHUNK_SIZE) {
        let mut qb: QueryBuilder<sqlx::Sqlite> = QueryBuilder::new(insert_cmd);

        qb.push_values(chunk, |mut b, record| {
            let manufacturer = record.get(2).unwrap_or_default(); // col: manufacturer_id
            let product_code = record.get(3).unwrap_or_default();

            let track_id = TrackId::new_from_parts(&[manufacturer, product_code]).to_string();
            let manufacturer_id = format!("trn:manufacturer:{}", slugify(manufacturer));

            let with_roadbed: i32 = record.get(4).unwrap_or("0").parse().unwrap_or(0);

            let length_mm: Option<i32> = record
                .get(5)
                .filter(|s| !s.is_empty())
                .and_then(|s| s.parse().ok());

            let radius_mm: Option<i32> = record
                .get(6)
                .filter(|s| !s.is_empty())
                .and_then(|s| s.parse().ok());

            let track_code: Option<String> = match record.get(7).unwrap_or_default() {
                "70" => Some("CODE_70".to_string()),
                "75" => Some("CODE_75".to_string()),
                "83" => Some("CODE_83".to_string()),
                "100" => Some("CODE_100".to_string()),
                _ => None,
            };

            let track_type: Option<String> =
                match record.get(8).unwrap_or_default().to_lowercase().as_str() {
                    "straight" => Some("STRAIGHT".to_string()),
                    "curved" => Some("CURVE".to_string()),
                    "turnout" => Some("TURNOUT".to_string()),
                    "flex" => Some("FLEX_TRACK".to_string()),
                    _ => None,
                };

            let description: Option<String> = record
                .get(9)
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string());

            b.push_bind(track_id.clone())
                .push_bind(track_id)
                .push_bind(manufacturer_id)
                .push_bind(product_code.to_string())
                .push_bind(with_roadbed)
                .push_bind(length_mm)
                .push_bind(radius_mm)
                .push_bind(track_code)
                .push_bind(track_type)
                .push_bind(description)
                .push_bind(&now)
                .push_bind(&now)
                .push_bind(0i32);
        });

        qb.push(" ON CONFLICT(track_id) DO UPDATE SET ");
        qb.push("manufacturer_id = EXCLUDED.manufacturer_id, ");
        qb.push("product_code = EXCLUDED.product_code, ");
        qb.push("with_roadbed = EXCLUDED.with_roadbed, ");
        qb.push("length_mm = EXCLUDED.length_mm, ");
        qb.push("radius_mm = EXCLUDED.radius_mm, ");
        qb.push("track_code = EXCLUDED.track_code, ");
        qb.push("track_type = EXCLUDED.track_type, ");
        qb.push("description = EXCLUDED.description, ");
        qb.push("updated_at = EXCLUDED.updated_at");

        qb.build()
            .execute(&mut *tx)
            .await
            .context("Failed to execute track_products upsert chunk")?;
    }

    tx.commit().await.context("Failed to commit transaction")?;
    Ok(())
}

/// Seed the default formation categories from the embedded CSV file.
///
/// Uses an `ON CONFLICT(id) DO UPDATE SET` upsert so it is safe to call
/// on every application startup. The `id` is derived as
/// `trn:formation-category:{slugify(name)}`; `is_custom` is always `0`.
pub async fn seed_train_categories(pool: &SqlitePool) -> anyhow::Result<()> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(TRAIN_CATEGORIES.as_bytes());

    let records: Vec<_> = rdr
        .records()
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to parse train_categories CSV records")?;

    let mut tx = pool.begin().await.context("Failed to start transaction")?;

    for chunk in records.chunks(CHUNK_SIZE) {
        let mut query_builder: QueryBuilder<sqlx::Sqlite> =
            QueryBuilder::new("INSERT INTO formation_categories (id, name, is_custom) ");

        query_builder.push_values(chunk, |mut b, record| {
            let name = record.get(0).unwrap_or_default();
            let id = format!("trn:formation-category:{}", slugify(name));

            b.push_bind(id).push_bind(name.to_string()).push_bind(0i64);
        });

        query_builder.push(" ON CONFLICT(id) DO UPDATE SET name = EXCLUDED.name");

        query_builder
            .build()
            .execute(&mut *tx)
            .await
            .context("Failed to execute formation_categories upsert chunk")?;
    }

    tx.commit().await.context("Failed to commit transaction")?;
    Ok(())
}

/// Seed the default prototype catalogue from the embedded CSV file.
///
/// CSV columns (0-based index):
/// ```text
/// 0:  id
/// 1:  railway_company_id
/// 2:  series_code
/// 3:  friendly_name               (may be empty)
/// 4:  specification_type          (LOCOMOTIVE | PASSENGER_CAR | FREIGHT_CAR | RAILCAR | ELECTRIC_MULTIPLE_UNIT)
/// 5:  locomotive_type             (may be empty)
/// 6:  locomotive_series           (may be empty)
/// 7:  service_level               (may be empty)
/// 8:  passenger_car_type          (may be empty)
/// 9:  freight_car_type            (may be empty)
/// 10: railcar_type                (may be empty)
/// 11: electric_multiple_unit_type (may be empty)
/// 12: elements_count              (may be empty)
/// 13: is_permanently_coupled      (0|1, may be empty)
/// 14: is_motorized                (0|1)
/// 15: default_is_dummy            (0|1)
/// ```
///
/// Uses an `ON CONFLICT(id) DO UPDATE SET` upsert so it is safe to call
/// on every application startup. `is_custom` is always `0` for seeded rows.
pub async fn seed_prototypes(pool: &SqlitePool) -> anyhow::Result<()> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(PROTOTYPES.as_bytes());

    let records: Vec<_> = rdr
        .records()
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to parse prototypes CSV records")?;

    let now = Utc::now().to_rfc3339();
    let mut tx = pool.begin().await.context("Failed to start transaction")?;

    let insert_cmd = r#"
        INSERT INTO prototypes (
            id, railway_company_id, series_code, friendly_name,
            specification_type,
            locomotive_type, locomotive_series,
            service_level, passenger_car_type,
            freight_car_type, railcar_type,
            electric_multiple_unit_type, elements_count, is_permanently_coupled,
            is_motorized, default_is_dummy, is_custom,
            created_at, updated_at, version
        )
    "#;

    for chunk in records.chunks(CHUNK_SIZE) {
        let mut query_builder: QueryBuilder<sqlx::Sqlite> = QueryBuilder::new(insert_cmd);

        query_builder.push_values(chunk, |mut b, record| {
            let opt = |col: usize| -> Option<String> {
                record
                    .get(col)
                    .filter(|s| !s.is_empty())
                    .map(str::to_string)
            };

            let id = record.get(0).unwrap_or_default();
            let railway_company_id = record.get(1).unwrap_or_default();
            let series_code = record.get(2).unwrap_or_default();
            let friendly_name = opt(3);
            let specification_type = record.get(4).unwrap_or_default();
            let locomotive_type = opt(5);
            let locomotive_series = opt(6);
            let service_level = opt(7);
            let passenger_car_type = opt(8);
            let freight_car_type = opt(9);
            let railcar_type = opt(10);
            let electric_multiple_unit_type = opt(11);
            let elements_count: Option<i64> = opt(12).and_then(|s| s.parse().ok());
            let is_permanently_coupled: Option<i64> = opt(13).and_then(|s| s.parse().ok());
            let is_motorized: i64 = record.get(14).unwrap_or("0").parse().unwrap_or(0);
            let default_is_dummy: i64 = record.get(15).unwrap_or("0").parse().unwrap_or(0);

            b.push_bind(id.to_string())
                .push_bind(railway_company_id.to_string())
                .push_bind(series_code.to_string())
                .push_bind(friendly_name)
                .push_bind(specification_type.to_string())
                .push_bind(locomotive_type)
                .push_bind(locomotive_series)
                .push_bind(service_level)
                .push_bind(passenger_car_type)
                .push_bind(freight_car_type)
                .push_bind(railcar_type)
                .push_bind(electric_multiple_unit_type)
                .push_bind(elements_count)
                .push_bind(is_permanently_coupled)
                .push_bind(is_motorized)
                .push_bind(default_is_dummy)
                .push_bind(0i64)
                .push_bind(&now)
                .push_bind(&now)
                .push_bind(0i64);
        });

        query_builder.push(" ON CONFLICT(id) DO UPDATE SET ");
        query_builder.push("series_code = EXCLUDED.series_code, ");
        query_builder.push("friendly_name = EXCLUDED.friendly_name, ");
        query_builder.push("specification_type = EXCLUDED.specification_type, ");
        query_builder.push("locomotive_type = EXCLUDED.locomotive_type, ");
        query_builder.push("locomotive_series = EXCLUDED.locomotive_series, ");
        query_builder.push("service_level = EXCLUDED.service_level, ");
        query_builder.push("passenger_car_type = EXCLUDED.passenger_car_type, ");
        query_builder.push("freight_car_type = EXCLUDED.freight_car_type, ");
        query_builder.push("railcar_type = EXCLUDED.railcar_type, ");
        query_builder.push("electric_multiple_unit_type = EXCLUDED.electric_multiple_unit_type, ");
        query_builder.push("elements_count = EXCLUDED.elements_count, ");
        query_builder.push("is_permanently_coupled = EXCLUDED.is_permanently_coupled, ");
        query_builder.push("is_motorized = EXCLUDED.is_motorized, ");
        query_builder.push("default_is_dummy = EXCLUDED.default_is_dummy, ");
        query_builder.push("updated_at = EXCLUDED.updated_at");

        query_builder
            .build()
            .execute(&mut *tx)
            .await
            .context("Failed to execute prototypes upsert chunk")?;
    }

    tx.commit().await.context("Failed to commit transaction")?;
    Ok(())
}

/// Seed the coupler type catalogue from the embedded CSV file.
///
/// CSV columns (0-based): manufacturer, name, compatible_socket
/// ID is derived as `trn:coupler:{slugify(manufacturer)}:{slugify(name)}`.
/// Uses an `ON CONFLICT(id) DO UPDATE SET` upsert so it is safe to call on every startup.
pub async fn seed_coupler_types(pool: &SqlitePool) -> anyhow::Result<()> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(COUPLERS.as_bytes());

    let records: Vec<_> = rdr
        .records()
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to parse couplers CSV records")?;

    let mut tx = pool.begin().await.context("Failed to start transaction")?;

    for chunk in records.chunks(CHUNK_SIZE) {
        let mut query_builder: QueryBuilder<sqlx::Sqlite> = QueryBuilder::new(
            "INSERT INTO coupler_types (id, manufacturer, name, compatible_socket) ",
        );

        query_builder.push_values(chunk, |mut b, record| {
            let manufacturer = record.get(0).unwrap_or_default();
            let name = record.get(1).unwrap_or_default();
            let compatible_socket = record.get(2).unwrap_or_default();

            let id = CouplerTypeId::new_from_parts(&[manufacturer, name]).to_string();

            b.push_bind(id)
                .push_bind(manufacturer.to_string())
                .push_bind(name.to_string())
                .push_bind(compatible_socket.to_string());
        });

        query_builder.push(" ON CONFLICT(id) DO UPDATE SET ");
        query_builder.push("manufacturer = EXCLUDED.manufacturer, ");
        query_builder.push("name = EXCLUDED.name, ");
        query_builder.push("compatible_socket = EXCLUDED.compatible_socket");

        query_builder
            .build()
            .execute(&mut *tx)
            .await
            .context("Failed to execute coupler_types upsert chunk")?;
    }

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

    #[sqlx::test(migrations = "./migrations")]
    async fn seeds_decoders(pool: SqlitePool) {
        // Ensure manufacturers exist (decoders have FK -> manufacturers)
        seed_manufacturers(&pool)
            .await
            .expect("seed_manufacturers should run without errors");

        // Run the decoders seeder
        seed_decoders(&pool)
            .await
            .expect("seed_decoders should run without errors");

        let mut conn = pool.acquire().await.expect("acquire conn");

        // Ensure table has rows
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM decoders")
            .fetch_one(&mut *conn)
            .await
            .expect("count query should succeed");
        assert!(count > 0, "expected at least one seeded decoder");

        // Check a concrete seeded entry exists (ESU 58410 -> id trn:decoder:esu:58410)
        let product_code: Option<String> =
            sqlx::query_scalar::<_, String>("SELECT product_code FROM decoders WHERE id = ?")
                .bind("trn:decoder:esu:58410")
                .fetch_optional(&mut *conn)
                .await
                .expect("select product_code query should succeed");

        assert!(
            product_code.is_some(),
            "expected a seeded decoder for id trn:decoder:esu:58410"
        );
        assert_eq!(product_code.unwrap(), "58410");
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn seeds_track_products(pool: SqlitePool) {
        seed_manufacturers(&pool)
            .await
            .expect("seed manufacturers first");
        seed_track_products(&pool)
            .await
            .expect("seed track_products");

        let mut conn = pool.acquire().await.expect("acquire conn");

        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM track_products")
            .fetch_one(&mut *conn)
            .await
            .expect("count query");
        assert!(count > 0, "expected seeded track products");

        // Spot-check: Roco 42410 → track_id = trn:track:roco:42410
        let product_code: Option<String> =
            sqlx::query_scalar("SELECT product_code FROM track_products WHERE track_id = ?")
                .bind("trn:track:roco:42410")
                .fetch_optional(&mut *conn)
                .await
                .expect("select query");
        assert!(product_code.is_some(), "expected trn:track:roco:42410");
        assert_eq!(product_code.unwrap(), "42410");
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn seeds_sellers(pool: SqlitePool) {
        seed_sellers(&pool)
            .await
            .expect("seed_sellers should run without errors");

        let mut conn = pool.acquire().await.expect("acquire conn");

        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM sellers")
            .fetch_one(&mut *conn)
            .await
            .expect("count query should succeed");
        assert!(count > 0, "expected at least one seeded seller");

        let seller_type: Option<String> =
            sqlx::query_scalar("SELECT type FROM sellers WHERE id = ?")
                .bind("trn:seller:model-center")
                .fetch_optional(&mut *conn)
                .await
                .expect("select type query should succeed");

        assert!(
            seller_type.is_some(),
            "expected a seeded seller for id trn:seller:model-center"
        );
        assert_eq!(seller_type.unwrap(), "SHOP");
    }
}
