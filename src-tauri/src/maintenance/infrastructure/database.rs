use crate::maintenance::infrastructure::entities::{
    MaintenanceCardRow, MaintenanceCardWithDisplayInfoRow, MaintenanceEventRow,
};
use sqlx::SqliteConnection;

/// Fetch a single [`MaintenanceCardRow`] for the given `owned_rolling_stock_id` TRN.
///
/// Returns `None` when no card exists for that stock item.
pub async fn find_maintenance_card_by_stock_id(
    executor: &mut SqliteConnection,
    owned_rolling_stock_trn: &str,
) -> Result<Option<MaintenanceCardRow>, sqlx::Error> {
    let sql = r#"SELECT
        id,
        owned_rolling_stock_id,
        last_maintenance_date,
        next_maintenance_date,
        created_at,
        updated_at,
        version
    FROM maintenance_cards
    WHERE owned_rolling_stock_id = ?"#;

    sqlx::query_as::<_, MaintenanceCardRow>(sql)
        .bind(owned_rolling_stock_trn)
        .fetch_optional(executor)
        .await
}

/// Fetch a single [`MaintenanceCardRow`] for the given maintenance card TRN.
///
/// Returns `None` when no card exists with that id.
pub async fn find_maintenance_card_by_id(
    executor: &mut SqliteConnection,
    card_trn: &str,
) -> Result<Option<MaintenanceCardRow>, sqlx::Error> {
    let sql = r#"SELECT
        id,
        owned_rolling_stock_id,
        last_maintenance_date,
        next_maintenance_date,
        created_at,
        updated_at,
        version
    FROM maintenance_cards
    WHERE id = ?"#;

    sqlx::query_as::<_, MaintenanceCardRow>(sql)
        .bind(card_trn)
        .fetch_optional(executor)
        .await
}

/// Fetch a single [`MaintenanceCardWithDisplayInfoRow`] for the given maintenance card TRN,
/// joining through `owned_rolling_stocks` → `rolling_stocks` → `railway_models` → `manufacturers`
/// to include human-readable display information.
///
/// Returns `None` when no card exists with that id.
pub async fn find_maintenance_card_with_display_by_id(
    executor: &mut SqliteConnection,
    card_trn: &str,
) -> Result<Option<MaintenanceCardWithDisplayInfoRow>, sqlx::Error> {
    let sql = r#"SELECT
        mc.id,
        mc.owned_rolling_stock_id,
        mc.last_maintenance_date,
        mc.next_maintenance_date,
        mfr.name            AS manufacturer_name,
        rm.product_code     AS product_code,
        rs.series_code      AS series_code,
        rs.road_number      AS road_number,
        rs.category         AS rolling_stock_category
    FROM maintenance_cards mc
    LEFT JOIN owned_rolling_stocks ors ON mc.owned_rolling_stock_id = ors.id
    LEFT JOIN rolling_stocks rs        ON ors.rolling_stock_id = rs.id
    LEFT JOIN railway_models rm        ON rs.railway_model_id = rm.id
    LEFT JOIN manufacturers mfr        ON rm.manufacturer_id = mfr.id
    WHERE mc.id = ?"#;

    sqlx::query_as::<_, MaintenanceCardWithDisplayInfoRow>(sql)
        .bind(card_trn)
        .fetch_optional(executor)
        .await
}

/// Load all [`MaintenanceEventRow`]s for a given maintenance card TRN, ordered most-recent first.
///
/// Used by both the domain hydration path and the view-model path.
pub async fn load_events_for_card(
    executor: &mut SqliteConnection,
    card_trn: &str,
) -> Result<Vec<MaintenanceEventRow>, sqlx::Error> {
    let sql = r#"SELECT
        id,
        maintenance_card_id,
        date_performed,
        notes,
        maintenance_type
    FROM maintenance_events
    WHERE maintenance_card_id = ?
    ORDER BY date_performed DESC"#;

    sqlx::query_as::<_, MaintenanceEventRow>(sql)
        .bind(card_trn)
        .fetch_all(executor)
        .await
}

/// Insert a single row into `maintenance_events`.
///
/// # Parameters
/// - `event_trn`       — TRN string for the new event (`trn:maintenance-event:<uuid>`)
/// - `card_trn`        — TRN string for the owning card  (`trn:maintenance-card:<uuid>`)
/// - `date_performed`  — Formatted as `YYYY-MM-DD`
/// - `maintenance_type` — Optional type string (e.g. `"INSPECTION"`)
/// - `notes`           — Optional free-text notes
pub async fn insert_maintenance_event(
    executor: &mut SqliteConnection,
    event_trn: &str,
    card_trn: &str,
    date_performed: &str,
    maintenance_type: Option<&str>,
    notes: Option<&str>,
) -> Result<(), sqlx::Error> {
    let sql = r#"INSERT INTO maintenance_events (
        id,
        maintenance_card_id,
        date_performed,
        maintenance_type,
        notes
    ) VALUES (?, ?, ?, ?, ?)"#;

    sqlx::query(sql)
        .bind(event_trn)
        .bind(card_trn)
        .bind(date_performed)
        .bind(maintenance_type)
        .bind(notes)
        .execute(executor)
        .await?;

    Ok(())
}

/// Set `last_maintenance_date` on a maintenance card and bump `updated_at`.
///
/// # Parameters
/// - `date_performed` — Formatted as `YYYY-MM-DD`
/// - `card_trn`       — TRN string for the owning card
pub async fn update_maintenance_card_last_date(
    executor: &mut SqliteConnection,
    date_performed: &str,
    card_trn: &str,
) -> Result<(), sqlx::Error> {
    let sql = r#"UPDATE maintenance_cards
        SET
            last_maintenance_date = ?,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = ?"#;

    sqlx::query(sql)
        .bind(date_performed)
        .bind(card_trn)
        .execute(executor)
        .await?;

    Ok(())
}

/// Set `next_maintenance_date` on a maintenance card and bump `updated_at`.
pub async fn update_maintenance_card_next_date(
    executor: &mut SqliteConnection,
    next_maintenance_date: Option<&str>,
    card_trn: &str,
) -> Result<(), sqlx::Error> {
    let sql = r#"UPDATE maintenance_cards
        SET
            next_maintenance_date = ?,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = ?"#;

    sqlx::query(sql)
        .bind(next_maintenance_date)
        .bind(card_trn)
        .execute(executor)
        .await?;

    Ok(())
}

/// Insert a new row into `maintenance_cards`.
///
/// Returns the raw `sqlx::Error` so the caller can inspect UNIQUE constraint violations
/// and map them to `DomainError::Conflict`.
///
/// # Parameters
/// - `card_trn`               — TRN string for the card (`trn:maintenance-card:<uuid>`)
/// - `owned_rolling_stock_trn` — TRN string for the owned rolling stock
/// - `now_dt`                 — Formatted as `YYYY-MM-DD HH:MM:SS` for `created_at`/`updated_at`
pub async fn insert_maintenance_card(
    executor: &mut SqliteConnection,
    card_trn: &str,
    owned_rolling_stock_trn: &str,
    now_dt: &str,
) -> Result<(), sqlx::Error> {
    let sql = r#"INSERT INTO maintenance_cards (
        id,
        owned_rolling_stock_id,
        created_at,
        updated_at,
        version
    ) VALUES (?, ?, ?, ?, 0)"#;

    sqlx::query(sql)
        .bind(card_trn)
        .bind(owned_rolling_stock_trn)
        .bind(now_dt)
        .bind(now_dt)
        .execute(executor)
        .await?;

    Ok(())
}

/// Fetch all [`MaintenanceCardRow`]s whose maintenance is overdue or has never been performed.
///
/// A card is considered due when any of the following conditions hold:
/// - `next_maintenance_date <= today`
/// - `next_maintenance_date IS NULL` and `last_maintenance_date <= today`
/// - both date columns are `NULL`
pub async fn find_due_maintenance_cards(
    executor: &mut SqliteConnection,
) -> Result<Vec<MaintenanceCardRow>, sqlx::Error> {
    let sql = r#"SELECT
        id,
        owned_rolling_stock_id,
        last_maintenance_date,
        next_maintenance_date,
        created_at,
        updated_at,
        version
    FROM maintenance_cards
    WHERE next_maintenance_date <= date('now')
       OR (
           next_maintenance_date IS NULL
           AND last_maintenance_date IS NOT NULL
           AND last_maintenance_date <= date('now')
       )
       OR (
           next_maintenance_date IS NULL
           AND last_maintenance_date IS NULL
       )"#;

    sqlx::query_as::<_, MaintenanceCardRow>(sql)
        .fetch_all(executor)
        .await
}

/// Look up the `maintenance_card_id` TRN for the given event TRN.
///
/// Returns `None` when the event does not exist (used to return `NotFound` in the caller).
pub async fn find_event_card_id(
    executor: &mut SqliteConnection,
    event_trn: &str,
) -> Result<Option<String>, sqlx::Error> {
    sqlx::query_scalar("SELECT maintenance_card_id FROM maintenance_events WHERE id = ?")
        .bind(event_trn)
        .fetch_optional(executor)
        .await
}

/// Hard-delete a single maintenance event row by its TRN.
pub async fn delete_maintenance_event(
    executor: &mut SqliteConnection,
    event_trn: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM maintenance_events WHERE id = ?")
        .bind(event_trn)
        .execute(executor)
        .await?;

    Ok(())
}

/// Recalculate `last_maintenance_date` for a card from the remaining events after a deletion,
/// then bump `updated_at`.
pub async fn recalculate_last_maintenance_date(
    executor: &mut SqliteConnection,
    card_trn: &str,
) -> Result<(), sqlx::Error> {
    let sql = r#"UPDATE maintenance_cards
       SET last_maintenance_date = (
           SELECT MAX(date_performed)
           FROM maintenance_events
           WHERE maintenance_card_id = ?
       ),
       updated_at = CURRENT_TIMESTAMP
       WHERE id = ?"#;

    sqlx::query(sql)
        .bind(card_trn)
        .bind(card_trn)
        .execute(executor)
        .await?;

    Ok(())
}

/// Fetch all due [`MaintenanceCardWithDisplayInfoRow`]s, joining catalog tables for display info.
///
/// Applies the same due-date logic as [`find_due_maintenance_cards`].
pub async fn find_due_maintenance_card_views(
    executor: &mut SqliteConnection,
) -> Result<Vec<MaintenanceCardWithDisplayInfoRow>, sqlx::Error> {
    let sql = r#"SELECT
        mc.id,
        mc.owned_rolling_stock_id,
        mc.last_maintenance_date,
        mc.next_maintenance_date,
        mfr.name            AS manufacturer_name,
        rm.product_code     AS product_code,
        rs.series_code      AS series_code,
        rs.road_number      AS road_number,
        rs.category         AS rolling_stock_category
    FROM maintenance_cards mc
    LEFT JOIN owned_rolling_stocks ors ON mc.owned_rolling_stock_id = ors.id
    LEFT JOIN rolling_stocks rs        ON ors.rolling_stock_id = rs.id
    LEFT JOIN railway_models rm        ON rs.railway_model_id = rm.id
    LEFT JOIN manufacturers mfr        ON rm.manufacturer_id = mfr.id
    WHERE mc.next_maintenance_date <= date('now')
       OR (
           mc.next_maintenance_date IS NULL
           AND mc.last_maintenance_date IS NOT NULL
           AND mc.last_maintenance_date <= date('now')
       )
       OR (
           mc.next_maintenance_date IS NULL
           AND mc.last_maintenance_date IS NULL
       )"#;

    sqlx::query_as::<_, MaintenanceCardWithDisplayInfoRow>(sql)
        .fetch_all(executor)
        .await
}
