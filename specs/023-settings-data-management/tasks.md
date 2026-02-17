# Tasks: Settings Data Management UI

**Input**: Design documents from `/specs/023-settings-data-management/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Tests are OPTIONAL for this feature - test tasks are included but not required for MVP delivery.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure for database backup feature

- [x] T001 Create backend module structure at src-tauri/src/database_backup/ with domain/, application/, infrastructure/ subdirectories
- [x] T002 [P] Create frontend feature structure at src/lib/features/database-backup/ with components/ subdirectory
- [x] T003 [P] Add i18n message keys to messages/en.json for data management UI strings (titles, descriptions, warnings, success/error messages)
- [x] T004 [P] Add i18n message keys to messages/it.json with Italian translations

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T005 [P] Create DatabaseBackupError enum in src-tauri/src/database_backup/domain/errors.rs
- [x] T006 [P] Implement CommandError conversion for DatabaseBackupError in src-tauri/src/database_backup/domain/errors.rs
- [x] T007 [P] Create database file validation function in src-tauri/src/database_backup/domain/validation.rs
- [x] T008 [P] Create domain module exports in src-tauri/src/database_backup/domain/mod.rs
- [x] T009 [P] Create database_backup module declaration in src-tauri/src/lib.rs
- [x] T010 [P] Create Result type alias in src/lib/services/types.ts for service layer responses (used SafeResult from errors.ts)
- [x] T011 [P] Create DatabaseBackupState type in src/lib/features/database-backup/DatabaseBackupState.svelte.ts

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Export Database Backup (Priority: P1) 🎯 MVP

**Goal**: Enable users to export their entire database to a local file for manual backups without cloud storage

**Independent Test**: Navigate to Settings page, click "Export Data", select save location, verify that a valid database backup file is created at the chosen location

### Tests for User Story 1 (OPTIONAL - only if tests requested) ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T012 [P] [US1] Create export use case unit test in src-tauri/src/database_backup/application/export_database.test.rs
- [ ] T013 [P] [US1] Create DataManagementSection component test in src/**tests**/features/database-backup/DataManagementSection.test.ts

### Implementation for User Story 1

- [x] T014 [P] [US1] Create ExportDatabaseArgs and ExportDatabaseResponse types in src-tauri/src/commands/database_backup.rs
- [x] T015 [P] [US1] Create export_database use case in src-tauri/src/database_backup/application/export_database.rs using VACUUM INTO
- [x] T016 [US1] Implement export_database command handler in src-tauri/src/commands/database_backup.rs with validation and error handling
- [x] T017 [US1] Register export_database command in src-tauri/src/commands/mod.rs
- [x] T018 [US1] Add export_database to invoke_handler in src-tauri/src/lib.rs
- [x] T019 [US1] Generate TypeScript bindings by running cargo build (debug mode regenerates bindings) + manual additions to bindings.ts
- [x] T020 [P] [US1] Create exportDatabase service function in src/lib/services/database-backup.ts
- [x] T021 [P] [US1] Export database-backup service from src/lib/services/index.ts
- [x] T022 [US1] Create DatabaseBackupController with handleExport method in src/lib/features/database-backup/DatabaseBackupController.svelte.ts
- [x] T023 [US1] Integrate @tauri-apps/plugin-dialog save() for export file picker in DatabaseBackupController
- [x] T024 [US1] Create DataManagementSection component with Export Data button in src/lib/features/database-backup/components/DataManagementSection.svelte
- [x] T025 [US1] Export DatabaseBackupController and types from src/lib/features/database-backup/index.ts
- [x] T026 [US1] Import and render DataManagementSection in src/routes/my-settings/+page.svelte above Cloud Backup section
- [x] T027 [US1] Add success toast notification for export completion using toaster service
- [x] T028 [US1] Add error toast notification for export failures using toaster service

**Checkpoint**: At this point, User Story 1 should be fully functional - users can export their database to a local file

---

## Phase 4: User Story 2 - Import Database Restore (Priority: P2)

**Goal**: Enable users to restore their database from a previously exported backup file for disaster recovery

**Independent Test**: Click "Import Data", select a previously exported backup file, confirm the warning dialog with "RESTORE", verify that the database is restored with the backup's contents

### Tests for User Story 2 (OPTIONAL - only if tests requested) ⚠️

- [ ] T029 [P] [US2] Create import use case unit test in src-tauri/src/database_backup/application/import_database.test.rs
- [ ] T030 [P] [US2] Create import validation tests in src-tauri/src/database_backup/domain/validation.test.rs

### Implementation for User Story 2

- [x] T031 [P] [US2] Create ImportDatabaseArgs and ImportDatabaseResponse types in src-tauri/src/commands/database_backup.rs
- [x] T032 [P] [US2] Create file copy utilities in src-tauri/src/database_backup/infrastructure/file_operations.rs
- [x] T033 [P] [US2] Implement confirmation validator for "RESTORE" string in src-tauri/src/commands/database_backup.rs
- [x] T034 [US2] Create import_database use case in src-tauri/src/database_backup/application/import_database.rs with validation
- [x] T035 [US2] Implement import_database command handler in src-tauri/src/commands/database_backup.rs
- [x] T036 [US2] Register import_database command in src-tauri/src/commands/mod.rs
- [x] T037 [US2] Add import_database to invoke_handler in src-tauri/src/lib.rs
- [x] T038 [US2] Regenerate TypeScript bindings (bindings.ts updated manually with new types and commands)
- [x] T039 [P] [US2] Create importDatabase service function in src/lib/services/database-backup.ts
- [x] T040 [US2] Add handleImport method to DatabaseBackupController in src/lib/features/database-backup/DatabaseBackupController.svelte.ts
- [x] T041 [US2] Integrate @tauri-apps/plugin-dialog open() for import file picker in DatabaseBackupController
- [x] T042 [US2] Create confirmation dialog logic for import with "RESTORE" validation in DatabaseBackupController (backend validates)
- [x] T043 [US2] Add Import Data button to DataManagementSection component in src/lib/features/database-backup/components/DataManagementSection.svelte
- [x] T044 [US2] Add success toast with restart requirement message for import completion
- [x] T045 [US2] Add error toast notification for import failures with validation messages
- [x] T046 [US2] Implement loading state management to disable buttons during import/export operations

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently - full backup and restore functionality

---

## Phase 5: User Story 3 - Visual Integration with Settings (Priority: P3)

**Goal**: Ensure the Data Management section appears above Cloud Backup with consistent styling for easy discovery and trust

**Independent Test**: Visually inspect the Settings page layout and verify that the Data Management section appears in the correct position with matching orange-bordered button styles

### Implementation for User Story 3

- [x] T047 [P] [US3] Update DataManagementSection styling to use card border-surface-700/40 border p-6 shadow-xl classes
- [x] T048 [P] [US3] Apply variant="default/outline" to Export and Import buttons (variant="filled" not in shadcn-svelte, used default/outline)
- [x] T049 [P] [US3] Add warning callout below Import button using warning color classes
- [x] T050 [US3] Verify DataManagementSection position above Cloud Backup section in src/routes/my-settings/+page.svelte
- [x] T051 [US3] Add section title and subtitle with consistent typography (text-xl font-bold and text-surface-400)
- [x] T052 [US3] Ensure button spacing and layout matches Cloud Backup section using flex gap-4

**Checkpoint**: All user stories should now be independently functional with consistent UI/UX

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Quality assurance and improvements that affect multiple user stories

- [x] T053 [P] Run pnpm format to format frontend code
- [x] T054 [P] Run pnpm lint and fix any ESLint warnings (fixed SvelteDate issue)
- [x] T055 [P] Run pnpm check to verify TypeScript types (0 errors, 0 warnings)
- [x] T056 [P] Run cargo fmt to format Rust code
- [x] T057 [P] Run cargo clippy and address any warnings (0 warnings)
- [x] T058 [P] Run cargo test to verify all backend tests pass (1085 passed)
- [ ] T059 Manual testing: Export database to local file and verify file integrity
- [ ] T060 Manual testing: Import previously exported database and verify data restoration
- [ ] T061 Manual testing: Test file picker cancellation for both export and import
- [ ] T062 Manual testing: Test error handling with invalid file paths and corrupted databases
- [ ] T063 Verify i18n strings render correctly in both English and Italian
- [ ] T064 Test UI responsiveness during export/import operations (loading states)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3)
- **Polish (Phase 6)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - Independent of US1 but builds on similar patterns
- **User Story 3 (P3)**: Depends on US1 and US2 UI components existing (or can be done in parallel with styling only)

### Within Each User Story

- Tests (if included) MUST be written and FAIL before implementation
- Backend types before use cases
- Use cases before command handlers
- Command registration before frontend service
- Service layer before controller
- Controller before UI components
- UI components before integration with Settings page

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel (T002, T003, T004)
- All Foundational tasks marked [P] can run in parallel (T005-T011)
- Within User Story 1: T014/T015 (backend), T020/T021 (service) can run in parallel
- Within User Story 2: T031/T032/T033 (backend prep), T039 (service) can run in parallel
- Within User Story 3: All styling tasks (T047-T052) can run in parallel
- All Polish tasks (T053-T058, T063) can run in parallel
- Different user stories can be worked on in parallel by different team members after Foundational phase

---

## Parallel Example: User Story 1

```bash
# Launch all backend types/use cases for User Story 1 together:
Task: "Create ExportDatabaseArgs and ExportDatabaseResponse types in src-tauri/src/commands/database_backup.rs"
Task: "Create export_database use case in src-tauri/src/database_backup/application/export_database.rs"

# Launch all frontend services for User Story 1 together:
Task: "Create exportDatabase service function in src/lib/services/database-backup.ts"
Task: "Export database-backup service from src/lib/services/index.ts"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001-T004)
2. Complete Phase 2: Foundational (T005-T011) - CRITICAL - blocks all stories
3. Complete Phase 3: User Story 1 (T014-T028)
4. **STOP and VALIDATE**: Test User Story 1 independently - can export database successfully
5. Run quality checks (Phase 6) and deploy/demo if ready

**MVP Deliverable**: Users can export their entire database to a local file for manual backups

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP - Export only!)
3. Add User Story 2 → Test independently → Deploy/Demo (Full backup/restore)
4. Add User Story 3 → Test independently → Deploy/Demo (Polished UI)
5. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together (T001-T011)
2. Once Foundational is done:
   - Developer A: User Story 1 (T014-T028) - Export functionality
   - Developer B: User Story 2 (T031-T046) - Import functionality
   - Developer C: User Story 3 (T047-T052) - UI polish
3. Stories complete and integrate independently
4. Final quality checks (Phase 6) done together

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Tests are OPTIONAL - included for reference but not required for MVP
- Verify tests fail before implementing (if tests are written)
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- After T019 and T038, TypeScript bindings will be auto-generated for frontend use
- File picker dialogs are handled by @tauri-apps/plugin-dialog (already installed)
- Follow existing Cloud Backup patterns for UI consistency
- All strings MUST use Paraglide i18n (no hardcoded English)
