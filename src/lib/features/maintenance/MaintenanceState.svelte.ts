/**
 * Maintenance State - Reactive state management for maintenance feature.
 *
 * This state manages:
 * - Loading and error states
 * - Maintenance cards list
 * - CRUD operations for cards and events
 */

import { setContext, getContext } from 'svelte';
import { MaintenanceService } from './services/MaintenanceService';
import type { MaintenanceCardView, OwnedRollingStockId, AddMaintenanceArgs } from '$lib/bindings';

// ─────────────────────────────────────────────────────────────
// CONTEXT KEY (for Dependency Injection)
// ─────────────────────────────────────────────────────────────
const STATE_KEY = Symbol('maintenance-state');

// ─────────────────────────────────────────────────────────────
// STATE CLASS
// ─────────────────────────────────────────────────────────────
export default class MaintenanceState {
  // Private reactive state
  #cards = $state<MaintenanceCardView[]>([]);
  #isLoading = $state(false);
  #error = $state<string | null>(null);

  // Service dependency
  #service: MaintenanceService;

  constructor(service?: MaintenanceService) {
    this.#service = service ?? new MaintenanceService();
  }

  // Public readonly getters
  get cards(): MaintenanceCardView[] {
    return this.#cards;
  }

  get isLoading(): boolean {
    return this.#isLoading;
  }

  get error(): string | null {
    return this.#error;
  }

  get hasCards(): boolean {
    return this.#cards.length > 0;
  }

  // ─────────────────────────────────────────────────────────────
  // PUBLIC METHODS
  // ─────────────────────────────────────────────────────────────

  /**
   * Load maintenance dashboard (top 10 due/overdue cards).
   */
  async loadDashboard(): Promise<void> {
    this.#error = null;

    // In a local-first Tauri app, DB reads are often < 10ms.
    // Defer the loading spinner to avoid flashing it for fast responses.
    const loadingTimeout = setTimeout(() => {
      this.#isLoading = true;
    }, 100);

    try {
      this.#cards = await this.#service.getDashboard();
    } catch (err) {
      const error = err as Error;
      this.#error = error.message;
      console.error('[MaintenanceState] Failed to load dashboard:', err);
    } finally {
      clearTimeout(loadingTimeout);
      this.#isLoading = false;
    }
  }

  /**
   * Create a new maintenance card for owned rolling stock.
   *
   * @param ownedRollingStockId - The ID of the owned rolling stock
   */
  async createMaintenanceCard(ownedRollingStockId: OwnedRollingStockId): Promise<void> {
    try {
      await this.#service.createCard(ownedRollingStockId);
      // Refresh the dashboard to show the new card
      await this.loadDashboard();
    } catch (err) {
      const error = err as Error;
      console.error('[MaintenanceState] Failed to create maintenance card:', err);
      throw error;
    }
  }

  /**
   * Add a maintenance event and update the card.
   *
   * @param args - The maintenance event arguments
   */
  async addMaintenanceEvent(args: AddMaintenanceArgs): Promise<void> {
    try {
      await this.#service.addEvent(args);
      // Refresh the dashboard to show updated due dates
      await this.loadDashboard();
    } catch (err) {
      const error = err as Error;
      console.error('[MaintenanceState] Failed to add maintenance event:', err);
      throw error;
    }
  }

  /**
   * Retry loading the dashboard after an error.
   */
  async retry(): Promise<void> {
    await this.loadDashboard();
  }
}

// ─────────────────────────────────────────────────────────────
// CONTEXT HELPERS
// ─────────────────────────────────────────────────────────────

/**
 * Set the MaintenanceState in the Svelte context.
 */
export function setMaintenanceState(state: MaintenanceState): void {
  setContext(STATE_KEY, state);
}

/**
 * Get the MaintenanceState from the Svelte context.
 */
export function getMaintenanceState(): MaintenanceState {
  const state = getContext<MaintenanceState>(STATE_KEY);
  if (!state) {
    throw new Error(
      'MaintenanceState not found in context. Did you forget to call setMaintenanceState?'
    );
  }
  return state;
}
