# Feature Specification: Rolling Stock Progressive Editing

**Feature Branch**: `024-rolling-stock-edit`
**Created**: 2026-02-17
**Status**: Draft
**Input**: User description: "High-density, context-aware management interface for railway model enthusiasts enabling progressive enrichment of rolling stock data via in-place editing, constrained selection, and contextual technical drawer."

## Overview

This feature transforms the railway model and rolling stock detail views from read-only displays into fully interactive management interfaces. Collectors can enrich their inventory at their own pace — starting with a quick description update, progressing to correcting classification badges (scale, era), and finally completing the full technical profile of each individual rolling stock unit via a structured side drawer.

The guiding principle is **progressive enrichment**: every interaction should be possible without leaving the current page, and every saved change should be immediately visible without a full page reload.

## User Scenarios & Testing _(mandatory)_

### User Story 1 — In-Place Text Editing for Model Description and Details (Priority: P1)

A collector notices that the description and the details notes for one of their railway models are incomplete. Without navigating away, they click directly on the description text, which becomes an editable field. They type their update and click outside to save. They then click the details field and enrich it in exactly the same way. Both fields update instantly on screen.

**Why this priority**: Description and details are the most common free-form fields requiring correction. This pattern also establishes the core interaction model (click-to-edit, blur-to-save) that all subsequent stories build on, making it the foundational MVP slice.

**Independent Test**: Navigate to a Railway Model detail page. Click on the description; edit the text; click outside; verify the new text persists after navigating away and returning. Repeat the identical steps for the details field and verify the same behaviour.

**Acceptance Scenarios**:

1. **Given** a Railway Model detail page with an existing description, **When** the collector clicks on the description text, **Then** the text transitions to an editable field with cursor focus placed at the click point.
   1a. **Given** the same page with an existing details field, **When** the collector clicks on the details text, **Then** the details field transitions to an editable area using the identical interaction behaviour as the description field.
2. **Given** an active in-place text edit, **When** the collector clicks outside the editable field, **Then** the change is saved silently and the field returns to static display with the updated value.
3. **Given** an active in-place text edit, **When** the collector presses Escape, **Then** the edit is cancelled and the original value is restored without saving.
4. **Given** a save attempt fails (e.g., connection unavailable), **When** the field loses focus, **Then** the system shows a clear error message and restores the field to an editable state with the unsaved value preserved.
5. **Given** the description field is empty and the collector focuses on it, **When** they hover over the area without clicking, **Then** a subtle visual affordance (e.g., dashed border or background tint) appears to indicate the area is editable.

---

### User Story 2 — In-Place Text Editing for Rolling Stock Identification Fields (Priority: P2)

A collector is reviewing their rolling stock list and spots an incorrect road number on one card. They click directly on the road number shown on the card, type the correction, and click outside to save — without opening any drawer or modal. They also click the series code and depot to add missing values using the same gesture. All three cards update instantly in place.

**Why this priority**: Series code, road number, livery, and depot are the most frequently referenced and corrected identification fields. Making them directly editable on the card enables rapid catalogue enrichment without the overhead of opening the full technical drawer for a single-field correction.

**Independent Test**: On a rolling stock listing, click the Road Number on any card. Edit the value. Click outside. Verify the updated value is shown on the card and persists after a page refresh. Repeat for Series Code, Livery, and Depot.

**Acceptance Scenarios**:

1. **Given** a rolling stock card displaying a Road Number, **When** the collector clicks on it, **Then** the value transitions to an editable input field with cursor focus, using the same visual behaviour as the model description field.
2. **Given** a rolling stock card, **When** the collector clicks the Series Code, Livery, or Depot field in turn, **Then** each transitions to an editable state independently, following the same click-to-edit interaction.
3. **Given** an active in-place edit on a rolling stock identification field, **When** the collector clicks outside the field, **Then** the change is saved and the field returns to static display with the updated value.
4. **Given** an active in-place edit on a rolling stock identification field, **When** the collector presses Escape, **Then** the edit is cancelled and the original value is restored without saving.
5. **Given** a rolling stock card where Series Code, Road Number, Livery, or Depot has not yet been entered, **When** the collector hovers over that field area, **Then** a visual affordance indicates the field is editable and invites entry.

---

### User Story 3 — Constrained Selection for Scale, Era, and Railway Company (Priority: P3)

A collector catalogued a locomotive but accidentally assigned the wrong scale. They click directly on the Scale badge displayed on the Model Details page. A compact selection menu appears showing all valid scale options. They pick the correct one and the badge updates immediately. On a rolling stock card, they also notice the railway company is wrong — they click the displayed company name and a list of known railway companies appears; they select the correct one and it updates in place.

**Why this priority**: Scale, Era, and Railway Company are classification fields that must remain valid and consistent across the catalogue. Errors here corrupt filtering and reporting. The badge-click / value-click pattern keeps the interaction fast and contextual without introducing a full edit form.

**Independent Test**: On a Railway Model detail page, click the Scale badge. Select a different scale from the picker. Verify the badge updates immediately. On a rolling stock card, click the Railway Company name. Select a different company. Verify the displayed value updates and persists.

**Acceptance Scenarios**:

1. **Given** a Railway Model detail page showing a Scale badge, **When** the collector clicks the badge, **Then** a selection menu appears positioned near the badge displaying all valid scale options.
2. **Given** the scale selection menu is open, **When** the collector selects an option, **Then** the menu closes, the badge updates to show the new value, and the change is saved without additional confirmation.
3. **Given** the scale selection menu is open, **When** the collector presses Escape or clicks outside the menu, **Then** the menu closes and the original value is preserved unchanged.
4. **Given** the Era badge on the Model Details page, **When** the collector clicks it, **Then** the same constrained-selection interaction applies, showing all valid era options.
5. **Given** a rolling stock card displaying a Railway Company name, **When** the collector clicks it, **Then** a selection menu appears showing the list of known railway companies.
6. **Given** the railway company selection menu is open, **When** the collector selects a company, **Then** the menu closes, the displayed company name updates, and the change is saved without additional confirmation.
7. **Given** the system is offline or the save fails, **When** the collector selects a new value in any constrained selector, **Then** an error is shown and the displayed value reverts to the previous one.

---

### User Story 4 — Technical Specification Drawer for Rolling Stock Units (Priority: P4)

A collector wants to record the DCC configuration and coupling type for a specific locomotive in their collection. They click the "Edit Specs" action on that rolling stock card. A side drawer slides in containing a structured form. They complete the relevant fields across the Identification, Technical, Control, and Coupling sections and save. The drawer closes and the card reflects the updated information.

**Why this priority**: The drawer handles the deepest level of data enrichment. It is the most complex interaction but also the most valuable for serious collectors building a complete digital twin. It depends on the in-place pattern being established (P1/P2) but can be implemented and tested independently.

**Independent Test**: On a page listing rolling stock units, click "Edit Specs" on any card. Fill in at least one field from each section. Save. Verify the saved values are visible on the card and persist after closing and reopening the drawer.

**Acceptance Scenarios**:

1. **Given** a rolling stock card with an "Edit Specs" action, **When** the collector clicks the action, **Then** a side drawer slides in from the edge of the screen without obscuring the full page, containing grouped form sections.
2. **Given** the drawer is open, **When** the collector fills in the Road Number and Depot fields and saves, **Then** the drawer closes, the card is updated with the new values, and re-opening the drawer shows the saved data.
3. **Given** the drawer is open with unsaved changes, **When** the collector attempts to close the drawer, **Then** the system warns that unsaved changes will be lost and asks for confirmation before discarding them.
4. **Given** the drawer form, **When** the collector views the Control section, **Then** the Control Type field offers only valid predefined options (e.g., Analogue, DCC Ready, DCC Fitted, DCC Sound) via a constrained selector.
5. **Given** a save attempt in the drawer fails, **When** the collector clicks Save, **Then** the drawer remains open, the error is shown inline, and all entered values are preserved so no data is lost.

---

### Edge Cases

- What happens when the collector clicks a second editable field while the first is still in an unsaved edit state? The first field auto-saves before the second enters edit mode.
- What happens when a required drawer field is left empty on save? The system highlights the empty required fields and prevents save until they are completed.
- What happens if the drawer is opened for a rolling stock unit that has no existing technical data? All fields appear empty and ready for first-time input.
- What happens if the collector edits a rolling stock identification field in-place on the card and then immediately opens the drawer? The drawer MUST display the already-saved value; edits made via the card in-place input and via the drawer form write to the same underlying record and do not conflict.
- What happens if two sessions edit the same record simultaneously? The last save wins; no conflict resolution is required for a single-user desktop application.
- How does the system behave when valid enum values change in a future version (e.g., a new scale is added)? Existing records retain their saved value; the new value becomes selectable immediately after an application update.

## Requirements _(mandatory)_

### Functional Requirements

**In-Place Text Editing**

- **FR-001**: The system MUST allow collectors to edit the model description by clicking or focusing directly on the displayed text, without navigating to a separate edit page.
- **FR-002**: The system MUST allow collectors to edit the model details field by clicking or focusing directly on the displayed text, using the exact same click-to-edit, blur-to-save, and Escape-to-cancel behaviour as the description field.
- **FR-003**: The system MUST save an in-place text change automatically when the collector moves focus away from the field (blur).
- **FR-004**: The system MUST display a visible "Save" action alongside an active in-place edit as an alternative commit mechanism.
- **FR-005**: The system MUST cancel an in-place text edit and restore the original value when the collector presses the Escape key.
- **FR-006**: The system MUST display a subtle visual hover affordance on all editable text areas to indicate they are interactive, without breaking the read-only visual layout.
- **FR-007**: The system MUST show a clear, non-blocking error notification when an in-place save fails and MUST restore the field to an editable state with the unsaved value preserved.

**In-Place Text Editing — Rolling Stock Identification**

- **FR-008**: The system MUST allow collectors to edit the Series Code of a rolling stock unit by clicking directly on the displayed value on the card, without opening the technical drawer.
- **FR-009**: The system MUST allow collectors to edit the Road Number of a rolling stock unit by clicking directly on the displayed value on the card, without opening the technical drawer.
- **FR-010**: The system MUST allow collectors to edit the Livery of a rolling stock unit by clicking directly on the displayed value on the card, without opening the technical drawer.
- **FR-011**: The system MUST allow collectors to edit the Depot of a rolling stock unit by clicking directly on the displayed value on the card, without opening the technical drawer.
- **FR-012**: All four rolling stock identification fields (Series Code, Road Number, Livery, Depot) MUST apply the same click-to-edit, blur-to-save, Escape-to-cancel, hover affordance, and save-failure error handling behaviour defined for model-level in-place text fields (FR-003 through FR-007).
- **FR-013**: When a rolling stock identification field is edited in-place on the card, the updated value MUST be reflected in the technical drawer the next time it is opened, without requiring a page reload.

**Constrained Selection Editing**

- **FR-014**: The system MUST allow collectors to change the Scale of a Railway Model by clicking the Scale badge, which opens a constrained selection menu showing all valid scale options.
- **FR-015**: The system MUST allow collectors to change the Era of a Railway Model by clicking the Era badge, which opens a constrained selection menu showing all valid era options.
- **FR-016**: The constrained selection menu MUST be positioned in close proximity to the triggering badge to preserve spatial context.
- **FR-017**: The system MUST save the selected value immediately upon selection and close the menu without requiring additional confirmation.
- **FR-018**: The system MUST close the selection menu and preserve the original value when the collector dismisses it without selecting (Escape or outside click).
- **FR-019**: The system MUST show an error and revert the displayed value to the original if the save fails after a constrained selection is made.
- **FR-020**: The system MUST allow collectors to change the Railway Company of a rolling stock unit by clicking the displayed company name on the card, which opens a constrained selection menu showing all known railway companies.

**Technical Specification Drawer**

- **FR-021**: Each rolling stock unit card MUST display an "Edit Specs" (or equivalent) action that opens a side-entry drawer.
- **FR-022**: The drawer MUST contain a structured form divided into four sections: Identification, Technical, Control, and Coupling.
- **FR-023**: The Identification section MUST include fields for: Road Number, Series Code, Depot, and Livery.
- **FR-024**: The Technical section MUST include fields for: Flywheel (present/absent), Body Material, Chassis Material, and Lighting type.
- **FR-025**: The Control section MUST include fields for: DCC Interface connector type and Control Type (selectable from predefined options: Analogue, DCC Ready, DCC Fitted, DCC Sound).
- **FR-026**: The Coupling section MUST include fields for: Coupling socket type and coupling feature flags (e.g., NEM compatible, close coupling).
- **FR-027**: The drawer MUST warn the collector before discarding unsaved changes when they attempt to close without saving.
- **FR-028**: The drawer MUST remain open with all entered values preserved when a save attempt fails, and MUST display an inline error message.
- **FR-029**: All constrained fields within the drawer (Control Type, DCC Interface, Coupling socket) MUST use predefined option lists rather than free-text input.

### Key Entities

- **Railway Model**: A catalogue entry representing a specific model product (e.g., "SNCF BB 15000 by Jouef"). Carries classification attributes — Scale and Era — and descriptive text fields. This is the entity targeted by in-place text editing and badge-click constrained selection.
- **Rolling Stock Unit**: A specific physical item in the collector's collection linked to a Railway Model (e.g., serial number or acquisition instance). Carries identification fields editable directly on the card (Series Code, Road Number, Livery, Depot), a constrained-selection Railway Company field, and deeper technical attributes managed via the side drawer.
- **Railway Company**: A classification value from a known list of railway operators (e.g., SNCF, DB, BR, ÖBB) assigned to a Rolling Stock Unit to indicate which operating company the model represents. Selected via constrained picker on the rolling stock card.
- **Scale**: A classification value from a fixed enumeration representing the physical size ratio of the model (e.g., H0, N, TT, O).
- **Era**: A classification value from a fixed enumeration representing the historical period the model depicts (e.g., Epoch I through Epoch VI).
- **Control Type**: A classification value indicating the operational readiness of the unit's control system (e.g., Analogue, DCC Ready, DCC Fitted, DCC Sound).

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: A collector can update either the model's description or details field and see the change reflected on screen in under 3 seconds from the moment they move focus away from the field.
- **SC-002**: A collector can correct a Scale or Era classification via badge click in under 5 seconds from landing on the Model Details page, and can correct a Railway Company via card click in under 5 seconds from landing on the rolling stock view.
- **SC-003**: A collector can open the technical drawer, complete all four sections, and save a full rolling stock specification in under 4 minutes.
- **SC-004**: 100% of constrained fields (Scale, Era, Railway Company, Control Type, DCC Interface, Coupling socket) accept only values from their predefined option lists — no free-text entry is possible for these fields.
- **SC-005**: When a save operation fails, zero data loss occurs — the collector's entered values remain available for re-submission in every failure scenario.
- **SC-006**: The editable affordance hover effect is visually distinct enough that 90% of new users, without instruction, identify description, details, and rolling stock identification fields as interactive on their first visit.
- **SC-008**: A collector can correct any single rolling stock identification field (Series Code, Road Number, Livery, or Depot) directly on the card in under 10 seconds, without opening the technical drawer.
- **SC-007**: The side drawer can be opened and populated for any rolling stock unit regardless of how many fields were previously filled — empty, partial, and complete records are all supported.

## Assumptions

- The application is a single-user desktop tool; no real-time collaboration or conflict resolution between multiple users is required.
- The set of valid Scale and Era values is managed centrally by the application and is not user-configurable within this feature scope.
- The set of valid Control Type and DCC Interface values is also fixed and not user-extensible in this feature.
- "Description" and "Details" are the two free-text fields eligible for in-place editing on the Railway Model view; both follow identical interaction rules. All other Railway Model attributes use the constrained selection pattern.
- "Series Code", "Road Number", "Livery", and "Depot" are the four free-text identification fields eligible for in-place editing directly on the Rolling Stock card; all four follow the same interaction rules as the model-level text fields.
- "Railway Company" on the rolling stock card uses constrained-selection (picker), not free-text, following the same interaction rules as Scale and Era on the Railway Model. The technical drawer remains the entry point for all other rolling stock attributes.
- The drawer does not need to support image attachments or file uploads as part of this feature.
- Saving an in-place edit or drawer form requires an active local application state (no offline queue/sync is needed since this is a local desktop app).
