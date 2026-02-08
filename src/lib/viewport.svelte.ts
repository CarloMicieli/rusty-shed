/**
 * Viewport State Management for Cross-Platform Resolution Awareness
 *
 * This module provides reactive viewport state management using Svelte 5 runes.
 * It tracks scale factor, window dimensions, mobile status, and high DPI state,
 * and automatically updates CSS custom properties for styling.
 *
 * @module viewport
 */

import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';

/**
 * Viewport state interface
 */
interface ViewportState {
  /** Current scale factor (DPI scaling) */
  scaleFactor: number;
  /** Current window width in pixels */
  width: number;
  /** Current window height in pixels */
  height: number;
  /** Whether the device is mobile (Android, iOS) */
  isMobile: boolean;
  /** Whether the display is high DPI (scale factor >= 1.5) */
  isHighDPI: boolean;
}

/**
 * Creates and manages viewport state with automatic event listeners.
 *
 * This function initializes the viewport state and sets up listeners for:
 * - Scale factor changes (monitor switching, DPI changes)
 * - Window resize events
 *
 * It also applies CSS custom properties to the document root for styling integration.
 *
 * @returns Reactive viewport state object
 *
 * @example
 * ```svelte
 * <script lang="ts">
 *   import { createViewport } from '$lib/viewport.svelte';
 *
 *   const viewport = createViewport();
 * </script>
 *
 * {#if viewport.isMobile}
 *   <MobileLayout />
 * {:else}
 *   <DesktopLayout />
 * {/if}
 * ```
 */
export function createViewport(): ViewportState {
  // Initialize state with default values
  let scaleFactor = $state(1.0);
  let width = $state(800);
  let height = $state(600);
  let isMobile = $state(false);
  const isHighDPI = $derived(scaleFactor >= 1.5);

  /**
   * Checks if the current platform is mobile (Android or iOS).
   *
   * @returns True if running on a mobile platform
   */
  function checkMobile(): boolean {
    // Use user agent detection for mobile platforms
    return /Android|iPhone|iPad|iPod/i.test(navigator.userAgent);
  }

  /**
   * Updates the viewport state by fetching the current scale factor and window size.
   */
  async function updateViewport(): Promise<void> {
    try {
      const window = getCurrentWebviewWindow();
      const factor = await window.scaleFactor();
      const size = await window.innerSize();

      scaleFactor = factor;
      width = size.width;
      height = size.height;

      console.log(`Viewport updated: ${width}×${height}, scale: ${factor}`);
    } catch (error) {
      console.error('Failed to update viewport:', error);
    }
  }

  /**
   * Applies viewport state as CSS custom properties on the document root.
   */
  function applyCSSVariables(): void {
    if (typeof document === 'undefined') return;

    const root = document.documentElement;
    root.style.setProperty('--app-scale', scaleFactor.toString());
    root.style.setProperty('--app-width', `${width}px`);
    root.style.setProperty('--app-height', `${height}px`);

    // Apply safe area insets for mobile devices
    if (isMobile) {
      root.style.setProperty('--safe-area-top', 'env(safe-area-inset-top, 0px)');
      root.style.setProperty('--safe-area-bottom', 'env(safe-area-inset-bottom, 0px)');
      root.style.setProperty('--safe-area-left', 'env(safe-area-inset-left, 0px)');
      root.style.setProperty('--safe-area-right', 'env(safe-area-inset-right, 0px)');
    }
  }

  // Initialize and setup event listeners
  $effect(() => {
    let scaleUnlisten: UnlistenFn | null = null;
    let resizeUnlisten: UnlistenFn | null = null;

    async function setup() {
      // Check platform
      isMobile = checkMobile();
      await updateViewport();

      // Listen to scale factor changes (monitor switching, DPI changes)
      scaleUnlisten = await listen('tauri://scale-change', async (event) => {
        console.log('Scale change event:', event.payload);
        await updateViewport();
      });

      // Listen to window resize events
      resizeUnlisten = await listen('tauri://resize', async (event) => {
        console.log('Resize event:', event.payload);
        await updateViewport();
      });

      console.log('Viewport listeners initialized');
    }

    setup();

    // Cleanup listeners on effect teardown
    return () => {
      if (scaleUnlisten) scaleUnlisten();
      if (resizeUnlisten) resizeUnlisten();
      console.log('Viewport listeners cleaned up');
    };
  });

  // Apply CSS variables whenever state changes
  $effect(() => {
    applyCSSVariables();
  });

  // Return reactive state object
  return {
    get scaleFactor() {
      return scaleFactor;
    },
    get width() {
      return width;
    },
    get height() {
      return height;
    },
    get isMobile() {
      return isMobile;
    },
    get isHighDPI() {
      return isHighDPI;
    }
  };
}
