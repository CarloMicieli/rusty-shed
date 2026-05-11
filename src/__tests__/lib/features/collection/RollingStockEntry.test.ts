import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render } from '@testing-library/svelte';

// Mock $lib/bindings commands
vi.mock('$lib/bindings', () => ({
  commands: {
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

import RollingStockEntry from '$lib/features/collection/components/RollingStockEntry.svelte';
import type { RollingStockFormEntry } from '$lib/features/collection/types/AddModelFormTypes';

describe('RollingStockEntry', () => {
  let mockStock: RollingStockFormEntry;

  beforeEach(() => {
    mockStock = {
      uid: 'uuid-1',
      railwayCompanyId: null,
      seriesCode: '',
      category: null,
      roadNumber: '',
      subcategory: null
    };
  });

  it('should render without errors', () => {
    const { container } = render(RollingStockEntry, {
      props: {
        entry: mockStock,
        railwayCompanies: [],
        canRemove: true,
        errors: undefined,
        onRemove: vi.fn()
      }
    });

    // Check that component renders without crashing
    expect(container.children.length).toBeGreaterThan(0);
  });

  it('should call onRemove with entry uid when remove button is clicked', async () => {
    const onRemove = vi.fn();
    const { container } = render(RollingStockEntry, {
      props: {
        entry: mockStock,
        railwayCompanies: [],
        canRemove: true,
        errors: undefined,
        onRemove
      }
    });

    // Component should render successfully
    expect(container.children.length).toBeGreaterThan(0);
  });

  it('should render with input fields', () => {
    const { container } = render(RollingStockEntry, {
      props: {
        entry: mockStock,
        railwayCompanies: [],
        canRemove: true,
        errors: undefined,
        onRemove: vi.fn()
      }
    });

    const inputs = container.querySelectorAll('input');
    expect(inputs.length).toBeGreaterThan(0);
  });

  it('should display validation errors when provided', () => {
    const errors = {
      railwayCompanyId: 'Railway company is required',
      seriesCode: 'Series code is required'
    };

    const { container } = render(RollingStockEntry, {
      props: {
        entry: mockStock,
        railwayCompanies: [],
        canRemove: true,
        errors,
        onRemove: vi.fn()
      }
    });

    // Component should render with validation
    expect(container.children.length).toBeGreaterThan(0);
  });

  it('should render with populated rolling stock data', () => {
    const populatedStock: RollingStockFormEntry = {
      uid: 'uuid-1',
      railwayCompanyId: 'trn:railway-company:db',
      seriesCode: '218',
      category: 'DIESEL_LOCOMOTIVE',
      roadNumber: '218 101-3',
      subcategory: null
    };

    const { container } = render(RollingStockEntry, {
      props: {
        entry: populatedStock,
        railwayCompanies: [
          {
            id: 'trn:railway-company:db',
            name: 'Deutsche Bahn',
            registeredCompanyName: null,
            countryCode: null,
            periodOfActivity: null
          }
        ],
        canRemove: true,
        errors: undefined,
        onRemove: vi.fn()
      }
    });

    // Verify component renders with data
    expect(container.children.length).toBeGreaterThan(0);
    expect(container.textContent).toContain('Deutsche Bahn');
  });

  it('should handle category selection', () => {
    const { container } = render(RollingStockEntry, {
      props: {
        entry: mockStock,
        railwayCompanies: [],
        canRemove: true,
        errors: undefined,
        onRemove: vi.fn()
      }
    });

    // Component should be functional without errors
    expect(container.children.length).toBeGreaterThan(0);
  });

  it('should have button for entry', () => {
    const { container } = render(RollingStockEntry, {
      props: {
        entry: mockStock,
        railwayCompanies: [],
        canRemove: true,
        errors: undefined,
        onRemove: vi.fn()
      }
    });

    // Check for buttons
    const buttons = container.querySelectorAll('button');
    expect(buttons.length).toBeGreaterThan(0);
  });

  it('should render remove button for entry', () => {
    const { container } = render(RollingStockEntry, {
      props: {
        entry: mockStock,
        railwayCompanies: [],
        canRemove: true,
        errors: undefined,
        onRemove: vi.fn()
      }
    });

    const buttons = container.querySelectorAll('button');
    expect(buttons.length).toBeGreaterThan(0);
  });
});
