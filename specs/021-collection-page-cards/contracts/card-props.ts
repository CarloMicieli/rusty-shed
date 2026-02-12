/**
 * TypeScript Interface Contracts for Collection Page Card Integration
 *
 * Feature: 021-collection-page-cards
 * Date: 2026-02-12
 * Phase: Phase 1 - Design & Contracts
 *
 * This file defines the TypeScript interfaces and types required for
 * integrating RailwayModelCard and RailwayModelPreviewCard components
 * into the collection page.
 */

/**
 * Data structure required by RailwayModelPreviewCard component.
 *
 * Source: src/lib/components/RailwayModelPreviewCard.svelte
 */
export interface RailwayModelCardData {
  /** Unique identifier for the model */
  id: string;

  /** Manufacturer name (e.g., "A.C.M.E.", "Märklin") */
  manufacturer: string | null;

  /** Manufacturer's product code (e.g., "1236", "H0-2047") */
  productCode: string | null;

  /** Series designation (e.g., "Class 140", "BR 01") */
  series: string | null;

  /** Model category for classification and placeholder icon selection */
  category: ModelCategory;

  /** Road number / identification marking (e.g., "50 80 26-81 517-7") */
  roadNumber: string | null;

  /** Model scale (e.g., "H0", "N", "TT", "Z") */
  scale: string | null;

  /** Power method (e.g., "DC", "AC", "DCC") */
  powerMethod: string | null;

  /** Historical era classification (e.g., "III", "IV", "V") */
  era: string | null;

  /** Purchase date in ISO 8601 format (YYYY-MM-DD) */
  purchaseDate: string | null;

  /** URL or path to model photo/image */
  photoUrl: string | null;

  /** Number of units in the set (null for single-unit models) */
  unitCount: number | null;

  /** Digital features available on the model */
  digitalFeatures: DigitalFeature[];
}

/**
 * Digital features that can be displayed as overlay badges
 * on the model thumbnail.
 */
export type DigitalFeature =
  | 'Sound' // Sound module installed (speaker icon)
  | 'DCC' // Digital Command Control (bolt icon)
  | 'Smoke' // Smoke generator (currently not detected)
  | 'Light'; // Lighting features (headlights, interior lights)

/**
 * Model categories for placeholder icon selection when photoUrl is null.
 *
 * Icon mapping:
 * - SteamLocomotive → Train icon
 * - ElectricLocomotive, DieselLocomotive → Zap icon
 * - Wagon, FreightCar → Box icon
 * - PassengerCar → Users icon
 * - Railcar, TrainSet → Layers icon
 * - Unknown → Train icon (fallback)
 */
export type ModelCategory =
  | 'SteamLocomotive'
  | 'ElectricLocomotive'
  | 'DieselLocomotive'
  | 'Wagon'
  | 'PassengerCar'
  | 'FreightCar'
  | 'Railcar'
  | 'TrainSet'
  | 'Unknown';

/**
 * Backend category enum values (from Rust).
 * Stored as SCREAMING_SNAKE_CASE in the database.
 */
export type Category =
  | 'LOCOMOTIVES'
  | 'TRAIN_SETS'
  | 'STARTER_SETS'
  | 'FREIGHT_CARS'
  | 'PASSENGER_CARS'
  | 'ELECTRIC_MULTIPLE_UNITS'
  | 'RAILCARS';

/**
 * Locomotive type refinement for LOCOMOTIVES category.
 * Used to determine specific locomotive icon (Steam vs Diesel vs Electric).
 */
export type LocomotiveType = 'STEAM_LOCOMOTIVE' | 'DIESEL_LOCOMOTIVE' | 'ELECTRIC_LOCOMOTIVE';

/**
 * Control type indicating decoder installation status.
 * Source: rolling_stocks.control field
 */
export type Control =
  | 'DCC_READY' // Can accept decoder (socket present)
  | 'DCC_FITTED' // Decoder installed (no sound)
  | 'DCC_SOUND' // Decoder with sound module installed
  | 'NO_DCC'; // No DCC support

/**
 * Feature flag for technical specifications (lights, interior lights, etc.)
 */
export type FeatureFlag =
  | 'YES' // Feature is present
  | 'NO' // Feature is absent
  | 'NOT_APPLICABLE'; // Feature not relevant for this type

/**
 * Purchase information discriminated union.
 * Different purchase states have different data shapes.
 */
export type PurchaseInfo =
  | { type: 'purchased'; data: PurchasedData }
  | { type: 'sold'; data: SoldData }
  | { type: 'preOrdered'; data: PreOrderedData };

export interface PurchasedData {
  purchaseDate: string; // ISO 8601 (YYYY-MM-DD)
  retailer: string | null;
  price: Money | null;
}

export interface SoldData {
  soldDate: string;
  buyer: string | null;
  price: Money | null;
}

export interface PreOrderedData {
  expectedDate: string;
  purchaseDate: string | null;
  retailer: string | null;
}

export interface Money {
  amount: number; // Cents (e.g., 2500 = $25.00)
  currency: string; // ISO 4217 (e.g., "USD", "EUR")
}

/**
 * Simplified collection item view structure.
 * Full structure available in TypeScript bindings.
 */
export interface CollectionItemView {
  id: string;
  addedDate: string;
  notes: string | null;
  railwayModel: CollectionRailwayModel;
  purchaseInfo: PurchaseInfo | null;
  rollingStocks: OwnedRollingStockView[];
}

export interface CollectionRailwayModel {
  railwayModelId: string;
  manufacturer: string | null;
  productCode: string | null;
  scale: string;
  epoch: string;
  description: string;
  category: Category;
}

export interface OwnedRollingStockView {
  id: string;
  roadNumber: string | null;
  control: Control | null;
  digital: DigitalSetup | null;
  // Discriminated union for rolling stock type
  locomotive?: LocomotiveData;
  electricMultipleUnit?: ElectricMultipleUnitData;
  railcar?: RailcarData;
  freightCar?: FreightCarData;
  passengerCar?: PassengerCarData;
}

export interface LocomotiveData {
  locomotiveType: LocomotiveType;
  technical_specifications: TechnicalSpecifications | null;
}

export interface ElectricMultipleUnitData {
  technical_specifications: TechnicalSpecifications | null;
}

export interface RailcarData {
  technical_specifications: TechnicalSpecifications | null;
}

export interface FreightCarData {
  // No technical specifications
}

export interface PassengerCarData {
  // No technical specifications
}

export interface TechnicalSpecifications {
  lights: FeatureFlag | null;
  interior_lights: FeatureFlag | null;
}

export interface DigitalSetup {
  interface: DccInterface;
  dcc_address: number;
  installed_decoder_id: string | null;
}

export type DccInterface =
  | 'NEM_651'
  | 'NEM_652'
  | 'NEM_654'
  | 'PLUX_8'
  | 'PLUX_12'
  | 'PLUX_16'
  | 'PLUX_22'
  | 'NEXT_18'
  | 'NEXT_18_S'
  | 'MTC_21';
