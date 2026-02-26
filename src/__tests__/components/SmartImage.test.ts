import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, waitFor } from '@testing-library/svelte';
import SmartImage from '$lib/components/SmartImage.svelte';

// ── Mock @tauri-apps/api/core ────────────────────────────────────────────────
const mockConvertFileSrc = vi.fn((path: string) => `asset://localhost/${path}`);
vi.mock('@tauri-apps/api/core', () => ({
  convertFileSrc: (path: string) => mockConvertFileSrc(path)
}));

// ── Mock image service ────────────────────────────────────────────────────────
const mockResolveImagePath = vi.fn();
vi.mock('$lib/services/image.service.svelte', () => ({
  imageService: {
    resolveImagePath: (...args: unknown[]) => mockResolveImagePath(...args)
  }
}));

describe('SmartImage.svelte', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders the smart-image container', () => {
    mockResolveImagePath.mockResolvedValue(null);
    render(SmartImage, { props: { id: 'img-1', category: 'railway_model' } });
    expect(document.querySelector('.smart-image')).not.toBeNull();
  });

  it('uses static path directly for category="static"', async () => {
    render(SmartImage, { props: { id: 'logo.png', category: 'static' } });
    await waitFor(() => {
      const img = document.querySelector('img') as HTMLImageElement;
      expect(img).not.toBeNull();
      expect(img?.src).toContain('/logos/logo.png');
    });
    // imageService should NOT be called for static images
    expect(mockResolveImagePath).not.toHaveBeenCalled();
  });

  it('shows "Image unavailable" fallback when imageService returns null', async () => {
    mockResolveImagePath.mockResolvedValue(null);
    render(SmartImage, { props: { id: 'img-1', category: 'railway_model' } });
    await waitFor(() => {
      expect(screen.getByText('Image unavailable')).toBeInTheDocument();
    });
  });

  it('shows error fallback when id is empty', async () => {
    mockResolveImagePath.mockResolvedValue(null);
    render(SmartImage, { props: { id: '', category: 'railway_model' } });
    await waitFor(() => {
      expect(screen.getByText('Image unavailable')).toBeInTheDocument();
    });
  });

  it('shows image when imageService resolves a valid path', async () => {
    mockResolveImagePath.mockResolvedValue('/path/to/model.jpg');
    mockConvertFileSrc.mockReturnValue('asset://localhost/path/to/model.jpg');
    render(SmartImage, { props: { id: 'img-1', category: 'railway_model', alt: 'Model Photo' } });
    await waitFor(() => {
      const img = document.querySelector('img') as HTMLImageElement;
      expect(img).not.toBeNull();
      expect(img?.alt).toBe('Model Photo');
    });
  });

  it('renders smart-image__fallback element for error state', async () => {
    mockResolveImagePath.mockResolvedValue(null);
    render(SmartImage, { props: { id: 'img-1', category: 'railway_model' } });
    await waitFor(() => {
      expect(document.querySelector('.smart-image__fallback')).not.toBeNull();
    });
  });

  it('renders smart-image__img when src resolves successfully', async () => {
    mockResolveImagePath.mockResolvedValue('/models/train.jpg');
    mockConvertFileSrc.mockReturnValue('asset://localhost/models/train.jpg');
    render(SmartImage, { props: { id: 'img-2', category: 'railway_model' } });
    await waitFor(() => {
      expect(document.querySelector('.smart-image__img')).not.toBeNull();
    });
  });

  it('shows error fallback when imageService throws', async () => {
    mockResolveImagePath.mockRejectedValue(new Error('Network error'));
    render(SmartImage, { props: { id: 'img-3', category: 'railway_model' } });
    await waitFor(() => {
      const fallback = document.querySelector('.smart-image__fallback');
      expect(fallback).not.toBeNull();
    });
  });

  it('calls imageService.resolveImagePath with correct id and category', async () => {
    mockResolveImagePath.mockResolvedValue(null);
    render(SmartImage, { props: { id: 'abc123', category: 'railway_model' } });
    await waitFor(() => {
      expect(mockResolveImagePath).toHaveBeenCalledWith('abc123', 'railway_model');
    });
  });

  it('applies custom className to smart-image__img', async () => {
    mockResolveImagePath.mockResolvedValue('/models/test.jpg');
    mockConvertFileSrc.mockReturnValue('asset://localhost/models/test.jpg');
    render(SmartImage, {
      props: { id: 'img-4', category: 'railway_model', class: 'rounded-xl' }
    });
    await waitFor(() => {
      const img = document.querySelector('.smart-image__img') as HTMLImageElement;
      expect(img?.classList.contains('rounded-xl')).toBe(true);
    });
  });
});
