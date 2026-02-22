# Research: Collection Item Detail View

**Branch**: `026-collection-item-detail` | **Date**: 2026-02-22

## Summary of Findings

All technical unknowns are resolved. No new Tauri backend commands are required. The feature is
implementable entirely with existing IPC contracts plus three targeted frontend changes:
a global singleton store, a new route, and a navigation extension.

---

## Decision 1: Global Collection State — Module Singleton vs. Svelte Context

**Decision**: Create `src/lib/state/collection.svelte.ts` as a **module-level Svelte 5 class singleton**.

**Rationale**:

- The current `CollectionState` class in `src/lib/features/collection/CollectionState.svelte.ts`
  is instantiated in `+layout.svelte` and passed via Svelte context. This means child components
  that need the collection must be within the context provider tree — which works for
  `CollectionDashboard` but fails for the new `/collection/[itemId]` route unless it also has
  access to the same layout context.
- In Svelte 5, a `.svelte.ts` file can contain a class with `$state` fields. Instantiating that
  class once at module scope and exporting it creates a true singleton: a single reactive object
  shared by any module that imports it, across all route navigations.
- The singleton avoids redundant `getCollection()` calls when navigating from the list to an item.
- The existing context-based `CollectionState` in `+layout.svelte` can delegate to the module
  singleton's `fetch()` method, preserving the existing startup preload behaviour.

**Alternatives considered**:

- **Keep context only, pass item as URL param**: Rejected — the detail page cannot access context
  if it is navigated to directly (direct URL refresh bypasses the component tree provision path
  without additional wiring).
- **Add a `getCollectionItem(id)` Tauri command**: Rejected — adds backend work and a new round
  trip on every navigation. The full collection is already loaded at startup.

**Fetch caching strategy**:

- `fetch()` checks `items.length > 0` before calling the IPC command.
- An explicit `refresh()` method bypasses the cache (for use after add/delete operations).
- `isLoading` prevents duplicate concurrent fetches.

---

## Decision 2: Seller Name Display

**Decision**: Call `commands.getSellerById(sellerId)` on the detail page to resolve the seller name.

**Rationale**:

- `CollectionItemView.purchaseInfo` contains `PurchasedInfo.seller: SellerId | null` — only the
  ID, not the full seller record. The existing `getSellerById(id)` IPC command (already generated
  in `bindings.ts`) returns `SellerView | null` which includes `name` and `websiteUrl`.
- This is a single lightweight read (<200ms SLO) that only happens on the detail page, not on
  the list page. It is acceptable as a second round trip.
- `SellerView.websiteUrl: string | null` is already present — FR-007 (seller hyperlink) is
  supported without any backend changes.

**Alternatives considered**:

- **Embed `SellerView` in `PurchasedInfo`**: Rejected — requires a Rust backend change to the
  query layer and re-generation of specta bindings. Out of scope for this feature.
- **Cache sellers in global state**: Deferred — premature optimisation. A single `getSellerById`
  call on detail page load is imperceptible to users.

---

## Decision 3: Decoder Name for Operational Snapshot

**Decision**: Parse the `installed_decoder_id` URN to display manufacturer and product code.
Optionally supplement with a `getDecoders()` lookup if an exact match is needed.

**Rationale**:

- `OwnedRollingStockView.digital: DigitalSetup | null` contains:
  - `dcc_address: number` — DCC address for prominent display.
  - `installed_decoder_id: DecoderId` — a URN of form `trn:decoder:{manufacturer}:{productCode}`.
- Parsing the URN gives us `{manufacturer}` and `{productCode}` without a network call.
  Example: `trn:decoder:esu:54621` → "esu / 54621".
- `getDecoders()` returns all decoders (`Decoder[]`), which can be filtered by id for a human-
  readable label. This is an optional enhancement; the URN parse covers the core requirement.

**Alternatives considered**:

- **`getDecoderById(id)` command**: Does not exist; would require backend change. Out of scope.
- **Display raw URN**: Rejected — poor UX; URNs are not human-readable at a glance.

---

## Decision 4: Navigation Active State for `/collection/*`

**Decision**: Extend `NavigationItem` type with `additionalPrefixes?: string[]` and update
`isActive()` in `utils.ts` to match against them. Add `additionalPrefixes: ['/collection']` to
the `'collection'` nav entry in `config.ts`.

**Rationale**:

- The collection nav item uses `href: '/my-collection'` (exact match, no `usePrefixMatch`).
  The new detail route `/collection/{itemId}` shares no prefix with `/my-collection`.
- `usePrefixMatch` + changing `href` to `/collection` would break the main collection page.
- Adding `additionalPrefixes` is surgical: it leaves the existing `href` intact and adds
  supplemental matching paths. The `isActive()` function checks both the primary `href` match
  and any `additionalPrefixes`.
- One line of config change; one line of logic change; zero impact on other nav items.

**Alternatives considered**:

- **Change main collection route from `/my-collection` to `/collection`**: Rejected — the user
  did not request this refactor and it touches existing routes and navigation links throughout
  the app.
- **Add a `/collection` layout with its own nav highlighting**: Rejected — unnecessary complexity;
  the nav item config is the right place for this concern.

---

## Decision 5: Old Route Disposition (`/models/[...modelId]`)

**Decision**: **Remove** `src/routes/models/[...modelId]/+page.svelte` entirely.

**Rationale**:

- Code inspection confirms the only caller is `CollectionDashboard.handleCardClick()` (line 145),
  which does `goto('/models/${item.railwayModel.railwayModelId}')`.
- That call site will be updated to `goto('/collection/${item.id}')` as part of this feature.
- No other routes, links, or external surfaces reference `/models/...`.
- A redirect would leave dead code; removal is cleaner. The spec states the route is
  internal-only, so removal is safe (Assumption 3 in spec.md).

---

## Decision 6: Layout of the Detail Page

**Decision**: Two-panel layout — railway model card (left, `flex-1`) + collection sidebar (right,
fixed width `w-80`). On narrow screens the sidebar stacks below the card.

**Rationale**:

- SC-002 requires all four sidebar sections visible without scrolling on 1080p. A fixed `w-80`
  (320px) right panel is consistent with the existing `FilterSidebar` in `CollectionDashboard`.
- Tailwind responsive classes (`flex-col lg:flex-row`) handle narrow-screen stacking as a best-
  effort, matching the spec assumption.

---

## Data Already Available in `CollectionItemView`

| Sidebar Section       | Field                                           | Source                      |
| --------------------- | ----------------------------------------------- | --------------------------- |
| Acquisition — Date    | `purchaseInfo.data.purchaseDate`                | `CollectionItemView`        |
| Acquisition — Price   | `purchaseInfo.data.price: MonetaryAmount`       | `CollectionItemView`        |
| Acquisition — Seller  | `purchaseInfo.data.seller: SellerId`            | fetched via `getSellerById` |
| Condition — Model     | `modelCondition: ModelCondition`                | `CollectionItemView`        |
| Condition — Box       | `boxCondition: BoxCondition`                    | `CollectionItemView`        |
| Condition — Purchase  | `purchaseCondition: PurchaseCondition`          | `CollectionItemView`        |
| Operational — DCC     | `rollingStocks[n].digital.dcc_address`          | `CollectionItemView`        |
| Operational — Decoder | `rollingStocks[n].digital.installed_decoder_id` | parsed URN                  |
| Personal — Added      | `addedDate: string`                             | `CollectionItemView`        |
| Personal — Notes      | `notes: string`                                 | `CollectionItemView`        |

**Model Card data** (not in `CollectionItemView`):

- `RailwayModelView` → fetched via `getRailwayModelById(item.railwayModel.railwayModelId)`
- `RailwayModelImageResponse` → fetched via `getRailwayModelImage(railwayModelId)`
