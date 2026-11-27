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
vi.mock('$lib/features/digital-roster/components/InPlaceDccAddressEdit.svelte', () => ({
  default: function InPlaceDccAddressEditStub() {}
}));

import DigitalRosterTable from '$lib/features/digital-roster/components/DigitalRosterTable.svelte';
import type { DigitalRollingStockView } from '$lib/bindings';

describe('DigitalRosterTable', () => {
  beforeEach(() => cleanup());

  it('shows loading and empty states', async () => {
    const onInstallDecoder = vi.fn();
    const { rerender } = render(DigitalRosterTable, {
      props: { rollingStocks: [], loading: true, onInstallDecoder }
    });
    expect(document.querySelector('.animate-spin')).not.toBeNull();

    await rerender({ rollingStocks: [], loading: false, onInstallDecoder });
    expect(screen.getByText('digital_roster_empty')).toBeInTheDocument();
  });

  it('filters text and clears the filter', async () => {
    const onFilterChange = vi.fn();
    render(DigitalRosterTable, {
      props: { rollingStocks: [], filterText: '3', onFilterChange }
    });

    const input = screen.getByPlaceholderText('digital_roster_filter_placeholder');
    await fireEvent.input(input, { target: { value: '3' } });
    expect(onFilterChange).toHaveBeenCalledWith('3');
  });

  it('renders rows and emits change/delete callbacks', async () => {
    const onChangeDecoder = vi.fn();
    const onDelete = vi.fn();
    const stock = {
      id: 'stock-1',
      owned_rolling_stock_id: 'owned-1',
      dcc_address: 42,
      road_number: '656 001',
      category: 'LOCOMOTIVE',
      railway_company_name: 'FS',
      scale: 'H0',
      power_method: 'DC',
      decoder: { manufacturer: 'ZIMO', product_code: 'MX645' }
    };
    render(DigitalRosterTable, {
      props: {
        rollingStocks: [stock as unknown as DigitalRollingStockView],
        onChangeDecoder,
        onDelete
      }
    });

    expect(screen.getByText('656 001')).toBeInTheDocument();
    expect(screen.getByText('LOCOMOTIVE')).toBeInTheDocument();
    expect(screen.getByText(/ZIMO MX645/)).toBeInTheDocument();

    await fireEvent.click(screen.getByRole('button', { name: 'digital_roster_change_decoder' }));
    expect(onChangeDecoder).toHaveBeenCalledWith(stock);

    await fireEvent.click(screen.getByRole('button', { name: 'common_delete' }));
    expect(onDelete).toHaveBeenCalledWith(stock);
  });
});
