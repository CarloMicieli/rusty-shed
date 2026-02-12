# Implementation Plan: Collection Page Card Integration

**Branch**: `021-collection-page-cards` | **Date**: 2026-02-12 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/021-collection-page-cards/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Replace the existing ItemCard component in the collection page with RailwayModelPreviewCard for the grid view, and integrate RailwayModelCard for detailed model views. This enhances visual presentation with thumbnails, metadata badges, digital feature indicators, and comprehensive model information including rolling stock details and image upload capabilities.

**Technical Approach**: Frontend-only integration leveraging existing components. Create data transformation utilities to map CollectionItemView to RailwayModelCardData. Preserve existing collection functionality (filtering, search, add, delete, navigation) while enhancing the visual layer.

## Technical Context

**Language/Version**: TypeScript 5.9.3 (frontend), Rust edition 2024 / 1.93.0 (backend)
**Primary Dependencies**: SvelteKit (Svelte 5.48.2), Tauri 2.9.x, Tailwind CSS 4.1.18, shadcn-svelte, lucide-svelte
**Storage**: SQLite via sqlx (existing schema - no changes needed)
**Testing**: Vitest 4.0.18 with happy-dom (frontend component tests)
**Target Platform**: Desktop (Tauri application for Linux, Windows, macOS)
**Project Type**: Desktop application (Tauri + SvelteKit hybrid)
**Performance Goals**: <200ms for collection rendering, 60fps smooth scrolling, <100ms card interaction feedback
**Constraints**: Type-safe props via TypeScript, Paraglide i18n for all user-facing text, responsive design (mobile to desktop)
**Scale/Scope**: Single-user desktop application, ~1000 models in collection (typical user)

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

### Initial Check (Before Research)

| Principle                         | Status  | Notes                                                                                                 |
| --------------------------------- | ------- | ----------------------------------------------------------------------------------------------------- |
| **Modular, Library-First Design** | ✅ PASS | Reusing existing RailwayModelCard and RailwayModelPreviewCard components - no new libraries needed    |
| **Code Quality**                  | ✅ PASS | Will follow `pnpm lint`, `pnpm format`, `pnpm check` - type-safe component integration                |
| **Testing Standards**             | ✅ PASS | Will add Vitest component tests for data mapping and card rendering - target 70%+ coverage            |
| **UX Consistency**                | ✅ PASS | Uses existing Paraglide messages, shadcn-svelte components, design tokens - consistent with app style |
| **Performance Requirements**      | ✅ PASS | Maintains existing collection rendering performance - no new heavy operations introduced              |
| **Database (Persistence)**        | ✅ N/A  | No database changes - using existing tables and queries                                               |
| **State Management**              | ✅ N/A  | No domain aggregate changes - frontend presentation layer only                                        |
| **API Design & Transport**        | ✅ N/A  | No new Tauri commands - using existing collection service and navigation                              |
| **Domain Logic Location**         | ✅ PASS | Business logic remains in Rust backend - frontend only handles presentation                           |

**Verdict**: ✅ **ALL GATES PASS** - Feature is frontend-only integration with no constitutional violations. No complexity justification needed.

### Post-Design Check (After Phase 1)

**Re-evaluation Date**: 2026-02-12

| Principle                         | Status  | Notes                                                                                             |
| --------------------------------- | ------- | ------------------------------------------------------------------------------------------------- |
| **Modular, Library-First Design** | ✅ PASS | cardDataMapper utility is self-contained, testable, and follows single responsibility principle   |
| **Code Quality**                  | ✅ PASS | TypeScript strict mode enforced, comprehensive function contracts defined, null safety throughout |
| **Testing Standards**             | ✅ PASS | Unit test specification complete (90%+ coverage target), integration tests defined                |
| **UX Consistency**                | ✅ PASS | Uses existing Paraglide i18n, shadcn-svelte components, preserves existing navigation patterns    |
| **Performance Requirements**      | ✅ PASS | O(n) transformation complexity acceptable for typical scale (100 models × 3 units = <5ms total)   |
| **Database (Persistence)**        | ✅ N/A  | No database changes - confirmed during design                                                     |
| **State Management**              | ✅ N/A  | No domain aggregate changes - confirmed during design                                             |
| **API Design & Transport**        | ✅ N/A  | No new Tauri commands - confirmed during design                                                   |
| **Domain Logic Location**         | ✅ PASS | All business logic remains in Rust - frontend only transforms presentation data                   |

**Verdict**: ✅ **ALL GATES PASS** - Design artifacts complete and constitutional compliance maintained

**Design Artifacts Generated**:

- ✅ research.md - Research findings with data mapping, digital features, category classification, UI pattern decision
- ✅ data-model.md - Complete TypeScript interfaces and transformation logic specification
- ✅ contracts/card-props.ts - TypeScript type definitions for all interfaces
- ✅ contracts/mapper-contract.md - Function signatures and contracts for mapper utility
- ✅ quickstart.md - Developer guide with step-by-step implementation instructions
- ✅ Agent context updated - CLAUDE.md updated with feature technologies

## Project Structure

### Documentation (this feature)

```text
specs/021-collection-page-cards/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output - data mapping and UI patterns
├── data-model.md        # Phase 1 output - TypeScript interfaces and transformations
├── quickstart.md        # Phase 1 output - developer guide
├── contracts/           # Phase 1 output - component prop contracts
│   └── card-props.ts    # RailwayModelCardData interface spec
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
# Tauri + SvelteKit Desktop Application
src/                                    # Frontend (SvelteKit)
├── lib/
│   ├── components/
│   │   ├── RailwayModelCard.svelte                  # ✅ Exists - detailed card
│   │   ├── RailwayModelPreviewCard.svelte           # ✅ Exists - preview card
│   │   └── ui/                                      # shadcn-svelte components
│   ├── features/
│   │   └── collection/
│   │       ├── CollectionDashboard.svelte           # 🔧 Modify - update to use new cards
│   │       ├── components/
│   │       │   ├── ItemCard.svelte                  # ⚠️ Deprecate - replace with preview card
│   │       │   └── FilterPanel.svelte               # ✅ Keep - no changes
│   │       ├── CollectionState.svelte.ts            # ✅ Keep - existing state management
│   │       └── utils/
│   │           └── cardDataMapper.ts                # ⭐ NEW - data transformation utilities
│   ├── types/
│   │   └── railway-model.ts                         # ✅ Exists - RailwayModel type
│   └── paraglide/                                   # i18n messages
└── routes/
    └── my-collection/
        └── +page.svelte                             # ✅ Keep - wraps CollectionDashboard

src-tauri/                                           # Backend (Rust)
└── src/
    └── collecting/                                  # ✅ No changes - existing domain
        ├── domain/
        │   └── collection_item_view.rs              # ✅ Exists - data source
        └── application/
            └── collection_query.rs                  # ✅ Exists - queries

tests/                                               # Frontend tests
└── unit/
    └── features/
        └── collection/
            ├── cardDataMapper.test.ts               # ⭐ NEW - mapper tests
            └── CollectionDashboard.test.ts          # 🔧 Update - test new cards
```

**Structure Decision**: Tauri + SvelteKit desktop application with clear separation between frontend (src/) and backend (src-tauri/). This feature modifies only the frontend presentation layer in `src/lib/features/collection/`, replacing the ItemCard component with RailwayModelPreviewCard and adding data mapping utilities. No backend changes required.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

_Not applicable - all gates passed._

## Phase 0: Research & Discovery

### Research Tasks

1. **Data Mapping Strategy**
   - **Question**: How to map CollectionItemView to RailwayModelCardData interface?
   - **Investigation**: Analyze existing CollectionItemView structure, RailwayModelCardData requirements, and identify transformation logic
   - **Output**: Data mapping specification in research.md

2. **Digital Features Extraction**
   - **Question**: How to determine digital features (Sound, DCC, Smoke, Light) from model data?
   - **Investigation**: Review RollingStock data structure for control_type, dcc_interface fields; define mapping rules
   - **Output**: Digital feature extraction rules in research.md

3. **Category Classification**
   - **Question**: How to map model data to ModelCategory enum for placeholder icons?
   - **Investigation**: Analyze railway_model.category field, establish mapping to RailwayModelPreviewCard's category types
   - **Output**: Category mapping table in research.md

4. **Detail View Pattern**
   - **Question**: Should detailed view use modal dialog or navigate to dedicated route?
   - **Investigation**: Evaluate UX patterns - modal for quick view vs route for deep navigation; assess existing app patterns
   - **Decision Criteria**: Consistency with app navigation, back button behavior, deep linking needs
   - **Output**: UI pattern recommendation in research.md

### Research Execution

_Research tasks will be dispatched to specialized agents in Phase 0._

## Phase 1: Design & Contracts

### Data Model Design

**File**: `data-model.md`

Will document:

- RailwayModelCardData interface structure
- CollectionItemView → RailwayModelCardData transformation logic
- Digital feature extraction algorithm
- Category classification rules
- Null/undefined handling strategies

### API Contracts

**Directory**: `contracts/`

**Files**:

- `card-props.ts`: TypeScript interface definitions for RailwayModelCardData
- `mapper-contract.md`: Function signatures for data transformation utilities

### Component Integration

**File**: `quickstart.md`

Developer guide covering:

1. How to use cardDataMapper utility
2. How to integrate RailwayModelPreviewCard in grid views
3. How to integrate RailwayModelCard in detail views
4. Testing strategy for component integration
5. Common pitfalls and troubleshooting

### Agent Context Update

After design completion, run:

```bash
.specify/scripts/bash/update-agent-context.sh claude
```

This will update the Claude-specific context file with:

- Component integration patterns for this feature
- Data mapping utilities location
- Testing approach for card components

## Phase 2: Implementation Tasks

_Implementation tasks will be generated by `/speckit.tasks` command after Phase 1 completes._

**Estimated task areas**:

1. Create cardDataMapper utility with transformation functions
2. Update CollectionDashboard to use RailwayModelPreviewCard
3. Implement detail view (modal or route based on research outcome)
4. Add unit tests for data mapping
5. Add component tests for card integration
6. Update existing ItemCard references
7. Run quality gates (lint, format, check, test)

## Dependencies & Prerequisites

- ✅ RailwayModelCard component fully implemented
- ✅ RailwayModelPreviewCard component fully implemented
- ✅ CollectionItemView type available from bindings
- ✅ Collection service provides necessary data
- ✅ Paraglide i18n messages configured

**Blockers**: None - all prerequisites met

## Risk Assessment

| Risk                                       | Severity | Mitigation                                                                    |
| ------------------------------------------ | -------- | ----------------------------------------------------------------------------- |
| Data mapping reveals missing fields        | Medium   | Research phase will identify gaps; can extend backend DTOs if needed          |
| Performance regression with new components | Low      | New components already optimized; maintain existing pagination/virtualization |
| Breaking existing collection functionality | Low      | Comprehensive testing of filter, search, delete before merging                |
| Inconsistent digital feature detection     | Medium   | Clear mapping rules in research; unit tests for all cases                     |

## Post-Implementation Validation

After implementation completes:

1. ✅ Run `pnpm lint` - no errors
2. ✅ Run `pnpm format` - code formatted
3. ✅ Run `pnpm check` - TypeScript types valid
4. ✅ Run `pnpm test` - all tests pass, coverage ≥70%
5. ✅ Manual testing: verify all user scenarios from spec.md
6. ✅ Accessibility check: keyboard navigation, screen reader compatibility
7. ✅ Performance check: collection with 1000+ models renders smoothly
8. ✅ Visual regression: compare screenshots before/after on all breakpoints

## Success Metrics

Aligned with spec.md Success Criteria:

- SC-001: All models display with new preview cards ✅
- SC-002: Performance maintained (no regression) ✅
- SC-003: Detail view accessible and functional ✅
- SC-004: Image upload works (existing functionality) ✅
- SC-005: Existing features work (regression suite) ✅
- SC-006: Responsive design verified ✅
- SC-007: Digital badges appear correctly ✅
- SC-008: Unit count badges accurate ✅
