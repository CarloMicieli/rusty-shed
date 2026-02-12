# Implementation Plan: Railway Model Preview Card Component

**Branch**: `019-model-preview-card` | **Date**: 2026-02-12 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/019-model-preview-card/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

A reusable Svelte component that displays railway model information in a compact card format with thumbnail (16:9 aspect ratio), metadata badges (scale, power method, era, purchase date), status overlays (unit count, digital features), and a high-visibility identification plate for road numbers. The component handles missing data gracefully with category-specific placeholders and supports responsive mobile layouts.

## Technical Context

**Language/Version**: TypeScript 5.9.3, Svelte 5.48.2, SvelteKit (Vite 7.3.1)
**Primary Dependencies**: Tailwind CSS 4.1.18, shadcn-svelte (Card, Badge, Button), lucide-svelte (icons), Paraglide i18n
**Storage**: N/A (presentational component - no direct database interaction)
**Testing**: Vitest 4.0.18 with happy-dom environment
**Target Platform**: Desktop (Tauri 2.x) with responsive mobile support (Linux 6.17.0-14-generic)
**Project Type**: Web application (SvelteKit frontend + Tauri Rust backend)
**Performance Goals**: <16ms render time (60fps), smooth scrolling with 100+ cards in view
**Constraints**: Must maintain readability at standard viewing distances (50-70cm desktop, mobile standard), mobile responsive
**Scale/Scope**: Reusable component across multiple views (collection, wishlist, search results) with support for diverse model categories

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

### ✅ Code Quality (§ Core Principles)

- Component MUST use TypeScript strict mode
- MUST pass `pnpm lint` and `pnpm check` without errors
- MUST follow existing component patterns in `src/lib/components/`
- MUST remove unused imports

### ✅ User Experience Consistency (§ Core Principles)

- All user-facing strings MUST use Paraglide messaging system
- MUST use shadcn-svelte design tokens (Card, Badge, Button components)
- MUST follow existing card styling patterns from Dashboard (ring-1 ring-border/40 for stats cards)
- MUST maintain accessibility standards (semantic HTML, ARIA labels where needed)

### ✅ Testing Standards (§ Core Principles)

- MUST include unit tests for component rendering with Vitest
- MUST test edge cases: missing data, long road numbers, multiple digital features, missing photos
- Tests MUST be deterministic and fast (no external dependencies)
- SHOULD aim for 60%+ coverage on UI components per constitution guidance

### ✅ Performance Requirements (§ Core Principles)

- Component MUST render in <16ms (60fps target)
- MUST support smooth scrolling with 100+ cards without layout thrashing
- MUST lazy-load images to avoid memory bloat
- MUST avoid unnecessary re-renders (proper use of Svelte 5 runes)

### ⚠️ Domain Logic Location (§ Architectural Laws)

- **Compliance**: Component is purely presentational - no business logic
- **Verification**: All data formatting, validation, and business rules handled by parent components/backend

### N/A Database/Persistence (§ Architectural Laws)

- **Rationale**: Component receives data as props, does not interact with database directly

### N/A API Design & Transport Boundary (§ Architectural Laws)

- **Rationale**: Component does not make Tauri IPC calls directly - data provided by parent components
- **Note**: Parent components using this card will need to follow Tauri IPC + specta patterns

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src/
├── lib/
│   ├── components/
│   │   ├── ui/                          # shadcn-svelte components
│   │   └── RailwayModelPreviewCard.svelte  # New component (this feature)
│   ├── paraglide/                       # i18n messages
│   └── bindings.ts                      # Tauri type bindings (may need RailwayModel type)
└── __tests__/
    └── components/
        └── RailwayModelPreviewCard.test.ts  # Component tests (this feature)

messages/
└── en.json                              # i18n strings for component (this feature)

specs/019-model-preview-card/
├── spec.md                              # Feature specification
├── plan.md                              # This file
├── research.md                          # Phase 0 output
├── data-model.md                        # Phase 1 output
├── quickstart.md                        # Phase 1 output
└── contracts/                           # Phase 1 output (if needed)
```

**Structure Decision**: This is a frontend-only component in a Tauri desktop application. The component lives in `src/lib/components/` following the existing SvelteKit structure. No backend changes are required as this is a pure presentation component. Tests follow the existing pattern in `src/__tests__/components/`. i18n strings for labels will be added to the Paraglide messages system.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

**Status**: ✅ No constitution violations requiring justification. This is a standard presentational component following established patterns.

---

## Post-Design Constitution Re-Check

**Date**: 2026-02-12
**Status**: ✅ All gates passed

### Code Quality (§ Core Principles)

✅ **PASSED**

- Component structure follows Svelte 5 patterns ($props, $state, $derived)
- Uses TypeScript strict mode with proper type definitions
- Will pass `pnpm lint` and `pnpm check` (standard Svelte component)
- Follows existing component patterns (StatsCard, RecentItemCard, RollingStockCard)

### User Experience Consistency (§ Core Principles)

✅ **PASSED**

- All user-facing strings use Paraglide i18n (verified in quickstart.md)
- Uses shadcn-svelte components (Card, Badge, Button, AlertDialog)
- Matches existing card styling: `card gauge-frame ring-1 ring-border/40`
- Responsive grid pattern matches Dashboard: `grid-cols-1 sm:grid-cols-2 lg:grid-cols-3`
- Accessibility: semantic HTML, ARIA labels for delete button, keyboard navigation via AlertDialog

### Testing Standards (§ Core Principles)

✅ **PASSED**

- Test plan includes: rendering tests, user interaction tests, accessibility tests, edge case tests
- Follows RollingStockCard.test.ts pattern (Vitest + @testing-library/svelte)
- Mocking strategy documented (paraglide messages, event handlers)
- Target: 60%+ coverage for UI components (per constitution)

### Performance Requirements (§ Core Principles)

✅ **PASSED**

- Component designed for <16ms render (60fps)
- Uses native browser lazy loading for images
- SmartImage component pattern handles async image resolution efficiently
- Responsive grid with gap-4 spacing prevents layout thrashing
- No unnecessary re-renders (proper use of $derived for computed values)

### Domain Logic Location (§ Architectural Laws)

✅ **PASSED**

- Component is purely presentational
- All business logic (data mapping, deletion) handled by parent components
- No direct database or backend calls

### Database/Persistence (§ Architectural Laws)

✅ **N/A**

- Component does not interact with database directly
- Parent components handle data fetching and mutations

### API Design & Transport Boundary (§ Architectural Laws)

✅ **N/A** (with note)

- Component does not make Tauri IPC calls directly
- **Note**: Parent components using this card must follow Tauri IPC + specta patterns for data fetching and deletion commands
- Component accepts standard TypeScript props and emits standard events

### Verification Summary

**All constitution gates passed.** Component follows established patterns:

- ✅ Uses existing tech stack (no new dependencies)
- ✅ Matches existing component patterns (SmartImage, StatsCard, RollingStockCard)
- ✅ Follows responsive design conventions from Dashboard
- ✅ Uses Paraglide for all user-facing strings
- ✅ Includes comprehensive test plan
- ✅ Meets performance requirements

**No architectural violations or deviations from constitution.**

Ready to proceed to Phase 2 (Tasks generation via `/speckit.tasks` command).

---

## Phase 1 Completion Summary

**Date**: 2026-02-12
**Status**: ✅ Complete

### Generated Artifacts

1. **research.md** ✅
   - Existing research from 2026-02-11 updated with codebase pattern verification
   - Added addendum documenting actual patterns from StatsCard, Dashboard, SmartImage, RollingStockCard
   - Verified all technology choices align with constitution and existing codebase

2. **data-model.md** ✅
   - Component props interface documented (RailwayModelPreviewCardProps)
   - Existing data model from 2026-02-11 preserved
   - Documents mapping from backend types (CollectionItemView, WishlistItemView)
   - Test fixtures and edge cases defined

3. **quickstart.md** ✅ (NEW)
   - Quick start guide for developers
   - Usage examples with responsive grid layout
   - Edge case handling (long road numbers, missing data, multiple digital features)
   - Testing examples with Vitest
   - Common patterns (loading states, empty states)
   - Troubleshooting section

4. **contracts/** ⊗ (Not needed)
   - Pure UI component, no backend API contracts required
   - Data contracts documented in data-model.md

5. **Agent Context Update** ✅
   - CLAUDE.md updated with active technologies
   - Technology stack documented: TypeScript 5.9.3, Svelte 5.48.2, SvelteKit, Tailwind CSS 4.1.18, shadcn-svelte, lucide-svelte, Paraglide i18n

### Key Decisions Documented

1. **Component Architecture**: Presentational component following existing patterns
2. **Image Handling**: Reuse SmartImage component pattern or create similar approach
3. **Responsive Layout**: Standard Tailwind breakpoints (not container queries)
4. **Card Styling**: `card gauge-frame ring-1 ring-border/40` matching Dashboard
5. **Icons**: lucide-svelte with category mapping
6. **i18n**: Paraglide messaging system (constitutional requirement)
7. **Testing**: Vitest + @testing-library/svelte following RollingStockCard.test.ts pattern
8. **Type Safety**: Use backend types (CollectionItemView) with mapping layer

### Next Steps

Execute `/speckit.tasks` command to generate actionable tasks from this plan.
