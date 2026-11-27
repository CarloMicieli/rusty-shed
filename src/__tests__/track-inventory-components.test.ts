import { cleanup, fireEvent, render, screen } from '@testing-library/svelte';
import { beforeEach, describe, expect, it, vi } from 'vitest';

const mockTrackContext = vi.hoisted(() => ({
  setItemRequired: vi.fn().mockResolvedValue(undefined),
  removeItem: vi.fn().mockResolvedValue(undefined)
}));

vi.mock('$lib/features/track-inventory', () => ({
  getTrackInventoryContext: () => mockTrackContext
}));
vi.mock('$lib/paraglide/messages', async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  return Object.fromEntries(
    Object.entries(actual).map(([key, value]) => [
      key,
      typeof value === 'function' ? () => key : value
    ])
  );
});
vi.mock('$lib/paraglide/messages.js', async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  return Object.fromEntries(
    Object.entries(actual).map(([key, value]) => [
      key,
      typeof value === 'function' ? () => key : value
    ])
  );
});

import CreateInventoryDialog from '$lib/features/track-inventory/components/CreateInventoryDialog.svelte';
import TrackItemCard from '$lib/features/track-inventory/components/TrackItemCard.svelte';
import type { TrackInventoryItemView } from '$lib/bindings';

describe('CreateInventoryDialog', () => {
  beforeEach(() => cleanup());

  it('validates an empty name', async () => {
    render(CreateInventoryDialog, { props: { open: true, onCreate: vi.fn() } });

    await fireEvent.submit(screen.getByRole('dialog').querySelector('form') as HTMLFormElement);
    expect(screen.getByText('track_inventory_create_validation_name')).toBeInTheDocument();
  });

  it('trims name and description and creates successfully', async () => {
    const onCreate = vi.fn().mockResolvedValue(undefined);
    render(CreateInventoryDialog, { props: { open: true, onCreate } });

    await fireEvent.input(screen.getByLabelText('track_inventory_create_field_name'), {
      target: { value: '  Main Layout  ' }
    });
    await fireEvent.input(screen.getByLabelText('track_inventory_create_field_description'), {
      target: { value: '  Oval plus sidings  ' }
    });
    await fireEvent.submit(screen.getByRole('dialog').querySelector('form') as HTMLFormElement);

    expect(onCreate).toHaveBeenCalledWith('Main Layout', 'Oval plus sidings');
  });

  it('closes and resets on cancel', async () => {
    const onClose = vi.fn();
    render(CreateInventoryDialog, { props: { open: true, onClose } });

    await fireEvent.click(screen.getByRole('button', { name: 'dialog_close_button' }));
    expect(onClose).toHaveBeenCalledOnce();
  });

  it('surfaces create callback failures', async () => {
    const onCreate = vi.fn().mockRejectedValue(new Error('Database write failed'));
    render(CreateInventoryDialog, { props: { open: true, onCreate } });

    await fireEvent.input(screen.getByLabelText('track_inventory_create_field_name'), {
      target: { value: 'Main Layout' }
    });
    await fireEvent.submit(screen.getByRole('dialog').querySelector('form') as HTMLFormElement);

    expect(await screen.findByText('Database write failed')).toBeInTheDocument();
  });
});

describe('TrackItemCard', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
  });

  const item = {
    track_id: 'track-1',
    quantity: 3,
    required: 5,
    track_product: {
      track_id: 'track-1',
      manufacturer_name: 'Fleischmann',
      description: 'Flextrack',
      product_code: '9135',
      track_type: 'CURVE'
    }
  };

  it('shows shortage count and opens required editing', async () => {
    const { container } = render(TrackItemCard, {
      props: { item: item as unknown as TrackInventoryItemView, inventoryId: 'inventory-1' }
    });

    expect(screen.getByText('track_inventory_item_shortage')).toBeInTheDocument();
    await fireEvent.click(container.querySelector('button.mt-1') as HTMLButtonElement);
    expect(screen.getByRole('spinbutton')).toBeInTheDocument();
  });

  it('rejects negative required quantities', async () => {
    const { container } = render(TrackItemCard, {
      props: { item: item as unknown as TrackInventoryItemView, inventoryId: 'inventory-1' }
    });

    await fireEvent.click(container.querySelector('button.mt-1') as HTMLButtonElement);
    await fireEvent.input(screen.getByRole('spinbutton'), { target: { value: '-1' } });
    await fireEvent.click(screen.getByRole('button', { name: 'save_button' }));

    expect(screen.getByText('track_inventory_invalid_required')).toBeInTheDocument();
  });

  it('saves a valid required quantity', async () => {
    const { container } = render(TrackItemCard, {
      props: {
        item: { ...item } as unknown as TrackInventoryItemView,
        inventoryId: 'inventory-1'
      }
    });

    await fireEvent.click(container.querySelector('button.mt-1') as HTMLButtonElement);
    await fireEvent.input(screen.getByRole('spinbutton'), { target: { value: '8' } });
    await fireEvent.click(screen.getByRole('button', { name: 'save_button' }));

    expect(mockTrackContext.setItemRequired).toHaveBeenCalledWith('inventory-1', 'track-1', 8);
  });

  it('cancels required editing', async () => {
    const { container } = render(TrackItemCard, {
      props: {
        item: { ...item } as unknown as TrackInventoryItemView,
        inventoryId: 'inventory-1'
      }
    });

    await fireEvent.click(container.querySelector('button.mt-1') as HTMLButtonElement);
    await fireEvent.input(screen.getByRole('spinbutton'), { target: { value: '9' } });
    await fireEvent.click(screen.getByRole('button', { name: 'app_cancel' }));

    expect(screen.queryByRole('spinbutton')).not.toBeInTheDocument();
  });
});
