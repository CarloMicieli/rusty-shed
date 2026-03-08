import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/svelte';

// Mock convertFileSrc (still used by ModelDetailsHeader for the <img> src)
vi.mock('@tauri-apps/api/core', () => ({
  convertFileSrc: vi.fn((path: string) => `asset://localhost/${path}`)
}));

// Mock Tauri event listener used by ImageDropZone's onMount
vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn(() => Promise.resolve(() => {}))
}));

// Mock Tauri fs plugin used by ImageDropZone and ImageUpload
vi.mock('@tauri-apps/plugin-fs', () => ({
  readFile: vi.fn(() => Promise.resolve(new Uint8Array([0xff, 0xd8])))
}));

// Mock Tauri dialog plugin used by ImageUpload
vi.mock('@tauri-apps/plugin-dialog', () => ({
  open: vi.fn(() => Promise.resolve(null))
}));

// Mock paraglide messages
vi.mock('$lib/paraglide/messages.js', () => ({
  model_image_alt_no_image: () => 'No image available',
  model_image_placeholder: () => 'No image yet',
  upload_image: () => 'Upload Image',
  replace_image: () => 'Change Image',
  drag_and_drop_hint: () => 'Drag and drop an image here',
  drop_image_here: () => 'Drop image here',
  upload_formats_hint: () => 'Supported formats: JPG, PNG, GIF',
  uploading: () => 'Uploading...',
  upload_success: () => 'Image uploaded successfully',
  upload_error_model_not_found: () => 'Model not found',
  upload_error_unknown: () => 'An unknown error occurred',
  upload_error_multiple_files: () => 'Please upload only one file at a time',
  railway_model_content_in_english_fallback: () => '(EN)'
}));

import ModelDetailsHeader from '../ModelDetailsHeader.svelte';
import type { RailwayModelView, RailwayModelImageResponse } from '$lib/bindings';

describe('ModelDetailsHeader', () => {
  const mockModel: RailwayModelView = {
    id: 'trn:railway-model:maer:37858',
    description: 'Class 218 Diesel Locomotive',
    descriptionLang: 'en',
    manufacturer: {
      manufacturerId: 'maer',
      display: 'Märklin'
    },
    productCode: '37858',
    category: 'LOCOMOTIVES' as const,
    scale: 'H0',
    epoch: 'IV',
    powerMethod: 'DC',
    deliveryDate: null,
    availabilityStatus: null,
    details: null,
    detailsLang: null,
    metadata: {
      version: 1,
      created_at: '2024-01-01T00:00:00Z',
      updated_at: '2024-01-01T00:00:00Z'
    },
    rollingStock: []
  };

  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('Rendering', () => {
    it('should render model title and subtitle', () => {
      render(ModelDetailsHeader, {
        props: {
          model: mockModel,
          imageResponse: null
        }
      });

      expect(screen.getByText('Class 218 Diesel Locomotive')).toBeInTheDocument();
      expect(screen.getByText(/Märklin/)).toBeInTheDocument();
      expect(screen.getByText(/37858/)).toBeInTheDocument();
    });

    it('should render all badges (scale, epoch, power)', () => {
      render(ModelDetailsHeader, {
        props: {
          model: mockModel,
          imageResponse: null
        }
      });

      expect(screen.getByText('H0')).toBeInTheDocument();
      expect(screen.getByText('IV')).toBeInTheDocument();
      expect(screen.getByText('DC')).toBeInTheDocument();
    });

    it('should render image when available', () => {
      const imageResponse: RailwayModelImageResponse = {
        imagePath: '/path/to/image.jpg',
        placeholderHtml: null,
        hasImage: true
      };

      render(ModelDetailsHeader, {
        props: {
          model: mockModel,
          imageResponse
        }
      });

      const img = screen.getByAltText('Class 218 Diesel Locomotive');
      expect(img).toBeInTheDocument();
      expect(img).toHaveAttribute('src', 'asset://localhost//path/to/image.jpg');
    });

    it('should render HTML placeholder when no image', () => {
      const imageResponse: RailwayModelImageResponse = {
        imagePath: null,
        placeholderHtml: '<div class="placeholder">Placeholder</div>',
        hasImage: false
      };

      render(ModelDetailsHeader, {
        props: {
          model: mockModel,
          imageResponse
        }
      });

      // Check for placeholder content
      const container = screen.getByText('Placeholder').closest('div');
      expect(container).toBeInTheDocument();
    });

    it('should render fallback placeholder when no imageResponse', () => {
      render(ModelDetailsHeader, {
        props: {
          model: mockModel,
          imageResponse: null
        }
      });

      expect(screen.getByText('No image yet')).toBeInTheDocument();
    });
  });

  describe('Accessibility', () => {
    it('should have proper heading hierarchy', () => {
      render(ModelDetailsHeader, {
        props: {
          model: mockModel,
          imageResponse: null
        }
      });

      const heading = screen.getByRole('heading', { name: /Class 218 Diesel Locomotive/i });
      expect(heading).toBeInTheDocument();
      expect(heading.tagName).toBe('H1');
    });

    it('should have aria-label for fallback placeholder', () => {
      render(ModelDetailsHeader, {
        props: {
          model: mockModel,
          imageResponse: null
        }
      });

      const placeholder = screen.getByLabelText('No image available');
      expect(placeholder).toBeInTheDocument();
      expect(placeholder).toHaveAttribute('role', 'img');
    });

    it('should have alt text for images', () => {
      const imageResponse: RailwayModelImageResponse = {
        imagePath: '/path/to/image.jpg',
        placeholderHtml: null,
        hasImage: true
      };

      render(ModelDetailsHeader, {
        props: {
          model: mockModel,
          imageResponse
        }
      });

      const img = screen.getByRole('img');
      expect(img).toHaveAttribute('alt', 'Class 218 Diesel Locomotive');
    });
  });

  describe('Responsive Design', () => {
    it('should apply responsive classes', () => {
      const { container } = render(ModelDetailsHeader, {
        props: {
          model: mockModel,
          imageResponse: null
        }
      });

      const heading = container.querySelector('h1');
      expect(heading?.className).toContain('text-3xl');
      expect(heading?.className).toContain('sm:text-4xl');
    });

    it('should render badges with flex-wrap for mobile', () => {
      const { container } = render(ModelDetailsHeader, {
        props: {
          model: mockModel,
          imageResponse: null
        }
      });

      const badgeContainer = container.querySelector('.flex.flex-wrap');
      expect(badgeContainer).toBeInTheDocument();
    });
  });
});
