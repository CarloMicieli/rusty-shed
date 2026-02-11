# Feature Specification: Reusable Railway Model Component

**Feature Branch**: `018-railway-model-component`  
**Created**: February 11, 2026  
**Status**: Draft  
**Input**: User description: "Build reusable railway model component for collection and wishlist details"

## Clarifications

### Session 2026-02-11

- Q: How should the component handle display of rolling stock details when there is only one unit? → A: Bypass list and expansion states, promote detailed specifications directly under global identity

## User Scenarios & Testing _(mandatory)_

### User Story 1 - View Basic Model Information (Priority: P1)

A railway model collector views the essential identification and characteristics of a railway model product (single locomotive or multi-car set) in a clear, organized hierarchy that distinguishes between product-level information (manufacturer, scale, era) and individual rolling stock details.

**Why this priority**: This is the foundation of the component - displaying basic product information is the minimum viable functionality that delivers immediate value by showing collectors what model they're looking at.

**Independent Test**: Can be fully tested by rendering the component with a sample model containing manufacturer, product code, scale, era, power method, category, and description. The component should display this information in the header and global specs sections without requiring any rolling stock data.

**Acceptance Scenarios**:

1. **Given** a railway model with basic product data (manufacturer, product code, scale), **When** the component is rendered, **Then** the header displays manufacturer name, product code, and scale clearly visible at the top
2. **Given** a railway model with global specifications (era, power method, category, description), **When** the component is rendered, **Then** these specifications are displayed in a dedicated "Global Specs" section below the hero area
3. **Given** a railway model with a status (In Collection or Wishlist), **When** the component is rendered, **Then** a badge indicating the status is displayed prominently in the hero section
4. **Given** a railway model without an image, **When** the component is rendered, **Then** a placeholder image is shown in the hero section

---

### User Story 2 - View and Manage Model Image (Priority: P2)

A collector uploads or updates the product image for their railway model by either browsing their file system or dragging and dropping an image file directly onto the hero section.

**Why this priority**: Visual identification is critical for collectors managing physical models, but the component can function without images (using placeholders), making this second priority.

**Independent Test**: Can be tested by rendering the component with image upload controls enabled, attempting both file browse and drag-drop operations, and verifying the image updates in the hero section.

**Acceptance Scenarios**:

1. **Given** a railway model without an image, **When** the collector clicks the browse button in the hero section, **Then** a file picker dialog opens allowing selection of image files
2. **Given** a railway model without an image, **When** the collector drags an image file over the hero section, **Then** a visual indicator shows the drop zone is active
3. **Given** an active drop zone, **When** the collector drops an image file, **Then** the image is uploaded and displayed in the hero section
4. **Given** a railway model with an existing image, **When** the collector uploads a new image, **Then** the hero section updates to show the new image
5. **Given** an invalid file type (not an image), **When** the collector attempts to upload it, **Then** an error message is displayed and the upload is rejected

---

### User Story 3 - View Rolling Stock Details (Priority: P2)

A collector views detailed specifications for each individual rolling stock unit within a railway model set (or a single unit for standalone locomotives) including series code, category/subcategory, road number, depot, livery, control type, DCC interface, and coupling type. For models with only one rolling stock unit, the detailed specifications appear directly under the global identity without tabs or expansion controls.

**Why this priority**: This is critical for serious collectors who need to catalog individual rolling stock specifications, but the component provides value even without this detail level (just showing global specs).

**Independent Test**: Can be tested by rendering the component with a model containing multiple rolling stock entries, verifying each entry displays all required fields in an organized format within the tabbed interface. Also test with a single-unit model to verify direct display without tabs.

**Acceptance Scenarios**:

1. **Given** a railway model set with multiple rolling stock units, **When** the Rolling Stock List tab is selected, **Then** each unit is displayed as a separate row with its key identification (series code with series name)
2. **Given** a rolling stock unit with complete specifications in a multi-unit set, **When** the row is expanded, **Then** all details are visible: category, subcategory, road number, depot, livery, control type, DCC interface, and coupling type
3. **Given** a rolling stock unit with a series (e.g., "E.656 (I Serie)"), **When** displayed, **Then** both the series code and series name are shown together
4. **Given** a single locomotive model (not a set), **When** the component is rendered, **Then** the rolling stock detailed specifications are displayed directly under the global specifications without tabs or expandable rows
5. **Given** a rolling stock unit with missing optional data, **When** displayed, **Then** only available fields are shown without empty placeholders

---

### User Story 4 - Navigate Between Model Details and Rolling Stock (Priority: P3)

A collector switches between viewing the general model details and the detailed rolling stock list using a tabbed interface, maintaining context and scroll position. This applies only to multi-unit sets; single-unit models display all information in a unified view without tabs.

**Why this priority**: This enhances usability for complex multi-unit sets but isn't essential for the component's core value proposition - displaying model information.

**Independent Test**: Can be tested by rendering a multi-unit model with both detail sections, clicking between tabs, and verifying the content switches appropriately and any scroll position within a tab is maintained. Verify single-unit models do not show tabs.

**Acceptance Scenarios**:

1. **Given** the component is displaying a multi-unit railway model set, **When** the Railway Model Details tab is selected (default), **Then** the general specifications are visible
2. **Given** the component is displaying a multi-unit railway model set, **When** the collector clicks the Rolling Stock List tab, **Then** the view switches to show individual rolling stock entries
3. **Given** the collector has scrolled within a tab's content in a multi-unit set, **When** they switch to another tab and back, **Then** the scroll position is preserved
4. **Given** the component is displaying a single-unit model, **When** the component is rendered, **Then** no tabs are shown and rolling stock details appear directly below global specifications

---

### Edge Cases

- What happens when a model has no rolling stock data? For multi-unit models, the Rolling Stock List tab should still be accessible but display a message indicating no rolling stock data is available. For single-unit models expecting rolling stock data, display an appropriate error or empty state.
- How does the system handle extremely long product descriptions or series names? Text should wrap appropriately and consider truncation with expand/collapse for very long content (>500 characters).
- How does the component determine if a model is single-unit or multi-unit? The distinction is based on the count of rolling stock units: exactly one unit triggers direct display mode, two or more units trigger tabbed list mode.
- What happens when a model belongs to both Collection and Wishlist? The status badge should indicate the primary status (Collection takes precedence) with a visual indicator or note about dual status.
- How does the component behave on mobile devices with limited screen width? The layout should stack vertically with the header remaining compact, tabs converting to a mobile-friendly format, and expandable rolling stock rows adapting to narrow viewports.
- What happens when image upload fails (network error, server rejection)? An error message should display and the previous image (or placeholder) should remain visible.
- How does the component handle missing required fields like manufacturer or product code? These are treated as required data - the component should display a clear error state if essential fields are missing.

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: Component MUST display product-level header information including manufacturer name, product code, and scale
- **FR-002**: Component MUST display a hero section containing a product image or placeholder with overlaid status badge(s)
- **FR-003**: Component MUST display global specifications applicable to the entire product including era, power method, category, and description
- **FR-004**: Component MUST provide a tabbed interface with at least two sections (Railway Model Details and Rolling Stock List) when displaying multi-unit sets; single-unit models MUST display rolling stock details directly without tabs
- **FR-005**: Component MUST display individual rolling stock entries in the Rolling Stock List for multi-unit sets, each showing series code with series name if available; single-unit models MUST display the rolling stock specifications directly under global specifications
- **FR-006**: Component MUST display rolling stock specifications including category, subcategory, road number, depot, livery, control type, DCC interface, and coupling type
- **FR-007**: Component MUST support expandable/collapsible rows for rolling stock entries in multi-unit sets to manage visual complexity; single-unit models MUST display all specifications directly without expansion controls
- **FR-008**: Component MUST allow image upload via file browser selection
- **FR-009**: Component MUST allow image upload via drag-and-drop onto the hero section
- **FR-010**: Component MUST validate uploaded files are image types (JPG, PNG, WebP, GIF)
- **FR-011**: Component MUST display appropriate status badges (e.g., "In Collection", "Wishlist") in the hero section
- **FR-012**: Component MUST handle both single-unit models (one locomotive) and multi-unit sets (multiple rolling stock entries) with the same interface
- **FR-013**: Component MUST be reusable across different contexts (collection details page, wishlist details page)
- **FR-014**: Component MUST maintain responsive behavior from mobile (320px) to desktop (1920px+) viewports
- **FR-015**: Component MUST handle missing optional data gracefully by hiding empty fields rather than showing empty placeholders

### Key Entities

- **Railway Model Product**: Represents the boxed product containing manufacturer, product code, scale, era, power method, category, and description. This is the "master" entity that contains one or more rolling stock units.
- **Rolling Stock Unit**: Represents an individual piece of rolling stock (locomotive, passenger car, freight car, etc.) with its own specifications including series code, series name, category, subcategory, road number, depot, livery, control type, DCC interface, and coupling type. A product contains one (single locomotive) to many (multi-car set) rolling stock units.
- **Product Image**: The visual representation of the railway model product, uploaded by the user, associated with the product (not individual rolling stock units).
- **Status Badge**: Indicates whether the model is in the user's collection, on their wishlist, or both (collection takes precedence for primary badge).

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: Collectors can identify a railway model product by viewing its header information (manufacturer, product code, scale) within 2 seconds of component rendering
- **SC-002**: Collectors can upload a product image in under 10 seconds using either browse or drag-drop methods with visual feedback throughout the process
- **SC-003**: Collectors can view all rolling stock specifications for a 10-car set without horizontal scrolling on desktop viewports (1280px+)
- **SC-004**: Component renders correctly on mobile devices (320px width) with all information accessible through scrolling and expansion without data truncation
- **SC-005**: 95% of collectors can locate specific rolling stock specifications (e.g., DCC interface type) within 15 seconds of viewing the component
- **SC-006**: Component successfully handles railway model sets ranging from 1 to 20 rolling stock units without performance degradation (render time under 500ms)
- **SC-007**: Tab switching between Railway Model Details and Rolling Stock List completes in under 100ms with smooth transition
