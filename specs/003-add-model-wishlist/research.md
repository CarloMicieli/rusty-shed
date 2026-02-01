# Research: Add Railway Model to Wishlist

**Feature**: 003-add-model-wishlist  
**Date**: 2026-01-30  
**Purpose**: Resolve unknowns and document best practices for frontend implementation

## Research Tasks

### 1. Existing Drawer Pattern Analysis

**Question**: How does the existing `ItemDrawer.svelte` in Collection work?

**Finding**: The `ItemDrawer.svelte` component follows this pattern:

- Opens from the right side with a dark overlay (`fixed inset-0 z-50`)
- Max width of `max-w-xl` with `overflow-y-auto`
- Closes on: backdrop click, Escape key, close button
- Uses `$props()` for configuration and callbacks
- Form state managed with `$state()` and reset via `$effect()`
- Submits via callback prop `onSubmit`

**Decision**: Mirror this exact pattern for `AddRailwayModelDrawer.svelte`

**Rationale**: Consistency with existing UI patterns; users already familiar with drawer behavior

**Alternatives Considered**:

- Modal dialog (rejected: less space for complex forms)
- Full page form (rejected: breaks workflow continuity)

---

### 2. Dropdown Data Loading Strategy

**Question**: How should dropdown data (manufacturers, railway companies, categories, scales, power methods) be loaded?

**Finding**: Existing commands available in `bindings.ts`:

- `commands.getManufacturers()` → `Manufacturer[]`
- `commands.getRailwayCompanies()` → `RailwayCompany[]`
- Categories, Scales, PowerMethods are TypeScript enums (static data)

**Decision**:

- Load manufacturers and railway companies when drawer opens (on-demand)
- Use static arrays for Category, Scale, PowerMethod derived from TypeScript types
- Cache loaded data in the drawer's local state for the session

**Rationale**: Minimizes initial page load; data only needed when drawer is active

**Alternatives Considered**:

- Preload all data on page mount (rejected: unnecessary network calls if drawer never opened)
- Load in WishlistState (rejected: data not needed elsewhere in wishlists feature)

---

### 3. Form State Management

**Question**: How should complex nested form state (railway model + rolling stocks array) be managed?

**Finding**: Svelte 5 Runes provide:

- `$state()` for reactive form fields
- `$state([])` for dynamic arrays (rolling stocks)
- `$derived()` for computed validation states

**Decision**:

- Single form state object with `$state<FormState>()`
- Rolling stocks as `$state<RollingStockEntry[]>([])`
- Validation computed via `$derived.by()`
- Reset form when drawer closes via `$effect()`

**Rationale**: Follows Svelte 5 best practices; matches existing patterns in codebase

**Alternatives Considered**:

- External form library (rejected: adds dependency; Svelte 5 runes sufficient)
- Separate state per field (rejected: harder to reset/validate as unit)

---

### 4. Wishlist Pre-selection Behavior

**Question**: How to handle pre-selecting the active wishlist when drawer opens from wishlist context?

**Finding**: `WishlistState.svelte.ts` exposes `activeWishlistId` which tracks currently selected wishlist.

**Decision**:

- Pass `preselectedWishlistId?: string` prop to drawer
- Initialize form's `wishlistId` from this prop in `$effect()` when drawer opens
- Allow user to change selection via dropdown

**Rationale**: Matches User Story 2 requirement; reduces clicks for contextual usage

**Alternatives Considered**:

- Lock wishlist selection when pre-selected (rejected: spec requires user can change)
- Always require manual selection (rejected: reduces efficiency for common case)

---

### 5. Backend Command Integration

**Question**: Which Tauri command should the frontend call?

**Finding**: `commands.addRailwayModelToWishList(args: AddRailwayModelToWishListArgs)` exists and accepts:

```typescript
type AddRailwayModelToWishListArgs = {
  railwayModel: SimplifiedRailwayModelArgs;
  wishlistId: string;
  priority: WishlistPriority | null;
  status: WishlistStatus | null;
  desiredPriceAmount: bigint | null;
  desiredPriceCurrency: string | null;
  notes: string | null;
  addedDate: string | null;
};

type SimplifiedRailwayModelArgs = {
  manufacturerId: string;
  productCode: string;
  description: string;
  category: string;
  scale: string;
  epoch: string;
  powerMethod: string;
  rollingStocks: SimplifiedRollingStockArgs[];
};

type SimplifiedRollingStockArgs = {
  railwayCompanyId: string;
  seriesCode: string;
  roadNumber: string | null;
  locomotiveType: string | null;
  category: string;
};
```

**Decision**:

- Map form state directly to `AddRailwayModelToWishListArgs`
- Use `new Date().toISOString().split('T')[0]` for `addedDate`
- Set `status` to `'WANTED'` (default per spec)
- Convert price to `bigint` cents before sending

**Rationale**: Direct mapping; command already handles railway model creation + wishlist addition atomically

**Alternatives Considered**:

- Two separate commands (create model, then add to wishlist) (rejected: command already combines both)

---

### 6. i18n String Requirements

**Question**: What new Paraglide message keys are needed?

**Finding**: Must add to `messages/en.json` and `messages/it.json`:

**Decision**: Add these keys:

```
wishlist_add_model_button
wishlist_drawer_title
wishlist_drawer_subtitle
wishlist_field_wishlist
wishlist_field_manufacturer
wishlist_field_product_code
wishlist_field_description
wishlist_field_category
wishlist_field_scale
wishlist_field_power_method
wishlist_field_epoch
wishlist_field_desired_price
wishlist_field_priority
wishlist_rolling_stocks_title
wishlist_rolling_stock_add
wishlist_rolling_stock_remove
wishlist_field_railway_company
wishlist_field_series_code
wishlist_field_road_number
wishlist_drawer_submit
wishlist_drawer_cancel
wishlist_priority_low
wishlist_priority_normal
wishlist_priority_high
wishlist_validation_required
wishlist_toast_adding
wishlist_toast_success
wishlist_toast_error
```

**Rationale**: Follows existing naming pattern (feature_component_element)

---

### 7. Styling Consistency with My Collection

**Question**: What styling changes are needed for My Wishlists page?

**Finding**: Comparing `WishlistsDashboard.svelte` with `CollectionDashboard.svelte`:

- Collection uses `space-y-6` layout with header section
- Collection has summary component and filter sidebar
- Wishlists uses `grid gap-6 lg:grid-cols-[320px,1fr]` layout

**Decision**:

- Add header section with title and "Add railway model" button matching Collection pattern
- Keep sidebar/main grid layout (appropriate for wishlists)
- Ensure button styling matches Collection's `variant-filled-primary btn gap-2`

**Rationale**: Partial alignment; wishlists sidebar is core to feature, full Collection parity not needed

---

## Summary

All research questions resolved. No NEEDS CLARIFICATION items remain.

| Topic          | Decision                                    | Reference                                     |
| -------------- | ------------------------------------------- | --------------------------------------------- |
| Drawer pattern | Mirror ItemDrawer.svelte                    | Collection feature                            |
| Data loading   | On-demand when drawer opens                 | commands.getManufacturers/getRailwayCompanies |
| Form state     | Single $state object + rolling stocks array | Svelte 5 runes                                |
| Pre-selection  | Prop-based with $effect initialization      | activeWishlistId                              |
| Backend        | Use addRailwayModelToWishList command       | bindings.ts                                   |
| i18n           | 28 new message keys                         | messages/\*.json                              |
| Styling        | Partial alignment with Collection header    | WishlistsDashboard.svelte                     |
