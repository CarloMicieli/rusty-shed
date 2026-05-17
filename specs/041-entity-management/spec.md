# Feature Specification: Centralized Entity Management

**Feature Branch**: `041-entity-management`  
**Created**: 2026-05-15  
**Status**: Draft  
**Input**: User description: "Centralized CRUD management of Manufacturers, Sellers and Buyers in Settings; follow-up to 040-quick-add-entities"

## Dependencies

- **Follows**: [`040-quick-add-entities`](../040-quick-add-entities/spec.md) — quick-add already ships create-only flows and a shared quick-add form.
- **Depends on**: Existing Settings page infrastructure.

## Overview

The application currently has no dedicated view for managing the reference data that underpins the collection — Manufacturers, Sellers, and Buyers. This feature adds a **Library** section to the Settings page providing a full Create, Read, Update, and Delete interface for all three entity types. It also introduces an ownership model that distinguishes system-seeded (read-only) records from user-created ones, and enforces referential integrity before any deletion. As a follow-up to feature 040, this feature extends the existing shared quick-add form into a mode-based shared entity form (`QUICK` and `FULL`) while preserving existing quick-add behavior.

## Clarifications

### Session 2026-05-17

- Q: In the shared buyer/seller table model, should usage count and deletion checks consider only the current role tab or all references to the shared party record? → A: Usage count and deletion checks must consider all references to the shared party record across both buyer and seller contexts.
- Q: In the shared buyer/seller table model, should Buyers and Sellers tabs represent one canonical shared party record or separate per-tab projections? → A: Use one canonical shared party record shown in both tabs with role-context labels; edits apply to the same underlying party.
- Q: When creating from Buyers or Sellers tab, should the new shared party appear only in the originating tab or in both tabs immediately? → A: Create one shared party and show it immediately in both tabs.
- Q: For API boundaries, should Buyer and Seller use distinct command surfaces or a single generic party command set? → A: Use distinct buyer and seller command surfaces while sharing repository/table logic internally.
- Q: For shared buyer/seller records, should merge operate per-tab or on canonical records across all contexts? → A: Merge operates on canonical shared party records and re-links buyer and seller references in one atomic transaction.

## User Scenarios & Testing _(mandatory)_

### User Story 1 - Browse and Search All Entities (Priority: P1)

A collector wants to review all the manufacturers in the system to check for duplicates or typos before adding items to their collection. They navigate to Settings > Library, switch between tabs, and use the search bar to filter by name or country.

**Why this priority**: Read access to the entity library is the foundation all other operations depend on; it must exist before edit, add, or delete can be built or tested.

**Independent Test**: Can be fully tested by navigating to Settings > Library, verifying the three tabs (Manufacturers, Sellers, Buyers) render their respective lists, and confirming the search bar filters rows by name in real time.

**Acceptance Scenarios**:

1. **Given** the user navigates to Settings, **When** they select the Library section, **Then** they see a tabbed view with three tabs: Manufacturers, Sellers, and Buyers.
2. **Given** the Manufacturers tab is active, **When** the list renders, **Then** each row shows the entity Name, Country, Usage Count, Origin badge (System / User), and available Actions.
3. **Given** the search bar is empty, **When** the user types a partial name (e.g., "mark"), **Then** the list filters in real time to show only rows whose Name or Country contains that string (case-insensitive).
4. **Given** the search returns no matches, **When** the filtered list is empty, **Then** an appropriate empty-state message is shown.

---

### User Story 2 - Add a New Entity from Settings (Priority: P1)

A collector discovers a new manufacturer not yet in the system while browsing their collection offline. They navigate to Settings > Library > Manufacturers, click "Add New", fill in all available fields (Name, Website, Country, Notes), and save.

**Why this priority**: The add flow establishes the full-field Settings workflow and upgrades the existing shared quick-add form to a mode-based shared form contract.

**Independent Test**: Can be fully tested by clicking "Add New" on any tab, completing the full-field form, saving, and verifying the new entity appears in the list — independently of the Quick-Add drawer.

**Acceptance Scenarios**:

1. **Given** the user is on the Manufacturers tab, **When** they click "Add New", **Then** an entity form drawer opens in full-field mode showing Name (required), Website, Country, and Notes fields.
2. **Given** the form is open with a unique, non-empty name, **When** the user saves, **Then** the entity is persisted and immediately appears in the list without a full page reload.
3. **Given** the form is open and the user types a name that already exists (case-insensitive), **Then** a real-time duplicate warning is shown and the Save button is disabled.
4. **Given** the user cancels or closes the form without saving, **Then** no entity is created and the list is unchanged.

---

### User Story 3 - Edit a User-Created Entity (Priority: P1)

A collector notices a typo in a seller name they created ("Pikoo" instead of "Piko"). They click Edit on that row, correct the name in the form, and save. All collection records referencing that seller are updated automatically.

**Why this priority**: Edits are the most common post-creation operation and directly affect data integrity across the collection.

**Independent Test**: Can be fully tested by editing a user-created entity's name and verifying the change is reflected in the entity list and in any linked acquisition or collection record.

**Acceptance Scenarios**:

1. **Given** a user-created entity row, **When** the user clicks Edit, **Then** the same entity form drawer opens pre-populated with the entity's current data.
2. **Given** the form is open, **When** the user changes the name to a unique value and saves, **Then** the entity is updated and all linked records reflect the new name.
3. **Given** the entity is system-seeded (`is_system_seeded = true`), **When** the user views that row, **Then** the Edit button is absent or disabled and a "Protected" indicator is shown.
4. **Given** the entity is system-seeded, **When** the user opens its detail view, **Then** extended metadata (e.g., personal Notes) may be edited, but the Name field is read-only.

---

### User Story 4 - Delete an Unused User-Created Entity (Priority: P2)

A collector created a seller by mistake ("Temp Seller") and wants to remove it. The system checks that the entity has no linked records and allows deletion after a confirmation prompt.

**Why this priority**: Deletion with dependency checking prevents orphaned data and is less critical than read/edit but necessary for housekeeping.

**Independent Test**: Can be tested by creating a new entity, immediately deleting it (zero usage), and verifying it is removed. Also tested by attempting to delete an entity with linked records and verifying the system blocks the deletion.

**Acceptance Scenarios**:

1. **Given** a user-created entity with zero linked records, **When** the user clicks Delete, **Then** a confirmation modal appears listing the entity name and stating it has no linked items.
2. **Given** the confirmation modal, **When** the user confirms, **Then** the entity is deleted and removed from the list.
3. **Given** a user-created entity with one or more linked records, **When** the user views that row, **Then** the Delete button is replaced by a "Locked" indicator showing the usage count (e.g., "Used in 5 items").
4. **Given** a system-seeded entity (any usage count), **When** the user views that row, **Then** no Delete control is present.

---

### User Story 5 - Merge Two Duplicate Entities (Priority: P3)

A collector has both "Hornby" and "Hornyb" in their manufacturer list due to a past typo. They select both, choose Merge, pick the canonical name to keep, and the system reassigns all linked records to the surviving entity and deletes the duplicate.

**Why this priority**: Merge is a power-user data-hygiene operation; it delivers high value but is complex and can be deferred after the core CRUD is shipped.

**Independent Test**: Can be tested by creating two entities, linking records to each, performing a merge, and verifying the duplicate is removed and all formerly linked records now reference the surviving entity.

**Acceptance Scenarios**:

1. **Given** two user-created entities selected in the list, **When** the user chooses Merge, **Then** a modal asks which entity name to keep as the canonical record.
2. **Given** the user confirms the merge, **When** the operation completes, **Then** all records previously linked to the removed entity now reference the surviving entity, and the duplicate row is deleted.
3. **Given** either selected entity is system-seeded, **When** the user attempts to merge, **Then** the merge action is blocked with an explanation that system-seeded records cannot be merged.

---

### User Story 6 - Mobile Responsive Table (Priority: P3)

On a narrow screen, the tabular layout shifts to a card-based view so entity rows remain readable and actionable without horizontal scrolling.

**Why this priority**: Mobile usability is required but the desktop table is the primary interface; the responsive adaptation is a polish item.

**Independent Test**: Can be tested by resizing to a mobile viewport and verifying each entity renders as a card with Name, Origin, Status, and action buttons accessible without scrolling.

**Acceptance Scenarios**:

1. **Given** the user is on a mobile viewport (< 768 px wide), **When** they open the Library section, **Then** entities are displayed as cards rather than table rows.
2. **Given** the card layout, **When** the user interacts with Edit or Delete, **Then** the same form drawer and confirmation behaviour applies as in the desktop table.

---

### Edge Cases

- What happens when an edit changes a name to one that already exists (case-insensitive)? The save must be blocked with a duplicate warning, identical to the add flow.
- What happens when a merge operation fails mid-way (e.g., a database error during re-linking)? The system must roll back the entire operation and leave both entities intact.
- What happens when all entities in a tab are system-seeded? The "Add New" button must still be available; the Delete and Edit-Name controls are simply absent from all rows.
- What happens when the usage count changes between when the Delete button is clicked and when the confirmation is accepted? The backend must re-check dependencies at the moment of deletion and reject it if the count is now non-zero.
- What happens when a user tries to delete an entity and the request fails? The entity must remain in the list and an error message must be shown.

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: The Settings page MUST include a **Library** section containing a tabbed view with three tabs: Manufacturers, Sellers, and Buyers.
- **FR-002**: Each tab MUST display a table (desktop) or card list (mobile) of all entities of that type, showing at minimum: Name, Country, Usage Count, Origin badge, and available Actions. Buyer/Seller tabs MUST render the same canonical shared party records with role-context labeling rather than separate duplicated records.
- **FR-003**: A search bar MUST filter the visible entity list in real time by Name or Country using a case-insensitive match.
- **FR-004**: Each tab MUST provide an "Add New" button that opens the shared entity form drawer in **full-field mode** (Name, Website, Country, Notes).
- **FR-005**: The "Add New" form MUST perform a real-time, case-insensitive duplicate check on the Name field and disable Save while a duplicate is detected or the Name is empty/whitespace.
- **FR-006**: Clicking Edit on a user-created entity MUST open the same shared entity form drawer pre-populated with that entity's current data; saving MUST persist the changes and update all linked records atomically.
- **FR-007**: System-seeded entities (`is_system_seeded = true`) MUST NOT expose Edit-Name or Delete controls; their Name field MUST be read-only in any form view. Extended metadata fields (e.g., Notes) MAY remain editable.
- **FR-008**: The Delete control MUST be visible only for user-created entities with zero linked records. When an entity has linked records, the control MUST be replaced by a "Locked" indicator showing the usage count. For seller/buyer records stored in the shared table, usage count MUST include all links to the shared party record across both buyer and seller contexts.
- **FR-009**: Before executing a deletion the backend MUST re-validate that `is_system_seeded = false` AND `usage_count = 0`; if either condition fails the operation MUST be rejected and the UI MUST display the reason. For seller/buyer records stored in the shared table, this re-validation MUST use total usage across both buyer and seller contexts.
- **FR-010**: A confirmation modal MUST be shown before any deletion, stating the entity name and the affected linked-item count (or explicit zero), and requiring explicit user confirmation.
- **FR-011**: The Merge action MUST allow selecting exactly two user-created entities of the same type, choosing the canonical name to retain, and atomically re-linking all records from the removed entity to the surviving one before deleting the duplicate. For shared buyer/seller records, merge MUST operate on canonical party records and re-link references across both buyer and seller contexts in the same transaction.
- **FR-012**: Every entity record MUST carry an `is_system_seeded` boolean flag set at the time of creation. User-created records always have `is_system_seeded = false`; system-provided seed records have `is_system_seeded = true`.
- **FR-013**: The shared entity form component MUST accept a `mode` parameter (`QUICK` or `FULL`) that controls which fields are shown; `FULL` mode exposes all fields (used in Settings); `QUICK` mode shows only Name, Website, and Country (used in the Quick-Add drawer in feature 040).
- **FR-014**: A system-seeded entity row MUST display a visible "Protected / System" badge; a user-created entity in use MUST display an "In Use (N)" badge; an unused user-created entity MUST display an "Unused" badge.
- **FR-015**: Buyers and sellers MUST be exposed as distinct aggregates and command surfaces in the application layer, while persisting to the same underlying database table. Buyer/Seller behavior is distinguished by context and command intent, not by separate storage tables.
- **FR-016**: Editing a shared buyer/seller party record from either tab MUST update the same canonical underlying record, and the updated values MUST be reflected immediately in both tabs.
- **FR-017**: Creating a new buyer/seller party from either tab MUST create one shared canonical party record and that record MUST appear immediately in both Buyers and Sellers tabs.
- **FR-018**: The backend command contract MUST expose distinct Buyer and Seller command/query entry points (e.g., `create_buyer` and `create_seller`) even when both execute against shared repository logic and a shared underlying table.

### Key Entities

- **Manufacturer**: Model railway manufacturer. Attributes: id (UUID), name (unique, case-insensitive), website (optional), country (optional), notes (optional), is_system_seeded (boolean), created_at, updated_at. Linked to: collection items, acquisitions, wishlist items.
- **Seller**: Party that sells in an acquisition. Attributes: id (UUID), name (unique, case-insensitive), website (optional), country (optional), notes (optional), is_system_seeded (boolean), created_at, updated_at. Linked to: acquisitions.
- **Buyer**: Party that buys in an acquisition. Domain behavior is represented as a Buyer aggregate and command surface, but persistence is shared with Seller in the same underlying database table. Attributes: id (UUID), name (unique, case-insensitive), website (optional), country (optional), notes (optional), is_system_seeded (boolean), created_at, updated_at. Linked to: acquisitions.

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: A user can update a manufacturer's name and the change is reflected across all linked collection items within a single database transaction (no stale references).
- **SC-002**: The system presents a warning modal before any deletion that correctly states how many items will be affected (or confirms zero); no deletion occurs without explicit confirmation.
- **SC-003**: A user-created entity with zero links can be deleted in <=3 clicks from the list view (Edit row -> Delete button -> Confirm modal).
- **SC-004**: A system-seeded entity cannot be deleted or have its name edited under any user-initiated action path.
- **SC-005**: The duplicate-check warning appears within 500 milliseconds of the user finishing typing a name.
- **SC-006**: The desktop table and mobile card layout render without horizontal overflow or clipped content on screens as narrow as 375 px.
- **SC-007**: The shared entity form component is reused without modification in the Quick-Add drawer (feature 040); no forked copy exists.

## Assumptions

- System-seeded records are inserted via database migrations or seed scripts at install/upgrade time; this feature does not define the seed data itself.
- "Linked records" means any row in `collection_items`, `acquisitions`, or `wishlist_items` that references the entity's ID via a foreign key.
- Case-insensitive uniqueness is enforced at the database layer using `LOWER(name)` comparison; the implementation target is one unique index for manufacturers and one shared unique index for buyer/seller records in the shared table.
- The Merge feature (US5) is P3 and may ship as a follow-on iteration after core CRUD is live.
- A "mobile viewport" is defined as screen width below 768 px.
