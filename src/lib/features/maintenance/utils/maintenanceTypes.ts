import type { MaintenanceType } from '$lib/bindings';
import * as m from '$lib/paraglide/messages.js';

export function getMaintenanceTypes(): Array<{ value: MaintenanceType; label: string }> {
  return [
    { value: 'WHEEL_CLEANING', label: m.maintenance_type_wheel_cleaning() },
    { value: 'TRACK_CLEANING', label: m.maintenance_type_track_cleaning() },
    { value: 'CONTACT_CLEANING', label: m.maintenance_type_contact_cleaning() },
    { value: 'LUBRICATION', label: m.maintenance_type_lubrication() },
    { value: 'GEAR_GREASE', label: m.maintenance_type_gear_grease() },
    { value: 'MOTOR_BRUSH_REPLACEMENT', label: m.maintenance_type_motor_brush_replacement() },
    { value: 'TRACTION_TIRE_REPLACEMENT', label: m.maintenance_type_traction_tire_replacement() },
    { value: 'DECODER_INSTALL', label: m.maintenance_type_decoder_install() },
    { value: 'FIRMWARE_UPDATE', label: m.maintenance_type_firmware_update() },
    { value: 'SPEAKER_REPAIR', label: m.maintenance_type_speaker_repair() },
    { value: 'STAY_ALIVE_INSTALL', label: m.maintenance_type_stay_alive_install() },
    { value: 'COUPLER_ADJUSTMENT', label: m.maintenance_type_coupler_adjustment() },
    { value: 'COUPLER_CHANGE', label: m.maintenance_type_coupler_change() },
    { value: 'DETAIL_REPAIR', label: m.maintenance_type_detail_repair() },
    { value: 'WEATHERING', label: m.maintenance_type_weathering() },
    { value: 'GENERAL_INSPECTION', label: m.maintenance_type_general_inspection() },
    { value: 'OTHER', label: m.maintenance_type_other() }
  ];
}
