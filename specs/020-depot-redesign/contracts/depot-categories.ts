/**
 * Frontend Category Grouping Contracts
 *
 * These types define how backend RollingStockCategory enum values
 * are grouped and displayed in the depot UI.
 *
 * File: specs/020-depot-redesign/contracts/depot-categories.ts
 * Date: 2026-02-12
 * Status: Contract Definition
 */

import type { ComponentType } from 'svelte';
import { RollingStockCategory, type DepotRollingStockView } from './depot-view';

/**
 * UI-specific category keys for depot display
 *
 * Maps to the 4 accordion sections in the redesigned depot page.
 */
export type DepotCategoryKey = 'locomotives' | 'railcarsEmuDmu' | 'passengerCars' | 'freightCars';

/**
 * Category group with items and display metadata
 *
 * Represents a single accordion section in the depot UI.
 */
export interface CategoryGroup {
  /** Unique key for this category group */
  key: DepotCategoryKey;

  /** i18n message key for the category title */
  title: string;

  /** Lucide icon component for the category */
  icon: ComponentType;

  /** Filtered rolling stock items in this category */
  items: DepotRollingStockView[];

  /** Number of items in this category */
  count: number;

  /** i18n message key for empty state */
  emptyMessage: string;

  /** Tone class for styling (e.g., "default", "secondary") */
  toneClass?: string;
}

/**
 * Maps backend RollingStockCategory enum to UI category keys
 *
 * This mapping defines how the 5 backend categories are organized
 * into 4 frontend display categories.
 *
 * Mapping:
 * - Locomotive → locomotives (1:1)
 * - ElectricMultipleUnit + Railcar → railcarsEmuDmu (2:1)
 * - PassengerCar → passengerCars (1:1)
 * - FreightCar → freightCars (1:1)
 */
export const CATEGORY_MAPPING: Record<RollingStockCategory, DepotCategoryKey> = {
  [RollingStockCategory.Locomotive]: 'locomotives',
  [RollingStockCategory.ElectricMultipleUnit]: 'railcarsEmuDmu',
  [RollingStockCategory.Railcar]: 'railcarsEmuDmu',
  [RollingStockCategory.PassengerCar]: 'passengerCars',
  [RollingStockCategory.FreightCar]: 'freightCars'
};

/**
 * Inverted mapping: UI category to backend categories
 *
 * Useful for filtering or validation.
 */
export const CATEGORY_TO_ENUM: Record<DepotCategoryKey, RollingStockCategory[]> = {
  locomotives: [RollingStockCategory.Locomotive],
  railcarsEmuDmu: [RollingStockCategory.ElectricMultipleUnit, RollingStockCategory.Railcar],
  passengerCars: [RollingStockCategory.PassengerCar],
  freightCars: [RollingStockCategory.FreightCar]
};

/**
 * Helper function to map a rolling stock item to its UI category
 */
export function getCategoryKey(item: DepotRollingStockView): DepotCategoryKey {
  return CATEGORY_MAPPING[item.category];
}

/**
 * Helper function to group rolling stock by category
 */
export function groupByCategory(
  items: DepotRollingStockView[]
): Record<DepotCategoryKey, DepotRollingStockView[]> {
  const grouped: Record<DepotCategoryKey, DepotRollingStockView[]> = {
    locomotives: [],
    railcarsEmuDmu: [],
    passengerCars: [],
    freightCars: []
  };

  for (const item of items) {
    const categoryKey = getCategoryKey(item);
    grouped[categoryKey].push(item);
  }

  return grouped;
}

/**
 * Helper function to check if all backend categories are mapped
 *
 * Use this in tests to ensure no categories are missing from CATEGORY_MAPPING.
 */
export function validateCategoryMapping(): boolean {
  const backendCategories = Object.values(RollingStockCategory);
  const mappedCategories = Object.keys(CATEGORY_MAPPING);

  return backendCategories.every((cat) => mappedCategories.includes(cat));
}

/**
 * i18n message keys for category titles and empty states
 */
export const CATEGORY_I18N_KEYS = {
  locomotives: {
    title: 'depot_locomotives_title',
    empty: 'depot_empty_locomotives'
  },
  railcarsEmuDmu: {
    title: 'depot_railcars_and_emu_title',
    empty: 'depot_empty_railcars_and_emu'
  },
  passengerCars: {
    title: 'depot_passenger_cars_title',
    empty: 'depot_empty_passenger_cars'
  },
  freightCars: {
    title: 'depot_freight_cars_title',
    empty: 'depot_empty_freight_cars'
  }
} as const;

/**
 * Type for valid i18n key lookups
 */
export type CategoryI18nKey = keyof typeof CATEGORY_I18N_KEYS;

/**
 * Helper to get i18n keys for a category
 */
export function getCategoryI18nKeys(category: DepotCategoryKey): { title: string; empty: string } {
  return CATEGORY_I18N_KEYS[category];
}
