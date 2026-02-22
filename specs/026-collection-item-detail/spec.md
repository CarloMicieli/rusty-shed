# Feature Specification: Collection Item Detail View

**Feature Branch**: `026-collection-item-detail`
**Created**: 2026-02-22
**Status**: Draft

## User Scenarios & Testing _(mandatory)_

### User Story 1 — Navigate to a Collection Item (Priority: P1)

As a collector, when I tap on an item in my collection list I want to be taken to a dedicated detail page for that specific item, so I can see full context about that particular piece without losing my place in the app.

**Why this priority**: This is the foundation of the feature — without a working route, no other stories can be tested. It directly replaces the existing but awkward model-ID-based route.

**Independent Test**: Open the collection, tap any item card. The app navigates to a clean URL (`/collection/{itemId}`) and the "Collection" entry in the main navigation remains visually highlighted. Navigating back returns to the main collection page.

**Acceptance Scenarios**:

1. **Given** the user is on the main collection page, **When** they select a collection item card, **Then** the app navigates to `/collection/{itemId}` where `{itemId}` is the collection item's unique identifier.
2. **Given** the user is on a collection item detail page, **When** they look at the navigation sidebar/bar, **Then** the "Collection" navigation entry is highlighted as the active section.
3. **Given** the user is on a collection item detail page, **When** they press the back button, **Then** they are returned to the main collection page (`/my-collection`).
4. **Given** the user navigates directly to `/collection/{itemId}` via URL, **When** the page loads, **Then** the correct item data is displayed and the navigation correctly shows "Collection" as active.

---

### User Story 2 — View Railway Model Card (Priority: P2)

As a collector, I want to see the railway model information (the existing model card) displayed prominently on the detail page, so I have the full catalogue description alongside my personal collection data.

**Why this priority**: The main content area already exists and must be preserved during the route refactor. It forms the left/primary panel of the two-panel layout.

**Independent Test**: Navigate to a collection item detail page and verify the existing railway model card renders correctly with model name, manufacturer, scale, epoch, category, and any associated image.

**Acceptance Scenarios**:

1. **Given** the user is on the collection item detail page, **When** the page loads, **Then** the railway model card is displayed in the main content area with all model details visible.
2. **Given** a collection item whose railway model has an image, **When** the detail page loads, **Then** the model image is displayed within the card.

---

### User Story 3 — View Acquisition Summary (Priority: P2)

As a collector, I want to see purchase details — seller, date, and price — in a sidebar panel so I can quickly recall where and when I bought the item and what I paid.

**Why this priority**: This is the financial identity of the item. It is the most commonly referenced personal data a collector needs at a glance.

**Independent Test**: Navigate to a collection item that has purchase information recorded. The sidebar displays the seller name, transaction date in a human-readable format (e.g., "Feb 22, 2026"), and the price with currency symbol.

**Acceptance Scenarios**:

1. **Given** a collection item with purchase information, **When** the detail page loads, **Then** the sidebar displays the seller name, purchase date formatted readably, and the price with its currency.
2. **Given** the seller has a website URL on record, **When** the sidebar renders, **Then** the seller name is displayed as a clickable link to that website.
3. **Given** a collection item with no purchase information recorded, **When** the detail page loads, **Then** the Acquisition Summary section is either hidden or shows a clear "not recorded" state without errors.

---

### User Story 4 — View Condition & Grading (Priority: P3)

As a collector, I want to see the model and box condition at a glance so I can quickly assess the item's quality and resale viability.

**Why this priority**: Condition is secondary to acquisition data but important for catalogue-quality collectors.

**Independent Test**: Navigate to a collection item where condition fields are set. The sidebar shows model condition, box condition, and whether the item was purchased new or second-hand, each clearly labelled.

**Acceptance Scenarios**:

1. **Given** a collection item with condition data, **When** the sidebar loads, **Then** model condition, box condition, and purchase condition (new/second-hand) are each displayed with a distinct label or badge.
2. **Given** a collection item with no condition data recorded, **When** the sidebar loads, **Then** the Condition section shows a graceful "not recorded" state.

---

### User Story 5 — View Operational Snapshot (Priority: P3)

As a layout operator, I want to see the DCC address and installed decoder for this item in the sidebar so I can quickly reference it while at my controller without opening a separate app.

**Why this priority**: Operational data serves a specific hands-on use case. Useful but not blocking for initial delivery.

**Independent Test**: Navigate to a collection item with a rolling stock that has a DCC address assigned. The sidebar prominently displays the DCC address number and the decoder model name.

**Acceptance Scenarios**:

1. **Given** a collection item with rolling stock that has a DCC address, **When** the sidebar renders, **Then** the DCC address is prominently displayed.
2. **Given** a collection item with an installed decoder on record, **When** the sidebar renders, **Then** the decoder model/name is displayed.
3. **Given** a collection item with no rolling stock or DCC data, **When** the sidebar renders, **Then** the Operational section shows a graceful "not configured" state.

---

### User Story 6 — View Personal Context (Priority: P4)

As a collector, I want to see when I added the item to my shed and any personal notes so I can remember the item's story and provenance.

**Why this priority**: Personal notes and dates are supplementary context. Valuable but last in priority.

**Independent Test**: Navigate to a collection item with a notes value and an `added_date`. The sidebar shows the formatted date and a truncated preview of the notes text.

**Acceptance Scenarios**:

1. **Given** a collection item with an `added_date`, **When** the sidebar renders, **Then** the date is displayed in a human-readable format.
2. **Given** a collection item with notes text, **When** the sidebar renders, **Then** a truncated preview of the notes is displayed (full text available on expansion or tooltip).
3. **Given** a collection item with no notes, **When** the sidebar renders, **Then** no notes section is shown or an empty state is indicated clearly.

---

### Edge Cases

- What happens when a collection item ID in the URL does not exist (deleted item, bad link)? The page must show a clear "item not found" state and offer navigation back to the collection.
- What happens when a collection item exists but the associated railway model has been removed from the catalogue? The sidebar data should still render; the model card should show a graceful degraded state.
- What happens when a collection item has multiple rolling stocks? The Operational Snapshot should handle displaying one or more entries without overflowing.
- What happens when the notes field is very long? The sidebar must truncate the display without breaking layout.
- What happens when the screen is narrow (e.g., tablet portrait)? The two-panel layout must adapt gracefully — sidebar may collapse or stack below the model card.

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: The system MUST serve the collection item detail view at the URL path `/collection/{itemId}`, where `{itemId}` is the unique identifier of a collection item.
- **FR-002**: The "Collection" entry in the main application navigation MUST remain in its active/highlighted visual state when the user is on any `/collection/{itemId}` page.
- **FR-003**: The detail page MUST include a back navigation control that returns the user to the main collection page (`/my-collection`).
- **FR-004**: The detail page MUST display the existing railway model card in the primary (left) content area.
- **FR-005**: The detail page MUST display a sidebar (right panel) containing collection-instance-specific information: acquisition summary, condition grading, operational snapshot, and personal context.
- **FR-006**: The Acquisition Summary section MUST display the seller name, purchase date (human-readable), and purchased price with currency from the purchase record linked to the collection item.
- **FR-007**: If the seller record contains a website URL, the seller name MUST be rendered as a hyperlink to that URL.
- **FR-008**: The Condition & Grading section MUST display the model condition, box condition, and purchase condition (new vs. second-hand) for the collection item.
- **FR-009**: The Operational Snapshot section MUST display the DCC address and installed decoder name/model for rolling stock associated with the collection item.
- **FR-010**: The Personal Context section MUST display the `added_date` (human-readable) and a truncated preview of the `notes` field from the collection item record.
- **FR-011**: All sidebar sections MUST degrade gracefully when the relevant data is absent — showing a clear empty/not-recorded state rather than errors or empty boxes.
- **FR-012**: The system MUST display a clear "item not found" state when an invalid or non-existent `{itemId}` is requested.
- **FR-013**: The old route (`/models/[...modelId]`) MUST either be removed or redirect to the new `/collection/{itemId}` route to avoid broken navigation.

### Key Entities

- **CollectionItem**: A specific instance of a railway model owned by the user. Holds condition fields (`model_condition`, `box_condition`, `purchase_condition`), `added_date`, `removed_date`, and `notes`. Belongs to a Collection and references a RailwayModel.
- **PurchaseInfo**: The financial record of acquiring a CollectionItem. Holds `purchase_date`, `purchased_price_amount`, `purchased_price_currency`, and references a Seller.
- **Seller**: The vendor from whom the item was acquired. Holds `name` and optionally a website URL.
- **OwnedRollingStock**: A specific rolling stock unit within a CollectionItem. Holds `dcc_address` and references an installed decoder.
- **RailwayModel**: The catalogue entry for the model. Displayed in the main card area (not the sidebar).

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: A collector can navigate from the collection list to a specific item's detail page and back in under 5 seconds on a standard desktop.
- **SC-002**: All four sidebar sections (Acquisition, Condition, Operational, Personal) are visible without scrolling on a standard 1080p display when data is populated.
- **SC-003**: The "Collection" navigation item remains highlighted on the collection item detail page in 100% of cases — no scenario leaves the navigation in an unhighlighted or ambiguous state.
- **SC-004**: Every sidebar section handles missing data gracefully — zero unhandled errors occur when any data field is absent.
- **SC-005**: The page loads and renders all visible sidebar data without a secondary loading phase — populated items appear complete on first render.

## Assumptions

- The `sellers` table may or may not have a website URL column; if absent, the seller name is displayed as plain text and **FR-007** is deferred until schema support exists.
- A collection item has at most one active `purchase_info` record; if multiple exist, the most recent by `purchase_date` is displayed.
- The existing `/models/[...modelId]` route is used only internally within the app (not shared as external links), so a simple redirect or removal is sufficient for **FR-013**.
- "Collection" navigation active state will be achieved by configuring the collection nav entry to use prefix-based path matching, covering both `/my-collection` and `/collection/*` paths.
- The two-panel layout (model card left, sidebar right) is the desktop default; graceful stacking on narrow screens is a best-effort outcome, not a hard requirement for initial delivery.
