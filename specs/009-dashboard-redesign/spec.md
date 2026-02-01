# Feature Specification: Dashboard Redesign

**Feature Branch**: `009-dashboard-redesign`  
**Created**: 2026-01-30  
**Status**: Draft  
**Input**: User description: "Unified Page Identity with integrated title, streamlined quick actions, visual Recently Added gallery, functional Depot workspace, and structured data grid widgets"

## User Scenarios & Testing _(mandatory)_

### User Story 1 - View Dashboard with Clear Page Identity (Priority: P1)

As a collector, I want the Dashboard page to have a clear, integrated title and contextual breadcrumb so I immediately know where I am in the app and feel the same cohesive branding experience as the "My Collection" view.

**Why this priority**: This is the foundational visual change that establishes consistency across the app. Without a clear page identity, users feel disoriented when navigating between sections.

**Independent Test**: Can be fully tested by navigating to the Dashboard and verifying the page title, subtitle/breadcrumb, and visual hierarchy match the Collection view's styling pattern.

**Acceptance Scenarios**:

1. **Given** a user navigates to the Dashboard, **When** the page loads, **Then** the user sees a bold "Dashboard" title displayed directly on the page (not in navigation bar) with a contextual subtitle showing "DASHBOARD / OVERVIEW"
2. **Given** a user compares Dashboard to My Collection view, **When** viewing both pages, **Then** both pages have consistent title styling (same font weight, size, and spacing patterns)
3. **Given** a user is on the Dashboard, **When** viewing the page header, **Then** the title section includes a brief description of the dashboard's purpose

---

### User Story 2 - Access Quick Actions from Command Center (Priority: P1)

As a collector, I want three distinct, high-priority action buttons (Add to Collection, Add to Wishlist, Log Maintenance) grouped in a clearly labeled "Command Center" area so I can quickly perform common tasks without hunting for scattered links.

**Why this priority**: Quick actions are the primary entry points for user interaction. Streamlining these buttons directly impacts user productivity and satisfaction.

**Independent Test**: Can be fully tested by verifying all three buttons are present, visually distinct, and functional when clicked.

**Acceptance Scenarios**:

1. **Given** a user is on the Dashboard, **When** viewing the Command Center area, **Then** the user sees exactly three buttons: "Add to Collection", "Add to Wishlist", and "Log Maintenance"
2. **Given** a user clicks "Add to Collection", **When** the action completes, **Then** the user is directed to the new model entry form
3. **Given** a user clicks "Add to Wishlist", **When** the action completes, **Then** the wishlist modal opens for adding a new item
4. **Given** a user clicks "Log Maintenance", **When** the action completes, **Then** the user sees a toast notification indicating "Maintenance logging coming soon" (placeholder until maintenance feature is implemented)
5. **Given** the Command Center is displayed, **When** viewing its position, **Then** it appears on the right side of the layout, separate from statistics

---

### User Story 3 - Browse Recently Added Models in Visual Gallery (Priority: P2)

As a collector, I want to see my newest models displayed as large visual cards with photos in a gallery format so I can appreciate my "pride of the fleet" and quickly access details of recent additions.

**Why this priority**: The visual gallery transforms the dashboard from a utilitarian list into an engaging showcase, enhancing the emotional connection collectors have with their models.

**Independent Test**: Can be fully tested by adding a model with an image, then verifying it appears as a large visual card with the photo prominently displayed and clickable.

**Acceptance Scenarios**:

1. **Given** a user has recently added models to their collection, **When** viewing the Recently Added section, **Then** models are displayed as large visual cards with prominent images
2. **Given** a model in the gallery has an associated image, **When** displayed in the gallery, **Then** the image fills the card's visual area with the model title overlaid
3. **Given** a model in the gallery has no image, **When** displayed in the gallery, **Then** a placeholder visual is shown (e.g., initials or icon)
4. **Given** a user clicks on a gallery card, **When** the click is registered, **Then** the user navigates to the model's detail page based on its source (Collection items → `/my-collection/{id}`, Wishlist items → `/my-wishlists/{id}`)
5. **Given** there are no recently added models, **When** viewing the Recently Added section, **Then** an empty state message is displayed with a prompt to add models

---

### User Story 4 - Monitor Depot Work-in-Progress (Priority: P2)

As a collector, I want to see a functional utility list in the Depot section showing which models are currently under repair versus ready for service, with color-coded status badges, so I can track the health of my collection at a glance.

**Why this priority**: The Depot is about utility and workflow management. Color-coded status badges provide immediate visual feedback on collection health without requiring users to read detailed text.

**Data Note**: The current `DashboardDepotEntry` type does not include a `status` field. For MVP, status badges will use placeholder/mock status until maintenance tracking is implemented. A future iteration will derive status from maintenance records.

**Independent Test**: Can be fully tested by verifying depot items display status badges with appropriate styling (mock data acceptable for MVP).

**Acceptance Scenarios**:

1. **Given** a user has models in various maintenance states, **When** viewing the Depot section, **Then** each model displays a color-coded status badge
2. **Given** a model is marked as "In Service", **When** displayed in the Depot, **Then** it shows a badge with a positive/neutral color (e.g., green or blue)
3. **Given** a model is marked as "Under Repair", **When** displayed in the Depot, **Then** it shows a badge with an attention color (e.g., orange or yellow)
4. **Given** the Depot section is displayed, **When** comparing to the Recently Added gallery, **Then** the Depot uses a list/table format optimized for scanning status rather than visual showcase
5. **Given** no models are in the depot, **When** viewing the section, **Then** an empty state is shown with guidance on how to log maintenance

---

### User Story 5 - View Statistics in Widget Cards (Priority: P3)

As a collector, I want the Yard Statistics (Total Value, Stock Count, Maintenance Alerts) displayed in distinct "info-card" widgets so the raw numbers are visually separated from charts and the top of the page feels organized rather than overwhelming.

**Why this priority**: Structured data presentation prevents information overload and makes the dashboard scannable. This is a polish item that improves overall UX but doesn't block core functionality.

**Independent Test**: Can be fully tested by loading the dashboard and verifying statistics appear in individual card containers with visual separation from charts below.

**Acceptance Scenarios**:

1. **Given** a user is on the Dashboard, **When** viewing the statistics area, **Then** each statistic (Total Value, Stock Count, Alerts) appears in its own distinct info-card widget
2. **Given** the statistics cards are displayed, **When** viewing their layout, **Then** they are visually separated from the charts section below with clear spacing or dividers
3. **Given** a statistic has an alert condition (e.g., maintenance due), **When** displayed in its card, **Then** the card shows a visual indicator (color, badge, or icon) drawing attention
4. **Given** data is loading, **When** viewing the statistics area, **Then** skeleton loading states are shown for each card position

---

### Edge Cases

- What happens when there are more than 6 recently added items? Gallery limits display to most recent items and shows "View All" link to collection
- How does the layout adapt on mobile devices? Cards stack vertically, gallery becomes horizontally scrollable with snap-to-card behavior
- What happens when statistics data fails to load? Show error state with retry option, maintaining card layout structure
- How does the system handle a model with very long title text? Truncate with ellipsis after 2 lines, full title accessible on hover/focus
- What happens when maintenance logging is clicked but no models exist? Prompt user to add a model first with link to Add to Collection

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: System MUST display the page title "Dashboard" directly on the page content area, not in the navigation bar
- **FR-002**: System MUST display a contextual subtitle or breadcrumb path (e.g., "DASHBOARD / OVERVIEW") beneath the main title
- **FR-003**: System MUST provide exactly three quick action buttons: "Add to Collection", "Add to Wishlist", and "Log Maintenance"
- **FR-004**: System MUST group quick action buttons in a designated "Command Center" area positioned on the right side of the layout
- **FR-005**: System MUST display recently added models as large visual cards with prominent imagery
- **FR-006**: System MUST make each gallery card clickable, navigating to the respective model's detail page
- **FR-007**: System MUST display the Depot section as a utility list with focus on status information
- **FR-008**: System MUST show color-coded status badges for depot items (distinguishing "In Service" from "Under Repair" states)
- **FR-009**: System MUST display statistics (Total Value, Stock Count, Alerts) in individual info-card widgets
- **FR-010**: System MUST visually separate statistics cards from chart components with clear spacing
- **FR-011**: System MUST display appropriate empty states for Recently Added gallery and Depot sections when no data exists
- **FR-012**: System MUST show loading skeleton states while data is being fetched

### Key Entities

- **Dashboard Statistics**: Collection value, total rolling stock count, maintenance alerts count; displayed in widget format
- **Recently Added Item**: Model entry with id, title, subtitle, image URL; displayed as visual gallery card
- **Depot Item**: Model entry with id, title, current maintenance status; displayed in utility list with status badge
- **Quick Action**: Button with label, icon, and navigation/modal target; grouped in Command Center

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: Users can identify their current location (Dashboard) within 2 seconds of page load without looking at browser tabs or navigation
- **SC-002**: Users can access any of the three primary actions (Add to Collection, Add to Wishlist, Log Maintenance) within one click from the dashboard
- **SC-003**: Users can visually distinguish between the gallery showcase (Recently Added) and utility list (Depot) sections without reading section headers
- **SC-004**: Users can identify which models need attention (maintenance) within 5 seconds by scanning color-coded status badges
- **SC-005**: Dashboard page maintains visual consistency with My Collection page styling patterns (title hierarchy, spacing, card styling)
- **SC-006**: Page renders complete layout including all sections within 3 seconds on standard connection
- **SC-007**: 90% of users can correctly identify the purpose of each dashboard section on first visit (based on visual hierarchy and labeling)
