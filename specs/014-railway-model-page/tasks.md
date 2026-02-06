# Implementation Tasks: Railway Model Details Page (Feature 014)

**Feature**: Railway Model Details Page  
**Branch**: `014-railway-model-page`  
**Status**: Ready for Implementation  
**Created**: February 6, 2026

---

## Phase 1: Backend Setup - Media Module Structure

### T001: Create Media Module Directory Structure

- [ ] T001 Create domain/, application/, infrastructure/, interface/ directories in `src-tauri/src/media/`
- [ ] T001 Create mod.rs files for each layer with public exports
- [ ] T001 Create `src-tauri/src/media/mod.rs` as module root

**File Paths**:

- `src-tauri/src/media/mod.rs`
- `src-tauri/src/media/domain/mod.rs`
- `src-tauri/src/media/application/mod.rs`
- `src-tauri/src/media/infrastructure/mod.rs`
- `src-tauri/src/media/interface/mod.rs`

---

## Phase 2: Backend - Domain Layer

### T002: Implement ImageError Type

- [ ] T002 Create `src-tauri/src/media/domain/image_error.rs`
- [ ] T002 Define `ImageError` enum with variants: NotFound, InvalidPath, IoError, InvalidModelId
- [ ] T002 Implement Display, Error, From conversions
- [ ] T002 Add comprehensive rustdoc comments
- [ ] T002 Export from `domain/mod.rs`

**File Paths**: `src-tauri/src/media/domain/image_error.rs`

### T003: Implement RailwayModelImage Entity

- [ ] T003 Create `src-tauri/src/media/domain/railway_model_image.rs`
- [ ] T003 Define `RailwayModelImage` struct with id, path, exists fields
- [ ] T003 Implement `from_model_id()` method to resolve filename from RailwayModelId
- [ ] T003 Implement `resolve_filename()` to transform model ID (: → \_)
- [ ] T003 Add path validation helper
- [ ] T003 Add comprehensive rustdoc
- [ ] T003 Export from `domain/mod.rs`

**File Paths**: `src-tauri/src/media/domain/railway_model_image.rs`

**Dependencies**: Uses RailwayModelId from catalog domain

### T004: Implement ImagePlaceholder Value Object

- [ ] T004 Create `src-tauri/src/media/domain/image_placeholder.rs`
- [ ] T004 Define `ImagePlaceholder` value object with text and html fields
- [ ] T004 Implement `generate()` constructor method
- [ ] T004 Implement `html_content()` to return complete HTML string
- [ ] T004 Design placeholder HTML/CSS (responsive, accessible)
- [ ] T004 Add comprehensive rustdoc
- [ ] T004 Export from `domain/mod.rs`

**File Paths**: `src-tauri/src/media/domain/image_placeholder.rs`

**Design Requirements**:

- Responsive (mobile, tablet, desktop)
- Accessible semantic HTML
- Centered "No picture yet" text with optional icon
- Light background color or subtle gradient

### T005: Verify Domain Layer Compilation

- [ ] T005 Run `pnpm rust:check` - verify domain layer compiles
- [ ] T005 Ensure no unused imports or warnings
- [ ] T005 Verify rustdoc builds without errors

---

## Phase 3: Backend - Application Layer

### T006: Implement GetRailwayModelImage Use Case

- [ ] T006 Create `src-tauri/src/media/application/get_railway_model_image.rs`
- [ ] T006 Define `GetRailwayModelImage` struct with execute method
- [ ] T006 Signature: `async fn execute(model_id: &RailwayModelId, models_dir: &Path) -> Result<RailwayModelImage, ImageError>`
- [ ] T006 Call infrastructure to find image
- [ ] T006 Handle both found and not-found scenarios
- [ ] T006 Add comprehensive rustdoc and examples
- [ ] T006 Export from `application/mod.rs`

**File Paths**: `src-tauri/src/media/application/get_railway_model_image.rs`

### T007: Implement GetImagePlaceholder Use Case

- [ ] T007 Create `src-tauri/src/media/application/get_image_placeholder.rs`
- [ ] T007 Define `GetImagePlaceholder` struct with execute method
- [ ] T007 Signature: `fn execute() -> ImagePlaceholder`
- [ ] T007 Generate and return placeholder value object
- [ ] T007 Add comprehensive rustdoc
- [ ] T007 Export from `application/mod.rs`

**File Paths**: `src-tauri/src/media/application/get_image_placeholder.rs`

### T008: Verify Application Layer Compilation

- [ ] T008 Run `pnpm rust:check` - verify application layer compiles
- [ ] T008 Ensure no unused imports or warnings
- [ ] T008 Verify all use cases properly exported

---

## Phase 4: Backend - Infrastructure Layer

### T009: Implement ImageRepository Service

- [ ] T009 Create `src-tauri/src/media/infrastructure/image_repository.rs`
- [ ] T009 Implement `find_image()` async method
  - [ ] Try extensions: .png, .jpg, .jpeg
  - [ ] Use `tokio::fs::metadata()` for file existence
  - [ ] Canonicalize path (convert to absolute)
  - [ ] Return PathBuf on success, ImageError on failure
- [ ] T009 Implement `sanitize_path()` validation method
  - [ ] Check for path traversal attempts (..)
  - [ ] Validate all path components are Normal
  - [ ] Return error for suspicious paths
- [ ] T009 Add comprehensive rustdoc and examples
- [ ] T009 Add unit tests for edge cases
- [ ] T009 Export from `infrastructure/mod.rs`

**File Paths**: `src-tauri/src/media/infrastructure/image_repository.rs`

**Test Cases**:

- File exists with .png extension
- File exists with .jpg extension
- File exists with .jpeg extension
- File not found (return error)
- Path traversal attempt rejected
- Non-Unicode path handled

### T010: Implement PlaceholderGenerator Service

- [ ] T010 Create `src-tauri/src/media/infrastructure/placeholder_generator.rs`
- [ ] T010 Implement `generate_html()` function
  - [ ] Create responsive HTML/CSS placeholder
  - [ ] Use Tailwind CSS utility classes
  - [ ] Include "No picture yet" text
  - [ ] Make centered and visually consistent
  - [ ] Return complete HTML string
- [ ] T010 Design responsive layout
  - [ ] Mobile: single column, centered
  - [ ] Tablet: medium container
  - [ ] Desktop: large container
- [ ] T010 Ensure accessibility
  - [ ] Semantic HTML (use appropriate tags)
  - [ ] Color contrast sufficient
  - [ ] Screen reader friendly
- [ ] T010 Add comprehensive rustdoc
- [ ] T010 Export from `infrastructure/mod.rs`

**File Paths**: `src-tauri/src/media/infrastructure/placeholder_generator.rs`

**Design Specifications**:

- Background: light gray (#f3f4f6) or subtle gradient
- Text: "No picture yet" in readable font size
- Icon: optional (use Lucide icon if desired)
- Padding: generous whitespace
- Border: subtle rounded corners

### T011: Verify Infrastructure Layer Compilation

- [ ] T011 Run `pnpm rust:check` - verify infrastructure layer compiles
- [ ] T011 Ensure no unused imports or warnings
- [ ] T011 Run `pnpm rust:test` - verify all infrastructure tests pass

---

## Phase 5: Backend - Interface & Integration Layer

### T012: Implement RailwayModelImageResponse DTO

- [ ] T012 Create `src-tauri/src/media/interface/image_dto.rs`
- [ ] T012 Define `RailwayModelImageResponse` struct
  - [ ] `image_path: Option<String>` - file path if found
  - [ ] `placeholder_html: Option<String>` - HTML if no image
  - [ ] `has_image: bool` - quick flag
- [ ] T012 Implement Serialize for DTO
- [ ] T012 Add rustdoc and examples
- [ ] T012 Export from `interface/mod.rs`

**File Paths**: `src-tauri/src/media/interface/image_dto.rs`

### T013: Implement Tauri Command Handler

- [ ] T013 Create `src-tauri/src/media/interface/command_handlers.rs`
- [ ] T013 Implement `get_railway_model_image` command
  - [ ] Signature: `async fn get_railway_model_image(state: tauri::State<'_, AppState>, railway_model_id: RailwayModelId) -> Result<RailwayModelImageResponse, CommandError>`
  - [ ] Invoke application use case
  - [ ] Map ImageError to CommandError
  - [ ] Log operations
  - [ ] Return DTO with appropriate fields set
- [ ] T013 Add Tauri macros: `#[tauri::command]` and `#[specta::specta]`
- [ ] T013 Add comprehensive rustdoc
- [ ] T013 Handle both found and fallback scenarios
- [ ] T013 Export from `interface/mod.rs`

**File Paths**: `src-tauri/src/media/interface/command_handlers.rs`

### T014: Add Media Module to lib.rs

- [ ] T014 Add `pub mod media;` to module declarations in `src-tauri/src/lib.rs`
- [ ] T014 Import command handler: `use crate::media::interface::command_handlers as media_command_handlers;`
- [ ] T014 Add `media_command_handlers::get_railway_model_image` to `collect_commands![]` macro
- [ ] T014 Verify no duplicate commands

**File Paths**: `src-tauri/src/lib.rs`

### T015: Remove Old get_image_path Command

- [ ] T015 Remove `get_image_path()` function from `src-tauri/src/lib.rs`
- [ ] T015 Remove from `collect_commands![]` macro if not already done
- [ ] T015 Verify no other references to old command exist
- [ ] T015 Run `pnpm rust:check` to ensure no broken references

**File Paths**: `src-tauri/src/lib.rs`

### T016: Verify Backend Compilation and Tests

- [ ] T016 Run `pnpm rust:fmt` - format all Rust code
- [ ] T016 Run `pnpm rust:check` - verify entire backend compiles
- [ ] T016 Run `pnpm rust:test` - all unit tests pass
- [ ] T016 Run `pnpm rust:clippy` - zero warnings
- [ ] T016 Verify TypeScript bindings regenerate in `src/lib/bindings.ts`
- [ ] T016 Confirm `RailwayModelImageResponse` type appears in bindings

**Verification**: All backend infrastructure complete and tested

---

## Phase 6: Frontend - Route & Layout Structure

### T017: Create Railway Model Details Route

- [ ] T017 Create directory: `src/routes/models/[modelId]/`
- [ ] T017 Create `src/routes/models/[modelId]/+page.svelte` (main page)
- [ ] T017 Create `src/routes/models/[modelId]/+page.ts` (load function if needed)
- [ ] T017 Import layout, set metadata/title
- [ ] T017 Add TypeScript strict mode and proper typing

**File Paths**:

- `src/routes/models/[modelId]/+page.svelte`
- `src/routes/models/[modelId]/+page.ts` (optional)

### T018: Implement Page Header Component

- [ ] T018 Create `src/lib/components/ModelDetailsHeader.svelte`
- [ ] T018 Display hero image or placeholder
- [ ] T018 Show model description (title) prominently
- [ ] T018 Show manufacturer | product code (subtitle)
- [ ] T018 Display quick badges: Scale, Era, Power Method
- [ ] T018 Use Paraglide-JS for all text strings
- [ ] T018 Make responsive (mobile, tablet, desktop)
- [ ] T018 Add accessibility attributes (semantic HTML)
- [ ] T018 Style with Tailwind CSS + shadcn-svelte components
- [ ] T018 Add proper TypeScript types for props

**File Paths**: `src/lib/components/ModelDetailsHeader.svelte`

**Prop Interface**:

```typescript
interface Props {
  model: {
    id: string;
    description: string;
    manufacturer?: string;
    productCode?: string;
    scale?: string;
    era?: string;
    powerMethod?: string;
  };
  imageResponse?: {
    imagePath?: string;
    placeholderHtml?: string;
    hasImage: boolean;
  };
}
```

### T019: Create Tab Navigation Component

- [ ] T019 Create `src/lib/components/ModelDetailsTabs.svelte`
- [ ] T019 Implement two tabs: "Details" and "Rolling Stock"
- [ ] T019 Use shadcn-svelte Tabs component
- [ ] T019 Track active tab with Svelte $state
- [ ] T019 Preserve tab state when navigating
- [ ] T019 Use Paraglide-JS for tab labels
- [ ] T019 Style with Tailwind CSS
- [ ] T019 Add keyboard navigation support

**File Paths**: `src/lib/components/ModelDetailsTabs.svelte`

**Features**:

- Smooth tab switching (no page reload)
- Active tab indicator
- Tab content switching
- Accessibility (ARIA labels, keyboard support)

### T020: Implement Details Tab Content

- [ ] T020 Create `src/lib/components/ModelDetailsContent.svelte`
- [ ] T020 Display full model description
- [ ] T020 Handle missing/empty description gracefully
- [ ] T020 Use Paraglide-JS for labels
- [ ] T020 Make responsive
- [ ] T020 Style consistently with app

**File Paths**: `src/lib/components/ModelDetailsContent.svelte`

---

## Phase 7: Frontend - Rolling Stock & Components

### T021: Create RollingStockCard Component

- [ ] T021 Create `src/lib/components/RollingStockCard.svelte`
- [ ] T021 Card header: "{type} — {road_number}" (e.g., "Locomotive — 218 217-8")
- [ ] T021 Implement expand/collapse functionality
- [ ] T021 Expanded body displays: type, road number, depot, series code, railway company, country, livery, tech specs
- [ ] T021 Independent card states (one card expand doesn't affect others)
- [ ] T021 Smooth animations on expand/collapse (< 300ms target)
- [ ] T021 Use Svelte $state for expand/collapse state
- [ ] T021 Make responsive
- [ ] T021 Add accessibility (keyboard support, semantic HTML)
- [ ] T021 Use shadcn-svelte Collapsible or custom implementation

**File Paths**: `src/lib/components/RollingStockCard.svelte`

**Prop Interface**:

```typescript
interface Props {
  unit: {
    id: string;
    type: string;
    roadNumber: string;
    depot?: string;
    seriesCode?: string;
    railwayCompany?: string;
    country?: string;
    livery?: string;
    technicalSpecs?: Record<string, string>;
  };
}
```

### T022: Create TechSpecGrid Component

- [ ] T022 Create `src/lib/components/TechSpecGrid.svelte`
- [ ] T022 Display technical specifications in responsive grid/table
- [ ] T022 Format: key-value pairs (e.g., "Motor Type: DC")
- [ ] T022 Handle missing specs gracefully
- [ ] T022 Responsive: single column on mobile, multiple columns on larger screens
- [ ] T022 Style with Tailwind CSS
- [ ] T022 Semantic HTML (table if many specs, grid if few)

**File Paths**: `src/lib/components/TechSpecGrid.svelte`

**Prop Interface**:

```typescript
interface Props {
  specs: Record<string, string> | undefined;
}
```

### T023: Create HeaderBadges Component

- [ ] T023 Create `src/lib/components/HeaderBadges.svelte`
- [ ] T023 Display three badges: Scale, Era, Power Method
- [ ] T023 Use shadcn-svelte Badge component or custom styled divs
- [ ] T023 Make responsive (wrap on mobile)
- [ ] T023 Style consistently
- [ ] T023 Handle missing values gracefully

**File Paths**: `src/lib/components/HeaderBadges.svelte`

**Prop Interface**:

```typescript
interface Props {
  scale?: string;
  era?: string;
  powerMethod?: string;
}
```

### T024: Create ImagePlaceholder Component

- [ ] T024 Create `src/lib/components/ImagePlaceholder.svelte`
- [ ] T024 Render placeholder when no image available
- [ ] T024 Use HTML from backend response
- [ ] T024 Style with consistent dimensions
- [ ] T024 Make responsive
- [ ] T024 Optional: Add fade-in animation on load

**File Paths**: `src/lib/components/ImagePlaceholder.svelte`

**Prop Interface**:

```typescript
interface Props {
  placeholderHtml?: string;
  hasImage: boolean;
  imagePath?: string;
}
```

---

## Phase 8: Frontend - Data Binding & State

### T025: Implement Model Data Loading

- [x] T025 In `+page.ts` or `+page.svelte`: fetch railway model by ID
- [x] T025 Call `get_railway_model_by_id` from bindings
- [x] T025 Handle loading, success, error states
- [x] T025 Extract RailwayModelId from route parameter `[modelId]`
- [x] T025 Use proper error handling

**File Paths**: `src/routes/models/[modelId]/+page.ts` or `+page.svelte` script section

### T026: Implement Image Retrieval

- [x] T026 Call `get_railway_model_image` from bindings in page
- [x] T026 Pass RailwayModelId to image command
- [x] T026 Destructure response: `{ imagePath, placeholderHtml, hasImage }`
- [x] T026 Handle loading, success, error states
- [x] T026 Pass to header component for display

**File Paths**: `src/routes/models/[modelId]/+page.svelte` script section

### T027: Implement Rolling Stock Data Loading

- [x] T027 Query backend for rolling stock units owned for this model
- [x] T027 Handle empty state (no units)
- [x] T027 Pass list to RollingStock tab
- [x] T027 Handle error scenarios

**File Paths**: `src/routes/models/[modelId]/+page.svelte` script section

### T028: Assemble Page Layout

- [x] T028 In `+page.svelte`: import all components
- [x] T028 Arrange: Header → Tabs → Tab Content
- [x] T028 Pass all data to components via props
- [x] T028 Use Svelte $state for tab selection
- [x] T028 Add loading skeleton or spinner
- [x] T028 Add error boundary component

**File Paths**: `src/routes/models/[modelId]/+page.svelte`

---

## Phase 9: Frontend - Styling & Localization

### T029: Apply Tailwind CSS Styling

- [X] T029 Review all components for Tailwind utility classes
- [X] T029 Ensure consistent spacing, colors, typography
- [X] T029 Verify responsive design on mobile (320px), tablet (768px), desktop (1920px)
- [X] T029 Test dark mode compatibility (if applicable)
- [X] T029 Use shadcn-svelte component styling consistently
- [X] T029 Remove any custom CSS (use utilities instead)

**File Paths**: All component files

### T030: Implement Paraglide-JS Localization

- [X] T030 Extract all hardcoded user-facing text
- [X] T030 Add to `messages/en.json`
- [X] T030 Add translations to `messages/it.json`
- [X] T030 Import i18n runtime: `import { t } from '$lib/paraglide/runtime'`
- [X] T030 Replace all strings with `t('message.key')`
- [X] T030 Run `pnpm prepare` to compile messages
- [X] T030 Test both English and Italian in browser

**Strings to Localize**:

- Tab labels: "Details", "Rolling Stock"
- Column headers: "Type", "Road Number", "Depot", "Series Code", "Railway Company", "Country", "Livery"
- Section labels: "Technical Specifications"
- Empty states: "No units available"
- Error messages
- Placeholder: "No picture yet"

**File Paths**:

- All Svelte component files (replace text with t() calls)
- `messages/en.json` (add English strings)
- `messages/it.json` (add Italian strings)

### T031: Add Accessibility Features

- [X] T031 Review all components for semantic HTML
- [X] T031 Add ARIA labels where needed
- [X] T031 Ensure keyboard navigation works (Tab, Enter, Arrow keys)
- [X] T031 Test with screen reader (NVDA or JAWS)
- [X] T031 Verify color contrast ratios (WCAG 2.1 AA minimum)
- [X] T031 Ensure focus indicators visible
- [X] T031 Test with keyboard only (no mouse)

**File Paths**: All component files

### T032: Verify Frontend Styling

- [X] T032 Run `pnpm check` - TypeScript type check passes
- [X] T032 Run `pnpm lint` - ESLint passes
- [X] T032 Run `pnpm format` - Prettier formatting applied
- [X] T032 Review all files in browser at different viewport sizes
- [X] T032 Manual accessibility audit with screen reader

---

## Phase 10: Frontend - Testing

### T033: Create Page Component Tests

- [X] T033 Create `src/routes/models/[modelId]/+page.test.ts`
- [X] T033 Test page loads with valid model ID
- [X] T033 Test renders model information correctly
- [X] T033 Test image loads or placeholder displays
- [X] T033 Test tabs switch content
- [X] T033 Use Vitest + Playwright
- [X] T033 Add tests for error scenarios

**File Paths**: `src/routes/models/[modelId]/__tests__/+page.test.ts`

### T034: Create Component Unit Tests

- [X] T034 Create tests for ModelDetailsHeader.svelte
- [X] T034 Create tests for RollingStockCard.svelte
- [X] T034 Create tests for TechSpecGrid.svelte
- [X] T034 Create tests for ModelDetailsTabs.svelte
- [X] T034 Test props rendering
- [X] T034 Test user interactions
- [X] T034 Test accessibility

**File Paths**: `src/lib/components/__tests__/`

### T035: Test Localization

- [X] T035 Test English language strings display correctly
- [X] T035 Test Italian language strings display correctly
- [X] T035 Test language switching (if supported)
- [X] T035 Verify all UI text is localized

**File Paths**: Manual testing in browser

### T036: Cross-Platform Testing

- [X] T036 Test on Windows (native installer)
- [X] T036 Test on macOS (.dmg)
- [X] T036 Test on Linux (.AppImage)
- [X] T036 Verify page load performance (< 1 second target)
- [X] T036 Check animations smooth (< 300ms)
- [X] T036 Test image loading on slow network
- [X] T036 Test with no image available (placeholder)

---

## Phase 11: Integration & Verification

### T037: Test Backend-Frontend Integration

- [ ] T037 Run `pnpm dev` to start dev server
- [ ] T037 Navigate to collection, click model card
- [ ] T037 Verify details page opens with correct model
- [ ] T037 Verify image loads or placeholder displays
- [ ] T037 Verify tabs switch correctly
- [ ] T037 Verify rolling stock cards expand/collapse
- [ ] T037 Verify all data displays correctly
- [ ] T037 Check browser console for errors
- [ ] T037 Verify TypeScript bindings working

**Manual Testing Checklist**:

- Model description displays
- Manufacturer and product code show
- Badges (scale, era, power) display
- Image loads from filesystem or placeholder shows
- Details tab shows full description
- Rolling Stock tab shows card list
- Cards expand on click
- Cards display all fields
- Tech specs display in grid
- Tab state preserved when switching
- No console errors

### T038: Verify All Requirements Met

- [ ] T038 Map each FR (functional requirement) to implemented feature
- [ ] T038 Verify FR-001: Route `/models/{modelId}` works
- [ ] T038 Verify FR-002: Header displays title
- [ ] T038 Verify FR-003: Header displays manufacturer | product code
- [ ] T038 Verify FR-004: Hero image or placeholder displays
- [ ] T038 Verify FR-005: Badges show scale, era, power method
- [ ] T038 Verify FR-006: Two tabs present (Details, Rolling Stock)
- [ ] T038 Verify FR-007: Details tab shows description
- [ ] T038 Verify FR-008: Rolling Stock tab shows units
- [ ] T038 Verify FR-009: Card headers show "{type} — {road_number}"
- [ ] T038 Verify FR-010: Expanded cards show all details
- [ ] T038 Verify FR-011: Tech specs in responsive grid
- [ ] T038 Verify FR-012: Cards expand/collapse independently
- [ ] T038 Verify FR-013: Tab state preserved
- [ ] T038 Verify FR-014: Missing fields omitted (not shown as "N/A")
- [ ] T038 Verify FR-015: Empty state when no rolling stock
- [ ] T038 Verify FR-016: 100% Paraglide-JS localization

**File Paths**: Specification checklist in IMPLEMENTATION_PLAN.md

### T039: Verify Success Criteria

- [ ] T039 SC-001: Page load < 1 second (use DevTools Performance tab)
- [ ] T039 SC-002: No layout shifts on load (check Cumulative Layout Shift)
- [ ] T039 SC-003: Card animations < 300ms (check DevTools)
- [ ] T039 SC-004: 100% Paraglide-JS coverage (search for hardcoded strings)
- [ ] T039 SC-005: Responsive design on 320px, 768px, 1920px
- [ ] T039 SC-006: Keyboard accessible (Tab, Enter, Arrow keys)

---

## Phase 12: Code Quality & Documentation

### T040: Verify Rust Code Quality

- [ ] T040 Run `pnpm rust:fmt` - all Rust formatted
- [ ] T040 Run `pnpm rust:clippy` - zero warnings
- [ ] T040 Run `pnpm rust:check` - compiles without errors
- [ ] T040 Run `pnpm rust:test` - all tests pass
- [ ] T040 Verify rustdoc comments on all public APIs
- [ ] T040 Ensure error handling comprehensive (no unwrap)

**File Paths**: `src-tauri/src/media/**`

### T041: Verify TypeScript/Frontend Code Quality

- [ ] T041 Run `pnpm check` - TypeScript check passes
- [ ] T041 Run `pnpm lint` - ESLint passes
- [ ] T041 Run `pnpm format` - Prettier formatting
- [ ] T041 Run `pnpm test` - all tests pass
- [ ] T041 Verify JSDoc comments on exported functions
- [ ] T041 No `any` types (use proper types)
- [ ] T041 No hardcoded strings (use Paraglide-JS)

**File Paths**: `src/routes/models/`, `src/lib/components/`

### T042: Add Comprehensive Comments

- [ ] T042 Add rustdoc (///) to all public Rust items
- [ ] T042 Add JSDoc (/\*\* \*/) to all exported TS/Svelte functions
- [ ] T042 Include examples in rustdoc where helpful
- [ ] T042 Document why (not just what) for complex logic
- [ ] T042 Include error scenarios and edge cases

**File Paths**: All implementation files

### T043: Update CHANGELOG

- [ ] T043 Add entry to CHANGELOG.md
- [ ] T043 Format: "feat: add railway model details page (feature 014)"
- [ ] T043 Include: Details tab, Rolling Stock tab, expandable cards
- [ ] T043 Include: Backend media module for image management
- [ ] T043 Follow existing format and conventions

**File Paths**: `CHANGELOG.md`

---

## Phase 13: Code Review & Merge

### T044: Prepare for Code Review

- [ ] T044 Ensure all tests passing locally
- [ ] T044 Ensure all CI checks pass (if applicable)
- [ ] T044 Create clear commit messages (Conventional Commits)
- [ ] T044 Squash related commits into logical units
- [ ] T044 Write pull request description with screenshots
- [ ] T044 Link to specification docs
- [ ] T044 Note any breaking changes (none expected)

**Commit Message Format**:

```
feat(014): add railway model details page with media module

Backend:
- Create media module with DDD architecture
- Implement image retrieval from filesystem
- Add HTML/CSS placeholder fallback
- Ensure path traversal security

Frontend:
- Add /models/[modelId] route with details page
- Implement header with image and badges
- Add tabs for Details and Rolling Stock
- Create expandable cards for rolling stock units
- Full Paraglide-JS localization (EN, IT)
- WCAG 2.1 AA accessibility compliance
```

### T045: Conduct Code Review

- [ ] T045 Architecture follows DDD (domain, application, infrastructure, interface)
- [ ] T045 Error handling is comprehensive
- [ ] T045 Security (path validation, no panics, proper error messages)
- [ ] T045 Performance targets met (page load < 1s, animations < 300ms)
- [ ] T045 Accessibility compliant (WCAG 2.1 AA)
- [ ] T045 All tests passing
- [ ] T045 No Clippy warnings
- [ ] T045 Documentation complete (rustdoc, JSDoc)
- [ ] T045 Localization 100% (no hardcoded strings)
- [ ] T045 Code style consistent with codebase

### T046: Address Review Feedback

- [ ] T046 Fix any issues identified in code review
- [ ] T046 Re-run all verification steps
- [ ] T046 Request re-review if significant changes
- [ ] T046 Ensure all feedback addressed

### T047: Merge to Main

- [ ] T047 Ensure all GitHub checks pass
- [ ] T047 Squash and merge to main branch
- [ ] T047 Delete feature branch
- [ ] T047 Verify main branch CI passes
- [ ] T047 Confirm TypeScript bindings updated in repo

---

## Summary

| Phase | Component                  | Tasks     | Status  |
| ----- | -------------------------- | --------- | ------- |
| 1     | Backend Setup              | T001      | Pending |
| 2     | Domain Layer               | T002-T005 | Pending |
| 3     | Application Layer          | T006-T008 | Pending |
| 4     | Infrastructure Layer       | T009-T011 | Pending |
| 5     | Interface & Integration    | T012-T016 | Pending |
| 6     | Frontend Routes & Layout   | T017-T020 | Pending |
| 7     | Frontend Components        | T021-T024 | Pending |
| 8     | Data Binding               | T025-T028 | Pending |
| 9     | Styling & Localization     | T029-T032 | Pending |
| 10    | Frontend Testing           | T033-T036 | Pending |
| 11    | Integration & Verification | T037-T039 | Pending |
| 12    | Code Quality               | T040-T043 | Pending |
| 13    | Code Review & Merge        | T044-T047 | Pending |

**Total Tasks**: 47  
**Estimated Duration**: ~2 weeks  
**Team Size**: 2 (1 backend, 1 frontend, shared QA/review)

---

## Task Execution Tips

1. **Start with backend Phase 1-5** (foundation for frontend image functionality)
2. **Frontend Phases 6-8 can start in parallel** once backend Phase 1 is done
3. **Testing (Phase 10) happens throughout**, not just at the end
4. **Code quality checks (Phase 12) happen per commit**, not as final pass
5. **Code review (Phase 13) happens before merge**, address all feedback
6. **Daily standups** to track blockers and dependencies

---

**Status**: Ready for Sprint Planning  
**Next**: Assign tasks to team members and begin Phase 1
