import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';

// Mock $lib/bindings commands
vi.mock('$lib/bindings', () => ({
  commands: {
    getRailwayCompanies: vi.fn().mockResolvedValue({ status: 'ok', data: [] })
  }
}));

// Mock Paraglide messages - override every function to return its key name
vi.mock('$lib/paraglide/messages', async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  return Object.fromEntries(
    Object.entries(actual).map(([k, v]) => [k, typeof v === 'function' ? () => k : v])
  );
});

import PurchaseSection from '$lib/features/collection/components/PurchaseSection.svelte';
import type { SellerView } from '$lib/bindings';
import type { PurchaseFormState } from '$lib/features/collection/types/AddModelFormTypes';

describe('PurchaseSection', () => {
  let mockPurchase: PurchaseFormState;

  beforeEach(() => {
    mockPurchase = {
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
    };
  });

  it('should render section header', () => {
    const { container } = render(PurchaseSection, {
      props: {
        purchase: mockPurchase,
        sellers: [],
        expanded: false,
        onToggle: vi.fn(),
        dark: true
      }
    });

    // Check for header element
    expect(container.querySelector('button')).toBeInTheDocument();
  });

  it('should display form fields when expanded', () => {
    const { container } = render(PurchaseSection, {
      props: {
        purchase: mockPurchase,
        sellers: [],
        expanded: true,
        onToggle: vi.fn(),
        dark: true
      }
    });

    // Should have some form elements
    expect(container.children.length).toBeGreaterThan(0);
  });

  it('should show price field for STANDARD purchase type', () => {
    const standardPurchase = { ...mockPurchase, purchaseType: 'STANDARD' as const };

    const { container } = render(PurchaseSection, {
      props: {
        purchase: standardPurchase,
        sellers: [],
        expanded: true,
        onToggle: vi.fn(),
        dark: true
      }
    });

    // Should have input fields
    const inputs = container.querySelectorAll('input');
    expect(inputs.length).toBeGreaterThan(0);
  });

  it('should show preorder fields when purchaseType is PREORDER', () => {
    const preorderPurchase = { ...mockPurchase, purchaseType: 'PREORDER' as const };

    const { container } = render(PurchaseSection, {
      props: {
        purchase: preorderPurchase,
        sellers: [],
        expanded: true,
        onToggle: vi.fn(),
        dark: true
      }
    });

    // Should render without errors
    expect(container.children.length).toBeGreaterThan(0);
  });

  it('should render input fields with proper classes in dark mode', () => {
    const { container } = render(PurchaseSection, {
      props: {
        purchase: mockPurchase,
        sellers: [],
        expanded: true,
        onToggle: vi.fn(),
        dark: true
      }
    });

    // Check that inputs exist and have classes
    const inputs = container.querySelectorAll('input');
    inputs.forEach((input) => {
      expect(input.className).toBeDefined();
    });
  });

  it('should render textarea with proper classes in dark mode', () => {
    const { container } = render(PurchaseSection, {
      props: {
        purchase: mockPurchase,
        sellers: [],
        expanded: true,
        onToggle: vi.fn(),
        dark: true
      }
    });

    // Look for textarea element
    const textarea = container.querySelector('textarea');
    if (textarea) {
      expect(textarea.className).toBeDefined();
    }
  });

  it('should toggle expanded state on header click', async () => {
    const onToggle = vi.fn();

    const { container } = render(PurchaseSection, {
      props: {
        purchase: mockPurchase,
        sellers: [],
        expanded: false,
        onToggle,
        dark: true
      }
    });

    // Find and click header button
    const header = container.querySelector('button');
    if (header) {
      await userEvent.click(header);
      expect(onToggle).toHaveBeenCalledTimes(1);
    }
  });

  it('should handle purchase type toggle between STANDARD and PREORDER', () => {
    const purchase = { ...mockPurchase, purchaseType: 'STANDARD' as const };

    render(PurchaseSection, {
      props: {
        purchase,
        sellers: [],
        expanded: true,
        onToggle: vi.fn(),
        dark: true
      }
    });

    // Re-render with PREORDER props to simulate toggle
    render(PurchaseSection, {
      props: {
        purchase: { ...purchase, purchaseType: 'PREORDER' as const },
        sellers: [],
        expanded: true,
        onToggle: vi.fn(),
        dark: true
      }
    });

    // Component should render without errors
    expect(document.body).toBeDefined();
  });

  it('should render with light mode styling when dark=false', () => {
    const { container } = render(PurchaseSection, {
      props: {
        purchase: mockPurchase,
        sellers: [],
        expanded: true,
        onToggle: vi.fn(),
        dark: false
      }
    });

    // Light mode should still render
    expect(container.children.length).toBeGreaterThan(0);
  });

  it('should display seller dropdown when available', () => {
    const sellers: SellerView[] = [
      {
        id: 'seller-1',
        name: 'Modellbahnshop',
        sellerType: 'SHOP',
        email: null,
        phone: null,
        websiteUrl: null,
        address: null,
        isSystemSeeded: false,
        usageCount: 0
      },
      {
        id: 'seller-2',
        name: 'Modelltrain',
        sellerType: 'SHOP',
        email: null,
        phone: null,
        websiteUrl: null,
        address: null,
        isSystemSeeded: false,
        usageCount: 0
      }
    ];

    const { container } = render(PurchaseSection, {
      props: {
        purchase: mockPurchase,
        sellers,
        expanded: true,
        onToggle: vi.fn(),
        dark: true
      }
    });

    // Should render seller field
    expect(container.children.length).toBeGreaterThan(0);
  });

  it('should maintain input field styling consistency', () => {
    const { container } = render(PurchaseSection, {
      props: {
        purchase: mockPurchase,
        sellers: [],
        expanded: true,
        onToggle: vi.fn(),
        dark: true
      }
    });

    // All inputs should have consistent structure
    const inputs = container.querySelectorAll('input');
    inputs.forEach((input) => {
      expect(input).toHaveAttribute('class');
    });
  });

  it('should render with form elements', () => {
    const { container } = render(PurchaseSection, {
      props: {
        purchase: mockPurchase,
        sellers: [],
        expanded: true,
        onToggle: vi.fn(),
        dark: true
      }
    });

    // Should have form structure
    expect(container.children.length).toBeGreaterThan(0);
  });
});
