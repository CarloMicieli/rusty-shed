# Feature Specification: Dashboard Collector's Overview Redesign

**Feature Branch**: `017-dashboard-redesign`  
**Created**: February 9, 2026  
**Status**: Draft  
**Input**: User description: "To transition your dashboard from a technical 'data-dump' feel to a curated collector's overview, the redesign focuses on grouping and visual hierarchy."

## User Scenarios & Testing _(mandatory)_

### User Story 1 - View Recent Acquisitions Grouped by Purchase Event (Priority: P1)

As a railway model collector, I want to see my recent acquisitions organized by when and where I purchased them, so I can quickly recall and appreciate my latest additions without being overwhelmed by a data table.

**Why this priority**: This is the core transformation from data-dump to collector's overview. It delivers immediate visual value and contextual understanding of the collection.

**Independent Test**: Can be fully tested by adding 2-3 models from different purchase events and verifying they appear grouped chronologically with purchase metadata (date, source, notes).

**Acceptance Scenarios**:

1. **Given** I have acquired models from 3 different purchase events in the past month, **When** I open the dashboard, **Then** I see 2-3 most recent purchase groups displayed chronologically (newest first)
2. **Given** a purchase group contains metadata (date, seller name, personal notes), **When** viewing the dashboard, **Then** all contextual information is clearly visible at the group level
3. **Given** I purchased 5 models in a single transaction, **When** viewing that purchase group on the dashboard, **Then** I see the first 3 models with a "+2 more items" indicator
4. **Given** I want to understand a purchase context, **When** viewing a purchase group, **Then** I can read personal notes like "Birthday gift from Maria" or "Found at Milan swap meet"

---

### User Story 2 - Quick Visual Recognition of Individual Models (Priority: P1)

As a collector browsing my dashboard, I want to immediately recognize each model through its thumbnail image, manufacturer, and condition status, so I can appreciate my collection visually rather than reading technical data.

**Why this priority**: Visual recognition is essential for the "collector's overview" experience. Without this, the dashboard remains a data table.

**Independent Test**: Can be tested independently by viewing any single model card and verifying it displays thumbnail, manufacturer, product code, condition badge, and truncated description.

**Acceptance Scenarios**:

1. **Given** a model has an uploaded thumbnail image, **When** viewing it in the dashboard, **Then** the image is prominently displayed as the primary visual element
2. **Given** each model has manufacturer and product code information, **When** viewing a model card, **Then** both brand (e.g., "Roco") and product code are clearly visible
3. **Given** a model is marked as "New" or "Pre-owned", **When** viewing the card, **Then** I see a high-contrast visual badge indicating the condition
4. **Given** a model has a long descriptive name, **When** displayed on a card, **Then** the description is truncated gracefully without breaking the grid layout
5. **Given** multiple models are displayed, **When** viewing the dashboard, **Then** all cards maintain consistent height and spacing

---

### User Story 3 - Navigate to Full Model Details (Priority: P2)

As a collector who wants more information about a specific model, I want to click on any model card to view its complete specification page, so I can access detailed information without leaving the dashboard workflow.

**Why this priority**: Provides necessary depth while maintaining the lightweight dashboard experience. Essential for usability but not required for initial visual value.

**Independent Test**: Can be tested by clicking any model card and verifying navigation to the model's full specification page.

**Acceptance Scenarios**:

1. **Given** I see a model card on the dashboard, **When** I click on it, **Then** I am navigated to that model's full specification page
2. **Given** I am viewing model details, **When** I navigate back, **Then** I return to the dashboard in the same scroll position

---

### User Story 4 - Access Complete Collection Inventory (Priority: P3)

As a collector who needs to search or manage my entire collection, I want a clear way to navigate from the dashboard overview to the full collection table, so I can perform detailed inventory management tasks.

**Why this priority**: Important for power users but not essential for the core "collector's overview" experience. The dashboard focuses on recent activity, not comprehensive inventory management.

**Independent Test**: Can be tested by clicking the "View All" link and verifying navigation to the full Depot/Collection view with all models.

**Acceptance Scenarios**:

1. **Given** I want to see all 500+ models in my collection, **When** I click "View All" or similar navigation element, **Then** I am taken to the full Collection/Depot table view
2. **Given** I am in the full collection view, **When** I want to return to the overview, **Then** there is clear navigation back to the dashboard

---

### Edge Cases

- What happens when a model has no thumbnail image? (Display placeholder with manufacturer logo or generic model icon)
- What happens when a purchase has only 1 model? (Display as a single-item group, no "+X more" indicator)
- What happens when a purchase has no source/seller information? (Display date and notes only, with "Unknown source" if completely empty)
- What happens when there are no recent purchases? (Display empty state message: "No recent additions" with link to add models)
- How does the dashboard handle very long seller names or notes? (Truncate with ellipsis, show full text on hover)
- What happens if a user has 50 models in a single purchase? (Show first 3 with "+47 more items" counter)
- What happens when viewing on mobile devices? (Cards stack vertically, maintain visual hierarchy with responsive breakpoints)

## Requirements _(mandatory)_

### Functional Requirements

#### Purchase Grouping

- **FR-001**: System MUST display models grouped by their acquisition/purchase event rather than as individual entries
- **FR-002**: System MUST sort purchase groups chronologically with most recent acquisitions displayed first
- **FR-003**: System MUST limit the dashboard view to the 2-3 most recent purchase events to maintain consistent viewport height

#### Purchase Group Metadata

- **FR-004**: Each purchase group MUST display the acquisition date in a human-readable format (e.g., "January 15, 2026")
- **FR-005**: Each purchase group MUST display the seller/shop name when this information is available
- **FR-006**: Each purchase group MUST display any user-provided notes about the transaction (e.g., "Birthday gift from Maria")
- **FR-007**: System MUST show a counter (e.g., "+2 more items") when a purchase contains more than 3 models

#### Model Card Visual Design

- **FR-008**: Each model card MUST display a thumbnail image in 16:9 aspect ratio (recommended minimum width: 160px) as the primary visual element
- **FR-009**: Each model card MUST clearly display the manufacturer name (e.g., Roco, Piko)
- **FR-010**: Each model card MUST display the manufacturer's product code
- **FR-011**: Each model card MUST display a high-contrast visual badge indicating condition status (New vs Pre-owned)
- **FR-012**: Each model card MUST display a truncated description that maintains grid consistency
- **FR-013**: System MUST truncate long descriptions to 100 characters maximum using CSS line-clamp-2 for graceful multi-line truncation with ellipsis

#### Navigation & Interaction

- **FR-014**: Each model card MUST act as a clickable link to that model's full specification page
- **FR-015**: Dashboard MUST provide a "View All" or similar link to navigate to the complete Collection/Depot table view
- **FR-016**: System MUST maintain user's scroll position when navigating back from model details

#### Fallback Handling

- **FR-017**: System MUST display a placeholder image when a model has no uploaded thumbnail
- **FR-018**: System MUST handle missing purchase metadata gracefully (show available information only)
- **FR-019**: System MUST display an appropriate empty state when no models exist or no recent purchases

### Key Entities

- **Purchase Event**: Represents a single acquisition transaction containing one or more models. Attributes include: acquisition date, seller/shop name, user notes, list of associated models.
- **Model Card**: Visual summary representation of an individual railway model. Attributes include: thumbnail image, manufacturer name, product code, condition status (New/Pre-owned), description/name, link to full specification.
- **Dashboard View**: The curated overview page displaying recent purchase events, distinct from the comprehensive Collection/Depot table view used for inventory management.

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: Users can recognize and appreciate their 3 most recent acquisitions within 5 seconds of opening the dashboard (visual scanning time)
- **SC-002**: Dashboard maintains consistent viewport height regardless of purchase group sizes (no excessive scrolling required for overview)
- **SC-003**: Users can identify a model's manufacturer, condition, and product code without clicking through to details (information density test)
- **SC-004**: 90% of collectors prefer the grouped overview format over the previous data-table approach (user satisfaction survey)
- **SC-005**: Model cards display correctly on viewports ranging from 320px to 2560px width (responsive design test)
- **SC-006**: Users can navigate from dashboard overview to full collection view and back within 2 clicks
- **SC-007**: Page load time for dashboard with 10 purchase groups (30 visible models) remains under 2 seconds (performance benchmark)

## Assumptions

1. **Data Availability**: All models already have purchase date information stored. If not available, system will default to the date the model was added to the database.
2. **Image Storage**: Model thumbnail images are already stored and accessible. The system uses existing image infrastructure.
3. **Mobile-First**: The redesign assumes responsive design is required, with mobile devices being a significant use case.
4. **Seller Data Optional**: Seller/shop information is optional metadata; many models may not have this information.
5. **Standard Grid Layout**: The dashboard uses a responsive grid system (likely CSS Grid or Flexbox) for card layout, assuming modern browser support.
6. **Existing Full Collection View**: A separate "Collection" or "Depot" view already exists for comprehensive inventory management; this dashboard is supplementary.
7. **Purchase Event Grouping Logic**: Models are considered part of the same purchase event if they share the same acquisition date and seller information.
8. **Default Display Count**: Showing 2-3 purchase groups is based on typical screen heights (1080p to 1440p displays) to avoid scrolling.
9. **Condition Status**: All models have a condition field populated (New/Pre-owned/Used). If missing, defaults to "Unknown" badge style.
10. **Navigation Patterns**: Users are familiar with card-based navigation patterns common in modern web applications.

## Out of Scope

- Editing purchase event metadata directly from the dashboard (use full collection view for data management)
- Filtering or searching within the dashboard view (dashboard shows only recent additions, not searchable inventory)
- Bulk operations on models from the dashboard (export, delete, tag operations remain in full collection view)
- Detailed statistics or analytics (e.g., spending by manufacturer, collection growth charts)
- Comparison features between models
- Wishlist or "want to buy" integration with the dashboard view
- Print/PDF export of the dashboard layout
- Custom dashboard layouts or user-configurable widgets
- Real-time collaboration or sharing of dashboard views with other users
- Integration with external pricing databases or market values
- Maintenance tracking or service history display on dashboard cards
- Multi-language support for model descriptions (existing i18n for UI elements only)
