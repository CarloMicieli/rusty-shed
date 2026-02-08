# Implementation Plan: Model Image Upload System

**Branch**: `015-model-image-upload` | **Date**: February 8, 2026 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/015-model-image-upload/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Implement an image upload system for the Model Details page that allows users to add photographs to railway models via file explorer selection or drag & drop. The system validates web-friendly image formats (JPEG, PNG, WEBP), copies files to the application's AppData directory with deterministic filenames based on model IDs, and persists references for rendering. Images use a naming convention that replaces colons in model IDs with underscores for file system compatibility, eliminating the need for a database column.

## Technical Context

**Language/Version**: Rust 1.93.0 (backend), TypeScript 5.9.3 (frontend)  
**Primary Dependencies**:

- Backend: Tauri 2.9.x, `@tauri-apps/plugin-dialog`, `@tauri-apps/plugin-fs`
- Frontend: SvelteKit (Svelte 5.48.2), Vite 7.3.1, shadcn-svelte, Tailwind CSS 4.1.18
- Image validation: `image` crate (Rust), MIME type checking
- File operations: `tokio::fs`, `std::fs`

**Storage**:

- Image files: Filesystem in AppData directory (`{app_data_dir}/models/`)
- Model references: SQLite via `sqlx` (no schema changes needed - using deterministic file naming)
- Naming convention: `{model_id_with_underscores}.{extension}` (e.g., `marklin_39216.jpg`)

**Testing**:

- Backend: `cargo test` with unit tests for file operations, validation, and path resolution
- Frontend: Vitest 4.0.18 with `happy-dom` for component testing
- Integration: Manual testing with real file uploads, drag & drop interactions

**Target Platform**: Desktop (Linux, Windows, macOS via Tauri)

**Project Type**: Desktop application (Tauri 2 + SvelteKit)

**Performance Goals**:

- Upload operations complete within 5 seconds for files under 10MB
- File validation completes in <200ms
- Visual feedback (drag states, loading) appears within 100ms

**Constraints**:

- Maximum file size: 50MB per image
- Supported formats: JPEG, PNG, WEBP only
- Single image per model (replacement supported)
- Strict Tauri security model (no direct file:// protocol access)

**Scale/Scope**:

- Single-user desktop application
- Expected: 10-1000 railway models per user
- Image storage: Typical 1-5MB per image
- Total storage: ~50MB-5GB depending on collection size

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

### ✅ Database (Persistence) — REQUIRED

**Compliance**: PASS ✅

- No new database tables or columns required
- Existing SQLite database accessed via `sqlx`
- No schema migrations needed (using deterministic file naming instead of DB column)
- Foreign key enforcement already enabled globally
- **Rationale**: File paths are computed deterministically from model ID, eliminating need for persistence

### ✅ State Management / Persistence Strategy — REQUIRED

**Compliance**: PASS ✅

- No domain aggregates modified
- File uploads are stateless operations (copy file, compute path)
- No domain events required (pure infrastructure operation)
- Model entity remains unchanged
- **Rationale**: Image upload is an infrastructure concern, not a domain state change

### ✅ API Design & Transport Boundary — REQUIRED

**Compliance**: PASS ✅

- New Tauri command: `upload_model_image` with `UploadModelImageArgs`
- Args will derive: `Debug, Clone, validator::Validate, specta::Type, serde::Deserialize`
- Validation at boundary: file format, size, model existence
- Type generation via `specta-typescript` in build pipeline
- No local network ports (using Tauri IPC)
- **Rationale**: Follows ADR 8 conventions for transport DTOs

### ✅ Domain Logic Location — REQUIRED

**Compliance**: PASS ✅

- All validation logic (format, size, filename sanitization) in Rust backend
- Frontend only handles UI interactions and triggers backend commands
- File operations isolated in infrastructure layer
- **Rationale**: Business rules (validation, storage) remain server-side

### ✅ Code Quality (Constitution v1.2.0)

**Compliance**: PASS ✅

- Will follow `cargo fmt` and `cargo clippy -D warnings`
- Frontend linting with ESLint and Prettier
- Type-safe interfaces with `specta` type generation
- Comprehensive error handling with `thiserror`
- Unit tests for validation, file operations, path resolution

### ✅ Testing Standards (Constitution v1.2.0)

**Compliance**: PASS ✅

- Unit tests: File validation, path resolution, filename sanitization
- Integration tests: End-to-end upload flow (manual testing)
- Component tests: Drag & drop component behavior
- Error scenario coverage: Invalid formats, file system errors, permissions

### ✅ User Experience Consistency (Constitution v1.2.0)

**Compliance**: PASS ✅

- All user-facing strings via Paraglide-JS messaging system
- Consistent error messages and feedback patterns
- shadcn-svelte components for upload UI
- Tailwind design tokens for styling
- Accessible drag & drop zones with keyboard alternatives

### ✅ Performance Requirements (Constitution v1.2.0)

**Compliance**: PASS ✅

- Target: <5s for files under 10MB (I/O bound, reasonable for desktop)
- File validation: <200ms (image format detection)
- UI feedback: <100ms (drag state changes)
- No UI blocking: File operations run in Rust backend (async)
- **Benchmarking plan**: Manual testing with various file sizes, profiling if needed

### ✅ Safe Rust Practices (Constitution v1.2.0)

**Compliance**: PASS ✅

- Error handling via `Result<T, E>` (no panics in production)
- Avoid `unsafe` code
- Use `tokio::fs` for async file operations
- Path validation to prevent directory traversal
- MIME type validation to prevent malicious file uploads

### Summary

**All Constitutional Gates: PASS ✅**

No violations or complexity justifications required. This feature aligns with existing architectural patterns and introduces minimal complexity.

## Project Structure

### Documentation (this feature)

```text
specs/015-model-image-upload/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
# Backend: Tauri Rust Application (src-tauri/)
src-tauri/
├── src/
│   ├── media/                          # Existing module (Feature 014)
│   │   ├── application/
│   │   │   └── upload_model_image.rs   # NEW: Upload use case
│   │   ├── domain/
│   │   │   └── image_validation.rs     # NEW: Validation logic
│   │   ├── infrastructure/
│   │   │   ├── file_storage.rs         # NEW: File operations
│   │   │   └── path_resolver.rs        # Existing: Path resolution (reuse)
│   │   └── interface/
│   │       └── commands.rs             # MODIFY: Add upload_model_image command
│   └── lib.rs                          # MODIFY: Register new command
│
├── capabilities/
│   └── default.json                    # MODIFY: Add fs:allow-write, fs:allow-read
│
└── Cargo.toml                          # MODIFY: Add dependencies (image, mime, uuid)

# Frontend: SvelteKit Application (src/)
src/
├── lib/
│   ├── components/
│   │   └── model-details/
│   │       ├── ImageUpload.svelte      # NEW: Upload UI component
│   │       └── ImageDropZone.svelte    # NEW: Drag & drop zone
│   └── bindings.ts                     # AUTO-GENERATED: TypeScript types
│
├── routes/
│   └── models/
│       └── [modelId]/
│           └── +page.svelte            # MODIFY: Integrate upload component
│
└── __tests__/
    └── components/
        └── ImageUpload.test.ts         # NEW: Component tests

# Localization
messages/
├── en.json                             # MODIFY: Add upload-related messages
└── it.json                             # MODIFY: Add upload-related messages
```

**Structure Decision**: This feature extends the existing **media** module (introduced in Feature 014) with upload capabilities. The backend follows the project's layered DDD architecture (domain, application, infrastructure, interface). Frontend components live alongside existing model-details components. No new top-level modules required.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation                  | Why Needed         | Simpler Alternative Rejected Because |
| -------------------------- | ------------------ | ------------------------------------ |
| [e.g., 4th project]        | [current need]     | [why 3 projects insufficient]        |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient]  |

---

## Planning Summary

### Phase 0: Research ✅ COMPLETE

**Deliverable**: [research.md](./research.md)

**Key Decisions**:

- File selection: `@tauri-apps/plugin-dialog` with format filters
- File operations: `tokio::fs` in Rust backend (no Tauri fs plugin needed)
- Format validation: `image` crate for magic byte detection
- Filename strategy: Model ID with `:` → `_`, deterministic naming
- Drag & drop: Two-command approach (path-based + bytes-based)
- Image display: Reuse existing asset protocol from Feature 014
- Error handling: `thiserror` + Paraglide i18n messages
- Storage location: `{app_data_dir}/models/`

**Dependencies Added**:

- Backend: `image = "0.25"`, `thiserror = "2.0"`, `tokio` with `fs` feature
- Frontend: `@tauri-apps/plugin-dialog`

---

### Phase 1: Design & Contracts ✅ COMPLETE

**Deliverables**:

- [data-model.md](./data-model.md) - Domain entities, value objects, DTOs
- [contracts/upload_model_image.md](./contracts/upload_model_image.md) - Path-based upload API
- [contracts/upload_model_image_bytes.md](./contracts/upload_model_image_bytes.md) - Bytes-based upload API
- [contracts/delete_model_image.md](./contracts/delete_model_image.md) - Delete API
- [quickstart.md](./quickstart.md) - Developer onboarding guide

**Key Entities**:

- `ImageFormat` - Value object for JPEG/PNG/WEBP
- `ModelImagePath` - Value object for file path resolution
- `FileSize` - Value object with 50MB validation
- `ImageValidator` - Domain service for validation
- `FileStorage` - Infrastructure for file operations

**APIs Defined**:

1. `upload_model_image(modelId, filePath)` - File Explorer upload
2. `upload_model_image_bytes(modelId, fileName, fileData)` - Drag & Drop upload
3. `delete_model_image(modelId)` - Image deletion

**No Database Changes**: Uses deterministic file naming convention

---

### Phase 2: Tasks (Next Step)

**Status**: NOT STARTED

**Next Command**: `/speckit.tasks`

This will generate detailed implementation tasks based on:

- User stories from [spec.md](./spec.md)
- Technical design from this plan
- Contracts from [contracts/](./contracts/)

---

## Constitution Re-Check (Post-Design)

All Constitutional Gates: **PASS ✅**

No violations introduced during design phase. All patterns align with existing architecture.

---

## Architectural Integration

### Extends Feature 014 (Railway Model Details Page)

**Reuses**:

- Media module structure (`src-tauri/src/media/`)
- Path resolution logic (`:` → `_` sanitization)
- Asset protocol for image display
- `getRailwayModelImage` command for retrieval

**Adds**:

- Upload use cases and validation
- File storage infrastructure
- Three new Tauri commands
- Upload UI components (button + drag & drop zone)

### No Breaking Changes

- No existing APIs modified
- No database migrations
- No configuration changes required
- Purely additive feature

---

## Implementation Estimate

**Complexity**: Medium

**Estimated Effort**: 8-12 hours

**Breakdown**:

- Backend (50%): 4-6 hours
  - Validation logic: 1-2h
  - Use cases: 2-3h
  - Commands: 1h
  - Tests: 1h
- Frontend (40%): 3-5 hours
  - Components: 2-3h
  - Integration: 1-2h
- Localization (10%): 1h

**Prerequisites**: None (Feature 014 already merged)

**Blockers**: None identified

---

## Risk Assessment

| Risk                              | Likelihood | Impact | Mitigation                                  |
| --------------------------------- | ---------- | ------ | ------------------------------------------- |
| Large files slow UI               | Medium     | Low    | Async operations, loading indicators        |
| Drag & drop browser compatibility | Low        | Medium | Graceful fallback to file explorer          |
| Disk space exhaustion             | Low        | High   | Check space before upload, clear errors     |
| Malicious file uploads            | Low        | High   | Magic byte validation, format whitelist     |
| Permission errors                 | Low        | Medium | Early directory check, clear error messages |

**Overall Risk**: LOW

---

## Next Steps

1. **Generate Tasks**: Run `/speckit.tasks` to create implementation task breakdown
2. **Backend First**: Implement validation → use cases → commands
3. **Frontend Second**: Build components → integrate → test
4. **Polish**: Add i18n messages → format → lint
5. **Test**: Manual testing all scenarios
6. **Merge**: PR review → merge to main

---

**Planning Phase Complete**: February 8, 2026

**Feature**: 015-model-image-upload  
**Branch**: `015-model-image-upload`  
**Ready for**: Task generation and implementation
