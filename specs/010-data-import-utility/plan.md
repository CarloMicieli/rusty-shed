# Implementation Plan: Data Import Utility

**Branch**: `010-data-import-utility` | **Date**: January 30, 2026 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `/specs/010-data-import-utility/spec.md`

## Summary

Implement a package-based data import system allowing users to migrate railway model collection data (catalogue models, collection items, sellers, maintenance logs) from external archives (`.zip`/`.gz`) into Rusty Shed. The system performs two-stage validation (JSON schema + asset validation), applies a local-first skip policy for duplicates, and provides a preview/confirmation workflow before atomic database writes.

## Technical Context

**Language/Version**: Rust 1.93.0 (backend), TypeScript 5.9.3 (frontend)  
**Primary Dependencies**:

- Backend: Tauri 2.9.x, sqlx (SQLite), tokio, serde/serde_json, zip crate, flate2 (gzip), jsonschema (validation)
- Frontend: SvelteKit (Svelte 5.48.2), Tailwind CSS 4.x, Skeleton UI 4.x, tauri-specta bindings

**Storage**: SQLite via sqlx (existing infrastructure)  
**Testing**: cargo test (Rust), Vitest (frontend)  
**Target Platform**: Desktop (Linux, macOS, Windows via Tauri)  
**Project Type**: Tauri desktop app with Svelte frontend  
**Performance Goals**: Import 50 records + 20 images in <30s; 1000+ records without UI freeze  
**Constraints**: <200ms for initial validation display; atomic transactions; offline-capable  
**Scale/Scope**: Typical import: 10-500 records; max expected: 5000 records

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

| Principle                            | Status  | Notes                                                                                                                   |
| ------------------------------------ | ------- | ----------------------------------------------------------------------------------------------------------------------- |
| **Database (Persistence)**           | ✅ PASS | Import uses sqlx with SQLite; new migrations for import session tracking if needed; `PRAGMA foreign_keys = ON` enforced |
| **State Management / Domain Events** | ✅ PASS | Import writes through existing repositories that use domain event pattern; atomic transactions for rollback on failure  |
| **API Design & Transport Boundary**  | ✅ PASS | New Tauri commands follow ADR 8: `ImportPackageArgs` with validation, specta-generated TypeScript bindings              |
| **Domain Logic Location**            | ✅ PASS | All validation, duplicate detection, and import logic in Rust backend; frontend only for UI/workflow                    |
| **Code Quality**                     | ✅ PASS | clippy -D warnings, rustfmt, thiserror for errors                                                                       |
| **Testing Standards**                | ✅ PASS | Unit tests for validation/duplicate logic; integration tests with fixture archives                                      |
| **User Experience Consistency**      | ✅ PASS | Paraglide for strings; Skeleton UI components; progress/feedback per constitution                                       |
| **Performance Requirements**         | ✅ PASS | Import on background thread; progress indicators; targets defined in Success Criteria                                   |

## Project Structure

### Documentation (this feature)

```text
specs/010-data-import-utility/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output (JSON Schema)
│   └── manifest.schema.json
└── tasks.md             # Phase 2 output (/speckit.tasks)
```

### Source Code (repository root)

```text
# Backend (Rust - src-tauri/src/)
src-tauri/src/
├── import/                     # NEW FEATURE MODULE
│   ├── mod.rs
│   ├── domain/
│   │   ├── mod.rs
│   │   ├── import_package.rs       # Import package value object
│   │   ├── import_session.rs       # Import session aggregate
│   │   ├── import_result.rs        # Import result value object
│   │   ├── validation_error.rs     # Validation error types
│   │   └── manifest.rs             # Manifest DTOs for deserialization
│   ├── application/
│   │   ├── mod.rs
│   │   ├── validate_package.rs     # Validation use case
│   │   ├── preview_import.rs       # Preview generation use case
│   │   └── execute_import.rs       # Execution use case
│   ├── infrastructure/
│   │   ├── mod.rs
│   │   ├── archive_extractor.rs    # ZIP/GZ extraction
│   │   ├── schema_validator.rs     # JSON Schema validation
│   │   ├── duplicate_checker.rs    # Database duplicate lookup
│   │   └── media_storage.rs        # Image file management
│   └── interface/
│       ├── mod.rs
│       └── commands.rs             # Tauri IPC commands

# Frontend (Svelte - src/)
src/lib/features/import/          # NEW FEATURE MODULE
├── components/
│   ├── ImportDropZone.svelte       # Drag-and-drop file input
│   ├── ImportPreview.svelte        # Preview summary display
│   ├── ImportProgress.svelte       # Progress indicator
│   └── ImportReport.svelte         # Completion report
├── import.controller.svelte.ts     # Feature controller with $state
└── types.ts                        # Frontend-specific types

src/routes/my-settings/import/
└── +page.svelte                    # Import page (or modal)

# Shared
src-tauri/src/import/domain/manifest_schema.json  # Embedded JSON Schema
messages/en.json                    # i18n keys for import strings
messages/it.json
```

**Structure Decision**: Follows existing DDD layered structure (domain → application → infrastructure → interface) per ADR-0004. Import is a new feature module at the same level as `catalog`, `collecting`, `sellers`, and `maintenance`.

## Complexity Tracking

> No constitution violations requiring justification.

---

## Phase Status

- [x] Technical Context filled
- [x] Constitution Check passed
- [x] Phase 0: Research (research.md) ✓ Complete
- [x] Phase 1: Design & Contracts (data-model.md, contracts/, quickstart.md) ✓ Complete
- [x] Phase 2: Tasks (tasks.md) ✓ Complete
