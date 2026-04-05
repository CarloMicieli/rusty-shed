/**
 * Image Blob URL Cache
 *
 * Module-level singleton that survives component unmount/remount cycles.
 * Used by RailwayModelPreviewCard to avoid redundant Tauri IPC calls and
 * file-system reads when a virtual-scrolled card scrolls back into view.
 *
 * Keys are model IDs; values are blob: URLs created via URL.createObjectURL().
 * URLs are never revoked — the collection is bounded in size and the app
 * session is short-lived (Tauri desktop), so the memory trade-off is fine.
 */
const blobCache = new Map<string, string>();

export function getCachedImage(modelId: string): string | undefined {
  return blobCache.get(modelId);
}

export function setCachedImage(modelId: string, url: string): void {
  blobCache.set(modelId, url);
}
