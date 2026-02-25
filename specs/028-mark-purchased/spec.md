# Feature Specification: Mark Wishlist Item as Purchased

**Feature Branch**: `028-mark-purchased`
**Created**: 2026-02-24
**Status**: Draft
**Input**: User description: "The Mark as Purchased Action — Purchase button on Wishlist Preview Card and Wishlist Item Detail Page, modal collecting Seller (optional), Purchase Date (default: now), Price (required), Condition (dropdown). Item status changes from Wishlist to Owned."

## Clarifications

### Session 2026-02-24

- Q: Is the condition dropdown required or optional? → A: Optional — user may leave it unselected; collection entry is valid without a condition value.
- Q: Should the seller field be free text or linked to the existing sellers domain? → A: Autocomplete from known sellers only — user selects from the existing sellers list or leaves the field blank; ad-hoc name entry is not supported.
- Q: Does the price field include a currency selector or use the app's default currency? → A: Price is always recorded in the app's configured default currency; no currency picker is shown in the purchase dialog.
- Q: What is the user-facing behavior when the save operation fails? → A: The dialog stays open and displays an inline error message; the wishlist item is left unchanged; the user may retry the submission or cancel.

## User Scenarios & Testing _(mandatory)_

### User Story 1 - Purchase from Detail Page (Priority: P1)

A collector has been tracking a locomotive on their wishlist. Having found it at a fair, they open the wishlist item detail page and click "Purchase". A dialog appears pre-filled with today's date; they enter the price, optionally name the seller, select the condition, and confirm. The item is immediately added to their collection and removed from the wishlist.

**Why this priority**: The detail page is the most complete context for performing a purchase — the user has already reviewed the item. This is the primary purchase entry point and delivers the full feature value on its own.

**Independent Test**: Can be fully tested by opening a wishlist item detail page, completing the purchase dialog, and verifying the item appears in the collection with correct details and no longer appears in the wishlist.

**Acceptance Scenarios**:

1. **Given** a wishlist item with status Wanted or On Order is open on the detail page, **When** the user clicks "Purchase", **Then** a purchase dialog opens with today's date pre-filled and all other fields empty.
2. **Given** the purchase dialog is open, **When** the user provides a valid price and confirms, **Then** the item is added to the collection with the entered details, the wishlist item status changes to Purchased, and the item no longer appears in the active wishlist.
3. **Given** the purchase dialog is open with a valid price, **When** the user also provides a seller name and selects a condition, **Then** all three values are stored and visible in the collection entry.
4. **Given** the purchase dialog is open, **When** the user confirms without entering a price, **Then** the form shows a clear validation error and submission is blocked.

---

### User Story 2 - Purchase from Wishlist Preview Card (Priority: P2)

A collector is browsing their wishlist and spots a recently acquired item in the list. Without navigating to the detail page, they click the "Purchase" button on the preview card. The same purchase dialog appears, they fill in the details, and confirm. The card disappears from the wishlist.

**Why this priority**: Speeds up recording purchases when the user has multiple items to log at once. Depends on the core purchase flow (P1) being in place.

**Independent Test**: Can be fully tested by clicking the Purchase button on a wishlist preview card, completing the dialog, and verifying the item is removed from the list and appears in the collection.

**Acceptance Scenarios**:

1. **Given** a wishlist preview card is visible, **When** the user clicks its "Purchase" button, **Then** the same purchase dialog opens as in the detail page flow.
2. **Given** the purchase dialog is completed from the preview card, **When** the user confirms, **Then** the card is immediately removed from the wishlist view without requiring a page reload.

---

### User Story 3 - Cancel Purchase Flow (Priority: P3)

A collector accidentally clicks the "Purchase" button. They dismiss the dialog without filling in any details. The wishlist item remains exactly as it was.

**Why this priority**: Data safety requirement. Ensures no unintended state changes occur from accidental interaction.

**Independent Test**: Can be fully tested by opening the purchase dialog and dismissing it (close button, pressing Escape, or clicking outside), then verifying the wishlist item status is unchanged.

**Acceptance Scenarios**:

1. **Given** the purchase dialog is open, **When** the user dismisses it (via cancel button, close icon, or keyboard Escape), **Then** the wishlist item status is unchanged and no collection entry is created.
2. **Given** the purchase dialog has partially filled-in values, **When** the user dismisses it, **Then** no data is persisted and the wishlist item remains unmodified.

---

### Edge Cases

- What happens when the user submits with a price of zero? (A gifted or inherited item has no paid price — zero should be accepted as valid.)
- What happens if the purchase date is set to a future date? (Should be blocked; purchase date must be today or in the past.)
- What happens if the same wishlist item is opened in two windows and purchased in both simultaneously? (The second confirmation should fail gracefully with an informative message indicating the item has already been purchased.)
- What happens if the seller name contains only whitespace? (Should be treated as empty / not provided.)

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: System MUST display a "Purchase" action control on each wishlist item preview card, visible for items with an active (Wanted or On Order) status.
- **FR-002**: System MUST display a "Purchase" action control on the wishlist item detail page for items with an active status.
- **FR-003**: System MUST open a purchase details dialog when either Purchase control is activated.
- **FR-004**: The purchase dialog MUST include a price field that is required; the user cannot confirm the purchase without providing a price. The price is recorded in the application's configured default currency — no currency selector is shown.
- **FR-005**: The purchase dialog MUST include a purchase date field, defaulting to the current date; the user may change it to any past date but not a future date.
- **FR-006**: The purchase dialog MUST include an optional seller field that allows the user to select from the application's existing sellers list. Ad-hoc seller name entry is not supported; if no suitable seller exists in the list, the field may be left blank.
- **FR-007**: The purchase dialog MUST include an optional condition dropdown with the following options: New, Pre-Owned – Like New, Pre-Owned – Very Good, Pre-Owned – Good, Pre-Owned – Acceptable. The user may leave it unselected.
- **FR-008**: System MUST reject form submission and display a validation message if the price field is empty or blank.
- **FR-009**: System MUST reject form submission and display a validation message if the purchase date is set to a future date.
- **FR-010**: On successful confirmation, system MUST create a collection entry for the item containing the price, purchase date, seller (if provided), and condition (if provided); condition is optional and the collection entry is valid without it.
- **FR-011**: On successful confirmation, system MUST update the wishlist item's status to Purchased and record the price paid against the wishlist item.
- **FR-012**: On successful confirmation, system MUST remove the item from the active wishlist view immediately.
- **FR-013**: If the purchase dialog is dismissed without confirmation, system MUST NOT modify the wishlist item or create any collection entry.
- **FR-014**: System MUST provide visible feedback (success notification or view transition) confirming the purchase was recorded successfully.
- **FR-015**: Purchase controls MUST NOT be shown for wishlist items already in Purchased or Ignored status.
- **FR-016**: If the save operation fails after the user confirms, the purchase dialog MUST remain open, display an inline error message indicating the failure, and leave the wishlist item in its original state. The user may retry the submission or cancel.

### Key Entities

- **Wishlist Item**: A model the user intends to acquire. Has a lifecycle status (Wanted → On Order → Purchased / Ignored) and may carry a desired price for comparison. Upon purchase, the actual price paid is also recorded on this entity.
- **Purchase Details**: The transactional record associated with acquiring the item: date of purchase, price paid, seller (optional — must be a known seller from the sellers list), and the condition of the item at the time of purchase (optional).
- **Collection Entry**: The item as it exists in the user's owned inventory, linked to its catalog model reference and enriched with full purchase details.
- **Condition**: A classification of the item's physical and packaging state at time of purchase. Ranges from factory-new to well-worn, capturing both purchase type (new vs. pre-owned) and physical state.

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: Users can complete the full "mark as purchased" flow — from clicking the Purchase button to confirmation — in under 60 seconds.
- **SC-002**: After a successful purchase, the item disappears from the wishlist view and appears in the collection view without requiring a manual refresh.
- **SC-003**: All purchase details entered (price, date, seller, condition) are accurately reflected in the collection entry with no data loss.
- **SC-004**: The Purchase action is reachable in at most 2 interactions from either the wishlist list view or the item detail page.
- **SC-005**: 100% of submission attempts with a missing or blank price field are rejected with a visible, actionable error message.
- **SC-006**: Dismissing the purchase dialog in any supported way (cancel button, close icon, Escape key) leaves the wishlist item in its pre-dialog state in 100% of cases.
- **SC-007**: Users who have already purchased an item cannot trigger the purchase flow again for that same item (Purchase control is absent for Purchased-status items).
