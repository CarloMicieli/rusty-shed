/**
 * Pure helper functions for prototype display logic.
 */

import type { PrototypeView } from '$lib/bindings';

/**
 * Derives the icon-type string expected by `PrototypeIcon` from a `PrototypeView`.
 *
 * Maps the `specification_type` discriminator (and its sub-type field) to one
 * of the keys recognised by `PrototypeIcon`'s `iconMap`:
 * `Locomotive` | `PowerCar` | `Coach` | `Couchette` | `Dining` | `Sleeping` |
 * `ControlCar` | `BaggageCar` | `FreightWagon`
 */
export function prototypeIconType(proto: PrototypeView): string {
  switch (proto.specification_type) {
    case 'LOCOMOTIVE':
      return 'Locomotive';
    case 'PASSENGER_CAR':
      return proto.passenger_car_type ?? 'Coach';
    case 'FREIGHT_CAR':
      return proto.freight_car_type ?? 'FreightWagon';
    case 'RAILCAR':
      return proto.railcar_type ?? 'Coach';
    case 'ELECTRIC_MULTIPLE_UNIT':
      return 'PowerCar';
    default:
      return 'Coach';
  }
}
