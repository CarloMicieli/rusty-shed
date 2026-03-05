# Research: Drop-to-Crop Image Workflow

**Feature**: 035-drop-to-crop
**Phase**: 0 — Research
**Date**: 2026-03-05

---

## R-01: Cropper.js v2 Integration with Svelte 5

**Decision**: Use `cropperjs` v2.x managed via `bind:this` ref + `onMount` lifecycle.

**Important**: v2 is a ground-up rewrite using native Web Components. The API is substantially different from v1. The installed version is `^2.1.0`.

**Key differences from v1**:

| Concern | v1 | v2 (installed) |
|---|---|---|
| CSS import | `import 'cropperjs/dist/cropper.css'` | **None — Shadow DOM styles built-in** |
| Constructor options | `{ aspectRatio, viewMode, dragMode, ... }` | `{}` — properties set post-construction on elements |
| `aspectRatio` | constructor option | `sel.aspectRatio = NaN` on `<cropper-selection>` |
| `autoCropArea` | constructor option | `sel.initialCoverage = 0.8` on `<cropper-selection>` |
| `ready` event | constructor callback | `cropperImage.$ready(cb)` — Promise-based |
| Crop output | `cropper.getCroppedCanvas()` — sync | `sel.$toCanvas()` — async `Promise<HTMLCanvasElement>` |
| TypeScript types | `@types/cropperjs` (separate) | Built-in, bundled in package |

**Svelte 5 `onMount` pattern (v2)**:

```svelte
<script lang="ts">
  import { onMount } from 'svelte';
  import Cropper from 'cropperjs';
  // NO CSS import needed — v2 uses Shadow DOM

  let imageEl = $state<HTMLImageElement | null>(null);
  let cropperInstance: Cropper | null = null;
  let isReady = $state(false);

  onMount(() => {
    if (!imageEl) return;

    cropperInstance = new Cropper(imageEl);

    // Configure selection after construction
    const sel = cropperInstance.getCropperSelection()!;
    sel.aspectRatio = NaN;          // free crop — best for varied catalog photos
    sel.initialCoverage = 0.8;
    sel.movable = true;
    sel.resizable = true;

    // Wait for image load (v2 promise-based)
    cropperInstance.getCropperImage()!.$ready().then(() => {
      isReady = true;
    });

    return () => { cropperInstance?.destroy(); cropperInstance = null; };
  });
</script>
```

**Canvas → `number[]` conversion (v2 — async)**:

```ts
const sel = cropperInstance!.getCropperSelection()!;
const canvas = await sel.$toCanvas({ width: 2048, height: 2048 });
const fileData = await new Promise<number[]>((resolve, reject) =>
  canvas.toBlob(async (blob) => {
    if (!blob) return reject(new Error('toBlob returned null'));
    resolve(Array.from(new Uint8Array(await blob.arrayBuffer())));
  }, 'image/jpeg', 0.9)
);
// Then call the existing Tauri command unchanged:
await commands.uploadModelImageBytes({ modelId, fileName: 'cropped.jpg', fileData });
```

**Aspect ratio recommendation**: Free crop (`NaN`) as default. Model photos come from varied sources (manufacturer catalog scans, personal photos, box art) with no universal ratio.

**Cleanup**: Always destroy via `onMount` cleanup return. Set `cropperInstance = null` to prevent stale references. Guard `$toCanvas()` behind `isReady` flag.

**Alternatives considered**:
- `svelte-easy-crop` — simpler API but fewer controls; Cropper.js is explicitly specified in the feature request.

---

## R-02: Image Loading Into Cropper.js — Two Source Paths

**Decision**: The `ImageCropDialog` receives a pre-resolved `string` URL. The two entry points produce URLs differently:

| Entry Point | File Access | URL for Cropper |
|---|---|---|
| Drag-and-drop (`ImageDropZone`) | Browser `File` object via `dataTransfer.files` | `URL.createObjectURL(file)` |
| Browse dialog (`ImageUpload`) | Filesystem path string from Tauri dialog | `convertFileSrc(filePath)` from `@tauri-apps/api/core` |

`convertFileSrc` produces `https://asset.localhost/...` URLs. The CSP in `tauri.conf.json` permits `img-src 'self' data: asset: https://asset.localhost`, so Cropper.js can load these URLs in the `<img>` element.

Blob URLs from `createObjectURL` must be revoked on dialog close (both confirm and cancel) to avoid memory leaks.

---

## R-03: Drag-and-Drop in Tauri 2.0 WebView

**Decision**: Keep the existing HTML5 `event.dataTransfer.files` approach.

**Rationale**: The existing `ImageDropZone.svelte` already uses `event.dataTransfer?.files` and is proven working in this Tauri app on Linux (WebKitGTK). The component has passing integration tests and is in production. `tauri.conf.json` does not set `fileDropEnabled: false`, meaning the WebView receives file drop events normally on the target platform.

**Counter-pattern for nested drag events**: Replace the current `relatedTarget` check with the `dragenter` counter pattern for more robust drag-leave detection when the drop zone has child elements (the overlay text and icon are children). The counter is more reliable across platforms:

```ts
let dragCounter = $state(0);
const isDragging = $derived(dragCounter > 0);

function onDragEnter(e: DragEvent) { e.preventDefault(); dragCounter++; }
function onDragLeave(e: DragEvent) { e.preventDefault(); dragCounter--; }
function onDrop(e: DragEvent) { e.preventDefault(); dragCounter = 0; /* handle */ }
```

**Tauri `onDragDropEvent` API**: An alternative that gives file paths from OS drags via position-based hit testing. This would require replacing `dataTransfer.files` with a listener approach and manual DOM hit detection. **Not recommended for this feature** since the existing approach works and the overhead is unjustified. Flagged as a future fallback if platform compatibility issues arise on Windows/macOS.

---

## R-04: Tailwind 4 `@theme inline` and Drag-Over Styling

**Decision**: Use `border-primary`, `bg-primary/10`, `ring-2 ring-primary ring-inset`, and `transition-colors duration-200` for the drag-over state.

**Rationale**: The `layout.css` file contains `@theme inline { --color-primary: var(--primary); }`. In Tailwind v4, `@theme inline` inlines the CSS variable reference directly into utilities, so `border-primary` compiles to `border-color: var(--primary)`. This means the drag-over accent color automatically tracks the theme token — copper `hsl(30 50% 50%)` in the current steampunk dark theme — without any hardcoded hex values.

**Conditional class pattern** (Svelte 5 Runes, avoids string interpolation which prevents Tailwind class scanning):

```svelte
<div
  class={[
    'border-2 border-dashed rounded-lg transition-colors duration-200',
    isDragging
      ? 'border-primary bg-primary/10 ring-2 ring-primary ring-inset'
      : 'border-muted-foreground/25 hover:border-muted-foreground/50'
  ]}
>
```

The array syntax with boolean guards is the Svelte 5 idiomatic pattern — Tailwind's scanner detects full class name strings as static tokens inside the array.

**Alternatives considered**: Template literal interpolation — avoided because it can prevent Tailwind from scanning class names at build time.

---

## R-05: New i18n Message Keys Required

**Decision**: Add 4 keys to `messages/en.json` and `messages/it.json`.

| Key | English value |
|---|---|
| `drop_here_to_update_photo` | `"Drop here to update photo"` |
| `crop_dialog_title` | `"Crop Image"` |
| `crop_confirm` | `"Apply Crop"` |
| `crop_cancel` | `"Cancel"` |

These are the minimum keys needed. Existing keys (`drop_image_here`, `uploading`, `upload_success`, etc.) are reused unchanged.

---

## R-06: No Backend Changes Required

**Decision**: No Rust code, IPC commands, SQL migrations, or `specta` bindings changes needed.

**Rationale**:
- `uploadModelImageBytes` already accepts `Vec<u8>` and handles all storage. Cropper output (canvas bytes) maps directly to this existing interface.
- The spec requirement "Keep the current code to store/retrieve images" is fully satisfied by the crop-then-save pipeline.
- No new domain events, aggregates, or repositories are needed for this frontend UX enhancement.

---

## Summary: All NEEDS CLARIFICATION Resolved

| Item | Resolution |
|---|---|
| Cropper.js lifecycle in Svelte 5 | `onMount` + `bind:this` + cleanup return; **v2 Web Components API** |
| Canvas → Tauri bytes | `sel.$toCanvas()` (async, v2) → `toBlob() → Uint8Array → number[]` |
| Aspect ratio | Free crop: `sel.aspectRatio = NaN` set post-construction on `<cropper-selection>` |
| v2 ready detection | `cropperImage.$ready()` (Promise) — no constructor callback |
| v2 CSS | None needed — Shadow DOM styles built-in |
| Drag-over color | `border-primary` / `bg-primary/10` via `@theme inline` tokens |
| Nested drag-leave | `dragenter` counter pattern |
| Browse path image loading | `convertFileSrc(filePath)` |
| Drop path image loading | `URL.createObjectURL(file)` |
| Save command | Unchanged: `uploadModelImageBytes` |
| New dependency | `cropperjs` v2.1.0 — **installed and approved** |
