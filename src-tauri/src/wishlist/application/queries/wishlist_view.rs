use crate::core::domain::Currency;
use crate::wishlist::application::WishlistItemView;
use crate::wishlist::domain::wishlist::Wishlist;
use crate::wishlist::domain::wishlist_id::WishlistId;
use crate::wishlist::domain::wishlist_preview::WishlistPreview;
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::collections::HashMap;

/// View model representing a wishlist used by queries.
///
/// `WishlistView` is returned by the query handlers. It contains the preview
/// fields (matching `WishlistPreview`) and an optional `items` collection. For
/// list queries each view will have `items == None` to keep payloads small;
/// for the single-wishlist query the `items` field will be `Some(vec![...])`.
#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct WishlistView {
    /// Unique identifier for the wishlist (TRN format).
    pub id: WishlistId,
    /// Human-readable name for the wishlist.
    pub name: String,
    /// Optional free-form notes attached to the wishlist.
    pub notes: Option<String>,
    /// Whether this wishlist is the default for the user.
    pub is_default: bool,
    /// Number of items contained in the wishlist (derived from `items` when present).
    pub count: i64,
    /// Timestamp of the last update for the wishlist.
    pub updated_at: DateTime<Utc>,
    /// Summed monetary totals per currency for the wishlist items.
    pub total_value: HashMap<Currency, i64>,
    /// Optional detailed items for the wishlist. This is `None` for list views and
    /// `Some(vec!)` for single-wishlist views returned by the `get_wishlist_by_id` query.
    pub items: Option<Vec<WishlistItemView>>,
}

/// Convert a `WishlistPreview` (domain preview) into a `WishlistView`.
///
/// Converts `updated_at` from a `NaiveDateTime` (preview) into `DateTime<Utc>`
/// by assuming UTC. The resulting view will have `items == None`.
impl From<WishlistPreview> for WishlistView {
    fn from(p: WishlistPreview) -> Self {
        let updated_at = DateTime::from_naive_utc_and_offset(p.updated_at, Utc);
        WishlistView {
            id: p.id,
            name: p.name,
            notes: p.notes,
            is_default: p.is_default,
            count: p.count,
            updated_at,
            total_value: p.total_value,
            items: None,
        }
    }
}

/// Convert a full domain `Wishlist` aggregate into a `WishlistView`.
///
/// Maps the aggregate's items into `WishlistItemView` and sets `items = Some(...)`.
impl From<Wishlist> for WishlistView {
    fn from(w: Wishlist) -> Self {
        let items: Option<Vec<WishlistItemView>> = Some(
            w.items
                .into_iter()
                .map(|i| WishlistItemView {
                    id: i.id,
                    railway_model_id: i.railway_model_id,
                    priority: i.priority,
                    status: i.status,
                    added_date: i.added_date,
                    removed_date: i.removed_date,
                    notes: i.notes,
                    desired_price: i.desired_price,
                    purchased_price: i.purchased_price,
                })
                .collect(),
        );

        let count = match &items {
            Some(v) => v.len() as i64,
            None => 0,
        };

        WishlistView {
            id: w.id,
            name: w.name,
            notes: w.notes,
            is_default: w.is_default,
            count,
            updated_at: w.metadata.updated_at,
            total_value: HashMap::new(),
            items,
        }
    }
}

// Tests for conversions
#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::domain::railway_model::RailwayModelId;
    use crate::core::domain::metadata::Metadata;
    use crate::wishlist::domain::wishlist::Wishlist;
    use crate::wishlist::domain::wishlist_id::WishlistId;
    use crate::wishlist::domain::wishlist_item::WishlistItem;
    use crate::wishlist::domain::wishlist_item_id::WishlistItemId;
    use crate::wishlist::domain::wishlist_preview::WishlistPreview;
    use crate::wishlist::domain::wishlist_priority::WishlistPriority;
    use crate::wishlist::domain::wishlist_status::WishlistStatus;
    use chrono::NaiveDate;

    #[test]
    fn it_should_preview_converts_to_view_with_no_items() {
        let preview = WishlistPreview {
            id: WishlistId::default(),
            name: "Preview".to_string(),
            notes: None,
            is_default: false,
            count: 0,
            updated_at: NaiveDate::from_ymd_opt(2020, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            total_value: HashMap::new(),
        };

        let view: WishlistView = preview.into();
        assert!(view.items.is_none());
        assert_eq!(view.count, 0);
    }

    #[test]
    fn it_should_wishlist_converts_to_view_with_items() {
        let railway_id = RailwayModelId::try_from("trn:railway-model:acme:1").unwrap();
        let item = WishlistItem {
            id: WishlistItemId::default(),
            railway_model_id: railway_id,
            priority: WishlistPriority::Normal,
            status: WishlistStatus::Wanted,
            added_date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            removed_date: None,
            notes: None,
            desired_price: None,
            purchased_price: None,
        };

        let wishlist = Wishlist {
            id: WishlistId::default(),
            name: "List".to_string(),
            notes: None,
            is_default: false,
            items: vec![item],
            pending_events: Vec::new(),
            metadata: Metadata::default(),
        };

        let view: WishlistView = wishlist.into();
        assert!(view.items.is_some());
        assert_eq!(view.count, 1);
    }
}
