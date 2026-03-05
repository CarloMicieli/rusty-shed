# Quickstart: Drop-to-Crop Implementation Guide

**Feature**: 035-drop-to-crop
**Date**: 2026-03-05

---

## Prerequisites

Before implementing, confirm the following with the user:

1. **Approve `cropperjs` dependency**: Per `CLAUDE.md`, new dependencies require explicit user approval.
   Run: `pnpm add cropperjs` and `pnpm add -D @types/cropperjs` (if types not bundled).

2. **Branch**: You should be on `035-drop-to-crop`.

---

## Implementation Order

Follow this sequence to avoid broken intermediate states:

### Step 1 — Add i18n message keys

Add to `messages/en.json`:
```json
"drop_here_to_update_photo": "Drop here to update photo",
"crop_dialog_title": "Crop Image",
"crop_confirm": "Apply Crop",
"crop_cancel": "Cancel"
```

Add equivalent translations to `messages/it.json`. Run `pnpm prepare` to regenerate Paraglide bindings.

### Step 2 — Create `ImageCropDialog.svelte`

Location: `src/lib/components/model-details/ImageCropDialog.svelte`

**Cropper.js v2 notes** (breaking changes from v1):
- Import: `import Cropper from 'cropperjs'` — same, but **no CSS import** (v2 uses Shadow DOM)
- Constructor: `new Cropper(imageEl)` — no options object
- Options set post-construction on element refs:
  ```ts
  const sel = cropperInstance.getCropperSelection()!;
  sel.aspectRatio = NaN;        // free crop
  sel.initialCoverage = 0.8;
  sel.movable = true;
  sel.resizable = true;
  ```
- Ready detection: `await cropperInstance.getCropperImage()!.$ready()` (Promise, not callback)
- Crop output: `const canvas = await sel.$toCanvas({ width: 2048, height: 2048 })` (async, v2 API)

Responsibilities:
- Accept `{ open, imageSrc, fileName, modelId, onSaveSuccess?, onCancel? }` props (see `contracts/component-interfaces.ts`).
- Wrap a shadcn-svelte `Dialog` component for the modal shell.
- Mount Cropper.js v2 on the `<img>` element inside a fixed-height container (`max-h-[70vh]`).
- On confirm:
  1. `const sel = cropperInstance.getCropperSelection()!`
  2. `const canvas = await sel.$toCanvas({ width: 2048, height: 2048 })`
  3. Convert to bytes: `canvas.toBlob() → arrayBuffer() → Array.from(new Uint8Array(...))`
  4. Call `commands.uploadModelImageBytes({ modelId, fileName: normalizeToJpg(fileName), fileData })`
  5. Handle success/error states with loading spinner
  6. On success: call `onSaveSuccess?.()`, revoke blob URL if applicable, close dialog
- On cancel: call `onCancel?.()`, revoke blob URL if applicable.
- Revoke blob URLs via `if (imageSrc.startsWith('blob:')) URL.revokeObjectURL(imageSrc)`.
- Import all user-facing strings from Paraglide messages.

### Step 3 — Modify `ImageDropZone.svelte`

Changes:
1. Replace `isDragging` boolean state with `dragCounter` counter + `$derived(dragCounter > 0)`.
2. Add `ondragenter` handler (increment counter).
3. Change `ondragleave` to decrement counter (not `relatedTarget` check).
4. Add `pendingFile = $state<File | null>(null)` state.
5. In `handleDrop`: after validation passes, instead of calling `uploadModelImageBytes`:
   - Set `pendingFile = file`
   - Remove the `isUploading`, `showSuccess`, and upload block entirely from this component
6. Add `<ImageCropDialog>` to template:
   ```svelte
   <ImageCropDialog
     open={pendingFile !== null}
     imageSrc={pendingFile ? URL.createObjectURL(pendingFile) : ''}
     fileName={pendingFile?.name ?? 'image.jpg'}
     {modelId}
     onSaveSuccess={() => { pendingFile = null; onUploadSuccess?.(); }}
     onCancel={() => { pendingFile = null; }}
   />
   ```
   **Note**: `URL.createObjectURL` should be called lazily (only when `pendingFile` is set) to avoid creating and immediately discarding blob URLs. Use a `$derived` for `blobSrc`.
7. Update drag-over class bindings to use array syntax with full Tailwind class strings.
8. Remove unused imports (`commands`, `upload_success`, `uploading`, `upload_error_multiple_files` usage in template).

### Step 4 — Modify `ImageUpload.svelte`

Changes:
1. Add `pendingFilePath = $state<string | null>(null)`.
2. In `handleUpload`: after `open()` returns a non-null path, instead of calling `uploadModelImage`:
   - Import `convertFileSrc` from `@tauri-apps/api/core`
   - Set `pendingFilePath = file` (the returned path)
   - Remove the `isUploading` block from this path
3. Add `<ImageCropDialog>` to template:
   ```svelte
   <ImageCropDialog
     open={pendingFilePath !== null}
     imageSrc={pendingFilePath ? convertFileSrc(pendingFilePath) : ''}
     fileName={pendingFilePath ? pendingFilePath.split('/').pop() ?? 'image.jpg' : 'image.jpg'}
     {modelId}
     onSaveSuccess={() => { pendingFilePath = null; onUploadSuccess?.(); }}
     onCancel={() => { pendingFilePath = null; }}
   />
   ```
4. Keep the delete flow entirely unchanged.
5. Remove `uploadModelImage` from imports if it is no longer called directly from this component.

### Step 5 — Write / Update Tests

**New**: `src/lib/components/model-details/__tests__/ImageCropDialog.test.ts`
- Test: cancel closes dialog without saving
- Test: confirm triggers `uploadModelImageBytes` with correct `modelId`
- Test: confirm calls `onSaveSuccess` on success
- Test: confirm shows error on backend failure
- Test: blob URL is revoked on close

**Modify**: `src/lib/components/model-details/__tests__/ImageDropZone.test.ts`
- Update mocks to include `ImageCropDialog` (mock the module)
- Verify: valid drop opens crop dialog (not calls `uploadModelImageBytes` directly)
- Verify: invalid file still shows error without opening dialog
- Existing MIME type tests remain valid; update assertions where needed

**Modify**: `src/lib/components/model-details/__tests__/ImageUpload.test.ts`
- Update mocks to include `ImageCropDialog`
- Verify: file selection opens crop dialog (not calls `uploadModelImage` directly)
- Delete tests remain unchanged

### Step 6 — Verify

```bash
pnpm prepare          # Regenerate Paraglide messages
pnpm fmt              # Format
pnpm lint             # ESLint
pnpm check            # svelte-check + TypeScript
pnpm test:unit        # Vitest
```

---

## Cropper.js v2 — No CSS Import Needed

Cropper.js v2 uses Shadow DOM for all its styles. There is **no** `cropperjs/dist/cropper.css` file to import. Do not add any CSS import for cropperjs — it will cause a build error in v2.

---

## Tailwind Drag-Over Classes

Use full class name strings in array syntax (not interpolation):

```svelte
class={[
  'relative flex min-h-[200px] flex-col items-center justify-center',
  'rounded-lg border-2 border-dashed p-6 text-center',
  'transition-colors duration-200 ease-in-out',
  isDragging
    ? 'border-primary bg-primary/10 ring-2 ring-primary ring-inset cursor-copy'
    : 'border-muted-foreground/25 hover:border-muted-foreground/50 cursor-pointer'
]}
```

The `border-primary` class resolves to `var(--primary)` = `hsl(30 50% 50%)` (copper tone) via the `@theme inline` block in `layout.css` — no hardcoded hex required.

---

## Important: No Backend Changes

- Do **not** modify any Rust files.
- Do **not** run `pnpm tauri dev` for binding regeneration — no new commands.
- The only Tauri command used for saving is the unchanged `uploadModelImageBytes`.
