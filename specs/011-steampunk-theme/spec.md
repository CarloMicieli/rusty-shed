# Feature Specification: Modern Steampunk Theme System

**Feature Branch**: `011-steampunk-theme`  
**Created**: 2026-01-30  
**Status**: Draft  
**Input**: User description: "Transform Rusty Shed from the default Cerberus dark theme to a bespoke 'Modern Steampunk' design system featuring dual light/dark themes optimized for model railway collectors."

## Overview

### Objective

Transform Rusty Shed from the default Cerberus dark theme to a bespoke "Modern Steampunk" design system featuring dual light/dark themes optimized for model railway collectors.

### Design Philosophy

- **Light Theme (Parchment & Brass):** Evokes a Victorian engineer's ledger — warm, aged paper tones with polished brass accents
- **Dark Theme (Iron & Copper):** Industrial control room aesthetic — cold iron surfaces with burnished copper highlights
- **Shared DNA:** Both themes use identical component structures, spacing, and interaction patterns

### Technical Constraints

- Skeleton UI 4.x design token architecture
- Tailwind CSS 4 with `@theme` directive
- CSS-only textures and patterns (no static image assets)
- Theme preference persisted via Tauri backend settings
- Cross-platform: Desktop (Tauri window) and responsive mobile

## User Scenarios & Testing _(mandatory)_

### User Story 1 - Theme Persistence & Switching (Priority: P1)

As a user, I want my theme preference (light/dark/system) to persist across app restarts so I don't have to reconfigure my visual preference each time.

**Why this priority**: Theme persistence is foundational—without it, users would lose their preference on every restart, creating frustration.

**Independent Test**: Can be fully tested by selecting a theme, closing and reopening the app, and verifying the theme persists.

**Acceptance Scenarios**:

1. **Given** I have set my theme to "steampunk-dark", **When** I restart the app, **Then** the dark theme is automatically applied
2. **Given** I have set my theme to "system", **When** my OS switches to dark mode, **Then** the app follows and displays the dark theme
3. **Given** I am in the settings page, **When** I switch themes, **Then** the change applies immediately without page reload

---

### User Story 2 - Light Theme Experience (Priority: P2)

As a user, I want to use a light "Parchment & Brass" theme that evokes a Victorian engineer's ledger for comfortable daytime use.

**Why this priority**: Light mode is essential for daytime use and accessibility—many users prefer or require lighter interfaces.

**Independent Test**: Can be tested by enabling light theme and verifying all pages display correctly with warm paper tones and brass accents.

**Acceptance Scenarios**:

1. **Given** light theme is active, **When** I view any page, **Then** I see aged paper backgrounds (#F4EBD0) with brass (#B8860B) accents
2. **Given** light theme is active, **When** I read text, **Then** all text meets WCAG AA contrast ratios (4.5:1 minimum)
3. **Given** light theme is active, **When** I view cards and panels, **Then** they display parchment texture styling

---

### User Story 3 - Dark Theme Experience (Priority: P2)

As a user, I want to use a dark "Iron & Copper" theme for comfortable evening/low-light use that maintains the steampunk industrial aesthetic.

**Why this priority**: Dark mode is essential for eye comfort in low-light conditions and was the original app experience.

**Independent Test**: Can be tested by enabling dark theme and verifying all pages display correctly with cold iron surfaces and copper highlights.

**Acceptance Scenarios**:

1. **Given** dark theme is active, **When** I view any page, **Then** I see cold iron backgrounds (#1A1A1B) with copper (#CD7F32) accents
2. **Given** dark theme is active, **When** I read text, **Then** all text meets WCAG AA contrast ratios
3. **Given** dark theme is active, **When** I view interactive elements, **Then** they have visible furnace-orange (#FF4500) accent highlights

---

### User Story 4 - Steampunk Component Styling (Priority: P3)

As a user, I want UI components to feature steampunk design elements (rivets, metal textures, mechanical styling) so the interface feels immersive.

**Why this priority**: Component styling creates the full themed experience but depends on base colors being established first.

**Independent Test**: Can be tested by navigating to pages with cards and verifying riveted panel styling, metal gradients, and mechanical design elements.

**Acceptance Scenarios**:

1. **Given** I view a card component, **When** displayed, **Then** it shows riveted panel styling with corner rivets
2. **Given** I interact with a button, **When** I click it, **Then** it animates with a mechanical lever press effect
3. **Given** I view dividers, **When** displayed, **Then** they use the train track pattern styling

---

### User Story 5 - Responsive Steampunk Design (Priority: P4)

As a mobile user, I want the steampunk theme to adapt appropriately for touch interfaces without performance issues.

**Why this priority**: Mobile experience is important but secondary to desktop where the full theme experience is showcased.

**Independent Test**: Can be tested by viewing the app on mobile viewport sizes and verifying textures are disabled and touch targets are adequate.

**Acceptance Scenarios**:

1. **Given** I am on mobile or tablet (<1024px), **When** I view the interface, **Then** complex textures are disabled for performance
2. **Given** I am on mobile, **When** I tap buttons, **Then** touch targets are at least 44×44px
3. **Given** I am on tablet, **When** I view the sidebar, **Then** it is collapsible and uses simplified decorations

---

### Edge Cases

- What happens when users have high contrast mode enabled? Theme should not interfere with OS accessibility features.
- What happens with `prefers-reduced-motion`? All animations must be disabled or minimized.
- What happens with system theme when OS is restarted? App should re-detect preference on mount.
- What happens if Google Fonts fail to load? Fallback fonts must be visually acceptable.

## Requirements _(mandatory)_

### Functional Requirements

#### Theme Persistence (Backend)

- **FR-001**: System MUST persist theme preference in SQLite via existing Tauri settings infrastructure with column `theme`
- **FR-002**: System MUST support values: `"steampunk-light"`, `"steampunk-dark"`, `"system"` with default `"system"`
- **FR-003**: System MUST extend existing `get_settings` and `update_settings` Tauri commands to include theme field

#### Theme State Management (Frontend)

- **FR-004**: System MUST implement `themeStore.svelte.ts` with state shape: `{ current: ThemeValue, resolved: 'light' | 'dark', isLoading: boolean }`
- **FR-005**: System MUST initialize theme from Tauri settings on app mount in `+layout.svelte`
- **FR-006**: System MUST sync resolved theme to `document.body.dataset.theme` for CSS targeting
- **FR-007**: System MUST detect OS theme via `window.matchMedia('(prefers-color-scheme: dark)')` when theme is `"system"`

#### Color Tokens

- **FR-008**: System MUST define complete color token scales (50-950) for: surface, primary, secondary, tertiary, accent, error, success, warning
- **FR-009**: Light theme primary MUST be Burnished Gold/Brass (#B8860B core)
- **FR-010**: Dark theme primary MUST be Polished Copper (#CD7F32 core)
- **FR-011**: All color combinations MUST meet WCAG 2.1 AA contrast ratios (4.5:1 text, 3:1 UI)

#### Typography

- **FR-012**: System MUST load Google Fonts: Cinzel Decorative (headings), Courier Prime (body), Spectral (accent)
- **FR-013**: System MUST use `display=swap` for font loading to prevent FOUT
- **FR-014**: System MUST provide fallback font stacks for each font role

#### CSS-Only Textures

- **FR-015**: System MUST implement textures as CSS gradients only (no image assets)
- **FR-016**: System MUST disable complex textures on mobile (<1024px) for performance
- **FR-017**: System MUST respect `prefers-reduced-motion` for animated textures

#### Component Variants

- **FR-018**: System MUST define `variant-steampunk-*` CSS classes that extend (not override) Skeleton defaults
- **FR-019**: System MUST implement variants: riveted, embossed, brass, copper, parchment, gauge, valve, lever
- **FR-020**: System MUST preserve all existing Skeleton component functionality

#### New Components

- **FR-021**: System MUST implement RivetedCard.svelte with configurable rivet placement
- **FR-022**: System MUST implement ToggleValve.svelte as themed toggle switch
- **FR-023**: System MUST implement RailDivider.svelte with train track pattern
- **FR-024**: System MUST implement PressureGauge.svelte for visual progress indicators

#### Accessibility

- **FR-025**: System MUST provide visible focus states with 3:1 minimum contrast
- **FR-026**: System MUST ensure touch targets are minimum 44×44px on mobile
- **FR-027**: System MUST support keyboard navigation for all interactive elements

### Key Entities

- **ThemeValue**: Enum type `"steampunk-light" | "steampunk-dark" | "system"`
- **ThemeState**: Store state containing current preference, resolved theme, and loading state
- **Theme Configuration**: CSS custom properties organized into light/dark theme files
- **Component Variants**: CSS classes prefixed with `variant-steampunk-*` for themed styling

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: Theme preference persists correctly across app restarts (verified via Tauri settings)
- **SC-002**: System theme detection works within 100ms of OS preference change
- **SC-003**: All text meets WCAG 2.1 AA contrast (4.5:1 normal text, 3:1 large text, 3:1 UI components)
- **SC-004**: All Skeleton component functionality remains intact (no regressions in existing features)
- **SC-005**: Complex textures disabled on viewports <1024px (verified via CSS media queries)
- **SC-006**: All animations respect `prefers-reduced-motion` (verified via media query test)
- **SC-007**: Touch targets ≥44×44px on mobile devices
- **SC-008**: Google Fonts load without visible FOUT (Font loading tested with network throttling)
- **SC-009**: No new Lighthouse performance regression >5% on mobile audit

## Assumptions

- The `user_settings` table and Tauri commands already exist or can be extended for theme storage
- Paraglide will be used for theme label localization ("Parchment & Brass", "Iron & Copper")
- Lucide icons will be kept but colored with theme tokens (no custom steampunk icon set)
- The settings page route exists or will be created for theme switching UI
- Charts use LayerChart which can accept custom color tokens
