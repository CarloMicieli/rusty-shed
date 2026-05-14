import { describe, expect, it, vi } from 'vitest';
import { fireEvent, render } from '@testing-library/svelte';
import RailwayModelImagePanel from '../RailwayModelImagePanel.svelte';
import type { RailwayModel } from '$lib/types/railway-model';

vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn(() => Promise.resolve(() => {}))
}));

vi.mock('@tauri-apps/plugin-fs', () => ({
  readFile: vi.fn(() => Promise.resolve(new Uint8Array([1, 2, 3])))
}));

vi.mock('$lib/toaster', () => ({
  toaster: {
    error: vi.fn(),
    dismiss: vi.fn(),
    success: vi.fn(),
    loading: vi.fn()
  }
}));

vi.mock('$lib/paraglide/messages', () => ({
  railway_model_image_alt: () => 'Model image',
  drag_drop_image_here: () => 'Drop image here',
  railway_model_no_image: () => 'No image',
  replace_image: () => 'Replace image',
  upload_image: () => 'Upload image',
  upload_error_unsupported_format: () => 'Unsupported format',
  upload_error_unknown: () => 'Unknown upload error'
}));

function makeModel(overrides: Partial<RailwayModel> = {}): RailwayModel {
  return {
    id: 'rm-1',
    manufacturer: 'Roco',
    product_code: '1234',
    scale: 'H0',
    era: null,
    power_method: null,
    category: null,
    delivery_date: null,
    description: 'Demo model',
    descriptionLang: 'en',
    details: null,
    detailsLang: null,
    image_path: null,
    status: 'InCollection',
    rolling_stock: [],
    ...overrides
  };
}

describe('RailwayModelImagePanel', () => {
  it('calls onError for non-image file drops', async () => {
    const onError = vi.fn();

    const { container } = render(RailwayModelImagePanel, {
      props: {
        model: makeModel(),
        editable: true,
        onError
      }
    });

    const panel = container.querySelector('[role="img"]') as HTMLElement;
    const file = new File(['pdf'], 'spec.pdf', { type: 'application/pdf' });

    await fireEvent.drop(panel, {
      dataTransfer: { files: [file] }
    });

    expect(onError).toHaveBeenCalledWith('Invalid image format');
  });

  it('calls onError when dropped image exceeds 50MB', async () => {
    const onError = vi.fn();

    const { container } = render(RailwayModelImagePanel, {
      props: {
        model: makeModel(),
        editable: true,
        onError
      }
    });

    const panel = container.querySelector('[role="img"]') as HTMLElement;
    const file = new File(['big'], 'huge.png', { type: 'image/png' });
    Object.defineProperty(file, 'size', { value: 51 * 1024 * 1024 });

    await fireEvent.drop(panel, {
      dataTransfer: { files: [file] }
    });

    expect(onError).toHaveBeenCalledWith('File too large. Maximum size is 50 MB.');
  });

  it('calls onImageDropped for valid image files', async () => {
    const onImageDropped = vi.fn();

    const { container } = render(RailwayModelImagePanel, {
      props: {
        model: makeModel(),
        editable: true,
        onImageDropped
      }
    });

    const panel = container.querySelector('[role="img"]') as HTMLElement;
    const file = new File(['png'], 'ok.png', { type: 'image/png' });

    await fireEvent.drop(panel, {
      dataTransfer: { files: [file] }
    });

    expect(onImageDropped).toHaveBeenCalledOnce();
    expect(onImageDropped.mock.calls[0][0]).toBe(file);
  });
});
