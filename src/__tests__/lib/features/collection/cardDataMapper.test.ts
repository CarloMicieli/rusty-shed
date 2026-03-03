import { describe, it, expect } from 'vitest';
import {
  collectionItemToCardData,
  mapCategory,
  extractDigitalFeatures,
  extractPurchaseDate
} from '$lib/features/collection/utils/cardDataMapper';
import type { CollectionItemView, OwnedRollingStockView, PurchaseInfo } from '$lib/bindings';

// ---------------------------------------------------------------------------
// Helpers to build minimal fixtures
// ---------------------------------------------------------------------------

function makeRollingStock(overrides: Partial<OwnedRollingStockView> = {}): OwnedRollingStockView {
  return {
    id: 'rs-1',
    rollingStockId: 'catalog-rs-1',
    notes: null,
    series: null,
    roadNumber: null,
    livery: null,
    control: null,
    railwayCompanyName: null,
    digital: null,
    depot: null,
    ...overrides
  };
}

function makeItem(overrides: Partial<CollectionItemView> = {}): CollectionItemView {
  return {
    id: 'coll-1',
    railwayModel: {
      railwayModelId: 'trn:railway-model:marklin:3000',
      manufacturer: 'Märklin',
      productCode: '3000',
      scale: 'H0',
      epoch: 'III',
      description: 'BR 89.0',
      category: 'LOCOMOTIVES',
      powerMethod: 'AC'
    },
    addedDate: '2024-03-16',
    removedDate: null,
    purchaseCondition: null,
    modelCondition: null,
    boxCondition: null,
    notes: null,
    rollingStocks: [makeRollingStock({ roadNumber: '89 006', control: 'DCC_SOUND' })],
    purchaseInfo: {
      kind: 'purchased',
      data: { id: 'pi-1', purchaseDate: '2024-03-15', price: null, seller: null }
    },
    ...overrides
  };
}

// ---------------------------------------------------------------------------
// collectionItemToCardData
// ---------------------------------------------------------------------------

describe('collectionItemToCardData', () => {
  it('maps all direct fields correctly', () => {
    const result = collectionItemToCardData(makeItem());

    expect(result.id).toBe('trn:railway-model:marklin:3000');
    expect(result.manufacturer).toBe('Märklin');
    expect(result.productCode).toBe('3000');
    expect(result.series).toBe('BR 89.0');
    expect(result.scale).toBe('H0');
    expect(result.era).toBe('III');
  });

  it('maps description to series', () => {
    const item = makeItem();
    item.railwayModel.description = 'Class 140';
    expect(collectionItemToCardData(item).series).toBe('Class 140');
  });

  it('sets powerMethod and photoUrl when available', () => {
    const result = collectionItemToCardData(makeItem());
    expect(result.powerMethod).toBe('AC');
    expect(result.photoUrl).toBeNull();
  });

  it('extracts roadNumber from first rolling stock', () => {
    const item = makeItem({
      rollingStocks: [
        makeRollingStock({ roadNumber: '50 80 26-81 517-7' }),
        makeRollingStock({ roadNumber: '50 80 26-81 518-5' })
      ]
    });
    expect(collectionItemToCardData(item).roadNumber).toBe('50 80 26-81 517-7');
  });

  it('returns null roadNumber when rolling stock array is empty', () => {
    const item = makeItem({ rollingStocks: [] });
    expect(collectionItemToCardData(item).roadNumber).toBeNull();
  });

  it('sets unitCount when more than one rolling stock', () => {
    const item = makeItem({
      rollingStocks: [makeRollingStock(), makeRollingStock(), makeRollingStock()]
    });
    expect(collectionItemToCardData(item).unitCount).toBe(3);
  });

  it('sets unitCount to null for single rolling stock', () => {
    const item = makeItem({ rollingStocks: [makeRollingStock()] });
    expect(collectionItemToCardData(item).unitCount).toBeNull();
  });

  it('sets unitCount to null for empty rolling stock', () => {
    const item = makeItem({ rollingStocks: [] });
    expect(collectionItemToCardData(item).unitCount).toBeNull();
  });
});

// ---------------------------------------------------------------------------
// mapCategory
// ---------------------------------------------------------------------------

describe('mapCategory', () => {
  it('maps FREIGHT_CARS to FreightCar', () => {
    expect(mapCategory('FREIGHT_CARS')).toBe('FreightCar');
  });

  it('maps PASSENGER_CARS to PassengerCar', () => {
    expect(mapCategory('PASSENGER_CARS')).toBe('PassengerCar');
  });

  it('maps TRAIN_SETS to TrainSet', () => {
    expect(mapCategory('TRAIN_SETS')).toBe('TrainSet');
  });

  it('maps STARTER_SETS to TrainSet', () => {
    expect(mapCategory('STARTER_SETS')).toBe('TrainSet');
  });

  it('maps ELECTRIC_MULTIPLE_UNITS to TrainSet', () => {
    expect(mapCategory('ELECTRIC_MULTIPLE_UNITS')).toBe('TrainSet');
  });

  it('maps RAILCARS to Railcar', () => {
    expect(mapCategory('RAILCARS')).toBe('Railcar');
  });

  it('maps LOCOMOTIVES to SteamLocomotive (default)', () => {
    expect(mapCategory('LOCOMOTIVES')).toBe('SteamLocomotive');
  });

  it('returns Unknown for null category', () => {
    expect(mapCategory(null)).toBe('Unknown');
  });
});

// ---------------------------------------------------------------------------
// extractDigitalFeatures
// ---------------------------------------------------------------------------

describe('extractDigitalFeatures', () => {
  it('detects Sound from DCC_SOUND control', () => {
    const rs = [makeRollingStock({ control: 'DCC_SOUND' })];
    expect(extractDigitalFeatures(rs)).toContain('Sound');
  });

  it('also detects DCC when control is DCC_SOUND', () => {
    const rs = [makeRollingStock({ control: 'DCC_SOUND' })];
    const features = extractDigitalFeatures(rs);
    expect(features).toContain('Sound');
    expect(features).toContain('DCC');
  });

  it('detects DCC from DCC_FITTED control', () => {
    const rs = [makeRollingStock({ control: 'DCC_FITTED' })];
    expect(extractDigitalFeatures(rs)).toContain('DCC');
    expect(extractDigitalFeatures(rs)).not.toContain('Sound');
  });

  it('detects DCC from DCC_READY control', () => {
    const rs = [makeRollingStock({ control: 'DCC_READY' })];
    expect(extractDigitalFeatures(rs)).toContain('DCC');
  });

  it('detects DCC from digital setup even when control is null', () => {
    const rs = [
      makeRollingStock({
        control: null,
        digital: { interface: 'PLUX_22', dcc_address: 3, installed_decoder_id: 'dec-1' }
      })
    ];
    expect(extractDigitalFeatures(rs)).toContain('DCC');
  });

  it('returns empty for NO_DCC control with no digital setup', () => {
    const rs = [makeRollingStock({ control: 'NO_DCC', digital: null })];
    expect(extractDigitalFeatures(rs)).toEqual([]);
  });

  it('returns empty for null control with no digital setup', () => {
    const rs = [makeRollingStock({ control: null, digital: null })];
    expect(extractDigitalFeatures(rs)).toEqual([]);
  });

  it('returns empty for empty rolling stock array', () => {
    expect(extractDigitalFeatures([])).toEqual([]);
  });

  it('deduplicates features across multiple units', () => {
    const rs = [
      makeRollingStock({ control: 'DCC_FITTED' }),
      makeRollingStock({ control: 'DCC_FITTED' })
    ];
    const features = extractDigitalFeatures(rs);
    expect(features.filter((f) => f === 'DCC')).toHaveLength(1);
  });

  it('aggregates features from multiple units', () => {
    const rs = [
      makeRollingStock({ control: 'DCC_SOUND' }),
      makeRollingStock({ control: 'DCC_FITTED' })
    ];
    const features = extractDigitalFeatures(rs);
    expect(features).toContain('Sound');
    expect(features).toContain('DCC');
  });
});

// ---------------------------------------------------------------------------
// extractPurchaseDate
// ---------------------------------------------------------------------------

describe('extractPurchaseDate', () => {
  it('extracts date from purchased kind', () => {
    const info: PurchaseInfo = {
      kind: 'purchased',
      data: { id: 'p-1', purchaseDate: '2024-03-15', price: null, seller: null }
    };
    expect(extractPurchaseDate(info)).toBe('2024-03-15');
  });

  it('extracts orderDate from preOrdered kind', () => {
    const info: PurchaseInfo = {
      kind: 'preOrdered',
      data: {
        id: 'po-1',
        orderDate: '2024-06-01',
        deposit: { amount: 5000n, currency: 'EUR' },
        totalPrice: { amount: 25000n, currency: 'EUR' },
        seller: null,
        expectedDate: null
      }
    };
    expect(extractPurchaseDate(info)).toBe('2024-06-01');
  });

  it('returns null for sold kind', () => {
    const info: PurchaseInfo = {
      kind: 'sold',
      data: {
        id: 's-1',
        purchaseDate: '2023-01-01',
        purchasePrice: null,
        saleDate: '2024-12-01',
        salePrice: { amount: 15000n, currency: 'EUR' },
        buyer: null,
        seller: null
      }
    };
    expect(extractPurchaseDate(info)).toBeNull();
  });

  it('returns null for null purchaseInfo', () => {
    expect(extractPurchaseDate(null)).toBeNull();
  });
});
