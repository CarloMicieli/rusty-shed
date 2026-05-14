import { describe, expect, it, vi } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import FormFieldsRenderer from '$lib/components/drawer/FormFieldsRenderer.svelte';
import type { Manufacturer } from '$lib/bindings';

vi.mock('$lib/paraglide/messages.js', () => ({
  wishlist_modal_product_code: () => 'Product code',
  acquisition_item_product_code_placeholder: () => 'Product code placeholder',
  wishlist_modal_description: () => 'Description',
  acquisition_item_description_placeholder: () => 'Description placeholder',
  wishlist_modal_category: () => 'Category',
  acquisition_item_category_placeholder: () => 'Category placeholder'
}));

vi.mock('$lib/components/drawer/FormInput.svelte', async () => {
  const module = await import('../../../stubs/DrawerFieldStub.svelte');
  return { default: module.default };
});

vi.mock('$lib/components/drawer/FormSelect.svelte', async () => {
  const module = await import('../../../stubs/DrawerFieldStub.svelte');
  return { default: module.default };
});

vi.mock('$lib/components/drawer/ManufacturerSelect.svelte', async () => {
  const module = await import('../../../stubs/ManufacturerSelectStub.svelte');
  return { default: module.default };
});

describe('FormFieldsRenderer', () => {
  const manufacturers: Manufacturer[] = [
    {
      id: 'trn:manufacturer:acme',
      name: 'ACME',
      registeredCompanyName: null,
      countryCode: null,
      status: 'ACTIVE',
      websiteUrl: null
    }
  ];

  it('renders all fields by default and forwards required markers', () => {
    render(FormFieldsRenderer, {
      props: {
        manufacturerId: null,
        productCode: 'E636',
        description: 'Electric locomotive',
        category: null,
        manufacturers,
        showRequired: true
      }
    });

    expect(screen.getByText('Manufacturer *')).toBeInTheDocument();
    expect(screen.getByText('Product code *')).toBeInTheDocument();
    expect(screen.getByText('Description *')).toBeInTheDocument();
    expect(screen.getByText('Category *')).toBeInTheDocument();
  });

  it('skips configured fields when fieldsConfig disables them', () => {
    render(FormFieldsRenderer, {
      props: {
        manufacturerId: null,
        productCode: 'E636',
        description: 'Electric locomotive',
        category: null,
        manufacturers,
        showRequired: false,
        fieldsConfig: {
          manufacturer: false,
          productCode: true,
          description: false,
          category: true
        }
      }
    });

    expect(screen.queryByText('Manufacturer')).not.toBeInTheDocument();
    expect(screen.getByText('Product code')).toBeInTheDocument();
    expect(screen.queryByText('Description')).not.toBeInTheDocument();
    expect(screen.getByText('Category')).toBeInTheDocument();
  });
});
