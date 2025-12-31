console.log('[hooks.client.ts] File is loading - before import');

import { attachConsole } from '@tauri-apps/plugin-log';

console.log('[hooks.client.ts] After import, checking __TAURI__:', '__TAURI__' in window);

/**
 * SvelteKit client-side initialization hook.
 * This runs once when the app loads in the browser.
 *
 * Attaches Tauri's log plugin to intercept console.* calls
 * and forward them to the Rust backend for terminal output.
 */

// Attach console forwarding when running in Tauri
if ('__TAURI__' in window) {
  console.log('[hooks.client.ts] Tauri detected, calling attachConsole()');
  // attachConsole() is async and returns a Promise<void>
  // We await it to ensure it completes before any logs are sent
  attachConsole()
    .then(() => {
      console.log('[hooks.client.ts] Console forwarding attached to Tauri backend');
      console.log('[hooks.client.ts] Test log - you should see this in the terminal!');
    })
    .catch((err) => {
      console.error('[hooks.client.ts] Failed to attach console:', err);
    });
} else {
  console.log('[hooks.client.ts] NOT running in Tauri, __TAURI__ not found');
}
