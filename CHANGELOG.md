# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
