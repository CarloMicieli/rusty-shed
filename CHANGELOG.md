# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added - Feature 043: Mobile Redesign (Tauri Mobile WebView Support)

#### Mobile-First Responsive Architecture

- **Viewport-Gated Mobile Behavior** (< 768px):
  - CSS-first responsive switching using Tailwind media queries
  - Safe-area inset handling for notched devices (iOS, Android)
  - Preservation of desktop behavior at 768px and above
  - Zero regression for existing desktop workflows

- **Safe-Area Utilities** (`src/routes/layout.css`):
  - Tailwind v4 custom utilities for device safe areas
  - `safe-area-pad-*` classes for padding insets
  - `touch-hover` variant for tap feedback on mobile
  - GPU-friendly compositor hints for smooth animations

#### Mobile Navigation & Readability (US1)

- **Responsive Bottom Navigation** (`src/lib/components/BottomNavigation.svelte`):
  - 4 primary navigation items (Dashboard, Collection, Maintenance, Wishlists)
  - "More" drawer for secondary destinations (Settings, Debug)
  - Touch-friendly 64x64px navigation targets (well above 44px standard)
  - Mobile-safe label truncation for Italian and other long-text languages
  - Indicator highlight for active route with glow effect

- **App Shell Updates** (`src/routes/+layout.svelte`):
  - Dynamic page-title context for mobile header display
  - Drawer registry with bounded stack (max depth 2)
  - Hardware back-button synchronization with drawer state
  - Safe-area padding for notched devices

- **Mobile Page Titles** (`src/lib/state/page-title.svelte`):
  - Context-based title management for dynamic headers
  - Automatic title clearing on route change
  - Integration with mobile header display

#### Mobile Collection Workflow (US2)

- **Touch-Optimized Collection View** (`src/lib/features/collection/CollectionDashboard.svelte`):
  - Single-column layout enforced on mobile (`itemMinWidth=320px`)
  - Contextual FAB for quick model additions
  - Filter chip interface with 36x36px remove buttons (exception sizing)
  - Touch-target compliance: 44x44px for primary controls
  - Hide table toggle on mobile (layout inappropriate)

- **Responsive Preview Cards** (`src/lib/components/RailwayModelPreviewCard.svelte`):
  - Option B reflow for mobile: Category badge repositioning
  - Readable typography at small viewports
  - Proper aspect-ratio handling for images

#### Mobile Editing Through Sheets (US3)

- **Bottom-Sheet Drawer System** (`src/lib/components/drawer/DrawerShell.svelte`):
  - Mobile: Full-width bottom sheets with gesture dismiss
  - Desktop: Side-panel drawer (768px and above)
  - Bounded stack: Parent + one child maximum
  - GPU-optimized transform animations (`translateY`)
  - Reduced-motion fallback for accessibility

- **Unified Edit Flow**:
  - Disable inline editing on mobile (redirects to sheet flow)
  - Parent/child sheet nesting for detail-level actions
  - Back-button pop behavior: Child first, then parent
  - Camera capability probing with graceful fallback

- **Media Upload Fallback** (`src/lib/components/model-details/ImageUpload.svelte`):
  - Capability detection for camera access
  - Automatic fallback to file picker if camera unavailable
  - Form state preservation across fallback transitions
  - Clear UX messaging for capability constraints

#### Mobile Experience Stability (US4)

- **Regression Testing**:
  - Desktop parity tests at 1280x800
  - Multilingual overflow tests (en/it at 375px)
  - Startup placeholder behavior (non-blocking)

- **Touch-Target Audit**:
  - All primary controls: ≥44x44px
  - Chip remove buttons: 36x36px (per spec exception)
  - Navigation items: 64x64px
  - Complete audit documented in test matrix

- **Mobile Metrics & Evidence**:
  - Success criteria validation (SC-001 through SC-006)
  - Startup timing assertions (<1s target)
  - Multilingual regression checklist
  - Rollout and desktop parity verification protocol

#### Localization (i18n)

- **Mobile-Specific Text** (`messages/en.json`, `messages/it.json`):
  - Bottom navigation labels (mobile-safe truncation key)
  - More drawer labels
  - Camera fallback notices (status + mode variables)
  - Touch target audit exceptions documentation
  - All strings defined in both English and Italian

#### Testing

- **Mobile-Specific Test Coverage**:
  - Drawer registry: Stack bounds, dismiss order, back-pop behavior
  - Mobile layout: Safe-area rendering, sheet interactions
  - Touch targets: 44x44px and 36x36px verification
  - Viewport simulation: 375x812 (baseline) and 430x932 (large)
  - Desktop parity: No regressions at 768px+ (104 tests)
  - Contract conformance: IPC and drawer/media operations (17 tests)

- **Regression Validation**:
  - 173 test files, 1693 tests total
  - 2097 Rust backend tests (zero failures)
  - Italian 375px overflow tests
  - Startup placeholder timing tests

- **Quality Gates**:
  - svelte-check: 0 errors, 0 warnings
  - ESLint: Pass
  - Prettier: All formatted
  - Cargo clippy: -D warnings (zero warnings)

#### Architecture & Decisions

- **CSS-First Responsive**: Avoids JS resize churn and layout thrashing
- **Bounded Sheet Registry**: Prevents UX ambiguity with depth > 2
- **Capability-Aware Fallback**: Camera probe with graceful file-picker fallback
- **Compositor-Safe Animations**: GPU-friendly transforms with reduced-motion support
- **Phased Rollout**: Milestone-based delivery with desktop parity checks

#### Files Modified

- **Frontend Components**: 23 files (navigation, collection, editing, drawer, media)
- **State Management**: page-title, match-media, drawer-registry
- **Test Infrastructure**: 16 new test files + helpers for mobile viewport simulation
- **Documentation**: Test matrix, contract mapping, metrics protocol, regression checklist
- **Localization**: messages/en.json, messages/it.json (new mobile-specific keys)
- **Build Configuration**: touch-hover variant registered in Tailwind

### Added - Feature 015: Model Image Upload System

#### Image Management Architecture

- **Model Image Upload System** with comprehensive validation and management:
  - **File Upload**: Upload JPEG, PNG, or WebP images via file dialog
    - File size validation (max 50MB)
    - Format validation using magic byte detection
    - Filename sanitization for cross-platform compatibility
  - **Drag & Drop**: Intuitive drag-and-drop interface
    - Visual feedback for drag states (hover, uploading)
    - Single-file enforcement with clear error messages
    - Support for all three image formats
  - **Image Replacement**: Seamless image replacement with automatic cleanup
    - Multi-format deletion (removes any existing format before upload)
    - Prevents orphaned files during format changes
    - Dynamic button label ("Upload Image" vs "Replace Image")
  - **Image Deletion**: Explicit deletion with confirmation dialog
    - Confirmation dialog using AlertDialog component
    - Idempotent deletion (no error if image doesn't exist)
    - Immediate UI update after successful deletion

#### Backend Implementation

- **Domain Layer** (`src-tauri/src/media/domain/`):
  - `ImageFormat` enum: JPEG, PNG, WebP with magic byte detection
  - `FileSize` value object: Validates max 50MB limit
  - `ModelImagePath`: Deterministic path generation (`{model_id}.{ext}`)
  - `ImageValidator`: Format and size validation with detailed errors
  - `ValidationError` and `StorageError` types for robust error handling

- **Application Layer** (`src-tauri/src/media/application/`):
  - `UploadModelImage`: Upload from file path with model validation
  - `UploadModelImageBytes`: Upload from drag-drop bytes
  - `DeleteModelImage`: Delete image with format-agnostic resolution
  - Unit tests: 32 validation tests, 13 upload tests, 5 delete tests

- **Infrastructure Layer** (`src-tauri/src/media/infrastructure/`):
  - `FileStorage`: Async file operations with tokio::fs
  - Configurable storage directory via Tauri config
  - Atomic file operations with proper error handling

- **Interface Layer** (`src-tauri/src/media/interface/`):
  - Tauri commands: `upload_model_image`, `upload_model_image_bytes`, `delete_model_image`
  - TypeScript bindings auto-generated via specta
  - Comprehensive error mapping for UI display

#### Frontend Implementation

- **ImageUpload Component** (`src/lib/components/model-details/ImageUpload.svelte`):
  - File dialog with format filter (.jpg, .jpeg, .png, .webp)
  - Conditional rendering: Upload vs Replace button based on image state
  - Delete button with destructive variant (red/warning color)
  - Loading states with disabled buttons during operations
  - Toast notifications for success/error feedback

- **ImageDropZone Component** (`src/lib/components/model-details/ImageDropZone.svelte`):
  - Drag-and-drop interface with visual feedback
  - Multi-file rejection with user-friendly error
  - Format validation via MIME type checking
  - Progress indicator during upload

- **ModelDetailsHeader Component**: Integrated image display with upload/replace controls

#### Localization

- **English Messages** (`messages/en.json`):
  - `upload_image`, `replace_image`, `delete_image`
  - `drag_and_drop_hint`, `drop_image_here`, `uploading`, `deleting`
  - Error messages: `upload_error_corrupted`, `upload_error_unsupported_format`, etc.
  - Confirmation: `confirm_delete_image_title`, `confirm_delete_image_description`

- **Italian Translations** (`messages/it.json`):
  - Complete translations for all upload, replace, and delete messages
  - User-friendly error messages in Italian

#### Testing

- **Backend Tests**: 1069 total tests (50 new tests for image upload feature)
  - Format validation: JPEG, PNG, WebP detection via magic bytes
  - Invalid format rejection: TIFF, BMP, PDF, TXT, GIF, corrupted files
  - File size validation: Empty files, oversized files, edge cases
  - Replacement flow: Cross-format replacement, orphan prevention
  - Delete flow: Model existence, idempotent deletion, all formats

- **Frontend Tests**: 211 total tests (22 new tests for image components)
  - ImageUpload: File dialog filter, upload flow, delete button visibility
  - ImageDropZone: Multi-file rejection, MIME validation, drag states
  - Error handling: Model not found, validation errors, unknown errors

- **Type Safety**: 0 TypeScript errors, full type coverage via bindings

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
