# Research: Wishlist Item Sidebar Inline Editing

**Feature**: 037-wishlist-inline-edit  
**Date**: 2026-03-09  
**Status**: Complete — all NEEDS CLARIFICATION resolved

---

## 1. Does an `update_wishlist_item` Tauri command already exist?

**Decision**: No. The current wishlist command surface (`src-tauri/src/wishlist/interface/command_handlers.rs`) contains `add_to_wishlist`, `remove_from_wishlist`, `move_item_to_list`, `purchase_wishlist_item`, and several read/meta commands — but no command for editing individual item fields (priority, status, desired price, added date) post-creation.

**Rationale**: A new Tauri command `update_wishlist_item` must be introduced, together with a corresponding use case and domain event.

**Alternatives considered**: Reusing `add_to_wishlist` with a remove + re-add pattern was rejected because it risks losing data (e.g., `purchasedPrice`, `notes`) and breaks referential integrity on the item's stable `id`.

---

## 2. Domain Event pattern for item mutation

**Decision**: Add a new `WishlistEvent::ItemUpdated { ... }` variant to `src-tauri/src/wishlist/domain/wishlist_event.rs`. The event carries only the item's `id` plus the four mutable fields (all as `Option` to support partial updates). Add a corresponding `update_item(...)` method to the `Wishlist` aggregate. The repository maps the event to targeted `UPDATE` SQL statement covering those four columns.

**Rationale**: The codebase strictly follows the Domain Event Tracking pattern (Constitution / ADR). All persistent state changes MUST be expressed as domain events drained atomically by the repository. Ad-hoc SQL outside the event/repository layer is forbidden.

**Alternatives considered**: A PATCH-style command that directly runs SQL was rejected per the Persistence Constitution Law.

---

## 3. Currency access in the frontend

**Decision**: The Svelte frontend reads the default currency from `SettingsState.settings.currency` (a `$state` field on the `SettingsState` class). `SettingsState` is already available via Svelte context in the `+layout.svelte` tree. `WishlistItemSidebar.svelte` must receive `defaultCurrency: string` as a prop (passed from the parent page/component that already holds the settings context), or call `getContext` directly.

**Rationale**: `SettingsState` is the single source of truth for user preferences. Currency is already stored there (`settings.currency`, default `'EUR'`). No new Tauri call is needed just to read the currency.

**Alternatives considered**: Calling `invoke('get_settings')` from inside the sidebar was rejected as it creates redundant IPC traffic and breaks the existing state-management contract.

---

## 4. Calendar / date-picker component availability

**Decision**: The `calendar` shadcn-svelte component does **not** yet exist under `src/lib/components/ui/`. It must be added via `pnpm dlx shadcn-svelte@latest add calendar`. This brings in the `bits-ui` `Calendar` primitive (already used by other shadcn primitives in the project). The date-picker for the "Added" field will be assembled as a `Popover` (already available as `src/lib/components/ui/...` — wait, `popover` is not in the current list either). Both `calendar` and `popover` components must be added.

**Rationale**: The project uses shadcn-svelte as the component library; adding unseen components from the same library is compliant with the existing architecture and does not introduce a new dependency constellation.

**Alternatives considered**: Using a third-party date-picker library was rejected to keep the UI consistent and avoid new dependencies (Hard Constraint: never add dependencies without user approval). The use of a plain `<input type="date">` was considered but rejected for UI consistency with the project's dark-mode design system.

---

## 5. Inline edit UX pattern — single field active at a time

**Decision**: Manage which field is currently active using a single `$state<'priority' | 'status' | 'price' | 'date' | null>` reactive variable inside `WishlistItemSidebar.svelte`. Opening a new field sets this variable; the previous field's component reacts by reverting to read-only without persisting. Escape and outside-click are handled per-field with `onkeydown` and a `use:clickOutside` action.

**Rationale**: Svelte 5 Runes make this trivial without a store. A single discriminated-union variable is simpler and more type-safe than multiple boolean flags.

**Alternatives considered**: A per-field `isEditing` boolean array was rejected for verbosity. A full form with a submit button was rejected as it contradicts the hover-and-click design requirement.

---

## 6. Hover affordance implementation

**Decision**: Use a CSS `group/field` hover pattern (Tailwind `group-hover/field:visible`) to show a small pencil icon (`Pencil` from `lucide-svelte`, already in the project) on the `<dd>` element. The field row `<div>` gets `group/field relative cursor-pointer rounded-sm transition-colors hover:bg-white/5`.

**Rationale**: Tailwind v4 named groups (`group/field`) allow precise hover scoping without JavaScript. This matches the existing dark-mode aesthetic.

---

## 7. Persisting changes — optimistic update vs. wait-for-backend

**Decision**: Use optimistic update. On user confirmation, immediately update the local `WishlistItem` prop copy in the sidebar (or propagate up via a callback prop `onUpdate`), fire the `update_wishlist_item` IPC command, and revert on error with a `toaster.error(...)` notification (`sonner` already used in `WishlistState`).

**Rationale**: Optimistic updates deliver the snappy UX implied by the spec ("immediately saves the change"). Sonner is already wired for error toasts in the wishlist feature.

---

## 8. Testing approach

**Decision**:

- **Unit tests** (Vitest/happy-dom): Test `WishlistItemSidebar.svelte` — field activation, Escape/blur cancellation, inline validation of price. Mock `invoke`.
- **Rust unit tests**: Test `Wishlist::update_item(...)` domain method — valid/invalid field combinations, event emission.
- **Rust integration tests**: Test the repository's handling of `WishlistEvent::ItemUpdated` against a real in-memory SQLite database.

**Rationale**: Constitution requires unit + integration test coverage for new features. Domain logic MUST be tested in Rust; UI logic MUST be tested with Vitest.
