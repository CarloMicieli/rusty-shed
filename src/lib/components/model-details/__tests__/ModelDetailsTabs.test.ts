import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';

// Mock paraglide messages
vi.mock('$lib/paraglide/messages.js', () => ({
  model_details_tab: () => 'Details',
  model_rolling_stock_tab: () => 'Rolling Stock'
}));

import ModelDetailsTabs from '../ModelDetailsTabs.svelte';

describe('ModelDetailsTabs', () => {
  let onTabChangeMock: (tab: 'details' | 'rolling-stock') => void;

  beforeEach(() => {
    onTabChangeMock = vi.fn() as (tab: 'details' | 'rolling-stock') => void;
    vi.clearAllMocks();
  });

  describe('Rendering', () => {
    it('should render both tab buttons', () => {
      render(ModelDetailsTabs, {
        props: {
          activeTab: 'details',
          onTabChange: onTabChangeMock
        }
      });

      expect(screen.getByText('Details')).toBeInTheDocument();
      expect(screen.getByText('Rolling Stock')).toBeInTheDocument();
    });

    it('should highlight active tab', () => {
      const { container: _container } = render(ModelDetailsTabs, {
        props: {
          activeTab: 'details',
          onTabChange: onTabChangeMock
        }
      });

      const detailsButton = screen.getByText('Details').closest('button');
      expect(detailsButton?.className).toContain('border-primary');
      expect(detailsButton?.className).toContain('text-primary');
    });

    it('should not highlight inactive tab', () => {
      const { container: _container } = render(ModelDetailsTabs, {
        props: {
          activeTab: 'details',
          onTabChange: onTabChangeMock
        }
      });

      const rollingStockButton = screen.getByText('Rolling Stock').closest('button');
      expect(rollingStockButton?.className).toContain('border-transparent');
      expect(rollingStockButton?.className).toContain('text-muted-foreground');
    });

    it('should set aria-current on active tab', () => {
      render(ModelDetailsTabs, {
        props: {
          activeTab: 'details',
          onTabChange: onTabChangeMock
        }
      });

      const detailsButton = screen.getByText('Details').closest('button');
      expect(detailsButton).toHaveAttribute('aria-current', 'page');

      const rollingStockButton = screen.getByText('Rolling Stock').closest('button');
      expect(rollingStockButton).not.toHaveAttribute('aria-current');
    });
  });

  describe('User Interactions', () => {
    it('should call onTabChange when Details tab is clicked', async () => {
      render(ModelDetailsTabs, {
        props: {
          activeTab: 'rolling-stock',
          onTabChange: onTabChangeMock
        }
      });

      const detailsButton = screen.getByText('Details');
      await fireEvent.click(detailsButton);

      expect(onTabChangeMock).toHaveBeenCalledWith('details');
      expect(onTabChangeMock).toHaveBeenCalledTimes(1);
    });

    it('should call onTabChange when Rolling Stock tab is clicked', async () => {
      render(ModelDetailsTabs, {
        props: {
          activeTab: 'details',
          onTabChange: onTabChangeMock
        }
      });

      const rollingStockButton = screen.getByText('Rolling Stock');
      await fireEvent.click(rollingStockButton);

      expect(onTabChangeMock).toHaveBeenCalledWith('rolling-stock');
      expect(onTabChangeMock).toHaveBeenCalledTimes(1);
    });

    it('should handle multiple tab switches', async () => {
      render(ModelDetailsTabs, {
        props: {
          activeTab: 'details',
          onTabChange: onTabChangeMock
        }
      });

      const detailsButton = screen.getByText('Details');
      const rollingStockButton = screen.getByText('Rolling Stock');

      await fireEvent.click(rollingStockButton);
      expect(onTabChangeMock).toHaveBeenCalledWith('rolling-stock');

      await fireEvent.click(detailsButton);
      expect(onTabChangeMock).toHaveBeenCalledWith('details');

      expect(onTabChangeMock).toHaveBeenCalledTimes(2);
    });
  });

  describe('Accessibility', () => {
    it('should have proper navigation landmark', () => {
      const { container } = render(ModelDetailsTabs, {
        props: {
          activeTab: 'details',
          onTabChange: onTabChangeMock
        }
      });

      const nav = container.querySelector('nav[aria-label="Tabs"]');
      expect(nav).toBeInTheDocument();
    });

    it('should have button type for tabs', () => {
      render(ModelDetailsTabs, {
        props: {
          activeTab: 'details',
          onTabChange: onTabChangeMock
        }
      });

      const buttons = screen.getAllByRole('button');
      buttons.forEach((button) => {
        expect(button).toHaveAttribute('type', 'button');
      });
    });

    it('should be keyboard accessible', () => {
      render(ModelDetailsTabs, {
        props: {
          activeTab: 'details',
          onTabChange: onTabChangeMock
        }
      });

      const buttons = screen.getAllByRole('button');
      expect(buttons).toHaveLength(2);

      // Both buttons should be focusable
      buttons.forEach((button) => {
        expect(button).toBeInstanceOf(HTMLButtonElement);
      });
    });
  });

  describe('Visual States', () => {
    it('should apply hover styles to inactive tabs', () => {
      const { container: _container } = render(ModelDetailsTabs, {
        props: {
          activeTab: 'details',
          onTabChange: onTabChangeMock
        }
      });

      const rollingStockButton = screen.getByText('Rolling Stock').closest('button');
      expect(rollingStockButton?.className).toContain('hover:border-border');
      expect(rollingStockButton?.className).toContain('hover:text-foreground');
    });

    it('should have transition classes for smooth animations', () => {
      const { container: _container } = render(ModelDetailsTabs, {
        props: {
          activeTab: 'details',
          onTabChange: onTabChangeMock
        }
      });

      const buttons = screen.getAllByRole('button');
      buttons.forEach((button) => {
        expect(button.className).toContain('transition-colors');
      });
    });

    it('should have bottom border styling', () => {
      const { container } = render(ModelDetailsTabs, {
        props: {
          activeTab: 'details',
          onTabChange: onTabChangeMock
        }
      });

      const borderContainer = container.querySelector('.border-b');
      expect(borderContainer).toBeInTheDocument();
      expect(borderContainer?.className).toContain('border-border');
    });
  });

  describe('Responsive Design', () => {
    it('should use responsive spacing', () => {
      const { container } = render(ModelDetailsTabs, {
        props: {
          activeTab: 'details',
          onTabChange: onTabChangeMock
        }
      });

      const nav = container.querySelector('nav');
      expect(nav?.className).toContain('space-x-8');
    });

    it('should prevent line breaking in tab labels', () => {
      render(ModelDetailsTabs, {
        props: {
          activeTab: 'details',
          onTabChange: onTabChangeMock
        }
      });

      const buttons = screen.getAllByRole('button');
      buttons.forEach((button) => {
        expect(button.className).toContain('whitespace-nowrap');
      });
    });
  });
});
