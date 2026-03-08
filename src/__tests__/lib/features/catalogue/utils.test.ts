import { describe, it, expect } from 'vitest';
import {
  createDefaultRollingStock,
  normalizeRollingStock,
  type RollingStockForm,
  type NullableEnum
} from '$lib/features/catalogue/utils';

describe('Catalogue Utils', () => {
  describe('createDefaultRollingStock', () => {
    it('should create a default rolling stock form with all fields', () => {
      const defaultForm = createDefaultRollingStock();

      expect(defaultForm).toEqual({
        category: '',
        railway_company_id: '',
        friendly_name: '',
        series_code: '',
        road_number: '',
        series: null,
        depot: null,
        livery: null,
        locomotive_type: '',
        passenger_car_type: '',
        freight_car_type: '',
        electric_multiple_unit_type: '',
        service_level: '',
        is_dummy: null,
        control: '',
        dcc_interface: '',
        length_over_buffers: null,
        technical_specifications: null
      });
    });

    it('should return a new object each time', () => {
      const form1 = createDefaultRollingStock();
      const form2 = createDefaultRollingStock();

      expect(form1).not.toBe(form2);
      expect(form1).toEqual(form2);
    });

    it('should have empty string for category', () => {
      const defaultForm = createDefaultRollingStock();
      expect(defaultForm.category).toBe('');
    });

    it('should have null values for optional fields', () => {
      const defaultForm = createDefaultRollingStock();
      expect(defaultForm.series).toBeNull();
      expect(defaultForm.depot).toBeNull();
      expect(defaultForm.livery).toBeNull();
      expect(defaultForm.is_dummy).toBeNull();
      expect(defaultForm.length_over_buffers).toBeNull();
      expect(defaultForm.technical_specifications).toBeNull();
    });
  });

  describe('normalizeRollingStock', () => {
    it('should normalize a Locomotive rolling stock', () => {
      const form: RollingStockForm = {
        ...createDefaultRollingStock(),
        category: 'Locomotive',
        railway_company_id: 'company-1',
        friendly_name: 'Test Locomotive',
        series_code: 'BR-001',
        road_number: '5001',
        series: 'Class 5',
        depot: 'Depot A',
        livery: 'Original',
        locomotive_type: 'STEAM_LOCOMOTIVE',
        is_dummy: false,
        control: 'DCC_FITTED',
        dcc_interface: 'NEM_651'
      };

      const normalized = normalizeRollingStock(form);

      expect(normalized.category).toBe('Locomotive');
      expect(normalized.railway_company_id).toBe('company-1');
      expect(normalized.friendly_name).toBe('Test Locomotive');
      expect(normalized.series_code).toBe('BR-001');
      expect(normalized.road_number).toBe('5001');
      expect(normalized.series).toBe('Class 5');
      expect(normalized.depot).toBe('Depot A');
      expect(normalized.livery).toBe('Original');
      expect((normalized as any).locomotive_type).toBe('STEAM_LOCOMOTIVE');
      expect((normalized as any).is_dummy).toBe(false);
      expect((normalized as any).control).toBe('DCC_FITTED');
      expect((normalized as any).dcc_interface).toBe('NEM_651');
    });

    it('should normalize a PassengerCar rolling stock', () => {
      const form: RollingStockForm = {
        ...createDefaultRollingStock(),
        category: 'PassengerCar',
        railway_company_id: 'company-2',
        friendly_name: 'Passenger Car',
        series_code: 'PC-100',
        road_number: undefined as any,
        passenger_car_type: 'COMPARTMENT_COACH',
        service_level: 'FIRST_SECOND'
      };

      const normalized = normalizeRollingStock(form);

      expect(normalized.category).toBe('PassengerCar');
      expect((normalized as any).passenger_car_type).toBe('COMPARTMENT_COACH');
      expect((normalized as any).service_level).toBe('FIRST_SECOND');
    });

    it('should normalize a FreightCar rolling stock', () => {
      const form: RollingStockForm = {
        ...createDefaultRollingStock(),
        category: 'FreightCar',
        railway_company_id: 'company-3',
        friendly_name: 'Freight Car',
        series_code: 'FC-200',
        road_number: '1234',
        freight_car_type: 'GONDOLA'
      };

      const normalized = normalizeRollingStock(form);

      expect(normalized.category).toBe('FreightCar');
      expect((normalized as any).freight_car_type).toBe('GONDOLA');
    });

    it('should normalize a Railcar rolling stock', () => {
      const form: RollingStockForm = {
        ...createDefaultRollingStock(),
        category: 'Railcar',
        railway_company_id: 'company-4',
        friendly_name: 'Railcar',
        series_code: 'RC-50',
        road_number: undefined as any,
        control: 'NO_DCC',
        dcc_interface: null
      };

      const normalized = normalizeRollingStock(form);

      expect(normalized.category).toBe('Railcar');
      expect((normalized as any).control).toBe('NO_DCC');
      expect((normalized as any).dcc_interface).toBeNull();
    });

    it('should normalize an ElectricMultipleUnit rolling stock', () => {
      const form: RollingStockForm = {
        ...createDefaultRollingStock(),
        category: 'ElectricMultipleUnit',
        railway_company_id: 'company-5',
        friendly_name: 'EMU Train',
        series_code: 'EMU-300',
        road_number: '300001',
        electric_multiple_unit_type: 'MOTOR_CAR',
        is_dummy: true,
        control: 'DCC_SOUND',
        dcc_interface: 'PLUX_22'
      };

      const normalized = normalizeRollingStock(form);

      expect(normalized.category).toBe('ElectricMultipleUnit');
      expect((normalized as any).electric_multiple_unit_type).toBe('MOTOR_CAR');
      expect((normalized as any).is_dummy).toBe(true);
      expect((normalized as any).control).toBe('DCC_SOUND');
      expect((normalized as any).dcc_interface).toBe('PLUX_22');
    });

    it('should throw error for invalid category', () => {
      const form: any = {
        ...createDefaultRollingStock(),
        category: 'InvalidCategory'
      };

      expect(() => normalizeRollingStock(form)).toThrow('Invalid rolling stock category');
    });

    it('should convert empty strings to null for optional enum fields', () => {
      const form: RollingStockForm = {
        ...createDefaultRollingStock(),
        category: 'Locomotive',
        railway_company_id: 'company-1',
        locomotive_type: '',
        control: '',
        dcc_interface: ''
      };

      const normalized = normalizeRollingStock(form);

      expect((normalized as any).locomotive_type).toBe('');
      expect((normalized as any).control).toBeNull();
      expect((normalized as any).dcc_interface).toBeNull();
    });

    it('should preserve livery when provided', () => {
      const form: RollingStockForm = {
        ...createDefaultRollingStock(),
        category: 'Locomotive',
        railway_company_id: 'company-1',
        livery: 'Red and Cream'
      };

      const normalized = normalizeRollingStock(form);

      expect(normalized.livery).toBe('Red and Cream');
    });

    it('should convert empty livery to null', () => {
      const form: RollingStockForm = {
        ...createDefaultRollingStock(),
        category: 'Locomotive',
        railway_company_id: 'company-1',
        livery: null
      };

      const normalized = normalizeRollingStock(form);

      expect(normalized.livery).toBeNull();
    });

    it('should preserve length_over_buffers when provided', () => {
      const form: RollingStockForm = {
        ...createDefaultRollingStock(),
        category: 'Locomotive',
        railway_company_id: 'company-1',
        length_over_buffers: {
          millimeters: 125,
          inches: null
        }
      };

      const normalized = normalizeRollingStock(form);

      expect(normalized.length_over_buffers).toEqual({
        millimeters: 125,
        inches: null
      });
    });

    it('should handle all RollingStock categories', () => {
      const categories: Array<
        'Locomotive' | 'PassengerCar' | 'FreightCar' | 'Railcar' | 'ElectricMultipleUnit'
      > = ['Locomotive', 'PassengerCar', 'FreightCar', 'Railcar', 'ElectricMultipleUnit'];

      for (const category of categories) {
        const form: RollingStockForm = {
          ...createDefaultRollingStock(),
          category,
          railway_company_id: 'test'
        };

        const normalized = normalizeRollingStock(form);
        expect(normalized.category).toBe(category);
      }
    });

    it('should handle undefined friendly_name as empty string', () => {
      const form: RollingStockForm = {
        ...createDefaultRollingStock(),
        category: 'Locomotive',
        railway_company_id: 'company-1',
        friendly_name: undefined as any
      };

      const normalized = normalizeRollingStock(form);

      expect(normalized.friendly_name).toBe('');
    });

    it('should handle undefined road_number as null for PassengerCar', () => {
      const form: RollingStockForm = {
        ...createDefaultRollingStock(),
        category: 'PassengerCar',
        railway_company_id: 'company-2',
        road_number: undefined as any
      };

      const normalized = normalizeRollingStock(form);

      expect((normalized as any).road_number).toBeNull();
    });
  });

  describe('NullableEnum type', () => {
    it('should accept empty string', () => {
      const value: NullableEnum = '';
      expect(value).toBe('');
    });

    it('should accept null', () => {
      const value: NullableEnum = null;
      expect(value).toBeNull();
    });

    it('should accept string values', () => {
      const value: NullableEnum<'OPTION_A' | 'OPTION_B'> = 'OPTION_A';
      expect(value).toBe('OPTION_A');
    });
  });

  describe('Edge cases and integration', () => {
    it('should handle complete locomotive workflow', () => {
      const form = createDefaultRollingStock();
      form.category = 'Locomotive';
      form.railway_company_id = 'company-1';
      form.friendly_name = 'Express Locomotive';
      form.series_code = 'EXP-001';
      form.road_number = '8001';
      form.locomotive_type = 'ELECTRIC_LOCOMOTIVE';

      const normalized = normalizeRollingStock(form);

      expect(normalized.category).toBe('Locomotive');
      expect(normalized.friendly_name).toBe('Express Locomotive');
      expect((normalized as any).locomotive_type).toBe('ELECTRIC_LOCOMOTIVE');
    });

    it('should handle freight car with all optional fields', () => {
      const form: RollingStockForm = {
        ...createDefaultRollingStock(),
        category: 'FreightCar',
        railway_company_id: 'company-3',
        friendly_name: 'Tank Car',
        series_code: 'TANK-50',
        road_number: '5000',
        series: 'TK-Series',
        depot: 'MainDepot',
        livery: 'Black',
        freight_car_type: 'TANK_CARS'
      };

      const normalized = normalizeRollingStock(form);

      expect((normalized as any).freight_car_type).toBe('TANK_CARS');
      expect(normalized.livery).toBe('Black');
      expect(normalized.depot).toBe('MainDepot');
    });
  });
});
