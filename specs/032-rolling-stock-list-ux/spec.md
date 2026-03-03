# Feature Specification: Rolling Stock List UX

**Feature Branch**: `032-rolling-stock-list-ux`
**Created**: 2026-03-03
**Status**: Draft
**Input**: User description: "Rolling Stock List Component improvements covering empty field display, inline editing, and unified add workflow via Side Drawer."

## User Scenarios & Testing _(mandatory)_

### User Story 1 - Clean Empty Field Display (Priority: P1)

A collector is viewing the detail panel of a Railway Model Card that has rolling stock entries. Some fields (e.g., Depot, Livery) have no data yet. Instead of seeing italic placeholder text like "Depot" or "Livery", they see a neutral dash ("—") under the field label. The label itself remains in its compact uppercase header style.

**Why this priority**: Removing misleading placeholder text from the read-only view is a pure display fix with no data-model impact. It delivers immediate visual quality improvement and is prerequisite to a coherent inline-edit experience.

**Independent Test**: Open a rolling stock entry that has at least one empty field. Confirm the UI shows "—" under the label without any italic placeholder text. No editing interaction needed.

**Acceptance Scenarios**:

1. **Given** a rolling stock entry where the Depot field has no value, **When** the user views the entry in read-only mode, **Then** the Depot label ("DEPOT") is shown in small uppercase, and the value area displays "—" (not the word "Depot" in italics).
2. **Given** a rolling stock entry where Livery, Length, or any other optional field has no value, **When** the user views the entry, **Then** every empty field shows "—" as its value.
3. **Given** a rolling stock entry where all fields are populated, **When** the user views the entry, **Then** actual field values are shown (no dashes appear for populated fields).

---

### User Story 2 - Inline Field Editing (Priority: P1)

A collector wants to update the Depot on an existing rolling stock entry. Instead of activating an edit mode that shows Save/Cancel buttons, they click directly on the value (or the "—" dash) and an input field appears in place. They type the new value, press Enter or click away, and the change is saved automatically. Pressing Escape reverts to the previous value without saving.

**Why this priority**: Inline editing removes the cognitive overhead of a modal edit mode and eliminates the Save/Cancel button clutter. It represents the primary productivity improvement in this feature.

**Independent Test**: On a rolling stock entry with at least one populated field and one empty field: click a populated value, verify input appears with existing value; type a change; press Enter; verify value is saved. Then click the "—" dash on an empty field; type a value; press Escape; verify no change was saved.

**Acceptance Scenarios**:

1. **Given** the user is viewing a rolling stock entry, **When** they click on a field value or "—" dash, **Then** an input control appears in place of the display value, pre-filled with the current value (empty if dash).
2. **Given** an input is active, **When** the user presses Enter or clicks outside the field, **Then** the new value is saved and the field returns to read-only display.
3. **Given** an input is active, **When** the user presses Escape, **Then** the field reverts to its previous value without saving, and no network/persistence action is triggered.
4. **Given** an input is active, **When** the user clears the value and confirms (Enter/blur), **Then** the field saves as empty and subsequently displays "—".
5. **Given** the user views a rolling stock entry, **Then** no "Save" or "Cancel" buttons are visible in the UI; all persistence is handled automatically on blur or Enter.

---

### User Story 3 - Add First Rolling Stock via Drawer (Priority: P2)

A collector opens a Railway Model Card that has no rolling stock entries. Instead of a "No additional details" message, they see a clear call to action (an "Add Rolling Stock" button or a large empty-state card with a plus icon). Clicking it opens a Side Drawer from the right side of the screen containing a form with all rolling stock fields. They fill in the form and submit it to create the first entry.

**Why this priority**: Once inline editing is in place, the path to creating the first entry becomes the next friction point. This story establishes the creation UX pattern used by all subsequent entries.

**Independent Test**: Navigate to a Railway Model Card with zero rolling stock entries. Verify the empty-state CTA is present. Click it, verify the Side Drawer opens with a creation form containing all fields. Fill in required fields and submit; verify the entry appears in the list and the drawer closes.

**Acceptance Scenarios**:

1. **Given** a Railway Model Card has no rolling stock entries, **When** the user views the rolling stock section, **Then** a prominent "Add Rolling Stock" call-to-action (button or empty-state card) is displayed instead of a "No additional details" message.
2. **Given** the user clicks the "Add Rolling Stock" CTA, **When** the Side Drawer opens, **Then** it slides in from the right of the screen and displays a form containing all rolling stock fields: Series Code, Depot, Livery, Length, and Control Type at minimum.
3. **Given** the drawer form is open and the user submits a valid entry, **When** the submission is confirmed, **Then** the drawer closes, the new rolling stock entry appears in the list, and no "No additional details" message is shown.
4. **Given** the drawer is open, **When** the user dismisses it (close button, overlay click, or Escape key), **Then** the drawer closes without creating any entry.

---

### User Story 4 - Add More Rolling Stock (Priority: P2)

A collector views a Railway Model Card that already has one or more rolling stock entries. They want to add another. An "+ Add Rolling Stock" button is visible (either in the section header or below the list). Clicking it opens the same Side Drawer as in the empty-state flow.

**Why this priority**: Unified creation experience—the same drawer is used regardless of whether entries already exist—reduces cognitive load and ensures implementation consistency.

**Independent Test**: Navigate to a Railway Model Card with at least one rolling stock entry. Verify the "+ Add Rolling Stock" button is visible. Click it, verify the same Side Drawer opens as in the empty state. Add a new entry and verify it appends to the list.

**Acceptance Scenarios**:

1. **Given** a Railway Model Card has one or more rolling stock entries, **When** the user views the rolling stock section, **Then** an "+ Add Rolling Stock" button is visible (in the section header or below the last entry).
2. **Given** the user clicks the "+ Add Rolling Stock" button, **When** the Side Drawer opens, **Then** it is identical in appearance and behaviour to the drawer triggered from the empty state.
3. **Given** the user adds a new entry via the drawer, **When** the submission succeeds, **Then** the new entry is appended to the existing list and the drawer closes.

---

### Edge Cases

- What happens when a user rapidly double-clicks a field value? The first click should initiate edit mode; the second click should be absorbed by the now-active input.
- What happens when the user starts editing a field and another field in the same entry is also in edit mode? Only one field should be in edit mode at a time; clicking another field should save the current edit before activating the next.
- What happens if the auto-save fails (network/persistence error)? The field should display an error indicator and retain the unsaved value so the user can retry.
- What happens when the user opens the Side Drawer and navigates away (e.g., closes the parent card)? The drawer should be dismissed without saving.
- What if the Control Type field is a fixed set of values (enum)? The inline edit for that field should present a dropdown or selection control rather than a free-text input.

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: The system MUST display a "—" character for any rolling stock field that has no saved value when the entry is in read-only (view) mode.
- **FR-002**: The system MUST NOT display italic placeholder text (field names in italics) for empty rolling stock fields in view mode.
- **FR-003**: Field labels (e.g., "DEPOT", "LIVERY") MUST remain in their small uppercase header style in both read-only and edit modes.
- **FR-004**: Users MUST be able to activate inline editing for a rolling stock field by clicking its displayed value or "—" dash.
- **FR-005**: The system MUST automatically save an inline field edit when the user moves focus away from the active input (blur) or presses Enter.
- **FR-006**: The system MUST revert an inline field edit to its previous value when the user presses Escape, without persisting any change.
- **FR-007**: The system MUST NOT display Save or Cancel buttons alongside any inline-editable rolling stock field.
- **FR-008**: Only one inline edit input MUST be active at a time per rolling stock entry; activating a second field MUST save and close the first.
- **FR-009**: When a Railway Model Card has no rolling stock entries, the system MUST display a prominent "Add Rolling Stock" call-to-action in place of any "No additional details" message.
- **FR-010**: Clicking the "Add Rolling Stock" CTA (empty state) MUST open a Side Drawer that slides in from the right side of the screen.
- **FR-011**: The Side Drawer MUST contain a creation form with all standard rolling stock fields: Series Code, Depot, Livery, Length, and Control Type at minimum.
- **FR-012**: Submitting the creation form in the Side Drawer MUST create the new rolling stock entry, close the drawer, and display the entry in the list.
- **FR-013**: When a Railway Model Card has one or more rolling stock entries, an "+ Add Rolling Stock" button MUST be visible in the rolling stock section (header or below the list).
- **FR-014**: The "+ Add Rolling Stock" button (populated state) MUST open the same Side Drawer as the empty-state CTA, providing a unified creation experience.
- **FR-015**: The user MUST be able to dismiss the Side Drawer without saving by closing it (close button, backdrop click, or Escape key).

### Key Entities

- **Rolling Stock Entry**: An individual model associated with a Railway Model Card. Key attributes: Series Code, Depot, Livery, Length, Control Type. Any attribute may be absent (null/empty). Each entry belongs to exactly one Railway Model Card.
- **Railway Model Card**: The parent entity that groups one or more rolling stock entries. Determines whether the empty-state or populated-state CTA is shown.
- **Side Drawer**: A panel that slides in from the right edge of the screen, containing the rolling stock creation form. Shared between empty-state and populated-state add workflows.

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: Users can update a rolling stock field value with no more than two interactions (one click to activate, one action to confirm), compared to the previous multi-step edit flow.
- **SC-002**: Zero instances of italic placeholder field-name text appear in read-only rolling stock views after the feature is released.
- **SC-003**: Users can add a new rolling stock entry (first or additional) through a single consistent interaction path (the Side Drawer), regardless of how many entries already exist.
- **SC-004**: The time to add a new rolling stock entry is reduced compared to the previous workflow, as measured by the number of user actions required (target: under 5 actions from intent to saved entry).
- **SC-005**: 100% of rolling stock fields support inline editing without requiring a separate edit mode or explicit Save/Cancel buttons.

## Assumptions

- The Control Type field has a fixed set of allowed values (enum); inline editing for this field will use an appropriate selection control, not free-text input.
- The Side Drawer form validation follows the same rules as any existing rolling stock creation/edit forms in the application.
- "Series Code" is treated as a required field for creation; all other fields are optional.
- The inline-edit auto-save only triggers a persistence action if the value has actually changed; no-op saves are suppressed.
- The "—" empty state indicator is a display-only convention; no dash character is ever persisted to the database.
