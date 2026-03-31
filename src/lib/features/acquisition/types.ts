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
  scale: Scale | null;
  epoch: string | null;
  powerMethod: PowerMethod | null;
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

export function createDefaultItem(defaults: BatchDefaults): AcquisitionItemEntry {
  return {
    uid: crypto.randomUUID(),
    manufacturerId: null,
    productCode: '',
    description: '',
    category: null,
    scale: defaults.scale,
    epoch: null,
    powerMethod: defaults.powerMethod,
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
    items: [createDefaultItem(batchDefaults)]
  };
}

export function validateForm(form: AcquisitionFormState): AcquisitionValidationErrors {
  const errors: AcquisitionValidationErrors = {};
  if (form.items.length === 0) {
    errors.general = 'Add at least one item before saving.';
    return errors;
  }
  const itemErrors: AcquisitionItemErrors[] = form.items.map((item) => {
    const e: AcquisitionItemErrors = {};
    if (!item.manufacturerId) e.manufacturerId = 'Manufacturer is required';
    if (!item.productCode.trim()) e.productCode = 'Product code is required';
    if (!item.category) e.category = 'Category is required';
    return e;
  });
  if (itemErrors.some((e) => Object.keys(e).length > 0)) {
    errors.items = itemErrors;
  }
  return errors;
}

export function hasErrors(errors: AcquisitionValidationErrors): boolean {
  return !!errors.general || (errors.items?.some((e) => Object.keys(e).length > 0) ?? false);
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
      scale: item.scale ?? '',
      epoch: item.epoch ?? '',
      powerMethod: item.powerMethod ?? '',
      priceAmount: item.priceAmount != null ? Number(item.priceAmount) : Number(0),
      priceCurrency: currency
    }))
  };
}
