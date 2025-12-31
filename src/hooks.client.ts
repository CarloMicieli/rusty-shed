import { attachConsole } from '@tauri-apps/plugin-log';

/**
 * SvelteKit client-side initialization hook.
 * This runs once when the app loads in the browser.
 * 
 * Attaches Tauri's log plugin to intercept console.* calls
 * and forward them to the Rust backend for terminal output.
 */

// Attach console forwarding when running in Tauri
if ('__TAURI__' in window) {
	attachConsole();
}
