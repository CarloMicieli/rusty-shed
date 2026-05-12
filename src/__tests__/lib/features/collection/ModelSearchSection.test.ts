import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';

// Mock $lib/bindings commands
vi.mock('$lib/bindings', () => ({
  commands: {
    getManufacturers: vi.fn().mockResolvedValue({ status: 'ok', data: [] }),
    getRailwayCompanies: vi.fn().mockResolvedValue({ status: 'ok', data: [] }),
    getPrototypes: vi.fn().mockResolvedValue({ status: 'ok', data: [] })
  }
}));

// Mock Paraglide messages - override every function to return its key name
vi.mock('$lib/paraglide/messages', async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  return Object.fromEntries(
    Object.entries(actual).map(([k, v]) => [k, typeof v === 'function' ? () => k : v])
  );
});

import ModelSearchSection from './ModelSearchSectionHarness.svelte';
import type { AddModelFormState } from '$lib/features/collection/types/AddModelFormTypes';

describe('ModelSearchSection', () => {
  let mockForm: AddModelFormState;

  beforeEach(() => {
    mockForm = {
      manufacturerId: 'trn:manufacturer:test',
      productCode: '12345',
      description: 'Test Model',
      category: 'LOCOMOTIVES',
      scale: 'H0',
      powerMethod: 'AC',
      epoch: 'IV',
      rollingStocks: [],
      purchase: {
        sellerId: null,
        priceAmount: null,
        priceCurrency: 'EUR',
        purchaseCondition: null,
        modelCondition: null,
        boxCondition: null,
        notes: '',
        purchaseDate: new Date().toISOString().split('T')[0],
        purchaseType: 'STANDARD',
        depositAmount: null,
        depositCurrency: null,
        preorderTotalAmount: null,
        preorderTotalCurrency: null,
        expectedDate: null
      }
    };
  });

  it('should render without errors', () => {
    const { container } = render(ModelSearchSection, {
      props: {
        form: mockForm,
        manufacturers: [],
        railwayCompanies: [],
        sellers: [],
        showPurchaseSection: false,
        isRollingStockExpanded: false,
        validationErrors: {},
        onAddRollingStock: vi.fn(),
        onRemoveRollingStock: vi.fn(),
        onTogglePurchaseSection: vi.fn()
      }
    });

    // Check for collapsible section structure
    expect(
      container.querySelector('[role="region"]') || container.querySelector('div')
    ).toBeInTheDocument();
  });

  it('should display empty state when no rolling stocks exist', () => {
    const { container } = render(ModelSearchSection, {
      props: {
        form: mockForm,
        manufacturers: [],
        railwayCompanies: [],
        sellers: [],
        showPurchaseSection: false,
        isRollingStockExpanded: true,
        validationErrors: {},
        onAddRollingStock: vi.fn(),
        onRemoveRollingStock: vi.fn(),
        onTogglePurchaseSection: vi.fn()
      }
    });

    // Check for dashed border container (empty state indicator)
    const dashedContainer = container.querySelector('.border-dashed');
    expect(dashedContainer).toBeInTheDocument();
  });

  it('should display empty state with CTA button', () => {
    const { container } = render(ModelSearchSection, {
      props: {
        form: mockForm,
        manufacturers: [],
        railwayCompanies: [],
        sellers: [],
        showPurchaseSection: false,
        isRollingStockExpanded: true,
        validationErrors: {},
        onAddRollingStock: vi.fn(),
        onRemoveRollingStock: vi.fn(),
        onTogglePurchaseSection: vi.fn()
      }
    });

    // Check for button in dashed container
    const dashedContainer = container.querySelector('.border-dashed');
    const button = dashedContainer?.querySelector('button');
    expect(button).toBeInTheDocument();
  });

  it('should call onAddRollingStock when empty state button is clicked', async () => {
    const onAddRollingStock = vi.fn();
    const { container } = render(ModelSearchSection, {
      props: {
        form: mockForm,
        manufacturers: [],
        railwayCompanies: [],
        sellers: [],
        showPurchaseSection: false,
        isRollingStockExpanded: true,
        validationErrors: {},
        onAddRollingStock,
        onRemoveRollingStock: vi.fn(),
        onTogglePurchaseSection: vi.fn()
      }
    });

    const dashedContainer = container.querySelector('.border-dashed');
    const button = dashedContainer?.querySelector('button');
    if (button) {
      await userEvent.click(button);
      expect(onAddRollingStock).toHaveBeenCalledTimes(1);
    }
  });

  it('should hide empty state when rolling stocks exist', () => {
    const formWithStock = {
      ...mockForm,
      rollingStocks: [
        {
          uid: 'uuid-1',
          railwayCompanyId: 'trn:railway-company:db',
          seriesCode: '218',
          category: 'DIESEL_LOCOMOTIVE',
          roadNumber: '218 101',
          subcategory: null
        }
      ]
    };

    const { container } = render(ModelSearchSection, {
      props: {
        form: formWithStock,
        manufacturers: [],
        railwayCompanies: [],
        sellers: [],
        showPurchaseSection: false,
        isRollingStockExpanded: true,
        validationErrors: {},
        onAddRollingStock: vi.fn(),
        onRemoveRollingStock: vi.fn(),
        onTogglePurchaseSection: vi.fn()
      }
    });

    // Empty state should not be visible when items exist
    const dashedContainer = container.querySelector('.border-dashed');
    expect(dashedContainer).not.toBeInTheDocument();
  });

  it('should display rolling stock count in header', () => {
    const formWithStocks = {
      ...mockForm,
      rollingStocks: [
        {
          uid: 'uuid-1',
          railwayCompanyId: 'trn:railway-company:db',
          seriesCode: '218',
          category: 'DIESEL_LOCOMOTIVE',
          roadNumber: '',
          subcategory: null
        },
        {
          uid: 'uuid-2',
          railwayCompanyId: 'trn:railway-company:db',
          seriesCode: '103',
          category: 'ELECTRIC_LOCOMOTIVE',
          roadNumber: '',
          subcategory: null
        }
      ]
    };

    const { container } = render(ModelSearchSection, {
      props: {
        form: formWithStocks,
        manufacturers: [],
        railwayCompanies: [],
        sellers: [],
        showPurchaseSection: false,
        isRollingStockExpanded: false,
        validationErrors: {},
        onAddRollingStock: vi.fn(),
        onRemoveRollingStock: vi.fn(),
        onTogglePurchaseSection: vi.fn()
      }
    });

    // Should show count in header
    expect(container.textContent).toContain('2');
  });

  it('should have chevron icon in collapsible header', () => {
    const { container } = render(ModelSearchSection, {
      props: {
        form: mockForm,
        manufacturers: [],
        railwayCompanies: [],
        sellers: [],
        showPurchaseSection: false,
        isRollingStockExpanded: false,
        validationErrors: {},
        onAddRollingStock: vi.fn(),
        onRemoveRollingStock: vi.fn(),
        onTogglePurchaseSection: vi.fn()
      }
    });

    // Check for chevron SVG icon
    const chevron = container.querySelector('svg');
    expect(chevron).toBeInTheDocument();
  });

  it('should support adding rolling stocks', async () => {
    const onAddRollingStock = vi.fn();
    const { container } = render(ModelSearchSection, {
      props: {
        form: mockForm,
        manufacturers: [],
        railwayCompanies: [],
        sellers: [],
        showPurchaseSection: false,
        isRollingStockExpanded: true,
        validationErrors: {},
        onAddRollingStock,
        onRemoveRollingStock: vi.fn(),
        onTogglePurchaseSection: vi.fn()
      }
    });

    const dashedContainer = container.querySelector('.border-dashed');
    const button = dashedContainer?.querySelector('button');
    if (button) {
      await userEvent.click(button);
      expect(onAddRollingStock).toHaveBeenCalled();
    }
  });

  it('should render with proper spacing and layout', () => {
    const { container } = render(ModelSearchSection, {
      props: {
        form: mockForm,
        manufacturers: [],
        railwayCompanies: [],
        sellers: [],
        showPurchaseSection: false,
        isRollingStockExpanded: true,
        validationErrors: {},
        onAddRollingStock: vi.fn(),
        onRemoveRollingStock: vi.fn(),
        onTogglePurchaseSection: vi.fn()
      }
    });

    // Component should render without errors
    expect(container.children.length).toBeGreaterThan(0);
  });
});
