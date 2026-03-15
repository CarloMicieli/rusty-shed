import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent, waitFor } from '@testing-library/svelte';
import { flushSync } from 'svelte';

// Mock Tauri dialog — open() returns an absolute path string or null
vi.mock('@tauri-apps/plugin-dialog', () => ({
  open: vi.fn()
}));

// Mock Tauri webviewWindow — onDragDropEvent returns a no-op unlisten promise
vi.mock('@tauri-apps/api/webviewWindow', () => ({
  getCurrentWebviewWindow: vi.fn(() => ({
    onDragDropEvent: vi.fn(() => Promise.resolve(() => {}))
  }))
}));

import ImportDropZone from '$lib/features/import/components/ImportDropZone.svelte';
import { open } from '@tauri-apps/plugin-dialog';

describe('ImportDropZone.svelte', () => {
  const onFileSelected = vi.fn();

  beforeEach(() => {
    vi.clearAllMocks();
    onFileSelected.mockResolvedValue(undefined);
  });

  it('renders the drop zone heading', () => {
    render(ImportDropZone, { props: {} });
    expect(screen.getByText('Drop your import file here')).toBeInTheDocument();
  });

  it('renders the instructional text', () => {
    render(ImportDropZone, { props: {} });
    expect(screen.getByText('Supported formats: .zip, .tar.gz')).toBeInTheDocument();
  });

  it('renders "Select File" button', () => {
    render(ImportDropZone, { props: {} });
    expect(screen.getByRole('button', { name: 'Select File' })).toBeInTheDocument();
  });

  it('has role="button" on the drop zone container', () => {
    render(ImportDropZone, { props: {} });
    const dropZone = document.querySelector('[role="button"]');
    expect(dropZone).not.toBeNull();
  });

  it('adds "dragging" class on dragover event', async () => {
    render(ImportDropZone, { props: { onFileSelected } });
    const dropZone = document.querySelector('.import-drop-zone') as HTMLElement;
    expect(dropZone).not.toBeNull();

    await fireEvent.dragOver(dropZone);
    flushSync();

    expect(dropZone.classList.contains('dragging')).toBe(true);
  });

  it('removes "dragging" class on dragleave event', async () => {
    render(ImportDropZone, { props: { onFileSelected } });
    const dropZone = document.querySelector('.import-drop-zone') as HTMLElement;

    await fireEvent.dragOver(dropZone);
    flushSync();
    expect(dropZone.classList.contains('dragging')).toBe(true);

    await fireEvent.dragLeave(dropZone);
    flushSync();
    expect(dropZone.classList.contains('dragging')).toBe(false);
  });

  it('removes "dragging" class after file drop', async () => {
    render(ImportDropZone, { props: { onFileSelected } });
    const dropZone = document.querySelector('.import-drop-zone') as HTMLElement;

    await fireEvent.dragOver(dropZone);
    flushSync();

    await fireEvent.drop(dropZone);
    flushSync();

    expect(dropZone.classList.contains('dragging')).toBe(false);
  });

  it('calls open() dialog when clicked', async () => {
    vi.mocked(open).mockResolvedValue('/home/user/backup.zip');
    render(ImportDropZone, { props: { onFileSelected } });
    const btn = screen.getByRole('button', { name: 'Select File' });

    await fireEvent.click(btn);

    await waitFor(() => {
      expect(open).toHaveBeenCalledWith({
        multiple: false,
        filters: [{ name: 'Archive', extensions: ['zip', 'tar.gz', 'tgz'] }]
      });
      expect(onFileSelected).toHaveBeenCalledWith('/home/user/backup.zip');
    });
  });

  it('does not call onFileSelected when dialog is cancelled', async () => {
    vi.mocked(open).mockResolvedValue(null);
    render(ImportDropZone, { props: { onFileSelected } });
    const btn = screen.getByRole('button', { name: 'Select File' });

    await fireEvent.click(btn);

    await waitFor(() => {
      expect(open).toHaveBeenCalled();
      expect(onFileSelected).not.toHaveBeenCalled();
    });
  });

  it('adds "disabled" class when disabled=true', () => {
    render(ImportDropZone, { props: { disabled: true } });
    const dropZone = document.querySelector('.import-drop-zone') as HTMLElement;
    expect(dropZone.classList.contains('disabled')).toBe(true);
  });

  it('does not add "dragging" class when disabled prop is true', async () => {
    render(ImportDropZone, { props: { disabled: true, onFileSelected } });
    const dropZone = document.querySelector('.import-drop-zone') as HTMLElement;

    await fireEvent.dragOver(dropZone);
    flushSync();

    expect(dropZone.classList.contains('dragging')).toBe(false);
  });

  it('does not call open() when disabled and clicked', async () => {
    render(ImportDropZone, { props: { disabled: true, onFileSelected } });
    const dropZone = document.querySelector('.import-drop-zone') as HTMLElement;

    await fireEvent.click(dropZone);

    expect(open).not.toHaveBeenCalled();
  });

  it('is keyboard accessible with tabindex=0 when not disabled', () => {
    render(ImportDropZone, { props: {} });
    const dropZone = document.querySelector('[role="button"]') as HTMLElement;
    expect(dropZone?.getAttribute('tabindex')).toBe('0');
  });

  it('has tabindex=-1 when disabled', () => {
    render(ImportDropZone, { props: { disabled: true } });
    const dropZone = document.querySelector('[role="button"]') as HTMLElement;
    expect(dropZone?.getAttribute('tabindex')).toBe('-1');
  });
});
