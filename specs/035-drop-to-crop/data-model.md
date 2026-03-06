# Data Model: Drop-to-Crop Image Workflow

**Feature**: 035-drop-to-crop
**Phase**: 1 — Design
**Date**: 2026-03-05

---

## Overview

This feature is a **pure frontend enhancement**. No new persistent entities or database tables are introduced. The data model section focuses on:

1. The transient in-memory state within the new `ImageCropDialog` component.
2. The existing `Model Photo` entity that is updated via the existing backend command.

---

## Existing Persistent Entity (Unchanged)

### Model Photo

Managed entirely by the Rust backend. The frontend does not define this schema.

| Attribute         | Type                      | Notes                                            |
| ----------------- | ------------------------- | ------------------------------------------------ |
| `modelId`         | `RailwayModelId` (string) | The catalog entry owning this photo              |
| `filePath`        | `string`                  | Absolute path on disk within app data directory  |
| `hasImage`        | `boolean`                 | True when a photo exists for this model          |
| `placeholderHtml` | `string \| null`          | SVG placeholder rendered when `hasImage = false` |

**Access**: read via `commands.getRailwayModelImage({ railwayModelId })`, write via `commands.uploadModelImageBytes(args)` or `commands.uploadModelImage(args)`, delete via `commands.deleteModelImage(args)`. All are pre-existing commands; none are added or modified by this feature.

---

## Transient In-Component State

### `ImageCropDialog` Internal State

This state exists only while the dialog is open. It is never persisted.

| State Variable    | Type              | Lifecycle                                                       | Description                                                     |
| ----------------- | ----------------- | --------------------------------------------------------------- | --------------------------------------------------------------- |
| `cropperInstance` | `Cropper \| null` | Created on `onMount`, destroyed on close/unmount                | The Cropper.js controller bound to the `<img>` element          |
| `isReady`         | `boolean`         | `false` until `cropperImage.$ready()` Promise resolves (v2 API) | Guards against calling `sel.$toCanvas()` before the image loads |
| `isSaving`        | `boolean`         | `true` during the async `uploadModelImageBytes` call            | Controls spinner and disabled state of confirm button           |
| `saveError`       | `string \| null`  | Set on command failure; cleared on open                         | Displayed in the dialog footer                                  |

### `ImageDropZone` Modified State

Replaces current direct-upload state with a crop-dialog open trigger.

| State Variable | Type                | Change                              | Description                                                           |
| -------------- | ------------------- | ----------------------------------- | --------------------------------------------------------------------- |
| `dragCounter`  | `number`            | NEW (replaces `isDragging` boolean) | Counter pattern: > 0 means drag active                                |
| `isDragging`   | `boolean` (derived) | NEW (`$derived(dragCounter > 0)`)   | Replaces direct `$state(false)`                                       |
| `pendingFile`  | `File \| null`      | NEW                                 | The validated file awaiting crop; drives crop dialog open state       |
| `isUploading`  | `boolean`           | REMOVED                             | Moved into `ImageCropDialog`; drop zone no longer saves directly      |
| `showSuccess`  | `boolean`           | REMOVED                             | Moved into `ImageCropDialog`                                          |
| `error`        | `string \| null`    | KEPT                                | Still used for validation rejection (wrong MIME type, multiple files) |

### `ImageUpload` Modified State

| State Variable    | Type             | Change  | Description                                                       |
| ----------------- | ---------------- | ------- | ----------------------------------------------------------------- |
| `pendingFilePath` | `string \| null` | NEW     | Path returned by Tauri file dialog; drives crop dialog open state |
| `isUploading`     | `boolean`        | REMOVED | Moved into `ImageCropDialog`                                      |
| `showSuccess`     | `boolean`        | REMOVED | Moved into `ImageCropDialog`                                      |
| `error`           | `string \| null` | KEPT    | Upload/delete errors still displayed in this component            |

---

## State Flow Diagrams

### Drop Path

```
OS file drag
    ↓
ondragenter  → dragCounter++
ondragleave  → dragCounter--
ondrop       → dragCounter = 0
    ↓
Validate MIME type
  ✗ → show toast, return
  ✓ → pendingFile = file
        ↓
    ImageCropDialog opens (src = URL.createObjectURL(file))
        ↓
    User adjusts crop
        ↓
    Confirm → getCroppedCanvas() → toBlob() → Uint8Array → uploadModelImageBytes
            → onSaveSuccess() → revoke blob URL → close dialog
    Cancel  → revoke blob URL → close dialog → pendingFile = null
```

### Browse Path

```
Click "Upload Image" / "Change Image" button
    ↓
Tauri file dialog (open({ multiple: false, filters: [...] }))
    ↓
User cancels → return
User selects → pendingFilePath = filePath
    ↓
ImageCropDialog opens (src = convertFileSrc(filePath))
    ↓
User adjusts crop
    ↓
Confirm → getCroppedCanvas() → toBlob() → Uint8Array → uploadModelImageBytes
        → onSaveSuccess() → close dialog
Cancel  → close dialog → pendingFilePath = null
```

---

## Key Validation Rules (Frontend, Pre-Crop)

These rules are applied in `ImageDropZone` before opening the crop dialog. They mirror the existing validation already in the component.

| Rule                                                         | Trigger                         | Response                                               |
| ------------------------------------------------------------ | ------------------------------- | ------------------------------------------------------ |
| Only one file allowed                                        | `dataTransfer.files.length > 1` | Toast error, no crop dialog                            |
| MIME type must be `image/jpeg`, `image/png`, or `image/webp` | `file.type` check               | Toast error, no crop dialog                            |
| MIME type empty                                              | `file.type === ''`              | Pass through to backend validation (existing behavior) |

No new validation rules are added. The backend performs authoritative validation.
