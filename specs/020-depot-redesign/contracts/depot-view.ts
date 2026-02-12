/**
 * TypeScript Type Contracts: Depot View
 *
 * Auto-generated from Rust types via specta/tauri-specta.
 * These types represent the expected structure after backend changes.
 *
 * File: specs/020-depot-redesign/contracts/depot-view.ts
 * Date: 2026-02-12
 * Status: Contract Definition (not yet generated)
 */

/**
 * Rolling stock categories in the depot
 */
export enum RollingStockCategory {
  /** Steam, diesel, and electric locomotives */
  Locomotive = 'Locomotive',
  /** Self-propelled electric multiple units */
  ElectricMultipleUnit = 'ElectricMultipleUnit',
  /** Self-propelled diesel railcars and DMUs */
  Railcar = 'Railcar',
  /** Unpowered passenger coaches */
  PassengerCar = 'PassengerCar',
  /** Unpowered freight vehicles */
  FreightCar = 'FreightCar'
}

/**
 * Control method for powered rolling stock
 */
export enum Control {
  /** Analog/DC control */
  Analogue = 'Analogue',
  /** DCC decoder fitted */
  DccFitted = 'DccFitted',
  /** DCC with sound decoder */
  DccSound = 'DccSound'
}

/**
 * Read-only view of an owned rolling stock item in the depot
 *
 * Generated from: src-tauri/src/collecting/domain/depot_view.rs
 */
export interface DepotRollingStockView {
  /** Unique identifier for this owned rolling stock instance */
  id: string;

  /** Series or class code (e.g., "Class 103", "BR 01") */
  seriesCode: string;

  /** Optional road/running number for identification */
  roadNumber: string | null;

  /** Optional user-assigned friendly name */
  friendlyName: string | null;

  /** Optional depot or storage location */
  depot: string | null;

  /** Category classification */
  category: RollingStockCategory;

  /** Manufacturer display name (e.g., "Hornby", "Roco") */
  manufacturerName: string;

  /** Product code/catalog number */
  productCode: string;

  /** Control method (DCC, analogue, etc.) - null for unpowered stock */
  control: Control | null;

  /** Livery or paint scheme description */
  livery: string | null;

  /** Railway company/operator name */
  railwayCompanyName: string | null;

  /**
   * ERA/EPOCH field (NEW in depot redesign)
   *
   * Format examples:
   * - Single: "I", "II", "III", "IV", "V", "VI"
   * - Half: "Ia", "Ib", "IIa", "IIb"
   * - Range: "I/II", "III/IV"
   * - Museum: "Vm"
   *
   * Null for models without era information.
   */
  epoch: string | null;
}

/**
 * Complete depot view containing all owned rolling stock
 *
 * Generated from: src-tauri/src/collecting/domain/depot_view.rs
 */
export interface DepotView {
  /** List of all owned rolling stock items in the depot */
  rollingStocks: DepotRollingStockView[];
}

/**
 * Type guard to check if a value is a valid RollingStockCategory
 */
export function isRollingStockCategory(value: string): value is RollingStockCategory {
  return Object.values(RollingStockCategory).includes(value as RollingStockCategory);
}

/**
 * Type guard to check if a value is a valid Control method
 */
export function isControl(value: string): value is Control {
  return Object.values(Control).includes(value as Control);
}
