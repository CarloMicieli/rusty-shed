import type {
  CollectionItemView,
  OwnedRollingStockView,
  Category,
  PurchaseInfo,
  MonetaryAmount
} from '$lib/bindings';

import type {
  RailwayModelCardData,
  ModelCategory,
  DigitalFeature
} from '$lib/components/RailwayModelPreviewCard.svelte';

/**
 * Transform a CollectionItemView into the props shape expected by RailwayModelPreviewCard.
 */
export function collectionItemToCardData(item: CollectionItemView): RailwayModelCardData {
  const { railwayModel, purchaseInfo, rollingStocks } = item;

  return {
    id: railwayModel.railwayModelId,
    manufacturer: railwayModel.manufacturer,
    productCode: railwayModel.productCode,
    description: railwayModel.description,
    // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
    category: mapCategory(railwayModel.category),
    scale: railwayModel.scale,
    powerMethod: railwayModel.powerMethod,
    condition: item.purchaseCondition,
    era: railwayModel.epoch,
    purchaseDate: extractPurchaseDate(purchaseInfo),
    price: extractPurchasePrice(purchaseInfo),
    soldDate: extractSoldDate(purchaseInfo, item.removedDate),
    isSold: isSoldItem(purchaseInfo, item.removedDate),
    photoUrl: null,
    unitCount: rollingStocks.length > 1 ? rollingStocks.length : null,
    digitalFeatures: extractDigitalFeatures(rollingStocks)
  };
}

export function isSoldItem(purchaseInfo: PurchaseInfo | null, removedDate: string | null): boolean {
  return purchaseInfo?.kind === 'sold' || removedDate !== null;
}

/**
 * Map a backend Category enum value to the frontend ModelCategory type.
 *
 * OwnedRollingStockView does not carry locomotive-subtype information,
 * so LOCOMOTIVES always maps to 'SteamLocomotive' as a safe default.
 */
export function mapCategory(category: Category | null): ModelCategory {
  if (!category) return 'Unknown';

  switch (category) {
    case 'FREIGHT_CARS':
      return 'FreightCar';
    case 'PASSENGER_CARS':
      return 'PassengerCar';
    case 'TRAIN_SETS':
    case 'STARTER_SETS':
    case 'ELECTRIC_MULTIPLE_UNITS':
      return 'TrainSet';
    case 'RAILCARS':
      return 'Railcar';
    case 'LOCOMOTIVES':
      return 'SteamLocomotive';
    default:
      return 'Unknown';
  }
}

/**
 * Extract digital feature flags from a list of owned rolling stock.
 *
 * Detection rules:
 * - Sound: control === 'DCC_SOUND'
 * - DCC:   control is set and not 'NO_DCC', OR digital setup present
 */
export function extractDigitalFeatures(rollingStocks: OwnedRollingStockView[]): DigitalFeature[] {
  const features = new Set<DigitalFeature>();

  for (const unit of rollingStocks) {
    if (unit.control === 'DCC_SOUND') {
      features.add('Sound');
    }

    if ((unit.control && unit.control !== 'NO_DCC') || unit.digital !== null) {
      features.add('DCC');
    }
  }

  return Array.from(features);
}

/**
 * Extract the purchase date from the PurchaseInfo discriminated union.
 *
 * Returns null for sold items, pre-orders without a date, or missing info.
 */
export function extractPurchaseDate(purchaseInfo: PurchaseInfo | null): string | null {
  if (!purchaseInfo) return null;

  switch (purchaseInfo.kind) {
    case 'purchased':
      return purchaseInfo.data.purchaseDate;
    case 'preOrdered':
      return purchaseInfo.data.orderDate ?? null;
    case 'sold':
      return null;
    default:
      return null;
  }
}

/**
 * Extract sold date from purchase info with removedDate fallback.
 */
export function extractSoldDate(
  purchaseInfo: PurchaseInfo | null,
  removedDate: string | null
): string | null {
  if (purchaseInfo?.kind === 'sold') {
    return purchaseInfo.data.saleDate;
  }

  return removedDate;
}

export function extractPurchasePrice(purchaseInfo: PurchaseInfo | null): MonetaryAmount | null {
  if (!purchaseInfo) return null;

  switch (purchaseInfo.kind) {
    case 'purchased':
      return purchaseInfo.data.price;
    case 'sold':
      return purchaseInfo.data.purchasePrice ?? purchaseInfo.data.salePrice;
    case 'preOrdered':
      return purchaseInfo.data.totalPrice;
    default:
      return null;
  }
}
