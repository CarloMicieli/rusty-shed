use crate::collecting::infrastructure::entities::{
    CollectionItemRow, CollectionRow, OwnedRollingStockRow, PurchaseInfoRow,
};

use crate::collecting::domain::CollectionId;
use crate::core::domain::domain_error::DomainError;

/// Fetch a single collection row by id.
///
/// Parameters:
/// - `pool`: SQLite connection pool (moved/cloned by caller).
/// - `collection_id`: domain newtype identifying the collection. The function
///   binds the string form of the id to the SQL query.
///
/// Returns `Ok(Some(CollectionRow))` if found, `Ok(None)` if not found, or an
/// `Err` on database errors.
pub async fn get_collection(
    executor: &mut sqlx::SqliteConnection,
    collection_id: &CollectionId,
) -> Result<Option<CollectionRow>, DomainError> {
    let sql = r#"SELECT
             id,
             name,
             locomotives_count,
             passenger_cars_count,
             freight_cars_count,
             train_sets_count,
             railcars_count,
             electric_multiple_units_count,
             total_value_amount,
             total_value_currency,
             created_at,
             updated_at
   FROM collections
   WHERE id = ?1
   LIMIT 1"#;

    let row = sqlx::query_as::<_, CollectionRow>(sql)
        .bind(collection_id.to_string())
        .fetch_optional(executor)
        .await
        .map_err(DomainError::Infrastructure)?;

    Ok(row)
}

/// Fetch all collection items belonging to a collection.
///
/// Returns a vector of `CollectionItemRow`. The `collection_id` is bound as a
/// parameter to the query to avoid string concatenation.
pub async fn get_collection_items(
    executor: &mut sqlx::SqliteConnection,
    collection_id: &CollectionId,
) -> Result<Vec<CollectionItemRow>, DomainError> {
    let sql = r#"SELECT
             ci.id,
             ci.collection_id,
             ci.railway_model_id,
             ci.added_date,
             ci.removed_date,
             ci.purchase_condition,
             ci.model_condition,
             ci.box_condition,
             ci.notes,
             rm.category,
             rm.product_code,
             rm.scale,
             rm.epoch,
             rm.description,
             m.name AS manufacturer
   FROM collection_items ci
   JOIN railway_models rm ON rm.id = ci.railway_model_id
   JOIN manufacturers m ON m.id = rm.manufacturer_id
   WHERE ci.collection_id = ?1"#;

    let rows = sqlx::query_as::<_, CollectionItemRow>(sql)
        .bind(collection_id.to_string())
        .fetch_all(executor)
        .await
        .map_err(DomainError::Infrastructure)?;

    Ok(rows)
}

/// Fetch a single owned rolling stock row by id.
///
/// The function accepts the raw owned rolling stock id string and returns the
/// matching `OwnedRollingStockRow` if present.
pub async fn get_owned_rolling_stock(
    executor: &mut sqlx::SqliteConnection,
    owned_rolling_stock_id: String,
) -> Result<Option<OwnedRollingStockRow>, DomainError> {
    let sql = r#"SELECT
             id,
             collection_item_id,
             rolling_stock_id,
             notes
   FROM owned_rolling_stocks
   WHERE id = ?1
   LIMIT 1"#;

    let row = sqlx::query_as::<_, OwnedRollingStockRow>(sql)
        .bind(owned_rolling_stock_id)
        .fetch_optional(executor)
        .await
        .map_err(DomainError::Infrastructure)?;

    Ok(row)
}

/// Fetch all owned rolling stocks that belong to a collection.
///
/// This performs a join from `owned_rolling_stocks` to `collection_items` and
/// filters by `collection_items.collection_id = ?` using parameter binding.
pub async fn get_owned_rolling_stocks(
    executor: &mut sqlx::SqliteConnection,
    collection_id: &CollectionId,
) -> Result<Vec<OwnedRollingStockRow>, DomainError> {
    // Select owned rolling stocks and LEFT JOIN decoders to include decoder master data
    let sql = r#"SELECT
             ors.id,
             ors.collection_item_id,
             ors.rolling_stock_id,
             ors.notes,
             ors.dcc_address,
             ors.installed_decoder_id,
             d.id AS decoder_id,
             d.manufacturer_id AS decoder_manufacturer_id,
             d.product_code AS decoder_product_code,
             d.decoder_type AS decoder_type,
             d.protocol AS decoder_protocol,
             d.decoder_interface AS decoder_interface
   FROM owned_rolling_stocks AS ors
   JOIN collection_items AS ci ON ci.id = ors.collection_item_id
   LEFT JOIN decoders d ON d.id = ors.installed_decoder_id
   WHERE ci.collection_id = ?1"#;

    let rows = sqlx::query_as::<_, OwnedRollingStockRow>(sql)
        .bind(collection_id.to_string())
        .fetch_all(executor)
        .await
        .map_err(DomainError::Infrastructure)?;

    Ok(rows)
}

/// Fetch all purchase infos associated to a collection (via collection_items).
///
/// Joins `purchase_infos` to `collection_items` and binds the collection id
/// parameter to prevent SQL injection.
pub async fn get_purchase_infos(
    executor: &mut sqlx::SqliteConnection,
    collection_id: &CollectionId,
) -> Result<Vec<PurchaseInfoRow>, DomainError> {
    let sql = r#"SELECT
             pi.id,
             pi.collection_item_id,
             pi.purchase_type,
             pi.purchase_date,
             pi.seller_id,
             pi.buyer_id,
             pi.sale_date,
             pi.purchased_price_amount,
             pi.purchased_price_currency,
             pi.sale_price_amount,
             pi.sale_price_currency,
             pi.deposit_amount,
             pi.deposit_currency,
             pi.preorder_total_amount,
             pi.preorder_total_currency,
             pi.expected_date
   FROM purchase_infos pi
   JOIN collection_items ci ON ci.id = pi.collection_item_id
   WHERE ci.collection_id = ?1"#;

    let rows = sqlx::query_as::<_, PurchaseInfoRow>(sql)
        .bind(collection_id.to_string())
        .fetch_all(executor)
        .await
        .map_err(DomainError::Infrastructure)?;

    Ok(rows)
}

#[cfg(test)]
mod tests {
    use crate::catalog::domain::railway_model::Category;
    use crate::collecting::domain::{
        BoxCondition, CollectionId, ModelCondition, PurchaseCondition,
    };
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use sqlx::Sqlite;
    use sqlx::pool::PoolConnection;

    #[sqlx::test(migrations = "./migrations")]
    async fn get_collection_returns_none(mut conn: PoolConnection<Sqlite>) -> Result<()> {
        let result = super::get_collection(&mut conn, &CollectionId::default()).await?;

        assert!(result.is_none());

        Ok(())
    }

    #[sqlx::test(migrations = "./migrations", fixtures("test_collection"))]
    async fn get_collection_returns_some(mut conn: PoolConnection<Sqlite>) -> Result<()> {
        let result = super::get_collection(&mut conn, &CollectionId::default()).await?;

        assert!(result.is_some());

        let collection = result.unwrap();
        assert_eq!(collection.id, CollectionId::default());
        // Fixture sets the collection name to 'Test Collection'
        assert_eq!(collection.name, "Test Collection");

        Ok(())
    }

    #[sqlx::test(migrations = "./migrations", fixtures("test_collection"))]
    async fn get_collection_items(mut conn: PoolConnection<Sqlite>) -> Result<()> {
        let result = super::get_collection_items(&mut conn, &CollectionId::default()).await?;

        assert_eq!(result.len(), 1);

        let collection_item_row = &result[0];
        let expected_collection_item_id =
            "trn:collection-item:d20a1a95-1ae4-4970-9e87-b4c84676e730"
                .try_into()
                .expect("should be valid CollectionItemId");
        assert_eq!(collection_item_row.id, expected_collection_item_id);
        assert_eq!(collection_item_row.collection_id, CollectionId::default());
        let expected_railway_model_id = "trn:railway-model:acme:60100"
            .try_into()
            .expect("should be valid railwayModelId");
        assert_eq!(
            collection_item_row.railway_model_id,
            expected_railway_model_id
        );
        assert_eq!(
            collection_item_row.purchase_condition,
            Some(PurchaseCondition::New)
        );
        assert_eq!(
            collection_item_row.box_condition,
            Some(BoxCondition::OriginalMint)
        );
        assert_eq!(
            collection_item_row.model_condition,
            Some(ModelCondition::Mint)
        );
        assert_eq!(
            collection_item_row.notes,
            Some("My notes go here".to_string())
        );
        assert_eq!(collection_item_row.category, Category::Locomotives);
        assert_eq!(
            collection_item_row.description,
            "Locomotiva elettrica E.444.005 Tartaruga"
        );
        assert_eq!(collection_item_row.epoch, "IV".into());
        assert_eq!(collection_item_row.scale, "H0".try_into()?);

        Ok(())
    }

    #[sqlx::test(migrations = "./migrations", fixtures("test_collection"))]
    async fn get_owned_rolling_stocks(mut conn: PoolConnection<Sqlite>) -> Result<()> {
        let result = super::get_owned_rolling_stocks(&mut conn, &CollectionId::default()).await?;

        assert_eq!(result.len(), 1);

        let owned_rolling_stock_row = &result[0];
        let expected_owned_rolling_stock_id =
            "trn:owned-rolling-stock:77122924-783e-4f3c-a6b5-f4caec9e695d"
                .try_into()
                .expect("should be valid OwnedRollingStockId");
        assert_eq!(owned_rolling_stock_row.id, expected_owned_rolling_stock_id);

        let expected_collection_item_id =
            "trn:collection-item:d20a1a95-1ae4-4970-9e87-b4c84676e730"
                .try_into()
                .expect("should be valid CollectionItemId");
        assert_eq!(
            owned_rolling_stock_row.collection_item_id,
            expected_collection_item_id
        );
        // rolling_stock_id and notes are optional in the entity mapping
        let expected_rolling_stock_id = "trn:rolling-stock:70300b1c-b1df-475f-a7be-291e435b1cf8"
            .try_into()
            .expect("should be valid RollingStockId");
        assert_eq!(
            owned_rolling_stock_row.rolling_stock_id,
            Some(expected_rolling_stock_id)
        );
        assert_eq!(
            owned_rolling_stock_row.notes,
            Some("My rolling stock notes go here".to_string())
        );

        Ok(())
    }

    #[sqlx::test(migrations = "./migrations", fixtures("test_collection"))]
    async fn get_purchase_infos(mut conn: PoolConnection<Sqlite>) -> Result<()> {
        let result = super::get_purchase_infos(&mut conn, &CollectionId::default()).await?;

        assert_eq!(result.len(), 1);

        let expected_purchase_info_id = "trn:purchase:59adc26d-0274-4d6b-8c14-61e598d3fe0e"
            .try_into()
            .expect("should be valid PurchaseInfoId");
        let expected_collection_item_id =
            "trn:collection-item:d20a1a95-1ae4-4970-9e87-b4c84676e730"
                .try_into()
                .expect("should be valid CollectionItemId");

        let purchase_info_row = &result[0];

        assert_eq!(purchase_info_row.id, expected_purchase_info_id);
        assert_eq!(
            purchase_info_row.collection_item_id,
            expected_collection_item_id
        );
        assert_eq!(
            purchase_info_row.purchase_type,
            Some("PURCHASED".to_string())
        );
        // purchase_date is a NaiveDate; compare its string form to the fixture date
        assert_eq!(purchase_info_row.purchase_date.to_string(), "2025-12-26");
        assert_eq!(purchase_info_row.purchased_price_amount, Some(17500));
        assert_eq!(
            purchase_info_row.purchased_price_currency,
            Some("EUR".to_string())
        );

        Ok(())
    }
}
