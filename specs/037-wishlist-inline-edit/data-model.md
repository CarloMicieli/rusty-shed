# Data Model: Wishlist Item Sidebar Inline Editing

**Feature**: 037-wishlist-inline-edit  
**Date**: 2026-03-09

---

## Entities Involved

### WishlistItem (existing, modified)

| Field              | Type                           | Nullable | Editable | Notes                                          |
| ------------------ | ------------------------------ | -------- | -------- | ---------------------------------------------- |
| `id`               | `WishlistItemId` (UUID string) | No       | No       | Stable identifier                              |
| `railway_model_id` | `RailwayModelId` (string)      | No       | No       | References catalogue                           |
| `priority`         | `WishlistPriority`             | No       | **Yes**  | Dropdown: LOW, NORMAL, HIGH                    |
| `status`           | `WishlistStatus`               | No       | **Yes**  | Dropdown: WANTED, ON_ORDER, PURCHASED, IGNORED |
| `added_date`       | `NaiveDate` (YYYY-MM-DD)       | No       | **Yes**  | Calendar: past dates only (≤ today)            |
| `removed_date`     | `NaiveDate`                    | Yes      | No       | Set on removal; not exposed in sidebar         |
| `notes`            | `String`                       | Yes      | No       | Preserved through update; not edited here      |
| `desired_price`    | `MonetaryAmount`               | Yes      | **Yes**  | Numeric input; uses settings default currency  |
| `purchased_price`  | `MonetaryAmount`               | Yes      | No       | Preserved through update; not edited here      |

### WishlistPriority (existing enum, unchanged)

| Variant  | Serialised wire value |
| -------- | --------------------- |
| `Low`    | `"LOW"`               |
| `Normal` | `"NORMAL"`            |
| `High`   | `"HIGH"`              |

### WishlistStatus (existing enum, unchanged)

| Variant     | Serialised wire value |
| ----------- | --------------------- |
| `Wanted`    | `"WANTED"`            |
| `OnOrder`   | `"ON_ORDER"`          |
| `Purchased` | `"PURCHASED"`         |
| `Ignored`   | `"IGNORED"`           |

### MonetaryAmount (existing value object, unchanged)

| Field      | Type                      | Notes                                    |
| ---------- | ------------------------- | ---------------------------------------- |
| `amount`   | `i64` (bigint on TS side) | Stored in smallest currency unit (cents) |
| `currency` | `String`                  | ISO 4217 code, e.g. `"EUR"`              |

---

## New Domain Event: `WishlistEvent::ItemUpdated`

**Location**: `src-tauri/src/wishlist/domain/wishlist_event.rs`

```rust
ItemUpdated {
    item_id: WishlistItemId,
    priority: Option<WishlistPriority>,
    status: Option<WishlistStatus>,
    desired_price: Option<Option<MonetaryAmount>>,  // outer None = unchanged, inner None = clear
    added_date: Option<NaiveDate>,
}
```

> `desired_price` uses a double-Option pattern: `None` means "do not touch this field"; `Some(None)` means "clear the price"; `Some(Some(amount))` means "set to amount".

---

## New Use Case Input: `UpdateWishlistItemInput`

**Location**: `src-tauri/src/wishlist/application/inputs.rs`

```rust
pub struct UpdateWishlistItemInput {
    pub wishlist_id: WishlistId,
    pub item_id: WishlistItemId,
    pub priority: Option<WishlistPriority>,
    pub status: Option<WishlistStatus>,
    pub desired_price: Option<Option<MonetaryAmount>>,
    pub added_date: Option<NaiveDate>,
}
```

---

## New Transport DTO: `UpdateWishlistItemArgs`

**Location**: `src-tauri/src/wishlist/interface/command_args.rs`

```rust
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWishlistItemArgs {
    pub wishlist_id: String,
    pub item_id: String,
    pub priority: Option<WishlistPriority>,
    pub status: Option<WishlistStatus>,
    /// None = unchanged; Some(None) = clear price; Some(Some(n)) = set amount
    #[serde(default, deserialize_with = "deserialize_double_option")]
    pub desired_price_amount: Option<Option<i64>>,
    pub desired_price_currency: Option<String>,
    pub added_date: Option<NaiveDate>,
}
```

> **⚠️ Serde implementation note (D1)**: The default `serde::Deserialize` derive cannot distinguish an absent JSON key (`None` = "do not touch") from an explicit `null` (`Some(None)` = "clear the price") for `Option<Option<T>>`. The `#[serde(default, deserialize_with = "deserialize_double_option")]` attribute above requires a project-local helper function (or the `serde_with` crate's `double_option` helper) to be wired in. A unit test is **required** in T006: assert that JSON `{"desiredPriceAmount":null}` deserializes to `Some(None)` and a missing `desiredPriceAmount` key deserializes to `None`.

---

## State Transitions

### Desired Price field

```
[Not set]  ──(user sets amount)──►  [MonetaryAmount { amount, currency }]
[MonetaryAmount] ──(user clears)──►  [Not set / null]
```

### Added Date constraint

- Minimum: no lower bound (any past date is valid)
- Maximum: today (`chrono::Local::now().date_naive()`)

---

## Database

No schema migration required. The editable fields (`priority`, `status`, `desired_price_amount`, `desired_price_currency`, `added_date`) are already columns in the existing `wishlist_items` table. The repository processes `WishlistEvent::ItemUpdated` by running a targeted `UPDATE wishlist_items SET ... WHERE id = ?` covering only changed fields.
