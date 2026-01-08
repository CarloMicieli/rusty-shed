use crate::catalog::domain::manufacturer::ManufacturerId;
use crate::catalog::domain::railway_company::RailwayCompanyId;
use crate::catalog::domain::railway_model::{Category, Epoch, PowerMethod, RailwayModelId};
use crate::catalog::domain::scale::Scale;
use crate::core::domain::domain_error::DomainError;
use crate::core::domain::{Currency, MonetaryAmount};
use crate::dashboard::domain::{
    DashboardDepotEntry, DashboardDepotManufacturerEntry, DashboardDepotRailwayCompanyEntry,
    DashboardRecentItem, DashboardTotals, Source,
};
use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct DashboardDepotEntryRow {
    pub id: RailwayModelId,
    pub manufacturer_id: ManufacturerId,
    pub manufacturer_name: String,
    pub product_code: String,
    pub category: Category,
    pub scale: Scale,
    pub epoch: Epoch,
    pub railway_company_id: RailwayCompanyId,
    pub railway_company_name: String,
    pub railway_company_country_code: Option<String>,
    pub description: String,
    pub power_method: PowerMethod,
}

impl TryFrom<DashboardDepotEntryRow> for DashboardDepotEntry {
    type Error = DomainError;

    fn try_from(row: DashboardDepotEntryRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.id,
            manufacturer: DashboardDepotManufacturerEntry {
                manufacturer_id: row.manufacturer_id,
                name: row.manufacturer_name,
            },
            product_code: row.product_code,
            category: row.category,
            scale: row.scale,
            epoch: row.epoch,
            railway_company: DashboardDepotRailwayCompanyEntry {
                railway_company_id: row.railway_company_id,
                name: row.railway_company_name,
                country_code: row.railway_company_country_code,
            },
            description: row.description,
            power_method: row.power_method,
        })
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::domain::railway_company::RailwayCompanyId;
    use crate::catalog::domain::railway_model::{Category, PowerMethod};
    use crate::catalog::domain::scale::Scale;
    use crate::core::domain::Currency;

    #[test]
    fn it_should_convert_row_to_dashboard_depot_entry() {
        let manufacturer_id = ManufacturerId::try_from("trn:manufacturer:acme").unwrap();
        let railway_company_id = RailwayCompanyId::try_from("trn:railway-company:fs").unwrap();
        let railway_model_id = RailwayModelId::try_from("trn:railway-model:acme:12345").unwrap();

        let row = DashboardDepotEntryRow {
            id: railway_model_id.clone(),
            manufacturer_id: manufacturer_id.clone(),
            manufacturer_name: "Test Manufacturer".to_string(),
            product_code: "TM123".to_string(),
            category: Category::Locomotives,
            scale: Scale::H0,
            epoch: "IV".into(),
            railway_company_id: railway_company_id.clone(),
            railway_company_name: "Test Railway".to_string(),
            railway_company_country_code: Some("US".to_string()),
            description: "A test railway model".to_string(),
            power_method: PowerMethod::DC,
        };

        let entry: DashboardDepotEntry = row.try_into().unwrap();
        assert_eq!(entry.id, railway_model_id);
        assert_eq!(entry.product_code, "TM123");
        assert_eq!(entry.category, Category::Locomotives);
        assert_eq!(entry.scale, Scale::H0);
        assert_eq!(entry.epoch, "IV".into());
        assert_eq!(entry.description, "A test railway model");
        assert_eq!(entry.power_method, PowerMethod::DC);
        assert_eq!(entry.manufacturer.manufacturer_id, manufacturer_id);
        assert_eq!(entry.manufacturer.name, "Test Manufacturer");
        assert_eq!(entry.railway_company.railway_company_id, railway_company_id);
        assert_eq!(entry.railway_company.name, "Test Railway");
    }

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
