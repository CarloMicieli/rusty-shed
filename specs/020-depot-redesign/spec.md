# Feature Specification: Depot Page Redesign

**Feature Branch**: `020-depot-redesign`
**Created**: 2026-02-12
**Status**: Draft
**Input**: User description: "Redesign depot page with categorized, collapsible rolling stock list view"

## User Scenarios & Testing _(mandatory)_

### User Story 1 - Quick Search for Specific Model (Priority: P1)

As a model railway collector, I need to quickly find a specific locomotive or car in my collection so I can verify details, update information, or locate it physically in my storage.

**Why this priority**: This is the most common use case - users frequently need to look up a specific model by road number, manufacturer, or series. Without fast search, the depot page becomes unusable for collections with 100+ items.

**Independent Test**: Can be fully tested by typing a road number or series name into the search field and verifying the correct model appears within 200ms, delivering immediate value even if other features are incomplete.

**Acceptance Scenarios**:

1. **Given** I have 200+ rolling stock items in my collection, **When** I type "103 113" in the search box, **Then** only items matching that road number appear in the list
2. **Given** I'm viewing filtered search results, **When** I clear the search input, **Then** all rolling stock items reappear organized by category
3. **Given** I type "Roco" in the search, **When** the system filters the list, **Then** all models from manufacturer Roco are shown across all categories
4. **Given** I search for a non-existent item, **When** no matches are found, **Then** all category sections are hidden and I see a clear "no results" state

---

### User Story 2 - Browse by Rolling Stock Type (Priority: P2)

As a collector organizing my physical layout or planning operations, I need to view all locomotives separately from passenger cars and freight cars so I can understand what motive power and rolling stock I have available.

**Why this priority**: Categorization is essential for understanding collection composition, but users can still find items via search (P1) if categories aren't implemented yet. This adds organizational value once basic search works.

**Independent Test**: Can be tested by expanding each category section and verifying the correct models appear in each group (e.g., all steam/diesel/electric locos in "Locomotives" section), delivering value for browsing even without search functionality.

**Acceptance Scenarios**:

1. **Given** I open the depot page, **When** I view the category headers, **Then** I see four sections: Locomotives, Railcars & EMU/DMU, Passenger Cars, and Freight Cars
2. **Given** I have models in each category, **When** I view the category headers, **Then** each header shows a count badge (e.g., "Locomotives (24)")
3. **Given** a category section is expanded, **When** I click the category header, **Then** the section collapses to hide the items
4. **Given** I expand the Locomotives section, **When** I scroll through a long list of items, **Then** the category header remains visible at the top of the viewport (sticky behavior)
5. **Given** I have applied a search filter, **When** a category has zero matching items, **Then** that category section is completely hidden to keep the view compact
6. **Given** I have removed a model from my collection, **When** I view the depot page, **Then** that model does not appear in any category (even if soft-deleted in the database)
7. **Given** I own three identical locomotives (same road number and series), **When** I view the Locomotives category, **Then** all three duplicates are displayed in the list

---

### User Story 3 - View Technical Details at a Glance (Priority: P3)

As a DCC-equipped layout operator, I need to see key technical and identification details (road number, series, era, DCC address) in a scannable table format so I can quickly reference information during operations or when configuring decoders.

**Why this priority**: Detailed information display enhances usability, but users can still search and browse (P1, P2) without this. Table layout is the final polish for power users who need quick reference access.

**Independent Test**: Can be tested by viewing any expanded category and verifying all required columns (Series, Road Number, Manufacturer, Product Code, DCC Address, Era, Livery) are visible and readable, delivering value for detailed reference even if search/categories have limitations.

**Acceptance Scenarios**:

1. **Given** I expand any category section, **When** I view the item list, **Then** I see a table with columns for Series, Road Number, Manufacturer, Product Code, DCC Address, Era, and Livery
2. **Given** I'm viewing a model in the table, **When** I look at the DCC Address column, **Then** I see the decoder address if configured, or an empty/placeholder value if not applicable
3. **Given** I have 50+ items in a category, **When** I scroll through the table, **Then** I can easily scan rows to find specific information without losing context
4. **Given** I'm viewing technical details, **When** I look at secondary information like product codes, **Then** they use muted styling to distinguish from primary information (road number, series)

---

### Edge Cases

- What happens when a rolling stock item doesn't fit neatly into one of the four categories? (e.g., maintenance vehicles, special equipment)
- How does the system handle models with missing data (no road number, no DCC address, unknown era)?
- What happens when searching with special characters or numbers that might appear in road numbers (e.g., "103-7", "103 113-7")?
- How does the interface perform with collections exceeding 1000 items?
- What happens if all categories are empty (new user with no rolling stock)?
- How are soft-deleted models distinguished from active models to ensure they don't appear in the depot?
- What happens when a user owns multiple identical models (same road number, same series) - should they all be displayed?
- How does the system handle models that were added to a collection but later removed - do they remain in the database but excluded from the depot view?

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: System MUST organize rolling stock into four distinct categories: Locomotives, Railcars & EMU/DMU, Passenger Cars, and Freight Cars
- **FR-002**: System MUST display an item count for each category in the category header (e.g., "Locomotives (24)")
- **FR-003**: System MUST provide a global search input that filters across all categories simultaneously
- **FR-004**: System MUST search against the following attributes: Road Number, Series, Manufacturer, and Product Code
- **FR-005**: System MUST debounce search input by 150ms to maintain UI responsiveness
- **FR-006**: System MUST hide category sections that have zero items matching the current search filter
- **FR-007**: System MUST display rolling stock in a table format with the following columns: Series, Road Number, Manufacturer, Product Code, DCC Address, Era, Livery
- **FR-008**: System MUST support expanding and collapsing each category section independently
- **FR-009**: System MUST keep category headers visible when scrolling through long lists (sticky positioning)
- **FR-010**: System MUST display DCC Address only when applicable (blank or placeholder for non-DCC models)
- **FR-011**: System MUST use visual hierarchy to distinguish primary information (road number, series) from secondary information (product codes)
- **FR-012**: System MUST show a clear empty state when search yields no results across all categories
- **FR-013**: System MUST only display rolling stock that is currently owned (part of an active collection)
- **FR-014**: System MUST exclude models that have been removed from the collection, including soft-deleted items
- **FR-015**: System MUST display all duplicate models in the collection without filtering or deduplication

### Categorization Rules

**Locomotives Category**:

- Traction units (powered locomotives)
- Steam, diesel, and electric locomotives
- Switchers/shunters

**Railcars & EMU/DMU Category**:

- Electric Multiple Units (EMU)
- Diesel Multiple Units (DMU)
- Self-propelled passenger units

**Passenger Cars Category**:

- Unpowered passenger coaches
- Dining cars, sleepers, baggage cars
- Observation cars

**Freight Cars Category**:

- All unpowered freight rolling stock
- Box cars, hoppers, gondolas, tank cars, etc.

### Key Entities _(include if feature involves data)_

- **Rolling Stock Item**: Represents a single model in the collection with attributes including series, road number, manufacturer, product code, DCC address (optional), era, livery, and category classification
- **Category**: Logical grouping of rolling stock (Locomotives, Railcars & EMU/DMU, Passenger Cars, Freight Cars) with item count
- **Search Filter**: Active search query applied across multiple attributes to filter the visible items

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: Users can locate a specific model by road number or manufacturer in under 10 seconds
- **SC-002**: Search results appear within 200 milliseconds of the last keystroke
- **SC-003**: Interface remains responsive and usable with collections of 500+ rolling stock items
- **SC-004**: Users can identify the category and count of their rolling stock at a glance without scrolling
- **SC-005**: Category headers remain visible while scrolling through lists of 100+ items in a category
- **SC-006**: Users can distinguish between primary identification details (series, road number) and secondary technical details (product code, DCC address) through visual hierarchy
- **SC-007**: 90% of users successfully find a specific model on their first search attempt

## Assumptions

- Rolling stock items already have sufficient metadata (series, road number, manufacturer, etc.) stored in the database
- The existing data model includes a field or property that can be used to categorize items into the four specified categories
- The database has an ownership relationship linking rolling stock to collections, allowing filtering by "owned" status
- The system uses soft-delete functionality (items marked as deleted but not physically removed from database)
- The data model includes a way to identify active vs. removed/deleted collection items
- Era and livery information is available for most rolling stock items
- DCC addresses are stored separately and may not be present for all items (analog models)
- Users may legitimately own multiple identical models (duplicates are valid and should be shown)
- Users understand model railway terminology (EMU, DMU, road number, series, era)
- The depot page currently exists and needs to be redesigned (not built from scratch)
- Search is case-insensitive for better user experience
- The "industrial aesthetic" refers to a clean, professional design suitable for technical/hobbyist users

## Dependencies

- Existing rolling stock data in the database
- UI component library (shadcn-svelte Accordion components)
- Icon library (lucide-svelte for category icons)
- Existing table or list rendering infrastructure

## Out of Scope

- Editing rolling stock details from the depot page (viewing only)
- Sorting options (may be added in future iteration)
- Advanced filters beyond text search (e.g., filter by era, filter by DCC-equipped only)
- Export functionality
- Bulk operations on rolling stock
- Mobile-optimized view (desktop-first)
- Adding new rolling stock from this page
