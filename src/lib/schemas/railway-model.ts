/**
 * Zod validation schemas for Railway Model creation form
 *
 * These schemas mirror the TypeScript types from bindings.ts but exclude
 * ID fields (generated server-side). Used for client-side validation.
 */

import { z } from 'zod';

// ============================================================================
// Enum Schemas
// ============================================================================

export const powerMethodSchema = z.enum(['AC', 'DC', 'TRIX_EXPRESS']);

export const scaleSchema = z.enum([
  'H0',
  'H0m',
  'H0e',
  'N',
  'TT',
  'Z',
  'G',
  'Scale1',
  'Scale0',
  'Scale00'
]);

export const categorySchema = z.enum([
  'LOCOMOTIVES',
  'TRAIN_SETS',
  'STARTER_SETS',
  'FREIGHT_CARS',
  'PASSENGER_CARS',
  'ELECTRIC_MULTIPLE_UNITS',
  'RAILCARS'
]);

export const availabilityStatusSchema = z.enum([
  'ANNOUNCED',
  'AVAILABLE',
  'CANCELLED',
  'DISCONTINUED'
]);

export const controlSchema = z.enum(['DCC_READY', 'DCC_FITTED', 'DCC_SOUND', 'NO_DCC']);

export const locomotiveTypeSchema = z.enum([
  'STEAM_LOCOMOTIVE',
  'DIESEL_LOCOMOTIVE',
  'ELECTRIC_LOCOMOTIVE'
]);

export const passengerCarTypeSchema = z.enum([
  'BAGGAGE_CAR',
  'BUFFET_CAR',
  'COMBINE_CAR',
  'COMPARTMENT_COACH',
  'DINING_CAR',
  'DOUBLE_DECKER',
  'DOME_CAR',
  'DRIVING_TRAILER',
  'LOUNGE',
  'OBSERVATION',
  'OPEN_COACH',
  'RAILWAY_POST_OFFICE',
  'SLEEPING_CAR',
  'SLEEPERETTE'
]);

export const freightCarTypeSchema = z.enum([
  'AUTO_TRANSPORT_CARS',
  'BRAKE_WAGON',
  'CLOSED_CARGO_VEHICLE',
  'CONTAINER_CARS',
  'COVERED_FREIGHT_CARS',
  'DEEP_WELL_FLAT_CARS',
  'DUMP_CARS',
  'GONDOLA',
  'HEAVY_GOODS_WAGONS',
  'HINGED_COVER_WAGONS',
  'HOPPER_WAGON',
  'REFRIGERATOR_CARS',
  'SILO_CONTAINER_CARS',
  'SLIDE_TARPAULIN_WAGON',
  'SLIDING_WALL_BOXCARS',
  'SPECIAL_TRANSPORT',
  'STAKE_WAGONS',
  'SWING_ROOF_WAGON',
  'TANK_CARS',
  'TELESCOPE_HOOD_WAGONS'
]);

export const serviceLevelSchema = z.enum([
  'FIRST',
  'SECOND',
  'THIRD',
  'FIRST_SECOND',
  'SECOND_THIRD',
  'FIRST_SECOND_THIRD'
]);

export const dccInterfaceSchema = z.enum([
  'NEM_651',
  'NEM_652',
  'NEM_654',
  'PLUX_8',
  'PLUX_12',
  'PLUX_16',
  'PLUX_22',
  'NEXT_18',
  'NEXT_18_S',
  'MTC_21'
]);

export const electricMultipleUnitTypeSchema = z.enum([
  'DRIVING_CAR',
  'HIGH_SPEED_TRAIN',
  'MOTOR_CAR',
  'POWER_CAR',
  'TRAILER_CAR',
  'TRAIN_SET'
]);

export const couplingSocketSchema = z.enum([
  'NONE',
  'NEM_355',
  'NEM_356',
  'NEM_357',
  'NEM_359',
  'NEM_360',
  'NEM_362',
  'NEM_365'
]);

export const featureFlagSchema = z.enum(['YES', 'NO', 'NOT_APPLICABLE']);

export const bodyShellChassisTypeSchema = z.enum(['PLASTIC', 'METAL_DIE_CAST']);

// ============================================================================
// Technical Specifications Schema
// ============================================================================

const couplingSchema = z.object({
  socket: couplingSocketSchema,
  close_couplers: featureFlagSchema.nullable(),
  digital_shunting: featureFlagSchema.nullable()
});

const technicalSpecificationsSchema = z.object({
  minimum_radius: z.number().positive().nullable(),
  coupling: couplingSchema.nullable(),
  flywheel_fitted: featureFlagSchema.nullable(),
  body_shell: bodyShellChassisTypeSchema.nullable(),
  chassis: bodyShellChassisTypeSchema.nullable(),
  interior_lights: featureFlagSchema.nullable(),
  lights: featureFlagSchema.nullable(),
  sprung_buffers: featureFlagSchema.nullable()
});

// ============================================================================
// Length Over Buffers Schema
// ============================================================================

const lengthOverBuffersSchema = z.object({
  millimeters: z.number().positive().nullable(),
  inches: z.number().positive().nullable()
});

// ============================================================================
// Rolling Stock Schemas (Discriminated Union)
// ============================================================================

const baseRollingStockSchema = z.object({
  railway_company_id: z.string().min(1, 'Railway company is required'),
  livery: z.string().nullable(),
  length_over_buffers: lengthOverBuffersSchema.nullable(),
  technical_specifications: technicalSpecificationsSchema.nullable()
});

// Locomotive variant
const locomotiveSchema = baseRollingStockSchema.extend({
  category: z.literal('Locomotive'),
  class_name: z.string().min(1, 'Class name is required for locomotives'),
  road_number: z.string().min(1, 'Road number is required for locomotives'),
  series: z.string().nullable(),
  depot: z.string().nullable(),
  locomotive_type: locomotiveTypeSchema,
  is_dummy: z.boolean().default(false).nullable(),
  control: controlSchema.nullable(),
  dcc_interface: dccInterfaceSchema.nullable()
});

// Passenger Car variant
const passengerCarSchema = baseRollingStockSchema.extend({
  category: z.literal('PassengerCar'),
  type_name: z.string().min(1, 'Type name is required for passenger cars'),
  road_number: z.string().nullable(),
  series: z.string().nullable(),
  depot: z.string().nullable(),
  passenger_car_type: passengerCarTypeSchema,
  service_level: serviceLevelSchema.nullable()
});

// Freight Car variant
const freightCarSchema = baseRollingStockSchema.extend({
  category: z.literal('FreightCar'),
  type_name: z.string().min(1, 'Type name is required for freight cars'),
  road_number: z.string().nullable(),
  series: z.string().nullable(),
  depot: z.string().nullable(),
  freight_car_type: freightCarTypeSchema.nullable()
});

// Railcar variant
const railcarSchema = baseRollingStockSchema.extend({
  category: z.literal('Railcar'),
  type_name: z.string().min(1, 'Type name is required for railcars'),
  road_number: z.string().nullable(),
  series: z.string().nullable(),
  depot: z.string().nullable(),
  control: controlSchema.nullable(),
  dcc_interface: dccInterfaceSchema.nullable()
});

// Electric Multiple Unit variant
const electricMultipleUnitSchema = baseRollingStockSchema.extend({
  category: z.literal('ElectricMultipleUnit'),
  type_name: z.string().min(1, 'Type name is required for EMUs'),
  road_number: z.string().nullable(),
  series: z.string().nullable(),
  depot: z.string().nullable(),
  electric_multiple_unit_type: electricMultipleUnitTypeSchema,
  is_dummy: z.boolean().default(false).nullable(),
  control: controlSchema.nullable(),
  dcc_interface: dccInterfaceSchema.nullable()
});

// Discriminated union of all rolling stock variants
export const rollingStockSchema = z.discriminatedUnion('category', [
  locomotiveSchema,
  passengerCarSchema,
  freightCarSchema,
  railcarSchema,
  electricMultipleUnitSchema
]);

// ============================================================================
// Railway Model Schema (Main Form)
// ============================================================================

/**
 * Delivery date format validation
 * Accepts: "2025", "2025/06", "2025/Q2"
 */
const deliveryDateSchema = z
  .string()
  .regex(/^\d{4}(\/\d{2}|\/Q[1-4])?$/, {
    message: 'Invalid format. Use: 2025, 2025/06, or 2025/Q2'
  })
  .nullable();

export const createRailwayModelSchema = z.object({
  // Basic Information
  manufacturer_id: z.string().min(1, 'Manufacturer is required'),
  product_code: z.string().min(1, 'Product code is required'),
  description: z.string().min(1, 'Description is required'),
  details: z.string().nullable(),

  // Technical Specifications
  power_method: powerMethodSchema,
  scale: scaleSchema,
  epoch: z.string().min(1, 'Epoch is required'),
  category: categorySchema,

  // Availability
  delivery_date: deliveryDateSchema,
  availability_status: availabilityStatusSchema.nullable(),

  // Rolling Stock (array of variants)
  rolling_stocks: z.array(rollingStockSchema).min(1, 'At least one rolling stock is required')
});

// ============================================================================
// TypeScript Types (inferred from schemas)
// ============================================================================

export type CreateRailwayModelInput = z.infer<typeof createRailwayModelSchema>;
export type RollingStockInput = z.infer<typeof rollingStockSchema>;
export type LocomotiveInput = z.infer<typeof locomotiveSchema>;
export type PassengerCarInput = z.infer<typeof passengerCarSchema>;
export type FreightCarInput = z.infer<typeof freightCarSchema>;
export type RailcarInput = z.infer<typeof railcarSchema>;
export type ElectricMultipleUnitInput = z.infer<typeof electricMultipleUnitSchema>;
