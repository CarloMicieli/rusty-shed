/**
 * Toast notification store using shadcn-svelte compatible implementation
 * Manages error and success toast notifications with top-right positioning
 * @fileoverview Toast management system replacing Skeleton's toaster
 */

import { writable, derived } from 'svelte/store';
import { v4 as uuidv4 } from 'uuid';

export interface Toast {
  id: string;
  title: string;
  description?: string;
  variant?: 'default' | 'destructive' | 'success';
  duration?: number;
  action?: {
    label: string;
    onClick: () => void;
  };
}

export interface ToastState {
  toasts: Toast[];
}

function createToastStore() {
  const { subscribe, set, update } = writable<ToastState>({ toasts: [] });

  return {
    subscribe,

    /**
     * Add a new toast notification
     * @param toast Toast configuration
     * @returns Toast ID for removal reference
     */
    add: (toast: Omit<Toast, 'id'>) => {
      const id = uuidv4();
      const newToast: Toast = {
        ...toast,
        id,
        duration: toast.duration ?? 4000
      };

      update((state) => ({
        toasts: [...state.toasts, newToast]
      }));

      // Auto-dismiss after duration
      if (newToast.duration && newToast.duration > 0) {
        setTimeout(() => {
          toastStore.remove(id);
        }, newToast.duration);
      }

      return id;
    },

    /**
     * Remove a specific toast by ID
     * @param id Toast ID to remove
     */
    remove: (id: string) => {
      update((state) => ({
        toasts: state.toasts.filter((t) => t.id !== id)
      }));
    },

    /**
     * Clear all toasts
     */
    clear: () => {
      set({ toasts: [] });
    },

    /**
     * Show an error toast
     * @param title Error title
     * @param description Optional error details
     */
    error: (title: string, description?: string) => {
      return toastStore.add({
        title,
        description,
        variant: 'destructive',
        duration: 5000
      });
    },

    /**
     * Show a success toast
     * @param title Success title
     * @param description Optional details
     */
    success: (title: string, description?: string) => {
      return toastStore.add({
        title,
        description,
        variant: 'success',
        duration: 4000
      });
    }
  };
}

export const toastStore = createToastStore();

/** Derived store for total toast count */
export const toastCount = derived(
  toastStore,
  ($toastStore) => $toastStore.toasts.length
);
