import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent, waitFor } from '@testing-library/svelte';
import { flushSync } from 'svelte';
import ImportDropZone from '$lib/features/import/components/ImportDropZone.svelte';

describe('ImportDropZone.svelte', () => {
  const onFilesSelected = vi.fn();

  beforeEach(() => {
    vi.clearAllMocks();
    onFilesSelected.mockResolvedValue(undefined);
  });

  it('renders the drop zone heading', () => {
    render(ImportDropZone, { props: {} });
    expect(screen.getByText('Drop your import package here')).toBeInTheDocument();
  });

  it('renders the instructional text', () => {
    render(ImportDropZone, { props: {} });
    expect(screen.getByText('Or click to select a .zip or .tar.gz file')).toBeInTheDocument();
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
    render(ImportDropZone, { props: { onFilesSelected } });
    const dropZone = document.querySelector('.import-drop-zone') as HTMLElement;
    expect(dropZone).not.toBeNull();

    await fireEvent.dragOver(dropZone);
    flushSync();

    expect(dropZone.classList.contains('dragging')).toBe(true);
  });

  it('removes "dragging" class on dragleave event', async () => {
    render(ImportDropZone, { props: { onFilesSelected } });
    const dropZone = document.querySelector('.import-drop-zone') as HTMLElement;

    await fireEvent.dragOver(dropZone);
    flushSync();
    expect(dropZone.classList.contains('dragging')).toBe(true);

    await fireEvent.dragLeave(dropZone);
    flushSync();
    expect(dropZone.classList.contains('dragging')).toBe(false);
  });

  it('removes "dragging" class after file drop', async () => {
    render(ImportDropZone, { props: { onFilesSelected } });
    const dropZone = document.querySelector('.import-drop-zone') as HTMLElement;

    await fireEvent.dragOver(dropZone);
    flushSync();

    // Create a mock DataTransfer
    const file = new File(['content'], 'backup.zip', { type: 'application/zip' });
    const dataTransfer = { files: [file] };
    await fireEvent.drop(dropZone, { dataTransfer });
    flushSync();

    expect(dropZone.classList.contains('dragging')).toBe(false);
  });

  it('calls onFilesSelected when file is dropped', async () => {
    render(ImportDropZone, { props: { onFilesSelected } });
    const dropZone = document.querySelector('.import-drop-zone') as HTMLElement;

    const file = new File(['content'], 'backup.zip', { type: 'application/zip' });
    const dataTransfer = {
      files: Object.assign([file], {
        item: (i: number) => [file][i],
        length: 1
      })
    };

    await fireEvent.drop(dropZone, { dataTransfer });

    await waitFor(() => {
      expect(onFilesSelected).toHaveBeenCalledTimes(1);
    });
  });

  it('adds "disabled" class when disabled=true', () => {
    render(ImportDropZone, { props: { disabled: true } });
    const dropZone = document.querySelector('.import-drop-zone') as HTMLElement;
    expect(dropZone.classList.contains('disabled')).toBe(true);
  });

  it('does not add "dragging" class when disabled prop is true', async () => {
    render(ImportDropZone, { props: { disabled: true, onFilesSelected } });
    const dropZone = document.querySelector('.import-drop-zone') as HTMLElement;

    await fireEvent.dragOver(dropZone);
    flushSync();

    expect(dropZone.classList.contains('dragging')).toBe(false);
  });

  it('does not call onFilesSelected when disabled and file dropped', async () => {
    render(ImportDropZone, { props: { disabled: true, onFilesSelected } });
    const dropZone = document.querySelector('.import-drop-zone') as HTMLElement;

    const file = new File(['content'], 'backup.zip', { type: 'application/zip' });
    const dataTransfer = { files: [file] };
    await fireEvent.drop(dropZone, { dataTransfer });

    expect(onFilesSelected).not.toHaveBeenCalled();
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

  it('renders file input with correct accept attribute', () => {
    render(ImportDropZone, { props: { accept: '.zip,.tar.gz' } });
    const input = document.querySelector('input[type="file"]') as HTMLInputElement;
    expect(input?.getAttribute('accept')).toBe('.zip,.tar.gz');
  });
});
