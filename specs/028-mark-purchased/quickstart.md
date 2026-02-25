# Quickstart: Mark Wishlist Item as Purchased (028)

**Branch**: `028-mark-purchased`

---

## Development Flow

```bash
# 1. Start the full desktop app
pnpm tauri dev

# 2. After backend changes, rebuild to regenerate specta bindings
pnpm rust:build   # or pnpm tauri dev restarts automatically

# 3. Run backend tests
pnpm rust:test

# 4. Run a specific test
cargo test --manifest-path src-tauri/Cargo.toml \
  -p rusty_shed_lib wishlist::application::purchase_wishlist_item::tests

# 5. Run frontend checks
pnpm check && pnpm lint && pnpm test
```

---

## Implementation Order

Follow this order to avoid compilation errors between steps:

```
Step 1  ── Domain: Add WishlistEvent::ItemPurchased variant
Step 2  ── Domain: Add Wishlist::purchase_item() method
Step 3  ── Infrastructure: Handle ItemPurchased event in repository
Step 4  ── Application: Update PurchaseWishlistItemService to use new method + pass conditions
Step 5  ── Interface: Extend PurchaseWishlistArgs with `condition` field + decomposition logic
Step 6  ── Rust build + existing test updates
Step 7  ── Frontend: PurchaseDialog.svelte component
Step 8  ── Frontend: Purchase button in WishlistItemCard
Step 9  ── Frontend: Purchase button in wishlist item detail page
Step 10 ── Frontend: Wire dialog state in WishlistController.svelte.ts
Step 11 ── End-to-end manual test
```

---

## Key File Locations

| What                           | Path                                                              |
| ------------------------------ | ----------------------------------------------------------------- |
| Wishlist domain events         | `src-tauri/src/wishlist/domain/wishlist_event.rs`                 |
| Wishlist aggregate             | `src-tauri/src/wishlist/domain/wishlist.rs`                       |
| Wishlist infrastructure repo   | `src-tauri/src/wishlist/infrastructure/`                          |
| Purchase service (application) | `src-tauri/src/wishlist/application/purchase_wishlist_item.rs`    |
| Tauri command handler          | `src-tauri/src/wishlist/interface/command_handlers.rs`            |
| Command args struct            | `src-tauri/src/wishlist/interface/` (find `PurchaseWishlistArgs`) |
| Collection use case            | `src-tauri/src/collecting/application/add_collection_item.rs`     |
| PurchaseCondition enum         | `src-tauri/src/collecting/domain/purchase_condition.rs`           |
| ModelCondition enum            | `src-tauri/src/collecting/domain/model_condition.rs`              |
| Frontend feature root          | `src/lib/features/wishlist/`                                      |
| Generated TS bindings          | `src/lib/bindings.ts` (auto-generated — do not edit)              |
| Paraglide messages             | `messages/en.json` (add all new UI strings here)                  |

---

## Testing the Feature Manually

1. Launch app with `pnpm tauri dev`
2. Navigate to the Wishlist page
3. Ensure at least one wishlist item exists with status Wanted or On Order
4. **Card trigger**: Click the "Purchase" button on the item card → dialog opens
5. Verify purchase date is pre-filled with today's date
6. Submit without a price → verify validation error appears
7. Enter a price, pick a seller and condition, confirm → item disappears from wishlist
8. Navigate to the Collection page → verify the item appears with correct price, date, seller, condition
9. Return to wishlist → verify the Purchase button is no longer shown on the (now Purchased) item
10. **Detail trigger**: Open a different item's detail page → verify the "Purchase" button is present
11. Complete the same flow from the detail page

---

## Condition Mapping Quick Reference

| Dialog shows           | `condition` param      | `purchase_condition` | `model_condition` |
| ---------------------- | ---------------------- | -------------------- | ----------------- |
| New                    | `"New"`                | `New`                | `None`            |
| Pre-Owned – Like New   | `"PreOwnedLikeNew"`    | `PreOwned`           | `NearMint`        |
| Pre-Owned – Very Good  | `"PreOwnedVeryGood"`   | `PreOwned`           | `VeryGood`        |
| Pre-Owned – Good       | `"PreOwnedGood"`       | `PreOwned`           | `Good`            |
| Pre-Owned – Acceptable | `"PreOwnedAcceptable"` | `PreOwned`           | `Fair`            |
| (not selected)         | `null`                 | `None`               | `None`            |
