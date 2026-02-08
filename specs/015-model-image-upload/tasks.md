# Tasks: Model Image Upload System

**Input**: Design documents from `/specs/015-model-image-upload/`  
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/ ✅

**Tests**: Unit tests included for backend validation and file operations as per constitution requirements.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Backend**: `src-tauri/src/` (Rust Tauri application)
- **Frontend**: `src/lib/` (SvelteKit application)
- **Tests**: `src-tauri/src/` (inline with Rust modules), `src/__tests__/` (frontend tests)
- **Messages**: `messages/` (Paraglide localization)

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and dependency setup

- [x] T001 Add backend dependencies to src-tauri/Cargo.toml (image, thiserror, tokio with fs feature)
- [x] T002 Add frontend dependencies to package.json (@tauri-apps/plugin-dialog)
- [x] T003 Run pnpm install to install @tauri-apps/plugin-dialog

**File Paths**:

- `src-tauri/Cargo.toml`
- `package.json`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core validation and storage infrastructure that ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

### Backend Foundation

- [x] T004 [P] Create ImageFormat enum in src-tauri/src/media/domain/image_validation.rs
- [x] T005 [P] Create FileSize value object with 50MB validation in src-tauri/src/media/domain/image_validation.rs
- [x] T006 [P] Create ModelImagePath value object in src-tauri/src/media/domain/image_validation.rs
- [x] T007 [P] Define ValidationError enum with thiserror in src-tauri/src/media/domain/image_validation.rs
- [x] T008 [P] Define StorageError enum with thiserror in src-tauri/src/media/domain/image_validation.rs
- [x] T009 Create ImageValidator domain service in src-tauri/src/media/domain/image_validation.rs
- [x] T010 Implement validate_format using image crate (magic bytes) in ImageValidator
- [x] T011 Implement validate_size using fs::metadata in ImageValidator
- [x] T012 Write unit tests for ImageValidator (format detection, size limits, corrupted files)
- [x] T013 Create FileStorage infrastructure in src-tauri/src/media/infrastructure/file_storage.rs
- [x] T014 Implement storage directory initialization with create_dir_all in FileStorage
- [x] T015 Implement copy_image method using tokio::fs::copy in FileStorage
- [x] T016 Implement write_image method using tokio::fs::write in FileStorage
- [x] T017 Implement delete_image method using tokio::fs::remove_file in FileStorage
- [x] T018 Write unit tests for FileStorage (directory creation, file operations, error handling)
- [x] T019 Update src-tauri/src/media/domain/mod.rs to export validation types
- [x] T020 Update src-tauri/src/media/infrastructure/mod.rs to export FileStorage
- [x] T020b Implement AppData directory writability check in FileStorage::new() before file operations (FR-020)

**File Paths**:

- `src-tauri/src/media/domain/image_validation.rs`
- `src-tauri/src/media/infrastructure/file_storage.rs`
- `src-tauri/src/media/domain/mod.rs`
- `src-tauri/src/media/infrastructure/mod.rs`

**Checkpoint**: Foundation ready - all validation and storage infrastructure is in place. User story implementation can now begin.

---

## Phase 3: User Story 1 - Add Primary Model Image via File Explorer (Priority: P1) 🎯 MVP

**Goal**: Enable users to select an image via OS file dialog and upload it to a model

**Independent Test**: Open model details page → click upload button → select JPEG/PNG/WEBP → image displays and persists after restart

### Backend Implementation for US1

- [x] T021 [P] [US1] Create UploadModelImageArgs DTO in src-tauri/src/media/interface/commands.rs
- [x] T022 [P] [US1] Create UploadImageInput use case input in src-tauri/src/media/application/upload_model_image.rs
- [x] T023 [US1] Implement UploadModelImage use case in src-tauri/src/media/application/upload_model_image.rs
- [x] T024 [US1] Add model existence validation in upload use case (query railway_models table)
- [x] T025 [US1] Add file validation step (call ImageValidator) in upload use case
- [x] T026 [US1] Add destination path computation with sanitization (: → \_) in upload use case (no database update needed)
- [x] T027 [US1] Add old image deletion logic (if exists) in upload use case
- [x] T028 [US1] Add file copy operation (FileStorage::copy_image) in upload use case
- [x] T029 [US1] Write unit tests for UploadModelImage use case (happy path, validation failures, storage errors)
- [x] T030 [US1] Create upload_model_image Tauri command in src-tauri/src/media/interface/commands.rs
- [x] T031 [US1] Add argument validation (args.validate()) in command handler
- [x] T032 [US1] Map ValidationError and StorageError to CommandError in command handler
- [x] T033 [US1] Register upload_model_image command in src-tauri/src/lib.rs
- [x] T034 [US1] Add #[specta::specta] macro to command for type generation
- [x] T035 [US1] Update src-tauri/src/media/application/mod.rs to export use case
- [x] T036 [US1] Update src-tauri/src/media/interface/mod.rs to export command

**File Paths**:

- `src-tauri/src/media/application/upload_model_image.rs`
- `src-tauri/src/media/interface/commands.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/src/media/application/mod.rs`
- `src-tauri/src/media/interface/mod.rs`

### Frontend Implementation for US1

- [x] T037 [P] [US1] Generate TypeScript bindings by running pnpm run generate:types
- [x] T038 [US1] Create ImageUpload.svelte component in src/lib/components/model-details/ImageUpload.svelte
- [x] T039 [US1] Add upload button with click handler in ImageUpload component
- [x] T040 [US1] Implement file selection using @tauri-apps/plugin-dialog open() method
- [x] T041 [US1] Add file filter for images (jpg, jpeg, png, webp) in dialog configuration
- [x] T042 [US1] Add isUploading state with $state rune in ImageUpload component
- [x] T043 [US1] Add error state with $state rune for error messages
- [x] T044 [US1] Call commands.uploadModelImage with modelId and filePath
- [x] T045 [US1] Add loading indicator (spinner or disabled button) during upload
- [x] T046 [US1] Add error message display using Alert component from shadcn-svelte
- [x] T047 [US1] Add success feedback and trigger image refresh after upload
- [x] T048 [US1] Handle CommandError mapping (ValidationError, NotFound, InfrastructureError)
- [x] T049 [US1] Integrate ImageUpload component into src/routes/models/[modelId]/+page.svelte
- [x] T050 [US1] Position upload button near model image display area
- [x] T051 [US1] Add conditional rendering (show upload button if no image, or show replace option)

**File Paths**:

- `src/lib/components/model-details/ImageUpload.svelte`
- `src/routes/models/[modelId]/+page.svelte`
- `src/lib/bindings.ts` (auto-generated)

### Localization for US1

- [x] T052 [P] [US1] Add upload_image message to messages/en.json
- [x] T053 [P] [US1] Add uploading message to messages/en.json
- [x] T054 [P] [US1] Add upload_success message to messages/en.json
- [x] T055 [P] [US1] Add upload*error*\* messages (unsupported_format, file_too_large, unknown) to messages/en.json
- [x] T056 [P] [US1] Add Italian translations for all upload messages to messages/it.json

**File Paths**:

- `messages/en.json`
- `messages/it.json`

**Checkpoint**: User Story 1 complete - users can upload images via file explorer, images display and persist

---

## Phase 4: User Story 2 - Add Model Image via Drag & Drop (Priority: P2)

**Goal**: Enable users to drag & drop images from desktop onto the model details page

**Independent Test**: Open model details page → drag image from desktop → drop on zone → image displays

### Backend Implementation for US2

- [x] T057 [P] [US2] Create UploadModelImageBytesArgs DTO in src-tauri/src/media/interface/commands.rs
- [x] T058 [P] [US2] Create UploadImageBytesInput use case input in src-tauri/src/media/application/upload_model_image.rs
- [x] T059 [US2] Implement UploadModelImageBytes use case in src-tauri/src/media/application/upload_model_image.rs
- [x] T060 [US2] Add temporary file creation in OS temp dir in use case
- [x] T061 [US2] Add file data write to temporary file in use case
- [x] T062 [US2] Add validation step (reuse ImageValidator) on temporary file
- [x] T063 [US2] Add destination path resolution (same logic as US1)
- [x] T064 [US2] Add old image deletion (same logic as US1)
- [x] T065 [US2] Add file move from temp to destination
- [x] T066 [US2] Add temporary file cleanup (even on error)
- [x] T067 [US2] Write unit tests for UploadModelImageBytes use case
- [x] T068 [US2] Create upload_model_image_bytes Tauri command in src-tauri/src/media/interface/commands.rs
- [x] T069 [US2] Add validation and error mapping in command handler
- [x] T070 [US2] Register upload_model_image_bytes command in src-tauri/src/lib.rs

**File Paths**:

- `src-tauri/src/media/application/upload_model_image.rs`
- `src-tauri/src/media/interface/commands.rs`
- `src-tauri/src/lib.rs`

### Frontend Implementation for US2

- [x] T071 [US2] Generate TypeScript bindings (pnpm run generate:types)
- [x] T072 [US2] Create ImageDropZone.svelte component in src/lib/components/model-details/ImageDropZone.svelte
- [x] T073 [US2] Add isDragging state with $state rune
- [x] T074 [US2] Implement ondragover handler with preventDefault
- [x] T075 [US2] Implement ondragleave handler to clear isDragging state
- [x] T076 [US2] Implement ondrop handler to extract dropped files
- [x] T077 [US2] Add file validation (check files.length === 1, check MIME type hint)
- [x] T078 [US2] Read file as ArrayBuffer using file.arrayBuffer()
- [x] T079 [US2] Convert ArrayBuffer to number[] for Tauri command
- [x] T080 [US2] Call commands.uploadModelImageBytes with modelId, fileName, fileData
- [x] T081 [US2] Add visual feedback for drag states (border, background color change)
- [x] T082 [US2] Add drop zone styling with Tailwind classes
- [x] T083 [US2] Add loading state during upload
- [x] T084 [US2] Add error handling and display
- [x] T085 [US2] Integrate ImageDropZone component into src/routes/models/[modelId]/+page.svelte
- [x] T086 [US2] Position drop zone near or combined with upload button

**File Paths**:

- `src/lib/components/model-details/ImageDropZone.svelte`
- `src/routes/models/[modelId]/+page.svelte`

### Localization for US2

- [x] T087 [P] [US2] Add drag_and_drop_hint message to messages/en.json
- [x] T088 [P] [US2] Add drop_image_here message to messages/en.json
- [x] T089 [P] [US2] Add Italian translations to messages/it.json

**File Paths**:

- `messages/en.json`
- `messages/it.json`

**Checkpoint**: User Stories 1 AND 2 complete - users can upload via file explorer OR drag & drop

---

## Phase 5: User Story 3 - Reject Invalid File Formats (Priority: P2)

**Goal**: Validate file formats and provide clear error messages for unsupported formats

**Independent Test**: Attempt to upload TIFF, BMP, PDF, TXT files → verify each is rejected with clear error message

**Note**: Most validation logic already implemented in Phase 2 (ImageValidator). This phase focuses on comprehensive testing and error message refinement.

### Enhancement & Testing for US3

- [x] T090 [P] [US3] Add comprehensive format validation tests in ImageValidator tests (TIFF, BMP, RAW, PDF, TXT)
- [x] T091 [P] [US3] Test corrupted image file rejection in ImageValidator tests
- [x] T092 [P] [US3] Test multiple file drop rejection in ImageDropZone component
- [x] T093 [US3] Verify file dialog filter works correctly (only shows JPEG, PNG, WEBP)
- [x] T094 [US3] Add error message refinement - list supported formats clearly
- [x] T095 [US3] Add error message for corrupted files - user-friendly explanation
- [x] T096 [US3] Add error message for multiple files - "only one image at a time"
- [x] T097 [US3] Verify frontend MIME type hint check in drag & drop (pre-validation before upload)

**File Paths**:

- `src-tauri/src/media/domain/image_validation.rs` (tests)
- `src/lib/components/model-details/ImageDropZone.svelte`
- `src/lib/components/model-details/ImageUpload.svelte`
- `messages/en.json`
- `messages/it.json`

### Additional Error Messages

- [x] T098 [P] [US3] Add upload_error_corrupted message to messages/en.json
- [x] T099 [P] [US3] Add upload_error_multiple_files message to messages/en.json
- [x] T100 [P] [US3] Add Italian translations to messages/it.json

**File Paths**:

- `messages/en.json`
- `messages/it.json`

### Edge Case Testing for US3

- [x] T101 [US3] Test special characters in filenames (spaces, unicode, emojis) are sanitized correctly
- [ ] T102 [US3] Test unusual aspect ratios (10000x100 pixels, 100x10000 pixels) are handled gracefully
- [x] T103 [US3] Test corrupted image files (truncated, invalid headers) are rejected with clear errors
- [x] T104 [US3] Test extremely large valid files (49MB, 50MB, 51MB) respect size limit
- [x] T105 [US3] Test filename collision scenario (same filename for different models) is handled by deterministic naming

**Checkpoint**: Robust validation in place - all invalid formats rejected with clear, helpful error messages, edge cases handled

---

## Phase 6: User Story 4 - Replace Existing Model Image (Priority: P3)

**Goal**: Allow users to replace existing images, with automatic cleanup of old files

**Independent Test**: Upload image → upload different image → verify old image deleted, new image displays

**Note**: Replacement logic already implemented in US1/US2 (old image deletion step). This phase focuses on testing and UX refinement.

### Testing & UX Enhancement for US4

- [x] T106 [US4] Test image replacement flow - verify old file deleted from filesystem
- [x] T107 [US4] Add integration test for replacement scenario (upload → replace → verify cleanup)
- [x] T108 [US4] Verify no orphaned files remain after multiple replacements
- [ ] T109 [US4] Add visual confirmation when replacing (optional: "Replace existing image?" confirmation)
- [x] T110 [US4] Update button label to "Replace Image" when image exists
- [x] T111 [US4] Test replacement with different format (JPEG → PNG, PNG → WEBP)
- [x] T112 [US4] Verify destination path changes extension based on new format

**File Paths**:

- `src/lib/components/model-details/ImageUpload.svelte`
- `src/tauri/src/media/application/upload_model_image.rs`

### Localization for US4

- [x] T113 [P] [US4] Add replace_image message to messages/en.json
- [ ] T114 [P] [US4] Add replace_image_confirm message (if confirmation added) to messages/en.json
- [x] T115 [P] [US4] Add Italian translations to messages/it.json

**File Paths**:

- `messages/en.json`
- `messages/it.json`

**Checkpoint**: Image replacement works seamlessly with automatic cleanup

---

## Phase 7: User Story 5 - Delete Model Image (Priority: P3)

**Goal**: Allow users to explicitly delete model images with confirmation

**Independent Test**: Upload image → click delete button → confirm → verify image removed from page and filesystem

### Backend Implementation for US5

- [x] T111 [P] [US5] Create DeleteModelImageArgs DTO in src-tauri/src/media/interface/commands.rs
- [x] T112 [US5] Implement DeleteModelImage use case in src-tauri/src/media/application/delete_model_image.rs
- [x] T113 [US5] Add model existence validation in use case
- [x] T114 [US5] Add image path resolution (all supported extensions)
- [x] T115 [US5] Add image deletion (FileStorage::delete_image) - idempotent (no error if not exists)
- [x] T116 [US5] Write unit tests for DeleteModelImage use case
- [x] T117 [US5] Create delete_model_image Tauri command in src-tauri/src/media/interface/commands.rs
- [x] T118 [US5] Add validation and error mapping in command handler
- [x] T119 [US5] Register delete_model_image command in src-tauri/src/lib.rs
- [x] T120 [US5] Update src-tauri/src/media/application/mod.rs to export use case

**File Paths**:

- `src-tauri/src/media/application/delete_model_image.rs`
- `src-tauri/src/media/interface/commands.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/src/media/application/mod.rs`

### Frontend Implementation for US5

- [x] T126 [US5] Generate TypeScript bindings (pnpm run generate:types)
- [x] T127 [US5] Add delete button to ImageUpload component (conditional: only show if image exists)
- [x] T128 [US5] Add confirmation dialog using AlertDialog from shadcn-svelte
- [x] T129 [US5] Implement delete handler calling commands.deleteModelImage
- [x] T130 [US5] Add isDeleting state for loading indicator
- [x] T131 [US5] Update image display after successful deletion (clear image, show upload button)
- [x] T132 [US5] Add error handling for delete operation
- [x] T133 [US5] Style delete button with destructive variant (red/warning color)

**File Paths**:

- `src/lib/components/model-details/ImageUpload.svelte`
- `src/routes/models/[modelId]/+page.svelte`

### Localization for US5

- [x] T134 [P] [US5] Add delete_image message to messages/en.json
- [x] T135 [P] [US5] Add deleting message to messages/en.json
- [x] T136 [P] [US5] Add confirm_delete_image_title message to messages/en.json
- [x] T137 [P] [US5] Add confirm_delete_image_description message to messages/en.json
- [x] T138 [P] [US5] Add image_deleted message (success) to messages/en.json
- [x] T139 [P] [US5] Add Italian translations to messages/it.json

**File Paths**:

- `messages/en.json`
- `messages/it.json`

**Checkpoint**: All user stories complete - full upload, replace, and delete functionality working

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Code quality, documentation, and final testing

### Code Quality

- [x] T140 [P] Run cargo fmt on all Rust files (src-tauri/src/media/\*\*)
- [x] T141 [P] Run cargo clippy -D warnings and fix all warnings
- [x] T142 [P] Run cargo test to verify all unit tests pass
- [x] T143 [P] Run pnpm format to format all frontend files
- [x] T144 [P] Run pnpm lint and fix all linting errors
- [x] T145 [P] Run pnpm check to verify TypeScript types
- [x] T146 [P] Verify no TypeScript errors in bindings.ts

**Terminal Commands**:

```bash
cd src-tauri && cargo fmt
cd src-tauri && cargo clippy -D warnings
cd src-tauri && cargo test
pnpm format
pnpm lint
pnpm check
```

### Manual Testing

- [ ] T147 Test complete upload flow (file explorer) with JPEG, PNG, WEBP
- [ ] T148 Test complete drag & drop flow with various image formats
- [ ] T149 Test invalid format rejection (PDF, TIFF, BMP, TXT)
- [ ] T150 Test file size limit (try uploading >50MB file)
- [ ] T151 Test image replacement (upload → replace → verify old deleted)
- [ ] T152 Test image deletion (upload → delete → verify removed)
- [ ] T153 Test persistence (upload → close app → reopen → verify image still there)
- [ ] T154 Test error scenarios (disk full simulation, permission denied simulation)
- [ ] T155 Test drag & drop visual feedback (hover states, loading states)
- [ ] T156 Test localization (switch to Italian, verify all messages translated)

### Documentation

- [x] T157 [P] Update CHANGELOG.md with feature description
- [x] T158 [P] Verify quickstart.md is accurate for current implementation
- [x] T159 [P] Add inline documentation comments to complex validation logic

**File Paths**:

- `CHANGELOG.md`
- `specs/015-model-image-upload/quickstart.md`
- Various source files (inline comments)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - start immediately
- **Foundational (Phase 2)**: Depends on Setup - BLOCKS all user stories
- **User Stories (Phase 3-7)**: All depend on Foundational phase completion
  - US1 (P1): Can start after Foundational ✅ MVP
  - US2 (P2): Can start after Foundational (no dependency on US1 code, but builds on concept)
  - US3 (P2): Enhances validation from Foundational (mostly testing)
  - US4 (P3): Builds on US1/US2 (replacement logic already exists)
  - US5 (P3): Independent delete functionality
- **Polish (Phase 8)**: Depends on all desired user stories being complete

### Parallel Opportunities

**Within Foundational Phase**:

- T004-T008 (domain types) can run in parallel
- T013-T018 (FileStorage) can run after domain types
- Tests (T012, T018) can run in parallel after implementation

**Across User Stories (if team capacity allows)**:

- Once Foundational complete, US1, US2, US5 can start in parallel
- US3 is mostly testing, can run concurrently
- US4 depends on US1/US2 code existing but minimal coordination needed

**Within Each User Story**:

- Backend tasks that touch different files can run in parallel
- Frontend component creation can happen in parallel with backend
- Localization messages (marked [P]) can run in parallel
- Tests can be written in parallel with implementation

**Polish Phase**:

- All code quality tasks (T135-T141) can run in parallel
- Documentation tasks (T152-T154) can run in parallel

---

## Parallel Example: Foundational Phase

```bash
# Terminal 1: Domain types
code src-tauri/src/media/domain/image_validation.rs
# Create ImageFormat, FileSize, ModelImagePath, Errors, ImageValidator

# Terminal 2: Infrastructure
code src-tauri/src/media/infrastructure/file_storage.rs
# Create FileStorage with file operations

# Terminal 3: Tests (after implementation)
cargo test --package rusty-shed --lib media::domain::image_validation::tests
cargo test --package rusty-shed --lib media::infrastructure::file_storage::tests
```

---

## Parallel Example: User Story 1

```bash
# Terminal 1: Backend use case
code src-tauri/src/media/application/upload_model_image.rs

# Terminal 2: Backend command (can start after use case structure exists)
code src-tauri/src/media/interface/commands.rs

# Terminal 3: Frontend component (can start in parallel with backend)
code src/lib/components/model-details/ImageUpload.svelte

# Terminal 4: Localization (completely parallel)
code messages/en.json messages/it.json
```

---

## Implementation Strategy

### Recommended Sequence

1. **Phase 1**: Setup (15 minutes)
2. **Phase 2**: Foundational - Build all validation and storage (2-3 hours)
3. **Phase 3**: User Story 1 (P1) - Core upload via file explorer (3-4 hours) → **MVP COMPLETE**
4. **Phase 4**: User Story 2 (P2) - Add drag & drop (2-3 hours)
5. **Phase 5**: User Story 3 (P2) - Validation testing (1 hour)
6. **Phase 6**: User Story 4 (P3) - Replacement testing (1 hour)
7. **Phase 7**: User Story 5 (P3) - Delete functionality (1-2 hours)
8. **Phase 8**: Polish & Testing (2-3 hours)

**Total Estimate**: 12-18 hours

### MVP Definition

**Minimum Viable Product = Phase 1 + Phase 2 + Phase 3 (US1)**

This delivers:

- ✅ File explorer image selection
- ✅ Format validation (JPEG, PNG, WEBP)
- ✅ File size validation (50MB limit)
- ✅ Storage in AppData with deterministic naming
- ✅ Image display with persistence
- ✅ Error handling with user-friendly messages

Users can add images to models. Drag & drop, delete, and advanced features are enhancements.

---

## Summary

- **Total Tasks**: 159
- **Phases**: 8 (Setup, Foundational, 5 User Stories, Polish)
- **User Stories**: 5 (P1, P2, P2, P3, P3)
- **Backend Tasks**: ~70 (validation, use cases, commands, tests)
- **Frontend Tasks**: ~60 (components, integration, UX)
- **Localization Tasks**: ~20 (en + it messages)
- **Quality Tasks**: ~4 (formatting, linting, docs)
- **Edge Case Tests**: 5 (special chars, aspect ratios, corruption, size limits, collisions)
- **Parallel Tasks**: ~40 (marked with [P])

**MVP Scope**: Phase 1 + Phase 2 + Phase 3 (US1) = ~50 tasks, 6-8 hours

**Full Feature**: All phases = ~159 tasks, 12-18 hours

---

## Task Execution Tips

1. **Start with foundational phase** - all user stories depend on it
2. **Implement US1 first** - it's the MVP and validates the entire flow
3. **Run tests frequently** - catch issues early
4. **Generate bindings after each backend change** - keep TypeScript types in sync
5. **Test manually after each user story** - verify end-to-end functionality
6. **Use parallel opportunities** - marked tasks can run simultaneously
7. **Follow code quality checks** - run fmt, clippy, lint after each phase

---

**Ready to implement!** Start with Phase 1 (Setup) → Phase 2 (Foundational) → Phase 3 (US1 MVP)
