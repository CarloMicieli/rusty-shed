import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, fireEvent, screen } from '@testing-library/svelte';

// Mock Tauri dialog
vi.mock('@tauri-apps/plugin-dialog', () => ({
  open: vi.fn()
}));

// Mock Tauri commands
vi.mock('$lib/bindings', () => ({
  commands: {
    uploadModelImage: vi.fn(),
    deleteModelImage: vi.fn()
  }
}));

// Mock paraglide messages
vi.mock('$lib/paraglide/messages.js', () => ({
  upload_image: () => 'Upload Image',
  upload_image_filter_name: () => 'Images',
  uploading: () => 'Uploading...',
  upload_success: () => 'Image uploaded successfully',
  replace_image: () => 'Change Image',
  upload_error_model_not_found: () => 'Model not found',
  upload_error_unknown: () => 'An unknown error occurred',
  delete_image: () => 'Delete Image',
  deleting: () => 'Deleting...',
  confirm_delete_image_title: () => 'Delete Image?',
  confirm_delete_image_description: () => 'This action cannot be undone',
  image_deleted: () => 'Image deleted successfully'
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

    const { container: _container } = render(ImageUpload, {
      props: {
        modelId: mockModelId,
        hasExistingImage: false
      }
    });

    // Click upload button
    const uploadButton = screen.getByText('Upload Image');
    await fireEvent.click(uploadButton);

    // Verify dialog was opened with correct filters
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

    const { container: _container } = render(ImageUpload, {
      props: {
        modelId: mockModelId,
        hasExistingImage: false
      }
    });

    const uploadButton = screen.getByText('Upload Image');
    await fireEvent.click(uploadButton);

    // Verify multiple: false
    expect(open).toHaveBeenCalledWith(
      expect.objectContaining({
        multiple: false
      })
    );
  });

  it('should proceed with upload when file is selected', async () => {
    const selectedFilePath = '/path/to/image.jpg';
    vi.mocked(open).mockResolvedValue(selectedFilePath);
    vi.mocked(commands.uploadModelImage).mockResolvedValue({ status: 'ok', data: null });

    const { container: _container } = render(ImageUpload, {
      props: {
        modelId: mockModelId,
        hasExistingImage: false
      }
    });

    const uploadButton = screen.getByText('Upload Image');
    await fireEvent.click(uploadButton);

    // Should call upload command with selected file
    expect(commands.uploadModelImage).toHaveBeenCalledWith({
      modelId: mockModelId,
      filePath: selectedFilePath
    });
  });

  it('should not proceed when user cancels dialog', async () => {
    vi.mocked(open).mockResolvedValue(null); // User cancelled

    const { container: _container } = render(ImageUpload, {
      props: {
        modelId: mockModelId,
        hasExistingImage: false
      }
    });

    const uploadButton = screen.getByText('Upload Image');
    await fireEvent.click(uploadButton);

    // Should NOT call upload command
    expect(commands.uploadModelImage).not.toHaveBeenCalled();
  });
});

describe('ImageUpload - Upload Flow', () => {
  const mockModelId = 'trn:railway-model:maer:37858';

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should handle successful upload', async () => {
    vi.mocked(open).mockResolvedValue('/path/to/image.jpg');
    vi.mocked(commands.uploadModelImage).mockResolvedValue({ status: 'ok', data: null });

    const onUploadSuccess = vi.fn();

    const { container: _container } = render(ImageUpload, {
      props: {
        modelId: mockModelId,
        hasExistingImage: false,
        onUploadSuccess
      }
    });

    const uploadButton = screen.getByText('Upload Image');
    await fireEvent.click(uploadButton);

    // Should call success callback
    expect(onUploadSuccess).toHaveBeenCalled();
  });

  it('should handle upload error', async () => {
    vi.mocked(open).mockResolvedValue('/path/to/image.jpg');
    vi.mocked(commands.uploadModelImage).mockResolvedValue({
      status: 'error',
      error: { BusinessRule: 'Unsupported format' }
    });

    const { container } = render(ImageUpload, {
      props: {
        modelId: mockModelId,
        hasExistingImage: false
      }
    });

    const uploadButton = screen.getByText('Upload Image');
    await fireEvent.click(uploadButton);

    // Should show error message
    expect(container.textContent).toContain('Unsupported format');
  });

  it('should handle model not found error', async () => {
    vi.mocked(open).mockResolvedValue('/path/to/image.jpg');
    vi.mocked(commands.uploadModelImage).mockResolvedValue({
      status: 'error',
      error: { NotFound: 'Model not found' }
    });

    const { container } = render(ImageUpload, {
      props: {
        modelId: mockModelId,
        hasExistingImage: false
      }
    });

    const uploadButton = screen.getByText('Upload Image');
    await fireEvent.click(uploadButton);

    // Should show translated error message
    expect(container.textContent).toContain('Model not found');
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

    // Should show delete button (icon only, no text)
    // Look for the trash icon SVG
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

    // Should not show delete button
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

    // Should show "Upload Image" not "Change Image"
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

    // Should show "Change Image"
    expect(screen.getByText('Change Image')).toBeInTheDocument();
  });
});
