# Feature Specification: Track Inventory Management

**Feature Branch**: `005-track-inventory`  
**Created**: 2026-01-30  
**Status**: Draft  
**Input**: User description: "I want to build a track inventory feature in the app with a My Tracks menu entry allowing users to manage multiple track inventories, purchases, and track products."

## User Scenarios & Testing _(mandatory)_

### User Story 1 - View Track Inventory (Priority: P1)

As a model railway collector, I want to view my track inventory so that I can see what track pieces I have in stock and what quantities I need.

**Why this priority**: This is the core read operation that users will perform most frequently. Without visibility into current stock and requirements, users cannot make informed purchasing decisions.

**Independent Test**: Can be fully tested by navigating to "My Tracks", selecting an inventory, and viewing the track type list with quantities - delivers immediate value by showing inventory status.

**Acceptance Scenarios**:

1. **Given** I have at least one track inventory, **When** I navigate to the "My Tracks" page, **Then** I see a list of all my track inventories with their names and descriptions.
2. **Given** I am viewing a track inventory, **When** I look at the inventory details, **Then** I see each track type with its current quantity in stock.
3. **Given** I am viewing a track inventory, **When** I look at a track type entry, **Then** I see the number of pieces required alongside the current stock quantity.
4. **Given** I have no track inventories yet, **When** I navigate to the "My Tracks" page, **Then** I see an empty state with guidance on how to create my first inventory.

---

### User Story 2 - Create and Manage Track Inventories (Priority: P1)

As a model railway collector, I want to create and manage multiple track inventories so that I can organize different layout projects or track collections separately.

**Why this priority**: Users need to create inventories before they can add purchases or view stock. This is foundational functionality required for all other features.

**Independent Test**: Can be fully tested by creating a new inventory with name and description, then verifying it appears in the inventory list.

**Acceptance Scenarios**:

1. **Given** I am on the "My Tracks" page, **When** I choose to create a new inventory and provide a name, **Then** a new empty track inventory is created and displayed.
2. **Given** I am creating a new inventory, **When** I optionally provide a description, **Then** the description is saved with the inventory.
3. **Given** I have an existing inventory, **When** I edit its name or description, **Then** the changes are saved and reflected in the interface.
4. **Given** I have an existing inventory, **When** I delete it, **Then** the inventory and all its associated purchases are removed.

---

### User Story 3 - Add Track Purchase (Priority: P1)

As a model railway collector, I want to add new track purchases to my inventory so that the stock quantities are automatically updated.

**Why this priority**: The primary method of updating inventory stock is through purchases. Without this, users cannot track what they've bought or maintain accurate stock levels.

**Independent Test**: Can be fully tested by adding a purchase with one or more track products and verifying the inventory quantities update correctly.

**Acceptance Scenarios**:

1. **Given** I am viewing an inventory, **When** I add a new purchase with track products and quantities, **Then** the purchase is recorded and inventory quantities are updated.
2. **Given** I am adding a purchase, **When** I add multiple different track products to the same purchase, **Then** all products are recorded with their respective quantities.
3. **Given** I am adding a purchase, **When** I enter the total price and select a seller, **Then** this information is saved with the purchase.
4. **Given** I add a purchase with quantity 5 of a track type where I had 10 in stock, **When** the purchase is saved, **Then** the stock quantity for that track type shows 15.

---

### User Story 4 - View Purchase History (Priority: P2)

As a model railway collector, I want to view the purchase history for each inventory so that I can track what I bought, when, from whom, and at what price.

**Why this priority**: While valuable for tracking spending and purchase sources, users can function with just current stock visibility. This enhances the experience but isn't essential for basic inventory management.

**Independent Test**: Can be fully tested by adding purchases and then viewing the purchase history list with all recorded details.

**Acceptance Scenarios**:

1. **Given** I am viewing an inventory, **When** I access the purchase history, **Then** I see a chronological list of all purchases.
2. **Given** I am viewing a purchase in the history, **When** I look at its details, **Then** I see all track products purchased, their quantities, the total price, and the seller.
3. **Given** I am viewing purchase history, **When** a purchase contains multiple track products, **Then** all products are displayed grouped under that purchase.

---

### User Story 5 - Manage Track Products (Priority: P2)

As a model railway collector, I want to define track products that I can reference when adding purchases so that I have consistent product information across my inventories.

**Why this priority**: Track products provide the catalog from which purchases are made. While essential, users could start with a pre-populated or simple product list initially.

**Independent Test**: Can be fully tested by creating track products with all attributes and then using them when adding purchases.

**Acceptance Scenarios**:

1. **Given** I am adding a track product, **When** I enter manufacturer, product code, description, length, and track code, **Then** the product is saved and available for purchases.
2. **Given** I am adding a track product, **When** I specify whether it has a roadbed, **Then** this attribute is recorded with the product.
3. **Given** I have defined track products, **When** I add a purchase, **Then** I can select from existing track products.

---

### User Story 6 - Set Required Quantities (Priority: P3)

As a model railway collector, I want to set the number of pieces required for each track type so that I can see at a glance what I still need to purchase.

**Why this priority**: This is a planning feature that enhances inventory visibility but isn't required for basic stock tracking. Users can manually compare quantities without this feature.

**Independent Test**: Can be fully tested by setting required quantities for track types and verifying they display correctly alongside stock quantities.

**Acceptance Scenarios**:

1. **Given** I am viewing an inventory, **When** I set a required quantity for a track type, **Then** this quantity is saved and displayed.
2. **Given** I have set required quantities, **When** I view the inventory, **Then** I can easily compare current stock vs required amounts.
3. **Given** stock quantity is less than required, **When** I view the inventory, **Then** the shortage is visually indicated.

---

### Edge Cases

- What happens when a user deletes an inventory that has purchases? All associated purchases must be deleted as well (with confirmation).
- How does the system handle adding a purchase for a track product that doesn't exist yet? The user should be able to create new products inline during purchase entry.
- What happens when a seller (shop or collector) is deleted? Existing purchases should retain the seller reference or show as "Unknown Seller".
- How does the system handle duplicate product codes from the same manufacturer? The combination of manufacturer + product code should be unique.

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: System MUST allow users to create multiple track inventories, each with a name and optional description.
- **FR-002**: System MUST display all track inventories on the "My Tracks" page with their name and description.
- **FR-003**: System MUST allow users to edit the name and description of existing inventories.
- **FR-004**: System MUST allow users to delete inventories (with all associated purchases).
- **FR-005**: System MUST display the current quantity in stock for each track type within an inventory.
- **FR-006**: System MUST display the required quantity for each track type within an inventory.
- **FR-007**: System MUST allow users to set/update required quantities for track types.
- **FR-008**: System MUST allow users to add purchases to an inventory.
- **FR-009**: System MUST support multiple track products within a single purchase, each with its own quantity.
- **FR-010**: System MUST record the total price for each purchase.
- **FR-011**: System MUST record the seller (shop or collector) for each purchase.
- **FR-012**: System MUST automatically update inventory quantities when a purchase is added.
- **FR-013**: System MUST display purchase history for each inventory in chronological order.
- **FR-014**: System MUST allow users to define track products with: manufacturer, product code, description, roadbed flag, length, and track code.
- **FR-015**: System MUST enforce uniqueness of manufacturer + product code combination for track products.
- **FR-016**: System MUST provide a track code enumeration for standardized track classification.
- **FR-017**: System MUST add a "My Tracks" entry to the application navigation menu.
- **FR-018**: System MUST persist all track inventory data locally.

### Key Entities

- **Track Inventory**: A named collection representing a user's track stock for a specific purpose (e.g., a layout project). Contains a name, optional description, associated purchases, and aggregated stock quantities by track type.

- **Track Purchase**: A transaction record capturing the acquisition of track products. Contains one or more track product items with quantities, a total price, a seller reference, and a purchase date.

- **Track Purchase Item**: A line item within a purchase, linking a specific track product to the quantity purchased.

- **Track Product**: A catalog entry defining a specific track piece. Contains manufacturer, product code, description, roadbed indicator, length measurement, and track code classification.

- **Track Code**: An enumeration of standard track types/configurations (e.g., straight, curved, turnout, crossing, etc.).

- **Seller**: A reference to either a shop or another collector from whom track was purchased (leverages existing seller infrastructure in the app).

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: Users can create a new track inventory and add their first purchase in under 3 minutes.
- **SC-002**: Users can view their complete track inventory status (all track types with stock vs required) at a glance on a single screen.
- **SC-003**: Adding a multi-product purchase updates all relevant inventory quantities immediately upon save.
- **SC-004**: Users can access their complete purchase history within 2 interactions from the main navigation.
- **SC-005**: The track inventory feature is accessible via the main navigation menu alongside other "My [X]" features.
- **SC-006**: All track inventory data persists across application restarts without data loss.
- **SC-007**: Users can identify track shortages (stock < required) within 1 second of viewing an inventory.
