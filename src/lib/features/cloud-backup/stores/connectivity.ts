import { writable } from 'svelte/store';
import type { ConnectivityStatus } from '$lib/bindings';

/**
 * Connectivity status store for cloud backup feature.
 */
export const connectivityStore = writable<ConnectivityStatus | null>(null);

/**
 * Update connectivity status from a backend check or event.
 */
export function setConnectivityStatus(status: ConnectivityStatus): void {
  connectivityStore.set(status);
}

/**
 * Mark connectivity as offline with a timestamp.
 */
export function setConnectivityOffline(checkedAt: string = new Date().toISOString()): void {
  connectivityStore.set({ isOnline: false, checkedAt });
}
