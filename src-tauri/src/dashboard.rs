use crate::collecting::application::get_collection::GetCollectionUseCase;
use crate::collecting::domain::collection::Collection;
use crate::core::domain::monetary_amount::MonetaryAmount;
use crate::core::infrastructure::error::CommandError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::maintenance::application::get_maintenance_dashboard::GetMaintenanceDashboardUseCase;
use crate::state::AppState;
use crate::wishlist::application::get_wishlists::GetWishlistsUseCase;
use serde::Serialize;
use specta::Type;
use tauri::State;

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct DashboardTotals {
    pub collection_items: u32,
    pub wishlists: u32,
    pub maintenance_due: u32,
    pub total_value: Option<MonetaryAmountSummary>,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct MonetaryAmountSummary {
    pub amount: u64,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct DashboardRecentItem {
    pub id: String,
    pub title: String,
    pub subtitle: Option<String>,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct DashboardDepotEntry {
    pub id: String,
    pub manufacturer: Option<String>,
    pub product_code: Option<String>,
    pub category: Option<String>,
    pub scale: Option<String>,
    pub railway_company: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct DashboardSummary {
    pub totals: DashboardTotals,
    pub recent_items: Vec<DashboardRecentItem>,
    pub depot_items: Vec<DashboardDepotEntry>,
}

#[tauri::command]
#[specta::specta]
pub async fn dashboard_summary(
    state: State<'_, AppState>,
) -> Result<DashboardSummary, CommandError> {
    let pool = state.db_pool();

    // Collection snapshot - always return a collection (either existing or default empty)
    let mut collecting_uow = SqliteUnitOfWork::new(&pool)
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;
    let collection = GetCollectionUseCase::new()
        .execute(&mut collecting_uow)
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;
    collecting_uow
        .commit()
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    // Wishlists snapshot - can be empty array
    let mut wishlist_uow = SqliteUnitOfWork::new(&pool)
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;
    let wishlists = GetWishlistsUseCase
        .execute(&mut wishlist_uow)
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;
    wishlist_uow
        .commit()
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    // Maintenance snapshot
    let mut maintenance_uow = SqliteUnitOfWork::new(&pool)
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;
    let maintenance_cards = GetMaintenanceDashboardUseCase::new()
        .execute(&mut maintenance_uow)
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;
    maintenance_uow
        .commit()
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    let totals = DashboardTotals {
        collection_items: collection.items.len() as u32,
        wishlists: wishlists.len() as u32,
        maintenance_due: maintenance_cards.len() as u32,
        total_value: collection.total_value.as_ref().map(monetary_summary),
    };

    let recent_items = pick_recent_items(&collection);
    let depot_items = pick_depot_entries(&collection);

    Ok(DashboardSummary {
        totals,
        recent_items,
        depot_items,
    })
}

fn monetary_summary(m: &MonetaryAmount) -> MonetaryAmountSummary {
    MonetaryAmountSummary {
        amount: m.amount,
        currency: m.currency.to_code().to_string(),
    }
}

fn pick_recent_items(collection: &Collection) -> Vec<DashboardRecentItem> {
    collection
        .items
        .iter()
        .rev()
        .take(3)
        .map(|item| {
            let (manufacturer, product_code) = parse_railway_model_id(&item.railway_model_id);
            DashboardRecentItem {
                id: item.id.to_string(),
                title: product_code
                    .clone()
                    .unwrap_or_else(|| item.railway_model_id.clone()),
                subtitle: manufacturer,
            }
        })
        .collect()
}

fn pick_depot_entries(collection: &Collection) -> Vec<DashboardDepotEntry> {
    collection
        .items
        .iter()
        .take(5)
        .map(|item| {
            let (manufacturer, product_code) = parse_railway_model_id(&item.railway_model_id);
            DashboardDepotEntry {
                id: item.id.to_string(),
                manufacturer,
                product_code,
                category: None,
                scale: None,
                railway_company: None,
                description: item.notes.clone(),
            }
        })
        .collect()
}

fn parse_railway_model_id(id: &str) -> (Option<String>, Option<String>) {
    let parts: Vec<&str> = id.split(':').collect();
    if parts.len() >= 4 {
        let manufacturer = parts.get(2).map(|s| s.to_string());
        let product_code = parts.get(3).map(|s| s.to_string());
        (manufacturer, product_code)
    } else {
        (None, None)
    }
}
