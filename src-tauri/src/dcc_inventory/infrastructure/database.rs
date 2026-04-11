use crate::collecting::domain::OwnedRollingStockId;
use crate::dcc_inventory::domain::{DecoderId, DigitalRollingStockId};
use crate::dcc_inventory::infrastructure::entities::{
    DecoderRow, DigitalRollingStockRow, EnrichedRow, InstallableRow, SummaryRow,
};

// ---------------------------------------------------------------------------
// Raw SQL functions
// ---------------------------------------------------------------------------

/// Fetch a single `digital_rolling_stocks` row by primary key.
///
/// Returns `Ok(None)` when no row matches.
///
/// # Errors
/// Propagates any [`sqlx::Error`] from the underlying query.
pub async fn find_digital_rolling_stock_by_id(
    executor: &mut sqlx::SqliteConnection,
    id: &DigitalRollingStockId,
) -> Result<Option<DigitalRollingStockRow>, sqlx::Error> {
    let sql = r#"
        SELECT id, owned_rolling_stock_id, dcc_address, installed_decoder_id
        FROM digital_rolling_stocks
        WHERE id = ?1
        LIMIT 1
    "#;

    sqlx::query_as::<_, DigitalRollingStockRow>(sql)
        .bind(id)
        .fetch_optional(executor)
        .await
}

/// Insert a new row into `digital_rolling_stocks` (handles the `Created` event).
///
/// # Errors
/// Propagates any [`sqlx::Error`] from the underlying query.
pub async fn insert_digital_rolling_stock(
    executor: &mut sqlx::SqliteConnection,
    id: &DigitalRollingStockId,
    owned_rolling_stock_id: &OwnedRollingStockId,
    dcc_address: u16,
    decoder_id: Option<DecoderId>,
) -> Result<(), sqlx::Error> {
    let sql = r#"
        INSERT INTO digital_rolling_stocks
            (id, owned_rolling_stock_id, dcc_address, installed_decoder_id)
        VALUES (?1, ?2, ?3, ?4)
    "#;

    sqlx::query(sql)
        .bind(id)
        .bind(owned_rolling_stock_id)
        .bind(dcc_address)
        .bind(decoder_id)
        .execute(executor)
        .await?;

    Ok(())
}

/// Update `installed_decoder_id` for an existing digital rolling stock row.
///
/// # Errors
/// Propagates any [`sqlx::Error`] from the underlying query.
pub async fn update_digital_rolling_stock_decoder(
    executor: &mut sqlx::SqliteConnection,
    id: &DigitalRollingStockId,
    decoder_id: Option<DecoderId>,
) -> Result<(), sqlx::Error> {
    let sql = r#"
        UPDATE digital_rolling_stocks
        SET installed_decoder_id = ?1
        WHERE id = ?2
    "#;

    sqlx::query(sql)
        .bind(decoder_id)
        .bind(id)
        .execute(executor)
        .await?;

    Ok(())
}

/// Update the `dcc_address` for an existing digital rolling stock row.
///
/// # Errors
/// Propagates any [`sqlx::Error`] from the underlying query.
pub async fn update_digital_rolling_stock_address(
    executor: &mut sqlx::SqliteConnection,
    id: &DigitalRollingStockId,
    dcc_address: u16,
) -> Result<(), sqlx::Error> {
    let sql = r#"
        UPDATE digital_rolling_stocks
        SET dcc_address = ?1
        WHERE id = ?2
    "#;

    sqlx::query(sql)
        .bind(dcc_address)
        .bind(id)
        .execute(executor)
        .await?;

    Ok(())
}

/// Run the enriched JOIN query that backs the `DigitalRollingStockView` list.
///
/// Excludes `FUNCTION`-type decoders and orders by `dcc_address ASC`.
///
/// # Errors
/// Propagates any [`sqlx::Error`] from the underlying query.
pub async fn find_all_digital_rolling_stocks_view(
    executor: &mut sqlx::SqliteConnection,
) -> Result<Vec<EnrichedRow>, sqlx::Error> {
    let sql = r#"
        SELECT
            drs.id,
            drs.owned_rolling_stock_id,
            drs.dcc_address,
            d.id AS decoder_id,
            d.product_code AS decoder_product_code,
            d.decoder_type,
            d.protocol AS decoder_protocol,
            d.decoder_interface,
            m.name AS manufacturer_name,
            rs.category,
            rs.road_number,
            rs.series_code,
            rs.series AS description,
            rc.name AS railway_company_name,
            rm.scale,
            rm.power_method
        FROM digital_rolling_stocks drs
        JOIN decoders d ON drs.installed_decoder_id = d.id
        LEFT JOIN manufacturers m ON d.manufacturer_id = m.id
        JOIN owned_rolling_stocks ors ON drs.owned_rolling_stock_id = ors.id
        LEFT JOIN rolling_stocks rs ON ors.rolling_stock_id = rs.id
        LEFT JOIN railway_companies rc ON rs.railway_company_id = rc.id
        LEFT JOIN railway_models rm ON rs.railway_model_id = rm.id
        WHERE d.decoder_type != 'FUNCTION'
        ORDER BY drs.dcc_address ASC
    "#;

    sqlx::query_as::<_, EnrichedRow>(sql)
        .fetch_all(executor)
        .await
}

/// Run the COUNT/CASE summary query for the digital overview dashboard.
///
/// # Errors
/// Propagates any [`sqlx::Error`] from the underlying query.
pub async fn get_digital_summary(
    executor: &mut sqlx::SqliteConnection,
) -> Result<SummaryRow, sqlx::Error> {
    let sql = r#"
        SELECT
            COALESCE(SUM(CASE WHEN rs.is_dummy = 0 OR rs.is_dummy IS NULL THEN 1 ELSE 0 END), 0) as total_non_dummy,
            COALESCE(SUM(
                CASE
                    WHEN (rs.is_dummy = 0 OR rs.is_dummy IS NULL)
                    AND (rs.control IN ('DCC_SOUND', 'DCC_FITTED') OR drs.id IS NOT NULL)
                    THEN 1
                    ELSE 0
                END
            ), 0) as digital_count
        FROM owned_rolling_stocks ors
        LEFT JOIN rolling_stocks rs ON ors.rolling_stock_id = rs.id
        LEFT JOIN digital_rolling_stocks drs ON drs.owned_rolling_stock_id = ors.id
        JOIN collection_items ci ON ors.collection_item_id = ci.id
        WHERE ci.removed_date IS NULL
    "#;

    sqlx::query_as::<_, SummaryRow>(sql)
        .fetch_one(executor)
        .await
}

/// Check whether a DCC address is already assigned, optionally excluding one
/// record by its ID (used when updating an existing entry).
///
/// Returns the [`DigitalRollingStockId`] of the conflicting row, or `None` if
/// the address is free.
///
/// # Errors
/// Propagates any [`sqlx::Error`] from the underlying query.
pub async fn check_address_exists(
    executor: &mut sqlx::SqliteConnection,
    address: u16,
    exclude_id: Option<&DigitalRollingStockId>,
) -> Result<Option<DigitalRollingStockId>, sqlx::Error> {
    let sql = r#"
        SELECT id
        FROM digital_rolling_stocks
        WHERE dcc_address = ?1
        AND id != COALESCE(?2, '')
        LIMIT 1
    "#;

    let exclude_id_str = exclude_id.map(|id| id.to_string()).unwrap_or_default();

    let row: Option<(DigitalRollingStockId,)> = sqlx::query_as(sql)
        .bind(address)
        .bind(&exclude_id_str)
        .fetch_optional(executor)
        .await?;

    Ok(row.map(|(id,)| id))
}

/// Query rolling stocks that are eligible for DCC decoder installation.
///
/// Excludes dummy models and already-removed collection items.
///
/// # Errors
/// Propagates any [`sqlx::Error`] from the underlying query.
pub async fn find_installable_rolling_stocks(
    executor: &mut sqlx::SqliteConnection,
) -> Result<Vec<InstallableRow>, sqlx::Error> {
    let sql = r#"
        SELECT
            ors.id AS owned_rolling_stock_id,
            rs.category,
            rs.road_number,
            rs.series_code,
            rc.name AS railway_company_name,
            CASE WHEN drs.id IS NOT NULL THEN 1 ELSE 0 END AS has_decoder,
            rs.dcc_interface
        FROM owned_rolling_stocks ors
        LEFT JOIN rolling_stocks rs ON ors.rolling_stock_id = rs.id
        LEFT JOIN railway_companies rc ON rs.railway_company_id = rc.id
        LEFT JOIN digital_rolling_stocks drs ON drs.owned_rolling_stock_id = ors.id
        JOIN collection_items ci ON ors.collection_item_id = ci.id
        WHERE ci.removed_date IS NULL
        AND (rs.is_dummy = 0 OR rs.is_dummy IS NULL)
        ORDER BY rs.road_number ASC, ors.id ASC
    "#;

    sqlx::query_as::<_, InstallableRow>(sql)
        .fetch_all(executor)
        .await
}

/// Fetch all decoder rows ordered by `id`.
///
/// # Errors
/// Propagates any [`sqlx::Error`] from the underlying query.
pub async fn find_all_decoders(
    executor: &mut sqlx::SqliteConnection,
) -> Result<Vec<DecoderRow>, sqlx::Error> {
    let sql = r#"
        SELECT id, manufacturer_id, product_code, decoder_type, protocol, decoder_interface
        FROM decoders
        ORDER BY id
    "#;

    sqlx::query_as::<_, DecoderRow>(sql)
        .fetch_all(executor)
        .await
}
