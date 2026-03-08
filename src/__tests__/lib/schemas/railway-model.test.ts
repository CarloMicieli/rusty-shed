import { describe, it, expect } from 'vitest';
import {
  createRailwayModelSchema,
  powerMethodSchema,
  scaleSchema,
  categorySchema,
  controlSchema,
  rollingStockSchema
} from '$lib/schemas/railway-model';
import type { CreateRailwayModelInput } from '$lib/schemas/railway-model';

describe('railway-model schemas', () => {
  describe('enum schemas', () => {
    describe('powerMethodSchema', () => {
      it('accepts valid power methods', () => {
        expect(powerMethodSchema.parse('AC')).toBe('AC');
        expect(powerMethodSchema.parse('DC')).toBe('DC');
        expect(powerMethodSchema.parse('TRIX_EXPRESS')).toBe('TRIX_EXPRESS');
      });

      it('rejects invalid power methods', () => {
        expect(() => powerMethodSchema.parse('INVALID')).toThrow();
        expect(() => powerMethodSchema.parse('ac')).toThrow();
      });
    });

    describe('scaleSchema', () => {
      it('accepts valid scales', () => {
        expect(scaleSchema.parse('H0')).toBe('H0');
        expect(scaleSchema.parse('N')).toBe('N');
        expect(scaleSchema.parse('G')).toBe('G');
      });

      it('rejects invalid scales', () => {
        expect(() => scaleSchema.parse('HO')).toThrow();
        expect(() => scaleSchema.parse('Invalid')).toThrow();
      });
    });

    describe('categorySchema', () => {
      it('accepts valid categories', () => {
        expect(categorySchema.parse('LOCOMOTIVES')).toBe('LOCOMOTIVES');
        expect(categorySchema.parse('FREIGHT_CARS')).toBe('FREIGHT_CARS');
      });

      it('rejects invalid categories', () => {
        expect(() => categorySchema.parse('TRAINS')).toThrow();
      });
    });

    describe('controlSchema', () => {
      it('accepts valid control types', () => {
        expect(controlSchema.parse('DCC_READY')).toBe('DCC_READY');
        expect(controlSchema.parse('NO_DCC')).toBe('NO_DCC');
      });

      it('rejects invalid control types', () => {
        expect(() => controlSchema.parse('UNKNOWN')).toThrow();
      });
    });
  });

  describe('createRailwayModelSchema', () => {
    const validInput: CreateRailwayModelInput = {
      manufacturer_id: 'mfg-1',
      product_code: 'BR-001',
      description: 'A sample locomotive',
      details: 'Additional details',
      power_method: 'DC',
      scale: 'H0',
      epoch: 'IV',
      category: 'LOCOMOTIVES',
      delivery_date: '2025/12',
      availability_status: 'AVAILABLE',
      rolling_stocks: [
        {
          category: 'Locomotive',
          friendly_name: 'Sample Locomotive',
          series_code: 'BR-001',
          road_number: '5001',
          livery: 'Original',
          series: null,
          depot: null,
          railway_company_id: 'company-1',
          locomotive_type: 'STEAM_LOCOMOTIVE',
          is_dummy: null,
          control: 'NO_DCC',
          dcc_interface: null,
          length_over_buffers: null,
          technical_specifications: null
        }
      ]
    };

    it('validates a complete valid model', () => {
      const result = createRailwayModelSchema.parse(validInput);
      expect(result.manufacturer_id).toBe('mfg-1');
      expect(result.category).toBe('LOCOMOTIVES');
    });

    it('requires manufacturer_id', () => {
      const invalid = { ...validInput, manufacturer_id: '' };
      expect(() => createRailwayModelSchema.parse(invalid)).toThrow();
    });

    it('requires product_code', () => {
      const invalid = { ...validInput, product_code: '' };
      expect(() => createRailwayModelSchema.parse(invalid)).toThrow();
    });

    it('requires description', () => {
      const invalid = { ...validInput, description: '' };
      expect(() => createRailwayModelSchema.parse(invalid)).toThrow();
    });

    it('requires at least one rolling stock', () => {
      const invalid = { ...validInput, rolling_stocks: [] };
      expect(() => createRailwayModelSchema.parse(invalid)).toThrow();
    });

    it('allows null details and availability_status', () => {
      const minimal = {
        ...validInput,
        delivery_date: '2025/Q2',
        details: null,
        availability_status: null
      };
      const result = createRailwayModelSchema.parse(minimal);
      expect(result.details).toBeNull();
      expect(result.availability_status).toBeNull();
    });

    it('rejects invalid enum values', () => {
      const invalid = { ...validInput, power_method: 'INVALID' as any };
      expect(() => createRailwayModelSchema.parse(invalid)).toThrow();
    });

    it('rejects missing required nested fields', () => {
      const invalid = {
        ...validInput,
        rolling_stocks: [
          { ...validInput.rolling_stocks[0], friendly_name: '' }
        ]
      };
      expect(() => createRailwayModelSchema.parse(invalid)).toThrow();
    });
  });

  describe('rollingStockSchema', () => {
    it('validates a locomotive', () => {
      const locomotive = {
        category: 'Locomotive',
        friendly_name: 'BR-001 Locomotive',
        series_code: 'BR-001',
        road_number: '5001',
        livery: 'Original',
        series: null,
        depot: null,
        railway_company_id: 'company-1',
        locomotive_type: 'STEAM_LOCOMOTIVE',
        is_dummy: false,
        control: 'DCC_FITTED',
        dcc_interface: 'NEM_651',
        length_over_buffers: null,
        technical_specifications: null
      };
      const result = rollingStockSchema.parse(locomotive);
      expect(result.category).toBe('Locomotive');
    });

    it('validates a freight car', () => {
      const freightCar = {
        category: 'FreightCar',
        friendly_name: 'XYZ-100 Freight Car',
        series_code: 'XYZ-100',
        road_number: '1234',
        livery: 'Standard',
        series: null,
        depot: null,
        railway_company_id: 'company-1',
        freight_car_type: 'GONDOLA',
        length_over_buffers: null,
        technical_specifications: null
      };
      const result = rollingStockSchema.parse(freightCar);
      expect(result.category).toBe('FreightCar');
    });

    it('requires series_code', () => {
      const invalid = {
        category: 'Locomotive',
        friendly_name: 'Test Locomotive',
        series_code: '',
        road_number: '5001',
        livery: 'Original',
        series: null,
        depot: null,
        railway_company_id: 'company-1',
        locomotive_type: 'STEAM_LOCOMOTIVE',
        is_dummy: false,
        control: 'NO_DCC',
        dcc_interface: null,
        length_over_buffers: null,
        technical_specifications: null
      };
      expect(() => rollingStockSchema.parse(invalid)).toThrow();
    });
  });
});
