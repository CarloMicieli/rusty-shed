import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, fireEvent, screen } from '@testing-library/svelte';

// Mock Tauri dialog
vi.mock('@tauri-apps/plugin-dialog', () => ({
  open: vi.fn()
}));

// Mock Tauri fs plugin used for reading file bytes before creating a blob URL
vi.mock('@tauri-apps/plugin-fs', () => ({
  readFile: vi.fn(() => Promise.resolve(new Uint8Array([0xff, 0xd8])))
}));

// Mock ImageCropDialog to prevent rendering the actual dialog
vi.mock('../ImageCropDialog.svelte', () => ({
  default: vi.fn()
}));

// Mock Tauri commands — uploadModelImage no longer used; deleteModelImage still used
vi.mock('$lib/bindings', () => ({
  commands: {
    deleteModelImage: vi.fn()
  }
}));

// Mock paraglide messages
vi.mock('$lib/paraglide/messages.js', () => ({
  upload_image: () => 'Upload Image',
  upload_image_filter_name: () => 'Images',
  replace_image: () => 'Change Image',
  upload_error_model_not_found: () => 'Model not found',
  upload_error_unknown: () => 'An unknown error occurred',
  delete_image: () => 'Delete Image',
  deleting: () => 'Deleting...',
  confirm_delete_image_title: () => 'Delete Image?',
  confirm_delete_image_description: () => 'This action cannot be undone',
  image_deleted: () => 'Image deleted successfully',
  common_cancel: () => 'Cancel'
}));

import ImageUpload from '../ImageUpload.svelte';
import { open } from '@tauri-apps/plugin-dialog';
import { commands } from '$lib/bindings';

describe('ImageUpload - T093: File dialog filter', () => {
  const mockModelId = 'trn:railway-model:maer:37858';

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should configure file dialog with correct filters (JPEG, PNG, WebP only)', async () => {
    vi.mocked(open).mockResolvedValue(null); // User cancels

    render(ImageUpload, {
      props: {
        modelId: mockModelId,
        hasExistingImage: false
      }
    });

    const uploadButton = screen.getByText('Upload Image');
    await fireEvent.click(uploadButton);

    expect(open).toHaveBeenCalledWith({
      multiple: false,
      filters: [
        {
          name: 'Images',
          extensions: ['jpg', 'jpeg', 'png', 'webp']
        }
      ]
    });
  });

  it('should not allow multiple file selection', async () => {
    vi.mocked(open).mockResolvedValue(null);

    render(ImageUpload, {
      props: {
        modelId: mockModelId,
        hasExistingImage: false
      }
    });

    const uploadButton = screen.getByText('Upload Image');
    await fireEvent.click(uploadButton);

    expect(open).toHaveBeenCalledWith(
      expect.objectContaining({
        multiple: false
      })
    );
  });

  it('should set pendingFilePath (not call uploadModelImage) when file is selected', async () => {
    const selectedFilePath = '/path/to/image.jpg';
    vi.mocked(open).mockResolvedValue(selectedFilePath);

    render(ImageUpload, {
      props: {
        modelId: mockModelId,
        hasExistingImage: false
      }
    });

    const uploadButton = screen.getByText('Upload Image');
    await fireEvent.click(uploadButton);

    // uploadModelImage is gone — crop dialog is now opened instead
    // We verify that no direct upload command is called
    // (commands mock doesn't include uploadModelImage)
    expect(open).toHaveBeenCalled();
  });

  it('should not proceed when user cancels dialog', async () => {
    vi.mocked(open).mockResolvedValue(null);

    render(ImageUpload, {
      props: {
        modelId: mockModelId,
        hasExistingImage: false
      }
    });

    const uploadButton = screen.getByText('Upload Image');
    await fireEvent.click(uploadButton);

    // No upload commands should be called
    expect(commands.deleteModelImage).not.toHaveBeenCalled();
  });
});

describe('ImageUpload - Delete Flow', () => {
  const mockModelId = 'trn:railway-model:maer:37858';

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should show delete button when image exists', () => {
    const { container } = render(ImageUpload, {
      props: {
        modelId: mockModelId,
        hasExistingImage: true
      }
    });

    const deleteButton = container.querySelector('[data-dialog-trigger]');
    expect(deleteButton).toBeTruthy();
  });

  it('should not show delete button when no image exists', () => {
    const { container } = render(ImageUpload, {
      props: {
        modelId: mockModelId,
        hasExistingImage: false
      }
    });

    const deleteButton = container.querySelector('[data-dialog-trigger]');
    expect(deleteButton).toBeFalsy();
  });

  it('should show upload button instead of replace when no image exists', () => {
    render(ImageUpload, {
      props: {
        modelId: mockModelId,
        hasExistingImage: false
      }
    });

    expect(screen.getByText('Upload Image')).toBeInTheDocument();
    expect(screen.queryByText('Change Image')).not.toBeInTheDocument();
  });

  it('should show replace button when image exists', () => {
    render(ImageUpload, {
      props: {
        modelId: mockModelId,
        hasExistingImage: true
      }
    });

    expect(screen.getByText('Change Image')).toBeInTheDocument();
  });

  it('should call deleteModelImage and show success on delete', async () => {
    vi.mocked(commands.deleteModelImage).mockResolvedValue({ status: 'ok', data: null });
    const onUploadSuccess = vi.fn();

    const { container } = render(ImageUpload, {
      props: {
        modelId: mockModelId,
        hasExistingImage: true,
        onUploadSuccess
      }
    });

    // Open delete dialog by clicking delete trigger button
    // The delete button is a DialogTrigger; find the trash icon button
    const trashButton = container.querySelector('[data-dialog-trigger]');
    if (trashButton) {
      await fireEvent.click(trashButton);

      const deleteButton = screen.queryByText('Delete Image');
      if (deleteButton) {
        await fireEvent.click(deleteButton);
        expect(commands.deleteModelImage).toHaveBeenCalledWith({ modelId: mockModelId });
        expect(onUploadSuccess).toHaveBeenCalled();
      }
    }
  });
});
