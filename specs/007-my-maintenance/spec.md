# Feature Specification: My Maintenance Page

**Feature Branch**: `007-my-maintenance`  
**Created**: January 30, 2026  
**Status**: Draft  
**Input**: User description: "Add My Maintenance page to track rolling stock maintenance"

## User Scenarios & Testing _(mandatory)_

<!--
  IMPORTANT: User stories should be PRIORITIZED as user journeys ordered by importance.
  Each user story/journey must be INDEPENDENTLY TESTABLE - meaning if you implement just ONE of them,
  you should still have a viable MVP (Minimum Viable Product) that delivers value.

  Assign priorities (P1, P2, P3, etc.) to each story, where P1 is the most critical.
  Think of each story as a standalone slice of functionality that can be:
  - Developed independently
  - Tested independently
  - Deployed independently
  - Demonstrated to users independently
-->

### User Story 1 - View Urgent Maintenance Overview (Priority: P1)

As a model railway collector, I want to see a prioritized list of my rolling stock that requires upcoming maintenance so that I can keep my collection in good running condition and never miss critical maintenance deadlines.

**Why this priority**: This is the core value proposition of the feature. Without the ability to view maintenance status, the entire page has no purpose. Users need visibility into their maintenance backlog before they can take any action.

**Independent Test**: Can be fully tested by navigating to the My Maintenance page and viewing the top 10 maintenance cards ordered by due date. Delivers immediate value by showing collectors which items need attention first.

**Acceptance Scenarios**:

1. **Given** a user has rolling stock with maintenance cards, **When** they navigate to the My Maintenance page, **Then** they see up to 10 maintenance cards sorted by due date (most urgent first)
2. **Given** a maintenance card is overdue (due date in the past), **When** the page displays this card, **Then** it is visually highlighted with urgent/critical styling
3. **Given** a maintenance card is due within 7 days, **When** the page displays this card, **Then** it is visually highlighted with warning styling
4. **Given** a maintenance card is due beyond 7 days, **When** the page displays this card, **Then** it is displayed with normal styling
5. **Given** a user views a maintenance card, **When** the card is displayed, **Then** it shows the manufacturer, product code, series code/road number, maintenance type due, and due date

---

### User Story 2 - Create Maintenance Card (Priority: P2)

As a model railway collector, I want to create a new maintenance card for a rolling stock item so that I can start tracking its maintenance schedule and history.

**Why this priority**: Users need to be able to add new items to the maintenance tracking system. Without this, the system would only work for pre-existing data.

**Independent Test**: Can be fully tested by clicking the "Add Maintenance Card" button, selecting a rolling stock item, and successfully creating a new maintenance card. Delivers value by enabling users to track maintenance for newly acquired items.

**Acceptance Scenarios**:

1. **Given** a user is on the My Maintenance page, **When** they click the "Add Maintenance Card" quick action button, **Then** they are presented with a form/modal to create a new maintenance card
2. **Given** a user is creating a maintenance card, **When** they select a rolling stock item from their collection, **Then** the item's details (manufacturer, product code, series code) are pre-populated
3. **Given** a user completes the maintenance card form with valid data, **When** they submit the form, **Then** the maintenance card is created and appears in their maintenance list
4. **Given** a user attempts to create a maintenance card for rolling stock that already has one, **When** they search for items, **Then** already-tracked items are clearly indicated

---

### User Story 3 - Add Maintenance Event (Priority: P3)

As a model railway collector, I want to quickly log a maintenance event for a rolling stock item so that I can keep an accurate maintenance history and update the next due date.

**Why this priority**: This is essential for ongoing maintenance tracking. Once cards exist, users need to record when maintenance was performed to keep the system accurate.

**Independent Test**: Can be fully tested by clicking the "Add Maintenance Event" button, selecting a maintenance card, entering event details, and confirming the event is recorded. Delivers value by allowing users to track maintenance history and automatically calculate next due dates.

**Acceptance Scenarios**:

1. **Given** a user is on the My Maintenance page, **When** they click the "Add Maintenance Event" quick action button, **Then** they are presented with a form/modal to log a maintenance event
2. **Given** a user is adding a maintenance event, **When** they select a maintenance card, **Then** they can specify what maintenance was performed and when
3. **Given** a user submits a maintenance event, **When** the event is recorded, **Then** the maintenance card's due date is recalculated based on the maintenance interval
4. **Given** a user logs a maintenance event, **When** the event is saved, **Then** it is recorded in the maintenance history for that rolling stock item

---

### Edge Cases

- What happens when a user has no rolling stock in their collection? The page displays an empty state with guidance on adding items to their collection first.
- What happens when a user has rolling stock but no maintenance cards? The page displays an empty state with a prominent call-to-action to create their first maintenance card.
- What happens when all maintenance is up-to-date (no items due soon)? The page displays a success message indicating all maintenance is current, with the list still visible.
- How does the system handle rolling stock that is sold or removed from collection? Maintenance cards for removed items should be archived or hidden from the active list.
- What happens when a maintenance event date is in the future? The system should prevent logging future maintenance events (maintenance cannot be done in advance).

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: System MUST display up to 10 maintenance cards ordered by due date (most urgent first)
- **FR-002**: System MUST visually distinguish urgency levels: overdue (critical), due within 7 days (warning), and due later (normal)
- **FR-003**: System MUST display for each maintenance card: manufacturer name, product code, series code/road number, maintenance type, and due date
- **FR-004**: System MUST provide a quick action button to create a new maintenance card
- **FR-005**: System MUST provide a quick action button to add a maintenance event
- **FR-006**: System MUST allow users to select rolling stock from their collection when creating a maintenance card
- **FR-007**: System MUST persist all maintenance cards and events
- **FR-008**: System MUST recalculate the next due date when a maintenance event is logged
- **FR-009**: System MUST display an appropriate empty state when no maintenance cards exist
- **FR-010**: System MUST allow users to view maintenance cards only for rolling stock they own

### Key Entities _(include if feature involves data)_

- **Maintenance Card**: Represents a maintenance tracking record for a specific rolling stock item. Contains reference to rolling stock, maintenance type/schedule, current due date, and creation date. Links to rolling stock entity.
- **Maintenance Event**: Represents a single maintenance activity performed on a rolling stock item. Contains reference to maintenance card, date performed, notes/description of work done, and who performed it.
- **Rolling Stock**: Pre-existing entity representing a model railway item in the user's collection. Provides manufacturer, product code, and series code/road number information.

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: Users can view their most urgent maintenance items within 2 seconds of navigating to the page
- **SC-002**: Users can identify overdue maintenance at a glance without reading detailed text
- **SC-003**: Users can create a new maintenance card in under 30 seconds
- **SC-004**: Users can log a maintenance event in under 20 seconds
- **SC-005**: Urgency indicators (overdue, warning, normal) are visually distinct with color contrast meeting WCAG AA standards (4.5:1 ratio)
- **SC-006**: Overdue cards display a distinct visual treatment (red/critical styling) that is verifiable by visual regression or accessibility testing

## Assumptions

- Rolling stock entities already exist in the system with manufacturer, product code, and series code/road number data
- Users have already added rolling stock items to their collection before using the maintenance feature
- The maintenance interval (frequency) will be defined per maintenance card, allowing different items to have different maintenance schedules
- Date calculations for urgency are based on the current system date
- All users of the app have a single collection (no multi-user considerations for this feature)
