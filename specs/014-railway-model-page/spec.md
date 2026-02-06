# Feature Specification: Railway Model Details Page

**Feature Branch**: `014-railway-model-page`  
**Created**: February 6, 2026  
**Status**: Draft  
**Input**: User description: "Add a new page to show the railway model information. The app should open this page every time the user is clicking on a railway model card."

---

## Implementation Scope

### Backend: Media Module Creation (Prerequisite)

Creating a new **media** feature module in the Rust backend to handle image management. This is a prerequisite for the details page image functionality.

**Scope**:

- Move existing `get_image_path` command from `lib.rs` to structured `media` module
- Implement DDD architecture with application, domain, infrastructure, and interface layers
- Add fallback placeholder generation (HTML/CSS) when no image available
- Validate filesystem paths to prevent traversal attacks

**Artifacts**:

- Implementation plan: [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md)
- Tech stack reference: [TECH_STACK.md](TECH_STACK.md)

### Frontend: Railway Model Details Page (Feature Story)

Building the UI page that displays comprehensive model information organized in tabs.

**Scope**:

- Create route `/models/[modelId]/+page.svelte`
- Implement header with title, subtitle, hero image, and quick badges
- Organize content into "Details" and "Rolling Stock" tabs
- Create expandable rolling stock cards with full unit information
- Use shadcn-svelte components and Tailwind CSS
- Ensure 100% Paraglide-JS localization

**Implementation**: See [User Scenarios](#user-scenarios--testing-mandatory) below

---

## User Scenarios & Testing _(mandatory)_

### User Story 1 - View Railway Model Details (Priority: P1)

A collector clicks on a railway model card in their collection and wants to see all detailed information about that specific model in a dedicated page view. This is the core user journey where the model details page is opened and displays comprehensive information organized in a clear, scannable format.

**Why this priority**: This is the foundational feature. Without the ability to view model details, the page serves no purpose. Every other interaction depends on this working correctly.

**Independent Test**: Can be fully tested by clicking on any railway model card in the collection view and verifying the details page loads with all expected sections (header, tabs, content).

**Acceptance Scenarios**:

1. **Given** user is viewing the collection, **When** user clicks on a railway model card, **Then** the model details page opens and displays the correct model information
2. **Given** the details page is open, **When** page loads, **Then** the header section displays model description, manufacturer, and product code prominently
3. **Given** the details page is open, **When** page loads, **Then** a hero image displays (actual image if available, placeholder otherwise)
4. **Given** the details page is open, **When** page loads, **Then** quick badges show scale, era, and power method for at-a-glance identification

---

### User Story 2 - Navigate Model Information via Tabs (Priority: P1)

A collector wants to view different aspects of the model (detailed description vs. rolling stock inventory) without excessive scrolling. The page is split into tabs allowing the user to focus on one information category at a time.

**Why this priority**: This is equally critical to the base feature. The tab organization prevents cognitive overload and allows users to find relevant information quickly. Without tabs, collectors would face an excessively long page.

**Independent Test**: Can be fully tested by opening a model details page and clicking through tabs to verify each tab displays appropriate content without page reload.

**Acceptance Scenarios**:

1. **Given** details page is open, **When** user views the page, **Then** two distinct tabs are visible: "Details" and "Rolling Stock"
2. **Given** "Details" tab is active, **When** page displays, **Then** detailed description of the model is shown
3. **Given** "Rolling Stock" tab is active, **When** page displays, **Then** a list of rolling stock units is shown
4. **Given** user switches between tabs, **When** tab content changes, **Then** the previously selected tab state is maintained when switching back

---

### User Story 3 - Explore Individual Rolling Stock Units (Priority: P1)

A collector has multiple units of the same model and wants to inspect each one individually (e.g., multiple locomotives in the same class). Each unit is presented in an expandable card showing type, road number, and can be expanded to reveal additional details like depot, series code, railway company, country, livery, and technical specifications.

**Why this priority**: Rolling stock is a core concept in the railway modeling domain. Collectors track individual units with unique attributes. This feature directly addresses the user's need to manage and view multiple units of the same model class.

**Independent Test**: Can be fully tested by opening a model details page, navigating to the Rolling Stock tab, and expanding/collapsing individual unit cards to verify all information displays correctly.

**Acceptance Scenarios**:

1. **Given** Rolling Stock tab is active, **When** page displays, **Then** each rolling stock unit appears as a collapsed card
2. **Given** a rolling stock card is collapsed, **When** user clicks the card, **Then** it expands to reveal full details
3. **Given** a rolling stock card is expanded, **When** page displays, **Then** all unit details are visible: type, road number, depot, series code, railway company, country, livery, and technical specs
4. **Given** multiple cards are displayed, **When** user expands one card, **Then** other cards remain in their previous state (not auto-closed)
5. **Given** a rolling stock card header is displayed, **When** the card is collapsed, **Then** card header shows "{type} — {road_number}" (e.g., "Locomotive — 218 217-8") for quick identification

---

### Edge Cases

- What happens when a model has no hero image? (Display HTML/CSS placeholder)
- What happens when a model has no rolling stock units? (Display empty state message in Rolling Stock tab)
- What happens when rolling stock unit is missing optional fields (depot, livery, etc.)? (Omit the field from display)
- What happens when the technical specifications grid is very long? (Use responsive layout that wraps on smaller screens)
- What happens when a user navigates to the page directly via URL? (Page should load the specified model if valid)

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: System MUST display a dedicated page route (`/models/{modelId}`) that opens when a user clicks a railway model card
- **FR-002**: The page header MUST display the model's description as the primary title
- **FR-003**: The page header MUST display manufacturer name and product code as a subtitle
- **FR-004**: The page MUST display a hero image showing the railway model; if no image is available, display an HTML/CSS placeholder with appropriate styling
- **FR-005**: The page header MUST display four quick-access badges: scale, era, and power method
- **FR-006**: System MUST organize model information into two tabs: "Details" (description content) and "Rolling Stock" (inventory list)
- **FR-007**: The Details tab MUST display the full detailed description of the model
- **FR-008**: The Rolling Stock tab MUST display all owned rolling stock units associated with the model as expandable cards
- **FR-009**: Each rolling stock card header MUST display the format "{type} — {road_number}" (e.g., "Locomotive — 218 217-8")
- **FR-010**: When a rolling stock card is expanded, the card body MUST display: type, road number, depot, series code, railway company, country, livery, and technical specifications
- **FR-011**: Technical specifications MUST be displayed in a responsive grid/table layout
- **FR-012**: Users MUST be able to expand and collapse rolling stock cards independently
- **FR-013**: System MUST preserve tab selection when navigating away and back to the same model
- **FR-014**: System MUST handle missing optional fields gracefully (omit from display rather than showing "N/A" or empty values)
- **FR-015**: System MUST display an empty state message when a model has no rolling stock units
- **FR-016**: All user-facing text MUST use Paraglide-JS i18n (no hardcoded strings)

### Key Entities _(include if feature involves data)_

- **RailwayModel (Box)**: Represents a single product/box, identified by manufacturer and product code. Key attributes: description, manufacturer, product code, scale, era, power method, image, detailed description. No data schema changes required; existing database tables used.
- **RollingStock**: Individual units owned by the collector belonging to a specific railway model. Key attributes: type, road number, depot, series code, railway company, country, livery, technical specifications. Existing `owned_rolling_stocks` table provides the foundation.
- **TechnicalSpecification**: Key-value pairs describing technical details of a rolling stock unit (e.g., "Motor Type", "Decoder Compatibility", "Max Speed"). Displayed in a responsive table/grid format.

### Assumptions

- The app already has routing infrastructure to support dynamic `/models/{modelId}` routes
- Railway model data (description, manufacturer, product code, scale, era, power method, image) is available from existing database/API
- Rolling stock data is available from the `owned_rolling_stocks` table with all required attributes
- Users have at least one railway model in their collection (happy path); empty collection is handled elsewhere
- No real-time updates required; page loads data once on mount
- Placeholder image can be generated using HTML/CSS (gradient or icon-based design)

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: Users can navigate from a railway model card to its details page in under 1 second (page load and display time)
- **SC-002**: All railway model details page elements (header, tabs, content) render correctly on first page load with no layout shifts
- **SC-003**: Rolling stock cards expand/collapse with smooth animations in under 300ms
- **SC-004**: 100% of user-facing text strings are localized using Paraglide-JS (zero hardcoded strings in feature components)
- **SC-005**: The page displays correctly on mobile (320px width), tablet (768px width), and desktop (1920px width) viewports
- **SC-006**: All interactive elements (tabs, expandable cards, links) are keyboard accessible and pass WCAG 2.1 AA standards
