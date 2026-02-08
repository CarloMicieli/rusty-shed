# Tasks: Data Archive Export

**Input**: Design documents from `/specs/016-data-archive-export/`  
**Prerequisites**: plan.md ✓, spec.md ✓, research.md ✓

---

## Format: `[ID] [P?] [Story?] Description`

- **[P]**: Can run in parallel (different files, no dependencies on incomplete tasks)
- **[US#]**: Which user story this task belongs to (US1, US2, US3, etc.)
- Include exact file paths in descriptions

## Path Conventions

- **Backend**: `src-tauri/src/export/` (Rust)
- **Frontend**: `src/lib/features/export/` (Svelte)
- **Routes**: `src/routes/my-settings/export/` or modal integration
- **Messages**: `messages/{en,it}.json`

---

## Phase 1: Setup & Infrastructure

**Purpose**: Project initialization, dependencies, and module structure

- [ ] T001 No new dependencies needed: `zip` crate already in `src-tauri/Cargo.toml`
- [ ] T002 [P] Create export module directory structure in `src-tauri/src/export/{domain,application,infrastructure,interface}/`
- [ ] T003 [P] Create export module entry points: `src-tauri/src/export/mod.rs` and layer mod.rs files
- [ ] T004 Register export module in `src-tauri/src/lib.rs` alongside import module
- [ ] T005 [P] Create frontend feature directory structure: `src/lib/features/export/components/`
- [ ] T005b [SPIKE] Verify Tauri file picker compatibility on Windows, macOS, Linux:
  - Test `app.dialog().file().blocking_save_file()` on each platform
  - Verify native dialogs appear and work as expected
  - Document any platform-specific behavior or limitations
  - Duration: 2-4 hours | Blocks: T016, T030 (file picker infrastructure)
- [ ] T006 [P] Add Paraglide i18n keys for export feature to `messages/en.json` and `messages/it.json`:
  - Keys: `export.dialog_title`, `export.button_export`, `export.button_cancel`, `export.success_notification`, `export.error_*`
- [ ] T007 Create test fixtures directory: `src-tauri/fixtures/export/` with sample data files

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core domain types and infrastructure that ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

### Domain Layer

- [ ] T008 [P] Create error types in `src-tauri/src/export/domain/error.rs`: `ExportError`, `ValidationError`, `DiskSpaceError`
- [ ] T009 [P] Create `ExportEntitySelection` struct in `src-tauri/src/export/domain/entity_selection.rs`:
  - Boolean fields: `include_railway_models`, `include_collection_items`, `include_sellers`, `include_maintenance_logs`, `include_dcc_roster`, `include_orphaned_images`
  - Validation: at least one entity type selected
- [ ] T010 [P] Create `ExportSession` aggregate in `src-tauri/src/export/domain/export_session.rs`:
  - State: Selecting → Previewing → Exporting → Completed
  - Fields: entity selection, destination path, estimated size, progress tracking
- [ ] T011 [P] Create `ExportConfig` value object in `src-tauri/src/export/domain/export_config.rs`:
  - Fields: destination_path, custom_filename, include_orphaned_images
- [ ] T012 [P] Create `ExportProgress` value object in `src-tauri/src/export/domain/export_progress.rs`:
  - Fields: phase (collecting|compressing|finalizing), percentage, current_item, estimated_seconds_remaining
- [ ] T013 [P] Create `ExportResult` value object in `src-tauri/src/export/domain/export_result.rs`:
  - Fields: archive_path, file_size_bytes, records_exported, warnings

### Infrastructure Layer - File System

- [ ] T014 Create disk space checker in `src-tauri/src/export/infrastructure/disk_space_checker.rs`:
  - Function: `check_available_space(path: &Path) -> Result<u64>` (returns available bytes)
  - Estimate archive size from entity counts
  - Validate 100MB safety buffer
- [ ] T015 [P] Create media collector in `src-tauri/src/export/infrastructure/media_collector.rs`:
  - Function: `collect_media_files(include_orphaned: bool) -> Result<Vec<MediaFile>>`
  - Scan media directory for image files
  - Detect orphaned images (not referenced by any record)
- [ ] T016 [P] Create file picker integration in `src-tauri/src/export/infrastructure/file_picker.rs`:
  - Function: `open_save_dialog(default_filename: &str) -> Result<Option<PathBuf>>`
  - Use Tauri dialog API: `app.dialog().file().set_file_name().add_filter().blocking_save_file()`
  - Return None on cancel

### Infrastructure Layer - Archive

- [ ] T017 Create manifest builder in `src-tauri/src/export/infrastructure/manifest_builder.rs`:
  - Function: `build_manifest(selection: &ExportEntitySelection) -> Result<ManifestDto>`
  - Query repositories for each selected entity type
  - Map domain models → ManifestDto (reuse from import feature)
  - Serialize to JSON with validation
- [ ] T018 Create archive writer in `src-tauri/src/export/infrastructure/archive_writer.rs`:
  - Function: `create_archive(manifest: ManifestDto, media_files: &[MediaFile], dest_path: &Path) -> Result<ExportResult>`
  - Stream manifest JSON to ZIP
  - Add media files to `/images/` folder
  - Return file size and summary

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - Export Complete Collection (Priority: P1) 🎯 MVP

**Goal**: Enable users to export their complete collection as a single ZIP archive with all data and images

**Independent Test**: Trigger export → receive archive file → verify manifest.json exists with all records → verify images present

### Implementation for User Story 1

- [ ] T019 [P] [US1] Create `preview_export` use case in `src-tauri/src/export/application/preview_export.rs`:
  - Function: `get_export_preview(selection: &ExportEntitySelection) -> Result<ExportPreview>`
  - Count records for each entity type
  - Estimate archive size
  - Detect warnings (missing images, orphaned files)
  - Return preview struct with counts and estimated size
- [ ] T020 [P] [US1] Create `collect_export_data` use case in `src-tauri/src/export/application/collect_export_data.rs`:
  - Function: `collect_data(selection: &ExportEntitySelection) -> Result<(ManifestDto, Vec<MediaFile>)>`
  - Query repositories for selected entities
  - Collect referenced media files
  - Build manifest DTO
- [ ] T021 [US1] Create `execute_export` use case in `src-tauri/src/export/application/execute_export.rs`:
  - Function: `export_to_archive(config: &ExportConfig, selection: &ExportEntitySelection) -> Result<ExportResult>`
  - Validate disk space (calls T014)
  - Collect data (calls T020)
  - Create archive (calls T018)
  - Emit progress events throughout
  - Return result with file location and summary
  - **Note**: This is backend business logic; frontend state management in T026 is separate (controller handles presentation state transitions, not export operations)
- [ ] T022 [US1] Create Tauri command interface in `src-tauri/src/export/interface/commands.rs`:
  - Command: `get_export_preview(app: AppHandle) -> Result<ExportPreview>`
  - Command: `open_export_dialog(app: AppHandle) -> Result<Option<String>>`
  - Command: `execute_export(app: AppHandle, selection: ExportEntitySelection) -> Result<ExportResult>`
  - Add progress event listener setup
- [ ] T023 [P] [US1] Create `ExportDialog.svelte` in `src/lib/features/export/components/ExportDialog.svelte`:
  - Shows file picker dialog (calls open_export_dialog)
  - Displays preview with record counts (calls get_export_preview)
  - Confirms before starting export
  - Triggers execute_export command
- [ ] T024 [P] [US1] Create `ExportPreview.svelte` in `src/lib/features/export/components/ExportPreview.svelte`:
  - Display summary: "X railway models, Y collection items, Z images"
  - Show estimated archive size
  - Display estimated time remaining
- [ ] T025 [P] [US1] Create `ExportProgress.svelte` in `src/lib/features/export/components/ExportProgress.svelte`:
  - Display progress bar (0-100%)
  - Show current phase (collecting/compressing/finalizing)
  - Show current item being processed
  - Show estimated time remaining
  - Update via Tauri event listener
- [ ] T026 [US1] Create export controller in `src/lib/features/export/export.controller.svelte.ts`:
  - State management: `$state` for export session (purely frontend presentation state)
  - Handle dialog lifecycle and UI transitions
  - Listen for progress events from backend (T021 emits these)
  - Manage progress updates display
  - Handle completion/errors display
  - **Note**: T021 (execute_export) handles all backend export logic; this controller only manages frontend component state and event subscriptions
- [ ] T027 [US1] Create `ExportReport.svelte` in `src/lib/features/export/components/ExportReport.svelte`:
  - Display success notification with archive location and file size
  - Show counts: "Exported X models, Y items, Z images"
  - Show any warnings (missing images, etc.)
  - Offer button to open archive location or try again
- [ ] T028 [US1] Integrate export UI into settings/collection area (route: `src/routes/my-settings/export/+page.svelte` or modal):
  - Add "Export Collection" button
  - Mount ExportDialog component
  - Handle result callbacks
- [ ] T029 [US1] Add round-trip test in `src-tauri/tests/integration/export_import_roundtrip.rs`:
  - Create fixture database with known data (50 models, 15 items, 10 images)
  - Export to temporary ZIP
  - Import ZIP into clean test database
  - Assert entity counts match exactly
  - Assert field values match (with normalized strings)

**Checkpoint**: User Story 1 is complete. Users can export their entire collection successfully. ✅

---

## Phase 4: User Story 2 - Choose Export Location (Priority: P1) 🎯 MVP

**Goal**: Allow users to select where the exported archive is saved

**Independent Test**: Trigger export → select custom location (desktop, external drive, etc.) → verify archive created at selected location

### Implementation for User Story 2

- [ ] T030 [P] [US2] Enhance file picker in `src-tauri/src/export/infrastructure/file_picker.rs`:
  - Support default filename generation: `rusty-shed-export-YYYY-MM-DD.zip`
  - Support date parameter for filename
  - Add ZIP file filter + "All Files" fallback
  - Test on Windows, macOS, Linux for native dialogs
- [ ] T031 [US2] Update `execute_export` use case in `src-tauri/src/export/application/execute_export.rs`:
  - Accept destination path from user selection
  - Validate path is writable before export
  - Create temporary file with UUID suffix during export
  - Rename to final filename on completion
  - Clean up temporary file on cancellation/failure
- [ ] T032 [P] [US2] Update `ExportDialog.svelte` in `src/lib/features/export/components/ExportDialog.svelte`:
  - Show file picker result (location and filename)
  - Allow user to confirm location before export
  - Display selected path clearly
- [ ] T033 [US2] Add test in `src-tauri/tests/integration/export_destination.rs`:
  - Test export to system temp directory
  - Test export to custom directory
  - Test export with custom filename
  - Verify file actually created at selected location
  - Test cancellation (no file left behind)

**Checkpoint**: User Story 2 is complete. Users have full control over where archives are saved. ✅

---

## Phase 5: User Story 3 - Export Preview Summary (Priority: P2)

**Goal**: Show users what will be exported before confirming

**Independent Test**: Open export dialog → view preview → verify counts match database

### Implementation for User Story 3

- [ ] T034 [P] [US3] Implement `ExportPreview` struct in `src-tauri/src/export/domain/export_preview.rs`:
  - Fields: railway_model_count, collection_item_count, seller_count, maintenance_log_count, dcc_roster_count, image_count, orphaned_image_count, estimated_size_bytes
  - Add warnings list
- [ ] T035 [US3] Enhance preview_export use case in `src-tauri/src/export/application/preview_export.rs`:
  - Query each repository for selected entity counts
  - Calculate estimated archive size (data + images + 10% overhead)
  - Detect missing images and orphaned files
  - Generate warning messages
- [ ] T036 [P] [US3] Enhance `ExportPreview.svelte` with detailed breakdown:
  - Show table: entity type → count
  - Show "X orphaned images detected" warning if applicable
  - Show estimated archive size in human-readable format (MB, GB)
  - Highlight if empty (0 records)
- [ ] T037 [US3] Add validation: disable export if no entities selected or database empty
  - In controller: prevent export if preview shows 0 records
  - Show message: "No data to export. Add items to your collection first."
- [ ] T038 [P] [US3] Add test in `src-tauri/tests/integration/export_preview.rs`:
  - Test preview with various entity combinations
  - Test preview accuracy (counts match database)
  - Test warning generation (missing images)
  - Test empty database preview

**Checkpoint**: User Story 3 is complete. Users see accurate preview before exporting. ✅

---

## Phase 6: User Story 4 - Selective Entity Export (Priority: P2)

**Goal**: Allow users to export only specific entity types

**Independent Test**: Select "Models only" → export → verify archive contains only models, no items/sellers → re-import to verify

### Implementation for User Story 4

- [ ] T039 [P] [US4] Create `ExportEntitySelector.svelte` in `src/lib/features/export/components/ExportEntitySelector.svelte`:
  - Checkboxes for each entity type (RailwayModel, CollectionItem, Seller, MaintenanceLog, DccRoster)
  - Checkbox for "Include orphaned images"
  - Show estimated record count per entity type
  - Show warning if selecting dependent types without dependencies
- [ ] T040 [US4] Enhance entity selection validation in `src-tauri/src/export/domain/entity_selection.rs`:
  - Validation: at least one entity type must be selected
  - Warning: if CollectionItems selected without RailwayModels
  - Warning: if MaintenanceLogs selected without CollectionItems
  - Allow selections anyway (users understand consequences)
- [ ] T041 [P] [US4] Update data collection in `src-tauri/src/export/application/collect_export_data.rs`:
  - Conditional queries based on `ExportEntitySelection` flags
  - Skip entities not selected
  - Validate relationships still intact for exported entities
- [ ] T042 [US4] Integrate entity selector into export flow:
  - Show selector before preview
  - Pass selection through to manifest builder
  - Update preview to show selected types only
- [ ] T043 [P] [US4] Add tests in `src-tauri/tests/integration/export_selective.rs`:
  - Test export with only RailwayModels selected
  - Test export with only CollectionItems (expect validation warning)
  - Test export with all entities selected
  - Re-import and verify only selected entities present

**Checkpoint**: User Story 4 is complete. Users have fine-grained control over what gets exported. ✅

---

## Phase 7: User Story 5 - Progress Feedback for Large Exports (Priority: P2)

**Goal**: Show progress updates during long-running exports

**Independent Test**: Export 500+ records → observe progress updates every ~500ms → completion within expected time

### Implementation for User Story 5

- [ ] T044 [P] [US5] Create progress event system in `src-tauri/src/export/infrastructure/archive_writer.rs`:
  - Emit `ExportProgress` events via Tauri `app.emit()`
  - Phase 1: collecting (emit per 10 records)
  - Phase 2: compressing (emit per 50MB)
  - Phase 3: finalizing
  - Ensure <500ms between events
- [ ] T045 [US5] Calculate ETA in `src-tauri/src/export/application/execute_export.rs`:
  - Track bytes written vs. total expected
  - Calculate time remaining
  - Emit with estimated_seconds_remaining field
- [ ] T046 [P] [US5] Enhance `ExportProgress.svelte` with real-time updates:
  - Add Tauri event listener for `ExportProgress` events
  - Update progress bar reactively
  - Show current phase name
  - Show "Processing record X of Y"
  - Show "~X seconds remaining"
  - Update minimum every 100ms, max every 500ms
- [ ] T047 [US5] Add time-based tests in `src-tauri/tests/integration/export_progress.rs`:
  - Export 500+ records
  - Measure event frequency (expect <500ms between updates)
  - Verify progress percentage increases monotonically
  - Verify final percentage is 100%
  - Verify total time < 10 seconds (for 500 records)

**Checkpoint**: User Story 5 is complete. Large exports provide clear progress feedback. ✅

---

## Phase 8: User Story 6 - Include Orphaned Images Warning (Priority: P3)

**Goal**: Detect and optionally include unlinked images in export

**Independent Test**: Upload image without linking to record → trigger export → see warning about orphaned image → optionally include

### Implementation for User Story 6

- [ ] T048 [P] [US6] Implement orphaned image detection in `src-tauri/src/export/infrastructure/media_collector.rs`:
  - Function: `detect_orphaned_images() -> Result<Vec<OrphanedImage>>`
  - Scan media directory for files
  - Query all repositories for image references
  - Find files not referenced
  - Return list with filenames and sizes
- [ ] T049 [US6] Integrate orphaned detection into preview:
  - Update `get_export_preview` to call orphaned detection
  - Add `orphaned_image_count` to preview
  - Add warning: "X orphaned images detected. Include in export?"
- [ ] T050 [P] [US6] Add orphaned images checkbox to `ExportEntitySelector.svelte`:
  - Checkbox: "Include X orphaned images"
  - Only show if orphaned images exist
  - Show file list tooltip on hover (filenames and sizes)
  - Default: unchecked (conservative)
- [ ] T051 [US6] Update media collection to include orphaned images if selected:
  - Function in `media_collector.rs`: `collect_media_files(include_orphaned: bool) -> Result<Vec<MediaFile>>`
  - If `include_orphaned = true`, add orphaned images to export
  - Add all to `/images/` folder in archive
- [ ] T052 [P] [US6] Add tests in `src-tauri/tests/integration/export_orphaned_images.rs`:
  - Create orphaned image files (not referenced by any record)
  - Test detection (correct count and filenames)
  - Test export without orphaned images
  - Test export with orphaned images (verify in ZIP)
  - Test export with no orphaned images

**Checkpoint**: User Story 6 is complete. No images are lost during export. ✅

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Improvements affecting multiple user stories

- [ ] T053 [P] Error handling & recovery:
  - Implement 5 key failure mode handlers:
    1. **Disk full**: Detect during pre-check (T014), display "Insufficient disk space" error before export
    2. **Permission denied**: Handle OS file access errors, display "Cannot write to selected location"
    3. **Path invalid/removed**: USB unplugged mid-export, detect during archive write, clean up and display "Destination no longer available"
    4. **Database locked**: Connection failure, display "Database access error, please try again"
    5. **Archive corruption**: ZIP write failure, delete partial file, display "Export failed to create archive"
  - Test each failure mode with integration tests
  - Ensure temporary files cleaned up after every failure
  - Add user-friendly error messages via Paraglide
- [ ] T054 [P] Documentation updates:
  - Update `docs/FEATURE_IMPLEMENTATION.md` with export feature overview
  - Update `docs/tauri-commands.md` with export commands
  - Add export feature quickstart
- [ ] T055 [P] Performance optimization:
  - Profile export with large datasets (1000+ records)
  - Optimize manifest serialization if needed
  - Verify streaming doesn't load everything in memory
  - Benchmark compression ratios and speeds
- [ ] T056 [P] Cross-platform testing:
  - Test on Windows with local and network paths
  - Test on macOS with local and iCloud Drive paths
  - Test on Linux with local and mounted paths
  - Verify dialog appearance and behavior per platform
- [ ] T057 Additional unit tests:
  - Unit test `ExportEntitySelection` validation
  - Unit test `ExportProgress` calculation
  - Unit test disk space checker with various scenarios
  - Unit test media collector with special characters in filenames
- [ ] T058 Security hardening:
  - Validate all file paths (prevent directory traversal)
  - Sanitize filenames for cross-platform compatibility (UTF-8)
  - Verify no sensitive data leaked in archive metadata
  - Test with Unicode and special characters
- [ ] T059 Run integration verification:
  - Execute full export/import round-trip test (T029)
  - Verify feature checklist in `specs/016-data-archive-export/checklists/` completed
  - Generate feature implementation report

**Checkpoint**: Feature complete, tested, documented, and ready for merge. ✅

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - **BLOCKS all user stories**
- **User Stories (Phase 3-8)**: All depend on Foundational phase completion
  - Phase 3 & 4 (US1 & US2): Core functionality, should start together (interdependent)
  - Phase 5-8 (US3-6): Enhancements, can run after US1/US2 or in parallel
- **Polish (Phase 9)**: Depends on desired user stories being complete

### Within-User-Story Dependencies

- **US1 → US2**: US2 builds on US1's foundation; US2 tasks depend on T021 completion
- **US3**: Independent once foundational is done; only requires US1's infrastructure
- **US4**: Independent but validates against US1's architecture
- **US5**: Independent; adds to any story's execution
- **US6**: Independent; enhances data completeness

### Parallel Opportunities

**Phase 1**: All tasks marked [P] can run in parallel (different files)

- T002, T003, T005, T006, T007 in parallel

**Phase 2**: All tasks marked [P] can run in parallel within layers

- Domain [P] tasks in parallel: T008, T009, T012
- Infrastructure [P] tasks in parallel: T014, T015, T016

**Phase 3 & 4** (US1 & US2): Can start immediately after Phase 2

- US1 frontend [P] tasks in parallel: T023, T024, T025, T027
- US2 enhancements: T030, T032 in parallel
- US1 & US2 can be developed by different team members in parallel

**After US1 & US2**: Phase 5, 6, 7, 8 can proceed in parallel by different developers

**Phase 9**: All [P] tasks can run in parallel once all user stories complete

---

## Parallel Example: User Story 1

```
Once Phase 2 foundational is complete:

Backend Track:
- Task T019 (preview_export use case)
- Task T020 (collect_export_data use case)
- Task T021 (execute_export use case)
- Task T022 (Tauri commands)
└─ All can run in parallel until T021 (only T021 depends on T020 completion)

Frontend Track (parallel to backend):
- Task T023 (ExportDialog.svelte)
- Task T024 (ExportPreview.svelte)
- Task T025 (ExportProgress.svelte)
- Task T027 (controller)
├─ T023, T024, T025 can run in parallel
├─ T026 depends on T023, T024, T025
└─ T027 can run independently

Testing Track (parallel to both):
- Task T029 (round-trip test)
└─ Requires export infrastructure complete (T018, T022)
```

---

## Implementation Strategy

### MVP First (User Stories 1 & 2 Only)

1. ✅ Complete Phase 1: Setup (1-2 days)
2. ✅ Complete Phase 2: Foundational (3-4 days)
3. ✅ Complete Phase 3: User Story 1 (3-4 days)
4. ✅ Complete Phase 4: User Story 2 (1-2 days)
5. **STOP and VALIDATE**: Test export/import round-trip works
6. Deploy/demo MVP

**MVP Features**: Full collection export, choose destination, success notification

**MVP Timeline**: 8-12 days with single developer, 4-6 days with parallel team

### Incremental Delivery

| Iteration | Deliverable                                  | Timeline  |
| --------- | -------------------------------------------- | --------- |
| MVP       | US1 + US2 (full export + location selection) | 8-12 days |
| 1.1       | Add US3 (preview summary)                    | +2-3 days |
| 1.2       | Add US4 (selective export)                   | +3-4 days |
| 1.3       | Add US5 (progress feedback)                  | +2-3 days |
| 1.4       | Add US6 (orphaned images) + Polish           | +2-3 days |

### Parallel Team Strategy (4-person team)

**Days 1-4**: Everyone on Phases 1-2 (setup + foundational)

**Days 5-7**:

- Developer A: US1 backend (T019-T022)
- Developer B: US1 frontend (T023-T027)
- Developer C: US2 enhancements (T030-T032)
- Developer D: Testing/Integration (T029, T033)

**Days 8+**:

- Pair up and tackle US3, US4, US5, US6 in parallel phases

---

## Testing Strategy

### Test-First Approach

1. Write failing integration tests first (T029, T033, T038, etc.)
2. Implement features to make tests pass
3. Add unit tests for edge cases
4. Round-trip validation before merging

### Test Coverage

- **Integration Tests**: Full export/import round-trip (T029)
- **Functional Tests**: Each user story (T033, T038, etc.)
- **Unit Tests**: Domain logic, error handling (T057)
- **Cross-Platform**: Windows, macOS, Linux (T056)
- **Performance**: Large datasets without UI freeze (T047)

### Acceptance Criteria

Each user story is considered complete when:

- All tasks marked [✓] are done
- Associated tests all pass
- Feature works independently (no regressions in other stories)
- Performance meets Success Criteria from spec.md

---

## Notes

- **[P] tasks**: Marked for parallel execution - different files, no blocking dependencies
- **[US#] labels**: Enable traceability to user stories and independent delivery
- **Exact paths**: All file paths specified - copy/paste ready
- **Test-first**: Write failing tests before implementation
- **Incremental**: Can stop at any checkpoint and have working feature
- **Parallel**: Setup for single developer or large team

---

## Sign-Off Checklist

- [ ] All Phase 1 tasks complete
- [ ] All Phase 2 tasks complete
- [ ] All Phase 3 & 4 tasks complete (MVP ready)
- [ ] MVP tested and validated
- [ ] All Phase 5-8 tasks complete
- [ ] Phase 9 Polish tasks complete
- [ ] Feature integration complete
- [ ] Round-trip export/import verified
- [ ] Cross-platform testing complete
- [ ] Performance benchmarks met
- [ ] Ready for merge and release

---

**Generated**: February 8, 2026 | **Status**: Ready for Implementation | **Total Tasks**: 59 | **Estimated Solo Dev**: 10-14 days | **Estimated 4-Person Team**: 4-6 days
