# Feature Specification: Acquisition Flow

**Feature Branch**: `038-acquisition-flow`
**Created**: 2026-03-12
**Status**: Draft
**Input**: User description: "feature 38: the Acquisition Flow — New Acquisition drawer with parent-child purchase model"

## User Scenarios & Testing _(mandatory)_

### User Story 1 — Record a Single-Item Purchase (Priority: P1)

A collector just bought a single locomotive at a hobby shop. They open the app, click "New Acquisition" on the Dashboard, fill in the seller and date, enter the model's details, and click "Finalize Purchase." The item is immediately recorded in their collection as a purchased acquisition.

**Why this priority**: This is the core happy path. Without a working single-item purchase flow, no other scenario is meaningful.

**Independent Test**: Open the acquisition drawer, complete the purchase metadata and one item entry, finalize — verify the item appears in the collection with the correct seller, date, and model details.

**Acceptance Scenarios**:

1. **Given** the Dashboard is open, **When** the user clicks "New Acquisition," **Then** the acquisition drawer opens without navigating away from the Dashboard.
2. **Given** the drawer is open, **When** the user enters a purchase date and selects a seller, **Then** the date defaults to today and sellers are searchable by name.
3. **Given** valid purchase metadata and one complete item entry, **When** the user clicks "Finalize Purchase," **Then** one new collection item is created with the specified purchase details and the drawer closes.
4. **Given** the finalization succeeds, **When** the drawer closes, **Then** the Dashboard's "Recent Acquisitions" section reflects the new item.

---

### User Story 2 — Record a Multi-Item Haul with Batch Defaults (Priority: P2)

A collector returns from a train show with five freight cars from the same manufacturer, all the same scale and DC power method. They set the scale and power method once in the "Batch Defaults" section, then quickly add each item by entering only the unique fields (product code, road number). They use the "Clone" button to duplicate similar items.

**Why this priority**: Multi-item entry with batch defaults is the primary UX differentiator of this feature. It directly reduces repetitive data entry — the feature's stated "magic."

**Independent Test**: Open the drawer, set batch defaults for Scale and Power Method, add three items using Clone, finalize — verify all three items are in the collection sharing the same scale/power method.

**Acceptance Scenarios**:

1. **Given** the user sets Scale = "H0" and Power Method = "DC" in Batch Defaults, **When** a new item entry is added, **Then** its Scale and Power Method fields are pre-filled with those defaults.
2. **Given** a pre-filled item card is displayed, **When** the user clicks the "Duplicate" icon, **Then** a new item card appears below it with all fields copied except Product Code (which is cleared to empty).
3. **Given** multiple item cards are present, **When** the user finalizes, **Then** all items are created as separate collection entries under the same purchase.
4. **Given** a batch default is set, **When** the user manually changes Scale on one specific item card, **Then** only that item's Scale is changed; other cards and the batch defaults are unaffected.

---

### User Story 3 — Open Acquisition Drawer via Keyboard Shortcut (Priority: P3)

A power user is browsing their catalogue when they spot a model they just acquired. Without navigating to the Dashboard, they press Ctrl+N to instantly open the acquisition drawer.

**Why this priority**: A global shortcut adds convenience for experienced users, but the feature delivers full value without it.

**Independent Test**: From any page in the app, press Ctrl+N — verify the acquisition drawer opens.

**Acceptance Scenarios**:

1. **Given** the user is on any page of the app, **When** Ctrl+N is pressed, **Then** the acquisition drawer opens immediately.
2. **Given** the acquisition drawer is already open, **When** Ctrl+N is pressed again, **Then** the drawer remains open (no toggle-close behavior).

---

### User Story 4 — Validate and Recover from Incomplete Entries (Priority: P2)

The user attempts to finalize a purchase without filling in required fields. The system highlights the missing fields and prevents submission until they are resolved.

**Why this priority**: Data integrity and user confidence depend on clear, non-blocking inline validation that guides correction rather than frustrating the user.

**Independent Test**: Attempt to finalize with an empty item list or a missing required field — verify targeted error indicators appear and the drawer remains open with all data intact.

**Acceptance Scenarios**:

1. **Given** the item list is empty, **When** the user clicks "Finalize Purchase," **Then** an inline message prompts the user to add at least one item before finalizing.
2. **Given** a required field (Manufacturer or Product Code) is empty on an item card, **When** the user clicks "Finalize Purchase," **Then** the specific card is highlighted with an error indicator and the submission is blocked.
3. **Given** validation errors are displayed, **When** the user corrects the flagged fields, **Then** the error indicators clear immediately without requiring another submit attempt.

---

### Edge Cases

- What happens if the user closes the drawer mid-session with items already entered? The system must prompt a discard-confirmation before closing.
- What happens if a seller search returns no results? The user sees a "no results" state; adding new sellers is out of scope for this feature.
- What happens if the same product code is entered for two item cards in the same session? Each is treated as a separate purchase line — no automatic deduplication within the drawer.
- What happens if "Finalize Purchase" fails due to a backend error? The drawer remains open, all entered data is preserved, and a visible error message is shown.
- Can the purchase date be set to a future date? No — the date picker must restrict selection to today or any past date.
- What if the user enters a model whose Manufacturer + Product Code already exists? The system silently reuses the existing catalog entry and records only the purchase — the user sees no error or disruption.
- What if the user has not configured a preferred currency in settings? A fallback currency (EUR) is used and displayed on the price field.

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: The Dashboard MUST display a "New Acquisition" button that replaces the existing "Add Railway Model" quick-action button.
- **FR-002**: Clicking "New Acquisition" MUST open a side drawer without navigating away from the Dashboard.
- **FR-003**: The drawer MUST contain a sticky Purchase Metadata section with: Seller (searchable combobox), Date (date picker defaulting to today), Scale (dropdown), and Power Method (dropdown).
- **FR-004**: The Scale and Power Method selections in the Purchase Metadata section MUST act as batch defaults, auto-populating newly added item cards.
- **FR-005**: The drawer MUST contain a scrollable Item List where each item card contains: Manufacturer (searchable dropdown), Product Code (short text), Description (single-line text), Epoch (dropdown), Category (dropdown), and Price (numeric input).
- **FR-006**: The Price field on each item card MUST display the user's preferred currency (as configured in application settings); the currency symbol is shown but not editable per-item.
- **FR-007**: Item cards MUST inherit batch-default Scale and Power Method values but MUST allow the user to override those fields per card.
- **FR-008**: Each item card MUST have a "Duplicate" action that creates a copy of that card with all fields preserved except Product Code, which is reset to empty.
- **FR-009**: The drawer MUST contain a sticky footer with "Add Another Item" and "Finalize Purchase" actions.
- **FR-010**: "Add Another Item" MUST append a new empty item card pre-filled with current batch defaults to the item list and scroll it into view.
- **FR-011**: "Finalize Purchase" MUST create one collection entry per item card, all associated with the same purchase (seller + date + price per item).
- **FR-012-a**: For each item, if the combination of Manufacturer and Product Code does not yet exist in the catalog, the system MUST create a new catalog entry before recording the purchase.
- **FR-012-b**: For each item, if the combination of Manufacturer and Product Code already exists in the catalog, the system MUST skip catalog creation and record only the purchase against the existing catalog entry.
- **FR-013**: The system MUST prevent finalization if the item list is empty or if any item card is missing Manufacturer, Product Code, or Category.
- **FR-014**: The system MUST display inline validation errors on the specific item card(s) with missing required fields; errors MUST clear as the user corrects them.
- **FR-015**: Closing the drawer while unsaved item data exists MUST present a discard-confirmation prompt before dismissing.
- **FR-016**: The keyboard shortcut Ctrl+N MUST open the acquisition drawer from any screen in the application.
- **FR-017**: The purchase date field MUST reject future dates; the date picker must limit selection to today and earlier.

### Key Entities

- **Acquisition**: A recorded purchase event. Has one seller, one date, and one or more line items. All items share the same purchase metadata.
- **Acquisition Line Item**: A single model being acquired. Has manufacturer, product code, description, epoch, category, scale, power method, and price (in the user's preferred currency). Belongs to exactly one acquisition.
- **Seller**: A known vendor or individual from whom the collector buys models. Searchable by name. Read-only within this flow (seller management is out of scope).
- **Batch Defaults**: Transient session-level state (Scale + Power Method) that auto-fills new item cards. Does not persist after the drawer is closed.

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: A collector can record a single-item purchase — from opening the drawer to finalization — in under 60 seconds.
- **SC-002**: A collector can record a 5-item haul using Clone and batch defaults in under 3 minutes.
- **SC-003**: All finalized items appear in the collection immediately after the drawer closes, with no manual page refresh required.
- **SC-004**: Validation errors are surfaced without submitting the form — the user never loses entered data due to a failed or blocked submission.
- **SC-005**: The Ctrl+N shortcut opens the acquisition drawer within 300ms from any screen.
- **SC-006**: First-time users can locate the "New Acquisition" entry point within 30 seconds of opening the Dashboard, without external guidance.

## Assumptions

- **Seller lookup is read-only in this flow**: The user must add new sellers through the Sellers management area before recording an acquisition. Creating sellers inline is out of scope.
- **Catalog entry creation is conditional**: If a catalog entry for the given Manufacturer + Product Code already exists, the system skips creation and records only the purchase. This prevents primary-key conflicts and avoids duplicating known models.
- **Price is per item, currency is global**: Each line item records its own price amount. The currency is not selectable per-item — it is always the currency configured in user settings. Price is optional; a blank price field is recorded as no price.
- **Scale and Power Method batch defaults are optional**: The user may finalize without setting batch defaults; those fields remain empty on item cards unless manually filled.
- **Epoch field is free-text or a known list**: Epoch accepts values like "III", "IV", "V" — the exact list is resolved by the existing catalog enumeration; no new epoch types are introduced.
