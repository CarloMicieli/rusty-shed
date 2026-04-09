import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent, cleanup } from '@testing-library/svelte';

vi.mock('$lib/bindings', () => ({
  commands: {}
}));

vi.mock('$lib/paraglide/runtime.js', () => ({
  getLocale: () => 'en'
}));

vi.mock('$lib/paraglide/messages.js', () => ({
  rolling_stock_edit_specs_button: () => 'Edit Specs',
  road_number: () => 'Road Number',
  edit_field_placeholder_empty: () => 'Click to add...',
  edit_save_error: () => 'Failed to save.',
  badge_picker_close: () => 'Close'
}));

import RollingStockListItemHeader from '../components/RollingStockListItemHeader.svelte';
import type { RollingStock } from '$lib/types/railway-model';

const mockUnit: RollingStock = {
  id: 'rs-1',
  ownedRollingStockId: 'owned-rs-1',
  currentCouplerId: null,
  railway_model_id: 1,
  railway_company: 'FS',
  country_code: 'IT',
  series_code: 'trn:rolling-stock:rs-1',
  series_name: 'E.645',
  rolling_stock_type: null,
  category: 'LOCOMOTIVE',
  subcategory: 'ELECTRIC_LOCOMOTIVE',
  road_number: 'E.645 018',
  depot: null,
  livery: null,
  length_mm: null,
  control_type: null,
  dcc_interface: null,
  coupling_type: null,
  close_couplers: null,
  digital_shunting: null
};

const defaultProps = {
  unit: mockUnit,
  editable: false,
  formState: undefined,
  specLoaded: false,
  onSaveRoadNumber: vi.fn().mockResolvedValue(undefined),
  onSaveCategory: vi.fn().mockResolvedValue(undefined),
  onSaveSubcategory: vi.fn().mockResolvedValue(undefined),
  onEditSpecs: vi.fn()
};

describe('RollingStockListItemHeader', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
  });

  describe('Identity column', () => {
    it('renders road number from unit', () => {
      render(RollingStockListItemHeader, { props: defaultProps });
      expect(screen.getByText('E.645 018')).toBeInTheDocument();
    });

    it('renders railway company from unit', () => {
      render(RollingStockListItemHeader, { props: defaultProps });
      expect(screen.getByText('FS')).toBeInTheDocument();
    });

    it('renders em-dash when road number is null', () => {
      render(RollingStockListItemHeader, {
        props: { ...defaultProps, unit: { ...mockUnit, road_number: null } }
      });
      expect(screen.getByText('—')).toBeInTheDocument();
    });

    it('does not render railway company span when null', () => {
      render(RollingStockListItemHeader, {
        props: { ...defaultProps, unit: { ...mockUnit, railway_company: null } }
      });
      expect(screen.queryByText('FS')).not.toBeInTheDocument();
    });
  });

  describe('Classification column', () => {
    it('renders category label', () => {
      render(RollingStockListItemHeader, { props: defaultProps });
      expect(screen.getByText(/Locomotive/)).toBeInTheDocument();
    });

    it('renders subcategory label when present', () => {
      render(RollingStockListItemHeader, { props: defaultProps });
      expect(screen.getByText(/Electric Locomotive/)).toBeInTheDocument();
    });

    it('renders em-dash when category is null', () => {
      render(RollingStockListItemHeader, {
        props: { ...defaultProps, unit: { ...mockUnit, category: null, subcategory: null } }
      });
      expect(screen.getByText('—')).toBeInTheDocument();
    });
  });

  describe('Edit Specs button', () => {
    it('is visible when editable is true', () => {
      render(RollingStockListItemHeader, { props: { ...defaultProps, editable: true } });
      expect(screen.getByRole('button', { name: /Edit Specs/i })).toBeInTheDocument();
    });

    it('is hidden when editable is false', () => {
      render(RollingStockListItemHeader, { props: defaultProps });
      expect(screen.queryByRole('button', { name: /Edit Specs/i })).not.toBeInTheDocument();
    });

    it('calls onEditSpecs when clicked', async () => {
      const onEditSpecs = vi.fn();
      render(RollingStockListItemHeader, {
        props: { ...defaultProps, editable: true, onEditSpecs }
      });
      await fireEvent.click(screen.getByRole('button', { name: /Edit Specs/i }));
      expect(onEditSpecs).toHaveBeenCalledOnce();
    });
  });

  describe('Layout', () => {
    it('renders a 3-column grid', () => {
      const { container } = render(RollingStockListItemHeader, { props: defaultProps });
      const grid = container.querySelector('.grid-cols-3');
      expect(grid).toBeInTheDocument();
    });

    it('centers items vertically', () => {
      const { container } = render(RollingStockListItemHeader, { props: defaultProps });
      const grid = container.querySelector('.grid');
      expect(grid?.className).toContain('items-center');
    });
  });
});
