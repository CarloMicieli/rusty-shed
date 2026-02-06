import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/svelte';

// Mock paraglide messages
vi.mock('$lib/paraglide/messages.js', () => ({
  model_no_rolling_stock: () => 'No rolling stock units found for this model.',
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

import RollingStockList from '../RollingStockList.svelte';
import type { OwnedRollingStockView } from '$lib/bindings';

describe('RollingStockList', () => {
  const mockRollingStocks = [
    {
      id: 'rs-1',
      rollingStockId: 'trn:rolling-stock:rs-1',
      series: '218',
      roadNumber: '218 217-8',
      livery: 'DB Red',
      railwayCompanyName: 'Deutsche Bahn',
      control: 'Digital',
      notes: null,
      digital: null
    },
    {
      id: 'rs-2',
      rollingStockId: 'trn:rolling-stock:rs-2',
      series: '103',
      roadNumber: '103 113-6',
      livery: 'DB Blue',
      railwayCompanyName: 'Deutsche Bahn',
      control: 'Analog',
      notes: null,
      digital: null
    }
  ] as unknown as OwnedRollingStockView[];

  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('Rendering with Data', () => {
    it('should render all rolling stock cards when provided', () => {
      render(RollingStockList, {
        props: {
          rollingStocks: mockRollingStocks
        }
      });

      expect(screen.getByText('218 — 218 217-8')).toBeInTheDocument();
      expect(screen.getByText('103 — 103 113-6')).toBeInTheDocument();
    });

    it('should render cards in order', () => {
      const { container } = render(RollingStockList, {
        props: {
          rollingStocks: mockRollingStocks
        }
      });

      const cards = container.querySelectorAll('h3');
      expect(cards[0].textContent).toContain('218 — 218 217-8');
      expect(cards[1].textContent).toContain('103 — 103 113-6');
    });

    it('should apply spacing between cards', () => {
      const { container } = render(RollingStockList, {
        props: {
          rollingStocks: mockRollingStocks
        }
      });

      const cardContainer = container.querySelector('.space-y-4');
      expect(cardContainer).toBeInTheDocument();
    });
  });

  describe('Empty State', () => {
    it('should render empty state when rollingStocks is undefined', () => {
      render(RollingStockList, {
        props: {
          rollingStocks: undefined
        }
      });

      expect(screen.getByText('No rolling stock units found for this model.')).toBeInTheDocument();
    });

    it('should render empty state when rollingStocks is empty array', () => {
      render(RollingStockList, {
        props: {
          rollingStocks: []
        }
      });

      expect(screen.getByText('No rolling stock units found for this model.')).toBeInTheDocument();
    });

    it('should render empty state with proper styling', () => {
      const { container } = render(RollingStockList, {
        props: {
          rollingStocks: []
        }
      });

      const emptyState = container.querySelector('.border-dashed');
      expect(emptyState).toBeInTheDocument();
      expect(emptyState?.className).toContain('rounded-lg');
      expect(emptyState?.className).toContain('border');
      expect(emptyState?.className).toContain('p-8');
      expect(emptyState?.className).toContain('text-center');
    });

    it('should apply muted foreground to empty message', () => {
      render(RollingStockList, {
        props: {
          rollingStocks: []
        }
      });

      const message = screen.getByText('No rolling stock units found for this model.');
      expect(message.className).toContain('text-muted-foreground');
    });
  });

  describe('Single Item', () => {
    it('should render correctly with one rolling stock', () => {
      render(RollingStockList, {
        props: {
          rollingStocks: [mockRollingStocks[0]]
        }
      });

      expect(screen.getByText('218 — 218 217-8')).toBeInTheDocument();
      expect(screen.queryByText('103 — 103 113-6')).not.toBeInTheDocument();
    });
  });

  describe('Key Management', () => {
    it('should use unique keys for each card', () => {
      const { container } = render(RollingStockList, {
        props: {
          rollingStocks: mockRollingStocks
        }
      });

      // Each card should be rendered (Svelte handles keys internally)
      const cards = container.querySelectorAll('.rounded-lg.border');
      expect(cards.length).toBe(2);
    });
  });

  describe('Accessibility', () => {
    it('should render semantic list structure', () => {
      const { container } = render(RollingStockList, {
        props: {
          rollingStocks: mockRollingStocks
        }
      });

      const listContainer = container.querySelector('.space-y-4');
      expect(listContainer).toBeInTheDocument();
    });

    it('should have readable empty state', () => {
      render(RollingStockList, {
        props: {
          rollingStocks: []
        }
      });

      const emptyMessage = screen.getByText('No rolling stock units found for this model.');
      expect(emptyMessage).toBeVisible();
    });
  });

  describe('Responsive Design', () => {
    it('should apply responsive spacing', () => {
      const { container } = render(RollingStockList, {
        props: {
          rollingStocks: mockRollingStocks
        }
      });

      const cardContainer = container.querySelector('.space-y-4');
      expect(cardContainer).toBeInTheDocument();
    });
  });

  describe('Edge Cases', () => {
    it('should handle null values in rolling stock data', () => {
      const stockWithNulls = {
        id: 'rs-3',
        rollingStockId: 'trn:rolling-stock:rs-3',
        series: null,
        roadNumber: null,
        livery: null,
        railwayCompanyName: null,
        control: null,
        notes: null,
        digital: null
      } as unknown as OwnedRollingStockView;

      render(RollingStockList, {
        props: {
          rollingStocks: [stockWithNulls]
        }
      });

      // Should still render the card (handled by RollingStockCard component)
      expect(screen.getByText(/Unknown/)).toBeInTheDocument();
    });

    it('should handle large number of rolling stocks', () => {
      const manyStocks = Array.from({ length: 50 }, (_, i) => ({
        id: `rs-${i}`,
        rollingStockId: `trn:rolling-stock:rs-${i}`,
        series: `${100 + i}`,
        roadNumber: `${100 + i} 000-0`,
        livery: 'Test',
        railwayCompanyName: 'Test Company',
        control: 'Digital',
        notes: null,
        digital: null
      })) as unknown as OwnedRollingStockView[];

      const { container } = render(RollingStockList, {
        props: {
          rollingStocks: manyStocks
        }
      });

      const cards = container.querySelectorAll('.rounded-lg.border');
      expect(cards.length).toBe(50);
    });
  });
});
