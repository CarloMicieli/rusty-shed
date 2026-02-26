import { describe, it, expect, vi, beforeEach } from 'vitest';
import { get } from 'svelte/store';
import { modalStore, getModalStore, modalCount } from '$lib/stores/modal';

// ─── tests ────────────────────────────────────────────────────────────────

describe('modalStore', () => {
  beforeEach(() => {
    // Reset store to empty state before each test
    modalStore.clear();
  });

  describe('initial state', () => {
    it('starts with empty modal stack', () => {
      expect(get(modalStore)).toHaveLength(0);
    });
  });

  describe('trigger (open)', () => {
    it('adds a modal to the stack and returns its ID', () => {
      const id = modalStore.trigger({ type: 'alert', title: 'Test' });

      expect(typeof id).toBe('string');
      expect(id).toContain('modal-');
      expect(get(modalStore)).toHaveLength(1);
    });

    it('assigns unique IDs to each modal', () => {
      const id1 = modalStore.trigger({ type: 'alert' });
      const id2 = modalStore.trigger({ type: 'confirm' });

      expect(id1).not.toBe(id2);
    });

    it('preserves all modal options', () => {
      modalStore.trigger({
        type: 'confirm',
        title: 'Delete?',
        body: 'This is irreversible',
        meta: { itemId: '123' }
      });

      const modals = get(modalStore);
      expect(modals[0].type).toBe('confirm');
      expect(modals[0].title).toBe('Delete?');
      expect(modals[0].body).toBe('This is irreversible');
      expect(modals[0].meta).toEqual({ itemId: '123' });
    });

    it('stacks multiple modals in order', () => {
      modalStore.trigger({ title: 'First' });
      modalStore.trigger({ title: 'Second' });
      modalStore.trigger({ title: 'Third' });

      const modals = get(modalStore);
      expect(modals).toHaveLength(3);
      expect(modals[0].title).toBe('First');
      expect(modals[1].title).toBe('Second');
      expect(modals[2].title).toBe('Third');
    });
  });

  describe('close', () => {
    it('closes modal by ID', () => {
      const id = modalStore.trigger({ title: 'Closeable' });
      expect(get(modalStore)).toHaveLength(1);

      modalStore.close(id);
      expect(get(modalStore)).toHaveLength(0);
    });

    it('closes only the matching modal when multiple are open', () => {
      const id1 = modalStore.trigger({ title: 'First' });
      const id2 = modalStore.trigger({ title: 'Second' });
      const id3 = modalStore.trigger({ title: 'Third' });

      modalStore.close(id2);

      const modals = get(modalStore);
      expect(modals).toHaveLength(2);
      expect(modals.find((m) => m.id === id1)).toBeTruthy();
      expect(modals.find((m) => m.id === id2)).toBeUndefined();
      expect(modals.find((m) => m.id === id3)).toBeTruthy();
    });

    it('closes the first modal when no ID is specified', () => {
      const id1 = modalStore.trigger({ title: 'First' });
      modalStore.trigger({ title: 'Second' });

      modalStore.close();

      const modals = get(modalStore);
      expect(modals).toHaveLength(1);
      // First modal should be removed
      expect(modals.find((m) => m.id === id1)).toBeUndefined();
    });

    it('does nothing when closing unknown ID', () => {
      modalStore.trigger({ title: 'Modal' });
      modalStore.close('non-existent-id');
      expect(get(modalStore)).toHaveLength(1);
    });

    it('does nothing when closing with no ID and stack is empty', () => {
      expect(() => modalStore.close()).not.toThrow();
      expect(get(modalStore)).toHaveLength(0);
    });
  });

  describe('clear', () => {
    it('removes all modals from the stack', () => {
      modalStore.trigger({ title: 'A' });
      modalStore.trigger({ title: 'B' });
      modalStore.trigger({ title: 'C' });

      modalStore.clear();

      expect(get(modalStore)).toHaveLength(0);
    });

    it('is idempotent when stack is already empty', () => {
      expect(() => modalStore.clear()).not.toThrow();
      expect(get(modalStore)).toHaveLength(0);
    });
  });

  describe('modalCount derived store', () => {
    it('returns 0 when no modals are open', () => {
      expect(get(modalCount)).toBe(0);
    });

    it('increments as modals are opened', () => {
      modalStore.trigger({ title: 'A' });
      expect(get(modalCount)).toBe(1);

      modalStore.trigger({ title: 'B' });
      expect(get(modalCount)).toBe(2);
    });

    it('decrements when a modal is closed', () => {
      const id = modalStore.trigger({ title: 'A' });
      modalStore.trigger({ title: 'B' });

      modalStore.close(id);
      expect(get(modalCount)).toBe(1);
    });

    it('returns 0 after clear()', () => {
      modalStore.trigger({ title: 'A' });
      modalStore.trigger({ title: 'B' });
      modalStore.clear();
      expect(get(modalCount)).toBe(0);
    });
  });

  describe('getModalStore', () => {
    it('returns the same modal store instance', () => {
      const store = getModalStore();
      const id = store.trigger({ title: 'via getModalStore' });
      expect(get(modalStore)).toHaveLength(1);
      expect(get(modalStore)[0].id).toBe(id);
    });
  });
});
