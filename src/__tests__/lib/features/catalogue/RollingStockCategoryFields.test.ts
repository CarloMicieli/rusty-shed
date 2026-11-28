import { cleanup, render, screen } from '@testing-library/svelte';
import { beforeEach, describe, expect, it, vi } from 'vitest';

vi.mock('$lib/paraglide/messages.js', async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  return Object.fromEntries(
    Object.entries(actual).map(([key, value]) => [
      key,
      typeof value === 'function' ? () => key : value
    ])
  );
});

import RollingStockCategoryFields from '$lib/features/catalogue/components/RollingStockCategoryFields.svelte';
import { createDefaultRollingStock } from '$lib/features/catalogue/utils';
import { formLabels } from '$lib/features/catalogue/constants';

function noErrors() {
  return undefined;
}

describe('RollingStockCategoryFields', () => {
  beforeEach(() => cleanup());

  it('renders locomotive-specific sections', () => {
    const rs = { ...createDefaultRollingStock(), category: 'Locomotive' as const };

    render(RollingStockCategoryFields, {
      props: {
        rs,
        index: 0,
        errorsFn: noErrors,
        formLabels,
        locomotiveTypesData: [{ id: 'ELECTRIC', labelKey: 'locomotive_type_electric' }],
        passengerCarTypesData: [],
        freightCarTypesData: [],
        controlsData: [{ id: 'DCC_READY', labelKey: 'control_dcc_ready' }],
        dccInterfacesData: [{ id: 'NEM_652', labelKey: 'dcc_interface_nem_652' }],
        serviceLevelsData: []
      }
    });

    expect(screen.getByText(/form_new_model_type/)).toBeInTheDocument();
    expect(screen.getByText(/form_new_model_is_dummy/)).toBeInTheDocument();
    expect(screen.getByText(/form_new_model_technical_details/)).toBeInTheDocument();
  });

  it('renders passenger/freight specific type and service fields', () => {
    const passenger = { ...createDefaultRollingStock(), category: 'PassengerCar' as const };
    const freight = { ...createDefaultRollingStock(), category: 'FreightCar' as const };

    const { rerender } = render(RollingStockCategoryFields, {
      props: {
        rs: passenger,
        index: 1,
        errorsFn: noErrors,
        formLabels,
        locomotiveTypesData: [],
        passengerCarTypesData: [{ id: 'COACH', labelKey: 'passenger_car_type_coach' }],
        freightCarTypesData: [{ id: 'BOXCAR', labelKey: 'freight_car_type_boxcar' }],
        controlsData: [],
        dccInterfacesData: [],
        serviceLevelsData: [{ id: 'EXPRESS', labelKey: 'service_level_express' }]
      }
    });

    expect(screen.getByText(/form_new_model_passenger_car_type/)).toBeInTheDocument();
    expect(screen.getByText(/form_new_model_service_level/)).toBeInTheDocument();

    rerender({
      rs: freight,
      index: 2,
      errorsFn: noErrors,
      formLabels,
      locomotiveTypesData: [],
      passengerCarTypesData: [{ id: 'COACH', labelKey: 'passenger_car_type_coach' }],
      freightCarTypesData: [{ id: 'BOXCAR', labelKey: 'freight_car_type_boxcar' }],
      controlsData: [],
      dccInterfacesData: [],
      serviceLevelsData: [{ id: 'EXPRESS', labelKey: 'service_level_express' }]
    });

    expect(screen.getByText(/form_new_model_freight_car_type/)).toBeInTheDocument();
  });

  it('renders railcar and EMU technical controls and dummy toggle', async () => {
    const railcar = { ...createDefaultRollingStock(), category: 'Railcar' as const };
    const emu = {
      ...createDefaultRollingStock(),
      category: 'ElectricMultipleUnit' as const
    };

    const { rerender } = render(RollingStockCategoryFields, {
      props: {
        rs: railcar,
        index: 3,
        errorsFn: noErrors,
        formLabels,
        locomotiveTypesData: [],
        passengerCarTypesData: [],
        freightCarTypesData: [],
        controlsData: [{ id: 'DCC_READY', labelKey: 'control_dcc_ready' }],
        dccInterfacesData: [{ id: 'NEM_652', labelKey: 'dcc_interface_nem_652' }],
        serviceLevelsData: []
      }
    });

    expect(screen.getByText(/form_new_model_control/)).toBeInTheDocument();
    expect(screen.getByText(/form_new_model_dcc_interface/)).toBeInTheDocument();

    await rerender({
      rs: emu,
      index: 4,
      errorsFn: noErrors,
      formLabels,
      locomotiveTypesData: [],
      passengerCarTypesData: [],
      freightCarTypesData: [],
      controlsData: [{ id: 'DCC_READY', labelKey: 'control_dcc_ready' }],
      dccInterfacesData: [{ id: 'NEM_652', labelKey: 'dcc_interface_nem_652' }],
      serviceLevelsData: []
    });

    expect(screen.getByText(/form_new_model_is_dummy/)).toBeInTheDocument();
  });
});
