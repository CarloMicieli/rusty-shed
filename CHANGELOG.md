# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added - Feature 013: Responsive Navigation System

#### Navigation Architecture

- **Responsive Navigation System** with dual-layout design:
  - **Desktop Navigation** (≥768px): Full sidebar with all 8 features visible
    - Home, Collection, Finance, Wishlists, Maintenance, Depot, Digital (DCC), Railway Tracks
    - Persistent sidebar for primary workspace navigation
    - Active state highlighting based on current route
  - **Mobile Navigation** (<768px): 5-slot bottom bar + More menu
    - 4 primary features: Home, Collection, Finance, Wishlists (optimized for mobile usage)
    - More button revealing 4 secondary features in bottom sheet: Maintenance, Depot, Digital (DCC), Railway Tracks
    - Touch-optimized 64px tap targets (WCAG AAA compliance)
    - Slide-up bottom sheet for secondary features

#### New Components

- **SidebarNavigation.svelte** - Desktop sidebar (updated from legacy version)
  - Configuration-driven feature list via NAVIGATION_ITEMS
  - Active state detection with route prefix matching support
  - Responsive visibility: `hidden lg:flex` (desktop only)
  - Locale-reactive rendering via `{#key locale}` block
  - Badge support for feature counts (e.g., wishlist count)
  - Settings button and app version footer
- **BottomNavigation.svelte** - Mobile bottom bar (updated from legacy version)
  - 5-slot layout: 4 primary item links + More button
  - Responsive visibility: `md:hidden` (mobile only)
  - More button with active state detection for secondary features
  - Slide-up MoreMenu integration for secondary features
  - Locale-reactive rendering via `{#key locale}` block
  - Accessible aria-label for More button via Paraglide
- **MoreMenu.svelte** - Bottom sheet drawer (NEW)
  - Secondary features in slide-up bottom sheet
  - Uses shadcn-svelte Sheet component
  - Active state highlighting for current secondary feature
  - Auto-close on feature selection or backdrop tap
  - ESC key support for accessibility
  - Locale-reactive rendering via `{#key locale}` block

#### New Modules

- **src/lib/components/navigation/types.ts** - Navigation type definitions
  - `NavigationItem`: Feature descriptor with id, label, icon, href, isPrimary, badgeCount, usePrefixMatch
  - `SidebarNavigationProps`, `BottomNavigationProps`, `MoreMenuProps` - Component contracts
- **src/lib/components/navigation/config.ts** - Centralized configuration
  - `NAVIGATION_ITEMS`: All 8 features with icons, Paraglide labels, and primary/secondary classification
  - `PRIMARY_ITEMS`: Filtered 4 primary features for mobile bottom bar
  - `SECONDARY_ITEMS`: Filtered 4 secondary features for More menu
  - Development-only validation: warns if primary count ≠ 4
- **src/lib/components/navigation/utils.ts** - Navigation utilities
  - `isActive()`: Detects active route with exact or prefix matching
  - `isMoreButtonActive()`: Returns true if any secondary feature route is active

#### New Tests

- **SidebarNavigation.test.ts** (9 tests): Desktop navigation behavior
  - All 9 items render with correct icons and labels
  - Active state applies to current route
  - Paraglide translations used for labels
  - Responsive classes applied correctly
  - Keyboard navigation support
- **BottomNavigation.test.ts** (9 tests): Mobile navigation behavior
  - 5-slot layout (4 links + 1 button)
  - Active state for primary features
  - Mobile/desktop visibility on responsive breakpoints
  - Touch targets ≥44px (64px with h-16)
  - More button active state detection
- **MoreMenu.test.ts** (9 tests): Secondary feature access
  - 4 secondary items display correctly
  - Opens/closes with touch interactions
  - Auto-close on feature selection
  - Prefix matching for secondary routes
- **Consistency.test.ts** (7 tests): Feature identity validation
  - Icons consistent across desktop/mobile (same lucide-svelte components)
  - Labels consistent (same Paraglide functions)
  - Feature names match specification
  - Icon mappings verified
  - Single source of truth in config
- **Localization.test.ts** (10 tests): i18n behavior
  - All labels use Paraglide message functions (no hardcoded strings)
  - MoreMenu uses Paraglide functions for all text
  - aria-label for More button uses `m.app_more_aria()`
  - Deprecated keys (app_dashboard, budget_title, etc.) removed

#### Updated Translations

- **messages/en.json**: Added 6 new keys
  - `app_home`: "Home" (replaces `app_dashboard`)
  - `app_finance`: "Finance" (replaces `budget_title`)
  - `app_digital_dcc`: "Digital (DCC)" (replaces `app_digital_roster`)
  - `app_railway_tracks`: "Railway Tracks" (replaces `app_tracks`)
  - `app_more`: "More"
  - `app_more_aria`: "Open more features menu"
- **messages/it.json**: Italian translations
  - `app_home`: "Home"
  - `app_finance`: "Finanze"
  - `app_digital_dcc`: "Digitale (DCC)"
  - `app_railway_tracks`: "Binari"
  - `app_more`: "Altro"
  - `app_more_aria`: "Apri menu funzionalità aggiuntive"

- **Deprecated keys removed** from both files:
  - `app_dashboard` (legacy "Dashboard")
  - `budget_title` (legacy "Budget Tracking")
  - `app_digital_roster` (legacy "My Digital Rolling Stocks")
  - `app_tracks` (legacy "My Tracks")

#### Technical Implementation

- **Configuration-Driven**: Single source of truth (NAVIGATION_ITEMS) used by all components
- **Reactive Localization**: All components wrapped in `{#key locale}` for instant language switching
- **Type Safety**: Full TypeScript support with strict mode
- **Responsive Design**: CSS media queries (`md:` breakpoint at 768px) - no JavaScript viewport detection
- **Accessibility**: WCAG AA compliant with 64px touch targets, keyboard navigation, ARIA labels
- **Performance**: Instant CSS transitions (<300ms), no animation delays
- **Icon Consistency**: lucide-svelte icons across all viewports (Home, Collection, Finance, Wishlists, Maintenance, Depot, CPU/DCC, TrainTrack, Ellipsis/More)

### Fixed - Feature 012: shadcn-svelte Migration

### Added - Feature 012: shadcn-svelte Migration

#### New Components

- **shadcn-svelte UI Components** (v1.1.1): Complete replacement of Skeleton UI 4.9.0
  - `Button` - Primary CTA component with 5 variants (default, destructive, outline, ghost, link) and 3 sizes
  - `Badge` - Status indicators with 4 variants (default, secondary, destructive, outline)
  - `Input` - Form input with accessibility support (aria-invalid, aria-describedby)
  - `Textarea` - Multi-line text input with Steampunk styling
  - `Checkbox` - Accessible checkbox with ARIA labels and keyboard navigation
  - `Select` - Native select dropdown with consistent styling
  - `RadioGroup` - Radio button group with accessibility
  - `Dialog` - Modal dialog with ESC key support, ARIA labels, and bindable open state
  - `Sheet` - Slide-out drawer component
  - `Card` - Content container with header, content, footer sections
  - `Table` - Data table with sortable columns and pagination support
  - `Alert` - Notification component with 4 variants (default, info, warning, destructive)
  - `Toggle` - Toggle switch component

#### Component Features

- **Tailwind CSS 4.1.18**: Upgraded from v3, automatic CSS purging via Vite plugin
- **Steampunk Theme Integration**: All components styled with custom Steampunk design system
  - Bronze/copper accents with gear motifs
  - Vintage typography (tracking-widest)
  - Consistent dark mode support
- **Accessibility Enhancements**:
  - ARIA labels and descriptions on all interactive components
  - Keyboard navigation (ESC, Enter, Tab)
  - Screen reader support
  - Focus visible indicators
  - WCAG 2.1 AA compliant color contrast

#### Developer Experience

- **Comprehensive Documentation**:
  - `/docs/SHADCN_ONBOARDING.md` - 890-line developer guide
  - Quick start guide
  - Component reference with code examples
  - Customization patterns
  - TypeScript best practices
  - Testing strategies
- **TypeScript Support**:
  - `/src/lib/components/types.ts` - Centralized type exports
  - Strict type checking for all component props
  - IntelliSense support in VSCode
- **Example Components**:
  - `LoginFormExample.svelte` - Form validation patterns
  - `DataTableExample.svelte` - Sorting and pagination implementation
  - `DialogFormExample.svelte` - Modal form workflows

#### UI Consistency

- **PageHeader Component**: Standardized 3-tier page headers across all routes
  - Section label (e.g., "Command Center")
  - Page title
  - Description text
  - Action buttons via snippet support
  - Applied to: Dashboard, Depot, Maintenance, Budget, Settings pages

#### Code Quality

- **Build Optimization**:
  - Tailwind CSS automatic purging (production builds)
  - Manual chunk splitting for lucide-svelte icons
  - Reduced bundle size with tree-shaking
- **Type Safety**:
  - Zero TypeScript errors
  - Zero Svelte errors via svelte-check
  - Strict prop validation

### Changed

- **Replaced Skeleton UI** with shadcn-svelte across all components
  - Button styling and variants
  - Form inputs (text, email, password, textarea)
  - Modal dialogs and sheets
  - Card layouts
  - Table components
  - Badge and status indicators
- **Updated All Routes** to use new shadcn components:
  - `/my-dashboard` - Dashboard stats and charts
  - `/my-depot` - Rolling stock depot views
  - `/my-collection` - Collection management
  - `/my-maintenance` - Maintenance tracking
  - `/my-budget` - Budget configuration
  - `/my-settings` - Application settings
  - `/my-tracks` - Track inventory
  - `/my-wishlists` - Wishlist management
  - `/my-digital-roster` - Digital roster management
- **Migrated Feature Modules**:
  - Budget tracking components
  - Collection management forms
  - Depot table and cards
  - Digital roster tables
  - Maintenance cards
  - Track inventory dialogs
  - Wishlist forms

### Fixed

- **Accessibility Issues**:
  - Added missing ARIA labels to Checkbox component
  - Enhanced Dialog keyboard navigation (ESC key)
  - Fixed focus management in modals
  - Improved screen reader support across forms
- **TypeScript Errors**:
  - Fixed sortField type assertions in DataTableExample
  - Added aria-invalid prop support to Input component
  - Made Dialog.open bindable with $bindable()
  - Resolved onclick attribute issues in TableHead
- **UI Consistency**:
  - Standardized page headers with 3-tier structure
  - Fixed unclosed div tags in my-budget page
  - Consistent button sizing and spacing
  - Uniform color palette across components

### Removed

- **Skeleton UI 4.9.0** - Completely removed from dependencies
  - `@skeletonlabs/skeleton` package
  - `@skeletonlabs/tw-plugin` Tailwind plugin
  - All Skeleton-specific classes and utilities
- **Deprecated Components**:
  - Removed old Skeleton button classes
  - Removed `variant-form-material` classes
  - Removed Skeleton modal and drawer components
  - Cleaned up legacy form styling

### Performance

- **Bundle Size**: Optimized with Tailwind CSS purging
- **Load Time**: No performance degradation (verified with Lighthouse)
- **Runtime**: Reduced component render time with lighter shadcn components

### Migration Notes

This migration maintains 100% feature parity with the previous Skeleton UI implementation while adding:

- Better accessibility (ARIA support, keyboard navigation)
- Improved TypeScript safety
- Enhanced developer documentation
- Consistent Steampunk theming
- Smaller bundle size
- Better performance

**Breaking Changes**: None - all existing functionality preserved

**Upgrade Path**: Automatic - no user action required

**Testing**:

- ✅ All routes manually tested
- ✅ All components visually verified
- ✅ Dark mode tested and working
- ✅ Accessibility audit passed (WCAG 2.1 AA)
- ✅ TypeScript strict mode: 0 errors
- ✅ svelte-check: 0 errors, 0 warnings

---

## [Previous Releases]

_(Previous changelog entries will be added here)_
