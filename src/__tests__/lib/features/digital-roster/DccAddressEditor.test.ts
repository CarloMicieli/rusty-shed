import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent, waitFor } from '@testing-library/svelte';
import DccAddressEditor from '$lib/features/digital-roster/components/DccAddressEditor.svelte';

// ── Paraglide messages ───────────────────────────────────────────────────────
vi.mock('$lib/paraglide/messages', () => ({
  digital_roster_edit_address_title: () => 'Change DCC Address',
  digital_roster_edit_address_subtitle: ({ roadNumber, railway }: Record<string, string>) =>
    `Rolling Stock: ${roadNumber} - ${railway}`,
  digital_roster_table_address: () => 'DCC Address',
  digital_roster_address_invalid_range: () => 'Address must be between 1 and 9999',
  digital_roster_duplicate_address_warning: () => 'Address already in use',
  digital_roster_duplicate_address_message: () =>
    'This DCC address is currently assigned to another rolling stock.',
  app_cancel: () => 'Cancel',
  app_loading: () => 'Loading...',
  digital_roster_edit_address_save: () => 'Save Address'
}));

const mockStock = {
  id: 'trn:digital:d1',
  owned_rolling_stock_id: 'trn:rolling-stock:rs1',
  dcc_address: 42,
  decoder: { id: 'dec-1', brand: 'ESU', model: 'LokSound' },
  category: 'LOCOMOTIVE',
  railway_company_name: 'DB',
  scale: 'HO',
  power_method: 'DCC',
  road_number: 'BR 218'
};

describe('DccAddressEditor.svelte', () => {
  const onSave = vi.fn();
  const onCheckDuplicate = vi.fn();
  const onClose = vi.fn();

  beforeEach(() => {
    vi.clearAllMocks();
    onSave.mockResolvedValue(true);
    onCheckDuplicate.mockResolvedValue({ isDuplicate: false });
  });

  it('renders dialog when open=true', () => {
    render(DccAddressEditor, {
      props: { open: true, stock: mockStock as never, onSave, onCheckDuplicate, onClose }
    });
    expect(screen.getByRole('dialog')).toBeInTheDocument();
  });

  it('does not render dialog when open=false', () => {
    render(DccAddressEditor, {
      props: { open: false, stock: mockStock as never, onSave, onCheckDuplicate, onClose }
    });
    expect(screen.queryByRole('dialog')).toBeNull();
  });

  it('renders the title', () => {
    render(DccAddressEditor, {
      props: { open: true, stock: mockStock as never, onSave, onCheckDuplicate, onClose }
    });
    expect(screen.getByText('Change DCC Address')).toBeInTheDocument();
  });

  it('renders stock details in subtitle', () => {
    render(DccAddressEditor, {
      props: { open: true, stock: mockStock as never, onSave, onCheckDuplicate, onClose }
    });
    expect(screen.getByText('Rolling Stock: BR 218 - DB')).toBeInTheDocument();
  });

  it('renders current DCC address in the input', () => {
    render(DccAddressEditor, {
      props: { open: true, stock: mockStock as never, onSave, onCheckDuplicate, onClose }
    });
    const input = screen.getByRole('spinbutton') as HTMLInputElement;
    expect(input.value).toBe('42');
  });

  it('shows validation error when address is out of range (0)', async () => {
    render(DccAddressEditor, {
      props: { open: true, stock: mockStock as never, onSave, onCheckDuplicate, onClose }
    });
    const input = screen.getByRole('spinbutton');

    await fireEvent.input(input, { target: { value: '0' } });
    await waitFor(() => {
      expect(screen.getByText('Address must be between 1 and 9999')).toBeInTheDocument();
    });
  });

  it('shows validation error when address is out of range (10000)', async () => {
    render(DccAddressEditor, {
      props: { open: true, stock: mockStock as never, onSave, onCheckDuplicate, onClose }
    });
    const input = screen.getByRole('spinbutton');

    await fireEvent.input(input, { target: { value: '10000' } });
    await waitFor(() => {
      expect(screen.getByText('Address must be between 1 and 9999')).toBeInTheDocument();
    });
  });

  it('clears validation error when a valid address is entered', async () => {
    render(DccAddressEditor, {
      props: { open: true, stock: mockStock as never, onSave, onCheckDuplicate, onClose }
    });
    const input = screen.getByRole('spinbutton');

    await fireEvent.input(input, { target: { value: '0' } });
    await waitFor(() => expect(screen.getByText('Address must be between 1 and 9999')));

    await fireEvent.input(input, { target: { value: '100' } });
    await waitFor(() => {
      expect(screen.queryByText('Address must be between 1 and 9999')).toBeNull();
    });
  });

  it('shows duplicate warning when onCheckDuplicate returns isDuplicate=true', async () => {
    onCheckDuplicate.mockResolvedValue({ isDuplicate: true, existingId: 'trn:digital:d2' });

    render(DccAddressEditor, {
      props: { open: true, stock: mockStock as never, onSave, onCheckDuplicate, onClose }
    });
    const input = screen.getByRole('spinbutton');

    await fireEvent.input(input, { target: { value: '99' } });
    await waitFor(() => {
      expect(screen.getByText('Address already in use')).toBeInTheDocument();
    });
  });

  it('calls onClose when Cancel button is clicked', async () => {
    render(DccAddressEditor, {
      props: { open: true, stock: mockStock as never, onSave, onCheckDuplicate, onClose }
    });
    await fireEvent.click(screen.getByText('Cancel'));
    expect(onClose).toHaveBeenCalledTimes(1);
  });

  it('calls onClose when background overlay is clicked', async () => {
    render(DccAddressEditor, {
      props: { open: true, stock: mockStock as never, onSave, onCheckDuplicate, onClose }
    });
    const overlay = document.querySelector('.fixed.inset-0') as HTMLElement;
    if (overlay) await fireEvent.click(overlay);
    expect(onClose).toHaveBeenCalled();
  });

  it('saves unchanged address by calling onClose without onSave', async () => {
    render(DccAddressEditor, {
      props: { open: true, stock: mockStock as never, onSave, onCheckDuplicate, onClose }
    });
    // Click save without changing address (same as current)
    await fireEvent.click(screen.getByText('Save Address'));

    // onSave should NOT be called, onClose should be called
    expect(onSave).not.toHaveBeenCalled();
    expect(onClose).toHaveBeenCalled();
  });
});
