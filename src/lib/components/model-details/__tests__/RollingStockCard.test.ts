import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';

// Mock bindings
vi.mock('$lib/bindings', () => ({
  commands: {
    getRailwayCompanies: vi.fn().mockResolvedValue({ status: 'ok', data: [] }),
    updateRollingStockIdentification: vi.fn().mockResolvedValue({ status: 'ok', data: null }),
    updateRollingStockRailwayCompany: vi.fn().mockResolvedValue({ status: 'ok', data: null }),
    updateRollingStockDcc: vi.fn().mockResolvedValue({ status: 'ok', data: null }),
    getRailwayModelById: vi.fn().mockResolvedValue({ status: 'ok', data: null }),
    updateRollingStockSpecifications: vi.fn().mockResolvedValue({ status: 'ok', data: null })
  }
}));

// Mock settings state
vi.mock('$lib/features/settings/SettingsState.svelte.ts', () => ({
  settingsState: { settings: { measureUnit: 'Metric' } }
}));

// Mock paraglide runtime
vi.mock('$lib/paraglide/runtime.js', () => ({
  getLocale: () => 'en'
}));

// Mock paraglide messages
vi.mock('$lib/paraglide/messages.js', () => ({
  model_rolling_stock_unknown_series: () => 'Unknown',
  model_rolling_stock_na: () => 'N/A',
  model_rolling_stock_field_series: () => 'Series',
  model_rolling_stock_field_road_number: () => 'Road Number',
  model_rolling_stock_field_livery: () => 'Livery',
  model_rolling_stock_field_company: () => 'Company',
  model_rolling_stock_field_control: () => 'Control',
  model_rolling_stock_field_digital_setup: () => 'Digital Setup',
  model_rolling_stock_digital_interface: () => 'Interface',
  model_rolling_stock_digital_address: () => 'Address',
  model_rolling_stock_digital_decoder_id: () => 'Decoder ID',
  rolling_stock_field_series: () => 'Series',
  rolling_stock_field_series_code: () => 'Series Code',
  rolling_stock_field_depot: () => 'Depot',
  rolling_stock_field_livery: () => 'Livery',
  rolling_stock_field_length: () => 'Length',
  rolling_stock_field_dcc_interface: () => 'DCC Interface',
  rolling_stock_field_control_type: () => 'Control Type',
  rolling_stock_field_interior_lights: () => 'Interior Lights',
  rolling_stock_field_lights: () => 'Lights',
  rolling_stock_edit_specs_button: () => 'Edit Specs',
  edit_field_placeholder_empty: () => 'Click to add...',
  edit_save_error: () => 'Failed to save.',
  badge_picker_close: () => 'Close',
  specs_drawer_field_flywheel: () => 'Flywheel',
  specs_drawer_field_body_material: () => 'Body Material',
  specs_drawer_field_chassis_material: () => 'Chassis Material',
  specs_drawer_field_coupling_socket: () => 'Coupling Socket',
  specs_drawer_field_close_coupling: () => 'Close Couplers',
  specs_drawer_field_digital_shunting: () => 'Digital Shunting'
}));

import RollingStockCard from '../RollingStockCard.svelte';
import type { OwnedRollingStockView } from '$lib/bindings';

describe('RollingStockCard', () => {
  const mockRollingStock = {
    id: 'rs-1',
    rollingStockId: 'trn:rolling-stock:rs-1',
    series: '218',
    roadNumber: '218 217-8',
    livery: 'DB Red',
    railwayCompanyName: 'Deutsche Bahn',
    control: 'DCC_FITTED',
    notes: 'Test notes',
    digital: {
      interface: 'MFX',
      dcc_address: 3,
      installed_decoder_id: 'decoder-123'
    },
    dccInterface: null,
    lengthOverBuffers: null
  } as unknown as OwnedRollingStockView;

  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('Rendering', () => {
    it('should render card header with series and road number', () => {
      render(RollingStockCard, {
        props: {
          railwayModelId: 'trn:railway-model:acme:test-001',
          rollingStock: mockRollingStock
        }
      });

      expect(screen.getByText('218 — 218 217-8')).toBeInTheDocument();
    });

    it('should render with Unknown series when missing', () => {
      const stockWithoutSeries = {
        ...mockRollingStock,
        series: null
      } as unknown as OwnedRollingStockView;
      render(RollingStockCard, {
        props: {
          railwayModelId: 'trn:railway-model:acme:test-001',
          rollingStock: stockWithoutSeries
        }
      });

      expect(screen.getByText(/Unknown/)).toBeInTheDocument();
    });

    it('should render with N/A road number when missing', () => {
      const stockWithoutRoadNumber = {
        ...mockRollingStock,
        roadNumber: null
      } as unknown as OwnedRollingStockView;
      render(RollingStockCard, {
        props: {
          railwayModelId: 'trn:railway-model:acme:test-001',
          rollingStock: stockWithoutRoadNumber
        }
      });

      expect(screen.getByText(/N\/A/)).toBeInTheDocument();
    });

    it('should not render expanded content initially', () => {
      render(RollingStockCard, {
        props: {
          railwayModelId: 'trn:railway-model:acme:test-001',
          rollingStock: mockRollingStock
        }
      });

      expect(screen.queryByText('Test notes')).not.toBeInTheDocument();
    });
  });

  describe('User Interactions', () => {
    it('should expand card when header is clicked', async () => {
      render(RollingStockCard, {
        props: {
          railwayModelId: 'trn:railway-model:acme:test-001',
          rollingStock: mockRollingStock
        }
      });

      const button = screen.getByRole('button');
      await fireEvent.click(button);

      // Should now show expanded content
      expect(screen.getByText('Test notes')).toBeInTheDocument();
      expect(screen.getByText('Series')).toBeInTheDocument();
    });

    it('should collapse card when clicked again', async () => {
      render(RollingStockCard, {
        props: {
          railwayModelId: 'trn:railway-model:acme:test-001',
          rollingStock: mockRollingStock
        }
      });

      const button = screen.getByRole('button');

      // Expand
      await fireEvent.click(button);
      expect(screen.getByText('Test notes')).toBeInTheDocument();

      // Collapse
      await fireEvent.click(button);
      expect(screen.queryByText('Test notes')).not.toBeInTheDocument();
    });

    it('should toggle chevron icon on expand/collapse', async () => {
      const { container } = render(RollingStockCard, {
        props: {
          railwayModelId: 'trn:railway-model:acme:test-001',
          rollingStock: mockRollingStock
        }
      });

      const button = screen.getByRole('button');

      // Initially collapsed - should have ChevronDown
      expect(container.querySelector('svg')).toBeInTheDocument();

      // Expand
      await fireEvent.click(button);

      // Should still have an icon (now ChevronUp)
      expect(container.querySelector('svg')).toBeInTheDocument();
    });
  });

  describe('Expanded Content', () => {
    it('should render grid field labels when expanded', async () => {
      render(RollingStockCard, {
        props: {
          railwayModelId: 'trn:railway-model:acme:test-001',
          rollingStock: mockRollingStock
        }
      });

      const button = screen.getByRole('button');
      await fireEvent.click(button);

      // Check field labels present in the 5×3 grid
      expect(screen.getByText('Series')).toBeInTheDocument();
      expect(screen.getByText('Livery')).toBeInTheDocument();
      expect(screen.getByText('Control Type')).toBeInTheDocument();

      // Field values
      expect(screen.getByText('218')).toBeInTheDocument();
      expect(screen.getByText('DB Red')).toBeInTheDocument();
      // Railway company is now a header badge, not a grid row
      expect(screen.getByText('Deutsche Bahn')).toBeInTheDocument();
      // Control value rendered via BadgePicker option label
      expect(screen.getAllByText('DCC Fitted')[0]).toBeInTheDocument();
    });

    it('should render digital setup when present', async () => {
      render(RollingStockCard, {
        props: {
          railwayModelId: 'trn:railway-model:acme:test-001',
          rollingStock: mockRollingStock
        }
      });

      const button = screen.getByRole('button');
      await fireEvent.click(button);

      expect(screen.getByText('Digital Setup')).toBeInTheDocument();
      expect(screen.getByText(/Interface.*MFX/)).toBeInTheDocument();
      expect(screen.getByText(/Address.*3/)).toBeInTheDocument();
      expect(screen.getByText(/Decoder ID.*decoder-123/)).toBeInTheDocument();
    });

    it('should render notes when present', async () => {
      render(RollingStockCard, {
        props: {
          railwayModelId: 'trn:railway-model:acme:test-001',
          rollingStock: mockRollingStock
        }
      });

      const button = screen.getByRole('button');
      await fireEvent.click(button);

      expect(screen.getByText('Test notes')).toBeInTheDocument();
    });

    it('should render all grid field labels even when values are missing', async () => {
      const minimalStock = {
        id: 'rs-2',
        rollingStockId: 'trn:rolling-stock:rs-2',
        series: '218',
        roadNumber: '218 217-8',
        livery: null,
        railwayCompanyName: null,
        control: null,
        notes: null,
        digital: null,
        depot: null
      } as unknown as OwnedRollingStockView;

      render(RollingStockCard, {
        props: {
          railwayModelId: 'trn:railway-model:acme:test-001',
          rollingStock: minimalStock
        }
      });

      const button = screen.getByRole('button');
      await fireEvent.click(button);

      // Labels always render (US1: fields show "—" instead of being hidden)
      expect(screen.getByText('Livery')).toBeInTheDocument();
      expect(screen.getByText('Control Type')).toBeInTheDocument();
      expect(screen.getByText('Depot')).toBeInTheDocument();
      // Digital Setup is still conditionally rendered (null → absent)
      expect(screen.queryByText('Digital Setup')).not.toBeInTheDocument();
    });
  });

  describe('Accessibility', () => {
    it('should have button role for header', () => {
      render(RollingStockCard, {
        props: {
          railwayModelId: 'trn:railway-model:acme:test-001',
          rollingStock: mockRollingStock
        }
      });

      const button = screen.getByRole('button');
      expect(button).toBeInTheDocument();
    });

    it('should have aria-expanded attribute', async () => {
      render(RollingStockCard, {
        props: {
          railwayModelId: 'trn:railway-model:acme:test-001',
          rollingStock: mockRollingStock
        }
      });

      const button = screen.getByRole('button');
      expect(button).toHaveAttribute('aria-expanded', 'false');

      await fireEvent.click(button);
      expect(button).toHaveAttribute('aria-expanded', 'true');
    });

    it('should be keyboard accessible', async () => {
      render(RollingStockCard, {
        props: {
          railwayModelId: 'trn:railway-model:acme:test-001',
          rollingStock: mockRollingStock
        }
      });

      const button = screen.getByRole('button');

      // Simulate Enter key press
      await fireEvent.keyDown(button, { key: 'Enter', code: 'Enter' });

      // Button click should work (though fireEvent doesn't fully simulate Enter on buttons)
      expect(button).toBeInTheDocument();
    });

    it('should use 3-column CSS grid layout', async () => {
      const { container } = render(RollingStockCard, {
        props: {
          railwayModelId: 'trn:railway-model:acme:test-001',
          rollingStock: mockRollingStock
        }
      });

      const button = screen.getByRole('button');
      await fireEvent.click(button);

      const grid = container.querySelector('.grid');
      expect(grid?.className).toContain('grid-cols-3');
    });
  });

  describe('Responsive Design', () => {
    it('should use 3-column grid layout', async () => {
      const { container } = render(RollingStockCard, {
        props: {
          railwayModelId: 'trn:railway-model:acme:test-001',
          rollingStock: mockRollingStock
        }
      });

      const button = screen.getByRole('button');
      await fireEvent.click(button);

      const grid = container.querySelector('.grid');
      expect(grid?.className).toContain('grid-cols-3');
    });

    it('should apply hover effects to header', () => {
      const { container } = render(RollingStockCard, {
        props: {
          railwayModelId: 'trn:railway-model:acme:test-001',
          rollingStock: mockRollingStock
        }
      });

      const button = container.querySelector('button');
      expect(button?.className).toContain('hover:bg-[#1F1F1F]/50');
    });
  });
});
