import { writable } from 'svelte/store';

// Holds the application version as reported by the Tauri backend.
export const appVersion = writable<string>('');

export function setAppVersion(v: string) {
	appVersion.set(v);
}
