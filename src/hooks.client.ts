import { attachConsole } from '@tauri-apps/plugin-log';
import { log } from '$lib/tauri-logger';

declare global {
  interface Window {
    __TAURI_INTERNALS__?: unknown;
  }
}

/**
 * SvelteKit client-side initialization hook.
 * This runs once when the app loads in the browser.
 *
 * Sets up:
 * 1. Tauri log plugin to forward console.* calls to the backend
 * 2. Global unhandled rejection handler
 * 3. Global error handler for uncaught exceptions
 */
export async function init() {
  try {
    // Attach console forwarding when running in Tauri
    const windowObj = typeof window !== 'undefined' ? window : null;
    const isAndroidWebView =
      typeof navigator !== 'undefined' && /Android/i.test(navigator.userAgent);

    if (windowObj && '__TAURI_INTERNALS__' in windowObj && !isAndroidWebView) {
      await attachConsole();
      console.log('[hooks.client.ts] Console forwarding attached to Tauri backend');
    }
  } catch (err) {
    console.error('[hooks.client.ts] Failed to attach console:', err);
  }

  // Handle unhandled promise rejections
  window.addEventListener('unhandledrejection', (event) => {
    log.error(`Unhandled promise rejection: ${String(event.reason)}`);
  });

  // Handle uncaught exceptions
  window.addEventListener('error', (event) => {
    log.error(
      `Uncaught error: ${event.message} at ${event.filename}:${event.lineno}:${event.colno}`
    );
  });
}
