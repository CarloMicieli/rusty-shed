/**
 * Image Service - Handles image path resolution and caching.
 *
 * This service provides:
 * - Image path resolution from Tauri backend
 * - Context API for dependency injection
 * - Error handling for missing images
 */

import { setContext, getContext } from 'svelte';
import { SvelteMap } from 'svelte/reactivity';
import { safeInvoke } from './TauriAdapter';

// ─────────────────────────────────────────────────────────────
// CONTEXT KEY (for Dependency Injection)
// ─────────────────────────────────────────────────────────────
const SERVICE_KEY = Symbol('image-service');

// ─────────────────────────────────────────────────────────────
// SERVICE CLASS
// ─────────────────────────────────────────────────────────────
export class ImageService {
  // Cache for resolved image paths
  #cache = $state<SvelteMap<string, string>>(new SvelteMap());

  // ─────────────────────────────────────────────────────────────
  // USE CASES (Public Methods)
  // ─────────────────────────────────────────────────────────────

  /**
   * Resolve an image path from the backend.
   *
   * @param id - The image ID
   * @param category - The image category (e.g., 'railway_model', 'manufacturer')
   * @returns The resolved image path or null if not found
   */
  async resolveImagePath(id: string, category: string): Promise<string | null> {
    const cacheKey = `${category}:${id}`;

    // Check cache first
    const cached = this.#cache.get(cacheKey);
    if (cached) {
      return cached;
    }

    // Fetch from backend
    const result = await safeInvoke<string>('get_image_path', { id, category });
    if (!result.ok) {
      console.error('Failed to resolve image path:', result.error);
      return null;
    }

    // Cache the result
    this.#cache.set(cacheKey, result.data);
    return result.data;
  }

  /**
   * Clear the image path cache.
   */
  clearCache(): void {
    this.#cache.clear();
  }

  /**
   * Get cache size (for debugging).
   */
  getCacheSize(): number {
    return this.#cache.size;
  }
}

// ─────────────────────────────────────────────────────────────
// CONTEXT HELPERS (Dependency Injection)
// ─────────────────────────────────────────────────────────────

/**
 * Initialize and set the ImageService in the current context.
 *
 * @param service - Optional service instance (for testing)
 * @returns The service instance
 */
export function setImageService(service?: ImageService): ImageService {
  const instance = service ?? new ImageService();
  setContext(SERVICE_KEY, instance);
  return instance;
}

/**
 * Get the ImageService from the current context.
 *
 * @returns The service instance
 * @throws Error if service is not found in context
 */
export function getImageService(): ImageService {
  const service = getContext<ImageService>(SERVICE_KEY);
  if (!service) {
    throw new Error(
      'ImageService not found in context. Did you call setImageService() in a parent component?'
    );
  }
  return service;
}

// ─────────────────────────────────────────────────────────────
// SINGLETON EXPORT (for components that can't use Context API)
// ─────────────────────────────────────────────────────────────

/**
 * @deprecated Prefer using getImageService() with Context API
 */
export const imageService = new ImageService();
