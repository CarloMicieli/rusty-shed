# Implementation Plan: Settings Data Management UI

**Branch**: `023-settings-data-management` | **Date**: 2026-02-15 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/023-settings-data-management/spec.md`

## Summary

Add a "Data Management" section to the Settings page that enables users to manually backup (export) and restore (import) their entire SQLite database to/from a local file. This provides a privacy-focused alternative to cloud backup. The feature leverages Tauri's native file dialog APIs for save/open operations and performs simple file copy operations on the database file. The UI will be positioned above the existing Cloud Backup section with consistent styling using orange-bordered buttons and appropriate warning messages for the destructive import operation.

## Technical Context

**Language/Version**: Rust 1.93.0 (backend), TypeScript 5.9.3 (frontend)
**Primary Dependencies**:

- Backend: Tauri 2.9.x, sqlx (for database operations), tokio (async runtime)
- Frontend: SvelteKit (Svelte 5.48.2), Tailwind CSS 4.1.18, shadcn-svelte
  **Storage**: SQLite database (accessed via AppHandle path resolver for database file location)
  **Testing**:
- Backend: cargo test
- Frontend: Vitest 4.0.18 with happy-dom environment
  **Target Platform**: Desktop (Windows, macOS, Linux) via Tauri
  **Project Type**: Desktop application (Tauri + SvelteKit)
  **Performance Goals**:
- Export/import operations complete within 30s for databases up to 100MB
- UI remains responsive during file operations
  **Constraints**:
- File operations must be non-blocking (use async)
- Must validate database file integrity before restore
- Must show progress indicators for operations >2s
- Warning dialogs required before destructive import
  **Scale/Scope**:
- 2 Tauri commands (export, import)
- 1 new UI section component
- Minimal changes to existing Settings page layout

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

### Core Principles Compliance

- ✅ **Modular, Library-First Design**: Feature is self-contained with clear boundaries (UI component + Tauri commands)
- ✅ **Deterministic Interfaces & Observability**: Tauri commands will use specta for type safety, operations logged
- ✅ **Test-First Emphasis**: Unit tests for commands, component tests for UI
- ✅ **Code Quality**: Will follow lint/format standards (pnpm lint, cargo clippy)
- ✅ **Testing Standards**: Command validation tests, UI interaction tests
- ✅ **User Experience Consistency**: Uses Paraglide for strings, shadcn-svelte components, existing design tokens
- ✅ **Performance Requirements**: File operations async, progress indicators for operations >2s
- ✅ **Safe Rust Practices**: Result<T, E> error handling, no panics in production paths

### Architectural Laws Compliance

- ✅ **Database (Persistence)**: No new migrations needed - working with existing database file via file system operations
- N/A **State Management / Persistence Strategy**: No domain aggregates involved - pure file I/O operations
- ✅ **API Design & Transport Boundary**: Will use Tauri IPC with specta-generated TypeScript types
  - Commands: `export_database`, `import_database`
  - Args types will derive `Debug, Clone, validator::Validate, specta::Type, serde::Deserialize`
  - Input validation at boundary before file operations
- ✅ **Domain Logic Location**: Minimal business logic (file validation, copy operations) in Rust backend

### Gate Results

**✅ PASS** - No constitutional violations. Feature follows established patterns for Tauri commands and UI components.

## Project Structure

### Documentation (this feature)

```text
specs/023-settings-data-management/
├── plan.md              # This file
├── research.md          # Phase 0 output (Tauri APIs, file operations)
├── data-model.md        # Phase 1 output (command contracts)
├── quickstart.md        # Phase 1 output (developer guide)
├── contracts/           # Phase 1 output (TypeScript types)
│   └── database-backup.ts
└── tasks.md             # Phase 2 output (/speckit.tasks - NOT created yet)
```

### Source Code (repository root)

This is a Tauri desktop application with frontend (SvelteKit) and backend (Rust).

```text
# Backend (Rust/Tauri)
src-tauri/src/
├── commands/
│   ├── mod.rs                    # [MODIFY] Register new commands
│   └── database_backup.rs        # [NEW] Export/import command handlers
├── database_backup/              # [NEW] Feature module
│   ├── mod.rs
│   ├── application/
│   │   ├── mod.rs
│   │   ├── export_database.rs    # Export use case
│   │   └── import_database.rs    # Import use case
│   ├── domain/
│   │   ├── mod.rs
│   │   ├── errors.rs             # DatabaseBackupError types
│   │   └── validation.rs         # Database file validation
│   └── infrastructure/
│       ├── mod.rs
│       └── file_operations.rs    # File copy utilities
└── lib.rs                        # [MODIFY] Add module declaration

# Frontend (SvelteKit)
src/
├── lib/
│   ├── features/
│   │   └── database-backup/       # [NEW] Feature module
│   │       ├── index.ts
│   │       ├── components/
│   │       │   └── DataManagementSection.svelte  # [NEW] UI section component
│   │       ├── DatabaseBackupController.svelte.ts  # [NEW] State management
│   │       └── DatabaseBackupState.svelte.ts       # [NEW] Reactive state
│   └── services/
│       └── database-backup.ts     # [NEW] Tauri command wrappers
├── routes/
│   └── my-settings/
│       └── +page.svelte           # [MODIFY] Add DataManagementSection
└── paraglide/
    └── messages/
        └── en.json                # [MODIFY] Add i18n strings

# Tests
src-tauri/src/database_backup/
└── application/
    ├── export_database.test.rs    # [NEW] Export tests
    └── import_database.test.rs    # [NEW] Import tests

src/__tests__/
└── features/
    └── database-backup/
        └── DataManagementSection.test.ts  # [NEW] Component tests
```

**Structure Decision**: Feature follows the established clean architecture pattern used in Rusty Shed:

- **Backend**: Domain-driven design with application/domain/infrastructure layers
- **Frontend**: Feature-based organization with controller/state pattern (Svelte 5 runes)
- **Transport**: Tauri IPC commands with specta type generation
- **Testing**: Unit tests alongside implementation files, component tests in `__tests__`

This structure is consistent with existing features like `cloud-backup`, `import`, and `export`.

## Complexity Tracking

**No violations** - This feature fully complies with the constitution and follows existing architectural patterns.
