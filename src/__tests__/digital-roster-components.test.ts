import { cleanup, fireEvent, render, screen } from '@testing-library/svelte';
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
vi.mock('$lib/paraglide/messages', async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  return Object.fromEntries(
    Object.entries(actual).map(([key, value]) => [
      key,
      typeof value === 'function' ? () => key : value
    ])
  );
});
import InPlaceDccAddressEdit from '$lib/features/digital-roster/components/InPlaceDccAddressEdit.svelte';

describe('InPlaceDccAddressEdit', () => {
  beforeEach(() => cleanup());

  it('starts editing and exposes duplicate-address validation', async () => {
    const onEditStart = vi.fn();
    const onSave = vi.fn().mockResolvedValue(true);
    render(InPlaceDccAddressEdit, {
      props: {
        value: 10,
        excludeId: 'stock-1',
        allAddresses: [
          { id: 'stock-1', dcc_address: 10 },
          { id: 'stock-2', dcc_address: 42 }
        ],
        onSave,
        onEditStart
      }
    });

    await fireEvent.click(screen.getByTitle('action_click_to_edit'));
    expect(onEditStart).toHaveBeenCalledOnce();

    const input = screen.getByRole('spinbutton');
    await fireEvent.input(input, { target: { value: '42' } });
    expect(screen.getByText('digital_roster_duplicate_address')).toBeInTheDocument();
    expect(screen.getByTitle('action_save')).toBeDisabled();

    await fireEvent.input(input, { target: { value: '43' } });
    await fireEvent.click(screen.getByTitle('action_save'));
    expect(onSave).toHaveBeenCalledWith(43);
  });

  it('shows out-of-range and cancels edits', async () => {
    render(InPlaceDccAddressEdit, {
      props: {
        value: 10,
        excludeId: 'stock-1',
        allAddresses: [{ id: 'stock-1', dcc_address: 10 }],
        onSave: vi.fn().mockResolvedValue(false)
      }
    });

    await fireEvent.click(screen.getByTitle('action_click_to_edit'));
    await fireEvent.input(screen.getByRole('spinbutton'), { target: { value: '0' } });
    expect(screen.getByText('digital_roster_address_range_hint')).toBeInTheDocument();

    await fireEvent.click(screen.getByTitle('action_cancel'));
    expect(screen.getByTitle('action_click_to_edit')).toHaveTextContent('10');
  });

  it('cancels on Escape and saves on Enter', async () => {
    const onSave = vi.fn().mockResolvedValue(true);
    const onEditEnd = vi.fn();
    render(InPlaceDccAddressEdit, {
      props: {
        value: 10,
        excludeId: 'stock-1',
        allAddresses: [{ id: 'stock-1', dcc_address: 10 }],
        onSave,
        onEditEnd
      }
    });

    await fireEvent.click(screen.getByTitle('action_click_to_edit'));
    const input = screen.getByRole('spinbutton');
    await fireEvent.input(input, { target: { value: '11' } });
    await fireEvent.keyDown(input, { key: 'Escape' });
    expect(screen.getByTitle('action_click_to_edit')).toBeInTheDocument();

    await fireEvent.click(screen.getByTitle('action_click_to_edit'));
    await fireEvent.input(screen.getByRole('spinbutton'), { target: { value: '12' } });
    await fireEvent.keyDown(screen.getByRole('spinbutton'), { key: 'Enter' });
    expect(onSave).toHaveBeenCalledWith(12);
  });
});
