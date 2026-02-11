# Implementation Plan: Reusable Railway Model Component

**Branch**: `018-railway-model-component` | **Date**: 2026-02-11 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/018-railway-model-component/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Build a reusable Svelte 5 component that displays comprehensive railway model information including product-level details (manufacturer, product code, scale), global specifications (era, power method, category), and individual rolling stock specifications. The component supports both single-unit models (direct display) and multi-unit sets (tabbed interface with expandable rolling stock list). Image upload via file browser or drag-and-drop is included. The component must be responsive, maintainable, and reusable across collection and wishlist contexts.

## Technical Context

**Language/Version**: TypeScript 5.9.3 (frontend), Rust edition 2024 / 1.93.0 (backend)
**Primary Dependencies**: SvelteKit (Svelte 5.48.2), Tauri 2.9.x, Tailwind CSS 4.1.18, shadcn-svelte, specta, sqlx
**Storage**: SQLite via sqlx (existing railway_model and rolling_stock tables)
**Testing**: Vitest 4.0.18 with happy-dom (frontend), cargo test (backend)
**Target Platform**: Desktop (Linux, Windows, macOS) via Tauri
**Project Type**: Desktop application (Tauri + SvelteKit)
**Performance Goals**: Component render <500ms for 20 rolling stock units, tab switching <100ms, image upload feedback <10s
**Constraints**: Responsive 320px-1920px+, <200ms for backend read queries, must follow Paraglide i18n, no hardcoded strings
**Scale/Scope**: Single reusable component, supports 1-20 rolling stock units per model, reusable across multiple pages

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

### Pre-Research Gates (Phase 0)

| Principle                         | Status         | Assessment                                                                                       |
| --------------------------------- | -------------- | ------------------------------------------------------------------------------------------------ |
| **Modular, Library-First Design** | ✅ PASS        | Component is designed as a reusable module with clear props interface, independently testable    |
| **Deterministic Interfaces**      | ✅ PASS        | Component props explicitly typed via TypeScript, Tauri commands will be documented with specta   |
| **Test-First Emphasis**           | ⚠️ CONDITIONAL | Must include component tests (Vitest) and backend command tests (cargo test) before merge        |
| **Code Quality**                  | ✅ PASS        | Will follow pnpm lint, pnpm check, cargo clippy, cargo fmt per workflow                          |
| **Testing Standards**             | ⚠️ CONDITIONAL | Target coverage: component logic 70%+, backend commands 80%+, integration tests for image upload |
| **UX Consistency**                | ✅ PASS        | Uses Paraglide for all strings, shadcn-svelte components, follows existing card/layout patterns  |
| **Performance Requirements**      | ✅ PASS        | Success criteria SC-006 and SC-007 define render and interaction performance targets             |
| **Database Law**                  | ✅ PASS        | Uses existing SQLite schema (railway_model, rolling_stock tables), no new migrations needed      |
| **State Management Law**          | N/A            | Component is read-only display; no aggregate mutations, no domain events                         |
| **API Design Law**                | ⚠️ CONDITIONAL | Image upload command must follow Args→Input→UseCase pattern with validation, specta types        |
| **Domain Logic Location**         | ⚠️ CONDITIONAL | Image validation (file type, size) must be in Rust backend, not just frontend                    |

**GATE RESULT**: CONDITIONAL PASS - proceed to Phase 0 research with the following requirements:

- Define Tauri command for image upload following ADR 8 conventions (Args, Input, validation)
- Ensure image validation logic (type, size limits) is implemented in Rust backend
- Plan for test coverage: component tests, backend tests, image upload integration test

### Post-Design Gates (Phase 1)

| Principle                         | Status  | Assessment                                                                                                                                                    |
| --------------------------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Modular, Library-First Design** | ✅ PASS | Component designed as reusable module in `src/lib/components/RailwayModelCard.svelte`, documented in quickstart.md                                            |
| **Deterministic Interfaces**      | ✅ PASS | Props interface fully typed, Tauri command `upload_model_image` documented with Args/Result types, specta generates TS bindings                               |
| **Test-First Emphasis**           | ✅ PASS | Test requirements defined in contracts/upload-model-image.md (unit tests for validation paths, integration tests for upload flow, component tests for UI)     |
| **Code Quality**                  | ✅ PASS | Design follows pnpm lint/check and cargo clippy/fmt standards                                                                                                 |
| **Testing Standards**             | ✅ PASS | Coverage targets defined: component logic 70%+, backend commands 80%+, includes unit/integration/component tests                                              |
| **UX Consistency**                | ✅ PASS | Paraglide i18n keys defined in research.md, uses shadcn-svelte components, follows MEMORY.md card styling conventions                                         |
| **Performance Requirements**      | ✅ PASS | Performance targets documented: render <500ms (20 units), tab switch <100ms, upload <10s, validation <100ms                                                   |
| **Database Law**                  | ✅ PASS | Uses existing `railway_model` and `rolling_stock` tables, no new migrations, foreign key enforcement assumed enabled                                          |
| **State Management Law**          | ✅ PASS | Component is read-only display; image upload is a command (no domain events needed)                                                                           |
| **API Design Law**                | ✅ PASS | `upload_model_image` command follows ADR 8: UploadModelImageArgs (validated via validator::Validate), maps to Input, specta types, validation at boundary     |
| **Domain Logic Location**         | ✅ PASS | Image validation (MIME type via magic numbers, size, dimensions) implemented in Rust backend (ModelImage::validate), frontend only does pre-validation for UX |

**GATE RESULT**: ✅ ALL PASS

**Design Compliance Summary**:

- ✅ Tauri command contract defined and follows ADR 8 conventions
- ✅ Image validation logic is server-side (MIME detection via magic numbers, size limits, dimension checks)
- ✅ Component is truly reusable (props-based, no tight coupling)
- ✅ Testing strategy defined for all layers (component, backend, integration)
- ✅ All user-facing strings use Paraglide i18n
- ✅ No new database schema changes required
- ✅ Performance targets are measurable and realistic

**Ready for Phase 2** (tasks generation via `/speckit.tasks` command)

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
# Frontend (SvelteKit)
src/
├── lib/
│   ├── components/
│   │   ├── RailwayModelCard.svelte        # New: Reusable display component
│   │   ├── ui/                            # Existing: shadcn-svelte components
│   │   └── model-details/                 # Existing: Model-specific components
│   ├── features/                          # Optional: Feature-specific controllers
│   └── paraglide/                         # Existing: i18n messages
└── __tests__/                             # Vitest tests
    └── components/
        └── RailwayModelCard.test.ts       # New: Component tests

# Backend (Rust/Tauri)
src-tauri/
├── src/
│   ├── catalog/                           # Existing domain: manages railway models
│   │   ├── domain/                        # Aggregates, value objects, repositories
│   │   ├── application/                   # Use cases
│   │   ├── infrastructure/                # Repository implementations
│   │   └── interface/                     # Tauri commands
│   ├── media/                             # Existing domain: handles images
│   │   ├── domain/
│   │   ├── application/                   # May add: UploadModelImage use case
│   │   ├── infrastructure/
│   │   └── interface/                     # May add: upload_model_image command
│   └── core/                              # Shared domain primitives
└── migrations/                            # SQLx migrations (none needed for this feature)
```

**Structure Decision**: This is a desktop application (Tauri + SvelteKit) following clean architecture. The new component lives in `src/lib/components/` as a reusable module. Image upload functionality will extend the existing `media` domain with a new use case and Tauri command. No database migrations are required as we're using existing `railway_model` and `rolling_stock` tables from the catalog domain.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

**No violations detected** - all constitutional principles and architectural laws are satisfied by this design.
