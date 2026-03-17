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
  wishlist_category_locomotives: () => 'Locomotives',
  wishlist_category_train_sets: () => 'Train Sets',
  wishlist_category_starter_sets: () => 'Starter Sets',
  wishlist_category_freight_cars: () => 'Freight Cars',
  wishlist_category_passenger_cars: () => 'Passenger Cars',
  wishlist_category_electric_multiple_units: () => 'Electric Multiple Units',
  wishlist_category_railcars: () => 'Railcars',
  wishlist_power_ac: () => 'AC',
  wishlist_power_dc: () => 'DC',
  wishlist_power_trix_express: () => 'Trix Express'
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

  // ── Section header ──────────────────────────────────────────────────────────

  it('renders section header label', () => {
    render(ModelInfoSection, { props: defaultProps });
    expect(screen.getByText('Model Details')).toBeInTheDocument();
  });

  // ── All fields render ───────────────────────────────────────────────────────

  it('renders all required field labels', () => {
    render(ModelInfoSection, { props: defaultProps });

    // "Manufacturer" appears as both a label span and button aria-label — check ≥1 element exists
    expect(screen.getAllByText(/manufacturer/i).length).toBeGreaterThan(0);
    expect(screen.getByLabelText(/product code/i)).toBeInTheDocument();
    expect(screen.getByLabelText(/description/i)).toBeInTheDocument();
    expect(screen.getAllByText(/category/i).length).toBeGreaterThan(0);
    expect(screen.getAllByText(/scale/i).length).toBeGreaterThan(0);
    expect(screen.getAllByText(/power method/i).length).toBeGreaterThan(0);
    expect(screen.getAllByText(/epoch/i).length).toBeGreaterThan(0);
  });

  // ── Loading state ────────────────────────────────────────────────────────────

  it('shows loading text when isLoading=true', () => {
    render(ModelInfoSection, { props: { ...defaultProps, isLoading: true } });
    expect(screen.getByText('Loading...')).toBeInTheDocument();
  });

  it('shows manufacturer select when isLoading=false', () => {
    render(ModelInfoSection, { props: { ...defaultProps, isLoading: false } });
    expect(screen.queryByText('Loading...')).toBeNull();
    // Manufacturer trigger button should be visible
    expect(screen.getByRole('button', { name: /manufacturer/i })).toBeInTheDocument();
  });

  // ── Scale display with ratio ─────────────────────────────────────────────────

  it('shows scale ratio annotation in trigger (H0 → H0 (1:87))', async () => {
    render(ModelInfoSection, { props: { ...defaultProps, scale: 'H0' } });
    // The trigger should display the scale with ratio
    expect(screen.getByText('H0 (1:87)')).toBeInTheDocument();
  });

  it('shows N scale ratio annotation (N → N (1:160))', async () => {
    render(ModelInfoSection, { props: { ...defaultProps, scale: 'N' } });
    expect(screen.getByText('N (1:160)')).toBeInTheDocument();
  });

  it('shows Z scale ratio annotation (Z → Z (1:220))', async () => {
    render(ModelInfoSection, { props: { ...defaultProps, scale: 'Z' } });
    expect(screen.getByText('Z (1:220)')).toBeInTheDocument();
  });

  // ── Category labels ─────────────────────────────────────────────────────────

  it('shows Paraglide category label for selected category', () => {
    render(ModelInfoSection, { props: { ...defaultProps, category: 'LOCOMOTIVES' } });
    expect(screen.getByText('Locomotives')).toBeInTheDocument();
  });

  it('shows Paraglide power method label for selected method', () => {
    render(ModelInfoSection, { props: { ...defaultProps, powerMethod: 'AC' } });
    expect(screen.getByText('AC')).toBeInTheDocument();
  });

  // ── Text input bindings ──────────────────────────────────────────────────────

  it('product code input reflects initial value', () => {
    render(ModelInfoSection, { props: { ...defaultProps, productCode: '37858' } });
    expect(screen.getByLabelText(/product code/i)).toHaveValue('37858');
  });

  it('description input reflects initial value', () => {
    render(ModelInfoSection, { props: { ...defaultProps, description: 'BR 218' } });
    expect(screen.getByLabelText(/description/i)).toHaveValue('BR 218');
  });

  // ── Error messages ───────────────────────────────────────────────────────────

  it('displays productCode error when provided', () => {
    render(ModelInfoSection, {
      props: { ...defaultProps, errors: { productCode: 'Product code is required' } }
    });
    expect(screen.getByText('Product code is required')).toBeInTheDocument();
  });

  it('displays description error when provided', () => {
    render(ModelInfoSection, {
      props: { ...defaultProps, errors: { description: 'Description is required' } }
    });
    expect(screen.getByText('Description is required')).toBeInTheDocument();
  });

  it('displays manufacturerId error when provided', () => {
    render(ModelInfoSection, {
      props: { ...defaultProps, errors: { manufacturerId: 'Please select a manufacturer' } }
    });
    expect(screen.getByText('Please select a manufacturer')).toBeInTheDocument();
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

  // ── Epoch placeholder ────────────────────────────────────────────────────────

  it('shows epoch placeholder when epoch is empty', () => {
    render(ModelInfoSection, { props: { ...defaultProps, epoch: '' } });
    expect(screen.getByText('e.g., IV or III/IV')).toBeInTheDocument();
  });

  it('shows selected epoch value when epoch is set', () => {
    render(ModelInfoSection, { props: { ...defaultProps, epoch: 'IV' } });
    expect(screen.getByText('IV')).toBeInTheDocument();
  });
});
