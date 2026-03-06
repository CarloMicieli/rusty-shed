# Feature Specification: The Signal Box — Error Management System

**Feature Branch**: `036-signal-box`
**Created**: 2026-03-06
**Status**: Draft
**Input**: User description: "The Signal Box Error Management System"

## Overview

Transform system error experiences in the Rusty Shed application from generic failure states into thematically consistent "Signal Failures." This feature introduces unique traceable Error IDs, a full-page Signal Failure view styled within the railway collector aesthetic, and Amber-bordered toast notifications for non-fatal faults — ensuring errors never break immersion.

## User Scenarios & Testing _(mandatory)_

### User Story 1 - Fatal Error Signal Failure View (Priority: P1)

A user is navigating the application when an unhandled system fault occurs (e.g., the inventory module crashes during data load). Instead of a blank or generic error page, they see a themed "Signal Failure" screen — with a railway signal icon, a unique Error ID, and clear actions to recover or report the issue.

**Why this priority**: This is the most disruptive failure mode. A complete, visually cohesive error state restores user trust and provides actionable recovery options. It forms the foundation for all other error handling improvements.

**Independent Test**: Can be fully tested by triggering an unhandled exception in any module and verifying the Signal Failure view renders with a unique Error ID, the correct themed copy, and working action buttons — before any other story is implemented.

**Acceptance Scenarios**:

1. **Given** the application is running normally, **When** an unhandled system fault occurs in any module, **Then** the Signal Failure view replaces the current view with the themed headline, icon, and subtext visible.
2. **Given** the Signal Failure view is displayed, **When** the user reads the technical metadata footer, **Then** three columns are visible: Error Code (unique ID in monospace), Module (name of the section where the fault occurred), and Status (CRITICAL).
3. **Given** the Signal Failure view is displayed, **When** the user clicks "Reset Signal," **Then** the application reloads the affected module and returns to its last stable state.
4. **Given** the Signal Failure view is displayed, **When** the user clicks "Report to Depot," **Then** the Error ID and relevant fault context are copied to the clipboard.
5. **Given** the Signal Failure view is displayed, **When** the view renders, **Then** the background remains the base dark color without any white flash during transition.

---

### User Story 2 - Non-Fatal Error Toast Notification (Priority: P2)

A user is viewing the maintenance schedule when a background sync operation fails (e.g., DCC configuration could not be retrieved). Instead of a full-page takeover, a brief Amber-bordered toast notification appears at a corner of the screen, identifies the issue in domain language, and auto-dismisses.

**Why this priority**: Non-fatal errors are frequent and should not interrupt the user's primary workflow. A toast pattern delivers awareness without losing context.

**Independent Test**: Can be fully tested by triggering a non-fatal background fault (e.g., a failed sync), verifying the toast appears with Amber styling and domain-language copy, and confirming it auto-dismisses without navigating away.

**Acceptance Scenarios**:

1. **Given** the user is on any screen, **When** a non-fatal background operation fails, **Then** an Amber-bordered toast notification appears without replacing the current view.
2. **Given** the toast is displayed, **When** the user reads its content, **Then** the fault is described using domain language (no terms like "server error" or "HTTP 500").
3. **Given** the toast is displayed, **When** sufficient time passes (auto-dismiss), **Then** the toast disappears without user interaction.
4. **Given** the toast is displayed, **When** the user clicks to dismiss it manually, **Then** the toast closes immediately.

---

### User Story 3 - Traceable Error Identification (Priority: P3)

A user encounters a Signal Failure and contacts support. They can provide the short Error ID (e.g., `ERR-8821-X`) displayed on the error view, which allows the development team to locate the corresponding full fault log entry in the backend.

**Why this priority**: Traceability requires both the frontend display and the backend logging to be wired together. It adds significant support value but depends on P1 being complete.

**Independent Test**: Can be fully tested by triggering a fault, capturing the displayed Error ID, and verifying that a corresponding log entry exists in the backend with the same ID and full fault context.

**Acceptance Scenarios**:

1. **Given** a fault has occurred and the Signal Failure view is showing, **When** the user reads the Error Code column, **Then** a unique short ID (format: `ERR-NNNN-X`) is displayed in monospace.
2. **Given** the Error ID is displayed in the UI, **When** the backend logs are examined, **Then** an entry with that exact ID exists, containing the full fault details and context.
3. **Given** two separate faults occur, **When** their Error IDs are compared, **Then** each ID is unique — no duplicates are generated.

---

### Edge Cases

- What happens when a fault occurs during the initial application startup before any module is loaded? (Module column should display "Startup" or equivalent neutral label.)
- What happens when the Error ID generation itself fails? (A fallback static ID such as `ERR-0000-F` must still be displayed.)
- What happens when two non-fatal faults occur simultaneously? (Both toasts should queue without overlapping or losing either notification.)
- What happens when the user triggers "Reset Signal" but the module fails to reload? (The Signal Failure view should remain; do not enter an infinite reload loop.)
- What happens when "Report to Depot" is clicked but clipboard access is unavailable? (A visible fallback should allow the user to read and manually copy the Error ID.)

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: System MUST generate a unique, short Error ID (format: `ERR-NNNN-X`) for every unhandled fault at the moment it occurs.
- **FR-002**: System MUST display the Signal Failure view — replacing the current view — whenever an unhandled fault is caught, regardless of which module triggered it.
- **FR-003**: The Signal Failure view MUST display: a railway-themed monochromatic icon, the headline "Signal Failure" in Amber, the subtext "The yard master encountered an unexpected obstruction," and a three-column technical metadata footer.
- **FR-004**: The three-column metadata footer MUST display: Error Code (the unique ID, monospace), Module (the name of the active sidebar section at time of fault), and Status (fixed value: CRITICAL).
- **FR-005**: The Signal Failure view MUST provide a primary "Reset Signal" action that reloads the affected module and a secondary "Report to Depot" action that copies the Error ID and fault context to the clipboard.
- **FR-006**: System MUST display an Amber-bordered toast notification — without full-page takeover — for all non-fatal background faults (e.g., failed syncs, non-critical data load failures).
- **FR-007**: Toast notifications MUST use domain language only; terms such as "server," "HTTP," "404," or "internal error" are prohibited.
- **FR-008**: Toast notifications MUST auto-dismiss after a reasonable display duration and MUST be manually dismissible by the user.
- **FR-009**: The backend MUST log a structured entry for every fault, keyed by the Error ID, containing: timestamp, fault description, module context, and full fault trace.
- **FR-010**: The Signal Failure view background MUST remain the base dark color (`#050505`) during render and transition — no white flash.
- **FR-011**: All visual containers in the Signal Failure view MUST use 1px borders in `#1F1F1F` with 8px rounded corners.
- **FR-012**: Color usage MUST be strictly limited to: `#050505` (base), `#0F0F0F` (surface), `#D48A42` (Amber accent), and `#808080` (muted elements including the icon).

### Key Entities

- **Signal Fault**: Represents a captured application failure. Key attributes: unique Error ID, severity (fatal / non-fatal), module name, timestamp, fault description, full trace.
- **Error ID**: A short unique identifier (format `ERR-NNNN-X`) generated at fault time. Acts as the join key between the UI display and the backend log entry.
- **Toast Notification**: A transient, non-blocking message for non-fatal faults. Attributes: message text (domain language), severity, display duration, dismissible flag.

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: Every unhandled fatal fault results in the Signal Failure view being displayed within 500ms of the fault being caught — no blank screens or generic browser error states.
- **SC-002**: Every fault generates a unique Error ID; zero duplicate IDs occur across any session.
- **SC-003**: 100% of non-fatal background faults surface as toast notifications; zero non-fatal faults result in a full-page Signal Failure view.
- **SC-004**: The Signal Failure view renders with the correct `#050505` background on first paint — measured by visual regression tests showing zero white-flash frames.
- **SC-005**: Every Error ID displayed in the UI has a corresponding backend log entry containing the full fault trace — verified by cross-referencing ID in UI against log output.
- **SC-006**: Users can copy the Error ID to clipboard via the "Report to Depot" action in a single interaction (one click/tap), with a visible confirmation.
- **SC-007**: No domain-prohibited terms ("server," "404," "internal error," "HTTP") appear in any user-facing error text — verified by content audit.

## Assumptions

- The application already has a concept of "sidebar sections" or "modules" (e.g., Collection, Wishlist, Maintenance) that can be identified at runtime; the active module name will be used to populate the Module column.
- Non-fatal vs. fatal fault classification will be determined at the point errors are thrown or returned — fatal faults are unrecoverable in-context, non-fatal faults are background operations that do not block the current user view.
- "Reset Signal" will perform a component-level reload (not a full application restart) where possible, falling back to a full restart only if necessary.
- Clipboard access is available in the desktop runtime environment; a graceful fallback (visible selectable text) will be provided if access is denied.
- The Error ID format `ERR-NNNN-X` uses a 4-digit random numeric segment and a single random uppercase letter suffix to provide sufficient uniqueness for session-scoped tracing.
