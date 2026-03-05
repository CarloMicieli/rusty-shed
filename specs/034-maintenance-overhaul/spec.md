# Feature Specification: Maintenance Page Overhaul

**Feature Branch**: `034-maintenance-overhaul`
**Created**: 2026-03-05
**Status**: Draft

## User Scenarios & Testing _(mandatory)_

### User Story 1 - Fix Duplicate Maintenance Card Prevention (Priority: P1)

A user who owns rolling stock wants to create a maintenance card, but the system currently allows multiple cards to be created for the same locomotive, causing data corruption and confusing history. The system must enforce a strict one-card-per-locomotive rule.

**Why this priority**: Data integrity is foundational. Duplicate cards make maintenance history unreliable and prevent the rest of the feature from working correctly. All other stories depend on a clean 1:1 relationship.

**Independent Test**: Can be fully tested by attempting to create a second maintenance card for the same owned rolling stock item and verifying the action is blocked with a clear message, even without any UI changes.

**Acceptance Scenarios**:

1. **Given** a rolling stock item already has a maintenance card, **When** a user attempts to create a new card for the same item, **Then** the system blocks the action and displays a clear error message indicating a card already exists.
2. **Given** the add-card form is open, **When** the user selects a rolling stock item that already has a card, **Then** the form either disables the submit action or removes that item from the selectable list.
3. **Given** two cards already exist for the same rolling stock (legacy data), **When** the user views the maintenance list, **Then** only one card is shown and the duplicate does not cause a crash.

---

### User Story 2 - Human-Readable Card Identification (Priority: P2)

A user browsing the maintenance card grid currently sees opaque GUID strings instead of model identity information, making it impossible to know which locomotive corresponds to which card at a glance.

**Why this priority**: Without identifiable cards, users cannot act on the maintenance list. This is the primary usability fix for the grid view and directly affects user confidence.

**Independent Test**: Can be fully tested by viewing the maintenance card grid and confirming each card displays Manufacturer, Product Code, Series Code, and Road Number in place of any GUID strings.

**Acceptance Scenarios**:

1. **Given** a maintenance card linked to an owned rolling stock item, **When** the card is displayed in the grid, **Then** it shows "{Manufacturer} {Product Code}" as the primary title in bold amber, the Series Code as a muted gray uppercase secondary label, and the Road Number in a pill badge in the top-right corner.
2. **Given** a rolling stock item with a Road Number, **When** the card renders, **Then** the Road Number badge uses a monospaced font to maintain the "mechanical precision" aesthetic.
3. **Given** a rolling stock item missing optional fields (e.g., no Series Code), **When** the card renders, **Then** only populated fields are shown; no blank or "N/A" labels appear.

---

### User Story 3 - Functional "Add Maintenance Event" in Detail View (Priority: P2)

A user viewing a specific locomotive's maintenance detail page needs to log a new maintenance event (e.g., "Motor Lubrication"). The current "Add Event" button on the main page is context-free and broken. The button must live inside the detail view and automatically associate the event with the correct card.

**Why this priority**: Logging events is the core action of the maintenance module. Without a working Add Event flow, the entire feature delivers no value beyond read-only display.

**Independent Test**: Can be fully tested by navigating to a single maintenance card's detail view, clicking "Add Event", filling in the form, saving, and verifying the new event appears in the timeline immediately without a page reload.

**Acceptance Scenarios**:

1. **Given** a user is on a maintenance card detail page, **When** they click "Add Event", **Then** a modal opens with a Date Performed field (defaulting to today's date), a Maintenance Type dropdown/input, and a Notes text area.
2. **Given** the Add Event modal is open, **When** the user submits a valid event, **Then** the modal closes and the new event appears at the top of the event timeline immediately without a full page refresh.
3. **Given** the Add Event modal is open, **When** the user submits with no Date Performed, **Then** the form shows an inline validation error and does not close.
4. **Given** a card has no prior events, **When** a first event is saved successfully, **Then** the empty-state placeholder ("No events logged yet") is replaced by the new event in the timeline.

---

### User Story 4 - Navigation: Active Sidebar State & Breadcrumb (Priority: P3)

When a user navigates from the maintenance grid into a card's detail view, the sidebar navigation loses its active state on the "Maintenance" item, making the user feel lost. A "Back" button is also needed to return to the grid.

**Why this priority**: Navigation correctness improves orientation within the app but does not block core maintenance workflows. It is a polish fix that can be developed independently.

**Independent Test**: Can be fully tested by navigating to any maintenance card detail view and confirming (a) the "Maintenance" sidebar item shows the active highlight, and (b) clicking the "Back" button returns to the maintenance grid.

**Acceptance Scenarios**:

1. **Given** a user navigates to a maintenance card detail page, **When** the page loads, **Then** the "Maintenance" sidebar navigation item displays with a 15% opacity amber background and a 2px left amber border.
2. **Given** a user is on a maintenance card detail page, **When** they click the "Back" button in the top-left of the view, **Then** they are returned to the main maintenance grid view.
3. **Given** a user uses in-app back navigation, **When** they return to the grid, **Then** the sidebar reverts to the standard active state for the maintenance section.

---

### Edge Cases

- What happens when a rolling stock record is deleted while a maintenance card exists for it? The existing cascade delete constraint handles removal; the card grid must gracefully reflect the absence without crashing.
- What happens if the maintenance event form is submitted while the backend is unavailable? The form must remain open and show an error message without data loss.
- What happens when a maintenance card has a very large number of events? The timeline must scroll within the detail view without breaking the page layout.
- How does the system display cards when the linked rolling stock item has no Manufacturer or Product Code set? Show available fields only; never display raw IDs or GUIDs.

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: System MUST prevent creation of more than one maintenance card per owned rolling stock item, rejecting duplicate creation attempts with a user-facing error message.
- **FR-002**: Each maintenance card in the grid MUST display: primary title as "{Manufacturer} {Product Code}" in bold amber, secondary label as "{Series Code}" in muted gray uppercase, and Road Number in a top-right pill badge using a monospaced font.
- **FR-003**: Optional rolling stock fields (Series Code, Road Number) MUST only appear on the card if they are populated; no empty placeholders or "N/A" labels.
- **FR-004**: The "Add Event" action MUST be accessible exclusively from within a maintenance card's detail view, not from the main grid page header.
- **FR-005**: When the "Add Event" action is triggered inside a detail view, the resulting event MUST be automatically associated with the `maintenance_card_id` of that specific page—no manual card selection is required from the user.
- **FR-006**: The Add Event modal MUST collect: Date Performed (defaults to current date), Maintenance Type (predefined options with free-text fallback), and Notes (multi-line text area).
- **FR-007**: Upon successful event submission, the new event MUST appear in the detail view's event timeline immediately, without a page reload.
- **FR-008**: The "Maintenance" sidebar navigation item MUST display in an active visual state (amber-tinted background, left border highlight) whenever the user is on any maintenance route, including card detail sub-pages.
- **FR-009**: The maintenance card detail view MUST include a "Back" navigation element in the top-left area that returns the user to the maintenance grid.
- **FR-010**: When a maintenance card has no logged events, the detail view event log area MUST display an empty-state with a centered monochromatic wrench icon and the text "No events logged yet."
- **FR-011**: Internal dividers and event cards in the timeline MUST use a 1px solid dark border consistent with the application design system.

### Key Entities

- **MaintenanceCard**: Represents a service record for one specific owned rolling stock item. Enforces a 1:1 relationship with `OwnedRollingStock`. Contains `last_maintenance_date`, `next_maintenance_date`, and a collection of events.
- **MaintenanceEvent**: A single logged maintenance action. Belongs to one `MaintenanceCard`. Fields: `date_performed`, `maintenance_type`, `notes`.
- **OwnedRollingStock**: The physical model being tracked. Provides display identity: Manufacturer, Product Code, Series Code, and Road Number.

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: Users can identify any maintenance card by locomotive name within 2 seconds of viewing the grid, without needing to open the detail view.
- **SC-002**: Zero duplicate maintenance cards can be created for the same rolling stock item through any user-facing action.
- **SC-003**: Users can log a new maintenance event and see it appear in the timeline within 5 seconds of clicking "Add Event" on a reliable local connection.
- **SC-004**: 100% of maintenance routes (grid and detail) keep the sidebar "Maintenance" item visually active when navigated to.
- **SC-005**: Users can return from any maintenance detail view to the grid in a single action (one "Back" button click).
- **SC-006**: The empty-state placeholder appears correctly for newly created cards with no events, verified by direct observation.

## Assumptions

- The existing `owned_rolling_stocks` table stores Manufacturer, Product Code, Series Code, and Road Number as structured fields accessible to the maintenance card query.
- The existing backend does not yet enforce the 1:1 uniqueness constraint at the data layer; a unique index or application-level guard must be added.
- A dedicated route for individual maintenance card detail pages does not yet exist and will need to be created.
- The "Add Event" button currently in the main page header is the broken entry point; it will be removed from the header and re-implemented exclusively in the detail view.
- "Maintenance Type" will use a predefined set of options consistent with existing Paraglide message keys (e.g., Motor Lubrication, Wheel Cleaning, Track Cleaning, Coupler Adjustment, General Inspection, Other) with a free-text fallback for unlisted types.
