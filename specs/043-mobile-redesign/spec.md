# Feature Specification: Mobile Redesign

**Feature Branch**: `043-mobile-redesign`  
**Created**: 2026-06-28  
**Status**: Draft  
**Input**: User description: "feature 43: mobile redesign"

## Clarifications

### Session 2026-06-28

- Q: What viewport range defines mobile redesign scope? → A: Below 768 px only; desktop unchanged at 768 px and above.
- Q: What is the fallback when camera capture is unavailable or denied? → A: Fall back to file picker/gallery in the same editing flow.
- Q: What is the maximum nested sheet depth on mobile? → A: One nested level maximum (parent + one child).
- Q: What mobile touch target minimum applies to interactive controls? → A: 44x44 px minimum, except chip remove affordances may be 36x36 px.
- Q: How are Settings and Debug exposed in mobile navigation? → A: Keep 4 primary tabs plus More; place Settings and Debug as top actions inside More.

## User Scenarios & Testing _(mandatory)_

### User Story 1 - Mobile Navigation And Readability (Priority: P1)

As a mobile user, I can navigate all primary areas and understand page context immediately without layout breakage or clipped content.

**Why this priority**: If navigation and readability fail on mobile, users cannot complete any other workflow.

**Independent Test**: Open the app on a 375 px viewport in both English and Italian, navigate across all primary destinations, and verify that labels remain legible, page context is visible, and no content is obscured by device safe areas.

**Acceptance Scenarios**:

1. **Given** a mobile viewport, **When** the user opens any primary route, **Then** page content is fully visible and not clipped by top or bottom device insets.
2. **Given** a mobile viewport with Italian language selected, **When** navigation labels or page titles are long, **Then** labels remain readable without overlapping adjacent controls.
3. **Given** a user on mobile, **When** they need non-primary destinations such as Settings or Debug, **Then** they can reach those destinations via the More panel in at most two taps.

---

### User Story 2 - Mobile Collection Workflow Efficiency (Priority: P2)

As a collector using a phone, I can browse, filter, and add items from the collection view quickly with touch-friendly controls.

**Why this priority**: Collection management is a high-frequency workflow and drives daily app usage.

**Independent Test**: In a mobile viewport, complete a browse-filter-add flow from collection landing to item interaction, and verify touch targets, card readability, and action discoverability.

**Acceptance Scenarios**:

1. **Given** the collection view on mobile, **When** the user scrolls the item grid, **Then** cards render in a single easy-to-scan column and preserve readable metadata.
2. **Given** active filters on mobile, **When** the user removes a filter via chip controls, **Then** the action is reliably tappable with one attempt.
3. **Given** the collection view on mobile, **When** the user needs to add a new item, **Then** a contextual quick-add action is visible without requiring navigation away from the page.
4. **Given** the collection view on mobile, **When** the user evaluates layout modes, **Then** only the mobile-appropriate layout options are shown.

---

### User Story 3 - Mobile Editing Through Sheets (Priority: P3)

As a mobile user, I can edit item details through a consistent bottom-sheet editing experience instead of dense inline controls.

**Why this priority**: Consolidated editing reduces cognitive load and prevents accidental input errors on small screens.

**Independent Test**: On a mobile viewport, open item details and perform an edit flow entirely via sheet-based forms, including nested actions and media capture.

**Acceptance Scenarios**:

1. **Given** an item detail page on mobile, **When** the user starts editing, **Then** editing opens in a unified sheet flow instead of inline edit controls.
2. **Given** a sheet is open on mobile, **When** a second sheet-level action is triggered, **Then** nested sheets are clearly layered and the active sheet remains visually dominant.
3. **Given** a media attachment step on mobile, **When** the user adds a photo using the device camera, **Then** the captured file is accepted in the same editing flow.

---

### User Story 4 - Stable Mobile Experience Across Devices (Priority: P4)

As a product owner, I can release mobile improvements incrementally while maintaining desktop parity and avoiding regressions.

**Why this priority**: The redesign must be shippable in phases and must not disrupt the approved desktop experience.

**Independent Test**: Validate each rollout milestone independently on mobile and desktop viewports; confirm desktop behavior remains unchanged while each mobile milestone is deployable.

**Acceptance Scenarios**:

1. **Given** any completed mobile milestone, **When** the app is tested on desktop breakpoints, **Then** desktop layout and interaction patterns remain unchanged.
2. **Given** mobile validation in Italian and English, **When** core pages are exercised on 375 px viewport and larger phones, **Then** text overflow and gutter collisions do not occur.
3. **Given** mobile production builds, **When** the app starts during backend bridge initialization, **Then** users see non-blocking loading placeholders instead of full-screen blocking spinners.

### Edge Cases

- What happens when a localized navigation label exceeds expected length on very narrow devices (320-360 px)?
- How does the app behave when a user rapidly opens and dismisses sheet layers up to the one-level nesting limit?
- What happens when safe-area inset values are unavailable or zero on devices without notches?
- If camera capture is unavailable, denied, or canceled, how does the flow fall back to gallery/file picker without losing current form state?
- How does collection browsing behave when data sets are sparse, very large, or filtered to zero results?

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: The system MUST provide a mobile-optimized app shell for viewports below 768 px while preserving current desktop behavior at 768 px and above.
- **FR-002**: The system MUST ensure all primary content remains visible above device bottom insets and navigation chrome on mobile.
- **FR-003**: The system MUST show page-context titles in the mobile header for inner pages so users always know where they are.
- **FR-004**: The system MUST keep four primary bottom-navigation destinations plus a More entry, and MUST expose Settings and Debug as top actions within More.
- **FR-005**: The system MUST ensure mobile navigation labels remain readable in all supported languages without overlap or truncation-induced ambiguity.
- **FR-006**: The system MUST present collection items in a mobile-friendly single-column browsing experience on phone-sized viewports.
- **FR-007**: The system MUST expose a contextual add action within the collection mobile view.
- **FR-008**: The system MUST ensure mobile interactive controls in collection filters and toggles meet touch accessibility minimum target size.
- **FR-009**: The system MUST hide or adapt layout controls that are not suitable for phone-sized viewports.
- **FR-010**: The system MUST route mobile detail editing through a unified sheet-based editing flow rather than inline edit widgets.
- **FR-011**: The system MUST support layered sheet interactions on mobile with clear visual hierarchy and reliable dismiss behavior.
- **FR-012**: The system MUST allow users to attach images from native camera capture within the mobile editing flow.
- **FR-016**: If camera capture is unavailable, denied, or canceled, the system MUST offer gallery/file picker fallback in the same editing flow without discarding in-progress edits.
- **FR-017**: Mobile editing sheets MUST allow at most one nested child sheet at a time (parent + one child), and dismiss actions MUST return focus and state to the parent sheet.
- **FR-018**: On mobile viewports below 768 px, interactive controls MUST provide at least a 44x44 px hit target, except chip remove affordances which MAY use a 36x36 px minimum target.
- **FR-013**: The system MUST support phased rollout by enabling each milestone to be independently releasable without blocking unfinished mobile milestones.
- **FR-014**: The system MUST provide consistent loading placeholders during mobile startup and asynchronous bridge initialization.
- **FR-015**: The system MUST maintain parity across supported mobile languages for spacing, typography, and interaction affordances.

### Key Entities _(include if feature involves data)_

- **Mobile View Context**: Represents viewport-specific UI state that determines whether mobile-optimized layout behaviors are active.
- **Navigation Destination**: Represents a reachable app area from mobile navigation, including primary and secondary destinations.
- **Collection Card Item**: Represents a browseable collection unit with title, metadata, badges, and available quick actions.
- **Editing Sheet Session**: Represents the active editing interaction state for detail updates, including nested sheet depth and dismissal behavior.
- **Media Attachment**: Represents a user-added image asset captured or selected during an editing session.
- **Localization Text Variant**: Represents a translated label or title that must fit mobile spacing and readability constraints.

### Assumptions

- The redesign targets mobile behavior only for viewports below 768 px, while behavior at 768 px and above remains unchanged.
- The app continues supporting at least English and Italian during this feature rollout.
- Mobile improvements are delivered as incremental milestones that can be validated independently.
- Existing data models and business rules remain valid; this feature primarily changes user interaction and presentation behavior.

### Dependencies

- Availability of approved mobile redesign guidelines in project documentation.
- Availability of test devices or emulators for common phone widths, including 375 px baseline.
- Availability of production-like mobile builds for native runtime validation.

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: In usability validation, at least 95% of users complete core mobile navigation to any destination in 2 taps or fewer.
- **SC-002**: In mobile collection testing, at least 90% of users complete a browse-filter-add workflow in under 90 seconds without assistance.
- **SC-003**: In touch-target audits for viewports below 768 px, 100% of interactive controls meet a 44x44 px minimum hit target, except chip remove affordances which meet at least 36x36 px.
- **SC-004**: In multilingual mobile regression testing at 375 px width, 0 critical text-overlap or clipping defects remain open.
- **SC-005**: In phased rollout validation, every delivered milestone is deployable independently with no desktop-severity regressions.
- **SC-006**: During mobile startup in native builds, users see visible non-blocking loading placeholders within 1 second of app launch.
