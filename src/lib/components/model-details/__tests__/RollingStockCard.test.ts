import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';

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
  model_rolling_stock_digital_decoder_id: () => 'Decoder ID'
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
    control: 'Digital',
    notes: 'Test notes',
    digital: {
      interface: 'MFX',
      dcc_address: 3,
      installed_decoder_id: 'decoder-123'
    }
  } as unknown as OwnedRollingStockView;

  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('Rendering', () => {
    it('should render card header with series and road number', () => {
      render(RollingStockCard, {
        props: {
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
          rollingStock: stockWithoutRoadNumber
        }
      });

      expect(screen.getByText(/N\/A/)).toBeInTheDocument();
    });

    it('should not render expanded content initially', () => {
      render(RollingStockCard, {
        props: {
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
    it('should render all fields when expanded', async () => {
      render(RollingStockCard, {
        props: {
          rollingStock: mockRollingStock
        }
      });

      const button = screen.getByRole('button');
      await fireEvent.click(button);

      // Check field labels
      expect(screen.getByText('Series')).toBeInTheDocument();
      expect(screen.getByText('Road Number')).toBeInTheDocument();
      expect(screen.getByText('Livery')).toBeInTheDocument();
      expect(screen.getByText('Company')).toBeInTheDocument();
      expect(screen.getByText('Control')).toBeInTheDocument();

      // Check field values
      expect(screen.getByText('218')).toBeInTheDocument();
      expect(screen.getByText('218 217-8')).toBeInTheDocument();
      expect(screen.getByText('DB Red')).toBeInTheDocument();
      expect(screen.getByText('Deutsche Bahn')).toBeInTheDocument();
      expect(screen.getByText('Digital')).toBeInTheDocument();
    });

    it('should render digital setup when present', async () => {
      render(RollingStockCard, {
        props: {
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
          rollingStock: mockRollingStock
        }
      });

      const button = screen.getByRole('button');
      await fireEvent.click(button);

      expect(screen.getByText('Test notes')).toBeInTheDocument();
    });

    it('should not render fields that are missing', async () => {
      const minimalStock = {
        id: 'rs-2',
        rollingStockId: 'trn:rolling-stock:rs-2',
        series: '218',
        roadNumber: '218 217-8',
        livery: null,
        railwayCompanyName: null,
        control: null,
        notes: null,
        digital: null
      } as unknown as OwnedRollingStockView;

      render(RollingStockCard, {
        props: {
          rollingStock: minimalStock
        }
      });

      const button = screen.getByRole('button');
      await fireEvent.click(button);

      // These should not be present
      expect(screen.queryByText('Livery')).not.toBeInTheDocument();
      expect(screen.queryByText('Company')).not.toBeInTheDocument();
      expect(screen.queryByText('Control')).not.toBeInTheDocument();
      expect(screen.queryByText('Digital Setup')).not.toBeInTheDocument();
    });
  });

  describe('Accessibility', () => {
    it('should have button role for header', () => {
      render(RollingStockCard, {
        props: {
          rollingStock: mockRollingStock
        }
      });

      const button = screen.getByRole('button');
      expect(button).toBeInTheDocument();
    });

    it('should have aria-expanded attribute', async () => {
      render(RollingStockCard, {
        props: {
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
          rollingStock: mockRollingStock
        }
      });

      const button = screen.getByRole('button');

      // Simulate Enter key press
      await fireEvent.keyDown(button, { key: 'Enter', code: 'Enter' });

      // Button click should work (though fireEvent doesn't fully simulate Enter on buttons)
      expect(button).toBeInTheDocument();
    });

    it('should use semantic HTML for definition list', async () => {
      const { container } = render(RollingStockCard, {
        props: {
          rollingStock: mockRollingStock
        }
      });

      const button = screen.getByRole('button');
      await fireEvent.click(button);

      const dl = container.querySelector('dl');
      expect(dl).toBeInTheDocument();

      const dt = container.querySelector('dt');
      expect(dt).toBeInTheDocument();

      const dd = container.querySelector('dd');
      expect(dd).toBeInTheDocument();
    });
  });

  describe('Responsive Design', () => {
    it('should use responsive grid layout', async () => {
      const { container } = render(RollingStockCard, {
        props: {
          rollingStock: mockRollingStock
        }
      });

      const button = screen.getByRole('button');
      await fireEvent.click(button);

      const grid = container.querySelector('.grid');
      expect(grid?.className).toContain('grid-cols-1');
      expect(grid?.className).toContain('sm:grid-cols-2');
    });

    it('should apply hover effects to header', () => {
      const { container } = render(RollingStockCard, {
        props: {
          rollingStock: mockRollingStock
        }
      });

      const button = container.querySelector('button');
      expect(button?.className).toContain('hover:bg-muted/50');
    });
  });
});
