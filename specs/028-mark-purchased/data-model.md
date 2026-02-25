# Data Model: Mark Wishlist Item as Purchased (028)

**Date**: 2026-02-24

---

## 1. Domain Changes — Wishlist

### 1.1 New Event Variant

**File**: `src-tauri/src/wishlist/domain/wishlist_event.rs`

Add to `WishlistEvent` enum:

```rust
/// Emitted when a wishlist item is successfully marked as purchased
/// and the item has been moved to the collection.
ItemPurchased {
    item_id: WishlistItemId,
    purchased_price: MonetaryAmount,
},
```

### 1.2 New Aggregate Method

**File**: `src-tauri/src/wishlist/domain/wishlist.rs`

Add to `Wishlist` aggregate:

```rust
/// Validates and transitions a wishlist item to the Purchased status.
///
/// Returns `DomainError::InvalidOperation` if the item is not found or
/// is already in a terminal status (Purchased, Ignored).
pub fn purchase_item(
    &mut self,
    item_id: &WishlistItemId,
    purchased_price: MonetaryAmount,
) -> Result<(), DomainError>
```

**Validation rules** (enforced by this method):

- Item with `item_id` must exist in `self.items`
- Item status must be `Wanted` or `OnOrder` — any other status returns `DomainError::InvalidOperation`

**State changes** (applied immediately via `apply_event`):

- `item.status = WishlistStatus::Purchased`
- `item.purchased_price = Some(purchased_price)`

**Event emitted**:

- `WishlistEvent::ItemPurchased { item_id, purchased_price }`

### 1.3 Infrastructure Handler

**File**: `src-tauri/src/wishlist/infrastructure/` (repository impl)

Add handling for `WishlistEvent::ItemPurchased` in the `save_wishlist` method:

```sql
UPDATE wishlist_items
SET status = 'PURCHASED',
    purchased_price_amount = ?,
    purchased_price_currency = ?
WHERE id = ?
```

---

## 2. Application Service Changes

**File**: `src-tauri/src/wishlist/application/purchase_wishlist_item.rs`

### 2.1 Updated Command Struct

Rename `MoveWishlistItemId` → keep as-is (backward compat) OR introduce a new command:

```rust
/// Command for the PurchaseWishlistItemService coordinator.
#[derive(Debug, Clone)]
pub struct PurchaseWishlistItemCommand {
    pub wishlist_id: WishlistId,
    pub wishlist_item_id: WishlistItemId,
    pub purchase_price: MonetaryAmount,
    pub purchase_date: NaiveDate,
    pub seller_id: Option<SellerId>,
    pub purchase_condition: Option<PurchaseCondition>,  // NEW
    pub model_condition: Option<ModelCondition>,         // NEW
}
```

(`box_condition` omitted from the dialog — UI only captures the combined condition picker, `box_condition` remains `None` unless added later.)

### 2.2 Updated Service Orchestration

```
PurchaseWishlistItemService::execute(cmd) [Coordinator]:
  1. Begin UoW transaction (implicit — already open when called from command handler)
  2. Load Wishlist aggregate via WishlistRepository::find_by_id(cmd.wishlist_id)
  3. wishlist.purchase_item(cmd.wishlist_item_id, cmd.purchase_price)
     └─ Wishlist Aggregate validates: item exists + status is Wanted/OnOrder
     └─ Emits WishlistEvent::ItemPurchased
  4. AddCollectionItem::execute(uow, ..., AddCollectionItemInput {
       railway_model_id: item.railway_model_id,
       price: cmd.purchase_price,
       seller_id: cmd.seller_id,
       added_date: today,
       purchase_date: cmd.purchase_date,
       purchase_condition: cmd.purchase_condition,  ← now populated
       model_condition: cmd.model_condition,         ← now populated
       box_condition: None,
       notes: item.notes,
     })
     └─ Collection Aggregate validates: no duplicate by railway_model_id (if applicable)
     └─ Emits CollectionEvent::RailwayModelAdded
  5. WishlistRepository::save_wishlist(wishlist) → persists ItemPurchased event → SQL UPDATE
  6. CollectionRepository::save(collection) → persists RailwayModelAdded event → SQL INSERTs
  (UoW commit happens in command handler)
```

---

## 3. Interface Layer Changes

**File**: `src-tauri/src/wishlist/interface/command_args.rs` (or wherever `PurchaseWishlistArgs` lives)

### 3.1 Updated `PurchaseWishlistArgs`

```rust
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseWishlistArgs {
    pub wishlist_id: String,
    pub wishlist_item_id: String,
    #[validate(range(min = 0))]
    pub price_amount: i64,
    pub price_currency: String,
    pub purchase_date: NaiveDate,
    pub seller_id: Option<String>,
    /// Combined condition selection. Valid values:
    /// "New" | "PreOwnedLikeNew" | "PreOwnedVeryGood" | "PreOwnedGood" | "PreOwnedAcceptable"
    pub condition: Option<String>,  // NEW
}
```

### 3.2 Condition Decomposition (in `TryFrom<PurchaseWishlistArgs>`)

```rust
let (purchase_condition, model_condition) = match args.condition.as_deref() {
    None => (None, None),
    Some("New") => (Some(PurchaseCondition::New), None),
    Some("PreOwnedLikeNew") => (Some(PurchaseCondition::PreOwned), Some(ModelCondition::NearMint)),
    Some("PreOwnedVeryGood") => (Some(PurchaseCondition::PreOwned), Some(ModelCondition::VeryGood)),
    Some("PreOwnedGood") => (Some(PurchaseCondition::PreOwned), Some(ModelCondition::Good)),
    Some("PreOwnedAcceptable") => (Some(PurchaseCondition::PreOwned), Some(ModelCondition::Fair)),
    Some(other) => return Err(DomainError::validation("condition", format!("unknown value: {other}"))),
};
```

---

## 4. Frontend Types

### 4.1 Condition Options (Frontend Enum)

```typescript
export const PURCHASE_CONDITION_OPTIONS = [
  { value: 'New', label: 'New' },
  { value: 'PreOwnedLikeNew', label: 'Pre-Owned – Like New' },
  { value: 'PreOwnedVeryGood', label: 'Pre-Owned – Very Good' },
  { value: 'PreOwnedGood', label: 'Pre-Owned – Good' },
  { value: 'PreOwnedAcceptable', label: 'Pre-Owned – Acceptable' }
] as const;
```

All labels are Paraglide message keys — hardcoded strings above are for reference only.

### 4.2 Purchase Dialog Form Schema (Zod)

```typescript
const PurchaseFormSchema = z.object({
  priceAmount: z.number().int().min(0, 'Price is required'),
  purchaseDate: z
    .string()
    .regex(/^\d{4}-\d{2}-\d{2}$/)
    .refine((d) => d <= today(), 'Date cannot be in the future'),
  sellerId: z.string().nullable().optional(),
  condition: z
    .enum(['New', 'PreOwnedLikeNew', 'PreOwnedVeryGood', 'PreOwnedGood', 'PreOwnedAcceptable'])
    .nullable()
    .optional()
});
```

### 4.3 Dialog State (in WishlistController / PurchaseDialogController)

```typescript
interface PurchaseDialogState {
  open: boolean;
  wishlistId: string;
  wishlistItemId: string;
  itemName: string; // display only
  defaultCurrency: string; // loaded from settings
  sellers: SellerView[]; // loaded from get_sellers
  isSubmitting: boolean;
  error: string | null;
}
```
