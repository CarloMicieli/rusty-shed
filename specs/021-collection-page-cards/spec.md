# Feature Specification: Collection Page Card Integration

**Feature Branch**: `021-collection-page-cards`
**Created**: 2026-02-12
**Status**: Draft
**Input**: User description: "Use the new RailwayModelCard and RailwayModelPreviewCard in the collection page"

## User Scenarios & Testing _(mandatory)_

### User Story 1 - View Collection with New Preview Cards (Priority: P1)

Users can browse their railway model collection displayed with the new RailwayModelPreviewCard components, which provide a richer visual presentation with thumbnails, metadata badges, and digital feature indicators.

**Why this priority**: This is the primary visual improvement that delivers immediate value by replacing the existing ItemCard with a more feature-complete preview card. Users will benefit from better visual hierarchy and information density.

**Independent Test**: Can be fully tested by opening the collection page and verifying that models display with thumbnails, metadata badges (scale, era, power method), digital feature overlays (sound, DCC), and unit count indicators. Delivers immediate visual improvement and enhanced information display.

**Acceptance Scenarios**:

1. **Given** a user has models in their collection, **When** they navigate to the collection page, **Then** each model displays using the RailwayModelPreviewCard component with thumbnail, manufacturer, product code, series, road number, and metadata badges
2. **Given** a model has digital features (sound, DCC), **When** the user views the collection, **Then** feature icons display as overlays on the model thumbnail
3. **Given** a model has multiple units, **When** displayed in the collection, **Then** a unit count badge appears on the thumbnail
4. **Given** a model has a long road number, **When** displayed, **Then** the road number truncates with an expand/collapse toggle
5. **Given** a user clicks a model card, **When** the click event fires, **Then** the user navigates to the model detail page (existing behavior preserved)
6. **Given** a user clicks the delete button on a card, **When** the button is clicked, **Then** the delete confirmation dialog appears (existing behavior preserved)

---

### User Story 2 - View Detailed Model Information (Priority: P2)

Users can view comprehensive model details in a modal or expanded view using the RailwayModelCard component, which provides full specifications, rolling stock details, and image management capabilities.

**Why this priority**: This enhances the user experience by providing in-depth information access without leaving the collection page. It's secondary to the grid view improvement but adds significant value for users examining model details.

**Independent Test**: Can be tested by clicking a model card and verifying that a detailed view (modal or page) opens showing the RailwayModelCard with full specifications, tabbed rolling stock information, and image upload functionality for models in the user's collection.

**Acceptance Scenarios**:

1. **Given** a user clicks on a model preview card, **When** the detail view opens, **Then** the RailwayModelCard displays with full specifications (era, power method, category, description)
2. **Given** a model has multiple rolling stock units, **When** viewing details, **Then** tabbed navigation allows switching between model details and rolling stock specifications
3. **Given** a single-unit model is viewed, **When** displayed in detail view, **Then** the card shows unified specifications without tabs
4. **Given** a model belongs to the user's collection, **When** viewing details, **Then** image upload functionality is enabled (editable mode)
5. **Given** a model without an image is viewed in editable mode, **When** displayed, **Then** drag-and-drop and browse buttons appear for image upload
6. **Given** a user uploads an image, **When** upload completes, **Then** the image displays in the model card and persists in the collection

---

### Edge Cases

- What happens when a model has no image (photoUrl is null)? - Display category-specific placeholder icon as defined in RailwayModelPreviewCard
- How does the system handle models with missing metadata (null manufacturer, product code, series)? - Display fallback text using i18n messages (e.g., "Unknown Manufacturer")
- What happens when the collection is empty? - Display existing empty state with "Add First Item" prompt
- How does filtering interact with the new cards? - Existing filter functionality continues to work; filtered items render using RailwayModelPreviewCard
- What happens when a model image fails to load? - Browser's default broken image handling; consider adding onerror handler for graceful fallback to placeholder icon
- How do the cards respond on mobile devices? - RailwayModelPreviewCard includes responsive grid breakpoints (sm, lg); cards stack vertically on small screens
- What happens if a user has edit permissions in detail view but the image upload fails? - Display error message using onError callback and i18n messages; maintain existing image if present

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: System MUST replace the existing ItemCard component with RailwayModelPreviewCard in the collection grid view
- **FR-002**: System MUST map CollectionItemView data to RailwayModelCardData interface required by RailwayModelPreviewCard
- **FR-003**: System MUST preserve existing click behavior to navigate to model detail page when a preview card is clicked
- **FR-004**: System MUST preserve existing delete functionality with confirmation dialog when delete button is clicked on a preview card
- **FR-005**: System MUST display RailwayModelCard component when user accesses detailed model view
- **FR-006**: System MUST determine if a model is editable based on ownership/collection membership and pass editable prop accordingly
- **FR-007**: System MUST provide image upload handlers (onImageUploaded, onError) when RailwayModelCard is in editable mode
- **FR-008**: System MUST refresh collection data after successful image upload to display updated image
- **FR-009**: System MUST map digital features (sound, DCC) from model data to digitalFeatures array for RailwayModelPreviewCard
- **FR-010**: System MUST calculate and display unit count from rolling stock data when multiple units exist
- **FR-011**: System MUST maintain responsive grid layout (existing: sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4) with new preview cards
- **FR-012**: System MUST preserve existing loading skeletons, empty states, and no-results states when using new components

### Key Entities _(include if feature involves data)_

- **RailwayModelCardData**: Data structure required by RailwayModelPreviewCard (id, manufacturer, productCode, series, category, roadNumber, scale, powerMethod, era, purchaseDate, photoUrl, unitCount, digitalFeatures)
- **CollectionItemView**: Existing data structure from backend containing railway model information, added date, notes, and relationship to collection
- **RailwayModel**: Complete model information including specifications, rolling stock, image path, and status
- **DigitalFeature**: Enumeration of digital capabilities ('Sound', 'DCC', 'Smoke', 'Light') for overlay badges
- **ModelCategory**: Classification for placeholder icon selection (SteamLocomotive, ElectricLocomotive, DieselLocomotive, Wagon, PassengerCar, FreightCar, Railcar, TrainSet, Unknown)

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: Users can view all collection models with new preview cards displaying thumbnails, metadata badges, and feature indicators
- **SC-002**: Collection page maintains existing performance characteristics (grid render time, filter responsiveness)
- **SC-003**: Users can access detailed model information through the new RailwayModelCard interface
- **SC-004**: Users with collection ownership can successfully upload and replace model images
- **SC-005**: All existing collection functionality (filtering, search, add, delete, navigation) continues to work without regression
- **SC-006**: New card components display correctly across all viewport sizes (mobile, tablet, desktop)
- **SC-007**: Digital feature badges (sound, DCC) appear on models with those capabilities
- **SC-008**: Multi-unit models display unit count badges accurately

## Assumptions

- RailwayModelCard and RailwayModelPreviewCard components are fully implemented and tested
- TypeScript types for RailwayModel and related interfaces are available from bindings
- Backend commands for image upload (upload_model_image, upload_model_image_bytes) are functional
- Existing navigation to model detail pages is handled by the router (goto function)
- The collection service (collectionService) can provide all necessary data for card population
- Digital features and category information can be derived from existing model data
- Image paths follow the pattern: models/${manufacturer}_${product_code}
- The application uses Paraglide for internationalization (i18n messages available)
