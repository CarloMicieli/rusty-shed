import type { RollingStockCategory } from '$lib/bindings';

export const CATEGORY_OPTIONS: { id: string; label: string }[] = [
  { id: 'LOCOMOTIVE', label: 'Locomotive' },
  { id: 'FREIGHT_CAR', label: 'Freight Car' },
  { id: 'PASSENGER_CAR', label: 'Passenger Car' },
  { id: 'ELECTRIC_MULTIPLE_UNIT', label: 'Electric Multiple Unit' },
  { id: 'RAILCAR', label: 'Railcar' }
];

export const LOCOMOTIVE_TYPE_OPTIONS: { id: string; label: string }[] = [
  { id: 'STEAM_LOCOMOTIVE', label: 'Steam Locomotive' },
  { id: 'DIESEL_LOCOMOTIVE', label: 'Diesel Locomotive' },
  { id: 'ELECTRIC_LOCOMOTIVE', label: 'Electric Locomotive' }
];

export const FREIGHT_CAR_TYPE_OPTIONS: { id: string; label: string }[] = [
  { id: '', label: '—' },
  { id: 'AUTO_TRANSPORT_CARS', label: 'Auto Transport Cars' },
  { id: 'BRAKE_WAGON', label: 'Brake Wagon' },
  { id: 'CLOSED_CARGO_VEHICLE', label: 'Closed Cargo Vehicle' },
  { id: 'CONTAINER_CARS', label: 'Container Cars' },
  { id: 'COVERED_FREIGHT_CARS', label: 'Covered Freight Cars' },
  { id: 'DEEP_WELL_FLAT_CARS', label: 'Deep Well Flat Cars' },
  { id: 'DUMP_CARS', label: 'Dump Cars' },
  { id: 'GONDOLA', label: 'Gondola' },
  { id: 'HEAVY_GOODS_WAGONS', label: 'Heavy Goods Wagons' },
  { id: 'HINGED_COVER_WAGONS', label: 'Hinged Cover Wagons' },
  { id: 'HOPPER_WAGON', label: 'Hopper Wagon' },
  { id: 'REFRIGERATOR_CARS', label: 'Refrigerator Cars' },
  { id: 'SILO_CONTAINER_CARS', label: 'Silo Container Cars' },
  { id: 'SLIDE_TARPAULIN_WAGON', label: 'Slide Tarpaulin Wagon' },
  { id: 'SLIDING_WALL_BOXCARS', label: 'Sliding Wall Boxcars' },
  { id: 'SPECIAL_TRANSPORT', label: 'Special Transport' },
  { id: 'STAKE_WAGONS', label: 'Stake Wagons' },
  { id: 'SWING_ROOF_WAGON', label: 'Swing Roof Wagon' },
  { id: 'TANK_CARS', label: 'Tank Cars' },
  { id: 'TELESCOPE_HOOD_WAGONS', label: 'Telescope Hood Wagons' }
];

export const PASSENGER_CAR_TYPE_OPTIONS: { id: string; label: string }[] = [
  { id: '', label: '—' },
  { id: 'BAGGAGE_CAR', label: 'Baggage Car' },
  { id: 'BUFFET_CAR', label: 'Buffet Car' },
  { id: 'COMBINE_CAR', label: 'Combine Car' },
  { id: 'COMPARTMENT_COACH', label: 'Compartment Coach' },
  { id: 'DINING_CAR', label: 'Dining Car' },
  { id: 'DOUBLE_DECKER', label: 'Double Decker' },
  { id: 'DOME_CAR', label: 'Dome Car' },
  { id: 'DRIVING_TRAILER', label: 'Driving Trailer' },
  { id: 'LOUNGE', label: 'Lounge' },
  { id: 'OBSERVATION', label: 'Observation' },
  { id: 'OPEN_COACH', label: 'Open Coach' },
  { id: 'RAILWAY_POST_OFFICE', label: 'Railway Post Office' },
  { id: 'SLEEPING_CAR', label: 'Sleeping Car' },
  { id: 'SLEEPERETTE', label: 'Sleeperette' }
];

export const EMU_TYPE_OPTIONS: { id: string; label: string }[] = [
  { id: 'DRIVING_CAR', label: 'Driving Car' },
  { id: 'HIGH_SPEED_TRAIN', label: 'High Speed Train' },
  { id: 'MOTOR_CAR', label: 'Motor Car' },
  { id: 'POWER_CAR', label: 'Power Car' },
  { id: 'TRAILER_CAR', label: 'Trailer Car' },
  { id: 'TRAIN_SET', label: 'Train Set' }
];

export const RAILCAR_TYPE_OPTIONS: { id: string; label: string }[] = [
  { id: 'POWER_CAR', label: 'Power Car' },
  { id: 'TRAILER_CAR', label: 'Trailer Car' }
];

export const SERVICE_LEVEL_OPTIONS: { id: string; label: string }[] = [
  { id: '', label: '—' },
  { id: 'FIRST', label: '1st Class' },
  { id: 'SECOND', label: '2nd Class' },
  { id: 'THIRD', label: '3rd Class' },
  { id: 'FIRST_SECOND', label: '1st/2nd Class' },
  { id: 'SECOND_THIRD', label: '2nd/3rd Class' },
  { id: 'FIRST_SECOND_THIRD', label: '1st/2nd/3rd Class' }
];

export function getSubcategoryOptions(
  category: RollingStockCategory | null
): { id: string; label: string }[] {
  switch (category) {
    case 'LOCOMOTIVE':
      return LOCOMOTIVE_TYPE_OPTIONS;
    case 'FREIGHT_CAR':
      return FREIGHT_CAR_TYPE_OPTIONS;
    case 'PASSENGER_CAR':
      return PASSENGER_CAR_TYPE_OPTIONS;
    case 'ELECTRIC_MULTIPLE_UNIT':
      return EMU_TYPE_OPTIONS;
    case 'RAILCAR':
      return RAILCAR_TYPE_OPTIONS;
    default:
      return [];
  }
}

export const CONTROL_OPTIONS: { id: string; label: string }[] = [
  { id: '', label: '—' },
  { id: 'DCC_READY', label: 'DCC Ready' },
  { id: 'DCC_FITTED', label: 'DCC Fitted' },
  { id: 'DCC_SOUND', label: 'DCC Sound' },
  { id: 'NO_DCC', label: 'Analogue (No DCC)' }
];

export const DCC_INTERFACE_OPTIONS: { id: string; label: string }[] = [
  { id: '', label: '—' },
  { id: 'NEM_651', label: 'NEM 651' },
  { id: 'NEM_652', label: 'NEM 652' },
  { id: 'NEM_654', label: 'NEM 654' },
  { id: 'PLUX_8', label: 'PluX 8' },
  { id: 'PLUX_12', label: 'PluX 12' },
  { id: 'PLUX_16', label: 'PluX 16' },
  { id: 'PLUX_22', label: 'PluX 22' },
  { id: 'NEXT_18', label: 'Next18' },
  { id: 'NEXT_18_S', label: 'Next18-S' },
  { id: 'MTC_21', label: 'MTC 21' }
];
