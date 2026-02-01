# Feature Specification: Add Railway Model to Wishlist

**Feature Branch**: `003-add-model-wishlist`  
**Created**: 2026-01-30  
**Status**: Draft  
**Input**: User description: "Add railway model to a wishlist - from 'My Wishlists' page, user clicks 'Add railway model' button, a slide-in dialog collects railway model data (manufacturer, product code, description, category, scale, power method, epoch) and rolling stocks (railway company, series code, category, road number), plus wishlist item details (desired price, priority). Creates new railway model in catalog and adds to selected wishlist."

## User Scenarios & Testing _(mandatory)_

### User Story 1 - Add Railway Model from Wishlists Overview (Priority: P1)

A user navigates to the "My Wishlists" page and wants to add a new railway model to one of their wishlists. They click the "Add railway model" button, complete the form in a side drawer, and submit. The application creates the railway model in the catalog and adds it to the selected wishlist.

**Why this priority**: This is the core functionality—users need to be able to add new railway models they desire to their wishlists. It provides the primary value of the feature.

**Independent Test**: Can be fully tested by navigating to "My Wishlists", clicking "Add railway model", filling the form, and verifying the item appears in the selected wishlist with correct details.

**Acceptance Scenarios**:

1. **Given** the user is on the "My Wishlists" page with at least one wishlist, **When** they click the "Add railway model" button, **Then** a side drawer slides in from the right with the railway model creation form.

2. **Given** the side drawer is open, **When** the user fills all required fields (wishlist, manufacturer, product code, description, category, scale, power method, epoch) and submits, **Then** a new railway model is created in the catalog and added to the selected wishlist with the current date.

3. **Given** the form includes rolling stock entries (see US3 for add/remove management), **When** the user submits, **Then** all rolling stocks with valid required fields (railway company, series code, category) are included in the created railway model.

4. **Given** the user provides a desired price and priority, **When** they submit the form, **Then** the wishlist item is created with those values.

5. **Given** the form is incomplete (missing required fields), **When** the user attempts to submit, **Then** the form displays validation errors and prevents submission.

---

### User Story 2 - Add Railway Model from Selected Wishlist Context (Priority: P2)

A user selects a specific wishlist from the sidebar and then clicks "Add railway model". The wishlist selector is pre-populated with the currently selected wishlist.

**Why this priority**: This improves efficiency for users who are already viewing a specific wishlist. It reduces clicks and cognitive load.

**Independent Test**: Can be tested by selecting a wishlist, clicking "Add railway model", and verifying the wishlist dropdown is pre-selected.

**Acceptance Scenarios**:

1. **Given** the user has selected a wishlist from the sidebar, **When** they click "Add railway model", **Then** the side drawer opens with that wishlist pre-selected in the dropdown.

2. **Given** the wishlist is pre-selected, **When** the user changes to a different wishlist in the dropdown, **Then** the form allows submission to the newly selected wishlist.

---

### User Story 3 - Manage Rolling Stocks in Railway Model (Priority: P3)

A user can add multiple rolling stocks to a railway model during creation, and can remove rolling stocks before submission.

**Why this priority**: Many railway models include multiple rolling stocks (e.g., train sets). This supports complete data entry but is not required for basic functionality.

**Independent Test**: Can be tested by adding multiple rolling stocks, removing some, and verifying the final submission includes only the remaining items.

**Acceptance Scenarios**:

1. **Given** the side drawer is open, **When** the user clicks "Add rolling stock", **Then** a new rolling stock entry section appears with fields for railway company, series code, category, and optional road number.

2. **Given** multiple rolling stock entries exist, **When** the user clicks the remove button on one entry, **Then** that entry is removed from the form.

3. **Given** no rolling stocks are added, **When** the user submits the form, **Then** the railway model is created without rolling stocks (this is valid).

---

### Edge Cases

- What happens when the user has no wishlists? The "Add railway model" button is disabled (per FR-014), and a tooltip explains that a wishlist must be created first.
- What happens when the manufacturer list is empty? The system should display an appropriate message and prevent form submission.
- How does the system handle network errors during submission? The system should display an error message and retain the form data for retry.
- What happens when the user closes the drawer mid-entry? Any unsaved data is discarded (standard behavior for this application).
- What happens if a required dropdown has no selection? The submit button remains disabled until all required fields are complete.

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: System MUST display an "Add railway model" button on the "My Wishlists" page that opens a side drawer from the right.
- **FR-002**: The side drawer MUST include a wishlist selector showing all available wishlists by name.
- **FR-003**: The side drawer MUST include required fields for railway model: manufacturer (dropdown), product code (text), description (text), category (dropdown), scale (dropdown), power method (dropdown), and epoch (text).
- **FR-004**: The side drawer MUST include optional wishlist item fields: desired price (currency amount) and priority (dropdown with Low/Normal/High).
- **FR-005**: Users MUST be able to add one or more rolling stocks to the railway model with fields: railway company (dropdown), series code (text), category (dropdown), and road number (optional text).
- **FR-006**: Users MUST be able to remove rolling stock entries before submission.
- **FR-007**: When a wishlist is already selected in the sidebar, the wishlist dropdown in the drawer MUST be pre-populated with that wishlist.
- **FR-008**: On successful submission, the system MUST create the railway model in the catalog.
- **FR-009**: On successful submission, the system MUST add the railway model to the selected wishlist with the current date as the added date.
- **FR-010**: On successful submission, the system MUST set the wishlist item status to "WANTED" (default status).
- **FR-011**: The form MUST prevent submission when required fields are incomplete.
- **FR-012**: The "My Wishlists" page styling MUST be updated to match the "My Collection" page style.
- **FR-013**: The side drawer MUST be closable by clicking outside, pressing Escape, or clicking a close button.
- **FR-014**: The "Add railway model" button MUST be disabled or hidden when no wishlists exist.

### Key Entities

- **Railway Model**: Represents a model railway product with manufacturer, product code, description, category, scale, power method, epoch, and associated rolling stocks.
- **Rolling Stock**: A component of a railway model representing a single rail vehicle with railway company, series code, category, and optional road number.
- **Wishlist**: A user-defined list for tracking desired railway models with name and default status.
- **Wishlist Item**: An entry in a wishlist referencing a railway model with priority, status, desired price, and added date.

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: Users can add a new railway model to a wishlist in under 90 seconds (excluding time spent selecting dropdown values).
- **SC-002**: 95% of users complete the form successfully on first attempt when all fields are correctly filled.
- **SC-003**: After submission, the new wishlist item appears in the wishlist view within 2 seconds.
- **SC-004**: Form validation prevents 100% of submissions with missing required fields.
- **SC-005**: The "My Wishlists" page visual styling is consistent with the "My Collection" page (same layout patterns, spacing, and component styles).

## Assumptions

- The application already has manufacturer, railway company, category, scale, and power method reference data available through existing commands.
- The existing `AddRailwayModelToWishListArgs` command structure supports all required fields.
- The user has at least one wishlist to add items to (edge case handles zero wishlists).
- Currency for desired price follows the user's locale or a default (EUR/USD) setting established elsewhere in the application.
- The "My Collection" page serves as the visual reference for consistent styling.
