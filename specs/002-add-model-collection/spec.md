# Feature Specification: Add Railway Model to Collection

**Feature Branch**: `002-add-model-collection`  
**Created**: 2026-01-30  
**Status**: Draft  
**Input**: User description: "Add railway model to collection - Enhanced form for adding railway models with detailed model information and purchase data"

## User Scenarios & Testing _(mandatory)_

### User Story 1 - Add a Complete Railway Model to Collection (Priority: P1)

As a collector, I want to add a new railway model to my collection with all its details (manufacturer, product code, description, category, scale, power method, epoch, and rolling stocks) so that I can accurately track what I own.

**Why this priority**: This is the core functionality of the feature. Without the ability to add railway model details, the feature has no value. Collectors need to record detailed information about their models.

**Independent Test**: Can be fully tested by navigating to My Collection, clicking "Add railway model", filling in the model details form with at least one rolling stock, and confirming the model appears in the collection with all entered data.

**Acceptance Scenarios**:

1. **Given** the user is on the "My Collection" page, **When** they click the "Add railway model" button, **Then** a side panel dialog slides in from the right side of the screen displaying the railway model form.

2. **Given** the add railway model dialog is open, **When** the user selects a manufacturer from the dropdown, **Then** the system displays all available manufacturers for selection.

3. **Given** the user is filling out the railway model form, **When** they enter the product code, description, select category, scale, power method, and epoch, **Then** all fields accept and display the entered/selected values correctly.

4. **Given** the user is on the railway model form, **When** they click "Add rolling stock", **Then** a new rolling stock entry appears where they can specify railway company, series code, category, and optional road number.

5. **Given** the user has added multiple rolling stocks, **When** they click the remove button on a rolling stock entry, **Then** that rolling stock is removed from the list.

6. **Given** the user has completed all required railway model fields with at least one rolling stock, **When** they confirm/save the form, **Then** the railway model is created in the catalog and added to their collection.

---

### User Story 2 - Record Purchase Information (Priority: P2)

As a collector, I want to record purchase details when adding a model to my collection (seller, price, conditions, notes) so that I can track acquisition history and model/box conditions.

**Why this priority**: Purchase information enhances the collection data but the model can still be added without it. This is valuable for tracking investment and condition history but secondary to core model data.

**Independent Test**: Can be fully tested by adding a railway model and filling in the purchase section with seller, price, and condition ratings, then verifying all purchase data is saved and displayed correctly.

**Acceptance Scenarios**:

1. **Given** the add railway model dialog is open, **When** the user navigates to the purchase information section, **Then** they see fields for seller, purchase price, purchase condition, model condition, box condition, and notes.

2. **Given** the user is entering purchase information, **When** they select a seller from the dropdown, **Then** the system displays all available sellers for selection.

3. **Given** the user has entered purchase price and optionally other purchase fields, **When** they save the railway model, **Then** all purchase information is stored with the collection entry.

4. **Given** the user does not enter any purchase information, **When** they save the railway model, **Then** the model is still added to the collection successfully with empty purchase fields.

---

### User Story 3 - Manage Multiple Rolling Stocks (Priority: P3)

As a collector, I want to add multiple rolling stocks to a single railway model and manage them (add/remove) dynamically so that I can accurately represent train sets or multi-piece models.

**Why this priority**: Many railway models include multiple rolling stocks (locomotives with coaches, freight sets, etc.). This is essential for accurate data representation but can be simplified to single rolling stock for MVP.

**Independent Test**: Can be fully tested by adding a railway model with 3+ rolling stocks, removing one, adding another, and verifying the final rolling stock list matches the expected entries.

**Acceptance Scenarios**:

1. **Given** the user is on the add railway model form, **When** they add multiple rolling stocks (e.g., 5 coaches), **Then** all rolling stocks are listed in the form with their individual details.

2. **Given** the user has multiple rolling stocks in the form, **When** they modify the railway company or other details for one rolling stock, **Then** only that specific rolling stock is updated.

3. **Given** the user has saved a railway model with multiple rolling stocks, **When** they view the model in their collection, **Then** all rolling stocks are displayed with their respective details.

---

### Edge Cases

- What happens when the user tries to save without selecting a manufacturer? → The form should prevent submission and highlight the required field.
- What happens when the user tries to save without entering a product code? → The form should prevent submission and highlight the required field.
- What happens when the user enters a duplicate product code for the same manufacturer? → The system should warn the user about the potential duplicate.
- How does the system handle when no rolling stocks are added? → The system should require at least one rolling stock before saving.
- What happens when a dropdown list (e.g., manufacturers) is empty? → The system should display an appropriate message and potentially allow adding a new entry.
- What happens when the user closes the dialog without saving? → The system should prompt the user to confirm discarding unsaved changes.

## Requirements _(mandatory)_

### Functional Requirements

#### Railway Model Data

- **FR-001**: System MUST display a side panel dialog when the user clicks "Add railway model" button from the My Collection page.
- **FR-002**: System MUST provide a manufacturer dropdown populated with all available manufacturers.
- **FR-003**: System MUST allow users to enter a product code as free text (required field).
- **FR-004**: System MUST allow users to enter a short description as free text (required field).
- **FR-005**: System MUST provide a category dropdown populated with all available categories.
- **FR-006**: System MUST provide a scale dropdown populated with available scales (e.g., H0, N, TT, Z, O, G).
- **FR-007**: System MUST provide a power method dropdown populated with available power methods (AC, DC, TRIX_EXPRESS).
- **FR-008**: System MUST allow users to specify the epoch (e.g., I, II, III, IV, V, VI).

#### Rolling Stock Management

- **FR-009**: System MUST allow users to add one or more rolling stocks to the railway model.
- **FR-010**: System MUST require at least one rolling stock before the railway model can be saved.
- **FR-011**: System MUST provide a railway company dropdown for each rolling stock populated with available railway companies.
- **FR-012**: System MUST allow users to enter a series code for each rolling stock (required field).
- **FR-013**: System MUST allow users to specify a category for each rolling stock.
- **FR-014**: System MUST allow users to optionally enter a road number for each rolling stock.
- **FR-015**: System MUST allow users to remove any rolling stock from the list.

#### Purchase Information

- **FR-016**: System MUST provide an optional seller dropdown populated with available sellers.
- **FR-017**: System MUST allow users to enter a purchase price (optional).
- **FR-018**: System MUST allow users to select purchase condition (optional: NEW, PRE_OWNED).
- **FR-019**: System MUST allow users to select model condition (optional: MINT, NEAR_MINT, EXCELLENT, VERY_GOOD, GOOD, FAIR, POOR, FOR_PARTS).
- **FR-020**: System MUST allow users to select box condition (optional: ORIGINAL_MINT, ORIGINAL_GOOD, ORIGINAL_WORN, REPLACEMENT_BOX, NO_BOX).
- **FR-021**: System MUST allow users to enter free-form notes (optional).

#### Data Persistence

- **FR-022**: System MUST create a new railway model entry in the catalog when the form is saved.
- **FR-023**: System MUST add the railway model to the user's collection when the form is saved.
- **FR-024**: System MUST persist all rolling stocks associated with the railway model.
- **FR-025**: System MUST persist all purchase information if provided.

#### Form Behavior

- **FR-026**: System MUST validate all required fields before allowing form submission.
- **FR-027**: System MUST display validation errors clearly next to the relevant fields.
- **FR-028**: System MUST prompt the user before discarding unsaved changes when closing the dialog.

### Key Entities

- **Railway Model**: The main catalog entry representing a model product. Contains manufacturer reference, product code, description, category, scale, power method, and epoch.
- **Rolling Stock**: A component within a railway model (locomotive, coach, wagon, etc.). Contains railway company reference, series code, category, and optional road number. A railway model has one or more rolling stocks.
- **Collection Item**: An entry in the user's personal collection linking to a railway model with purchase information (seller, price, conditions, notes).
- **Manufacturer**: A producer of railway models (e.g., Märklin, Roco, Fleischmann).
- **Railway Company**: An operator whose livery/branding appears on rolling stock (e.g., DB, SNCF, FS, SBB).
- **Seller**: A vendor from whom the model was purchased.

## Assumptions

- The system already has reference data for manufacturers, categories, scales, power methods, railway companies, and sellers that can be loaded into dropdowns.
- Epoch values follow standard European railway epoch classifications (I through VI).
- The "My Collection" page already exists and has a location for an "Add railway model" button.
- The existing popup functionality will be replaced or enhanced by this new side panel dialog approach.

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: Users can complete the full railway model entry (with one rolling stock and no purchase info) in under 2 minutes.
- **SC-002**: Users can add a railway model with 5 rolling stocks in under 4 minutes.
- **SC-003**: 95% of form submissions succeed without validation errors on second attempt (users correct errors after first feedback).
- **SC-004**: All required dropdown fields display their options within 1 second of user interaction.
- **SC-005**: After saving, the new railway model appears immediately in the collection view without requiring page refresh.
- **SC-006**: Users can abandon form entry without data loss by being prompted to confirm discard.
