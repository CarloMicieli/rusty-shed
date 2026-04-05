import type { Category, PowerMethod, RecordAcquisitionArgs, Scale } from '$lib/bindings';

/** Session state for the entire acquisition drawer. */
export interface AcquisitionFormState {
  sellerId: string | null;
  purchaseDate: string; // YYYY-MM-DD, defaults to today
  batchDefaults: BatchDefaults;
  items: AcquisitionItemEntry[];
}

export interface BatchDefaults {
  scale: Scale | null;
  powerMethod: PowerMethod | null;
}

export interface AcquisitionItemEntry {
  uid: string; // crypto.randomUUID()
  manufacturerId: string | null;
  productCode: string;
  description: string;
  category: Category | null;
  epoch: string | null;
  priceAmount: number | null;
}

export interface AcquisitionItemErrors {
  manufacturerId?: string;
  productCode?: string;
  category?: string;
}

export interface AcquisitionValidationErrors {
  general?: string;
  items?: AcquisitionItemErrors[];
}

export function createDefaultItem(): AcquisitionItemEntry {
  return {
    uid: crypto.randomUUID(),
    manufacturerId: null,
    productCode: '',
    description: '',
    category: null,
    epoch: null,
    priceAmount: null
  };
}

export function createDefaultFormState(
  defaults: Partial<BatchDefaults> = {}
): AcquisitionFormState {
  const batchDefaults: BatchDefaults = {
    scale: defaults.scale ?? null,
    powerMethod: defaults.powerMethod ?? null
  };
  return {
    sellerId: null,
    purchaseDate: new Date().toISOString().split('T')[0],
    batchDefaults,
    items: [createDefaultItem()]
  };
}

export function toRecordAcquisitionArgs(
  f: AcquisitionFormState,
  currency: string
): RecordAcquisitionArgs {
  return {
    sellerId: f.sellerId,
    purchaseDate: f.purchaseDate,
    items: f.items.map((item) => ({
      manufacturerId: item.manufacturerId!,
      productCode: item.productCode,
      description: item.description,
      category: item.category!,
      scale: f.batchDefaults.scale ?? '',
      epoch: item.epoch ?? '',
      powerMethod: f.batchDefaults.powerMethod ?? '',
      priceAmount: item.priceAmount != null ? Number(item.priceAmount) : Number(0),
      priceCurrency: currency
    }))
  };
}
