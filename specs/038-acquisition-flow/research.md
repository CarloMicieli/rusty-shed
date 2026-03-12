# Research: Acquisition Flow (038)

**Date**: 2026-03-12
**Status**: Complete — all unknowns resolved

---

## Decision 1: Upsert Strategy for Catalog Entries

**Question**: How do we create a catalog entry only when it doesn't already exist, given no `find_by_manufacturer_and_product_code` repository method?

**Decision**: Derive the `RailwayModelId` deterministically, then probe with `find_by_id`.

**Rationale**: `RailwayModelId::new(manufacturer_id, product_code)` produces a deterministic TRN —
`trn:railway-model:{manufacturer_nss}:{product_code.to_lowercase()}`. The existing `find_by_id` call
on the repository is sufficient to detect existence. No new DB index or query is needed. Logic:

```
id = RailwayModelId::new(manufacturer_id, product_code)?
if repository.find_by_id(id, "en").is_none() {
    repository.create(params)
}
// then record purchase
```

**Alternatives considered**:

- Add `find_id_by_manufacturer_and_product_code` repo method → unnecessary; deterministic ID achieves the same with less code.
- Attempt insert and catch PK violation → error-path control flow is an anti-pattern in this codebase.

---

## Decision 2: New Backend Command — `record_acquisition`

**Question**: Can we compose the acquisition flow from existing commands (`add_railway_model_to_collection` + `add_collection_item`), or does it need a new command?

**Decision**: New `record_acquisition` Tauri command with a dedicated use case.

**Rationale**:

- The existing `add_railway_model_to_collection` always creates a new catalog entry (no existence check).
- Multi-item batches must be accepted atomically — looping frontend calls would leave partially-created state on failure.
- Domain Logic Location law (constitution) requires all business rules (upsert, batch) to live in Rust.
- A single new command keeps the transport boundary clean.

**Alternatives considered**:

- Sequence `n` calls from the frontend → violates Domain Logic Location law; partial failure is unrecoverable from frontend.
- Reuse `add_railway_model_to_collection` per item → no existence check; duplicate catalog entries on re-purchase.

---

## Decision 3: Drawer UI Pattern

**Question**: Should the acquisition drawer use shadcn's `Drawer`/`Sheet` component or the hand-rolled pattern used in existing drawers?

**Decision**: Follow the existing hand-rolled pattern (`fixed top-0 right-0` + `translate-x-{0,full}` transition).

**Rationale**:

- Both `AddModelDrawer.svelte` and `AddRailwayModelDrawer.svelte` use the same pattern.
- No shadcn Drawer/Sheet dependency is present in the project — adding one would require dependency approval.
- The existing pattern is proven, accessible (`role="dialog" aria-modal="true"`), and handles scroll lock correctly.

**Structure**:

```
fixed overlay (backdrop-blur-sm, onclick → close request)
fixed right panel (max-w-2xl, translate-x animation)
  ├── sticky header (border-b p-4): purchase metadata + batch defaults
  ├── scrollable center (flex-1 overflow-y-auto p-4): item card list
  └── sticky footer (border-t p-4): "Add Another Item" + "Finalize Purchase"
```

**Alternatives considered**:

- shadcn Sheet → requires new dependency, not worth it for pattern parity.
- Full-page route (`/acquisition/new`) → destroys Dashboard context; spec requires it remain visible.

---

## Decision 4: Item Card List Management

**Question**: How should multi-item state be managed — array in parent vs. child component?

**Decision**: `$state<AcquisitionItemEntry[]>` array in the parent drawer, with a dedicated `AcquisitionItemCard.svelte` receiving a single item and index.

**Rationale**: Follows the same pattern as `AddModelDrawer`'s `rollingStocks` array. Keeps mutation (add, duplicate, remove) in one place. The card component only renders and emits `onUpdate`/`onRemove` events.

**Duplicate logic**: Clone all fields of an entry, assign a new `uid` via `crypto.randomUUID()`, clear `productCode`. Insert immediately after the cloned entry.

---

## Decision 5: Global Keyboard Shortcut (Ctrl+N)

**Question**: `tauri-plugin-global-shortcut` is not currently installed. How should Ctrl+N be implemented?

**Decision**: Add `tauri-plugin-global-shortcut` to `src-tauri/Cargo.toml` and the capabilities manifest. ✅ **Approved by user 2026-03-12. Dependency already added to Cargo.toml.**

**Rationale**: Tauri 2.0 provides a first-class global shortcut plugin. The alternative (window-level `keydown` listener) only works when the app window is focused — not "from any screen" as the spec requires. A Tauri-level global shortcut fires even when the window is in the background.

**Implementation pattern** (once approved):

```rust
// lib.rs setup()
app.global_shortcut().register("CommandOrControl+N", |app, _shortcut, _event| {
    app.emit("open-acquisition-drawer", ()).ok();
})?;
```

Frontend listens with `listen("open-acquisition-drawer", ...)` and sets `showAcquisitionDrawer = true`.

**Alternatives considered**:

- Document-level `keydown` → only fires when window focused; does not meet spec SC-005.
- Tauri menu shortcut → only works when menu bar is present; desktop-app UX is different.

---

## Decision 6: Price Handling

**Question**: How is price stored, and how does the currency default work?

**Decision**: Price input as decimal string in form (`"29.99"`), converted to integer cents on submit. Currency is a read-only display derived from `settingsState.settings.currency` (default `"EUR"` if not set).

**Rationale**: Matches the existing pattern in `AddModelDrawer` (`priceAmount: number | null` in cents, `priceCurrency: string`). The backend `AddCollectionItemArgs.price_amount` is `i64` cents. TypeScript needs `as unknown as bigint` cast when passing to Tauri specta-generated commands.

**Price is optional**: A blank price field → `price_amount = 0`, consistent with existing `toAddRailwayModelArgs` logic.

---

## Decision 7: No New SQLite Migrations

**Question**: Are new tables or columns needed?

**Decision**: No migrations required. All required tables exist:

- `railway_models` — catalog entry storage
- `collection_items` — one row per acquisition item
- `purchase_infos` — purchase metadata per collection item (type = `"Purchased"`)

The acquisition creates rows in all three (conditionally for `railway_models`), reusing the existing domain model.

---

## Decision 8: Feature Module Location

**Question**: Where does the new frontend code live?

**Decision**: New feature module at `src/lib/features/acquisition/`.

**Rationale**: All features follow the pattern `src/lib/features/{feature}/`. The acquisition flow is sufficiently distinct from `collection/` (different form layout, multi-item, different backend command) to warrant its own module.

**Structure**:

```
src/lib/features/acquisition/
├── AcquisitionDrawer.svelte        # Root drawer component
├── components/
│   ├── AcquisitionHeader.svelte    # Sticky header (purchase metadata + batch defaults)
│   ├── AcquisitionItemCard.svelte  # Per-item card with all model fields + price
│   └── AcquisitionFooter.svelte    # Sticky footer with actions
├── types.ts                        # AcquisitionFormState, AcquisitionItemEntry, BatchDefaults
└── AcquisitionState.svelte.ts      # Context/service: calls record_acquisition command
```
