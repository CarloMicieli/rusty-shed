# Feature Specification: On-the-Fly Entity Quick-Add

**Feature Branch**: `040-quick-add-entities`  
**Created**: 2026-05-15  
**Status**: Draft  
**Input**: User description: "On-the-fly entity creation for manufacturers, sellers and buyers in acquisition, collection and wishlist forms"

## Dependencies

- **Implementation ownership**: Feature 040 is self-contained for the Quick-Add scope described in this specification. It owns the required backend create commands, supporting database uniqueness migrations, shared quick-add UI components, and frontend wiring needed to deliver the flow end-to-end.

## Overview

Users filling in the Acquisition, Collection, or Wishlist forms currently must leave the form and navigate to Settings to add a new manufacturer, seller, or buyer — losing all their in-progress work. This feature introduces a contextual "Quick-Add" drawer that slides open alongside the primary form, allows the user to register a new entity with minimal required fields, and then automatically selects that entity back in the parent form — all without discarding any already-entered data.

## Clarifications

### Session 2026-05-15

- Q: What should the backend return after a successful save — just the new ID, or the full entity object? → A: The backend must return the full entity object (at minimum ID and Name) so the frontend can push it directly into the dropdown list without re-fetching the entire list.
- Q: Should the Quick-Add drawer support editing or deleting an entity after creation? → A: No. The Quick-Add drawer is strictly add-only. Edit and delete actions remain exclusively in the Settings page to prevent accidental data loss during an active form session.
- Q: How should the duplicate check in FR-004 handle case differences (e.g., "Hornby" vs "hornby")? → A: Case-insensitively. The check must compare using a case-insensitive method (e.g., `WHERE LOWER(name) = LOWER(?)`) to prevent logically duplicate entries that differ only in casing.

## User Scenarios & Testing _(mandatory)_

### User Story 1 - Add Manufacturer On-the-Fly During Acquisition (Priority: P1)

A collector is filling in a new acquisition form. The manufacturer they need is not yet in the system. Without leaving the form or losing any data they have already entered, they open a secondary "Quick-Add" drawer, create the manufacturer with just its name, and that manufacturer is immediately selected in the acquisition form.

**Why this priority**: Manufacturers are required on virtually every acquisition record. Blocking users on a missing manufacturer is the highest-friction pain point; resolving it first delivers the most immediate value.

**Independent Test**: Can be fully tested by opening the New Acquisition form, clicking `+` next to the Manufacturer dropdown, saving a new manufacturer name, and verifying it is selected in the parent form — without any other quick-add entities implemented.

**Acceptance Scenarios**:

1. **Given** the Acquisition form is open with some fields already filled, **When** the user clicks `+` next to the Manufacturer dropdown, **Then** a Quick-Add drawer opens and the Acquisition form remains visible but dimmed in the background with all previously entered data intact.
2. **Given** the Quick-Add drawer is open, **When** the user types a name that already exists in the database, **Then** a real-time warning is shown and the Save button is disabled.
3. **Given** the Quick-Add drawer is open with a unique, non-empty name, **When** the user clicks Save, **Then** the new manufacturer is persisted, the drawer closes, and the manufacturer is automatically selected in the Acquisition form's Manufacturer dropdown.
4. **Given** the Quick-Add drawer is open, **When** the user dismisses or cancels it, **Then** the Acquisition form is restored to full focus with no data lost and no entity created.
5. **Given** a new manufacturer was just saved, **When** the success state is shown, **Then** a confirmation notification appears stating the manufacturer name was added and selected.

---

### User Story 2 - Add Seller or Buyer On-the-Fly During Acquisition (Priority: P1)

A collector entering an acquisition needs to record a seller or buyer that is not yet in the system. They use the same Quick-Add mechanism adjacent to the Seller and Buyer dropdowns to create the missing entity and have it selected without interrupting the main form flow.

**Why this priority**: Sellers and buyers are equally core to the acquisition record; the same user pain applies.

**Independent Test**: Can be fully tested by opening the New Acquisition form, clicking `+` next to either the Seller or Buyer dropdown, completing the quick-add flow, and verifying the entity is selected in the parent form.

**Acceptance Scenarios**:

1. **Given** the Acquisition form is open, **When** the user clicks `+` next to the Seller dropdown, **Then** a Quick-Add drawer for sellers opens with fields for Name (required), Website (optional), and Country (optional).
2. **Given** the Acquisition form is open, **When** the user clicks `+` next to the Buyer dropdown, **Then** a Quick-Add drawer for buyers opens with the same minimal field set.
3. **Given** a Quick-Add seller/buyer drawer is open with valid, unique data, **When** the user saves, **Then** the entity is persisted, the drawer closes, and the new entity is selected in the corresponding dropdown.

---

### User Story 3 - Add Entities On-the-Fly from Collection and Wishlist Forms (Priority: P2)

A collector adding a new item to their collection or wishlist encounters a missing manufacturer. The same Quick-Add experience available in the Acquisition form is equally accessible from these two forms.

**Why this priority**: Collection and wishlist forms have the same underlying need but lower daily frequency; they benefit from the same mechanism once it exists.

**Independent Test**: Can be tested independently by opening the New Collection Item or New Wishlist Item form, triggering the Quick-Add drawer, saving a manufacturer, and confirming auto-selection.

**Acceptance Scenarios**:

1. **Given** the New Collection Item form is open, **When** the user clicks `+` next to the Manufacturer dropdown, **Then** the same Quick-Add drawer opens and behaves identically to the one in the Acquisition form.
2. **Given** the New Wishlist Item form is open, **When** the user clicks `+` next to the Manufacturer dropdown, **Then** the Quick-Add drawer opens with preserved state for the Wishlist form.
3. **Given** a Quick-Add entity is saved from either the Collection or Wishlist context, **Then** the newly created entity is auto-selected in the correct parent form field.

---

### User Story 4 - Mobile Quick-Add Experience (Priority: P3)

On a smaller screen (phone or tablet), a user filling in an acquisition form triggers the Quick-Add drawer. The experience adapts to a bottom-sheet that covers approximately 80% of the screen height and supports swipe-to-dismiss.

**Why this priority**: The app must be usable on mobile; however, the desktop flow is the primary entry point and must be delivered first.

**Independent Test**: Can be tested on a mobile viewport by triggering Quick-Add, verifying the bottom-sheet animation, completing the save flow, and verifying swipe-to-dismiss cancels without data loss.

**Acceptance Scenarios**:

1. **Given** the user is on a mobile viewport with the Acquisition form open, **When** they tap `+` next to a dropdown, **Then** a bottom-sheet slides up from the bottom covering roughly 80% of the screen.
2. **Given** the Quick-Add bottom-sheet is open, **When** the on-screen keyboard appears, **Then** the Save button remains visible above the keyboard.
3. **Given** the Quick-Add bottom-sheet is open, **When** the user swipes it downward, **Then** the sheet dismisses and the parent form remains unchanged.

---

### Edge Cases

- What happens when the database write fails (network error, constraint violation)? The Quick-Add drawer must remain open with the entered data and display an error message. The parent form must be unaffected.
- What happens if the user opens Quick-Add, navigates away (e.g., presses the hardware back button), and returns? The parent form state must be fully restored and no partial entity should have been persisted.
- What happens if the user types a name, the duplicate check passes, but by the time they click Save another user (or another session) has just saved the same name? The system must handle the conflict gracefully and display an appropriate error in the Quick-Add drawer.
- What happens when the entity name contains only whitespace? It must be treated as empty; the Save button must remain disabled.
- What happens if more than one Quick-Add drawer is open simultaneously (e.g., user tries to trigger it twice)? Only one secondary drawer may be open at a time; triggering a second one must have no effect or be prevented by the UI.

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: Each entity-selection dropdown in the Acquisition, Collection Item, and Wishlist Item forms MUST include a contextual action (e.g., a `+` icon) that opens a Quick-Add drawer for that entity type.
- **FR-002**: Opening the Quick-Add drawer MUST NOT reset, reload, or otherwise alter the data already entered in the parent form.
- **FR-003**: The Quick-Add form MUST require only the entity's **Name** field; Website and Country MUST be optional.
- **FR-004**: The Quick-Add form MUST perform a real-time uniqueness check on the Name field against existing records using a **case-insensitive** comparison (e.g., `WHERE LOWER(name) = LOWER(?)`) and display a warning if a duplicate is detected. This prevents logically duplicate entries that differ only in letter casing (e.g., "Hornby" and "hornby" must be treated as the same entity).
- **FR-005**: The Save action in the Quick-Add form MUST be disabled while the Name field is empty or contains only whitespace, or while a duplicate is detected.
- **FR-006**: Upon successful save, the system MUST return the **full entity object** (at minimum the generated identifier and the canonical Name as stored) so the frontend can push the new item directly into the dropdown's local state without re-fetching the entire list from the database.
- **FR-007**: Upon successful save, the Quick-Add drawer MUST close automatically and the newly created entity MUST be selected in the corresponding dropdown of the parent form.
- **FR-008**: Upon successful save, a non-blocking confirmation notification MUST be shown identifying the entity type and name that was added and selected.
- **FR-009**: If the save operation fails, the Quick-Add drawer MUST remain open with the user's data intact and display a clear error message.
- **FR-010**: Dismissing or cancelling the Quick-Add drawer MUST return full focus to the parent form with no data loss and no entity persisted.
- **FR-011**: The Quick-Add entity form MUST be implemented as a reusable shared component so the same form logic, validation rules, and field set can be used consistently across the Acquisition, Collection Item, and Wishlist Item flows without duplication.
- **FR-012**: On desktop, the parent form MUST be visually de-emphasised (reduced opacity) while the Quick-Add drawer is active, and MUST NOT accept input during that time.
- **FR-013**: On mobile viewports, the Quick-Add form MUST appear as a bottom sheet supporting swipe-to-dismiss, and the Save button MUST remain accessible when the on-screen keyboard is displayed.
- **FR-014**: Only one Quick-Add drawer MAY be open at any given time; attempting to open a second one while one is already active MUST be prevented.
- **FR-015**: The Quick-Add drawer MUST expose only the **create** action. Edit and delete controls MUST NOT be present in this contextual view; those operations remain exclusively in the Settings page.

### Key Entities

- **Manufacturer**: Represents a model railway manufacturer. Key attributes: Name (unique, required), Website (optional), Country (optional). Linked to collection items, acquisitions, and wishlist items.
- **Seller**: Represents a commercial counterparty involved in acquisition and purchase-related flows. Key attributes: Name (unique, required), Website (optional), Country (optional). Linked to acquisitions and other purchase-related records.
- **Buyer**: Represents the buyer role in an acquisition-related transaction. Buyers are not stored in a separate table; they reuse the Seller persistence model and are distinguished by UI context and field labeling.

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: Users can add a missing manufacturer, seller, or buyer and have it selected in the parent form in under 60 seconds from clicking the `+` trigger.
- **SC-002**: Zero data loss — returning to the parent form after Quick-Add (save or cancel) must always preserve 100% of previously entered data.
- **SC-003**: The duplicate-check warning appears within 500 milliseconds of the user finishing typing a name.
- **SC-004**: The Quick-Add form is implemented as a reusable shared component, and Acquisition, Collection, and Wishlist flows all use that shared implementation rather than duplicating create-form logic.
- **SC-005**: The Quick-Add flow works correctly on both desktop and mobile viewports with no layout breakage.
- **SC-006**: Internal peer review confirms that the `+` trigger is discoverable and the Quick-Add flow can be completed successfully in a simulated acquisition session without clarification from the implementer.

## Assumptions

- The existing Acquisition, Collection Item, and Wishlist Item forms already have Manufacturer, Seller, and Buyer dropdown fields implemented.
- Feature 040 establishes the reusable shared Quick-Add form component for this workflow; broader entity-management reuse outside these flows may be adopted later without changing the shared form contract introduced here.
- Entity names are treated as case-insensitively unique within their respective persisted tables; the uniqueness check uses `LOWER(name)` comparison at the database layer (see FR-004).
- A "mobile viewport" is defined as screen width below 768 px for the purpose of the bottom-sheet behaviour.
- Optional fields (Website, Country) can be left blank at creation time and populated later through the broader entity-management UI.
