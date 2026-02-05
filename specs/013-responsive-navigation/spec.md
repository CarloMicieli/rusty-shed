# Feature Specification: Responsive Navigation System

**Feature Branch**: `013-responsive-navigation`  
**Created**: February 5, 2026  
**Status**: Draft  
**Input**: User description: "Organize the app features into a responsive navigation system: a full-list desktop sidebar and a prioritized 5-slot mobile bottom bar"

## User Scenarios & Testing _(mandatory)_

### User Story 1 - Desktop Navigation Access (Priority: P1)

Desktop users need quick access to all application features through a persistent sidebar. They can see and access all 9 features (Home, Collection, Finance, Wishlists, Maintenance, Depot, Digital DCC, Railway Tracks) in a single view without additional navigation steps.

**Why this priority**: Desktop is the primary platform for detailed model railway management. Users expect to see all features immediately available without hidden menus.

**Independent Test**: Can be fully tested by opening the app on a desktop viewport (≥768px) and verifying all 9 navigation items are visible and clickable, delivering complete feature access without additional UI interactions.

**Acceptance Scenarios**:

1. **Given** a user opens the app on a desktop browser (viewport ≥768px), **When** they view the sidebar, **Then** all 9 features are displayed in a vertical list with proper icons and labels
2. **Given** a desktop user is viewing the sidebar, **When** they click any navigation item, **Then** they navigate to that feature and the active state is visually indicated
3. **Given** a desktop user is on any feature page, **When** they view the sidebar, **Then** the current feature is highlighted with an active visual state

---

### User Story 2 - Mobile Primary Navigation (Priority: P1)

Mobile users need quick access to the 5 most frequently used features through a bottom navigation bar. They can tap Home, Collection, Finance, Wishlists, or More without scrolling or opening menus.

**Why this priority**: Mobile users expect essential features at their fingertips. The bottom bar pattern is industry-standard for mobile apps, ensuring immediate access to primary features.

**Independent Test**: Can be fully tested by opening the app on a mobile viewport (<768px) and verifying the 5-slot bottom bar is visible and functional, delivering access to the 4 primary features plus the More menu.

**Acceptance Scenarios**:

1. **Given** a user opens the app on a mobile browser (viewport <768px), **When** they view the screen, **Then** a bottom navigation bar displays exactly 5 slots with icons and labels
2. **Given** a mobile user views the bottom bar, **When** they tap any of the first 4 slots (Home, Collection, Finance, Wishlists), **Then** they navigate to that feature immediately
3. **Given** a mobile user is on any of the 4 primary features, **When** they view the bottom bar, **Then** the corresponding slot shows an active visual state
4. **Given** a mobile user resizes their browser from mobile to desktop width, **When** the viewport exceeds 768px, **Then** the bottom bar is hidden and the full sidebar appears

---

### User Story 3 - Mobile Secondary Feature Access (Priority: P2)

Mobile users need access to secondary features (Maintenance, Depot, Digital DCC, Railway Tracks) through the "More" menu. They can tap the More button to reveal these features in a bottom sheet or drawer.

**Why this priority**: While secondary features are used less frequently, they must remain accessible on mobile. This provides a scalable solution that keeps the primary navigation clean.

**Independent Test**: Can be fully tested by tapping the More button on mobile and verifying all 4 secondary features appear in an accessible menu, delivering complete feature access.

**Acceptance Scenarios**:

1. **Given** a mobile user taps the More button in the bottom bar, **When** the action completes, **Then** a bottom sheet/drawer opens displaying the 4 secondary features with proper icons and labels
2. **Given** the More menu is open, **When** the user taps any secondary feature, **Then** they navigate to that feature and the menu closes
3. **Given** a mobile user is on any secondary feature page (Maintenance, Depot, Digital DCC, or Railway Tracks), **When** they view the bottom bar, **Then** the More button shows an active visual state
4. **Given** the More menu is open, **When** the user taps outside the menu or presses back, **Then** the menu closes without navigation

---

### User Story 4 - Consistent Feature Identity (Priority: P2)

Users experience consistent naming, iconography, and visual identity for all features across desktop and mobile views. Feature names and icons are identical regardless of the device or context.

**Why this priority**: Consistent labeling prevents confusion when switching devices and builds familiarity with the feature set. Users recognize features by their icons and names.

**Independent Test**: Can be fully tested by comparing feature labels and icons across desktop sidebar and mobile views, verifying they match exactly.

**Acceptance Scenarios**:

1. **Given** a user views any feature on desktop, **When** they switch to mobile and locate the same feature, **Then** the icon and label are identical
2. **Given** a user views the desktop sidebar, **When** they read feature names, **Then** they see the updated names (Home instead of Dashboard, Finance instead of Budget Tracking, etc.)
3. **Given** a user navigates the app, **When** they view any feature label, **Then** it is displayed in their selected language using Paraglide-JS translations

---

### User Story 5 - Localized Navigation (Priority: P3)

Users experience navigation labels in their preferred language. All feature names are properly translated and culturally appropriate.

**Why this priority**: Internationalization is important for user accessibility, but the navigation structure and functionality take precedence.

**Independent Test**: Can be fully tested by switching language preferences and verifying all navigation labels update appropriately.

**Acceptance Scenarios**:

1. **Given** a user changes their language preference, **When** they view the navigation, **Then** all feature labels update to the selected language
2. **Given** a user with a non-English locale, **When** they first open the app, **Then** navigation labels appear in their locale's language

---

### Edge Cases

- What happens when a user rapidly switches between mobile and desktop viewports?
- How does the system handle extremely narrow mobile viewports (<320px)?
- What happens when a user bookmarks a secondary feature and opens it on mobile?
- How does keyboard navigation work in the desktop sidebar?
- What happens when the More menu is open and the user resizes to desktop viewport? (Covered by FR-015)

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: System MUST display a full-list sidebar navigation on desktop viewports (≥768px) containing all 9 features
- **FR-002**: System MUST display a 5-slot bottom bar navigation on mobile viewports (<768px) containing Home, Collection, Finance, Wishlists, and More
- **FR-003**: Desktop sidebar MUST display features in order: Home, Collection, Finance, Wishlists, Maintenance, Depot, Digital (DCC), Railway Tracks
- **FR-004**: Mobile bottom bar MUST display slots in order: Home, Collection, Finance, Wishlists, More
- **FR-005**: System MUST show an active visual state on the currently navigated feature in both desktop and mobile views
- **FR-006**: Mobile More button MUST show an active visual state when the user is on any secondary feature (Maintenance, Depot, Digital DCC, or Railway Tracks)
- **FR-007**: System MUST display the More menu as a bottom sheet on mobile that reveals the 4 secondary features
- **FR-008**: More menu MUST contain features in order: Maintenance, Depot, Digital (DCC), Railway Tracks
- **FR-009**: System MUST use the following icon mappings: Home=LayoutDashboard, Collection=TrainFront, Finance=Wallet, Wishlists=Heart, Maintenance=Wrench, Depot=Warehouse, Digital (DCC)=Cpu, Railway Tracks=TrainTrack, More=Ellipsis
- **FR-010**: System MUST use updated feature names: "Home" (formerly Dashboard), "Finance" (formerly Budget Tracking), "Wishlists" (formerly Wish lists), "Digital (DCC)" (formerly My Digital rolling stocks), "Depot", "Railway Tracks" (formerly My Tracks)
- **FR-011**: All navigation labels MUST use Paraglide-JS translation functions for localization
- **FR-012**: System MUST hide the bottom bar on desktop viewports and show the full sidebar
- **FR-013**: System MUST hide the full sidebar on mobile viewports and show the bottom bar
- **FR-014**: Clicking a navigation item MUST navigate to the corresponding feature route
- **FR-015**: More menu MUST close when a user selects a feature, taps outside the menu, or when viewport is resized to desktop (≥768px)

### Key Entities

- **Navigation Item**: Represents a single feature in the navigation system; attributes include name, icon, route, and priority (primary vs secondary)
- **Viewport State**: Represents the current device context; determines which navigation layout to render (desktop sidebar or mobile bottom bar)
- **More Menu State**: Represents the open/closed state of the secondary features menu on mobile

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: Desktop users can access any of the 9 features with a single click from the sidebar
- **SC-002**: Mobile users can access any of the 4 primary features with a single tap from the bottom bar
- **SC-003**: Mobile users can access any of the 4 secondary features with 2 taps (More button + feature selection)
- **SC-004**: Navigation adapts to viewport changes within 300ms of resize
- **SC-005**: Users can identify the active feature by its visual state in both desktop and mobile layouts
- **SC-006**: 100% of navigation labels are properly localized using Paraglide-JS translations
- **SC-007**: More button correctly indicates active state when user is on any secondary feature route

## Assumptions _(mandatory)_

- The app uses a responsive breakpoint at 768px to distinguish mobile from desktop viewports
- The existing routing structure supports all 9 features with defined routes
- Paraglide-JS is already integrated and configured in the application
- The icon library (lucide-svelte or similar) includes all required icons
- The app follows a mobile-first responsive design approach
- Users primarily access desktop features via mouse/trackpad and mobile features via touch
- The bottom bar pattern is appropriate for the target user base
- Secondary features (in More menu) are accessed less frequently than primary features

## Out of Scope _(optional)_

- Customizable navigation order or user-defined feature priorities
- Tablet-specific layouts (tablets use desktop or mobile based on viewport width)
- Gesture-based navigation (swipe to open/close menus)
- Persistent More menu state across sessions
- Animation duration or easing customization
- Navigation search or filtering functionality
- Collapsible sidebar on desktop
- Multiple levels of nested navigation
- User analytics or usage tracking for navigation patterns
- Accessibility features beyond standard keyboard navigation (will be addressed in a separate accessibility audit)

## Dependencies _(optional)_

- Icon library must provide LayoutDashboard, TrainFront, Wallet, Heart, Wrench, Warehouse, Cpu, TrainTrack, and Ellipsis icons
- Paraglide-JS localization system must support all required language translations
- Routing system must support direct links to all 9 feature routes
- CSS framework or utility library must support responsive breakpoints at 768px
- Bottom sheet/drawer component must be available or implemented for the More menu

## Risks & Mitigations _(optional)_

- **Risk**: Users may not discover secondary features hidden in the More menu  
  **Mitigation**: Consider onboarding tooltips or first-use hints; monitor usage analytics for secondary features

- **Risk**: 5 slots may be too many for small mobile screens  
  **Mitigation**: Test on devices with widths down to 320px; ensure tap targets meet accessibility guidelines (≥44px)

- **Risk**: More button active state may confuse users (not immediately obvious which secondary feature is active)  
  **Mitigation**: Ensure the More menu opens with the active feature visually highlighted

- **Risk**: Rapid viewport resizing may cause layout flicker  
  **Mitigation**: Implement debouncing on resize events; use CSS transitions for smooth layout changes

## Notes _(optional)_

- The feature naming changes (Dashboard → Home, Budget Tracking → Finance, etc.) should be reflected in translation keys
- Old translation keys should be removed or marked as deprecated
- Consider visual hierarchy in the More menu to distinguish it from the primary bottom bar slots
- The More menu should use the same visual design language as the rest of the navigation
- Desktop sidebar may benefit from visual separators between primary and secondary features for clarity
