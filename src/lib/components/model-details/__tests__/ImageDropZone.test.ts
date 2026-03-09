import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, fireEvent } from '@testing-library/svelte';

// Mock Tauri event listener (tauri://drag-drop) — must return a cleanup fn
vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn(() => Promise.resolve(() => {}))
}));

// Mock Tauri fs plugin used for native drag-drop path reads
vi.mock('@tauri-apps/plugin-fs', () => ({
  readFile: vi.fn(() => Promise.resolve(new Uint8Array([0xff, 0xd8])))
}));

// Mock Tauri commands — no longer called directly from ImageDropZone
vi.mock('$lib/bindings', () => ({
  commands: {
    uploadModelImageBytes: vi.fn()
  }
}));

// Mock ImageCropDialog to prevent rendering the actual dialog
vi.mock('../ImageCropDialog.svelte', () => ({
  default: vi.fn()
}));

// Mock toaster
vi.mock('$lib/toaster', () => ({
  toaster: {
    error: vi.fn(),
    success: vi.fn()
  }
}));

// Mock paraglide messages
vi.mock('$lib/paraglide/messages.js', () => ({
  drag_and_drop_hint: () => 'Drag and drop your image here',
  drop_here_to_update_photo: () => 'Drop here to update photo',
  upload_error_multiple_files: () => 'Only one image at a time',
  upload_formats_hint: () => 'JPEG, PNG, or WebP (max 50MB)',
  upload_error_unsupported_format: ({ format }: { format: string }) =>
    `Unsupported format: .${format}. Supported formats: JPEG, PNG, WebP`,
  image_drop_zone_drag_message: () => 'Drop here to update photo'
}));

import ImageDropZone from '../ImageDropZone.svelte';
import { commands } from '$lib/bindings';
import { toaster } from '$lib/toaster';

const mockModelId = 'trn:railway-model:maer:37858';

describe('ImageDropZone - Valid file drop opens crop dialog', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('valid single JPEG drop does NOT call uploadModelImageBytes directly', async () => {
    const { container } = render(ImageDropZone, {
      props: { modelId: mockModelId }
    });

    const dropZone = container.querySelector('[role="button"]') as HTMLElement;
    const file = new File(['image'], 'test.jpg', { type: 'image/jpeg' });
    const dataTransfer = new DataTransfer();
    dataTransfer.items.add(file);

    await fireEvent.drop(dropZone, { dataTransfer });

    // Drop zone no longer calls upload directly — it sets pendingFile for crop dialog
    expect(commands.uploadModelImageBytes).not.toHaveBeenCalled();
  });
});

describe('ImageDropZone - Multiple file drop rejection', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should reject multiple files with a toast and not open crop dialog', async () => {
    const { container } = render(ImageDropZone, {
      props: { modelId: mockModelId }
    });

    const dropZone = container.querySelector('[role="button"]') as HTMLElement;
    const file1 = new File(['image1'], 'test1.jpg', { type: 'image/jpeg' });
    const file2 = new File(['image2'], 'test2.jpg', { type: 'image/jpeg' });
    const dataTransfer = new DataTransfer();
    dataTransfer.items.add(file1);
    dataTransfer.items.add(file2);

    await fireEvent.drop(dropZone, { dataTransfer });

    expect(toaster.error).toHaveBeenCalledWith('Only one image at a time');
    expect(commands.uploadModelImageBytes).not.toHaveBeenCalled();
  });
});

describe('ImageDropZone - MIME type rejection', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('.pdf drop shows rejection toast and does NOT open crop dialog', async () => {
    const { container } = render(ImageDropZone, {
      props: { modelId: mockModelId }
    });

    const dropZone = container.querySelector('[role="button"]') as HTMLElement;
    const file = new File(['pdf-data'], 'document.pdf', { type: 'application/pdf' });
    const dataTransfer = new DataTransfer();
    dataTransfer.items.add(file);

    await fireEvent.drop(dropZone, { dataTransfer });

    expect(toaster.error).toHaveBeenCalled();
    expect(commands.uploadModelImageBytes).not.toHaveBeenCalled();
  });

  it('.tiff drop shows rejection toast', async () => {
    const { container } = render(ImageDropZone, {
      props: { modelId: mockModelId }
    });

    const dropZone = container.querySelector('[role="button"]') as HTMLElement;
    const file = new File(['tiff-data'], 'image.tiff', { type: 'image/tiff' });
    const dataTransfer = new DataTransfer();
    dataTransfer.items.add(file);

    await fireEvent.drop(dropZone, { dataTransfer });

    expect(toaster.error).toHaveBeenCalled();
    expect(commands.uploadModelImageBytes).not.toHaveBeenCalled();
  });

  it('multi-file drop shows rejection toast', async () => {
    const { container } = render(ImageDropZone, {
      props: { modelId: mockModelId }
    });

    const dropZone = container.querySelector('[role="button"]') as HTMLElement;
    const file1 = new File(['a'], 'a.jpg', { type: 'image/jpeg' });
    const file2 = new File(['b'], 'b.png', { type: 'image/png' });
    const dataTransfer = new DataTransfer();
    dataTransfer.items.add(file1);
    dataTransfer.items.add(file2);

    await fireEvent.drop(dropZone, { dataTransfer });

    expect(toaster.error).toHaveBeenCalled();
    expect(commands.uploadModelImageBytes).not.toHaveBeenCalled();
  });

  it('should accept JPEG files without showing a toast', async () => {
    const { container } = render(ImageDropZone, {
      props: { modelId: mockModelId }
    });

    const dropZone = container.querySelector('[role="button"]') as HTMLElement;
    const file = new File(['jpeg-data'], 'photo.jpg', { type: 'image/jpeg' });
    const dataTransfer = new DataTransfer();
    dataTransfer.items.add(file);

    await fireEvent.drop(dropZone, { dataTransfer });

    expect(toaster.error).not.toHaveBeenCalled();
  });

  it('should accept PNG files without showing a toast', async () => {
    const { container } = render(ImageDropZone, {
      props: { modelId: mockModelId }
    });

    const dropZone = container.querySelector('[role="button"]') as HTMLElement;
    const file = new File(['png-data'], 'photo.png', { type: 'image/png' });
    const dataTransfer = new DataTransfer();
    dataTransfer.items.add(file);

    await fireEvent.drop(dropZone, { dataTransfer });

    expect(toaster.error).not.toHaveBeenCalled();
  });

  it('should accept WebP files without showing a toast', async () => {
    const { container } = render(ImageDropZone, {
      props: { modelId: mockModelId }
    });

    const dropZone = container.querySelector('[role="button"]') as HTMLElement;
    const file = new File(['webp-data'], 'photo.webp', { type: 'image/webp' });
    const dataTransfer = new DataTransfer();
    dataTransfer.items.add(file);

    await fireEvent.drop(dropZone, { dataTransfer });

    expect(toaster.error).not.toHaveBeenCalled();
  });

  it('should proceed when MIME type is empty (backend will validate)', async () => {
    const { container } = render(ImageDropZone, {
      props: { modelId: mockModelId }
    });

    const dropZone = container.querySelector('[role="button"]') as HTMLElement;
    const file = new File(['unknown-data'], 'photo.jpg', { type: '' });
    const dataTransfer = new DataTransfer();
    dataTransfer.items.add(file);

    await fireEvent.drop(dropZone, { dataTransfer });

    // Empty MIME type passes through — no toast error
    expect(toaster.error).not.toHaveBeenCalled();
  });
});

describe('ImageDropZone - Drag & Drop States', () => {
  it('should show drag state when dragging over (using dragenter)', async () => {
    const { container } = render(ImageDropZone, {
      props: { modelId: mockModelId }
    });

    const dropZone = container.querySelector('[role="button"]') as HTMLElement;

    await fireEvent.dragEnter(dropZone);

    // Should show "Drop here to update photo" message
    expect(container.textContent).toContain('Drop here to update photo');
  });

  it('should reset to idle state when drag leaves after entering', async () => {
    const { container } = render(ImageDropZone, {
      props: { modelId: mockModelId }
    });

    const dropZone = container.querySelector('[role="button"]') as HTMLElement;

    await fireEvent.dragEnter(dropZone);
    expect(container.textContent).toContain('Drop here to update photo');

    await fireEvent.dragLeave(dropZone);

    // Counter back to 0 → idle state
    expect(container.textContent).toContain('Drag and drop your image here');
  });
});
