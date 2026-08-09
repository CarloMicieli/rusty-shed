import { describe, expect, it, vi } from 'vitest';
import { render, screen } from '@testing-library/svelte';

vi.mock('@tauri-apps/plugin-dialog', () => ({
  open: vi.fn()
}));

vi.mock('@tauri-apps/plugin-fs', () => ({
  readFile: vi.fn(() => Promise.resolve(new Uint8Array([1, 2, 3])))
}));

vi.mock('$lib/bindings', () => ({
  commands: {
    deleteModelImage: vi.fn()
  }
}));

vi.mock('../ImageCropDialog.svelte', () => ({
  default: vi.fn()
}));

vi.mock('$lib/paraglide/messages.js', () => ({
  upload_image: () => 'Upload Image',
  upload_image_filter_name: () => 'Images',
  replace_image: () => 'Change Image',
  upload_error_model_not_found: () => 'Model not found',
  upload_error_unknown: () => 'Unknown error',
  delete_image: () => 'Delete',
  deleting: () => 'Deleting',
  confirm_delete_image_title: () => 'Delete image?',
  confirm_delete_image_description: () => 'Cannot be undone',
  image_deleted: () => 'Deleted',
  common_cancel: () => 'Cancel',
  image_upload_camera_fallback_notice: ({ status }: { status: string }) =>
    `Camera fallback active (${status})`
}));

import ImageUpload from '$lib/components/model-details/ImageUpload.svelte';

describe('ImageUpload camera capability fallback', () => {
  it('switches to gallery fallback when camera capability is unavailable', async () => {
    const enumerateDevices = vi.fn().mockResolvedValue([]);

    Object.defineProperty(globalThis, 'navigator', {
      configurable: true,
      value: {
        mediaDevices: {
          enumerateDevices
        }
      }
    });

    render(ImageUpload, {
      props: {
        modelId: 'trn:railway-model:test:1',
        hasExistingImage: false
      }
    });

    expect(await screen.findByText('Camera fallback active (unsupported)')).toBeInTheDocument();
    expect(enumerateDevices).toHaveBeenCalledTimes(1);
  });
});
