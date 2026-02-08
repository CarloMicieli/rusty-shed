# Implementation Plan: Data Archive Export

**Branch**: `016-data-archive-export` | **Date**: February 8, 2026 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `/specs/016-data-archive-export/spec.md`

## Summary

Implement a data export system allowing users to create backup archives (`.zip` format) containing their complete railway model collection data and associated images. The system mirrors the import feature's manifest structure for complete roundtrip compatibility, includes preview/confirmation workflows, supports selective entity export, provides progress feedback for large operations, and allows users to choose archive destination via native file picker.

## Technical Context

**Language/Version**: Rust 1.93.0 (backend), TypeScript 5.9.3 (frontend)  
**Primary Dependencies**:

- Backend: Tauri 2.9.x, sqlx (SQLite), tokio, serde/serde_json, zip crate, uuid (for temp files)
- Frontend: SvelteKit (Svelte 5.48.2), Tailwind CSS 4.x, shadcn-svelte, tauri-specta bindings

**Storage**: SQLite via sqlx (existing infrastructure, read-only for export)  
**Testing**: cargo test (Rust), Vitest (frontend)  
**Target Platform**: Desktop (Linux, macOS, Windows via Tauri)  
**Project Type**: Tauri desktop app with Svelte frontend  
**Performance Goals**: Export 50 records + 20 images in <15s; 1000+ records without UI freeze  
**Constraints**: <500ms for preview generation; <50MB intermediate temp files; progress updates every 500ms  
**Scale/Scope**: Typical export: 10-500 records + 10-100 images; max expected: 5000 records + 500 images

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

| Principle                            | Status  | Notes                                                                                                                |
| ------------------------------------ | ------- | -------------------------------------------------------------------------------------------------------------------- |
| **Database (Persistence)**           | ✅ PASS | Export reads from existing SQLite via sqlx; no migrations needed (read-only operation)                               |
| **State Management / Domain Events** | ✅ PASS | Export reads from existing domain aggregates via repositories; no state mutations; no event recording needed         |
| **API Design & Transport Boundary**  | ✅ PASS | New Tauri commands follow ADR 8: `ExportSessionArgs` with validation, specta-generated TypeScript bindings           |
| **Domain Logic Location**            | ✅ PASS | Export configuration, entity selection, and serialization logic in Rust backend; frontend only for UI/workflow       |
| **Code Quality**                     | ✅ PASS | clippy -D warnings, rustfmt, thiserror for errors, explicit error handling                                           |
| **Testing Standards**                | ✅ PASS | Unit tests for manifest generation; integration tests with fixture data; round-trip verification against import spec |
| **User Experience Consistency**      | ✅ PASS | Paraglide for strings; shadcn-svelte components; progress/feedback per constitution                                  |
| **Performance Requirements**         | ✅ PASS | Export on background thread; progress indicators; targets defined in Success Criteria                                |

## Project Structure

### Documentation (this feature)

```text
specs/016-data-archive-export/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
│   └── manifest.schema.json     # Reuse from import feature (spec 010)
└── tasks.md             # Phase 2 output (/speckit.tasks)
```

### Source Code (repository root)

```text
# Backend (Rust - src-tauri/src/)
src-tauri/src/
├── export/                         # NEW FEATURE MODULE
│   ├── mod.rs
│   ├── domain/
│   │   ├── mod.rs
│   │   ├── export_session.rs       # Export session aggregate with entity selection
│   │   ├── export_config.rs        # Configuration (destination, filename, entity types)
│   │   ├── export_result.rs        # Export result value object
│   │   ├── entity_selection.rs     # Selected entities (models, items, sellers, etc.)
│   │   └── manifest.rs             # Manifest generator (mirrors import structure)
│   ├── application/
│   │   ├── mod.rs
│   │   ├── preview_export.rs       # Preview generation use case
│   │   ├── collect_export_data.rs  # Aggregate data from repositories
│   │   └── execute_export.rs       # Execution use case
│   ├── infrastructure/
│   │   ├── mod.rs
│   │   ├── manifest_builder.rs     # JSON manifest construction
│   │   ├── archive_writer.rs       # ZIP archive creation
│   │   ├── file_picker.rs          # OS file picker integration (via Tauri dialog API)
│   │   ├── media_collector.rs      # Image file collection and copying
│   │   └── disk_space_checker.rs   # Available space validation
│   └── interface/
│       ├── mod.rs
│       └── commands.rs             # Tauri IPC commands

# Frontend (Svelte - src/)
src/lib/features/export/           # NEW FEATURE MODULE
├── components/
│   ├── ExportDialog.svelte         # Main export workflow dialog
│   ├── ExportEntitySelector.svelte # Checkboxes for entity type selection
│   ├── ExportPreview.svelte        # Preview summary display
│   ├── ExportProgress.svelte       # Progress indicator with ETA
│   └── ExportReport.svelte         # Completion report with warnings
├── export.controller.svelte.ts     # Feature controller with $state
└── types.ts                        # Frontend-specific types

# Shared
src-tauri/src/export/domain/manifest.rs    # Reuse manifest from import feature
messages/en.json                    # i18n keys for export strings
messages/it.json
```

**Structure Decision**: Follows existing DDD layered structure (domain → application → infrastructure → interface) per ADR-0004. Export is a new feature module at the same level as `catalog`, `collecting`, `sellers`, `maintenance`, and `import`. Manifest structure is shared with import feature (spec 010) for full roundtrip compatibility.

## Complexity Tracking

> No constitution violations requiring justification.

---

## Phase Dependencies & Critical Path

**Export depends on**:

- Import feature (spec 010) for manifest schema and format specification
- Existing domain models and repositories for data access
- Tauri file dialog API for destination selection

**Critical path**:

1. ✅ Specification complete
2. → Phase 0: Research (identify manifest schema reuse, OS file picker patterns, ZIP library selection)
3. → Phase 1: Design (data model, API contracts, quickstart)
4. → Phase 2: Implementation (task breakdown)

## Success Gates (Pre-Implementation)

Before development begins, the following must be verified:

- [ ] Manifest schema from import feature (spec 010) is stable and available
- [ ] Tauri dialog API capabilities documented (file picker support verified)
- [ ] Performance benchmarks for ZIP creation with 500+ images established
- [ ] Round-trip testing framework designed (export → import → compare)

---

## Notes for Implementation Phase

- **Manifest Reuse**: Export MUST generate manifests using identical structure to import feature. Consider shared domain model in both feature modules.
- **Cancellation Handling**: Partial ZIP files must be cleaned up immediately on cancellation. Use temporary filenames with guaranteed cleanup.
- **Progress Streaming**: Archive creation should not load entire manifest in memory; stream to ZIP as records are read.
- **Image Handling**: Consider symlinks vs. copying for media files; Windows compatibility may require copies.
- **Error Recovery**: Export failures must not leave database in locked state; ensure proper connection cleanup.
