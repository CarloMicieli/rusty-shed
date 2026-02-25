# IPC Contract: Purchase Wishlist Item

**Command**: `purchase_wishlist_item`
**Direction**: Frontend → Backend (Tauri `invoke`)
**File**: `src-tauri/src/wishlist/interface/command_handlers.rs`

---

## Request Payload (`PurchaseWishlistArgs`)

| Field            | Type             | Required | Constraints                                                                                                      |
| ---------------- | ---------------- | -------- | ---------------------------------------------------------------------------------------------------------------- |
| `wishlistId`     | `string`         | Yes      | TRN format: `trn:wishlist:{uuid}`                                                                                |
| `wishlistItemId` | `string`         | Yes      | TRN format: `trn:wishlist-item:{uuid}`                                                                           |
| `priceAmount`    | `number` (i64)   | Yes      | Integer ≥ 0, minor currency units (e.g. cents)                                                                   |
| `priceCurrency`  | `string`         | Yes      | One of `"EUR"`, `"USD"`, `"GBP"`, `"JPY"`                                                                        |
| `purchaseDate`   | `string`         | Yes      | ISO 8601 date `YYYY-MM-DD`, not in the future                                                                    |
| `sellerId`       | `string \| null` | No       | TRN format: `trn:seller:{slug}`, or `null`                                                                       |
| `condition`      | `string \| null` | No       | One of: `"New"`, `"PreOwnedLikeNew"`, `"PreOwnedVeryGood"`, `"PreOwnedGood"`, `"PreOwnedAcceptable"` — or `null` |

**Serialization**: camelCase (via `#[serde(rename_all = "camelCase")]`)

---

## Response

| Case    | Type           | Description                                              |
| ------- | -------------- | -------------------------------------------------------- |
| Success | `void`         | Purchase recorded. Both aggregates persisted atomically. |
| Error   | `CommandError` | Serialized error with `message` field.                   |

---

## Error Cases

| Condition                                         | Error Message                                             |
| ------------------------------------------------- | --------------------------------------------------------- |
| `wishlistItemId` not found in wishlist            | "Wishlist item not found"                                 |
| Item already has status `Purchased` or `Ignored`  | "Item is not available for purchase"                      |
| `priceAmount` is absent or negative               | Validation error: "Price is required"                     |
| `purchaseDate` is in the future                   | Validation error: "Purchase date cannot be in the future" |
| `priceCurrency` is not a recognized currency code | Validation error: "Unknown currency"                      |
| `condition` is an unrecognized string             | Validation error: "Unknown condition value"               |
| `sellerId` references a non-existent seller       | Domain error: "Seller not found"                          |
| Database/persistence failure                      | Error: "Failed to record purchase"                        |

---

## Side Effects

On success, atomically:

1. `WishlistEvent::ItemPurchased` emitted → `UPDATE wishlist_items SET status='PURCHASED', purchased_price_amount=?, purchased_price_currency=? WHERE id=?`
2. `CollectionEvent::RailwayModelAdded` emitted → `INSERT` into `collection_items`, `purchase_info`, and associated tables

Both mutations occur within a single SQLite transaction (UoW). On any error, the entire transaction rolls back.

---

## Supporting Queries (already registered, no changes)

### `get_sellers`

**Returns**: `SellerView[]`

Used by the frontend to populate the seller autocomplete in the purchase dialog.

```typescript
// SellerView shape (specta-generated)
interface SellerView {
  id: string; // "trn:seller:{slug}"
  name: string;
  sellerType: string;
}
```

### `get_settings`

**Returns**: `UserSettings`

Used by the frontend to read the default currency for the price field.

```typescript
// Relevant field
interface UserSettings {
  currency: string; // e.g., "EUR"
  // ...other fields
}
```
