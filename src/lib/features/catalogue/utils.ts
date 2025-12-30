import type { CreateRailwayModelInput, RollingStockInput } from '$lib/schemas/railway-model';

export type NullableEnum<T extends string | null = string> = T | '' | null;

export type RollingStockForm = {
  category: '' | RollingStockInput['category'];
  railway_company_id: string;
  class_name?: string;
  road_number?: string;
  series: string | null;
  depot: string | null;
  livery: string | null;
  locomotive_type?: NullableEnum<
    Extract<RollingStockInput, { category: 'Locomotive' }>['locomotive_type']
  >;
  passenger_car_type?: NullableEnum<
    Extract<RollingStockInput, { category: 'PassengerCar' }>['passenger_car_type']
  >;
  freight_car_type?: NullableEnum<
    Extract<RollingStockInput, { category: 'FreightCar' }>['freight_car_type']
  >;
  electric_multiple_unit_type?: NullableEnum<
    Extract<RollingStockInput, { category: 'ElectricMultipleUnit' }>['electric_multiple_unit_type']
  >;
  type_name?: string;
  service_level?: NullableEnum<
    Extract<RollingStockInput, { category: 'PassengerCar' }>['service_level']
  >;
  is_dummy?: boolean;
  control?: NullableEnum<
    Extract<
      CreateRailwayModelInput['rolling_stocks'][number],
      { category: 'Locomotive' | 'Railcar' | 'ElectricMultipleUnit' }
    >['control']
  >;
  dcc_interface?: NullableEnum<
    Extract<
      CreateRailwayModelInput['rolling_stocks'][number],
      { category: 'Locomotive' | 'Railcar' | 'ElectricMultipleUnit' }
    >['dcc_interface']
  >;
  length_over_buffers: CreateRailwayModelInput['rolling_stocks'][number]['length_over_buffers'];
  technical_specifications: CreateRailwayModelInput['rolling_stocks'][number]['technical_specifications'];
};

export function createDefaultRollingStock(): RollingStockForm {
  return {
    category: '',
    railway_company_id: '',
    class_name: '',
    road_number: '',
    series: null,
    depot: null,
    livery: null,
    locomotive_type: '',
    passenger_car_type: '',
    freight_car_type: '',
    electric_multiple_unit_type: '',
    type_name: '',
    service_level: '',
    is_dummy: false,
    control: '',
    dcc_interface: '',
    length_over_buffers: null,
    technical_specifications: null
  };
}

export function normalizeRollingStock(rs: RollingStockForm): RollingStockInput {
  const base = {
    category: rs.category as RollingStockInput['category'],
    railway_company_id: rs.railway_company_id,
    livery: rs.livery || null,
    length_over_buffers: rs.length_over_buffers ?? null,
    technical_specifications: rs.technical_specifications ?? null
  };

  switch (rs.category) {
    case 'Locomotive':
      return {
        ...base,
        category: 'Locomotive',
        class_name: rs.class_name ?? '',
        road_number: rs.road_number ?? '',
        series: rs.series || null,
        depot: rs.depot || null,
        locomotive_type: rs.locomotive_type || '',
        is_dummy: rs.is_dummy ?? false,
        control: rs.control || null,
        dcc_interface: rs.dcc_interface || null
      } as RollingStockInput;
    case 'PassengerCar':
      return {
        ...base,
        category: 'PassengerCar',
        type_name: rs.type_name ?? '',
        road_number: rs.road_number || null,
        series: rs.series || null,
        depot: rs.depot || null,
        passenger_car_type: rs.passenger_car_type || '',
        service_level: rs.service_level || null
      } as RollingStockInput;
    case 'FreightCar':
      return {
        ...base,
        category: 'FreightCar',
        type_name: rs.type_name ?? '',
        road_number: rs.road_number || null,
        series: rs.series || null,
        depot: rs.depot || null,
        freight_car_type: rs.freight_car_type || null
      } as RollingStockInput;
    case 'Railcar':
      return {
        ...base,
        category: 'Railcar',
        type_name: rs.type_name ?? '',
        road_number: rs.road_number || null,
        series: rs.series || null,
        depot: rs.depot || null,
        control: rs.control || null,
        dcc_interface: rs.dcc_interface || null
      } as RollingStockInput;
    case 'ElectricMultipleUnit':
      return {
        ...base,
        category: 'ElectricMultipleUnit',
        type_name: rs.type_name ?? '',
        road_number: rs.road_number || null,
        series: rs.series || null,
        depot: rs.depot || null,
        electric_multiple_unit_type: rs.electric_multiple_unit_type || '',
        is_dummy: rs.is_dummy ?? false,
        control: rs.control || null,
        dcc_interface: rs.dcc_interface || null
      } as RollingStockInput;
    default:
      throw new Error('Invalid rolling stock category');
  }
}
