import { cleanup, fireEvent, render, screen } from '@testing-library/svelte';
import { beforeEach, describe, expect, it, vi } from 'vitest';

vi.mock('$lib/paraglide/messages', async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  return Object.fromEntries(
    Object.entries(actual).map(([key, value]) => [
      key,
      typeof value === 'function' ? () => key : value
    ])
  );
});

import RollingStockBasicFields from '$lib/features/rolling-stock-edit/components/RollingStockBasicFields.svelte';

describe('RollingStockBasicFields', () => {
  beforeEach(() => cleanup());

  it('renders all identification labels', () => {
    render(RollingStockBasicFields, {
      props: {
        seriesCode: '',
        series: '',
        roadNumber: '',
        friendlyName: '',
        livery: '',
        depot: '',
        railwayCompanyId: null,
        companyOptions: []
      }
    });

    expect(screen.getByText(/specs_drawer_section_identification/)).toBeInTheDocument();
    expect(screen.getByLabelText(/rolling_stock_field_series_code/)).toBeInTheDocument();
  });

  it('binds series, road number, friendly name, livery, and depot inputs', async () => {
    render(RollingStockBasicFields, {
      props: {
        seriesCode: 'E656',
        series: 'Serie I',
        roadNumber: '001',
        friendlyName: 'Caimano',
        livery: 'Green',
        depot: 'Milano',
        railwayCompanyId: 'company-1',
        companyOptions: [{ value: 'company-1', label: 'FS' }]
      }
    });

    await fireEvent.input(screen.getByDisplayValue('E656'), { target: { value: 'E657' } });
    await fireEvent.input(screen.getByDisplayValue('Serie I'), { target: { value: 'Serie II' } });
    await fireEvent.input(screen.getByDisplayValue('001'), { target: { value: '002' } });
    await fireEvent.input(screen.getByDisplayValue('Caimano'), { target: { value: 'Tigre' } });
    await fireEvent.input(screen.getByDisplayValue('Green'), { target: { value: 'Red' } });
    await fireEvent.input(screen.getByDisplayValue('Milano'), { target: { value: 'Roma' } });

    expect(screen.getByDisplayValue('E657')).toBeInTheDocument();
    expect(screen.getByDisplayValue('Serie II')).toBeInTheDocument();
    expect(screen.getByDisplayValue('002')).toBeInTheDocument();
    expect(screen.getByDisplayValue('Tigre')).toBeInTheDocument();
    expect(screen.getByDisplayValue('Red')).toBeInTheDocument();
    expect(screen.getByDisplayValue('Roma')).toBeInTheDocument();
  });

  it('renders company options and the initial company selection', () => {
    render(RollingStockBasicFields, {
      props: {
        seriesCode: '',
        series: '',
        roadNumber: '',
        friendlyName: '',
        livery: '',
        depot: '',
        railwayCompanyId: 'company-1',
        companyOptions: [{ value: 'company-1', label: 'FS' }]
      }
    });

    expect(screen.getByText('FS')).toBeInTheDocument();
  });
});
