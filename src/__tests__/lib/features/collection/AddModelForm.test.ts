import { describe, it, expect } from 'vitest';
import type {
  AddModelFormState,
  RollingStockFormEntry,
  PurchaseFormState
} from '$lib/features/collection/types/AddModelFormTypes';
import type { AddRailwayModelToCollectionArgs } from '$lib/bindings';

// Helper Functions for Form Logic (these will be implemented in the component)

/**
 * Create default form state
 */
function createDefaultFormState(): AddModelFormState {
  return {
    manufacturerId: null,
    productCode: '',
    description: '',
    category: null,
    scale: null,
    powerMethod: null,
    epoch: null,
    rollingStocks: [createDefaultRollingStock()],
    purchase: createDefaultPurchaseState()
  };
}

/**
 * Create default rolling stock entry
 */
function createDefaultRollingStock(): RollingStockFormEntry {
  return {
    uid: crypto.randomUUID(),
    railwayCompanyId: null,
    seriesCode: '',
    category: null,
    roadNumber: '',
    locomotiveType: null
  };
}

/**
 * Create default purchase state
 */
function createDefaultPurchaseState(): PurchaseFormState {
  return {
    sellerId: null,
    priceAmount: '',
    priceCurrency: 'EUR',
    purchaseCondition: null,
    modelCondition: null,
    boxCondition: null,
    notes: '',
    purchaseDate: new Date().toISOString().split('T')[0] // YYYY-MM-DD
  };
}

/**
 * Validate form state
 */
interface ValidationErrors {
  manufacturerId?: string;
  productCode?: string;
  description?: string;
  category?: string;
  scale?: string;
  powerMethod?: string;
  epoch?: string;
  rollingStocks?: string;
  rollingStockErrors?: Array<{
    railwayCompanyId?: string;
    seriesCode?: string;
    category?: string;
  }>;
}

function validateForm(form: AddModelFormState): ValidationErrors {
  const errors: ValidationErrors = {};

  // Railway model validation
  if (!form.manufacturerId) errors.manufacturerId = 'Manufacturer is required';
  if (!form.productCode.trim()) errors.productCode = 'Product code is required';
  if (!form.description.trim()) errors.description = 'Description is required';
  if (!form.category) errors.category = 'Category is required';
  if (!form.scale) errors.scale = 'Scale is required';
  if (!form.powerMethod) errors.powerMethod = 'Power method is required';
  if (!form.epoch) errors.epoch = 'Epoch is required';

  // Rolling stocks validation
  if (form.rollingStocks.length === 0) {
    errors.rollingStocks = 'At least one rolling stock is required';
  } else {
    const rsErrors = form.rollingStocks.map((rs) => {
      const err: { railwayCompanyId?: string; seriesCode?: string; category?: string } = {};
      if (!rs.railwayCompanyId) err.railwayCompanyId = 'Railway company is required';
      if (!rs.seriesCode.trim()) err.seriesCode = 'Series code is required';
      if (!rs.category) err.category = 'Category is required';
      return err;
    });

    if (rsErrors.some((e) => Object.keys(e).length > 0)) {
      errors.rollingStockErrors = rsErrors;
    }
  }

  return errors;
}

/**
 * Transform form state to command args
 */
function toAddRailwayModelArgs(form: AddModelFormState): AddRailwayModelToCollectionArgs {
  const today = new Date().toISOString().split('T')[0];

  // Parse price amount from decimal to cents
  const priceInCents = form.purchase.priceAmount
    ? BigInt(Math.round(parseFloat(form.purchase.priceAmount) * 100))
    : BigInt(0);

  return {
    railwayModel: {
      manufacturerId: form.manufacturerId!,
      productCode: form.productCode,
      description: form.description,
      category: form.category!,
      scale: form.scale!,
      epoch: form.epoch!,
      powerMethod: form.powerMethod!,
      rollingStocks: form.rollingStocks.map((rs) => ({
        railwayCompanyId: rs.railwayCompanyId!,
        seriesCode: rs.seriesCode,
        roadNumber: rs.roadNumber || null,
        locomotiveType: rs.locomotiveType || null,
        category: rs.category!
      }))
    },
    priceAmount: priceInCents,
    priceCurrency: form.purchase.priceCurrency,
    sellerId: form.purchase.sellerId,
    addedDate: today,
    purchaseDate: form.purchase.purchaseDate || today,
    purchaseCondition: form.purchase.purchaseCondition,
    modelCondition: form.purchase.modelCondition,
    boxCondition: form.purchase.boxCondition,
    notes: form.purchase.notes || null
  };
}

// Tests

describe('AddModelForm - Form State Initialization', () => {
  it('should create default form state with one empty rolling stock', () => {
    const form = createDefaultFormState();

    expect(form.manufacturerId).toBeNull();
    expect(form.productCode).toBe('');
    expect(form.description).toBe('');
    expect(form.category).toBeNull();
    expect(form.scale).toBeNull();
    expect(form.powerMethod).toBeNull();
    expect(form.epoch).toBeNull();
    expect(form.rollingStocks).toHaveLength(1);
    expect(form.rollingStocks[0].uid).toBeTruthy();
    expect(form.rollingStocks[0].railwayCompanyId).toBeNull();
    expect(form.rollingStocks[0].seriesCode).toBe('');
  });

  it('should create default purchase state with EUR currency', () => {
    const purchase = createDefaultPurchaseState();

    expect(purchase.sellerId).toBeNull();
    expect(purchase.priceAmount).toBe('');
    expect(purchase.priceCurrency).toBe('EUR');
    expect(purchase.purchaseCondition).toBeNull();
    expect(purchase.modelCondition).toBeNull();
    expect(purchase.boxCondition).toBeNull();
    expect(purchase.notes).toBe('');
    expect(purchase.purchaseDate).toMatch(/^\d{4}-\d{2}-\d{2}$/);
  });

  it('should generate unique UIDs for rolling stock entries', () => {
    const rs1 = createDefaultRollingStock();
    const rs2 = createDefaultRollingStock();

    expect(rs1.uid).not.toBe(rs2.uid);
  });
});

describe('AddModelForm - Validation', () => {
  it('should validate all required railway model fields', () => {
    const form = createDefaultFormState();
    const errors = validateForm(form);

    expect(errors.manufacturerId).toBeDefined();
    expect(errors.productCode).toBeDefined();
    expect(errors.description).toBeDefined();
    expect(errors.category).toBeDefined();
    expect(errors.scale).toBeDefined();
    expect(errors.powerMethod).toBeDefined();
    expect(errors.epoch).toBeDefined();
  });

  it('should validate at least one rolling stock is required', () => {
    const form = createDefaultFormState();
    form.rollingStocks = [];

    const errors = validateForm(form);

    expect(errors.rollingStocks).toBe('At least one rolling stock is required');
  });

  it('should validate rolling stock required fields', () => {
    const form = createDefaultFormState();
    // Rolling stock has empty required fields

    const errors = validateForm(form);

    expect(errors.rollingStockErrors).toBeDefined();
    expect(errors.rollingStockErrors![0].railwayCompanyId).toBeDefined();
    expect(errors.rollingStockErrors![0].seriesCode).toBeDefined();
    expect(errors.rollingStockErrors![0].category).toBeDefined();
  });

  it('should pass validation with complete data', () => {
    const form = createDefaultFormState();
    form.manufacturerId = 'trn:manufacturer:marklin';
    form.productCode = '37858';
    form.description = 'Class 218 Diesel Locomotive';
    form.category = 'LOCOMOTIVES';
    form.scale = 'H0';
    form.powerMethod = 'AC';
    form.epoch = 'IV';
    form.rollingStocks[0].railwayCompanyId = 'trn:railway-company:db';
    form.rollingStocks[0].seriesCode = '218';
    form.rollingStocks[0].category = 'DIESEL_LOCOMOTIVE';

    const errors = validateForm(form);

    expect(Object.keys(errors).length).toBe(0);
  });

  it('should allow optional fields to be empty', () => {
    const form = createDefaultFormState();
    // Set required fields
    form.manufacturerId = 'trn:manufacturer:marklin';
    form.productCode = '37858';
    form.description = 'Class 218';
    form.category = 'LOCOMOTIVES';
    form.scale = 'H0';
    form.powerMethod = 'AC';
    form.epoch = 'IV';
    form.rollingStocks[0].railwayCompanyId = 'trn:railway-company:db';
    form.rollingStocks[0].seriesCode = '218';
    form.rollingStocks[0].category = 'DIESEL_LOCOMOTIVE';
    // roadNumber is optional
    form.rollingStocks[0].roadNumber = '';

    const errors = validateForm(form);

    expect(Object.keys(errors).length).toBe(0);
  });
});

describe('AddModelForm - toAddRailwayModelArgs Transformation', () => {
  it('should transform complete form to command args', () => {
    const form = createDefaultFormState();
    form.manufacturerId = 'trn:manufacturer:marklin';
    form.productCode = '37858';
    form.description = 'Class 218 Diesel Locomotive';
    form.category = 'LOCOMOTIVES';
    form.scale = 'H0';
    form.powerMethod = 'AC';
    form.epoch = 'IV';
    form.rollingStocks[0].railwayCompanyId = 'trn:railway-company:db';
    form.rollingStocks[0].seriesCode = '218';
    form.rollingStocks[0].roadNumber = '218 101-3';
    form.rollingStocks[0].category = 'DIESEL_LOCOMOTIVE';
    form.purchase.priceAmount = '249.99';
    form.purchase.sellerId = 'trn:seller:modellbahnshop';

    const args = toAddRailwayModelArgs(form);

    expect(args.railwayModel.manufacturerId).toBe('trn:manufacturer:marklin');
    expect(args.railwayModel.productCode).toBe('37858');
    expect(args.railwayModel.description).toBe('Class 218 Diesel Locomotive');
    expect(args.railwayModel.category).toBe('LOCOMOTIVES');
    expect(args.railwayModel.scale).toBe('H0');
    expect(args.railwayModel.powerMethod).toBe('AC');
    expect(args.railwayModel.epoch).toBe('IV');
    expect(args.railwayModel.rollingStocks).toHaveLength(1);
    expect(args.railwayModel.rollingStocks[0].railwayCompanyId).toBe('trn:railway-company:db');
    expect(args.railwayModel.rollingStocks[0].seriesCode).toBe('218');
    expect(args.railwayModel.rollingStocks[0].roadNumber).toBe('218 101-3');
    expect(args.railwayModel.rollingStocks[0].category).toBe('DIESEL_LOCOMOTIVE');
    expect(args.priceAmount).toBe(BigInt(24999)); // 249.99 * 100
    expect(args.sellerId).toBe('trn:seller:modellbahnshop');
  });

  it('should convert decimal price to cents correctly', () => {
    const form = createDefaultFormState();
    // Set minimum required fields
    form.manufacturerId = 'test';
    form.productCode = 'test';
    form.description = 'test';
    form.category = 'LOCOMOTIVES';
    form.scale = 'H0';
    form.powerMethod = 'AC';
    form.epoch = 'IV';
    form.rollingStocks[0].railwayCompanyId = 'test';
    form.rollingStocks[0].seriesCode = 'test';
    form.rollingStocks[0].category = 'DIESEL_LOCOMOTIVE';

    // Test various price formats
    form.purchase.priceAmount = '99.99';
    expect(toAddRailwayModelArgs(form).priceAmount).toBe(BigInt(9999));

    form.purchase.priceAmount = '100';
    expect(toAddRailwayModelArgs(form).priceAmount).toBe(BigInt(10000));

    form.purchase.priceAmount = '0.50';
    expect(toAddRailwayModelArgs(form).priceAmount).toBe(BigInt(50));

    form.purchase.priceAmount = '';
    expect(toAddRailwayModelArgs(form).priceAmount).toBe(BigInt(0));
  });

  it('should handle multiple rolling stocks', () => {
    const form = createDefaultFormState();
    form.manufacturerId = 'trn:manufacturer:marklin';
    form.productCode = '42999';
    form.description = 'TEE Train Set';
    form.category = 'TRAIN_SETS';
    form.scale = 'H0';
    form.powerMethod = 'AC';
    form.epoch = 'IV';

    form.rollingStocks = [
      {
        uid: crypto.randomUUID(),
        railwayCompanyId: 'trn:railway-company:db',
        seriesCode: 'TEE',
        category: 'POWER_CAR',
        roadNumber: 'VT 11.5 001',
        locomotiveType: null
      },
      {
        uid: crypto.randomUUID(),
        railwayCompanyId: 'trn:railway-company:db',
        seriesCode: 'TEE',
        category: 'TRAILER_CAR',
        roadNumber: 'VT 11.5 002',
        locomotiveType: null
      },
      {
        uid: crypto.randomUUID(),
        railwayCompanyId: 'trn:railway-company:db',
        seriesCode: 'TEE',
        category: 'TRAILER_CAR',
        roadNumber: 'VT 11.5 003',
        locomotiveType: null
      }
    ];

    const args = toAddRailwayModelArgs(form);

    expect(args.railwayModel.rollingStocks).toHaveLength(3);
    expect(args.railwayModel.rollingStocks[0].roadNumber).toBe('VT 11.5 001');
    expect(args.railwayModel.rollingStocks[1].roadNumber).toBe('VT 11.5 002');
    expect(args.railwayModel.rollingStocks[2].roadNumber).toBe('VT 11.5 003');
  });

  it('should handle optional purchase fields as null', () => {
    const form = createDefaultFormState();
    form.manufacturerId = 'test';
    form.productCode = 'test';
    form.description = 'test';
    form.category = 'LOCOMOTIVES';
    form.scale = 'H0';
    form.powerMethod = 'AC';
    form.epoch = 'IV';
    form.rollingStocks[0].railwayCompanyId = 'test';
    form.rollingStocks[0].seriesCode = 'test';
    form.rollingStocks[0].category = 'DIESEL_LOCOMOTIVE';

    // Leave purchase fields empty
    form.purchase.sellerId = null;
    form.purchase.purchaseCondition = null;
    form.purchase.modelCondition = null;
    form.purchase.boxCondition = null;
    form.purchase.notes = '';

    const args = toAddRailwayModelArgs(form);

    expect(args.sellerId).toBeNull();
    expect(args.purchaseCondition).toBeNull();
    expect(args.modelCondition).toBeNull();
    expect(args.boxCondition).toBeNull();
    expect(args.notes).toBeNull();
  });

  it("should use today's date for addedDate", () => {
    const form = createDefaultFormState();
    form.manufacturerId = 'test';
    form.productCode = 'test';
    form.description = 'test';
    form.category = 'LOCOMOTIVES';
    form.scale = 'H0';
    form.powerMethod = 'AC';
    form.epoch = 'IV';
    form.rollingStocks[0].railwayCompanyId = 'test';
    form.rollingStocks[0].seriesCode = 'test';
    form.rollingStocks[0].category = 'DIESEL_LOCOMOTIVE';

    const args = toAddRailwayModelArgs(form);
    const today = new Date().toISOString().split('T')[0];

    expect(args.addedDate).toBe(today);
    expect(args.addedDate).toMatch(/^\d{4}-\d{2}-\d{2}$/);
  });
});
