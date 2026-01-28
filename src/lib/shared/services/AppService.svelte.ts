/**
 * App Service - Manages global application state.
 *
 * This service replaces the legacy app store and provides:
 * - Application version management
 * - Global app state using Svelte 5 $state
 * - Context API for dependency injection
 */

import { setContext, getContext } from 'svelte';

// ─────────────────────────────────────────────────────────────
// CONTEXT KEY (for Dependency Injection)
// ─────────────────────────────────────────────────────────────
const SERVICE_KEY = Symbol('app-service');

// ─────────────────────────────────────────────────────────────
// SERVICE CLASS
// ─────────────────────────────────────────────────────────────
export class AppService {
  // Private reactive state
  #version = $state<string>('');
  #isReady = $state(false);

  // Public readonly getters (defensive encapsulation)
  get version(): string {
    return this.#version;
  }

  get isReady(): boolean {
    return this.#isReady;
  }

  // ─────────────────────────────────────────────────────────────
  // USE CASES (Public Methods)
  // ─────────────────────────────────────────────────────────────

  /**
   * Set the application version.
   *
   * @param version - The version string (e.g., '1.0.0')
   */
  setVersion(version: string): void {
    this.#version = version;
  }

  /**
   * Mark the application as ready (fully initialized).
   */
  markAsReady(): void {
    this.#isReady = true;
  }

  /**
   * Reset the application state (useful for testing).
   */
  reset(): void {
    this.#version = '';
    this.#isReady = false;
  }
}

// ─────────────────────────────────────────────────────────────
// CONTEXT HELPERS (Dependency Injection)
// ─────────────────────────────────────────────────────────────

/**
 * Initialize and set the AppService in the current context.
 *
 * @param service - Optional service instance (for testing)
 * @returns The service instance
 */
export function setAppService(service?: AppService): AppService {
  const instance = service ?? new AppService();
  setContext(SERVICE_KEY, instance);
  return instance;
}

/**
 * Get the AppService from the current context.
 *
 * @returns The service instance
 * @throws Error if service is not found in context
 */
export function getAppService(): AppService {
  const service = getContext<AppService>(SERVICE_KEY);
  if (!service) {
    throw new Error(
      'AppService not found in context. Did you call setAppService() in a parent component?'
    );
  }
  return service;
}

// ─────────────────────────────────────────────────────────────
// LEGACY COMPATIBILITY (to be removed after migration)
// ─────────────────────────────────────────────────────────────

/**
 * @deprecated Use AppService.setVersion() instead
 */
export function setAppVersion(_version: string): void {
  console.warn('setAppVersion is deprecated. Use AppService.setVersion() instead.');
}
