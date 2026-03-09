# IPC Contract: update_wishlist_item

**Type**: Tauri Command (write / mutation)  
**Feature**: 037-wishlist-inline-edit

---

## Overview

Allows the frontend to update one or more editable fields on a specific wishlist item without requiring a full replacement. Only provided (non-`null`) fields are changed; omitted fields are left untouched.

---

## Rust Signature

```rust
#[tauri::command]
#[specta::specta]
pub async fn update_wishlist_item(
    state: tauri::State<'_, AppState>,
    args: UpdateWishlistItemArgs,
) -> Result<WishlistItem, CommandError>
```

---

## Request: `UpdateWishlistItemArgs`

Derived `specta::Type` + `serde::Deserialize`; `serde(rename_all = "camelCase")`.

| Field                  | TypeScript type               | Required | Description                                                   |
| ---------------------- | ----------------------------- | -------- | ------------------------------------------------------------- |
| `wishlistId`           | `string`                      | ✅       | UUID of the parent wishlist                                   |
| `itemId`               | `string`                      | ✅       | UUID of the wishlist item to update                           |
| `priority`             | `WishlistPriority \| null`    | ❌       | New priority; omit or `null` to leave unchanged               |
| `status`               | `WishlistStatus \| null`      | ❌       | New status; omit or `null` to leave unchanged                 |
| `desiredPriceAmount`   | `number \| null \| undefined` | ❌       | `null` clears the price; a number sets it (in smallest unit)  |
| `desiredPriceCurrency` | `string \| null`              | ❌       | ISO 4217 code; required when `desiredPriceAmount` is a number |
| `addedDate`            | `string \| null`              | ❌       | ISO date `YYYY-MM-DD`; must be ≤ today                        |

---

## Response

Returns the full updated `WishlistItem` on success so the frontend can refresh its local state without an extra read call.

```typescript
// On success (200-equivalent)
WishlistItem;

// On failure
CommandError; // { message: string }
```

---

## Validation Rules (backend, enforced at transport boundary)

1. `wishlistId` MUST be a valid UUID (parse via `WishlistId::try_from`).
2. `itemId` MUST be a valid UUID.
3. `addedDate`, when provided, MUST be a valid calendar date and MUST NOT be in the future (`date <= today`).
4. `desiredPriceAmount`, when a number, MUST be `>= 0`.
5. When `desiredPriceAmount` is a non-null number, `desiredPriceCurrency` MUST also be provided and MUST be a non-empty string.
6. At least one mutable field MUST be provided (all-null patch is rejected with `DomainError::Validation`).

---

## Error Cases

| Condition                     | Error type                 | Suggested frontend handling                                       |
| ----------------------------- | -------------------------- | ----------------------------------------------------------------- |
| Wishlist not found (stale ID) | `CommandError { message }` | Revert field; show toast                                          |
| Item not found                | `CommandError { message }` | Revert field; show toast                                          |
| `addedDate` is in the future  | `CommandError { message }` | Inline validation (should be caught first by calendar constraint) |
| Invalid price amount          | `CommandError { message }` | Inline validation (should be caught first by input validation)    |
| Database failure              | `CommandError { message }` | Revert field; show toast                                          |

---

## TypeScript Usage Example

```typescript
import { commands } from '$lib/bindings';

const updated = await commands.updateWishlistItem({
  wishlistId: item.wishlistId,
  itemId: item.id,
  priority: 'HIGH',
  status: null, // unchanged
  desiredPriceAmount: null, // unchanged
  desiredPriceCurrency: null,
  addedDate: null // unchanged
});
```
