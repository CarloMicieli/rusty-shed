import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, fireEvent } from '@testing-library/svelte';

// Mock Tauri commands (use function for factory to avoid hoisting issues)
vi.mock('$lib/bindings', () => ({
  commands: {
    uploadModelImageBytes: vi.fn()
  }
}));

// Mock paraglide messages
vi.mock('$lib/paraglide/messages.js', () => ({
  drag_and_drop_hint: () => 'Drag and drop your image here',
  drop_image_here: () => 'Drop image here',
  uploading: () => 'Uploading...',
  upload_success: () => 'Image uploaded successfully',
  upload_error_model_not_found: () => 'Model not found',
  upload_error_unknown: () => 'An unknown error occurred',
  upload_error_multiple_files: () => 'Only one image at a time'
}));

import ImageDropZone from '../ImageDropZone.svelte';
import { commands } from '$lib/bindings';

describe('ImageDropZone - T092: Multiple file drop rejection', () => {
  const mockModelId = 'trn:railway-model:maer:37858';

  beforeEach(() => {
    vi.clearAllMocks();
  });

  // T092: Test multiple file drop rejection
  it('should reject multiple files and show error message', async () => {
    const { container } = render(ImageDropZone, {
      props: {
        modelId: mockModelId
      }
    });

    const dropZone = container.querySelector('[role="button"]') as HTMLElement;
    expect(dropZone).toBeTruthy();

    // Create DataTransfer with multiple files
    const file1 = new File(['image1'], 'test1.jpg', { type: 'image/jpeg' });
    const file2 = new File(['image2'], 'test2.jpg', { type: 'image/jpeg' });

    const dataTransfer = new DataTransfer();
    dataTransfer.items.add(file1);
    dataTransfer.items.add(file2);

    // Trigger drop event
    await fireEvent.drop(dropZone, { dataTransfer });

    // Should NOT call upload command
    expect(commands.uploadModelImageBytes).not.toHaveBeenCalled();

    // Should show error message
    expect(container.textContent).toContain('Only one image at a time');
  });

  it('should accept single file and proceed with upload', async () => {
    vi.mocked(commands.uploadModelImageBytes).mockResolvedValue({ status: 'ok', data: null });

    const { container } = render(ImageDropZone, {
      props: {
        modelId: mockModelId
      }
    });

    const dropZone = container.querySelector('[role="button"]') as HTMLElement;

    // Create DataTransfer with single file
    const file = new File(['image'], 'test.jpg', { type: 'image/jpeg' });
    const dataTransfer = new DataTransfer();
    dataTransfer.items.add(file);

    // Trigger drop event
    await fireEvent.drop(dropZone, { dataTransfer });

    // Should call upload command
    expect(commands.uploadModelImageBytes).toHaveBeenCalledWith({
      modelId: mockModelId,
      fileName: 'test.jpg',
      fileData: expect.any(Array)
    });
  });
});

describe('ImageDropZone - T097: MIME type hint check', () => {
  const mockModelId = 'trn:railway-model:maer:37858';

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should reject TIFF files based on MIME type', async () => {
    const { container } = render(ImageDropZone, {
      props: {
        modelId: mockModelId
      }
    });

    const dropZone = container.querySelector('[role="button"]') as HTMLElement;

    // Create TIFF file with MIME type
    const file = new File(['tiff-data'], 'image.tiff', { type: 'image/tiff' });
    const dataTransfer = new DataTransfer();
    dataTransfer.items.add(file);

    await fireEvent.drop(dropZone, { dataTransfer });

    // Should NOT call upload command
    expect(commands.uploadModelImageBytes).not.toHaveBeenCalled();

    // Should show unsupported format error
    expect(container.textContent).toContain('Unsupported format: image/tiff');
  });

  it('should reject PDF files based on MIME type', async () => {
    const { container } = render(ImageDropZone, {
      props: {
        modelId: mockModelId
      }
    });

    const dropZone = container.querySelector('[role="button"]') as HTMLElement;

    const file = new File(['pdf-data'], 'document.pdf', { type: 'application/pdf' });
    const dataTransfer = new DataTransfer();
    dataTransfer.items.add(file);

    await fireEvent.drop(dropZone, { dataTransfer });

    expect(commands.uploadModelImageBytes).not.toHaveBeenCalled();
    expect(container.textContent).toContain('Unsupported format: application/pdf');
  });

  it('should reject BMP files based on MIME type', async () => {
    const { container } = render(ImageDropZone, {
      props: {
        modelId: mockModelId
      }
    });

    const dropZone = container.querySelector('[role="button"]') as HTMLElement;

    const file = new File(['bmp-data'], 'image.bmp', { type: 'image/bmp' });
    const dataTransfer = new DataTransfer();
    dataTransfer.items.add(file);

    await fireEvent.drop(dropZone, { dataTransfer });

    expect(commands.uploadModelImageBytes).not.toHaveBeenCalled();
    expect(container.textContent).toContain('Unsupported format: image/bmp');
  });

  it('should accept JPEG files', async () => {
    vi.mocked(commands.uploadModelImageBytes).mockResolvedValue({ status: 'ok', data: null });

    const { container } = render(ImageDropZone, {
      props: {
        modelId: mockModelId
      }
    });

    const dropZone = container.querySelector('[role="button"]') as HTMLElement;

    const file = new File(['jpeg-data'], 'photo.jpg', { type: 'image/jpeg' });
    const dataTransfer = new DataTransfer();
    dataTransfer.items.add(file);

    await fireEvent.drop(dropZone, { dataTransfer });

    expect(commands.uploadModelImageBytes).toHaveBeenCalled();
  });

  it('should accept PNG files', async () => {
    vi.mocked(commands.uploadModelImageBytes).mockResolvedValue({ status: 'ok', data: null });

    const { container } = render(ImageDropZone, {
      props: {
        modelId: mockModelId
      }
    });

    const dropZone = container.querySelector('[role="button"]') as HTMLElement;

    const file = new File(['png-data'], 'photo.png', { type: 'image/png' });
    const dataTransfer = new DataTransfer();
    dataTransfer.items.add(file);

    await fireEvent.drop(dropZone, { dataTransfer });

    expect(commands.uploadModelImageBytes).toHaveBeenCalled();
  });

  it('should accept WebP files', async () => {
    vi.mocked(commands.uploadModelImageBytes).mockResolvedValue({ status: 'ok', data: null });

    const { container } = render(ImageDropZone, {
      props: {
        modelId: mockModelId
      }
    });

    const dropZone = container.querySelector('[role="button"]') as HTMLElement;

    const file = new File(['webp-data'], 'photo.webp', { type: 'image/webp' });
    const dataTransfer = new DataTransfer();
    dataTransfer.items.add(file);

    await fireEvent.drop(dropZone, { dataTransfer });

    expect(commands.uploadModelImageBytes).toHaveBeenCalled();
  });

  it('should proceed when MIME type is empty (backend will validate)', async () => {
    vi.mocked(commands.uploadModelImageBytes).mockResolvedValue({ status: 'ok', data: null });

    const { container } = render(ImageDropZone, {
      props: {
        modelId: mockModelId
      }
    });

    const dropZone = container.querySelector('[role="button"]') as HTMLElement;

    // File with no MIME type (e.g., from some file systems)
    const file = new File(['unknown-data'], 'photo.jpg', { type: '' });
    const dataTransfer = new DataTransfer();
    dataTransfer.items.add(file);

    await fireEvent.drop(dropZone, { dataTransfer });

    // Should proceed to backend validation
    expect(commands.uploadModelImageBytes).toHaveBeenCalled();
  });
});

describe('ImageDropZone - Drag & Drop States', () => {
  const mockModelId = 'trn:railway-model:maer:37858';

  it('should show drag state when dragging over', async () => {
    const { container } = render(ImageDropZone, {
      props: {
        modelId: mockModelId
      }
    });

    const dropZone = container.querySelector('[role="button"]') as HTMLElement;

    await fireEvent.dragOver(dropZone);

    // Should show "Drop image here" message
    expect(container.textContent).toContain('Drop image here');
  });

  it('should reset state when drag leaves', async () => {
    const { container } = render(ImageDropZone, {
      props: {
        modelId: mockModelId
      }
    });

    const dropZone = container.querySelector('[role="button"]') as HTMLElement;

    await fireEvent.dragOver(dropZone);
    expect(container.textContent).toContain('Drop image here');

    // Simulate leaving the drop zone
    await fireEvent.dragLeave(dropZone, { relatedTarget: null });

    // Should return to default state
    expect(container.textContent).toContain('Drag and drop your image here');
  });
});
