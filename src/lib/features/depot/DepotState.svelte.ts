import { setContext, getContext } from 'svelte';
import { safeInvoke, getErrorMessage } from '$lib/services';
import type { DepotView, DepotRollingStockView } from '$lib/bindings';
import type { Locomotive, TrainSet, Car } from './types';

export class DepotState {
  #depot = $state<DepotView | null>(null);
  #isLoading = $state(false);
  #error = $state<string | null>(null);
  #query = $state('');
  #viewMode = $state<'table' | 'grid'>('table');

  get isLoading() {
    return this.#isLoading;
  }

  get error() {
    return this.#error;
  }

  get query() {
    return this.#query;
  }

  get viewMode() {
    return this.#viewMode;
  }

  locomotives = $derived.by(() => {
    if (!this.#depot) return [];
    return this.#depot.rollingStocks
      .filter((item) => item.category === 'LOCOMOTIVE')
      .map((item) => this.mapToLocomotive(item));
  });

  trains = $derived.by(() => {
    if (!this.#depot) return [];
    return this.#depot.rollingStocks
      .filter((item) => item.category === 'ELECTRIC_MULTIPLE_UNIT' || item.category === 'RAILCAR')
      .map((item) => this.mapToTrainSet(item));
  });

  cars = $derived.by(() => {
    if (!this.#depot) return [];
    return this.#depot.rollingStocks
      .filter((item) => item.category === 'PASSENGER_CAR' || item.category === 'FREIGHT_CAR')
      .map((item) => this.mapToCar(item));
  });

  filteredLocomotives = $derived.by(() => {
    const q = this.#query.trim().toLowerCase();
    if (!q) return this.locomotives;
    return this.locomotives.filter((item) => this.filterMatch(item, q));
  });

  filteredTrains = $derived.by(() => {
    const q = this.#query.trim().toLowerCase();
    if (!q) return this.trains;
    return this.trains.filter((item) => this.filterMatch(item, q));
  });

  filteredCars = $derived.by(() => {
    const q = this.#query.trim().toLowerCase();
    if (!q) return this.cars;
    return this.cars.filter((item) => this.filterMatch(item, q));
  });

  totalFiltered = $derived.by(
    () => this.filteredLocomotives.length + this.filteredTrains.length + this.filteredCars.length
  );

  private filterMatch(item: Locomotive | TrainSet | Car, query: string): boolean {
    const fields = [
      item.roadNumber,
      item.railwayCompany,
      'group' in item ? item.group : null,
      'type' in item ? item.type : null,
      item.livery,
      item.productCode,
      'seriesCode' in item ? item.seriesCode : null,
      'control' in item ? item.control : null,
      'serviceLevel' in item ? item.serviceLevel : null
    ];
    return fields.some((field) => {
      if (field === null || field === undefined) return false;
      return String(field).toLowerCase().includes(query);
    });
  }

  private formatCategory(cat: string): string {
    return cat
      .replace(/_/g, ' ')
      .toLowerCase()
      .replace(/\b\w/g, (c) => c.toUpperCase());
  }

  private mapToLocomotive(item: DepotRollingStockView): Locomotive {
    return {
      id: item.id,
      group: item.friendlyName ?? item.seriesCode,
      manufacturer: item.manufacturerName,
      seriesCode: item.seriesCode,
      productCode: item.productCode,
      categoryLabel: this.formatCategory(item.category),
      roadNumber: item.roadNumber,
      railwayCompany: item.railwayCompanyName,
      livery: item.livery,
      control: item.control
    };
  }

  private mapToTrainSet(item: DepotRollingStockView): TrainSet {
    return {
      id: item.id,
      group: item.friendlyName ?? item.seriesCode,
      manufacturer: item.manufacturerName,
      seriesCode: item.seriesCode,
      productCode: item.productCode,
      categoryLabel: this.formatCategory(item.category),
      roadNumber: item.roadNumber,
      railwayCompany: item.railwayCompanyName,
      livery: item.livery,
      control: item.control
    };
  }

  private mapToCar(item: DepotRollingStockView): Car {
    return {
      id: item.id,
      type: item.friendlyName ?? item.seriesCode,
      manufacturer: item.manufacturerName,
      seriesCode: item.seriesCode,
      productCode: item.productCode,
      categoryLabel: this.formatCategory(item.category),
      roadNumber: item.roadNumber,
      railwayCompany: item.railwayCompanyName,
      livery: item.livery,
      category: item.category === 'PASSENGER_CAR' ? 'passenger' : 'freight',
      serviceLevel: null,
      control: item.control
    };
  }

  async load() {
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

  setQuery(query: string) {
    this.#query = query;
  }

  setViewMode(mode: 'table' | 'grid') {
    this.#viewMode = mode;
  }

  clearQuery() {
    this.#query = '';
  }
}

const DEPOT_CONTEXT_KEY = Symbol('depot-context');

export function createDepotState() {
  return new DepotState();
}

export function setDepotContext(state: DepotState) {
  setContext(DEPOT_CONTEXT_KEY, state);
}

export function getDepotContext(): DepotState {
  const state = getContext<DepotState>(DEPOT_CONTEXT_KEY);
  if (!state) {
    throw new Error(
      'DepotContext not provided. Ensure component is within a DepotContext provider.'
    );
  }
  return state;
}
