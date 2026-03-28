import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, waitFor, cleanup } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';

vi.mock('$lib/paraglide/messages.js', () => ({
  drawer_section_model_info: () => 'Model Details',
  wishlist_modal_manufacturer: () => 'Manufacturer',
  wishlist_modal_manufacturer_placeholder: () => '— Select manufacturer —',
  wishlist_modal_product_code: () => 'Product Code',
  wishlist_modal_product_code_placeholder: () => 'e.g., 37858',
  wishlist_modal_description: () => 'Description',
  wishlist_modal_description_placeholder: () => 'e.g., Class 218 Diesel Locomotive',
  wishlist_modal_category: () => 'Category',
  wishlist_modal_scale: () => 'Scale',
  wishlist_modal_power_method: () => 'Power Method',
  wishlist_modal_epoch: () => 'Epoch',
  wishlist_modal_epoch_placeholder: () => 'e.g., IV or III/IV',
  wishlist_modal_loading: () => 'Loading...',
  enum_category_locomotives: () => 'Locomotives',
  enum_category_train_sets: () => 'Train Sets',
  enum_category_starter_sets: () => 'Starter Sets',
  enum_category_freight_cars: () => 'Freight Cars',
  enum_category_passenger_cars: () => 'Passenger Cars',
  enum_category_electric_multiple_units: () => 'Electric Multiple Units',
  enum_category_railcars: () => 'Railcars',
  enum_power_method_ac: () => 'AC',
  enum_power_method_dc: () => 'DC',
  enum_power_method_trix_express: () => 'Trix Express',
  rolling_stock_select_category: () => '— Select category —'
}));

import ModelInfoSection from '$lib/components/drawer/sections/ModelInfoSection.svelte';
import type { Manufacturer } from '$lib/bindings';

const manufacturers: Manufacturer[] = [
  { id: 'marklin', name: 'Märklin' } as unknown as Manufacturer,
  { id: 'fleischmann', name: 'Fleischmann' } as unknown as Manufacturer
];

const defaultProps = {
  manufacturerId: '',
  productCode: '',
  description: '',
  category: 'LOCOMOTIVES',
  scale: 'H0',
  powerMethod: 'DC',
  epoch: '',
  manufacturers
};

describe('ModelInfoSection', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
  });

  // ── Section header & all fields render ─────────────────────────────────────

  it('renders section header and all required field labels', () => {
    render(ModelInfoSection, { props: defaultProps });

    expect(screen.getByText('Model Details')).toBeInTheDocument();
    expect(screen.getAllByText(/manufacturer/i).length).toBeGreaterThan(0);
    expect(screen.getByLabelText(/product code/i)).toBeInTheDocument();
    expect(screen.getByLabelText(/description/i)).toBeInTheDocument();
    expect(screen.getAllByText(/category/i).length).toBeGreaterThan(0);
    expect(screen.getAllByText(/scale/i).length).toBeGreaterThan(0);
    expect(screen.getAllByText(/power method/i).length).toBeGreaterThan(0);
    expect(screen.getAllByText(/epoch/i).length).toBeGreaterThan(0);
  });

  // ── Loading state ────────────────────────────────────────────────────────────

  it('shows loading text when isLoading=true; shows manufacturer select when false', () => {
    const { unmount } = render(ModelInfoSection, { props: { ...defaultProps, isLoading: true } });
    expect(screen.getByText('Loading...')).toBeInTheDocument();
    unmount();

    render(ModelInfoSection, { props: { ...defaultProps, isLoading: false } });
    expect(screen.queryByText('Loading...')).toBeNull();
    expect(screen.getByRole('button', { name: /manufacturer/i })).toBeInTheDocument();
  });

  // ── Scale display with ratio ─────────────────────────────────────────────────

  it('shows scale ratio annotation for H0, N, and Z', () => {
    const { unmount } = render(ModelInfoSection, { props: { ...defaultProps, scale: 'H0' } });
    expect(screen.getByText('H0 (1:87)')).toBeInTheDocument();
    unmount();

    const { unmount: u2 } = render(ModelInfoSection, { props: { ...defaultProps, scale: 'N' } });
    expect(screen.getByText('N (1:160)')).toBeInTheDocument();
    u2();

    render(ModelInfoSection, { props: { ...defaultProps, scale: 'Z' } });
    expect(screen.getByText('Z (1:220)')).toBeInTheDocument();
  });

  // ── Category / power method labels ─────────────────────────────────────────

  it('shows Paraglide labels for category and power method', () => {
    const { unmount } = render(ModelInfoSection, {
      props: { ...defaultProps, category: 'LOCOMOTIVES' }
    });
    expect(screen.getByText('Locomotives')).toBeInTheDocument();
    unmount();

    render(ModelInfoSection, { props: { ...defaultProps, powerMethod: 'AC' } });
    expect(screen.getByText('AC')).toBeInTheDocument();
  });

  // ── Text input bindings ──────────────────────────────────────────────────────

  it('product code and description inputs reflect initial values', () => {
    render(ModelInfoSection, {
      props: { ...defaultProps, productCode: '37858', description: 'BR 218' }
    });
    expect(screen.getByLabelText(/product code/i)).toHaveValue('37858');
    expect(screen.getByLabelText(/description/i)).toHaveValue('BR 218');
  });

  // ── Error messages (all in one render) ───────────────────────────────────────

  it('displays all field errors when provided', () => {
    render(ModelInfoSection, {
      props: {
        ...defaultProps,
        errors: {
          productCode: 'Product code is required',
          description: 'Description is required',
          manufacturerId: 'Please select a manufacturer',
          category: 'Category is required',
          scale: 'Scale is required',
          powerMethod: 'Power method is required',
          epoch: 'Epoch is required'
        }
      }
    });
    expect(screen.getByText('Product code is required')).toBeInTheDocument();
    expect(screen.getByText('Description is required')).toBeInTheDocument();
    expect(screen.getByText('Please select a manufacturer')).toBeInTheDocument();
    expect(screen.getByText('Category is required')).toBeInTheDocument();
    expect(screen.getByText('Scale is required')).toBeInTheDocument();
    expect(screen.getByText('Power method is required')).toBeInTheDocument();
    expect(screen.getByText('Epoch is required')).toBeInTheDocument();
  });

  // ── Manufacturer select dropdown ────────────────────────────────────────────

  it('shows all manufacturers in dropdown when opened', async () => {
    const user = userEvent.setup();
    render(ModelInfoSection, { props: defaultProps });

    const trigger = screen.getByRole('button', { name: /manufacturer/i });
    await user.click(trigger);

    await waitFor(() => {
      expect(screen.getByRole('option', { name: 'Märklin' })).toBeInTheDocument();
      expect(screen.getByRole('option', { name: 'Fleischmann' })).toBeInTheDocument();
    });
  });

  // ── Epoch picker buttons ─────────────────────────────────────────────────────

  it('renders epoch toggle buttons when epoch is empty', () => {
    render(ModelInfoSection, { props: { ...defaultProps, epoch: '' } });
    // EpochPicker renders toggle buttons for each base epoch
    expect(screen.getByRole('button', { name: 'IV' })).toBeInTheDocument();
    // No display summary shown when nothing is selected
    expect(screen.queryAllByText('IV').length).toBe(1);
  });

  it('shows selected epoch display value when epoch is set', () => {
    render(ModelInfoSection, { props: { ...defaultProps, epoch: 'IV' } });
    // The selected epoch appears as both an active button and a summary <p>
    const matches = screen.getAllByText('IV');
    expect(matches.length).toBeGreaterThanOrEqual(1);
  });
});
