# Tasks: Drop-to-Crop Railway Model Image Workflow

**Input**: Design documents from `/specs/035-drop-to-crop/`
**Prerequisites**: plan.md ✓, spec.md ✓, research.md ✓, data-model.md ✓, contracts/ ✓, quickstart.md ✓

**Tests**: Included — existing test suites must be updated; new suite required for `ImageCropDialog`.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies on incomplete tasks)
- **[Story]**: Which user story this task belongs to (US1, US2, US3)

---

## Phase 1: Setup (Dependencies & Messages)

**Purpose**: Install the new dependency and add i18n keys before any component work begins.

- [ ] T001 Obtain user approval and add `cropperjs` to `package.json` via `pnpm add cropperjs` (required — CLAUDE.md forbids adding deps without explicit approval)
- [ ] T002 Add 4 new message keys to `messages/en.json`: `drop_here_to_update_photo`, `crop_dialog_title`, `crop_confirm`, `crop_cancel` (see plan.md §New i18n Keys for English values)
- [ ] T003 [P] Add Italian translations for the 4 new keys to `messages/it.json` (values in plan.md §New i18n Keys)
- [ ] T004 Run `pnpm prepare` to regenerate Paraglide JS bindings in `src/lib/paraglide/`
- [ ] T005 Verify `svelte-sonner` `<Toaster>` is present in the root layout (`src/routes/+layout.svelte`); add it if missing, positioned after page content

**Checkpoint**: `pnpm prepare` succeeds, new message functions importable from `$lib/paraglide/messages.js`

---

## Phase 2: Foundational (ImageCropDialog — Blocks US1 and US2)

**Purpose**: The `ImageCropDialog` component is the core new building block. Both the drag-and-drop path (US1) and the browse path (US2) depend on it. It must be complete and tested before either story's integration work begins.

**⚠️ CRITICAL**: US1 and US2 cannot be wired up until this phase is complete.

- [ ] T006 Create `src/lib/components/model-details/ImageCropDialog.svelte` with shadcn-svelte `Dialog` shell, accepting props: `open: boolean`, `imageSrc: string`, `fileName: string`, `modelId: RailwayModelId`, `onSaveSuccess?: () => void`, `onCancel?: () => void`
- [ ] T007 Add Cropper.js v2 initialization to `src/lib/components/model-details/ImageCropDialog.svelte` using `bind:this={imageEl}` on the `<img>` element; mount in `onMount` via `new Cropper(imageEl)` (no options in constructor); post-construction set `sel.aspectRatio = NaN`, `sel.initialCoverage = 0.8`, `sel.movable = true`, `sel.resizable = true` on `getCropperSelection()`; destroy in `onMount` cleanup return; **no CSS import** (v2 uses Shadow DOM)
- [ ] T008 Add `isReady = $state(false)` and `isSaving = $state(false)` to `src/lib/components/model-details/ImageCropDialog.svelte`; set `isReady = true` after `await cropperInstance.getCropperImage()!.$ready()` (v2 Promise-based, not a constructor callback); disable confirm button while `!isReady || isSaving`
- [ ] T009 Implement confirm handler in `src/lib/components/model-details/ImageCropDialog.svelte`: get selection via `cropperInstance.getCropperSelection()!`, call `const canvas = await sel.$toCanvas({ width: 2048, height: 2048 })` (v2 async API — no `getCroppedCanvas`), convert via `canvas.toBlob() → arrayBuffer() → Array.from(new Uint8Array(...))`, call `commands.uploadModelImageBytes({ modelId, fileName: normalizedJpgName, fileData })`
- [ ] T010 Implement success/error/cancel flows in `src/lib/components/model-details/ImageCropDialog.svelte`: on success call `onSaveSuccess?.()` + close; on error display in dialog footer; on cancel call `onCancel?.()` + close; on any close revoke blob URL if `imageSrc.startsWith('blob:')`
- [ ] T011 Import and use Paraglide strings in `src/lib/components/model-details/ImageCropDialog.svelte`: `crop_dialog_title`, `crop_confirm`, `crop_cancel`, `uploading` (reuse existing key for the saving spinner)
- [ ] T012 Import `Cropper` from `'cropperjs'` in `src/lib/components/model-details/ImageCropDialog.svelte` script block — **no CSS import** (v2 uses Shadow DOM; importing `cropperjs/dist/cropper.css` would fail)
- [ ] T013 Create `src/lib/components/model-details/__tests__/ImageCropDialog.test.ts` with the following tests: cancel closes dialog without calling `uploadModelImageBytes`; confirm calls `uploadModelImageBytes` with correct `modelId` and `fileData`; confirm calls `onSaveSuccess` on backend success; confirm displays error message on backend failure; blob URL is revoked on cancel

**Checkpoint**: `pnpm test:unit` passes for `ImageCropDialog.test.ts` — crop dialog works in isolation

---

## Phase 3: User Story 1 — Drag Image onto Model Photo Zone (Priority: P1) 🎯 MVP

**Goal**: Drop a valid image file onto the model photo zone → crop dialog opens automatically → save cropped image.

**Independent Test**: Drag a `.jpg` from the OS onto the model photo zone, confirm crop, verify the model's photo updates. Can be verified without the browse button working.

### Implementation for User Story 1

- [ ] T014 [US1] Replace `isDragging = $state(false)` with `dragCounter = $state(0)` and `const isDragging = $derived(dragCounter > 0)` in `src/lib/components/model-details/ImageDropZone.svelte`
- [ ] T015 [US1] Add `ondragenter` handler (increment `dragCounter`) to `src/lib/components/model-details/ImageDropZone.svelte`; replace existing `relatedTarget`-based `ondragleave` handler with counter decrement (`dragCounter--`); reset `dragCounter = 0` at the start of `handleDrop`
- [ ] T016 [US1] Add `pendingFile = $state<File | null>(null)` and `blobUrl = $state<string | null>(null)` to `src/lib/components/model-details/ImageDropZone.svelte`
- [ ] T017 [US1] Update `handleDrop` in `src/lib/components/model-details/ImageDropZone.svelte`: after MIME validation passes, set `blobUrl = URL.createObjectURL(file)` and `pendingFile = file`; remove the `isUploading`, `arrayBuffer`, and `uploadModelImageBytes` call entirely; keep the MIME rejection logic but replace inline `error =` assignment with `toast.error(error_invalid_image_format())` via svelte-sonner
- [ ] T018 [US1] Update drag-over Tailwind class bindings in `src/lib/components/model-details/ImageDropZone.svelte` to use array syntax: idle state `'border-muted-foreground/25 hover:border-muted-foreground/50'`, dragging state `'border-primary bg-primary/10 ring-2 ring-primary ring-inset cursor-copy'`, add `'transition-colors duration-200 ease-in-out'` to the static base classes
- [ ] T019 [US1] Update dragging overlay text in `src/lib/components/model-details/ImageDropZone.svelte` to use `drop_here_to_update_photo()` Paraglide key instead of `drop_image_here()`
- [ ] T020 [US1] Add `<ImageCropDialog>` to `src/lib/components/model-details/ImageDropZone.svelte` template: `open={pendingFile !== null}`, `imageSrc={blobUrl ?? ''}`, `fileName={pendingFile?.name ?? 'image.jpg'}`, `{modelId}`, `onSaveSuccess={() => { pendingFile = null; blobUrl = null; onUploadSuccess?.(); }}`, `onCancel={() => { if (blobUrl) URL.revokeObjectURL(blobUrl); pendingFile = null; blobUrl = null; }}`
- [ ] T021 [US1] Remove now-unused state and imports from `src/lib/components/model-details/ImageDropZone.svelte`: `isUploading`, `showSuccess`, the `<Alert>` success/error blocks from template, `commands` import (or reduce to no longer include `uploadModelImageBytes` if still needed), `upload_success`, `uploading`, `upload_error_unknown`, `upload_error_model_not_found` Paraglide imports
- [ ] T022 [US1] Update `src/lib/components/model-details/__tests__/ImageDropZone.test.ts`: mock `ImageCropDialog` module; update "valid single file drop" test to assert `pendingFile` is set and crop dialog opens (not `uploadModelImageBytes` called directly); update MIME rejection tests to assert toast is shown instead of inline Alert; keep drag-state tests intact

**Checkpoint**: Drop a valid image → crop dialog opens. Drop invalid type → toast appears. `pnpm test:unit` green for `ImageDropZone.test.ts`.

---

## Phase 4: User Story 2 — Browse-and-Crop via File Dialog (Priority: P2)

**Goal**: Click "Upload Image" / "Change Image" → file dialog → select file → crop dialog opens → save cropped image. Delete flow unchanged.

**Independent Test**: Click the upload button, select a `.png`, complete the crop, verify the model photo updates. Does not depend on drag-and-drop working.

### Implementation for User Story 2

- [ ] T023 [US2] Add `pendingFilePath = $state<string | null>(null)` to `src/lib/components/model-details/ImageUpload.svelte`
- [ ] T024 [US2] Update `handleUpload` in `src/lib/components/model-details/ImageUpload.svelte`: after `open()` returns a non-null file path, set `pendingFilePath = file` instead of calling `uploadModelImage`; remove the `isUploading` block and `uploadModelImage` call from this handler entirely; import `convertFileSrc` from `@tauri-apps/api/core`
- [ ] T025 [US2] Add `<ImageCropDialog>` to `src/lib/components/model-details/ImageUpload.svelte` template: `open={pendingFilePath !== null}`, `imageSrc={pendingFilePath ? convertFileSrc(pendingFilePath) : ''}`, `fileName={pendingFilePath ? (pendingFilePath.split('/').pop() ?? 'image.jpg') : 'image.jpg'}`, `{modelId}`, `onSaveSuccess={() => { pendingFilePath = null; onUploadSuccess?.(); }}`, `onCancel={() => { pendingFilePath = null; }}`
- [ ] T026 [US2] Remove now-unused state and imports from `src/lib/components/model-details/ImageUpload.svelte`: `isUploading`, `showSuccess` for upload path (keep for delete path if shared), `commands.uploadModelImage` import (keep `commands.deleteModelImage`)
- [ ] T027 [US2] Update `src/lib/components/model-details/__tests__/ImageUpload.test.ts`: mock `ImageCropDialog` module; update "file selection" test to assert `pendingFilePath` is set and crop dialog opens (not `uploadModelImage` called directly); keep all delete flow tests entirely unchanged

**Checkpoint**: Browse → select file → crop dialog opens. Delete still works. `pnpm test:unit` green for `ImageUpload.test.ts`.

---

## Phase 5: User Story 3 — Invalid File Type Rejection (Priority: P3)

**Goal**: Dropping an unsupported file type (`.pdf`, `.tiff`, `.gif`, etc.) onto the photo zone shows a toast and does NOT open the crop dialog.

**Independent Test**: Drop a `.pdf` onto the model photo zone → toast appears → crop dialog does not open → zone returns to idle state.

**Note**: Validation logic and toast call were introduced in T017 (US1). This phase ensures the Toaster is correctly positioned in the app shell and that all rejection edge cases have explicit test coverage.

### Implementation for User Story 3

- [ ] T028 [US3] Verify `<Toaster>` from `svelte-sonner` is rendered in `src/routes/+layout.svelte` with theme-appropriate positioning (e.g., `position="bottom-right"`); add if not present
- [ ] T029 [US3] Add explicit test cases to `src/lib/components/model-details/__tests__/ImageDropZone.test.ts` for: `.pdf` drop shows rejection toast and `pendingFile` remains null; `.tiff` drop shows rejection toast; multi-file drop shows rejection toast; all rejection paths do NOT call `uploadModelImageBytes` and do NOT open crop dialog

**Checkpoint**: All rejection paths trigger a toast. Crop dialog never opens for invalid inputs. `pnpm test:unit` green.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final quality gate across all stories.

- [ ] T030 [P] Run `pnpm prepare` to confirm Paraglide bindings are up to date after all message changes
- [ ] T031 [P] Run `pnpm lint` and resolve any ESLint warnings in modified/new files: `ImageCropDialog.svelte`, `ImageDropZone.svelte`, `ImageUpload.svelte`
- [ ] T032 [P] Run `pnpm check` (svelte-check + TypeScript strict mode) and resolve all type errors
- [ ] T033 Run `pnpm test:unit` for full test suite; confirm zero regressions
- [ ] T034 Manual smoke test in running Tauri app: drag a `.jpg` onto the photo zone, verify drag-over copper border appears, confirm crop dialog opens and save succeeds
- [ ] T035 Manual smoke test in running Tauri app: click "Upload Image", select a `.png`, confirm crop dialog opens and save succeeds; confirm delete flow still works independently

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — start immediately. T002 and T003 can run in parallel.
- **Foundational (Phase 2)**: Depends on Phase 1 complete (Paraglide bindings must exist before importing new message keys in the component). **BLOCKS Phase 3 and Phase 4.**
- **User Story phases (Phase 3, 4, 5)**: All depend on Phase 2 completion.
  - Phase 3 (US1) and Phase 4 (US2) can proceed in parallel once Phase 2 is complete.
  - Phase 5 (US3) depends on Phase 3 being complete (toast call added in T017).
- **Polish (Phase 6)**: Depends on all desired user story phases being complete.

### User Story Dependencies

- **US1 (P1)**: Depends on Phase 2 (ImageCropDialog). No dependency on US2 or US3.
- **US2 (P2)**: Depends on Phase 2 (ImageCropDialog). No dependency on US1. Can be developed in parallel with US1.
- **US3 (P3)**: Depends on US1 Phase 3 completion (toast call is introduced in T017). Thin phase — mainly test coverage verification.

### Within Each User Story

- State changes before handler updates before template wiring (within same file).
- Unused imports/state cleanup after all new behaviour is wired (T021, T026).
- Tests updated after implementation is working (T022, T027, T029).

### Parallel Opportunities

- **T002 + T003**: Both are message file additions, different files — run in parallel.
- **T006 through T013**: All `ImageCropDialog` tasks — sequential within the component but T013 (tests) can be written alongside T006-T012.
- **Phase 3 + Phase 4** (after Phase 2 complete): `ImageDropZone` and `ImageUpload` are separate files — can be worked in parallel.
- **T030 + T031 + T032**: Polish tasks in different tools — run in parallel.

---

## Parallel Example: Phase 2 (Foundational)

```text
Sequential — all modify ImageCropDialog.svelte:
T006 → T007 → T008 → T009 → T010 → T011 → T012

Parallel with T006-T012:
T013 (write test file while implementing the component)
```

## Parallel Example: Phase 3 + Phase 4 (after Phase 2)

```text
Track A — ImageDropZone.svelte:
T014 → T015 → T016 → T017 → T018 → T019 → T020 → T021 → T022

Track B — ImageUpload.svelte (simultaneous):
T023 → T024 → T025 → T026 → T027
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001–T005)
2. Complete Phase 2: Foundational — ImageCropDialog (T006–T013)
3. Complete Phase 3: User Story 1 — Drop path (T014–T022)
4. **STOP and VALIDATE**: Drag a file onto the model photo zone, confirm crop dialog opens, crop and save, verify result.
5. Demo or release this increment — browse and delete still work as before.

### Incremental Delivery

1. Phase 1 + Phase 2 → `ImageCropDialog` exists, fully tested in isolation
2. Phase 3 → Drop-to-crop works end-to-end (**MVP delivered**)
3. Phase 4 → Browse-to-crop also works; both paths consistent
4. Phase 5 → All rejection edge cases covered and tested
5. Phase 6 → Full quality gate passes

### Parallel Team Strategy

With two developers after Phase 2 is complete:
- **Developer A**: Phase 3 (ImageDropZone — US1)
- **Developer B**: Phase 4 (ImageUpload — US2)
Both stories complete independently; Phase 5 and Phase 6 follow.

---

## Notes

- **[P]** tasks touch different files with no shared dependencies — safe to run simultaneously.
- **[Story]** labels map tasks to spec.md user stories for full traceability.
- `cropperjs/dist/cropper.css` must be imported inside the component, not in `app.css`, to avoid polluting global styles.
- The `blobUrl` state in `ImageDropZone` must be created eagerly (in the drop handler) and stored — do **not** use a `$derived` that calls `URL.createObjectURL`, as that would create a new URL on every reactive re-evaluation.
- The `convertFileSrc` import for `ImageUpload` is already available via `@tauri-apps/api/core` (installed); no new package needed.
- No Rust files change. No `pnpm tauri dev` binding regeneration needed.
- Delete flow in `ImageUpload` is entirely untouched — verify via existing tests.
