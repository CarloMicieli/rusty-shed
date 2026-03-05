# Feature Specification: Rolling Stock Information Grid

**Feature Branch**: `033-rolling-stock-info-grid`
**Created**: 2026-03-05
**Status**: Draft
**Input**: User description: "Rolling Stock List component: include missing information in both single and multi-entry views, with a structured 5-row information grid and In-Place Editing system with immediate data persistence."

## User Scenarios & Testing _(mandatory)_

### User Story 1 — Complete Field Grid Display (Priority: P1)

A collector opens a Railway Model Card and views a rolling stock entry. Instead of a partial field list, they see a structured information grid showing all attributes in labelled rows: Series / Depot / Livery (row 1), Control Type / DCC Interface / Length (row 2), Flywheel Fitted / Body Shell / Chassis (row 3), Interior Lights / Lights / — (row 4), Coupling Socket / Close Couplers / Digital Shunting (row 5). Above the grid, the header shows Series Code and Road Number on the left and the Railway Company badge on the right.

**Why this priority**: Making all fields visible at once is the foundational requirement. Read-only users benefit immediately; every other interaction (editing, toggling) depends on the grid being present.

**Independent Test**: Open any rolling stock entry. Verify all 15 attribute fields appear in the correct 5-row, 3-column grid with proper labels. No editing interaction needed.

**Acceptance Scenarios**:

1. **Given** a rolling stock entry with all fields populated, **When** the user views the entry, **Then** all 15 fields appear in their designated row and column positions with correct labels and values.
2. **Given** a rolling stock entry where some fields are empty, **When** the user views the entry, **Then** empty fields display an inviting placeholder (e.g., italicised "Add Depot") in their grid cell rather than collapsing the layout.
3. **Given** any rolling stock entry, **When** the user views the entry, **Then** the header shows Series Code and Road Number left-aligned and Railway Company right-aligned.
4. **Given** a boolean field (Flywheel Fitted, Interior Lights, Lights, Close Couplers, Digital Shunting) with a True value, **When** viewed, **Then** a checked/active indicator is shown; when False or null, an unchecked/inactive indicator is shown.

---

### User Story 2 — Inline Editing for Text and Numeric Fields (Priority: P1)

A collector clicks the "—" placeholder on the Depot field; an input appears, they type the depot name and click away. The value is saved automatically. The same flow works for Length (numeric), Road Number, and Series Code.

**Why this priority**: Text and numeric field editing is the most common data enrichment task and requires no dropdown infrastructure.

**Independent Test**: Click the placeholder on Depot. Verify an input appears. Type a value, click away, verify it is persisted. Click an existing Length value, change it, press Enter, verify it is saved.

**Acceptance Scenarios**:

1. **Given** a text/numeric field in view mode, **When** the user clicks its value or placeholder, **Then** an input appears in place, pre-filled with the current value (empty if placeholder).
2. **Given** an active input, **When** the user blurs or presses Enter, **Then** the new value is persisted and the field returns to read-only display.
3. **Given** an active input, **When** the user presses Escape, **Then** the field reverts to its previous value without any persistence action.
4. **Given** a field is saving, **When** viewed, **Then** a visual indicator (spinner or colour pulse) confirms background activity.
5. **Given** a persistence failure, **When** the save attempt ends, **Then** an inline error indicator appears and the field retains the unsaved value for retry.

---

### User Story 3 — Inline Editing for Enumerated Fields (Priority: P1)

A collector clicks the Control Type value; a searchable dropdown appears with all valid options. They select "DCC Fitted" and it is saved immediately. The same pattern applies to DCC Interface, Coupling Socket, Body Shell, and Chassis.

**Why this priority**: Enumerated fields constrain data to valid categories — free-text typos would corrupt catalogue quality. Dropdown editing is both a UX and data-integrity requirement.

**Independent Test**: Click the Control Type field. Verify a searchable dropdown appears with valid options. Select a different option. Verify the change is persisted without an explicit Save button.

**Acceptance Scenarios**:

1. **Given** an enumerated field (Control Type, DCC Interface, Coupling Socket, Body Shell, Chassis), **When** the user clicks its value or placeholder, **Then** a searchable dropdown appears listing all valid options.
2. **Given** a dropdown is open, **When** the user selects an option, **Then** the selection is immediately persisted, the dropdown closes, and the new value is displayed.
3. **Given** a dropdown is open, **When** the user presses Escape or clicks outside, **Then** the dropdown closes without saving any change.
4. **Given** a searchable dropdown, **When** the user types in the search input, **Then** the option list filters to matching items in real time.

---

### User Story 4 — Inline Toggle for Boolean Fields (Priority: P2)

A collector clicks the Flywheel Fitted toggle; it changes from unchecked to checked and the change is saved automatically. The same interaction applies to Interior Lights, Lights, Close Couplers, and Digital Shunting.

**Why this priority**: Boolean fields have the simplest interaction model (single click), but they are less frequently updated than identification or control fields.

**Independent Test**: Find a rolling stock entry where Flywheel Fitted is False. Click the toggle. Verify it changes to True and the change is persisted without additional confirmation.

**Acceptance Scenarios**:

1. **Given** a boolean field in unchecked state, **When** the user clicks the toggle, **Then** the state changes to checked/True and is immediately persisted.
2. **Given** a boolean field in checked state, **When** the user clicks the toggle, **Then** the state changes to unchecked/False and is immediately persisted.
3. **Given** a toggle click triggers a save, **When** saving is in progress, **Then** the toggle is temporarily disabled to prevent double-submission.
4. **Given** a persistence failure after a toggle click, **When** the save attempt ends, **Then** the toggle reverts to its previous state and an error indicator is displayed.

---

### User Story 5 — Consistent Grid Across Single and Multiple Entry Views (Priority: P2)

A card may display one rolling stock entry or several. The collector observes that the grid layout, field order, labels, and editing interactions are identical for each entry regardless of how many entries exist on the card.

**Why this priority**: Layout consistency eliminates confusion and ensures a single implementation serves both variants.

**Independent Test**: Open a card with one entry. Open a card with three entries. Verify grid structure and editing behaviour are identical per entry.

**Acceptance Scenarios**:

1. **Given** a card with exactly one rolling stock entry, **When** viewed, **Then** the grid and editing interactions are identical to entries on a multi-entry card.
2. **Given** a card with multiple rolling stock entries, **When** the user edits a field on one entry, **Then** only that entry's field is in edit mode; all other entries remain in read-only display.

---

### Edge Cases

- What happens when the user clicks different fields in rapid succession? The first field auto-saves before the second activates; only one field is in edit mode at a time across all entries on the card.
- What happens when a persistence failure occurs on a toggle? The toggle reverts to its previous state and shows an error indicator.
- What happens when an enumerated options list is empty? The dropdown shows a "No options available" message.
- What happens when rolling stock data is still loading? All field inputs are disabled and loading placeholders are shown throughout the grid.
- What happens when the Lights field value is null? It displays as unchecked/inactive, identical to False.

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: The system MUST display all 15 rolling stock attribute fields (Series, Depot, Livery, Control Type, DCC Interface, Length, Flywheel Fitted, Body Shell, Chassis, Interior Lights, Lights, Coupling Socket, Close Couplers, Digital Shunting) in a 5-row information grid with up to 3 columns per row.
- **FR-002**: The header section MUST display Series Code and Road Number left-aligned, and the Railway Company badge right-aligned.
- **FR-003**: The information grid layout and editing interactions MUST be identical for single-entry and multi-entry rolling stock views.
- **FR-004**: Empty fields MUST display an inviting placeholder (e.g., italicised "Add Depot") rather than collapsing or hiding the grid cell.
- **FR-005**: Users MUST be able to activate inline editing for any field by clicking its displayed value or placeholder.
- **FR-006**: The system MUST persist a field change automatically when the user blurs or presses Enter.
- **FR-007**: The system MUST revert a field to its previous value when the user presses Escape, without persisting any change.
- **FR-008**: Fields with a fixed set of valid values (Control Type, DCC Interface, Coupling Socket, Body Shell, Chassis) MUST present a searchable dropdown when activated.
- **FR-009**: Boolean fields (Flywheel Fitted, Interior Lights, Lights, Close Couplers, Digital Shunting) MUST be rendered as toggles or checkboxes; a click MUST immediately persist the new state.
- **FR-010**: Only one field across all rolling stock entries on a card MUST be in edit mode at any given time.
- **FR-011**: The system MUST display a visual saving indicator (spinner or colour pulse) between the user's confirmation action and back-end acknowledgement.
- **FR-012**: On persistence failure, the system MUST display an inline error indicator and retain the unsaved value for retry.
- **FR-013**: While rolling stock data is loading, all field inputs MUST be disabled and display loading placeholders.
- **FR-014**: The system MUST NOT display explicit Save or Cancel buttons alongside any inline-editable field.
- **FR-015**: The Livery field MUST accept free-text input, not a fixed-value dropdown, to support custom livery names.

### Key Entities

- **Rolling Stock Entry**: An individual locomotive or wagon on a Railway Model Card. Attributes: Series Code (required), Road Number, Livery (free text), Depot (free text), Control Type (enum), DCC Interface (enum), Length (numeric), Flywheel Fitted (boolean), Body Shell (enum), Chassis (enum), Interior Lights (boolean), Lights (boolean), Coupling Socket (enum), Close Couplers (boolean), Digital Shunting (boolean).
- **Railway Model Card**: The parent entity grouping one or more rolling stock entries. Provides the Railway Company context shown in the header.

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: All 15 rolling stock attribute fields are visible in a single view without opening a separate drawer or dialog.
- **SC-002**: Users can update any rolling stock field with no more than two interactions (one click to activate, one to confirm or toggle), compared to the previous drawer workflow requiring 4+ steps.
- **SC-003**: The grid layout and editing interactions are demonstrably identical for single-entry and multi-entry views, verifiable by side-by-side comparison.
- **SC-004**: A visual save indicator appears within the same interaction cycle as the user's confirmation action, with no perceptible delay before it is shown.
- **SC-005**: Zero rolling stock entries exist where any of the 15 grid fields is absent or hidden after the feature is released.

## Assumptions

- The Livery field accepts free-text; it is not constrained to a fixed list (consistent with existing InPlaceEdit usage in the codebase).
- Body Shell and Chassis are constrained enumerations (fixed material categories), not free text (consistent with RollingStockSpecsDrawer).
- The "Lights" field in Row 4 refers to exterior/headlights and is boolean (fitted / not fitted), consistent with how Interior Lights is treated.
- Boolean fields default to a null/unknown state before the user first sets them; null displays identically to False (unchecked/inactive).
- The loading state applies to the entire grid simultaneously, not per individual field.
- Auto-save is suppressed when the submitted value is unchanged (no-op saves are not sent to the back-end).
