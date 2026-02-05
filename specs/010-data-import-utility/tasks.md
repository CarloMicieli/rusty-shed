# Tasks: Data Import Utility

**Input**: Design documents from `/specs/010-data-import-utility/`  
**Prerequisites**: plan.md ✓, spec.md ✓, research.md ✓, data-model.md ✓, contracts/ ✓, quickstart.md ✓

---

## Format: `[ID] [P?] [Story?] Description`

- **[P]**: Can run in parallel (different files, no dependencies on incomplete tasks)
- **[US#]**: Which user story this task belongs to (US1, US2, US3, etc.)
- Include exact file paths in descriptions

## Path Conventions

- **Backend**: `src-tauri/src/import/` (Rust)
- **Frontend**: `src/lib/features/import/` (Svelte)
- **Routes**: `src/routes/my-settings/import/`
- **Messages**: `messages/{en,it}.json`

---

## Phase 1: Setup

**Purpose**: Project initialization, dependencies, and module structure

- [x] T001 Add new dependencies to `src-tauri/Cargo.toml`: zip, flate2, tar, jsonschema
- [x] T002 [P] Create import module directory structure: `src-tauri/src/import/{domain,application,infrastructure,interface}/`
- [x] T003 [P] Create import module entry points: `src-tauri/src/import/mod.rs` and layer mod.rs files
- [x] T004 Register import module in `src-tauri/src/lib.rs`
- [x] T005 [P] Copy manifest JSON schema to `src-tauri/src/import/domain/manifest_schema.json`
- [x] T006 [P] Create frontend feature directory structure: `src/lib/features/import/components/`
- [x] T007 [P] Add Paraglide i18n keys for import feature to `messages/en.json` and `messages/it.json`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core domain types and infrastructure that ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

### Domain Layer

- [x] T008 Create `ValidationError` types in `src-tauri/src/import/domain/validation_error.rs`
- [x] T009 [P] Create `ImportWarning` type in `src-tauri/src/import/domain/import_warning.rs`
- [x] T010 [P] Create `RecordCounts` value object in `src-tauri/src/import/domain/record_counts.rs`
- [x] T011 Create manifest DTOs (ManufacturerRecord, RailwayModelRecord, etc.) in `src-tauri/src/import/domain/manifest.rs`
- [x] T012 Create `ImportSession` aggregate with state machine in `src-tauri/src/import/domain/import_session.rs`
- [x] T013 [P] Create `ImportPreview` value object in `src-tauri/src/import/domain/import_preview.rs`
- [x] T014 [P] Create `ImportResult` value object in `src-tauri/src/import/domain/import_result.rs`
- [x] T015 Export all domain types in `src-tauri/src/import/domain/mod.rs`

### Infrastructure Layer

- [x] T016 Implement `ArchiveExtractor` with ZIP support in `src-tauri/src/import/infrastructure/archive_extractor.rs`
- [x] T017 Add tar.gz support to `ArchiveExtractor` in `src-tauri/src/import/infrastructure/archive_extractor.rs`
- [x] T018 Implement `SchemaValidator` with embedded JSON schema in `src-tauri/src/import/infrastructure/schema_validator.rs`
- [x] T019 [P] Implement scale string normalization utility in `src-tauri/src/import/infrastructure/normalizer.rs`
- [x] T020 Export infrastructure types in `src-tauri/src/import/infrastructure/mod.rs`

### Frontend Foundation

- [x] T021 Create `types.ts` with frontend-specific types in `src/lib/features/import/types.ts`
- [x] T022 Create base `import.controller.svelte.ts` with $state setup in `src/lib/features/import/import.controller.svelte.ts`

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - Import Valid Package (Priority: P1) 🎯 MVP

**Goal**: Users can drop a valid .zip/.tar.gz package and have all data imported to the database

**Independent Test**: Provide a valid archive with manifest and images → complete import → verify all records in database and images in media directory

### Application Layer for US1

- [ ] T023 [US1] Implement `ValidatePackageUseCase` in `src-tauri/src/import/application/validate_package.rs`
- [ ] T024 [US1] Implement `ExecuteImportUseCase` for writing records in `src-tauri/src/import/application/execute_import.rs`
- [ ] T025 [P] [US1] Create ID mapping service for external→internal ID resolution in `src-tauri/src/import/application/id_mapper.rs`
- [ ] T026 [US1] Export application layer in `src-tauri/src/import/application/mod.rs`

### Infrastructure for US1

- [ ] T027 [US1] Implement `MediaStorage` for copying images to app data dir in `src-tauri/src/import/infrastructure/media_storage.rs`
- [ ] T028 [US1] Add UUID-prefix collision avoidance to `MediaStorage`
- [ ] T092 [US1] Add image file extension validation (.png, .jpg, .jpeg only) in `ArchiveExtractor` or validation phase

### Interface Layer for US1

- [ ] T029 [US1] Create `AnalyzeImportPackageArgs` and response types in `src-tauri/src/import/interface/types.rs`
- [ ] T030 [US1] Implement `analyze_import_package` Tauri command in `src-tauri/src/import/interface/commands.rs`
- [ ] T031 [US1] Implement `execute_import` Tauri command in `src-tauri/src/import/interface/commands.rs`
- [ ] T032 [US1] Register commands in Tauri app builder (update `src-tauri/src/lib.rs` or main.rs)
- [ ] T033 [US1] Export interface types in `src-tauri/src/import/interface/mod.rs`

### Frontend for US1

- [ ] T034 [P] [US1] Create `ImportDropZone.svelte` component in `src/lib/features/import/components/ImportDropZone.svelte`
- [ ] T035 [US1] Add file handling methods to `import.controller.svelte.ts` (analyzePackage, executeImport)
- [ ] T036 [US1] Create import page route at `src/routes/my-settings/import/+page.svelte`

### Verification for US1

- [ ] T037 [US1] Create test fixture archive at `src-tauri/fixtures/test_import.zip`
- [ ] T038 [US1] Add unit tests for `ArchiveExtractor` in `src-tauri/src/import/infrastructure/archive_extractor.rs`
- [ ] T039 [US1] Add unit tests for `SchemaValidator` in `src-tauri/src/import/infrastructure/schema_validator.rs`

**Checkpoint**: User Story 1 complete - users can import valid packages

---

## Phase 4: User Story 2 - Preview Import Before Execution (Priority: P1)

**Goal**: Users see a detailed preview with record counts, duplicates, and validation issues before confirming

**Independent Test**: Load package → verify preview shows accurate counts → verify no data written until confirm

### Infrastructure for US2

- [ ] T040 [US2] Implement `DuplicateChecker` with batch key loading in `src-tauri/src/import/infrastructure/duplicate_checker.rs`
- [ ] T041 [US2] Add railway model duplicate detection (manufacturer_id + product_code) to `DuplicateChecker`
- [ ] T042 [US2] Add collection item duplicate detection (railway_model_id + purchase_date) to `DuplicateChecker`

### Application Layer for US2

- [ ] T043 [US2] Implement `PreviewImportUseCase` in `src-tauri/src/import/application/preview_import.rs`
- [ ] T044 [US2] Integrate `DuplicateChecker` into preview generation

### Interface Layer for US2

- [ ] T045 [US2] Create `GetImportPreviewArgs` and `ImportPreviewResponse` types in `src-tauri/src/import/interface/types.rs`
- [ ] T046 [US2] Implement `get_import_preview` Tauri command in `src-tauri/src/import/interface/commands.rs`

### Frontend for US2

- [ ] T047 [P] [US2] Create `ImportPreview.svelte` component in `src/lib/features/import/components/ImportPreview.svelte`
- [ ] T048 [US2] Add preview state and methods to `import.controller.svelte.ts`
- [ ] T049 [US2] Integrate preview step into import page workflow at `src/routes/my-settings/import/+page.svelte`
- [ ] T050 [US2] Add confirm/cancel buttons with proper state handling

**Checkpoint**: User Story 2 complete - users can preview before import

---

## Phase 5: User Story 3 - Handle Duplicate Records Gracefully (Priority: P2)

**Goal**: Existing local records are protected; duplicates are skipped with clear reporting

**Independent Test**: Add records manually → import package with same records → verify local records unchanged

### Backend for US3

- [ ] T051 [US3] Add manufacturer duplicate detection to `DuplicateChecker`
- [ ] T052 [US3] Add seller duplicate detection (by name) to `DuplicateChecker`
- [ ] T053 [US3] Add skip logic to `ExecuteImportUseCase` that uses DuplicateChecker results
- [ ] T054 [US3] Track skipped record counts in `ImportResult`

### Frontend for US3

- [ ] T055 [US3] Display duplicate counts in `ImportPreview.svelte`
- [ ] T056 [US3] Show which specific records will be skipped (expandable list)

**Checkpoint**: User Story 3 complete - duplicates handled gracefully

---

## Phase 6: User Story 4 - Receive Clear Completion Report (Priority: P2)

**Goal**: After import, users see a clear summary of what happened

**Independent Test**: Complete import → verify report shows added/skipped/warnings accurately

### Backend for US4

- [ ] T057 [US4] Ensure `ImportResult` includes all required counts (added, skipped, warnings)
- [ ] T058 [US4] Add image failure tracking to `ImportResult`

### Frontend for US4

- [ ] T059 [P] [US4] Create `ImportReport.svelte` component in `src/lib/features/import/components/ImportReport.svelte`
- [ ] T060 [US4] Add result state handling to `import.controller.svelte.ts`
- [ ] T061 [US4] Integrate report display into import page after execution completes

**Checkpoint**: User Story 4 complete - users get clear completion reports

---

## Phase 7: User Story 5 - Abort on Critical Validation Failure (Priority: P2)

**Goal**: Malformed packages are rejected atomically with clear error messages

**Independent Test**: Provide invalid manifest → verify zero records written → verify clear error shown

### Backend for US5

- [ ] T062 [US5] Implement relationship integrity validation in `SchemaValidator`
- [ ] T063 [US5] Add orphaned reference detection (e.g., manufacturerId not in manufacturers[])
- [ ] T064 [US5] Wrap `ExecuteImportUseCase` writes in sqlx transaction with rollback on any error
- [ ] T065 [US5] Map validation errors to user-friendly messages with path information

### Frontend for US5

- [ ] T066 [US5] Display validation errors in `ImportPreview.svelte` with affected record paths
- [ ] T067 [US5] Disable confirm button when `canImport: false`
- [ ] T068 [US5] Add error state display for aborted imports

**Checkpoint**: User Story 5 complete - validation failures abort cleanly

---

## Phase 8: User Story 6 - Handle Missing Images as Warnings (Priority: P3)

**Goal**: Missing images generate warnings but don't block data import

**Independent Test**: Provide package with missing images → verify data imports → verify warnings in report

### Backend for US6

- [ ] T069 [US6] Add image existence checking to validation phase
- [ ] T070 [US6] Create `ImportWarning` entries for missing images
- [ ] T071 [US6] Continue import when images missing, include warnings in result

### Frontend for US6

- [ ] T072 [US6] Display image warnings in `ImportPreview.svelte`
- [ ] T073 [US6] Show image warning count in `ImportReport.svelte`

**Checkpoint**: User Story 6 complete - missing images handled as warnings

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Progress reporting, session management, documentation, and verification

### Progress Reporting

- [ ] T074 [P] Implement `import-progress` Tauri event emission during long operations
- [ ] T075 [P] Create `ImportProgress.svelte` component in `src/lib/features/import/components/ImportProgress.svelte`
- [ ] T076 Subscribe to progress events in `import.controller.svelte.ts`
- [ ] T077 Display progress during import execution in page

### Session Management

- [ ] T078 Implement `cancel_import_session` Tauri command in `src-tauri/src/import/interface/commands.rs`
- [ ] T079 Add session cleanup logic (temporary files, in-memory state)
- [ ] T080 Handle page navigation during active import session

### Accessibility

- [ ] T090 [P] [a11y] Audit ImportDropZone.svelte for keyboard navigation and screen reader support
- [ ] T091 [P] [a11y] Audit ImportPreview.svelte, ImportProgress.svelte, ImportReport.svelte for ARIA labels and focus management

### Documentation & Verification

- [ ] T081 [P] Add rustdoc comments to all public types and functions
- [ ] T082 [P] Run `pnpm rust:format` and fix any formatting issues
- [ ] T083 Run `pnpm rust:clippy` and resolve all warnings
- [ ] T084 Run `pnpm rust:test` and verify all tests pass
- [ ] T085 [P] Run `pnpm lint` and `pnpm check` for frontend
- [ ] T086 Manual test: complete import with valid package
- [ ] T087 Manual test: verify duplicate detection with pre-existing records
- [ ] T088 Manual test: verify abort on invalid manifest
- [ ] T089 Update feature documentation if needed

### Performance Verification

- [ ] T093 [P] Create large test fixture (1000+ records) at `src-tauri/fixtures/test_import_large.zip`
- [ ] T094 Performance test: verify 50 records + 20 images imports in <30s (SC-001)
- [ ] T095 Load test: verify 1000+ records import completes without UI freeze >200ms (SC-007)

---

## Dependencies & Execution Order

### Phase Dependencies

```
Phase 1 (Setup) ─────────────────────────────────────────────────────────────►
                │
                ▼
Phase 2 (Foundational) ──────────────────────────────────────────────────────►
                │
                ├──────────────────┬──────────────────┬──────────────────────►
                │                  │                  │
                ▼                  ▼                  ▼
        Phase 3 (US1)      Phase 4 (US2)      [Can parallelize if staffed]
        Import Valid       Preview Import
                │                  │
                ▼                  ▼
        Phase 5 (US3) ◄────────────┘
        Duplicates
                │
                ▼
        Phase 6 (US4)
        Completion Report
                │
                ▼
        Phase 7 (US5)
        Validation Abort
                │
                ▼
        Phase 8 (US6)
        Missing Images
                │
                ▼
        Phase 9 (Polish)
```

### User Story Dependencies

| Story        | Depends On   | Can Start After                      |
| ------------ | ------------ | ------------------------------------ |
| **US1** (P1) | Foundational | Phase 2 complete                     |
| **US2** (P1) | Foundational | Phase 2 complete (parallel with US1) |
| **US3** (P2) | US1, US2     | Phase 4 complete                     |
| **US4** (P2) | US1          | Phase 3 complete                     |
| **US5** (P2) | US1, US2     | Phase 4 complete                     |
| **US6** (P3) | US1          | Phase 3 complete                     |

### MVP Scope

**Minimum Viable Product** = Phase 1 + Phase 2 + Phase 3 (User Story 1)

This delivers:

- ✅ Archive extraction (ZIP + tar.gz)
- ✅ Manifest schema validation
- ✅ Basic import to database
- ✅ Image file storage
- ✅ Drag-and-drop UI

---

## Parallel Execution Examples

### Setup Phase (Parallel)

```bash
# These can run simultaneously:
T002: Create directory structure
T003: Create module entry points
T005: Copy JSON schema
T006: Create frontend directories
T007: Add i18n keys
```

### Foundational Phase (Parallel)

```bash
# These can run simultaneously:
T009: ImportWarning type
T010: RecordCounts value object
T013: ImportPreview value object
T014: ImportResult value object
T019: Scale normalization utility
```

### User Story 1 + 2 (Team Parallel)

```bash
# Developer A: User Story 1 (Import)
T023-T039

# Developer B: User Story 2 (Preview) - starts after Phase 2
T040-T050
```

---

## Task Count Summary

| Phase                  | Tasks  | Parallelizable |
| ---------------------- | ------ | -------------- |
| Setup                  | 7      | 5              |
| Foundational           | 15     | 6              |
| US1 - Import Valid     | 18     | 4              |
| US2 - Preview          | 11     | 1              |
| US3 - Duplicates       | 6      | 0              |
| US4 - Report           | 5      | 1              |
| US5 - Validation Abort | 7      | 0              |
| US6 - Missing Images   | 5      | 0              |
| Polish                 | 21     | 9              |
| **Total**              | **95** | **26**         |
