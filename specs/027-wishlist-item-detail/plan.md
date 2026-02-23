# Implementation Plan: Wishlist Item Detail View

**Branch**: `027-wishlist-item-detail` | **Date**: 2026-02-23 | **Spec**: [spec.md](spec.md)

## Summary

Add a dedicated detail page at `/wishlists/{wishlistId}/items/{itemId}` that displays the railway model card alongside a wishlist-specific sidebar (wishlist name, priority, desired/purchased price, added date, notes). Fix the wishlist item cards on the main `/my-wishlists` page to navigate to this new detail page when clicked. This is a **frontend-only** feature — all required data is available from the existing `getWishlistById` Tauri command; no new backend commands or database schema changes are needed.

## Technical Context

**Language/Version**: TypeScript 5.9.3 / Svelte 5.48.2 / Rust 1.93.0 (edition 2024)
**Primary Dependencies**: SvelteKit (Vite 7.3.1), Tauri 2.9.x, Tailwind CSS 4, shadcn-svelte, Paraglide 2.7.1, specta (type generation)
**Storage**: SQLite via sqlx — no new migrations (read-only feature using existing schema)
**Testing**: Vitest 4.0.18 with happy-dom (frontend unit/component tests)
**Target Platform**: Desktop (Linux/macOS/Windows) via Tauri 2.x
**Project Type**: Tauri desktop app with SvelteKit frontend
**Performance Goals**: Detail page load < 200ms (read query SLO from constitution)
**Constraints**: No new Tauri commands; no new DB migrations; all UI strings via Paraglide
**Scale/Scope**: ~6 files changed/created; pure frontend routing + UI feature

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design._

| Principle                                | Status  | Notes                                                                             |
| ---------------------------------------- | ------- | --------------------------------------------------------------------------------- |
| Modular, Library-First Design            | ✅ PASS | New `WishlistItemSidebar` is a self-contained, scoped component                   |
| Deterministic Interfaces & Observability | ✅ PASS | Uses existing `getWishlistById` IPC command; no new transport boundary            |
| Test-First Emphasis                      | ✅ PASS | Unit tests required for sidebar component and navigation behavior                 |
| Code Quality                             | ✅ PASS | Must pass `pnpm lint`, `pnpm check`, `cargo clippy` (no Rust changes)             |
| Testing Standards                        | ✅ PASS | Vitest tests for sidebar rendering; navigation config tested via isActive utility |
| User Experience Consistency              | ✅ PASS | Uses Paraglide for all strings; follows existing sidebar/section patterns         |
| Performance Requirements                 | ✅ PASS | Single read command < 200ms; no new long-running operations                       |
| Safe Rust Practices                      | ✅ N/A  | No Rust changes required                                                          |
| Database Law                             | ✅ N/A  | No new schema or migrations                                                       |
| State Management Law                     | ✅ N/A  | No new aggregates or events                                                       |
| API Design & Transport Law               | ✅ N/A  | No new IPC commands; existing command reused                                      |
| Domain Logic Location                    | ✅ PASS | No domain logic in frontend; only display/routing                                 |

**Post-Phase-1 re-check**: All gates pass. No violations requiring justification.

## Project Structure

### Documentation (this feature)

```text
specs/027-wishlist-item-detail/
├── plan.md              # This file
├── spec.md              # Feature specification
├── research.md          # Phase 0 findings
├── data-model.md        # Phase 1 data model
├── quickstart.md        # Developer quickstart
├── contracts/
│   └── component-interfaces.md   # Component prop contracts
└── checklists/
    └── requirements.md  # Spec quality checklist
```

### Source Code

```text
src/
├── routes/
│   └── wishlists/
│       └── [wishlistId]/
│           └── items/
│               └── [itemId]/
│                   └── +page.svelte            ← NEW: detail page
├── lib/
│   ├── features/
│   │   └── wishlists/
│   │       └── components/
│   │           ├── WishlistItemSidebar.svelte  ← NEW: sidebar component
│   │           ├── WishlistItemCard.svelte     ← MODIFY: add navigation
│   │           └── WishlistItems.svelte        ← MODIFY: pass wishlistId
│   └── components/
│       └── navigation/
│           └── config.ts                       ← MODIFY: additionalPrefixes
└── messages/
    └── en.json                                 ← MODIFY: new wishlist_item_* keys
```

**Structure Decision**: Single Tauri/SvelteKit project (Option 1 variant). Frontend source in `src/`, no backend changes. All new code follows the existing feature-scoped component pattern under `src/lib/features/wishlists/`.

## Phase 0: Research

**Status**: ✅ Complete — see [research.md](research.md)

Key findings:

1. **No new Tauri commands**: `getWishlistById` returns `WishlistView` with both the wishlist name and full `items[]` array — everything the detail page and sidebar need.
2. **Navigation pattern confirmed**: `additionalPrefixes` in nav config is the correct mechanism (same approach used by collection item detail).
3. **Model card reuse confirmed**: `toRailwayModel(modelView, null, imageResponse)` correctly sets `status: 'Wishlist'` — no new mapper needed.
4. **Priority i18n already exists**: `wishlist_priority_low/normal/high` keys are in `en.json`; only status labels and detail page strings are new.
5. **WishlistItemCard has no navigation**: Click handler must be added alongside a new `wishlistId` prop.

## Phase 1: Design

**Status**: ✅ Complete — see [data-model.md](data-model.md), [contracts/](contracts/), [quickstart.md](quickstart.md)

### Implementation Steps (for tasks.md generation)

#### T1: Add i18n message keys

Add `wishlist_item_*` keys to `messages/en.json`. Full list in `contracts/component-interfaces.md`.

Reuse existing keys where possible:

- `wishlist_field_priority` → "Priority" label
- `wishlist_field_desired_price` → "Desired Price" label
- `wishlist_priority_low/normal/high` → priority enum labels
- `collection_item_not_recorded` → fallback empty state

New keys needed: `wishlist_item_back`, `wishlist_item_not_found`, `wishlist_item_not_found_message`, `wishlist_item_loading`, `wishlist_item_error`, `wishlist_item_section_details`, `wishlist_item_wishlist_name`, `wishlist_item_status`, `wishlist_item_purchased_price`, `wishlist_item_price_not_set`, `wishlist_item_section_personal_context`, `wishlist_item_added_date`, `wishlist_item_notes`, `wishlist_item_status_wanted`, `wishlist_item_status_on_order`, `wishlist_item_status_purchased`, `wishlist_item_status_ignored`.

#### T2: Fix navigation active state

In `src/lib/components/navigation/config.ts`, add `additionalPrefixes: ['/wishlists']` to the wishlists nav entry.

#### T3: Create WishlistItemSidebar component

New file: `src/lib/features/wishlists/components/WishlistItemSidebar.svelte`

Props: `item: WishlistItem`, `wishlistName: string`

Two sections:

1. **Wish List Details**: wishlist name, priority badge, status badge, desired price (or "Not set"), purchased price (if non-null)
2. **Personal Context**: added date (formatted), notes (if present)

Reference: `CollectionItemSidebar.svelte` for section/definition-list structure.

Price formatting: reuse `Intl.NumberFormat` with `{ style: 'currency', currency }` pattern from CollectionItemSidebar, converting `bigint amount` to `Number(amount) / 100`.

#### T4: Update WishlistItems to pass wishlistId

In `WishlistItems.svelte`, add `wishlistId: string` prop and pass it to each `<WishlistItemCard wishlistId={wishlistId} .../>`.

**Upstream impact**: `WishlistsDashboard.svelte` passes items to `WishlistItems`; it must also pass the active wishlist's ID.

#### T5: Add navigation click handler to WishlistItemCard

In `WishlistItemCard.svelte`:

1. Add `wishlistId: string` to the `Props` interface
2. Import `goto` from `$app/navigation`
3. Add `onclick={() => goto('/wishlists/${wishlistId}/items/${item.id}')}` on the card container element
4. Add `onclick={(e) => e.stopPropagation()}` on the Remove, Move, and Purchase button elements to prevent bubbling

#### T6: Create the detail page route

New file: `src/routes/wishlists/[wishlistId]/items/[itemId]/+page.svelte`

Pattern: mirrors `src/routes/collection/[itemId]/+page.svelte`

```typescript
// Params
const wishlistId = $page.params.wishlistId;
const itemId = $page.params.itemId;

// onMount
const wishlistView = await commands.getWishlistById(wishlistId);
const wishlistItem = wishlistView?.items.find((i) => i.id === itemId) ?? null;
if (!wishlistView || !wishlistItem) {
  notFound = true;
  return;
}

// Parallel fetches
const [model, imageResponse] = await Promise.all([
  commands.getRailwayModelById(wishlistItem.railwayModelId),
  commands.getRailwayModelImage(wishlistItem.railwayModelId)
]);

// Compose model card data
const railwayModel = toRailwayModel(model, null, imageResponse);
```

Layout: back button → two-panel (`RailwayModelCard` | `WishlistItemSidebar`)

#### T7: Write Vitest tests

Test targets:

- `WishlistItemSidebar.svelte`: renders wishlist name, priority label, desired price, "Not set" for null price, hides purchased price when null
- Navigation config: `isActive()` returns true for `/wishlists/X/items/Y` with the updated config
- `WishlistItemCard.svelte`: clicking card calls `goto`; clicking action buttons does not navigate

### Complexity Tracking

No constitution violations. No complexity justification required.
