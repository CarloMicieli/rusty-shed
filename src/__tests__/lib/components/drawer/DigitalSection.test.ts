import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, waitFor, cleanup, fireEvent } from '@testing-library/svelte';

vi.mock('$lib/paraglide/messages.js', () => ({
  drawer_section_digital: () => 'Digital Settings',
  digital_roster_address_label: () => 'DCC Address',
  digital_roster_address_range: () => 'Address must be between 1 and 9999',
  digital_roster_date_label: () => 'Installation Date'
}));

// DatePickerField uses @internationalized/date — stub the bits that DrawerShell pulls in
vi.mock('@internationalized/date', () => ({
  today: () => ({ year: 2026, month: 3, day: 17 }),
  getLocalTimeZone: () => 'Europe/Rome'
}));

// DatePickerField is a complex bits-ui component; stub it to a simple input so
// DigitalSection tests focus on its own logic rather than the datepicker internals.
vi.mock('$lib/components', async (importOriginal) => {
  const actual = await importOriginal<typeof import('$lib/components')>();
  return {
    ...actual,
    DatePickerField: (await import('./__mocks__/DatePickerFieldMock.svelte')).default
  };
});

import DigitalSection from '$lib/components/drawer/sections/DigitalSection.svelte';

describe('DigitalSection', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
  });

  // ── Section header ──────────────────────────────────────────────────────────

  it('renders section header', () => {
    render(DigitalSection, {
      props: { dccAddress: null, installationDate: '2026-03-17' }
    });
    expect(screen.getByText('Digital Settings')).toBeInTheDocument();
  });

  // ── DCC address input ────────────────────────────────────────────────────────

  it('renders DCC address label and input', () => {
    render(DigitalSection, {
      props: { dccAddress: null, installationDate: '2026-03-17' }
    });
    expect(screen.getByText('DCC Address')).toBeInTheDocument();
    expect(screen.getByPlaceholderText('1-9999')).toBeInTheDocument();
  });

  it('shows existing dccAddress value in input', () => {
    render(DigitalSection, {
      props: { dccAddress: 42, installationDate: '2026-03-17' }
    });
    expect(screen.getByPlaceholderText('1-9999')).toHaveValue(42);
  });

  it('fires onAddressChange when address input changes', async () => {
    const onAddressChange = vi.fn().mockResolvedValue(undefined);
    render(DigitalSection, {
      props: { dccAddress: null, installationDate: '2026-03-17', onAddressChange }
    });

    const input = screen.getByPlaceholderText('1-9999');
    fireEvent.input(input, { target: { value: '100' } });

    await waitFor(() => {
      expect(onAddressChange).toHaveBeenCalledWith(100);
    });
  });

  it('passes null to onAddressChange for empty input', async () => {
    const onAddressChange = vi.fn().mockResolvedValue(undefined);
    render(DigitalSection, {
      props: { dccAddress: 42, installationDate: '2026-03-17', onAddressChange }
    });

    const input = screen.getByPlaceholderText('1-9999');
    fireEvent.input(input, { target: { value: '' } });

    await waitFor(() => {
      expect(onAddressChange).toHaveBeenCalledWith(null);
    });
  });

  // ── Duplicate warning ────────────────────────────────────────────────────────

  it('shows duplicate warning when duplicateWarning prop is set', () => {
    render(DigitalSection, {
      props: {
        dccAddress: 42,
        installationDate: '2026-03-17',
        duplicateWarning: 'Address 42 is already in use'
      }
    });
    expect(screen.getByText('Address 42 is already in use')).toBeInTheDocument();
  });

  it('does not show duplicate warning when prop is null', () => {
    render(DigitalSection, {
      props: { dccAddress: null, installationDate: '2026-03-17', duplicateWarning: null }
    });
    expect(screen.queryByText(/already in use/i)).toBeNull();
  });

  // ── Validation error ─────────────────────────────────────────────────────────

  it('hides validation error when touched=false even if errors.address is set', () => {
    render(DigitalSection, {
      props: {
        dccAddress: null,
        installationDate: '2026-03-17',
        errors: { address: 'Address must be between 1 and 9999' },
        touched: false
      }
    });
    expect(screen.queryByText('Address must be between 1 and 9999')).toBeNull();
  });

  it('shows validation error when touched=true and errors.address is set', () => {
    render(DigitalSection, {
      props: {
        dccAddress: null,
        installationDate: '2026-03-17',
        errors: { address: 'Address must be between 1 and 9999' },
        touched: true
      }
    });
    expect(screen.getByText('Address must be between 1 and 9999')).toBeInTheDocument();
  });

  // ── Installation date ────────────────────────────────────────────────────────

  it('renders installation date label', () => {
    render(DigitalSection, {
      props: { dccAddress: null, installationDate: '2026-03-17' }
    });
    expect(screen.getByText('Installation Date')).toBeInTheDocument();
  });
});
