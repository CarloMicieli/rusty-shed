//! Seed data for the trains module.
//!
//! Provides `insert_default_prototypes()` and `insert_default_categories()`.
//! Both functions use `INSERT OR IGNORE` so they are idempotent and safe to
//! call on every application startup.

use anyhow::Context;
use sqlx::SqlitePool;

/// Seed the default formation categories into `formation_categories`.
///
/// Uses `INSERT OR IGNORE` so running this multiple times is safe.
pub async fn insert_default_categories(pool: &SqlitePool) -> anyhow::Result<()> {
    // Already included in the migration SQL; this function is a no-op guard.
    // The categories are seeded via INSERT OR IGNORE in the migration file.
    // Calling this at startup ensures they exist even on older DBs that
    // are migrating from a pre-039 schema.
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO formation_categories (id, name, is_custom) VALUES
            ('trn:formation-category:eurocity',  'EuroCity',   0),
            ('trn:formation-category:intercity', 'Intercity',  0),
            ('trn:formation-category:tee',       'TEE',        0),
            ('trn:formation-category:express',   'Express',    0),
            ('trn:formation-category:regional',  'Regional',   0),
            ('trn:formation-category:freight',   'Freight',    0),
            ('trn:formation-category:special',   'Special',    0),
            ('trn:formation-category:thematic',  'Thematic',   0)
        "#,
    )
    .execute(pool)
    .await
    .context("Failed to seed formation categories")?;

    Ok(())
}

/// Seed the default prototype catalog into `prototypes`.
///
/// Uses `INSERT OR IGNORE` so running this multiple times is safe.
/// Requires `railway_companies` rows for `trn:railway-company:fs`,
/// `trn:railway-company:sbb-cff-ffs`, and `trn:railway-company:db` to exist.
pub async fn insert_default_prototypes(pool: &SqlitePool) -> anyhow::Result<()> {
    // (id, railway_company_id, series_code, car_type, service_level, category, is_motorized, default_is_dummy)
    type PrototypeSeedRow<'a> = (
        &'a str,
        &'a str,
        &'a str,
        &'a str,
        Option<&'a str>,
        &'a str,
        i64,
        i64,
    );
    let prototypes: &[PrototypeSeedRow<'_>] = &[
        (
            "trn:prototype:fs-e444-tartaruga",
            "trn:railway-company:fs",
            "E.444 Tartaruga",
            "Locomotive",
            None,
            "Locomotive",
            1,
            0,
        ),
        (
            "trn:prototype:fs-e646",
            "trn:railway-company:fs",
            "E.646",
            "Locomotive",
            None,
            "Locomotive",
            1,
            0,
        ),
        (
            "trn:prototype:sbb-re44-ii",
            "trn:railway-company:sbb-cff-ffs",
            "Re 4/4 II",
            "Locomotive",
            None,
            "Locomotive",
            1,
            0,
        ),
        (
            "trn:prototype:db-e103",
            "trn:railway-company:db",
            "Baureihe 103",
            "Locomotive",
            None,
            "Locomotive",
            1,
            0,
        ),
        (
            "trn:prototype:fs-uic-z1-gran-comfort",
            "trn:railway-company:fs",
            "UIC-Z1 Gran Comfort",
            "Coach",
            Some("1st Class"),
            "Passenger",
            0,
            0,
        ),
        (
            "trn:prototype:fs-uic-x-1cl",
            "trn:railway-company:fs",
            "UIC-X (1982)",
            "Coach",
            Some("1st Class"),
            "Passenger",
            0,
            0,
        ),
        (
            "trn:prototype:fs-uic-x-2cl",
            "trn:railway-company:fs",
            "UIC-X (1982)",
            "Coach",
            Some("2nd Class"),
            "Passenger",
            0,
            0,
        ),
        (
            "trn:prototype:sbb-ewiv-1cl",
            "trn:railway-company:sbb-cff-ffs",
            "EW IV",
            "Coach",
            Some("1st Class"),
            "Passenger",
            0,
            0,
        ),
        (
            "trn:prototype:db-avmz-eurocity",
            "trn:railway-company:db",
            "Avmz (EuroCity)",
            "Coach",
            Some("1st Class"),
            "Passenger",
            0,
            0,
        ),
    ];

    let now = chrono::Utc::now().to_rfc3339();

    for (
        id,
        company_id,
        series_code,
        car_type,
        service_level,
        category,
        is_motorized,
        default_is_dummy,
    ) in prototypes
    {
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO prototypes
                (id, railway_company_id, series_code, car_type, service_level,
                 category, is_motorized, default_is_dummy, is_custom, created_at, updated_at, version)
            VALUES
                (?, ?, ?, ?, ?, ?, ?, ?, 0, ?, ?, 0)
            "#,
        )
        .bind(id)
        .bind(company_id)
        .bind(series_code)
        .bind(car_type)
        .bind(service_level)
        .bind(category)
        .bind(is_motorized)
        .bind(default_is_dummy)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await
        .context(format!("Failed to seed prototype {id}"))?;
    }

    Ok(())
}
