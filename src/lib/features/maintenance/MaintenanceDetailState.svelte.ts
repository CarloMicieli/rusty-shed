/**
 * MaintenanceDetailState - Reactive state for a single maintenance card detail view.
 *
 * Manages loading, error, and optimistic event insertion for the detail page.
 */

import { setContext, getContext } from 'svelte';
import { safeInvoke } from '$lib/shared/services/TauriAdapter';
import type {
  MaintenanceCardView,
  AddMaintenanceArgs,
  MaintenanceCardEventView
} from '$lib/bindings';

const STATE_KEY = Symbol('maintenance-detail-state');

export default class MaintenanceDetailState {
  #card = $state<MaintenanceCardView | null>(null);
  #isLoading = $state(false);
  #error = $state<string | null>(null);

  get card(): MaintenanceCardView | null {
    return this.#card;
  }

  get isLoading(): boolean {
    return this.#isLoading;
  }

  get error(): string | null {
    return this.#error;
  }

  async loadCard(id: string): Promise<void> {
    this.#isLoading = true;
    this.#error = null;

    try {
      const result = await safeInvoke<MaintenanceCardView | null>('get_maintenance_card', {
        cardId: id
      });
      if (!result.ok) {
        this.#error = result.error.message;
        return;
      }
      this.#card = result.data;
    } catch (err) {
      this.#error = err instanceof Error ? err.message : String(err);
    } finally {
      this.#isLoading = false;
    }
  }

  async deleteEvent(eventId: string): Promise<void> {
    if (!this.#card) return;

    // Optimistic removal
    const previous = this.#card.events;
    this.#card = {
      ...this.#card,
      events: this.#card.events.filter((e) => e.id !== eventId)
    };

    const result = await safeInvoke<null>('delete_maintenance_event', { eventId });
    if (!result.ok) {
      // Rollback
      if (this.#card) {
        this.#card = { ...this.#card, events: previous };
      }
      throw new Error(result.error.message);
    }

    await this.loadCard(this.#card?.id ?? '');
  }

  async addEvent(args: AddMaintenanceArgs): Promise<void> {
    if (!this.#card) return;

    // Optimistically prepend the new event before awaiting the backend
    const optimisticId = crypto.randomUUID();
    const optimisticEvent: MaintenanceCardEventView = {
      id: optimisticId,
      datePerformed: args.datePerformed,
      maintenanceType: null,
      notes: args.notes
    };
    this.#card = {
      ...this.#card,
      events: [optimisticEvent, ...this.#card.events]
    };

    const result = await safeInvoke<null>('add_maintenance_event', { input: args });
    if (!result.ok) {
      // Rollback optimistic update
      if (this.#card) {
        this.#card = {
          ...this.#card,
          events: this.#card.events.filter((e) => e.id !== optimisticId)
        };
      }
      throw new Error(result.error.message);
    }

    // Reload card to get the server-updated state (next_maintenance_date etc.)
    await this.loadCard(this.#card?.id ?? args.maintenanceCardId);
  }
}

export function setMaintenanceDetailState(state: MaintenanceDetailState): void {
  setContext(STATE_KEY, state);
}

export function getMaintenanceDetailState(): MaintenanceDetailState {
  const state = getContext<MaintenanceDetailState>(STATE_KEY);
  if (!state) {
    throw new Error(
      'MaintenanceDetailState not found in context. Did you forget to call setMaintenanceDetailState?'
    );
  }
  return state;
}
