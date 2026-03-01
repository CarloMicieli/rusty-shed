use crate::catalog::domain::railway_model::RailwayModelId;
use crate::collecting::domain::CollectionItemId;
use crate::collecting::domain::PurchaseCondition;
use crate::core::domain::domain_error::DomainError;
use crate::core::domain::{Currency, MonetaryAmount};
use crate::dashboard::domain::{
    DashboardRecentItem, DashboardTotals, ModelCard, PurchaseGroup, Source,
};
use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct DashboardRecentItemRow {
    pub id: RailwayModelId,
    pub title: String,
    pub subtitle: Option<String>,
    pub source: Source,
    pub created_at: NaiveDateTime,
}

impl TryFrom<DashboardRecentItemRow> for DashboardRecentItem {
    type Error = DomainError;

    fn try_from(row: DashboardRecentItemRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.id,
            title: row.title,
            subtitle: row.subtitle,
            source: row.source,
            created_at: row.created_at,
        })
    }
}

#[derive(Debug, Clone, Default, FromRow)]
pub struct DashboardTotalsRow {
    pub collection_items: u32,
    pub wishlists: u32,
    pub maintenance_due: u32,
    pub total_value_amount: Option<i64>,
    pub total_value_currency: Option<Currency>,
}

impl TryFrom<DashboardTotalsRow> for DashboardTotals {
    type Error = DomainError;

    fn try_from(row: DashboardTotalsRow) -> Result<Self, Self::Error> {
        Ok(Self {
            collection_items: row.collection_items,
            wishlists: row.wishlists,
            maintenance_due: row.maintenance_due,
            total_value: match (row.total_value_amount, row.total_value_currency) {
                (Some(amount), Some(currency)) => Some(MonetaryAmount { amount, currency }),
                _ => None,
            },
        })
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct PurchaseGroupRow {
    pub purchase_date: String,
    pub seller_id: Option<String>,
    pub seller_name: Option<String>,
    pub notes: Option<String>,
    pub model_count: i64,
}

#[derive(Debug, Clone, FromRow)]
pub struct ModelCardRow {
    pub collection_item_id: CollectionItemId,
    pub manufacturer_name: String,
    pub product_code: String,
    pub description: String,
    pub image_path: Option<String>,
    pub purchase_condition: Option<String>,
}

impl From<ModelCardRow> for ModelCard {
    fn from(row: ModelCardRow) -> Self {
        let condition = row
            .purchase_condition
            .as_deref()
            .and_then(|s| s.parse::<PurchaseCondition>().ok())
            .unwrap_or_default();

        Self {
            id: row.collection_item_id,
            thumbnail_path: row.image_path,
            manufacturer: row.manufacturer_name,
            product_code: row.product_code,
            condition,
            description: row.description,
        }
    }
}

impl From<(PurchaseGroupRow, Vec<ModelCardRow>)> for PurchaseGroup {
    fn from(value: (PurchaseGroupRow, Vec<ModelCardRow>)) -> Self {
        let (group_row, model_rows) = value;

        // Generate stable ID from purchase_date and seller_id
        let id = match &group_row.seller_id {
            Some(seller_id) => format!("purchase-{}-{}", group_row.purchase_date, seller_id),
            None => format!("purchase-{}-unknown", group_row.purchase_date),
        };

        // Take only first 3 model cards for display
        let model_cards: Vec<ModelCard> = model_rows.into_iter().take(3).map(Into::into).collect();

        Self {
            id,
            purchase_date: group_row.purchase_date,
            seller_name: group_row.seller_name,
            notes: group_row.notes,
            model_cards,
            total_count: group_row.model_count as usize,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::Currency;

    #[test]
    fn it_should_convert_row_to_dashboard_recent_item() {
        let railway_model_id = RailwayModelId::try_from("trn:railway-model:acme:12345").unwrap();
        let now = NaiveDateTime::default();
        let row = DashboardRecentItemRow {
            id: railway_model_id.clone(),
            title: "Recent Model".to_string(),
            subtitle: Some("Subtitle".to_string()),
            source: Source::Collection,
            created_at: now,
        };
        let item: DashboardRecentItem = row.try_into().unwrap();
        assert_eq!(item.id, railway_model_id);
        assert_eq!(item.title, "Recent Model");
        assert_eq!(item.subtitle.unwrap(), "Subtitle");
        assert_eq!(item.source, Source::Collection);
        assert_eq!(item.created_at, now);
    }

    #[test]
    fn it_should_convert_row_to_dashboard_totals() {
        let row = DashboardTotalsRow {
            collection_items: 100,
            wishlists: 20,
            maintenance_due: 5,
            total_value_amount: Some(50000),
            total_value_currency: Some(Currency::USD),
        };
        let totals: DashboardTotals = row.try_into().unwrap();
        assert_eq!(totals.collection_items, 100);
        assert_eq!(totals.wishlists, 20);
        assert_eq!(totals.maintenance_due, 5);
        assert!(totals.total_value.is_some());
        let total_value = totals.total_value.as_ref().unwrap();
        assert_eq!(total_value.amount, 50000);
        assert_eq!(total_value.currency, Currency::USD);
    }
}
