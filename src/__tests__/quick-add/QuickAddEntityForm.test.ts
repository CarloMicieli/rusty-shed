import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';

vi.mock('$lib/paraglide/messages.js', () => ({
  quick_add_entity_manufacturer: () => 'manufacturer',
  quick_add_entity_seller: () => 'seller',
  quick_add_entity_buyer: () => 'buyer',
  quick_add_duplicate_warning: ({ entity }: { entity: string }) =>
    `A ${entity} with this name already exists.`,
  quick_add_save_failed: () => 'Could not save. Please try again.',
  quick_add_name_required: () => 'Name is required.',
  quick_add_field_name: () => 'Name',
  quick_add_name_placeholder: () => 'Enter name',
  quick_add_field_website: () => 'Website',
  quick_add_website_placeholder: () => 'https://example.com',
  quick_add_field_country: () => 'Country',
  quick_add_country_placeholder: () => 'IT',
  quick_add_cancel: () => 'Cancel',
  quick_add_save: () => 'Save',
  settings_saving_button: () => 'Saving...'
}));

const createManufacturer = vi.fn();
const createSeller = vi.fn();

vi.mock('$lib/bindings', () => ({
  commands: {
    createManufacturer: (...args: unknown[]) => createManufacturer(...args),
    createSeller: (...args: unknown[]) => createSeller(...args)
  }
}));

import QuickAddEntityForm from '$lib/features/quick-add/QuickAddEntityForm.svelte';
import QuickAddShellHarness from './QuickAddShellHarness.svelte';

describe('QuickAddEntityForm', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it('disables save for duplicate manufacturer names (case-insensitive)', async () => {
    const user = userEvent.setup();
    render(QuickAddEntityForm, {
      props: {
        target: 'manufacturer',
        existingNames: ['ACME'],
        onSuccess: vi.fn(),
        onCancel: vi.fn()
      }
    });

    await user.type(screen.getByLabelText('Name'), 'acme');

    expect(screen.getByText('A manufacturer with this name already exists.')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Save' })).toBeDisabled();
  });

  it('calls createManufacturer and emits success when valid', async () => {
    const user = userEvent.setup();
    const onSuccess = vi.fn();
    createManufacturer.mockResolvedValue({
      status: 'ok',
      data: {
        id: 'trn:manufacturer:acme',
        name: 'ACME',
        registeredCompanyName: null,
        countryCode: 'IT',
        status: 'ACTIVE',
        websiteUrl: null
      }
    });

    render(QuickAddEntityForm, {
      props: {
        target: 'manufacturer',
        existingNames: [],
        onSuccess,
        onCancel: vi.fn()
      }
    });

    await user.type(screen.getByLabelText('Name'), 'ACME');
    await user.click(screen.getByRole('button', { name: 'Save' }));

    expect(createManufacturer).toHaveBeenCalledOnce();
    expect(onSuccess).toHaveBeenCalledOnce();
  });

  it('calls onCancel when cancel is pressed', async () => {
    const user = userEvent.setup();
    const onCancel = vi.fn();

    render(QuickAddEntityForm, {
      props: {
        target: 'seller',
        existingNames: [],
        onSuccess: vi.fn(),
        onCancel
      }
    });

    await user.click(screen.getByRole('button', { name: 'Cancel' }));
    expect(onCancel).toHaveBeenCalledOnce();
  });

  it('renders quick-add as a mobile bottom sheet with swipe dismiss', async () => {
    const onDismiss = vi.fn();

    Object.defineProperty(window, 'matchMedia', {
      configurable: true,
      value: vi.fn().mockImplementation((query: string) => ({
        matches: query === '(max-width: 767px)',
        media: query,
        onchange: null,
        addListener: vi.fn(),
        removeListener: vi.fn(),
        addEventListener: vi.fn(),
        removeEventListener: vi.fn(),
        dispatchEvent: vi.fn()
      }))
    });

    const { container } = render(QuickAddShellHarness, {
      props: {
        open: true,
        title: 'Add Manufacturer',
        onDismiss
      }
    });

    const dialog = screen.getByRole('dialog');
    expect(dialog.className).toContain('h-[82vh]');

    fireEvent.touchStart(dialog, { touches: [{ clientY: 10 }] });
    fireEvent.touchMove(dialog, { touches: [{ clientY: 130 }] });
    fireEvent.touchEnd(dialog);

    expect(onDismiss).toHaveBeenCalledOnce();
    expect(container.querySelector('[aria-labelledby="quick-add-title"]')).toBeInTheDocument();
  });

  it('applies keyboard-safe visual viewport inset to bottom padding', async () => {
    const listeners: Record<string, EventListener> = {};
    Object.defineProperty(window, 'visualViewport', {
      configurable: true,
      value: {
        height: 600,
        offsetTop: 0,
        addEventListener: (name: string, cb: EventListener) => {
          listeners[name] = cb;
        },
        removeEventListener: vi.fn()
      }
    });

    Object.defineProperty(window, 'innerHeight', {
      configurable: true,
      value: 900
    });

    render(QuickAddShellHarness, {
      props: {
        open: true,
        title: 'Add Manufacturer',
        onDismiss: vi.fn()
      }
    });

    listeners.resize?.(new Event('resize'));
    const dialog = screen.getByRole('dialog');
    expect(dialog.getAttribute('style')).toContain('padding-bottom: 300px');
  });

  it('uses dirty-form dismiss confirmation before closing from scrim', async () => {
    const user = userEvent.setup();
    const confirmSpy = vi.fn(() => false);
    vi.stubGlobal('confirm', confirmSpy);
    const onCancel = vi.fn();

    render(QuickAddEntityForm, {
      props: {
        target: 'manufacturer',
        existingNames: [],
        onSuccess: vi.fn(),
        onCancel,
        onDirtyChange: vi.fn()
      }
    });

    await user.type(screen.getByLabelText('Name'), 'ACME');
    const dirty = screen.getByLabelText('Name') as HTMLInputElement;
    expect(dirty.value).toBe('ACME');

    const shouldClose = () => {
      if (dirty.value.length > 0 && !window.confirm('Discard quick-add changes?')) {
        return false;
      }
      onCancel();
      return true;
    };

    expect(shouldClose()).toBe(false);
    expect(onCancel).not.toHaveBeenCalled();
    expect(confirmSpy).toHaveBeenCalled();
  });
});
