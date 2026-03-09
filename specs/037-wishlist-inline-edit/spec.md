# Feature Specification: Wishlist Item Sidebar Inline Editing

**Feature Branch**: `037-wishlist-inline-edit`  
**Created**: 2026-03-09  
**Status**: Draft  
**Input**: User description: "I want to make the wishlist item sidebar supports in-place editing"

## User Scenarios & Testing _(mandatory)_

### User Story 1 - Edit Priority via Dropdown (Priority: P1)

A collector viewing their wishlist item sidebar wants to quickly change the item's priority without navigating away. They hover over the Priority badge and see a visual cue that it is editable. Clicking it replaces the badge with a dropdown showing the available priority levels. Selecting an option immediately saves the change and reverts the field back to its read-only badge display.

**Why this priority**: Priority is the most frequently adjusted field and delivers the clearest demonstration of the hover-and-click interaction pattern for the whole sidebar.

**Independent Test**: Can be fully tested by hovering over the Priority field, clicking, selecting a different value, and confirming the badge reflects the new value. Delivers core inline-edit UX independently of all other fields.

**Acceptance Scenarios**:

1. **Given** the wishlist item sidebar is displayed, **When** the user hovers over the Priority field value, **Then** a subtle visual affordance (highlight or pencil icon) appears to indicate it is editable.
2. **Given** the priority field is in hover state, **When** the user clicks the priority value, **Then** a dropdown replaces the badge, is auto-focused, and shows all valid priority values (LOW, NORMAL, HIGH) with the current value pre-selected.
3. **Given** the priority dropdown is open, **When** the user selects a different priority, **Then** the change is persisted, the dropdown closes, and the updated badge is displayed.
4. **Given** the priority dropdown is open, **When** the user presses Escape or clicks outside the dropdown, **Then** no change is made and the field reverts to read-only display.

---

### User Story 2 - Edit Status via Dropdown (Priority: P1)

A collector wants to update the procurement status of a wishlist item (e.g., from "Wanted" to "On Order") directly from the sidebar, without opening a separate form.

**Why this priority**: Status drives the item lifecycle and is closely related to Priority in terms of interaction pattern; both are dropdowns and should be implemented together.

**Independent Test**: Can be fully tested by clicking the Status field, selecting a different value, and confirming the badge reflects the new status.

**Acceptance Scenarios**:

1. **Given** the sidebar is displayed, **When** the user hovers over the Status field value, **Then** the visual affordance appears.
2. **Given** the status field is in hover state, **When** the user clicks the status value, **Then** a dropdown appears with all valid status values (WANTED, ON_ORDER, PURCHASED, IGNORED) and the current value is pre-selected.
3. **Given** the status dropdown is open, **When** the user selects a new status, **Then** the change is persisted and the badge updates accordingly.
4. **Given** the status dropdown is open, **When** the user presses Escape or clicks outside, **Then** no change is made.

---

### User Story 3 - Edit Desired Price via Input (Priority: P2)

A collector wants to record how much they are willing to pay for an item. They click the "Desired Price" field value (showing "Not set" if empty), enter a numeric amount, and confirm. The currency defaults to the currency set in the application settings.

**Why this priority**: Price editing is valuable but involves more complex validation (numeric input, currency handling), making it a distinct P2 slice.

**Independent Test**: Can be fully tested by clicking the Desired Price field, entering a numeric value, confirming, and verifying the formatted price is displayed using the settings currency.

**Acceptance Scenarios**:

1. **Given** the sidebar is displayed, **When** the user hovers over the Desired Price value, **Then** the visual affordance appears.
2. **Given** the desired price field lacks a value, **When** the user clicks "Not set", **Then** an input box appears, auto-focused, empty, with the application default currency label shown.
3. **Given** the desired price field has a value, **When** the user clicks the formatted price, **Then** an input box appears auto-focused with the existing amount pre-selected.
4. **Given** the price input is active, **When** the user types a valid numeric amount and confirms (Enter or blur), **Then** the change is persisted and the field returns to formatted read-only display.
5. **Given** the price input is active, **When** the user enters a non-numeric or negative value, **Then** the input is rejected and the user is shown an inline error; the previous value is not overwritten.
6. **Given** the price input is active, **When** the user clears the input and confirms, **Then** the desired price is removed ("Not set" is displayed).
7. **Given** the price input is active, **When** the user presses Escape, **Then** no change is made.

---

### User Story 4 - Edit Added Date via Calendar (Priority: P2)

A collector wants to correct the date on which an item was added to their wishlist. They click the "Added" date field and a date-picker calendar appears. Only dates in the past are selectable. Upon choosing a date, the field updates.

**Why this priority**: Date editing requires a date-picker component with past-date validation, making it a distinct slice.

**Independent Test**: Can be fully tested by clicking the Added date field, selecting a past date from the calendar, confirming, and verifying the displayed date updates correctly.

**Acceptance Scenarios**:

1. **Given** the sidebar is displayed, **When** the user hovers over the Added date value, **Then** the visual affordance appears.
2. **Given** the hover state, **When** the user clicks the date value, **Then** a calendar date-picker appears with the current date pre-selected.
3. **Given** the calendar is open, **Then** dates after today are disabled and unselectable.
4. **Given** the calendar is open, **When** the user selects a past date, **Then** the change is persisted and the calendar closes.
5. **Given** the calendar is open, **When** the user presses Escape or clicks outside, **Then** no change is made and the field reverts to read-only display.

---

### User Story 5 - List Field Remains Read-Only (Priority: P3)

The "List" field (wishlist name) is intentionally non-editable from the item sidebar. No hover affordance or click activation is shown for this field.

**Why this priority**: This is a constraint, not a user action. It must be specified to avoid scope creep and clarify what does NOT receive the edit pattern.

**Independent Test**: Verified by confirming no hover state, pencil icon, or click activation occurs on the List field.

**Acceptance Scenarios**:

1. **Given** the sidebar is displayed, **When** the user hovers over the List (wishlist name) field, **Then** no visual affordance or edit cue is shown.
2. **Given** the sidebar is displayed, **When** the user clicks the List field value, **Then** nothing happens and no input is activated.

---

### Edge Cases

- What happens when a save request fails (e.g., network or persistence error)? The field reverts to its previous value and a non-blocking error notification is shown.
- What happens if two editable fields are activated simultaneously? Activating a second field must close the first (cancelling any uncommitted edit) before the new field activates.
- What happens when the desired price amount is too large to store? The input rejects the value with an inline validation error.
- What happens if the application settings have no default currency configured? The price input falls back to a sensible default (e.g., EUR).

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: The List field in the wishlist item sidebar MUST remain read-only with no hover affordance or activation behaviour.
- **FR-002**: The Priority field MUST display a hover visual affordance (background highlight or pencil icon) when the pointer is over it.
- **FR-003**: Clicking the Priority value MUST replace the read-only badge with a dropdown containing all valid priority values (LOW, NORMAL, HIGH); the current value MUST be pre-selected and the dropdown MUST receive focus automatically.
- **FR-004**: Selecting a new priority from the dropdown MUST persist the change and return the field to read-only badge display.
- **FR-005**: The Status field MUST display the same hover affordance pattern as Priority.
- **FR-006**: Clicking the Status value MUST replace the badge with a dropdown containing all valid status values (WANTED, ON_ORDER, PURCHASED, IGNORED); the current value MUST be pre-selected and auto-focused.
- **FR-007**: Selecting a new status MUST persist the change and return the field to read-only display.
- **FR-008**: The Desired Price field MUST display the hover affordance pattern.
- **FR-009**: Clicking the Desired Price value (including the "Not set" placeholder) MUST replace it with a numeric input box; existing amounts MUST be pre-selected; the input MUST auto-focus; the applicable currency symbol/code from the application settings MUST be visible.
- **FR-010**: Confirming a valid non-negative numeric value MUST persist the change and return the field to formatted read-only display.
- **FR-011**: Clearing the price input and confirming MUST remove the desired price value, reverting to the "Not set" placeholder.
- **FR-012**: A non-numeric or negative price value MUST be rejected with an inline validation message; no previously persisted value MUST be overwritten.
- **FR-013**: The Added date field MUST display the hover affordance pattern.
- **FR-014**: Clicking the Added date MUST open a calendar date-picker with the currently stored date pre-selected.
- **FR-015**: The calendar MUST disable all dates after today so that only past and current dates are selectable.
- **FR-016**: Selecting a date from the calendar MUST persist the change and close the picker.
- **FR-017**: Pressing Escape or clicking outside any active inline editor MUST cancel the edit and restore the previous read-only display without persisting any change.
- **FR-018**: Only one field in the sidebar MUST be in edit mode at any given time; activating a second field MUST close the first.
- **FR-019**: If a save operation fails, the field MUST revert to its previous value and the user MUST receive a non-blocking error notification.

### Key Entities

- **WishlistItem**: The aggregate being modified; key editable attributes are `priority` (enum), `status` (enum), `desiredPrice` (nullable monetary amount + currency), and `addedDate` (ISO date string, past dates only).
- **WishlistPriority**: Enumeration with values LOW, NORMAL, HIGH.
- **WishlistStatus**: Enumeration with values WANTED, ON_ORDER, PURCHASED, IGNORED.
- **ApplicationSettings**: Provides the default currency code used as the denomination for the desired price input.

## Assumptions

- The application has an existing settings mechanism that exposes a default currency code; if none is configured, EUR is used as the fallback.
- The `addedDate` field is always present (non-nullable) on a wishlist item; today is the maximum selectable date.
- Persistence is handled via existing Tauri commands; no new backend routes need to be introduced unless the current update command does not support partial field updates.
- The hover affordance is a subtle translucent background highlight on the value element, consistent with the existing dark-mode UI aesthetic.

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: A user can update Priority, Status, Desired Price, or Added date from the sidebar in under 5 seconds per field.
- **SC-002**: 100% of edit interactions on the List field produce no edit state (read-only constraint is never violated).
- **SC-003**: Invalid price entries (non-numeric, negative) are rejected inline before reaching persistence in 100% of cases.
- **SC-004**: The calendar date-picker prevents selection of future dates in 100% of interactions.
- **SC-005**: Pressing Escape from any active inline editor results in zero data changes persisted.
- **SC-006**: No more than one field is in edit mode simultaneously at any point during a user session.
- **SC-007**: A failed save operation is surfaced to the user as a notification and the displayed value rolls back to the last known good state without a page reload.
