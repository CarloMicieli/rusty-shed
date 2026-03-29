# Feature Specification: Train Formations

**Feature Branch**: `039-train-formations`
**Created**: 2026-03-29
**Status**: Draft

## Overview

The Train Formations module allows users to move beyond individual rolling stock management and recreate realistic, operational train consists. Users can catalog train sets by metadata, visually compose formations using their rolling stock inventory, verify operational readiness (motorization), and document historical context — all within a unified workspace.

## User Scenarios & Testing _(mandatory)_

### User Story 1 - Create and Manage a Train Formation (Priority: P1)

A user wants to document a specific historical train set by recording its name, category, service era, epoch, and notes. They need to create, edit, view, and delete formation records.

**Why this priority**: Formation metadata is the foundational data object. Without it, no other feature (composition, validation, etc.) can exist. It delivers immediate value as a catalog of named train sets.

**Independent Test**: Create a formation named "Gottardo 1974", assign it category "EuroCity", set service years 1970–1982, pick epoch IV, add markdown notes — then verify all fields persist and display correctly.

**Acceptance Scenarios**:

1. **Given** no formations exist, **When** the user creates a formation with a unique name, category, service era, epoch, and notes, **Then** the formation appears in the formations list with all fields correctly saved.
2. **Given** an existing formation, **When** the user edits any metadata field and saves, **Then** the updated values are reflected without data loss on other fields.
3. **Given** an existing formation, **When** the user deletes it, **Then** the formation and all its associated composition entries are permanently removed.
4. **Given** a formation form, **When** the user submits without a name, **Then** a validation error is shown and no record is created.
5. **Given** a formation with notes, **When** the user views it, **Then** the notes field renders as formatted rich text (Markdown).

---

### User Story 2 - Manage Formation Categories (Priority: P2)

A user wants to assign a category to their formation. Because category lists differ by railway company and era, they need the ability to add custom categories beyond the built-in defaults.

**Why this priority**: Categories are required to classify formations meaningfully. Without user-extensibility, users modeling non-European or niche railways cannot accurately categorize their consists.

**Independent Test**: Open the category picker; add a new custom category "Regionale"; create a formation using it — verify the custom category appears in all future category pickers.

**Acceptance Scenarios**:

1. **Given** the category field in the formation form, **When** the user selects from the list, **Then** built-in categories (EuroCity, Intercity, TEE, Express, Regional, etc.) are available.
2. **Given** the category picker, **When** the user types a new category name not in the list and confirms, **Then** the new category is saved globally and immediately available in all category pickers.
3. **Given** a custom category that exists, **When** the user opens any formation form, **Then** the custom category appears alongside built-in entries.
4. **Given** a custom category in use by one or more formations, **When** the user views the formations list, **Then** each formation correctly displays its category.

---

### User Story 3 - Build a Formation Composition (Priority: P3)

A user wants to assemble the ordered sequence of locomotives and carriages that form their train. They use a side drawer to search their rolling stock inventory and add individual units to the composition.

**Why this priority**: Defining the composition is the core differentiator of this feature. It transforms a metadata-only record into a meaningful, operational consist.

**Independent Test**: Open a formation; open the stock drawer; search for "Re 4/4"; click to add it; add two passenger carriages — verify all three appear in the composition grid in insertion order.

**Acceptance Scenarios**:

1. **Given** an existing formation, **When** the user opens the stock drawer, **Then** the drawer shows searchable rolling stock from the user's inventory (both owned and catalog entries).
2. **Given** an open stock drawer, **When** the user searches by series code or description, **Then** matching results are filtered in real time.
3. **Given** a search result in the drawer, **When** the user clicks a stock entry, **Then** that unit is appended to the end of the formation composition.
4. **Given** a formation composition with multiple entries, **When** the user drags a stock cell to a new position, **Then** the composition order updates and the change is persisted.
5. **Given** a stock cell in the composition, **When** the user removes it, **Then** the unit is removed from the composition and the remaining units shift to fill the gap.
6. **Given** a formation, **When** the user adds the same stock item more than once, **Then** duplicate entries are allowed (a real train can have multiple identical carriages).

---

### User Story 4 - Visualize Composition with Ownership Status (Priority: P4)

A user wants to see the complete formation laid out in a horizontal "track" view, including a side-profile icon for each unit, its series code, service level, and whether they own the matching physical model.

**Why this priority**: The visual grid is the primary interface for reviewing and presenting a formation — it makes the consist immediately understandable and highlights acquisition gaps.

**Independent Test**: Open a formation with 5 units (3 owned, 2 not owned); verify the track view renders all 5 cells, each with icon, labels, and a distinct visual indicator for owned vs. not-owned status.

**Acceptance Scenarios**:

1. **Given** a formation with composition entries, **When** the user views the formation, **Then** the composition renders as a horizontally scrollable row of stock cells.
2. **Given** a stock cell, **When** rendered, **Then** it displays: a side-profile SVG silhouette, a railway badge overlay (e.g., SBB, DB, FS), the series code (e.g., "Re 4/4 II"), and the service level (e.g., "1st Class").
3. **Given** a stock cell for a unit the user owns, **When** rendered, **Then** a visual indicator (tinted background or colored border) distinguishes it from unowned units.
4. **Given** a formation, **When** the view loads, **Then** a sticky identity card (column 1) shows the formation name and category and remains visible while scrolling horizontally.
5. **Given** a formation with many units, **When** the user scrolls horizontally, **Then** the identity card remains fixed and only the stock cells scroll.

---

### User Story 5 - Motorization Validation (Priority: P5)

A user wants to know immediately whether their formation has enough motorized traction units to operate. The system should automatically flag formations with no operative motor units.

**Why this priority**: Operational validity is key for serious modelers. A "no traction" warning prevents building unworkable consists and adds meaningful intelligence to the composition tool.

**Independent Test**: Create a formation containing only dummy (non-motorized) locomotive models; verify a "No Traction" warning icon appears. Add one active motorized locomotive; verify the warning disappears.

**Acceptance Scenarios**:

1. **Given** a formation whose composition contains at least one non-dummy motorized unit, **When** the formation is viewed, **Then** no traction warning is shown.
2. **Given** a formation whose composition contains only dummy or no motorized units, **When** the formation is viewed, **Then** a "No Traction" warning icon is displayed in the identity card.
3. **Given** a dummy motorized unit in the composition (locomotive model without a motor), **When** the user opts to count it as satisfying traction, **Then** the dummy unit is visually distinct but the warning is suppressed.
4. **Given** a formation with the "No Traction" warning active, **When** the user drags a non-dummy motorized locomotive into the composition, **Then** the warning disappears immediately without requiring a page reload.
5. **Given** a formation with an empty composition, **When** viewed, **Then** the traction warning is shown (zero units = no traction).

---

### Edge Cases

- What happens when a rolling stock item referenced in a formation composition is later deleted from the inventory? The composition entry should remain with a visual indicator that the referenced stock no longer exists.
- What happens when the user creates two formations with exactly the same name? A uniqueness validation error should prevent duplicate names.
- What happens when a formation's service start year is later than the end year? Validation should block saving with a clear error.
- What happens when the composition is empty and the user opens the formation's track view? An empty-state placeholder prompt to add stock should be displayed.
- What happens when the railway badge image for a specific operator is missing? A fallback showing the operator code as text should display instead.
- How does the system handle very long formations (50+ units)? The horizontal scroll must remain performant; no layout breakage.

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: The system MUST allow users to create a train formation with: a unique name, a category, a service start year, a service end year, a railway epoch, and a markdown-enabled notes field.
- **FR-002**: The system MUST validate that the formation name is unique; duplicate names MUST be rejected with an error message.
- **FR-003**: The system MUST validate that the service start year is less than or equal to the service end year; invalid ranges MUST be rejected.
- **FR-004**: Users MUST be able to edit all metadata fields of an existing formation and save changes.
- **FR-005**: Users MUST be able to delete a formation; all associated composition entries MUST be removed when the formation is deleted.
- **FR-006**: The system MUST provide a pre-defined list of formation categories (EuroCity, Intercity, TEE, Express, Regional at minimum).
- **FR-007**: Users MUST be able to add custom categories to the global category list; custom categories MUST persist and be available in all future category selections.
- **FR-008**: The system MUST display a horizontal scrolling track view where each composition entry is rendered as a stock cell showing: SVG side-profile icon, railway operator badge, series code, and service level.
- **FR-009**: The identity card (formation name, category, traction warning) MUST remain sticky/fixed while the user scrolls horizontally through the composition.
- **FR-010**: The system MUST visually distinguish owned stock from non-owned stock within the composition grid using a color-coded indicator (background tint or border).
- **FR-011**: The system MUST provide a side drawer panel that lets users search their rolling stock catalog and add units to the active formation composition.
- **FR-012**: The drawer search MUST filter results in real time as the user types.
- **FR-013**: Users MUST be able to reorder composition entries via drag-and-drop; the new order MUST be saved immediately.
- **FR-014**: A drag-and-drop interaction MUST show a ghost/placeholder element indicating the drop target before the user releases.
- **FR-015**: Users MUST be able to remove individual stock entries from the composition.
- **FR-016**: The system MUST allow the same stock item to appear multiple times in a single composition (duplicate entries permitted).
- **FR-017**: The system MUST evaluate traction for every composition: if no non-dummy motorized unit exists in the composition, a "No Traction" warning MUST be displayed in the identity card.
- **FR-018**: Users MUST be able to flag a dummy motorized unit as "counts for traction" on a per-entry basis; such entries MUST be visually distinct but suppress the no-traction warning.
- **FR-019**: The notes field MUST support inline rich-text editing with Markdown output, rendered in-place without navigating to a separate edit screen.
- **FR-020**: When a rolling stock item referenced in a composition is deleted from inventory, the composition entry MUST remain with a visual "stock not found" indicator rather than silently disappearing.

### Key Entities

- **TrainFormation**: Represents a named train set. Key attributes: unique name, category (link to FormationCategory), service start year, service end year, railway epoch, markdown notes, ordered list of composition entries.
- **FormationCategory**: A named classification for a formation type (e.g., EuroCity, TEE). Supports both built-in and user-created entries.
- **FormationEntry**: Represents a single rolling stock unit's slot in a formation. Key attributes: position (order index), reference to rolling stock, dummy-counts-for-traction override flag. Belongs to one TrainFormation.
- **RollingStock** _(existing entity)_: The locomotive or carriage being referenced. Relevant attributes: series code, service level, operator badge, is_dummy flag, ownership status.

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: Users can create a fully specified formation (metadata + at least 3 composition units) in under 3 minutes from an empty state.
- **SC-002**: The track view renders a formation with up to 50 stock cells within 2 seconds on a standard consumer laptop.
- **SC-003**: The traction warning updates within 500 milliseconds of a composition change (add, remove, or drag) without requiring a page navigation or manual refresh.
- **SC-004**: The real-time drawer search returns filtered results within 300 milliseconds of the last keystroke.
- **SC-005**: 90% of users can successfully build and reorder a 5-unit formation on their first attempt without consulting documentation, as measured by task-completion testing.
- **SC-006**: All formation metadata fields survive an app restart without data loss (full persistence).
- **SC-007**: Drag-and-drop reordering is usable on touch devices (tablet) as well as mouse-driven desktop without requiring a separate interaction mode.
