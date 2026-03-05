/**
 * Component Interface Contracts: Drop-to-Crop Image Workflow
 * Feature: 035-drop-to-crop
 *
 * These type definitions describe the public props of each component
 * involved in this feature. They are specification artifacts only —
 * the actual implementation must match these contracts.
 *
 * NOTE: Tauri IPC commands are unchanged from existing bindings.
 * See src/lib/bindings.ts for UploadModelImageBytesArgs, DeleteModelImageArgs, etc.
 */

import type { RailwayModelId } from '$lib/bindings';

// ─────────────────────────────────────────────────────────────────────────────
// NEW COMPONENT
// ─────────────────────────────────────────────────────────────────────────────

/**
 * ImageCropDialog — modal crop editor that appears between file acquisition
 * (drop or browse) and persistence.
 *
 * File: src/lib/components/model-details/ImageCropDialog.svelte
 */
export interface ImageCropDialogProps {
  /**
   * Whether the dialog is visible.
   * Controlled by the parent (ImageDropZone or ImageUpload).
   */
  open: boolean;

  /**
   * URL of the image to crop.
   * - For drag-and-drop path: `URL.createObjectURL(file)`  (blob URL)
   * - For browse path: `convertFileSrc(filePath)`           (asset:// URL)
   *
   * The dialog is responsible for revoking blob URLs on close.
   * Blob URLs are detected by the `blob:` scheme prefix.
   */
  imageSrc: string;

  /**
   * File name used as the `fileName` argument to `uploadModelImageBytes`.
   * For the crop output, the extension is normalized to `.jpg` (JPEG output).
   */
  fileName: string;

  /**
   * The model whose photo is being updated.
   */
  modelId: RailwayModelId;

  /**
   * Called after `uploadModelImageBytes` completes successfully.
   * The parent uses this to trigger an image refresh.
   */
  onSaveSuccess?: () => void;

  /**
   * Called when the user dismisses the dialog without saving.
   * The parent uses this to reset `pendingFile` / `pendingFilePath`.
   */
  onCancel?: () => void;
}

// ─────────────────────────────────────────────────────────────────────────────
// MODIFIED COMPONENTS — Prop changes only
// ─────────────────────────────────────────────────────────────────────────────

/**
 * ImageDropZone — drag-and-drop target for model photos.
 *
 * File: src/lib/components/model-details/ImageDropZone.svelte
 *
 * CHANGES from current version:
 * - Drop no longer calls `uploadModelImageBytes` directly.
 * - Valid drop sets `pendingFile` (internal state) and opens `ImageCropDialog`.
 * - `onUploadSuccess` callback is now forwarded from `ImageCropDialog.onSaveSuccess`.
 *
 * PROPS UNCHANGED — external interface is the same:
 */
export interface ImageDropZoneProps {
  modelId: RailwayModelId;
  onUploadSuccess?: () => void;
}

/**
 * ImageUpload — file-browser button + delete flow for model photos.
 *
 * File: src/lib/components/model-details/ImageUpload.svelte
 *
 * CHANGES from current version:
 * - After file selection, instead of calling `uploadModelImage` (path-based),
 *   the component sets `pendingFilePath` (internal state) and opens `ImageCropDialog`.
 * - `ImageCropDialog` calls `uploadModelImageBytes` (bytes-based) after crop.
 * - Delete flow is UNCHANGED.
 *
 * PROPS UNCHANGED — external interface is the same:
 */
export interface ImageUploadProps {
  modelId: RailwayModelId;
  hasExistingImage?: boolean;
  onUploadSuccess?: () => void;
}

// ─────────────────────────────────────────────────────────────────────────────
// REUSED IPC CONTRACTS (no changes)
// ─────────────────────────────────────────────────────────────────────────────

/**
 * The only Tauri command used for saving a cropped image.
 * Both ImageDropZone and ImageUpload now converge on this single save path.
 *
 * Defined in: src/lib/bindings.ts → commands.uploadModelImageBytes
 * Rust handler: src-tauri/src/media/interface/command_handlers.rs
 */
export interface UploadModelImageBytesArgs {
  modelId: string;
  fileName: string;
  /** Raw image bytes as a plain number array (Rust Vec<u8> serialization via specta) */
  fileData: number[];
}

/**
 * Internal helper type for the crop output pipeline.
 * Not a component prop — used inside ImageCropDialog.
 */
export interface CropOutput {
  /** JPEG bytes from canvas.toBlob('image/jpeg', 0.9) */
  fileData: number[];
  /** Normalized file name with .jpg extension */
  fileName: string;
}
