import { describe, it, expect } from 'vitest';
import { toRailwayModel } from '$lib/features/collection/utils/modelViewMapper';
import type {
  RailwayModelView,
  CollectionItemView,
  RailwayModelImageResponse,
  RollingStockView,
  OwnedRollingStockView
} from '$lib/bindings';

// ─── helpers ──────────────────────────────────────────────────────────────

function makeModelView(overrides: Partial<RailwayModelView> = {}): RailwayModelView {
  return {
    id: 'trn:railway-model:test:001',
    manufacturer: { manufacturerId: 'mfr-1', display: 'Roco' },
    productCode: '79894',
    description: 'BR 185 Electric Locomotive',
    descriptionLang: 'en',
    details: 'Additional details',
    detailsLang: 'en',
    powerMethod: 'DC',
    scale: 'H0',
    epoch: 'VI',
    category: 'LOCOMOTIVES',
    deliveryDate: null,
    availabilityStatus: null,
    metadata: {
      version: 1,
      created_at: '2024-01-01T00:00:00Z',
      updated_at: '2024-01-01T00:00:00Z'
    },
    rollingStock: [],
    ...overrides
  } as unknown as RailwayModelView;
}

function makeLocomotiveView(id: string = 'rs-loco-1'): RollingStockView {
  return {
    locomotive: {
      id,
      railway: { railwayCompanyId: 'rwy-1', display: 'DB' },
      prototype_id: null,
      livery: 'Red',
      length_over_buffer: { millimeters: '200', inches: null },
      technical_specifications: null,
      friendly_name: 'BR 185',
      series_code: 'BR185',
      road_number: '185-001',
      series: 'Series 1',
      depot: 'München',
      locomotive_type: 'ELECTRIC_LOCOMOTIVE',
      dcc_interface: 'NEM_651',
      control: 'DCC_FITTED',
      is_dummy: false
    }
  };
}

function makeEmuView(id: string = 'rs-emu-1'): RollingStockView {
  return {
    electricMultipleUnit: {
      id,
      railway: { railwayCompanyId: 'rwy-1', display: 'SBB' },
      prototype_id: null,
      livery: 'Silver',
      length_over_buffer: null,
      technical_specifications: null,
      friendly_name: 'ICN',
      series_code: 'ICN-001',
      road_number: '1001',
      series: null,
      depot: 'Zürich',
      electric_multiple_unit_type: 'HIGH_SPEED_TRAIN',
      dcc_interface: null,
      control: null,
      is_dummy: false
    }
  };
}

function makeRailcarView(id: string = 'rs-railcar-1'): RollingStockView {
  return {
    railcar: {
      id,
      railway: { railwayCompanyId: 'rwy-2', display: 'FS' },
      prototype_id: null,
      livery: 'Green',
      length_over_buffer: null,
      technical_specifications: null,
      friendly_name: 'ALe 582',
      series_code: 'ALe582',
      road_number: '582-001',
      series: null,
      depot: 'Milano',
      railcar_type: 'TRAILER_CAR',
      dcc_interface: null,
      control: null,
      is_dummy: false
    }
  };
}

function makePassengerCarView(id: string = 'rs-pass-1'): RollingStockView {
  return {
    passengerCar: {
      id,
      railway: { railwayCompanyId: 'rwy-1', display: 'DB' },
      prototype_id: null,
      livery: 'Blue',
      length_over_buffer: null,
      technical_specifications: null,
      friendly_name: 'Bpmbz',
      series_code: 'Bpmbz291',
      road_number: '91-80-0-291-001',
      series: null,
      passenger_car_type: 'OPEN_COACH',
      service_level: 'SECOND'
    }
  };
}

function makeFreightCarView(id: string = 'rs-frt-1'): RollingStockView {
  return {
    freightCar: {
      id,
      railway: { railwayCompanyId: 'rwy-1', display: 'DB' },
      prototype_id: null,
      livery: null,
      length_over_buffer: { millimeters: '95', inches: null },
      technical_specifications: null,
      friendly_name: null,
      series_code: 'Vbigkl',
      road_number: '23 45 678 901-2',
      freight_car_type: 'GONDOLA'
    }
  };
}

function makeCollectionItem(rollingStocks: OwnedRollingStockView[] = []): CollectionItemView {
  return {
    id: 'ci-001',
    railwayModel: {
      railwayModelId: 'trn:railway-model:test:001',
      manufacturer: 'Roco',
      productCode: '79894',
      description: 'BR 185',
      scale: 'H0',
      epoch: 'VI',
      category: 'LOCOMOTIVES',
      powerMethod: 'DC'
    },
    addedDate: '2024-01-01',
    removedDate: null,
    purchaseCondition: null,
    modelCondition: null,
    boxCondition: null,
    notes: null,
    rollingStocks,
    purchaseInfo: null
  } as unknown as CollectionItemView;
}

function makeOwnedRollingStock(
  rollingStockId: string,
  overrides: Partial<OwnedRollingStockView> = {}
): OwnedRollingStockView {
  return {
    id: `owned-${rollingStockId}`,
    rollingStockId,
    notes: null,
    series: null,
    roadNumber: '185-001',
    livery: null,
    control: 'DCC_FITTED',
    railwayCompanyName: 'DB',
    digital: { interface: 'NEM_651', dcc_address: 3, installed_decoder_id: 'decoder-1' },
    depot: null,
    dccInterface: null,
    lengthOverBuffers: null,
    ...overrides
  } as unknown as OwnedRollingStockView;
}

// ─── tests ────────────────────────────────────────────────────────────────

describe('toRailwayModel', () => {
  describe('basic field mapping', () => {
    it('maps id, manufacturer, productCode from modelView', () => {
      const modelView = makeModelView();
      const result = toRailwayModel(modelView);

      expect(result.id).toBe('trn:railway-model:test:001');
      expect(result.manufacturer).toBe('Roco');
      expect(result.product_code).toBe('79894');
    });

    it('maps scale, epoch, powerMethod, category', () => {
      const modelView = makeModelView({
        scale: 'N',
        epoch: 'IV',
        powerMethod: 'AC',
        category: 'FREIGHT_CARS'
      });
      const result = toRailwayModel(modelView);

      expect(result.scale).toBe('N');
      expect(result.era).toBe('IV');
      expect(result.power_method).toBe('AC');
      expect(result.category).toBe('FREIGHT_CARS');
    });

    it('maps description and details with their language codes', () => {
      const modelView = makeModelView({
        description: 'Test description',
        descriptionLang: 'it',
        details: 'Test details',
        detailsLang: 'it'
      });
      const result = toRailwayModel(modelView);

      expect(result.description).toBe('Test description');
      expect(result.descriptionLang).toBe('it');
      expect(result.details).toBe('Test details');
      expect(result.detailsLang).toBe('it');
    });
  });

  describe('status determination', () => {
    it('sets status to "Wishlist" when no collectionItem is provided', () => {
      const result = toRailwayModel(makeModelView());
      expect(result.status).toBe('Wishlist');
    });

    it('sets status to "InCollection" when a collectionItem is provided', () => {
      const result = toRailwayModel(makeModelView(), makeCollectionItem());
      expect(result.status).toBe('InCollection');
    });

    it('sets status to "Wishlist" when collectionItem is null explicitly', () => {
      const result = toRailwayModel(makeModelView(), null);
      expect(result.status).toBe('Wishlist');
    });
  });

  describe('image path mapping', () => {
    it('sets image_path to null when no imageResponse is provided', () => {
      const result = toRailwayModel(makeModelView());
      expect(result.image_path).toBeNull();
    });

    it('sets image_path to null when imageResponse.imagePath is null', () => {
      const imageResponse: RailwayModelImageResponse = {
        imagePath: null,
        placeholderHtml: '<div>placeholder</div>',
        hasImage: false
      };
      const result = toRailwayModel(makeModelView(), null, imageResponse);
      expect(result.image_path).toBeNull();
    });

    it('sets image_path from imageResponse.imagePath when provided', () => {
      const imageResponse: RailwayModelImageResponse = {
        imagePath: '/path/to/image.jpg',
        placeholderHtml: null,
        hasImage: true
      };
      const result = toRailwayModel(makeModelView(), null, imageResponse);
      expect(result.image_path).toBe('/path/to/image.jpg');
    });
  });

  describe('null/optional fields', () => {
    it('handles null details gracefully', () => {
      const modelView = makeModelView({ details: null, detailsLang: null });
      const result = toRailwayModel(modelView);

      expect(result.details).toBeNull();
      expect(result.detailsLang).toBeNull();
    });
  });

  describe('rolling stock mapping (Wishlist / no collection item)', () => {
    it('returns empty rolling_stock when rollingStock array is empty', () => {
      const result = toRailwayModel(makeModelView({ rollingStock: [] }));
      expect(result.rolling_stock).toHaveLength(0);
    });

    it('maps a locomotive rolling stock view correctly', () => {
      const modelView = makeModelView({ rollingStock: [makeLocomotiveView('loco-1')] });
      const result = toRailwayModel(modelView);

      expect(result.rolling_stock).toHaveLength(1);
      const rs = result.rolling_stock[0];
      expect(rs.id).toBe('loco-1');
      expect(rs.railway_company).toBe('DB');
      expect(rs.series_code).toBe('BR185');
      expect(rs.road_number).toBe('185-001');
      expect(rs.depot).toBe('München');
      expect(rs.livery).toBe('Red');
      expect(rs.length_mm).toBe(200);
      expect(rs.control_type).toBe('DCC_FITTED');
      expect(rs.dcc_interface).toBe('NEM_651');
      expect(rs.category).toBe('ELECTRIC_LOCOMOTIVE');
      expect(rs.subcategory).toBeNull();
    });

    it('maps an EMU rolling stock view with subcategory', () => {
      const modelView = makeModelView({ rollingStock: [makeEmuView('emu-1')] });
      const result = toRailwayModel(modelView);

      const rs = result.rolling_stock[0];
      expect(rs.id).toBe('emu-1');
      expect(rs.category).toBe('ELECTRIC_MULTIPLE_UNIT');
      expect(rs.subcategory).toBe('HIGH_SPEED_TRAIN');
      expect(rs.control_type).toBeNull();
    });

    it('maps a railcar rolling stock view with subcategory', () => {
      const modelView = makeModelView({ rollingStock: [makeRailcarView('railcar-1')] });
      const result = toRailwayModel(modelView);

      const rs = result.rolling_stock[0];
      expect(rs.id).toBe('railcar-1');
      expect(rs.category).toBe('RAILCAR');
      expect(rs.subcategory).toBe('TRAILER_CAR');
    });

    it('maps a passenger car rolling stock view', () => {
      const modelView = makeModelView({ rollingStock: [makePassengerCarView('pass-1')] });
      const result = toRailwayModel(modelView);

      const rs = result.rolling_stock[0];
      expect(rs.id).toBe('pass-1');
      expect(rs.railway_company).toBe('DB');
      expect(rs.category).toBe('OPEN_COACH');
      expect(rs.depot).toBeNull();
      expect(rs.control_type).toBeNull();
      expect(rs.dcc_interface).toBeNull();
    });

    it('maps a freight car rolling stock view', () => {
      const modelView = makeModelView({ rollingStock: [makeFreightCarView('frt-1')] });
      const result = toRailwayModel(modelView);

      const rs = result.rolling_stock[0];
      expect(rs.id).toBe('frt-1');
      expect(rs.category).toBe('GONDOLA');
      expect(rs.length_mm).toBe(95);
      expect(rs.depot).toBeNull();
      expect(rs.series_name).toBeNull(); // freightCar has no series field
    });

    it('maps length to null when length_over_buffer is null', () => {
      const emu = makeEmuView('emu-null');
      const modelView = makeModelView({ rollingStock: [emu] });
      const result = toRailwayModel(modelView);

      expect(result.rolling_stock[0].length_mm).toBeNull();
    });

    it('maps multiple rolling stock items', () => {
      const modelView = makeModelView({
        rollingStock: [makeLocomotiveView('loco-1'), makePassengerCarView('pass-1')]
      });
      const result = toRailwayModel(modelView);

      expect(result.rolling_stock).toHaveLength(2);
    });
  });

  describe('owned rolling stock mapping (InCollection)', () => {
    it('maps owned rolling stock using owned roadNumber and control', () => {
      const locoView = makeLocomotiveView('rs-loco-1');
      const modelView = makeModelView({ rollingStock: [locoView] });
      const ownedRS = makeOwnedRollingStock('rs-loco-1', {
        roadNumber: '185-999',
        control: 'DCC_FITTED',
        digital: {
          interface: 'NEM_651',
          dcc_address: 10,
          installed_decoder_id: 'decoder-1'
        } as unknown as OwnedRollingStockView['digital']
      });
      const collectionItem = makeCollectionItem([ownedRS]);

      const result = toRailwayModel(modelView, collectionItem);

      expect(result.rolling_stock).toHaveLength(1);
      const rs = result.rolling_stock[0];
      // Uses catalog rolling stock ID (rollingStockId from owned), not owned id
      expect(rs.id).toBe('rs-loco-1');
      // Road number from owned record
      expect(rs.road_number).toBe('185-999');
      // Control from owned record
      expect(rs.control_type).toBe('DCC_FITTED');
      // DCC interface from owned digital setup
      expect(rs.dcc_interface).toBe('NEM_651');
    });

    it('merges catalog data for matching owned rolling stock', () => {
      const locoView = makeLocomotiveView('rs-loco-1');
      const modelView = makeModelView({ rollingStock: [locoView] });
      const ownedRS = makeOwnedRollingStock('rs-loco-1');
      const collectionItem = makeCollectionItem([ownedRS]);

      const result = toRailwayModel(modelView, collectionItem);
      const rs = result.rolling_stock[0];

      // Catalog data: railway_company, series_code, livery from matching locomotive view
      expect(rs.railway_company).toBe('DB');
      expect(rs.series_code).toBe('BR185');
      expect(rs.livery).toBe('Red');
    });

    it('handles owned rolling stock with no matching catalog view', () => {
      // modelView has no rollingStock but collectionItem has an owned record
      const modelView = makeModelView({ rollingStock: [] });
      const ownedRS = makeOwnedRollingStock('rs-no-catalog');
      const collectionItem = makeCollectionItem([ownedRS]);

      const result = toRailwayModel(modelView, collectionItem);

      expect(result.rolling_stock).toHaveLength(1);
      const rs = result.rolling_stock[0];
      // Falls back to nulls for catalog data
      expect(rs.railway_company).toBeNull();
      expect(rs.series_code).toBe('');
      expect(rs.category).toBeNull();
    });

    it('maps multiple owned rolling stock items', () => {
      const modelView = makeModelView({
        rollingStock: [makeLocomotiveView('loco-1'), makePassengerCarView('pass-1')]
      });
      const ownedLoco = makeOwnedRollingStock('loco-1');
      const ownedPass = makeOwnedRollingStock('pass-1');
      const collectionItem = makeCollectionItem([ownedLoco, ownedPass]);

      const result = toRailwayModel(modelView, collectionItem);

      expect(result.rolling_stock).toHaveLength(2);
    });

    it('uses null digital interface when owned digital is null', () => {
      const locoView = makeLocomotiveView('rs-loco-1');
      const modelView = makeModelView({ rollingStock: [locoView] });
      const ownedRS = makeOwnedRollingStock('rs-loco-1', { digital: null });
      const collectionItem = makeCollectionItem([ownedRS]);

      const result = toRailwayModel(modelView, collectionItem);

      expect(result.rolling_stock[0].dcc_interface).toBeNull();
    });
  });
});
