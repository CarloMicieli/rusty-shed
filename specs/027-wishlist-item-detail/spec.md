# Feature Specification: Wishlist Item Detail View

**Feature Branch**: `027-wishlist-item-detail`
**Created**: 2026-02-23
**Status**: Draft

## User Scenarios & Testing _(mandatory)_

### User Story 1 — Navigate to a Wishlist Item (Priority: P1)

As a collector, when I tap on a wishlist item card in my wishlist I want to be taken to a dedicated detail page for that specific item, so I can see the full context about that particular model and my personal wishlist data for it.

**Why this priority**: This is the foundation of the feature — the navigation fix and new route must exist before any other story can be tested. Currently clicking an item card on the wishlist page does not navigate to a detail view.

**Independent Test**: Open the wishlist page, select a wishlist, and tap any item card. The app navigates to `/wishlists/{wishlistId}/items/{itemId}`. The "Wishlists" entry in the main navigation remains visually highlighted. Navigating back returns to the wishlist page.

**Acceptance Scenarios**:

1. **Given** the user is on the main wishlists page viewing a wishlist's items, **When** they click/tap on an item card, **Then** the app navigates to `/wishlists/{wishlistId}/items/{itemId}` where `{wishlistId}` is the parent wishlist's identifier and `{itemId}` is the wishlist item's unique identifier.
2. **Given** the user is on a wishlist item detail page, **When** they look at the navigation sidebar/bar, **Then** the "Wishlists" navigation entry is highlighted as the active section.
3. **Given** the user is on a wishlist item detail page, **When** they press the back button or back control, **Then** they are returned to the main wishlists page (`/my-wishlists`).
4. **Given** the user navigates directly to `/wishlists/{wishlistId}/items/{itemId}` via URL, **When** the page loads, **Then** the correct item data is displayed and the navigation correctly shows "Wishlists" as active.

---

### User Story 2 — View Railway Model Card (Priority: P2)

As a collector, I want to see the railway model information (the existing model card component) displayed prominently on the detail page, so I have the full catalogue description alongside my personal wishlist data for that item.

**Why this priority**: The model card forms the primary content of the page — it answers "which model is this?" The sidebar is secondary context.

**Independent Test**: Navigate to a wishlist item detail page and verify the railway model card renders correctly with model name, manufacturer, scale, epoch, category, and any associated image.

**Acceptance Scenarios**:

1. **Given** the user is on the wishlist item detail page, **When** the page loads, **Then** the railway model card is displayed in the main content area with all model details visible.
2. **Given** a wishlist item whose railway model has an image, **When** the detail page loads, **Then** the model image is displayed within the card.
3. **Given** a wishlist item whose railway model has no image, **When** the detail page loads, **Then** the model card renders with a placeholder without errors.

---

### User Story 3 — View Wishlist Context (Priority: P2)

As a collector, I want to see wishlist-specific information — the name of the wishlist the item belongs to, the item's priority, and my desired price — in a sidebar panel so I can quickly recall my intent and budget target for this model.

**Why this priority**: This is the personal data that distinguishes a wishlist item from a generic catalogue entry. It is the core reason for a dedicated detail page.

**Independent Test**: Navigate to a wishlist item that has priority and desired price set. The sidebar displays the wishlist name, the priority level (Low/Normal/High), and the desired price with currency symbol. Navigating to an item with no desired price shows a clear "not set" state.

**Acceptance Scenarios**:

1. **Given** a wishlist item with a desired price set, **When** the detail page loads, **Then** the sidebar displays the parent wishlist name, the item priority rendered as a human-readable label (e.g., "High", "Normal", "Low"), and the desired price formatted with its currency.
2. **Given** a wishlist item with no desired price set, **When** the detail page loads, **Then** the desired price field shows a clear "not set" state without errors.
3. **Given** a wishlist item with a status of PURCHASED (i.e., a purchased price was recorded), **When** the sidebar renders, **Then** the purchased price is also displayed alongside the desired price for comparison.
4. **Given** the user is viewing the sidebar, **When** the wishlist name is visible, **Then** it is displayed as a readable label (not a raw ID or technical identifier).

---

### User Story 4 — View Item Notes & Dates (Priority: P3)

As a collector, I want to see when I added the item to my wishlist and any personal notes so I can remember why I wanted this model.

**Why this priority**: Notes and dates are supplementary context — useful but not blocking for a working first delivery.

**Independent Test**: Navigate to a wishlist item with a notes value and an `added_date`. The sidebar shows the formatted date and a truncated preview of the notes text.

**Acceptance Scenarios**:

1. **Given** a wishlist item with an `added_date`, **When** the sidebar renders, **Then** the date is displayed in a human-readable format.
2. **Given** a wishlist item with notes text, **When** the sidebar renders, **Then** the notes are displayed (with truncation if very long).
3. **Given** a wishlist item with no notes, **When** the sidebar renders, **Then** no notes section is shown or an empty state is clearly indicated.

---

### Edge Cases

- What happens when a wishlist item ID in the URL does not exist (deleted item, bad link)? The page must show a clear "item not found" state and offer navigation back to the wishlists page.
- What happens when the `wishlistId` in the URL does not match the item's actual parent wishlist? The page should resolve from the item's own ID and display the correct data; mismatched parent IDs should not cause crashes.
- What happens when a wishlist item exists but the associated railway model has been removed from the catalogue? The sidebar data should still render; the model card should show a graceful degraded state.
- What happens when the notes field is very long? The sidebar must truncate the display without breaking layout.
- What happens when the screen is narrow? The two-panel layout must adapt gracefully — sidebar may collapse or stack below the model card.

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: The system MUST serve the wishlist item detail view at the URL path `/wishlists/{wishlistId}/items/{itemId}`, where `{wishlistId}` is the parent wishlist's unique identifier and `{itemId}` is the wishlist item's unique identifier.
- **FR-002**: The "Wishlists" entry in the main application navigation MUST remain in its active/highlighted visual state when the user is on any `/wishlists/*/items/*` page.
- **FR-003**: The detail page MUST include a back navigation control that returns the user to the main wishlists page (`/my-wishlists`).
- **FR-004**: The detail page MUST display the railway model card in the primary (left) content area, reusing the existing model card component.
- **FR-005**: The detail page MUST display a sidebar (right panel) containing wishlist-instance-specific information: wishlist name, item priority, desired price, item status, and personal context (added date, notes).
- **FR-006**: The Wishlist Context section MUST display the name of the parent wishlist, the item priority as a human-readable label, and the desired price with currency (if set).
- **FR-007**: If the item has a purchased price recorded (status is PURCHASED), the sidebar MUST also display the purchased price alongside the desired price.
- **FR-008**: All sidebar fields MUST degrade gracefully when the relevant data is absent — showing a clear empty/not-set state rather than errors or empty boxes.
- **FR-009**: The system MUST display a clear "item not found" state when an invalid or non-existent `{itemId}` is requested.
- **FR-010**: The wishlist item cards on the `/my-wishlists` page MUST navigate to the corresponding wishlist item detail page when clicked/tapped.
- **FR-011**: The item priority MUST be rendered using a visual indicator (e.g., badge or icon) consistent with the priority level (Low/Normal/High), matching the visual language used elsewhere in the app.

### Key Entities

- **WishlistItem**: A specific model the user wants to acquire. Holds `priority` (LOW/NORMAL/HIGH), `status` (WANTED/ON_ORDER/PURCHASED/IGNORED), `added_date`, `removed_date`, `notes`, `desired_price`, `purchased_price`, and references a `RailwayModel`.
- **Wishlist**: The parent list. Contributes its `name` to the sidebar display. Referenced by `{wishlistId}` in the URL.
- **RailwayModel**: The catalogue entry for the model. Displayed in the main card area (not the sidebar).

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: A collector can navigate from the wishlists page to a specific item's detail page and back in under 5 seconds on a standard desktop.
- **SC-002**: The wishlist context sidebar (wishlist name, priority, desired price) is visible without scrolling on a standard 1080p display when data is populated.
- **SC-003**: The "Wishlists" navigation item remains highlighted on the wishlist item detail page in 100% of cases — no scenario leaves the navigation in an unhighlighted or ambiguous state.
- **SC-004**: Every sidebar field handles missing data gracefully — zero unhandled errors occur when any optional field (desired price, notes) is absent.
- **SC-005**: The navigation fix on the wishlists page is complete — 100% of item card clicks navigate to the correct detail URL (previously no navigation occurred).

## Assumptions

- The existing railway model card component (built for collection item detail in feature 026) is reused as-is for the model card area; no new model card component is required.
- A wishlist item is always associated with exactly one parent wishlist; the `{wishlistId}` URL segment is used for route clarity and navigation context but the item's own ID is the authoritative lookup key.
- The `desired_price` is the "wanted price" referenced in the user description; it maps to the `desired_price` field on `WishlistItem`.
- "Name of the wishlist" in the sidebar refers to the `Wishlist.name` field of the parent wishlist, not the item's own identifier.
- The two-panel layout (model card left, sidebar right) mirrors the collection item detail layout; graceful stacking on narrow screens is a best-effort outcome, not a hard requirement for initial delivery.
- Clicking an item card on the wishlists page is the only navigation path being fixed; other actions (remove, move, purchase) remain as-is on the main wishlists page.
