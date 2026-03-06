# Implementation Plan: Drop-to-Crop Railway Model Image Workflow

**Branch**: `035-drop-to-crop` | **Date**: 2026-03-05 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `specs/035-drop-to-crop/spec.md`

---

## Summary

Introduce a Cropper.js-powered crop step between file acquisition (OS drag-and-drop or file browser) and image persistence. Both entry points converge on a single `ImageCropDialog` component that handles cropping and saving via the existing `uploadModelImageBytes` Tauri command. No backend changes are required. The feature is purely a frontend UX enhancement.

The drag-over visual state uses the app's `--primary` token (copper `hsl(30 50% 50%)`) via Tailwind 4's `@theme inline` mapping, ensuring the highlight is automatically theme-correct without hardcoded colors.

---

## Technical Context

**Language/Version**: TypeScript 5.9.3, Svelte 5.53.6 (Runes only), Rust 1.93+ (unchanged)
**Primary Dependencies**: `cropperjs` v2.1.0 (installed, approved), shadcn-svelte Dialog, Tailwind CSS v4.2, `@tauri-apps/api/core` (convertFileSrc — already installed)
**Storage**: N/A — existing SQLite image persistence via `src-tauri/src/media/` is unchanged
**Testing**: Vitest 4.0 + `@testing-library/svelte` + happy-dom environment
**Target Platform**: Tauri 2.0 WebView (Linux primary, macOS/Windows secondary)
**Performance Goals**: Crop dialog opens < 1s for images up to 20 MB; drag-over transition < 200ms
**Constraints**: No hardcoded strings (Paraglide); no new Rust commands; no schema migrations; `cropperjs` dependency requires explicit user approval before `pnpm add`
**Scale/Scope**: 3 Svelte components (1 new, 2 modified); 3 test files (1 new, 2 modified); 2 message files; 0 Rust files

---

## Constitution Check

_GATE: Must pass before implementation. Re-check after Phase 1 design._

| Principle                                   | Status     | Notes                                                                                                                                    |
| ------------------------------------------- | ---------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| Modular, Library-First Design               | PASS       | `ImageCropDialog` is self-contained, independently testable, with a stable prop interface                                                |
| Deterministic Interfaces & Observability    | PASS       | No new IPC commands; existing `uploadModelImageBytes` binding unchanged                                                                  |
| Test-First Emphasis                         | PASS       | New component requires tests; modified components require test updates                                                                   |
| Code Quality                                | PASS       | `pnpm lint`, `pnpm check`, Prettier required before merge                                                                                |
| Testing Standards                           | PASS       | Unit tests for new component; existing tests updated (not deleted)                                                                       |
| User Experience Consistency                 | PASS       | All strings via Paraglide; drag-over color via `--primary` theme token; shadcn-svelte Dialog for modal                                   |
| Performance Requirements                    | PASS       | Crop is client-side (canvas); large images handled via `maxWidth: 2048` on `getCroppedCanvas()`; `isSaving` guard prevents double-submit |
| Safe Rust Practices                         | PASS       | No Rust changes                                                                                                                          |
| Simplicity & Semantic Versioning            | PASS       | No new abstractions beyond minimum required; no over-engineering                                                                         |
| Architectural Laws — Database               | PASS (N/A) | No new tables or migrations                                                                                                              |
| Architectural Laws — State Management       | PASS (N/A) | No new aggregates or domain events                                                                                                       |
| Architectural Laws — API Design & Transport | PASS       | No new Tauri commands; existing `uploadModelImageBytes` reused                                                                           |
| Architectural Laws — Domain Logic in Rust   | PASS       | Image validation stays in Rust backend                                                                                                   |

**Gate result**: PASS — no violations. Implementation may proceed.

---

## Project Structure

### Documentation (this feature)

```text
specs/035-drop-to-crop/
├── plan.md                        # This file
├── research.md                    # Phase 0 output
├── data-model.md                  # Phase 1 output
├── quickstart.md                  # Phase 1 output
├── contracts/
│   └── component-interfaces.ts   # Phase 1 output
├── checklists/
│   └── requirements.md           # Spec quality checklist
└── tasks.md                       # Phase 2 output (/speckit.tasks — not yet created)
```

### Source Code (modified/created by this feature)

```text
src/
└── lib/
    └── components/
        └── model-details/
            ├── ImageCropDialog.svelte     ← NEW
            ├── ImageDropZone.svelte       ← MODIFY (add crop dialog, counter drag state)
            ├── ImageUpload.svelte         ← MODIFY (add crop dialog, use bytes path)
            └── __tests__/
                ├── ImageCropDialog.test.ts   ← NEW
                ├── ImageDropZone.test.ts     ← MODIFY
                └── ImageUpload.test.ts       ← MODIFY

messages/
├── en.json    ← MODIFY (add 4 new keys)
└── it.json    ← MODIFY (add 4 new keys)
```

**Not changed**:

- `src-tauri/` (all Rust code unchanged)
- `src/lib/bindings.ts` (no new specta-generated types)
- `src/lib/components/model-details/ModelDetailsHeader.svelte` (no layout changes)
- `src-tauri/tauri.conf.json`

**Structure Decision**: Single project (Tauri + SvelteKit). All new code is in the existing `model-details` feature module following the established feature-colocation pattern. No new top-level directories or services.

---

## Phase 0 Research Summary

All research completed. See [research.md](research.md) for full details.

| Question                             | Answer                                                                                       |
| ------------------------------------ | -------------------------------------------------------------------------------------------- |
| Cropper.js v2 lifecycle in Svelte 5? | `onMount` + `bind:this` + cleanup return; Web Components API                                 |
| v2 constructor options?              | None in constructor; set `sel.aspectRatio`, `sel.initialCoverage` post-construction          |
| v2 ready detection?                  | `cropperImage.$ready()` Promise — no callback in constructor                                 |
| v2 CSS import?                       | **None** — v2 uses Shadow DOM; no `cropper.css` needed                                       |
| Canvas → Tauri bytes?                | `sel.$toCanvas()` (async) → `toBlob()` → `arrayBuffer()` → `Array.from(new Uint8Array(...))` |
| Aspect ratio for catalog photos?     | Free crop: `sel.aspectRatio = NaN` post-construction                                         |
| Drag-over accent color?              | `border-primary` / `bg-primary/10` via `@theme inline` tokens; resolves to `hsl(30 50% 50%)` |
| Nested drag-leave flickering?        | `dragenter` counter pattern (more robust than `relatedTarget` check)                         |
| Browse path image → Cropper?         | `convertFileSrc(filePath)` from `@tauri-apps/api/core`                                       |
| Drop path image → Cropper?           | `URL.createObjectURL(file)` — revoke on close                                                |
| Tauri WebView drag-and-drop?         | HTML5 `dataTransfer.files` approach is proven working in this project; keep unchanged        |
| New dependency?                      | `cropperjs` v2.1.0 — installed and approved (T001 complete)                                  |

---

## Phase 1 Design

### New Component: `ImageCropDialog.svelte`

**Props**: `open: boolean`, `imageSrc: string`, `fileName: string`, `modelId: RailwayModelId`, `onSaveSuccess?: () => void`, `onCancel?: () => void`

**Internal flow**:

1. Modal shell: shadcn-svelte `Dialog` with `bind:open`
2. `<img bind:this={imageEl} src={imageSrc}>` inside a `max-h-[70vh] overflow-hidden` container
3. Cropper.js v2 initialized on `onMount` via `new Cropper(imageEl)`; post-construction: `sel.aspectRatio = NaN`, `sel.initialCoverage = 0.8`, `sel.movable = true`, `sel.resizable = true`; ready via `cropperImage.$ready()` Promise; destroyed in `onMount` cleanup return
4. Confirm button (disabled until `isReady && !isSaving`):
   - `const canvas = await sel.$toCanvas({ width: 2048, height: 2048 })` (v2 async API)
   - `canvas.toBlob(blob => ...)` → `Uint8Array` → `uploadModelImageBytes`
   - Success: `onSaveSuccess?.()` + close
   - Error: display in footer
5. Cancel: `onCancel?.()` + close
6. On close (either path): revoke blob URL if `imageSrc.startsWith('blob:')`
7. All strings from Paraglide: `crop_dialog_title`, `crop_confirm`, `crop_cancel`, `uploading`

**CSS Note**: No CSS import needed — Cropper.js v2 styles are encapsulated in Shadow DOM.

### Modified: `ImageDropZone.svelte`

| Change                | Detail                                                                                                       |
| --------------------- | ------------------------------------------------------------------------------------------------------------ |
| Drag state            | Replace `isDragging: $state(false)` with `dragCounter: $state(0)` + `isDragging = $derived(dragCounter > 0)` |
| `ondragenter` handler | Add — increments `dragCounter`                                                                               |
| `ondragleave` handler | Replace `relatedTarget` check with `dragCounter--`                                                           |
| `ondrop` handler      | After validation: set `pendingFile = file` instead of calling `uploadModelImageBytes`; `dragCounter = 0`     |
| Drag-over classes     | Array syntax: `isDragging ? 'border-primary bg-primary/10 ring-2 ring-primary ring-inset' : '...'`           |
| Overlay text          | Change dragging overlay to use `drop_here_to_update_photo` key                                               |
| Remove                | `isUploading`, `showSuccess`, `error` for upload path; `commands.uploadModelImageBytes` direct call          |
| Add                   | `<ImageCropDialog>` bound to `pendingFile !== null`                                                          |
| Keep                  | `error` state for MIME validation rejection toast                                                            |

### Modified: `ImageUpload.svelte`

| Change            | Detail                                                                                                |
| ----------------- | ----------------------------------------------------------------------------------------------------- |
| After file select | Set `pendingFilePath = file` instead of calling `uploadModelImage`                                    |
| Add               | `<ImageCropDialog>` bound to `pendingFilePath !== null`; `imageSrc = convertFileSrc(pendingFilePath)` |
| Remove            | Direct `uploadModelImage` call from the upload flow (delete flow keeps `deleteModelImage` unchanged)  |
| Keep              | Delete flow — entirely unchanged                                                                      |

### New i18n Keys

| Key                         | English                     | Italian (suggested)                   |
| --------------------------- | --------------------------- | ------------------------------------- |
| `drop_here_to_update_photo` | `Drop here to update photo` | `Rilascia qui per aggiornare la foto` |
| `crop_dialog_title`         | `Crop Image`                | `Ritaglia immagine`                   |
| `crop_confirm`              | `Apply Crop`                | `Applica ritaglio`                    |
| `crop_cancel`               | `Cancel`                    | `Annulla`                             |

---

## Complexity Tracking

> No constitution violations found. Section not required.

---

## Risk & Mitigations

| Risk                                                    | Likelihood | Mitigation                                                                                |
| ------------------------------------------------------- | ---------- | ----------------------------------------------------------------------------------------- |
| `cropperjs` CSS conflicts with Tailwind/shadcn styles   | Low        | Scope via `.cropper-container` specificity; Tailwind purge won't affect Cropper's own CSS |
| Blob URL not revoked on abnormal close                  | Medium     | Add `$effect` cleanup in `ImageCropDialog` that revokes on `open` going false             |
| `getCroppedCanvas()` called before `ready`              | Low        | `isReady` guard on confirm button; button disabled until Cropper fires `ready`            |
| Tauri `convertFileSrc` URL rejected by Cropper img load | Low        | CSP already allows `https://asset.localhost`; confirmed in `tauri.conf.json`              |
| Test isolation for `ImageCropDialog`                    | Medium     | Mock `cropperjs` in unit tests; test component logic independently of DOM rendering       |
