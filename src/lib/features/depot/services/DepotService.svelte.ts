/**
 * Depot Service - Manages depot/rolling stock state and operations.
 *
 * This service provides:
 * - Depot data fetching
 * - Filtering and search
 * - View mode management (table/grid)
 * - Rolling stock categorization
 */

import { setContext, getContext } from 'svelte';
import { safeInvoke } from '$lib/shared/services/TauriAdapter';
import { getErrorMessage } from '$lib/shared/domain/errors';
import type { DepotView, DepotRollingStockView } from '$lib/bindings';
import type { Locomotive, TrainSet, Car } from '../types';

// ─────────────────────────────────────────────────────────────
// CONTEXT KEY (for Dependency Injection)
// ─────────────────────────────────────────────────────────────
const SERVICE_KEY = Symbol('depot-service');

// ─────────────────────────────────────────────────────────────
// SERVICE CLASS
// ─────────────────────────────────────────────────────────────
export class DepotService {
  // Private reactive state
  #depot = $state<DepotView | null>(null);
  #isLoading = $state(false);
  #error = $state<string | null>(null);
  #query = $state('');
  #viewMode = $state<'table' | 'grid'>('table');

  // Public readonly getters (defensive encapsulation)
  get isLoading(): boolean {
    return this.#isLoading;
  }

  get error(): string | null {
    return this.#error;
  }

  get query(): string {
    return this.#query;
  }

  get viewMode(): 'table' | 'grid' {
    return this.#viewMode;
  }

  // Derived state - Categorized rolling stock
  locomotives = $derived.by(() => {
    if (!this.#depot) return [];
    return this.#depot.rollingStocks
      .filter((item) => item.category === 'LOCOMOTIVE')
      .map((item) => this.#mapToLocomotive(item));
  });

  trains = $derived.by(() => {
    if (!this.#depot) return [];
    return this.#depot.rollingStocks
      .filter((item) => item.category === 'ELECTRIC_MULTIPLE_UNIT' || item.category === 'RAILCAR')
      .map((item) => this.#mapToTrainSet(item));
  });

  cars = $derived.by(() => {
    if (!this.#depot) return [];
    return this.#depot.rollingStocks
      .filter((item) => item.category === 'PASSENGER_CAR' || item.category === 'FREIGHT_CAR')
      .map((item) => this.#mapToCar(item));
  });

  // Filtered items
  filteredLocomotives = $derived.by(() => {
    const q = this.#query.trim().toLowerCase();
    if (!q) return this.locomotives;
    return this.locomotives.filter((item) => this.#filterMatch(item, q));
  });

  filteredTrains = $derived.by(() => {
    const q = this.#query.trim().toLowerCase();
    if (!q) return this.trains;
    return this.trains.filter((item) => this.#filterMatch(item, q));
  });

  filteredCars = $derived.by(() => {
    const q = this.#query.trim().toLowerCase();
    if (!q) return this.cars;
    return this.cars.filter((item) => this.#filterMatch(item, q));
  });

  totalFiltered = $derived.by(
    () => this.filteredLocomotives.length + this.filteredTrains.length + this.filteredCars.length
  );

  // ─────────────────────────────────────────────────────────────
  // PRIVATE HELPERS
  // ─────────────────────────────────────────────────────────────

  #filterMatch(item: Locomotive | TrainSet | Car, query: string): boolean {
    const fields = [
      item.roadNumber,
      item.railwayCompany,
      'group' in item ? item.group : null,
      'type' in item ? item.type : null,
      item.livery,
      item.productCode,
      'seriesCode' in item ? item.seriesCode : null,
      'control' in item ? item.control : null,
      'serviceLevel' in item ? item.serviceLevel : null,
      item.dccAddress
    ];
    return fields.some((field) => {
      if (field === null || field === undefined) return false;
      return String(field).toLowerCase().includes(query);
    });
  }

  #formatCategory(cat: string): string {
    return cat
      .replace(/_/g, ' ')
      .toLowerCase()
      .replace(/\b\w/g, (c) => c.toUpperCase());
  }

  #mapToLocomotive(item: DepotRollingStockView): Locomotive {
    return {
      id: item.id,
      railwayModelId: item.railwayModelId,
      group: item.friendlyName ?? item.seriesCode,
      manufacturer: item.manufacturerName,
      seriesCode: item.seriesCode,
      productCode: item.productCode,
      categoryLabel: this.#formatCategory(item.category),
      roadNumber: item.roadNumber,
      railwayCompany: item.railwayCompanyName,
      livery: item.livery,
      control: item.control,
      dccAddress: item.dccAddress
    };
  }

  #mapToTrainSet(item: DepotRollingStockView): TrainSet {
    return {
      id: item.id,
      railwayModelId: item.railwayModelId,
      group: item.friendlyName ?? item.seriesCode,
      manufacturer: item.manufacturerName,
      seriesCode: item.seriesCode,
      productCode: item.productCode,
      categoryLabel: this.#formatCategory(item.category),
      roadNumber: item.roadNumber,
      railwayCompany: item.railwayCompanyName,
      livery: item.livery,
      control: item.control,
      dccAddress: item.dccAddress
    };
  }

  #mapToCar(item: DepotRollingStockView): Car {
    return {
      id: item.id,
      railwayModelId: item.railwayModelId,
      type: item.friendlyName ?? item.seriesCode,
      manufacturer: item.manufacturerName,
      seriesCode: item.seriesCode,
      productCode: item.productCode,
      categoryLabel: this.#formatCategory(item.category),
      roadNumber: item.roadNumber,
      railwayCompany: item.railwayCompanyName,
      livery: item.livery,
      category: item.category === 'PASSENGER_CAR' ? 'passenger' : 'freight',
      serviceLevel: null,
      control: item.control,
      dccAddress: item.dccAddress
    };
  }

  // ─────────────────────────────────────────────────────────────
  // USE CASES (Public Methods)
  // ─────────────────────────────────────────────────────────────

  /**
   * Load depot data from the backend.
   */
  async load(): Promise<void> {
    this.#isLoading = true;
    this.#error = null;

    try {
      const result = await safeInvoke<DepotView>('get_depot');
      if (result.ok) {
        this.#depot = result.data;
      } else {
        this.#error = getErrorMessage(result.error);
      }
    } catch (err) {
      this.#error = err instanceof Error ? err.message : 'Unknown error loading depot';
    } finally {
      this.#isLoading = false;
    }
  }

  /**
   * Set the search query.
   *
   * @param query - The search query
   */
  setQuery(query: string): void {
    this.#query = query;
  }

  /**
   * Set the view mode (table or grid).
   *
   * @param mode - The view mode
   */
  setViewMode(mode: 'table' | 'grid'): void {
    this.#viewMode = mode;
  }

  /**
   * Clear the search query.
   */
  clearQuery(): void {
    this.#query = '';
  }
}

// ─────────────────────────────────────────────────────────────
// CONTEXT HELPERS (Dependency Injection)
// ─────────────────────────────────────────────────────────────

/**
 * Initialize and set the DepotService in the current context.
 *
 * @param service - Optional service instance (for testing)
 * @returns The service instance
 */
export function setDepotService(service?: DepotService): DepotService {
  const instance = service ?? new DepotService();
  setContext(SERVICE_KEY, instance);
  return instance;
}

/**
 * Get the DepotService from the current context.
 *
 * @returns The service instance
 * @throws Error if service is not found in context
 */
export function getDepotService(): DepotService {
  const service = getContext<DepotService>(SERVICE_KEY);
  if (!service) {
    throw new Error(
      'DepotService not found in context. Did you call setDepotService() in a parent component?'
    );
  }
  return service;
}

// ─────────────────────────────────────────────────────────────
// LEGACY COMPATIBILITY (to be removed after migration)
// ─────────────────────────────────────────────────────────────

/**
 * @deprecated Use setDepotService() instead
 */
export function createDepotState(): DepotService {
  console.warn('createDepotState is deprecated. Use setDepotService() instead.');
  return new DepotService();
}

/**
 * @deprecated Use setDepotService() instead
 */
export function setDepotContext(state: DepotService): void {
  console.warn('setDepotContext is deprecated. Use setDepotService() instead.');
  setContext(SERVICE_KEY, state);
}

/**
 * @deprecated Use getDepotService() instead
 */
export function getDepotContext(): DepotService {
  console.warn('getDepotContext is deprecated. Use getDepotService() instead.');
  return getDepotService();
}
