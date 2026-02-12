# Feature Specification: Railway Model Preview Card Component

**Feature Branch**: `019-model-preview-card`
**Created**: 2026-02-11
**Status**: Draft
**Input**: User description: "Reusable RailwayModelPreviewCard component with thumbnail, metadata, and identification plate"

## Clarifications

### Session 2026-02-11

- Q: How should the component handle road numbers that exceed the available space in the identification plate (more than 25 characters)? → A: Truncate with ellipsis (e.g., "50 80 26-81 51...") - user must click/hover for full number
- Q: How should the identification plate appear when a model has no road number? → A: Display "# ---" in the identification plate to indicate missing data
- Q: When a model has both sound AND electric digital features, how should the top-left overlay icons be displayed? → A: Stack icons horizontally in the top-left corner (side by side)
- Q: Which aspect ratio should be the default/standard for the thumbnail (16:9 or 3:2)? → A: 16:9 (wider, modern standard, better for very long train models)
- Q: Should the delete button trigger an immediate confirmation dialog, or require a two-step action? → A: Single click opens confirmation dialog immediately
- Q: Must the component support mobile devices? → A: Yes, component must be responsive and work on mobile devices with cards stacking vertically

## User Scenarios & Testing _(mandatory)_

### User Story 1 - View Model Summary Information (Priority: P1)

As a model railway collector, I need to see a compact visual summary of each railway model in my collection or wishlist, so I can quickly browse and identify models without opening detailed views.

**Why this priority**: This is the core value of the component - providing at-a-glance information about railway models. Without this, users cannot effectively browse their collection.

**Independent Test**: Can be fully tested by rendering the component with a sample railway model containing manufacturer, product code, series, category, and road number. Component should display all information in a readable, scannable format.

**Acceptance Scenarios**:

1. **Given** a railway model with complete metadata, **When** the card is displayed, **Then** the user sees the manufacturer name, product code, series code, category name, and road number clearly formatted
2. **Given** a railway model with a photo, **When** the card is rendered, **Then** the thumbnail displays the photo at 16:9 or 3:2 aspect ratio
3. **Given** multiple railway model cards in a list, **When** the user scans the list, **Then** each card's road number is prominently displayed in a high-visibility identification plate

---

### User Story 2 - Identify Model Status and Characteristics (Priority: P2)

As a collector, I need to see key metadata badges (scale, power method, era, purchase date) and status indicators (unit count, digital features) at a glance, so I can quickly understand the model's specifications without reading detailed descriptions.

**Why this priority**: Metadata tags help collectors filter and compare models efficiently. This is essential for managing large collections but not required for basic browsing.

**Independent Test**: Can be tested by rendering cards with different combinations of metadata (different scales, power methods, eras) and status overlays (unit counts, sound/DCC indicators). All badges should be clearly visible and not overlap.

**Acceptance Scenarios**:

1. **Given** a railway model with scale H0 and DC power, **When** the card is displayed, **Then** badges showing "[H0]" and "[DC]" are visible in the meta tags area
2. **Given** a model set containing 3 units, **When** the card is rendered, **Then** a "×3" badge appears in the bottom-right corner of the thumbnail
3. **Given** a model with DCC sound capabilities, **When** the card is displayed, **Then** a sound icon overlay appears in the top-left corner of the thumbnail
4. **Given** a model purchased on a specific date, **When** the card is rendered, **Then** the purchase date is displayed as a badge in the format "PURCHASED: DD/MM/YYYY"

---

### User Story 3 - Handle Missing Visual Data Gracefully (Priority: P3)

As a collector with models that don't have photos, I need to see a category-appropriate placeholder icon in the thumbnail area, so the card still looks complete and I can identify the model type at a glance.

**Why this priority**: Not all models will have photos immediately. Placeholders maintain visual consistency and provide contextual hints about the model category.

**Independent Test**: Can be tested by rendering cards for models without photos across different categories (steam locomotive, electric locomotive, wagon, etc.). Each should display a distinct placeholder icon appropriate to its category.

**Acceptance Scenarios**:

1. **Given** a steam locomotive model without a photo, **When** the card is displayed, **Then** a stylized steam locomotive silhouette appears as the thumbnail
2. **Given** a wagon model without a photo, **When** the card is displayed, **Then** a stylized wagon silhouette appears as the thumbnail
3. **Given** an electric locomotive model without a photo, **When** the card is displayed, **Then** a stylized electric locomotive silhouette appears as the thumbnail

---

### User Story 4 - Remove Models from Collection (Priority: P2)

As a collector, I need to be able to delete a model from my collection or wishlist directly from the card view, so I can quickly manage my inventory without navigating to detail pages.

**Why this priority**: Quick removal actions improve workflow efficiency when managing collections. This is a common operation but not critical for initial browsing.

**Independent Test**: Can be tested by clicking the trash/delete button on a card and verifying that a confirmation prompt appears before removal.

**Acceptance Scenarios**:

1. **Given** a model card is displayed, **When** the user clicks the trash icon, **Then** a confirmation dialog appears asking to confirm deletion
2. **Given** the user confirms deletion, **When** the confirmation is accepted, **Then** the model is removed from the collection/wishlist
3. **Given** the user cancels deletion, **When** the confirmation is cancelled, **Then** the model remains in the collection/wishlist and the card is still visible

---

### Edge Cases

- **Long road numbers**: Road numbers exceeding 25 characters are truncated with ellipsis in the identification plate. Users can click or hover to see the full number.
- **Missing road number**: Display "# ---" in the identification plate to indicate no road number is assigned.
- **Multiple digital features**: When a model has multiple digital status indicators (e.g., sound + electric), stack icons horizontally side by side in the top-left corner.
- **Missing manufacturer information**: Display "Unknown" as placeholder text (per FR-014).
- **Missing or invalid purchase date**: Omit the purchase date badge entirely (per FR-015).
- **Missing category for placeholder**: Use a generic train silhouette as the default placeholder when category information is unavailable.
- **Missing scale or power method**: Omit the respective badges (per FR-015).

## Requirements _(mandatory)_

### Functional Requirements

#### Visual Display

- **FR-001**: Component MUST display a thumbnail image with a fixed 16:9 aspect ratio to accommodate the horizontal nature of railway models
- **FR-002**: Component MUST display the manufacturer name and product code on the primary information line (e.g., "A.C.M.E. • 1236")
- **FR-003**: Component MUST display the series code and category as the main title (e.g., "Class 140 Electric Locomotive")
- **FR-004**: Component MUST display the road number in a high-visibility, monospaced identification plate (e.g., "# 50 80 26-81 517-7")
- **FR-004a**: Component MUST truncate road numbers exceeding 25 characters with ellipsis and provide click/hover interaction to reveal the full number
- **FR-004b**: Component MUST display "# ---" in the identification plate when no road number is assigned
- **FR-005**: Component MUST display metadata badges for scale, power method, era, and purchase date in a consistent horizontal layout

#### Thumbnail and Placeholders

- **FR-006**: Component MUST display a category-specific placeholder icon when a model has no photo (e.g., steam locomotive silhouette for steam engines, wagon silhouette for wagons)
- **FR-006a**: Component MUST display a generic train silhouette as the default placeholder when category information is unavailable
- **FR-007**: Component MUST support displaying actual model photos when available, scaled to fit the thumbnail aspect ratio without distortion

#### Status Indicators

- **FR-008**: Component MUST display a unit count badge (e.g., "×3") in the bottom-right corner of the thumbnail when the model is part of a set with multiple units
- **FR-009**: Component MUST display digital status overlay icons in the top-left corner of the thumbnail (e.g., speaker icon for sound, bolt icon for electric), stacking multiple icons horizontally when present
- **FR-010**: Component MUST ensure overlay badges do not obscure critical parts of the thumbnail image

#### Interaction

- **FR-011**: Component MUST provide a delete/trash action button that triggers a confirmation dialog on single click before removing the model from the collection or wishlist
- **FR-012**: Component MUST be reusable across different contexts (collection view, wishlist view, search results)

#### Responsive Behavior

- **FR-016**: Component MUST be responsive and adapt to mobile device screen sizes
- **FR-017**: Component MUST display cards in a vertical stack (one card per row) on mobile devices

#### Data Handling

- **FR-013**: Component MUST handle missing or incomplete model data gracefully without breaking the layout
- **FR-014**: Component MUST display "Unknown" or similar placeholder text for missing manufacturer or category information
- **FR-015**: Component MUST omit badges for missing metadata (e.g., if purchase date is not set, do not show the purchase date badge)

### Key Entities _(include if feature involves data)_

- **Railway Model**: Represents a model railway item with attributes including:
  - Manufacturer name
  - Product code
  - Series code
  - Category (e.g., "Electric Locomotive", "Steam Locomotive", "Wagon")
  - Road number (identification marking)
  - Scale (e.g., "H0", "N", "TT")
  - Power method (e.g., "DC", "AC", "DCC")
  - Era (historical period)
  - Purchase date
  - Photo/image URL (optional)
  - Unit count (for sets containing multiple units)
  - Digital features (sound, DCC capabilities)

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: Users can identify a specific model's road number within 2 seconds of viewing the card
- **SC-002**: Component renders consistently across all supported model categories without layout breaks
- **SC-003**: Cards with and without photos maintain visual consistency with clear placeholders for missing images
- **SC-004**: Users can scan a list of 20+ model cards and locate a specific model by manufacturer and series within 10 seconds
- **SC-005**: All metadata badges are readable at typical screen distances (50-70cm on desktop, standard mobile viewing distance on mobile) without zooming
- **SC-006**: Component adapts to different container widths without truncating critical information (manufacturer, road number, series)
- **SC-007**: Status overlay badges are immediately recognizable (unit count, digital features) without requiring hover or interaction
- **SC-008**: On mobile devices, cards stack vertically with full readability of all information elements

## Out of Scope _(optional)_

- Integration with collection or wishlist pages (will be handled in a separate task)
- Detailed model information view (clicking the card to open full details)
- Inline editing of model information from the card
- Drag-and-drop reordering of cards
- Batch selection and bulk actions across multiple cards
- Animations or transitions when cards appear/disappear
- Filtering or sorting logic (component is purely presentational)

## Assumptions _(optional)_

- The component will receive a structured model data object as a prop
- Parent components will handle data fetching and state management
- The delete action will emit an event that the parent component handles
- Category types are predefined and consistent across the application
- Road numbers follow a standard format, though lengths may vary
- Thumbnail images will be pre-processed to reasonable dimensions (not handling multi-megabyte uploads)
- The component is designed to work across desktop and mobile devices, with responsive layout adapting to screen size
- The monospaced identification plate font is available in the application's design system
