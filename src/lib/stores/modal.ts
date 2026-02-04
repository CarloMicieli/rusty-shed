/**
 * Modal store - shadcn-svelte compatible modal management
 * Replaces Skeleton's getModalStore with a lightweight Svelte 5 implementation
 * @fileoverview Modal management system for displaying dialogs and modals
 */

import { writable, derived } from 'svelte/store';

export interface ModalOptions {
  title?: string;
  body?: string;
  component?: unknown;
  meta?: Record<string, unknown>;
  response?: (value: unknown) => void;
}

export interface Modal extends ModalOptions {
  id: string;
}

function createModalStore() {
  const { subscribe, set, update } = writable<Modal[]>([]);

  return {
    subscribe,

    /**
     * Trigger (open) a modal
     * @param modal Modal configuration
     * @returns Modal ID
     */
    trigger: (modal: ModalOptions) => {
      const id = `modal-${Date.now()}-${Math.random()}`;
      const newModal: Modal = {
        ...modal,
        id
      };

      update((modals) => [...modals, newModal]);
      return id;
    },

    /**
     * Close a modal by ID
     * @param id Modal ID or first modal if not specified
     */
    close: (id?: string) => {
      update((modals) => {
        if (!id && modals.length > 0) {
          // Close first modal if no ID specified
          return modals.slice(1);
        }
        return modals.filter((m) => m.id !== id);
      });
    },

    /**
     * Close all modals
     */
    clear: () => {
      set([]);
    }
  };
}

export const modalStore = createModalStore();

/**
 * Get modal store instance (Skeleton API compatibility)
 * @returns Modal store with trigger/close/clear methods
 */
export function getModalStore() {
  return modalStore;
}

/** Derived store for active modal count */
export const modalCount = derived(modalStore, ($modals) => $modals.length);

/** Derived store for first (active) modal */
export const activeModal = derived(modalStore, ($modals) => $modals[0] ?? null);
