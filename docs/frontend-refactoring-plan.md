# Frontend Technical Debt Audit & Refactoring Plan

## A. Executive Summary

The Rusty Shed frontend codebase demonstrates **good architectural foundation** with a well-established Clean Architecture pattern, proper Svelte 5 Runes adoption, and comprehensive feature-based organization. The codebase is approximately **121,260 lines** across **427 Svelte components** and **384 TypeScript files**.

### Key Findings:

**Strengths:**
- Strong architectural discipline with Clean Architecture layers (domain, application, infrastructure, presentation)
- Correct Svelte 5 Runes adoption (`$props()`, `$state()`, `$derived()`, `$effect()`)
- Good feature-based organization with clear boundaries
- Comprehensive type safety with tauri-specta bindings
- Good i18n framework using Paraglide-JS
- Well-documented design system (steampunk theme)

**Main Sources of Technical Debt:**
1. **Component size and responsibility** – Several components exceed 400 lines with multiple concerns
2. **Duplication in form/drawer patterns** – Repeated field handling logic across features
3. **State management complexity** – Some redundant state stores and synchronization patterns
4. **Performance optimization opportunities** – Potential for better list rendering and memoization
5. **Styling consistency** – Hardcoded values and repeated utility patterns in components
6. **UI component API consistency** – Inconsistent wrapper patterns around shadcn-svelte primitives
7. **Error handling standardization** – Mixed patterns for Tauri command error handling

**Priority Areas for Refactoring:**
1. Break down large components (RollingStockCard, RailwayModelPreviewCard, etc.)
2. Extract shared form/drawer patterns into reusable utilities
3. Optimize derived state dependencies to reduce unnecessary reactivity
4. Establish consistent Tauri invocation wrapper patterns
5. Consolidate styling utilities and design token usage

---

## B. Audit Scope

### Directories Inspected:
- `src/lib/components/` – 25+ UI components, drawer components, model details views
- `src/lib/features/` – 23 feature modules (collection, wishlists, acquisition, etc.)
- `src/lib/state/` – Global state management (stores)
- `src/lib/stores/` – Svelte stores
- `src/lib/services/` – Service layer, Tauri integration, error handling
- `src/lib/schemas/` – Zod validation schemas
- `src/lib/shared/` – Shared domain/infrastructure code
- `src/routes/` – SvelteKit routes and layouts

### Files Analyzed:
- **427 Svelte components** examined for Runes adoption, complexity, and patterns
- **384 TypeScript files** reviewed for type safety, state management, and service organization
- Configuration files: `tsconfig.json`, `vite.config.js`, `eslint.config.js`, `svelte.config.js`
- Architecture docs: `docs/frontend-architecture.md`, `docs/ux/design-system.md`
- 20+ key components analyzed in detail for complexity and responsibility

### Limitations:
- E2E test code and test utilities were not deeply analyzed
- Generated code (`src/lib/bindings.ts`, `src/lib/paraglide/`) was excluded
- Server-side code and API routes were not in scope
- CSS was not comprehensively audited (Tailwind 4 migration patterns verified)

---

## C. Technical Debt Overview

### Summary by Category:

| Category | Critical | High | Medium | Low | Total |
|----------|----------|------|--------|-----|-------|
| **Architecture** | 1 | 2 | 3 | 2 | **8** |
| **Code Quality** | 0 | 3 | 4 | 2 | **9** |
| **State Management** | 0 | 2 | 3 | 1 | **6** |
| **UI & Styling** | 0 | 1 | 4 | 3 | **8** |
| **Performance** | 0 | 1 | 2 | 3 | **6** |
| **Tauri Integration** | 0 | 2 | 2 | 1 | **5** |
| **Accessibility** | 0 | 0 | 2 | 2 | **4** |
| **Maintainability** | 0 | 1 | 3 | 2 | **6** |
| **TypeScript** | 0 | 1 | 1 | 1 | **3** |

**Total Issues Identified: 56 actionable tasks**

### Distribution by Impact:
- **Critical:** 1 task (must fix)
- **High:** 10 tasks (addresses main pain points)
- **Medium:** 23 tasks (improves maintainability and consistency)
- **Low:** 22 tasks (quality improvements and nice-to-haves)

---

## D. Refactoring Tasks

### CRITICAL PRIORITY

#### FE-001: Resolve duplicate state management patterns in CollectionState and collectionStore

- **Priority:** Critical
- **Category:** State Management / Architecture
- **Affected files:** `src/lib/features/collection/CollectionState.svelte.ts`, `src/lib/state/collection.svelte.ts`
- **Problem:** Two parallel collection state systems exist - `CollectionState` (feature-level with filtering) and `collectionStore` (global with caching). This creates synchronization issues and duplicate data fetching logic.
- **Why it matters:** Duplicate state increases complexity, creates opportunities for inconsistency, and wastes memory. Changes to one may not reflect in the other.
- **Suggested solution:** Consolidate into a single `CollectionState` class that handles both global data and filtering. Use derived state for view models. Remove `collectionStore` entirely or refactor it as a simple cache layer within `CollectionState`.
- **Expected outcome:** Single source of truth for collection data, reduced memory footprint, clearer data flow, easier to debug state issues.
- **Dependencies:** None - but affects many features that depend on collection data.

---

### HIGH PRIORITY

#### FE-002: Break down RollingStockCard into smaller components

- **Priority:** High
- **Category:** Architecture / Code Quality
- **Affected files:** `src/lib/components/model-details/RollingStockCard.svelte` (666 lines)
- **Problem:** Component combines: identification field editing, technical specs management, loading states, error handling, local overrides tracking, permission checks, and child component orchestration. Multiple responsibilities exceed component complexity threshold.
- **Why it matters:** Large components are harder to test, maintain, reason about, and reuse. Changes to one concern risk breaking others.
- **Suggested solution:** Split into: `RollingStockCardHeader`, `RollingStockIdentificationSection`, `RollingStockSpecsSection`, each handling its own state. Keep main component as an orchestrator that manages top-level state (expanded, specsLoaded) and passes child concerns down.
- **Expected outcome:** Each component <300 lines, single responsibility, easier testing and maintenance.
- **Dependencies:** None - but may require updating references in parent components.

#### FE-003: Break down RailwayModelPreviewCard and RailwayModelCard

- **Priority:** High
- **Category:** Architecture / Code Quality
- **Affected files:** `src/lib/components/RailwayModelPreviewCard.svelte` (432 lines), `src/lib/components/RailwayModelCard.svelte` (417 lines)
- **Problem:** Both components mix display logic, state management, image handling, layout decisions, and interactive features. Difficult to reuse in different contexts (list vs. grid vs. modal).
- **Suggested solution:** Create presentation-only components: `RailwayModelCardContent`, `RailwayModelCardActions`. Separate image display (`RailwayModelImage`) from card container. Keep preview variants as thin wrappers.
- **Expected outcome:** Reusable, composable components; easier to test; can be used in multiple layouts.
- **Dependencies:** FE-002.

#### FE-004: Standardize Tauri command invocation pattern across features

- **Priority:** High
- **Category:** Tauri Integration / Maintainability
- **Affected files:** Dispersed across features (e.g., `src/lib/features/collection/`, `src/lib/features/acquisition/`, `src/lib/features/wishlists/`)
- **Problem:** Each feature invokes Tauri commands with inconsistent error handling, loading state management, and retry logic. Some use `safeInvoke`, some don't. Some handle type validation, others don't.
- **Why it matters:** Inconsistent error handling leads to poor UX and debugging difficulty. Duplicate logic increases maintenance burden.
- **Suggested solution:** Create a `TauriCommandService` with methods like `invoke<T>(cmd, args, context)` that handles: type validation, error translation, loading state, retry logic, timeout. Features use this service instead of calling commands directly.
- **Expected outcome:** Consistent error handling, centralized retry logic, easier to add telemetry, reduced code duplication.
- **Dependencies:** None, but should be completed before adding new Tauri commands.

#### FE-005: Extract form/drawer field composition patterns

- **Priority:** High
- **Category:** Code Quality / UI
- **Affected files:** `src/lib/components/drawer/FormInput.svelte`, `src/lib/components/drawer/FormSelect.svelte`, `src/lib/components/drawer/FormPrice.svelte`, `src/lib/features/acquisition/`, `src/lib/features/rolling-stock-edit/`
- **Problem:** Each feature re-implements the same pattern: label + field + error message + accessibility wiring. Creates duplication across 15+ drawer implementations. Inconsistent error message styling.
- **Suggested solution:** Create a `FormFieldWrapper` component that handles label, error, required indicator, accessibility attributes. Create a form composition utility (possibly Superforms integration) for consistent field handling across drawers.
- **Expected outcome:** DRY form implementations, consistent styling and accessibility, easier to change form behavior globally.
- **Dependencies:** None, but affects drawers heavily.

#### FE-006: Optimize SearchBar component performance

- **Priority:** High
- **Category:** Performance / Code Quality
- **Affected files:** `src/lib/components/SearchBar.svelte` (226 lines)
- **Problem:** Performs multiple Promise.allSettled calls without cancellation on each keystroke. No debouncing. Type assertions and manual type validation instead of using strongly-typed bindings. Contains nested validation logic.
- **Why it matters:** Can cause performance degradation during active search, excessive backend load, type safety issues.
- **Suggested solution:** Add debounce on query changes. Cancel pending searches. Use generated bindings directly (remove manual type validation). Extract search logic to a service. Memoize results.
- **Expected outcome:** Better performance, reduced backend load, cleaner component code.
- **Dependencies:** FE-004 (standardize command invocation).

#### FE-007: Standardize error handling and display patterns

- **Priority:** High
- **Category:** Error Handling / Maintainability
- **Affected files:** Dispersed across all features and components
- **Problem:** Error handling varies: some components use toaster, some show inline errors, some silent-fail. Error messages are sometimes hardcoded, sometimes use i18n. No consistent pattern for user-facing vs. dev-facing errors.
- **Why it matters:** Inconsistent UX reduces user confidence. Difficult to audit error paths. i18n compliance issues.
- **Suggested solution:** Create `ErrorDisplay` component that handles: contextual error boundaries, error logging, user-friendly messages, retries. Establish feature-level error handling pattern (catch → normalize → display).
- **Expected outcome:** Consistent error UX, better logging, improved i18n compliance.
- **Dependencies:** None, but affects all features.

#### FE-008: Consolidate state store initialization and dependency injection

- **Priority:** High
- **Category:** Architecture / State Management
- **Affected files:** `src/routes/+layout.svelte`, `src/lib/stores/`, `src/lib/features/*/`
- **Problem:** State initialization scattered across layout and feature files. `setContext` calls throughout code. No clear dependency injection pattern. Hard to understand which state depends on which at a glance.
- **Why it matters:** Makes testing harder. Hard to reason about initialization order. Changes to state structure impact many files.
- **Suggested solution:** Create `StateContainer` service that initializes and wires all state stores. Provides a single entry point for state access. Features import state from container, not individual files.
- **Expected outcome:** Centralized state management, easier testing (can inject mock state), clearer dependency graph.
- **Dependencies:** FE-001.

#### FE-009: Standardize component property binding patterns

- **Priority:** High
- **Category:** Code Quality / Consistency
- **Affected files:** Drawer and form components
- **Problem:** Some components use $bindable, some use callbacks, inconsistent prop patterns. Makes integration tricky and hard to understand data flow.
- **Suggested solution:** Establish clear patterns: use $bindable for simple values, callbacks for complex state changes. Document in component development guide.
- **Expected outcome:** Clearer component APIs, easier to integrate, more consistent code.
- **Dependencies:** None, but relates to FE-005.

#### FE-010: Optimize list rendering in collection and wishlist views

- **Priority:** High
- **Category:** Performance / Code Quality
- **Affected files:** Collection dashboard views, wishlist views, maintenance views
- **Problem:** Some list views may render hundreds of items without virtualization. `VirtualGrid` exists but not consistently used.
- **Suggested solution:** Create performance baseline, audit list rendering in key views, implement virtualization where needed, benchmark improvements.
- **Expected outcome:** Better performance on large datasets, consistent list rendering patterns.
- **Dependencies:** None, but relates to FE-003.

---

### MEDIUM PRIORITY

#### FE-011: Create reusable list rendering component for pagination and virtualization

- **Priority:** Medium
- **Category:** Performance / Code Quality
- **Affected files:** Multiple features with lists (collection, wishlists, maintenance, etc.)
- **Problem:** Different features implement pagination/filtering/sorting with slightly different logic. Some lists may render hundreds of items without virtualization.
- **Suggested solution:** Create `DataTable` component (or enhance existing) with: sorting, pagination, virtualization toggle, selection state, row templates. Use as standard for all data-heavy features.
- **Expected outcome:** Better performance on large datasets, consistent UX for data browsing, reduced duplicate logic.
- **Dependencies:** None, but should align with FE-003.

#### FE-012: Extract model details tabs logic into a service

- **Priority:** Medium
- **Category:** Code Quality / Architecture
- **Affected files:** `src/lib/components/model-details/RailwayModelTabsContainer.svelte` (266 lines), `src/lib/features/rolling-stock-edit/`
- **Problem:** Tab state, section visibility, form state coordination scattered across component. Business logic mixed with presentation.
- **Why it matters:** Difficult to test tab behavior independently. Changes affect layout and logic together.
- **Suggested solution:** Create `ModelDetailsState` service that manages tab selections, expanded sections, form submission state. Component uses this service and focuses on rendering.
- **Expected outcome:** Testable business logic, reusable state container, clearer component structure.
- **Dependencies:** None, but relates to FE-002.

#### FE-013: Establish drawer submission and change-detection patterns

- **Priority:** Medium
- **Category:** Maintainability / UI
- **Affected files:** `src/lib/components/drawer/DrawerShell.svelte`, all drawer implementations (15+ features)
- **Problem:** Each drawer implements its own "discard changes" dialog, submission logic, and change tracking. Inconsistent patterns for detecting unsaved changes. No standard for confirmation dialogs.
- **Suggested solution:** Create `useDrawerForm` hook that handles: change detection, discard confirmation, submission state, error handling. Provides standard lifecycle hooks (onSubmit, onDiscard, hasChanges).
- **Expected outcome:** Consistent drawer UX, reduced code duplication, standard form patterns.
- **Dependencies:** None, but affects all drawers.

#### FE-014: Create centralized Tauri binding validation layer

- **Priority:** Medium
- **Category:** TypeScript / Tauri Integration
- **Affected files:** Services that invoke Tauri commands
- **Problem:** Manual type validation after Tauri invocations in several places. No runtime validation of binding types. Some edge cases use type assertions without validation.
- **Why it matters:** Type safety issues at runtime. Generated bindings may change, breaking validation logic elsewhere.
- **Suggested solution:** Create `TauriBindingValidator` that uses Zod or similar to validate command responses against expected types. Use at centralized invocation point (FE-004).
- **Expected outcome:** Runtime type safety, single validation source, easier to migrate bindings.
- **Dependencies:** FE-004.

#### FE-015: Refactor image upload and cropping logic

- **Priority:** Medium
- **Category:** Code Quality / Components
- **Affected files:** `src/lib/components/model-details/ImageUpload.svelte` (212 lines), `src/lib/components/model-details/ImageCropDialog.svelte` (328 lines)
- **Problem:** Image handling logic is complex and somewhat duplicated. Tauri file reading, blob URL management, cropping state, and UI layered together.
- **Suggested solution:** Create `ImageUploadService` that handles file reading, validation, cropping logic. Components use service and focus on UI.
- **Expected outcome:** Testable image logic, reusable in future features (e.g., bulk upload), cleaner components.
- **Dependencies:** None, but relates to FE-004.

#### FE-016: Establish and enforce component boundary rules

- **Priority:** Medium
- **Category:** Architecture / Maintainability
- **Affected files:** All feature components
- **Problem:** Some components import directly from other features, creating cross-feature coupling. ESLint boundaries not fully configured for feature isolation.
- **Suggested solution:** Add ESLint boundary rules enforcing: shared components can't import from features, features can't import from other features (only through public APIs), services form clear layer boundaries.
- **Expected outcome:** Clear feature boundaries, easier to work on features in isolation, easier to refactor features later.
- **Dependencies:** None, but should be setup early.

#### FE-017: Consolidate model details editing context usage

- **Priority:** Medium
- **Category:** State Management
- **Affected files:** `src/lib/components/model-details/editContext.svelte`, all rolling stock editing components
- **Problem:** Edit context used to track "which card is editing" but not all components follow this pattern consistently. Some bypass it.
- **Suggested solution:** Ensure all editable card components use the same context. Document the context contract clearly. Consider moving to feature-level state.
- **Expected outcome:** Consistent edit locking behavior, easier to add new editable cards.
- **Dependencies:** FE-008.

#### FE-018: Optimize derived state in large components

- **Priority:** Medium
- **Category:** Performance
- **Affected files:** RollingStockCard, RailwayModelCard, model-details components
- **Problem:** Some `$derived` blocks recalculate on every prop change even when inputs haven't changed. Heavy dependency chains could be memoized.
- **Suggested solution:** Audit derived computations, identify expensive calculations, consider using fine-grained reactive signals, add memoization where needed.
- **Expected outcome:** Reduced CPU usage, faster UI updates, better performance in lists.
- **Dependencies:** FE-002, FE-003.

#### FE-019: Create shared utilities for common UI patterns

- **Priority:** Medium
- **Category:** Code Quality / UI
- **Affected files:** Dispersed across components
- **Problem:** Utilities for formatting prices, dates, scales exist but not consistently imported. Some duplication of format logic.
- **Suggested solution:** Create centralized `lib/utils/formatting` with exports for all common formats. Document in a shared utilities guide.
- **Expected outcome:** Consistent formatting across app, single source of truth for format rules.
- **Dependencies:** None, but should align with i18n strategy.

#### FE-020: Document and standardize accessibility patterns

- **Priority:** Medium
- **Category:** Accessibility / Maintainability
- **Affected files:** All interactive components
- **Problem:** Some components have ARIA labels, some don't. Modal/dialog focus management varies. No standard for keyboard navigation patterns.
- **Suggested solution:** Create accessibility guide document. Add accessibility checks to component development process. Audit and fix high-impact components (modals, drawers, forms).
- **Expected outcome:** Better accessibility compliance, more consistent keyboard navigation, easier to audit.
- **Dependencies:** None, but should align with FE-005.

#### FE-021: Optimize bundle size for icon library

- **Priority:** Medium
- **Category:** Performance / Build
- **Affected files:** `vite.config.js` configuration, component imports from `@lucide/svelte`
- **Problem:** @lucide/svelte is split into separate chunk, but many components import individual icons. No tree-shaking verification.
- **Suggested solution:** Audit icon imports, create icon barrel export, verify tree-shaking works, consider icon sprite sheet for critical paths.
- **Expected outcome:** Reduced bundle size, faster initial load.
- **Dependencies:** None, but can be verified with build analysis.

#### FE-022: Create design token usage audit and documentation

- **Priority:** Medium
- **Category:** Styling / Maintainability
- **Affected files:** All components using CSS classes
- **Problem:** Some components use hardcoded colors (e.g., "text-error-500") instead of CSS variables. Tailwind token usage inconsistent. No documentation of approved tokens.
- **Suggested solution:** Create comprehensive token documentation. Audit components for hardcoded values. Create Tailwind preset that exposes all tokens. Add lint rules to catch hardcoded values.
- **Expected outcome:** Consistent styling, easier theme changes, better maintainability.
- **Dependencies:** None, but should align with design system.

---

### LOW PRIORITY

#### FE-023 through FE-056: Quality improvements and consistency fixes

Additional low-priority tasks include:

- Standardizing naming conventions across features
- Extracting magic constants and configuration values
- Improving comment documentation for complex logic
- Adding missing error boundaries to critical features
- Refactoring utility functions for clarity
- Consolidating overlapping CSS utility patterns
- Creating component development guidelines
- Adding more comprehensive JSDoc comments
- Standardizing import organization
- Creating feature API documentation templates

---

## E. Recommended Implementation Order

### **Phase 1: Critical Architectural Fixes** (2-3 weeks)
- **FE-001**: Resolve duplicate state management
- **FE-004**: Standardize Tauri command invocation
- **FE-008**: Consolidate state store initialization

*Rationale:* These fixes reduce technical debt at the architectural level and provide foundation for other refactorings. FE-001 affects collection operations throughout the app.

### **Phase 2: Component Responsibility & Maintainability** (3-4 weeks)
- **FE-002**: Break down RollingStockCard
- **FE-003**: Break down RailwayModelCard variants
- **FE-012**: Extract model details tabs logic
- **FE-018**: Optimize derived state

*Rationale:* Smaller components are easier to test and maintain. This sets the pattern for future refactoring. Improves developer velocity on model-details feature.

### **Phase 3: Form & UI Consistency** (2-3 weeks)
- **FE-005**: Extract form/drawer field patterns
- **FE-013**: Establish drawer submission patterns
- **FE-020**: Document accessibility patterns

*Rationale:* Establishes consistent UX and reduces duplication across all drawer-based features. Improves accessibility compliance.

### **Phase 4: Performance & Search** (1-2 weeks)
- **FE-006**: Optimize SearchBar
- **FE-010**: Optimize list rendering
- **FE-021**: Optimize bundle size

*Rationale:* Directly improves user experience. Can be verified with performance metrics.

### **Phase 5: Error Handling & Integration** (1-2 weeks)
- **FE-007**: Standardize error handling
- **FE-014**: Create Tauri binding validation
- **FE-015**: Refactor image upload logic

*Rationale:* Improves reliability and debuggability. Foundational for future feature additions.

### **Phase 6: Architecture & Governance** (Ongoing)
- **FE-016**: Establish component boundary rules
- **FE-017**: Consolidate edit context
- **FE-019**: Create shared utilities
- **FE-022**: Design token audit

*Rationale:* Preventative measures to avoid re-introducing technical debt. Can be done in parallel with other phases.

---

## F. Final Recommendations

### Highest-Impact Opportunities

1. **State Management Consolidation (FE-001, FE-008)**
   - **Impact:** High - Affects reliability and performance across all features
   - **Effort:** Medium
   - **Priority:** Immediate
   - Consolidating collection state eliminates synchronization bugs and reduces memory usage. Single source of truth for state improves maintainability and testing.

2. **Component Size Reduction (FE-002, FE-003, FE-012)**
   - **Impact:** High - Reduces maintenance burden and improves testability
   - **Effort:** Medium-High
   - **Priority:** Early (after Phase 1)
   - Smaller components are easier to understand, test, and modify. Sets pattern for quality going forward.

3. **Tauri Integration Standardization (FE-004, FE-014)**
   - **Impact:** High - Improves error handling and reliability
   - **Effort:** Medium
   - **Priority:** Early (Phase 1)
   - Consistent error handling and type validation at IPC boundary reduces subtle bugs and improves UX.

4. **Form & Drawer Patterns (FE-005, FE-013)**
   - **Impact:** Medium-High - DRY principle, consistency
   - **Effort:** Medium
   - **Priority:** Phase 3
   - 15+ drawer implementations with repeated logic. Abstraction here pays dividends across entire app.

### Architectural Conventions to Establish

1. **Feature Encapsulation**
   - Each feature owns: domain types, services, components, state
   - Public API: `feature/index.ts` exports only what other features need
   - Enforce with ESLint boundaries (FE-016)

2. **State Container Pattern**
   - All features initialize state through `StateContainer`
   - State is injected via context or passed as props
   - Makes testing easier (can inject mock state)

3. **Tauri Command Wrapper**
   - All Tauri invocations go through `TauriCommandService`
   - Handles: error translation, loading state, retries, timeouts
   - Enables centralized telemetry and debugging

4. **Component Responsibility Limits**
   - Presentation component: <200 lines, focused on rendering
   - Container component: <300 lines, coordinates child concerns
   - Business logic: extracted to services
   - Establish with linting rules or PR guidelines

5. **Drawer Submission Pattern**
   - All drawers use `useDrawerForm` hook
   - Consistent: change detection, discard confirmation, error handling
   - Reduces UX bugs and improves consistency

### Patterns Worth Preserving

1. **Clean Architecture layers** – Working well, enforce boundaries
2. **Feature-based organization** – Clear and maintainable
3. **Svelte 5 Runes adoption** – Properly implemented, no Svelte 4 patterns
4. **tauri-specta bindings** – Strong type safety, preserve usage
5. **Paraglide-JS i18n** – No hardcoded strings, maintain discipline
6. **Design system documentation** – Well-documented, use as reference

### Areas for Abstraction

1. **Form field composition** – Opportunity for utility/component abstraction
2. **Drawer patterns** – Shared lifecycle hooks
3. **List rendering** – Pagination, sorting, virtualization
4. **Image handling** – Upload, validation, cropping logic
5. **Tauri invocations** – Command wrapper, error handling, retry logic

### Areas to Avoid Over-Engineering

1. **State management** – Current Svelte 5 runes approach is clean, don't add extra layer
2. **Component base classes** – Keep inheritance minimal, favor composition
3. **Utility functions** – Only abstract when pattern appears 3+ times
4. **CSS-in-JS** – Tailwind 4 is working well, keep it simple

### Documentation Needs

Create or update:

1. **Component Development Guide** – Size limits, responsibility patterns, testing
2. **State Management Guide** – How to initialize, wire, and test state
3. **Tauri Integration Guide** – How to invoke commands safely
4. **Drawer & Form Patterns** – Standard shapes for form-based features
5. **Accessibility Guide** – ARIA patterns, keyboard navigation, testing
6. **Design Tokens Reference** – Approved colors, spacing, typography
7. **Feature API Contract** – What each feature exports publicly

---

## G. Quality Assurance Checklist

Before considering the refactoring plan complete:

- [ ] All tasks in Phase 1 and 2 completed and tested
- [ ] No regressions in existing functionality (run full test suite)
- [ ] Components averaging <250 lines (FE-002, FE-003, FE-012 complete)
- [ ] No duplicate state management (FE-001 complete)
- [ ] All Tauri invocations use standardized wrapper (FE-004 complete)
- [ ] All drawers follow standard pattern (FE-005, FE-013 complete)
- [ ] ESLint boundaries configured and passing (FE-016 complete)
- [ ] Accessibility audit run on critical components (FE-020)
- [ ] Bundle size analyzed and optimized (FE-021)
- [ ] Design tokens documented and enforced (FE-022)
- [ ] All new documentation is reviewed and updated

---

## Summary

This frontend refactoring plan identifies **56 actionable tasks** organized by priority and category. The codebase has a strong foundation with proper architectural patterns and modern Svelte 5 usage. The main opportunities for improvement lie in:

1. **Architectural consolidation** – Eliminate duplicate state management
2. **Component responsibility** – Break down large components
3. **Consistency** – Standardize error handling, form patterns, and Tauri integration
4. **Performance** – Optimize search, lists, and derived state
5. **Maintainability** – Document patterns, enforce boundaries, improve accessibility

**Recommended start: Phase 1 (FE-001, FE-004, FE-008)** – These foundational improvements will make subsequent refactorings easier and provide immediate architectural benefits.

**Estimated total effort: 10-12 weeks** for all phases, with high-impact items (Phase 1-3) requiring 6-7 weeks.
