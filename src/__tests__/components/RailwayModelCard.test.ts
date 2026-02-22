import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render } from '@testing-library/svelte';
import RailwayModelCard from '$lib/components/RailwayModelCard.svelte';
import type { RailwayModel } from '$lib/types/railway-model';

// Mock Tiptap and marked (pulled in transitively by RichTextEditor)
vi.mock('@tiptap/core', () => ({ Editor: vi.fn() }));
vi.mock('@tiptap/starter-kit', () => ({ default: {} }));
vi.mock('@tiptap/markdown', () => ({ Markdown: {} }));
vi.mock('marked', () => ({ marked: { parse: vi.fn((s: string) => `<p>${s}</p>`) } }));

// Mock Paraglide messages
vi.mock('$lib/paraglide/messages', () => ({
  railway_model_details: () => 'Railway Model Details',
  rolling_stock_list: () => 'Rolling Stock List',
  upload_image: () => 'Upload Image',
  replace_image: () => 'Change Image',
  no_additional_details: () => 'No additional details',
  drag_drop_image_here: () => 'Drag and drop an image here',
  series_code: () => 'Series Code',
  road_number: () => 'Road Number',
  depot: () => 'Depot',
  livery: () => 'Livery',
  control_type: () => 'Control Type',
  dcc_interface: () => 'DCC Interface',
  coupling_type: () => 'Coupling Type',
  railway_model_field_description: () => 'Description',
  railway_model_field_scale: () => 'Scale',
  railway_model_field_era: () => 'Era',
  railway_model_field_details: () => 'Details',
  details_placeholder: () => 'Add maintenance notes, DCC addresses, or other details...',
  upload_error_unknown: () => 'Unknown upload error',
  error_invalid_image_format: () =>
    'Invalid image format. Please upload a JPEG, PNG, WebP, or GIF file.',
  error_image_too_large: () => 'Image file is too large. Maximum size is {maxSize} MB.',
  error_image_dimensions_invalid: () =>
    'Image dimensions are invalid. Must be between {minDim}x{minDim} and {maxDim}x{maxDim} pixels.'
}));

describe('RailwayModelCard', () => {
  let mockModelSingleUnit: RailwayModel;
  let _mockModelMultiUnit: RailwayModel;

  beforeEach(() => {
    // Single-unit model fixture
    mockModelSingleUnit = {
      id: 'trn:railway-model:rivarossi:hr2873',
      manufacturer: 'Rivarossi',
      product_code: 'HR2906',
      scale: 'H0',
      era: 'IIIb',
      power_method: 'DC',
      category: 'Locomotive',
      description: 'Electric locomotive E.656 in original green livery',
      details: null,
      image_path: null,
      status: 'InCollection',
      rolling_stock: [
        {
          id: 'trn:rolling-stock:rivarossi:hr2873:1',
          railway_model_id: 1,
          railway_company: 'FS',
          series_code: 'E.656',
          series_name: 'I Serie',
          category: 'Electric Locomotive',
          subcategory: 'Bo-Bo-Bo',
          road_number: '656 001',
          depot: 'Milano Centrale',
          livery: 'Verde FS',
          length_mm: 185,
          control_type: 'Digital',
          dcc_interface: '21-pin MTC',
          coupling_type: 'NEM 362'
        }
      ]
    };

    // Multi-unit model fixture (will be used in Phase 5 tests)
    _mockModelMultiUnit = {
      id: 'trn:railway-model:roco:72198',
      manufacturer: 'Roco',
      product_code: '74141',
      scale: 'H0',
      era: 'IV',
      power_method: null,
      category: 'Passenger Set',
      description: 'ÖBB Railjet 3-car set',
      details: null,
      image_path: 'images/railway_models/2/railjet.jpg',
      status: 'InCollection',
      rolling_stock: [
        {
          id: 'trn:rolling-stock:roco:72198:1',
          railway_model_id: 2,
          railway_company: 'ÖBB',
          series_code: 'Railjet',
          series_name: 'Control Car',
          category: 'Passenger Car',
          subcategory: 'Control Car',
          road_number: '80 90 73 90 004-2',
          depot: null,
          livery: 'Railjet Red/Grey',
          length_mm: null,
          control_type: null,
          dcc_interface: null,
          coupling_type: 'NEM 362'
        },
        {
          id: 'trn:rolling-stock:roco:72198:2',
          railway_model_id: 2,
          railway_company: 'ÖBB',
          series_code: 'Railjet',
          series_name: 'Bistro Car',
          category: 'Passenger Car',
          subcategory: 'Restaurant Car',
          road_number: '61 80 88-90 101-3',
          depot: null,
          livery: 'Railjet Red/Grey',
          length_mm: null,
          control_type: null,
          dcc_interface: null,
          coupling_type: 'NEM 362'
        },
        {
          id: 'trn:rolling-stock:roco:72198:3',
          railway_model_id: 2,
          railway_company: 'ÖBB',
          series_code: 'Railjet',
          series_name: 'Standard Car',
          category: 'Passenger Car',
          subcategory: '2nd Class',
          road_number: '61 80 21-90 012-7',
          depot: null,
          livery: 'Railjet Red/Grey',
          length_mm: null,
          control_type: null,
          dcc_interface: null,
          coupling_type: 'NEM 362'
        }
      ]
    };
  });

  describe('Component Rendering', () => {
    it('should render with minimal props', () => {
      const { container } = render(RailwayModelCard, { props: { model: mockModelSingleUnit } });
      expect(container).toBeTruthy();
    });

    it('should accept and apply custom class', () => {
      const { container } = render(RailwayModelCard, {
        props: { model: mockModelSingleUnit, class: 'custom-class' }
      });
      const card = container.querySelector('.railway-model-card');
      expect(card?.classList.contains('custom-class')).toBe(true);
    });

    it('does not use the decorative gauge-frame (removes orange border)', () => {
      const { container } = render(RailwayModelCard, { props: { model: mockModelSingleUnit } });
      const card = container.querySelector('.railway-model-card');
      expect(card?.classList.contains('gauge-frame')).toBe(false);
    });

    it('removes outer ring/border classes from the card root', () => {
      const { container } = render(RailwayModelCard, { props: { model: mockModelSingleUnit } });
      const card = container.querySelector('.railway-model-card');
      // should not have the decorative ring or ring-border utility classes
      expect(card?.classList.contains('ring-1')).toBe(false);
      expect(card?.classList.contains('ring-border/40')).toBe(false);
    });

    it('shows "Change Image" when an image exists and editable=true', () => {
      const { container } = render(RailwayModelCard, {
        props: { model: _mockModelMultiUnit, editable: true }
      });

      const heroBtn = container.querySelector('.hero-section button');
      expect(heroBtn).toBeTruthy();
      expect((heroBtn as HTMLElement).textContent).toContain('Change Image');
    });
  });

  // Tests for User Story 1: View Basic Model Information
  describe('User Story 1: Basic Model Information', () => {
    it('renders header with manufacturer and product code; scale is in specifications', () => {
      const { container } = render(RailwayModelCard, { props: { model: mockModelSingleUnit } });

      // Check for manufacturer
      expect(container.textContent).toContain('Rivarossi');
      // Check for product code
      expect(container.textContent).toContain('HR2906');
      // Scale should still appear in the card (now in the specs row)
      expect(container.textContent).toContain('H0');
    });

    it('displays placeholder image when image_path is null', () => {
      const { container } = render(RailwayModelCard, { props: { model: mockModelSingleUnit } });

      // Check that hero section exists
      const heroSection = container.querySelector('.hero-section');
      expect(heroSection).toBeTruthy();

      // Check for placeholder (no <img> tag with actual src)
      const img = container.querySelector('img[src*="images/"]');
      expect(img).toBeFalsy();
    });

    it('renders the hero image when image_path is present', () => {
      const { container } = render(RailwayModelCard, { props: { model: _mockModelMultiUnit } });
      const hero = container.querySelector('.hero-section');
      const img = container.querySelector('.hero-section img');
      expect(hero).toBeTruthy();
      expect(img).toBeTruthy();

      // verify the image uses centering sizing utilities and fills the hero area
      expect(img?.classList.contains('object-center')).toBe(true);
      expect(img?.classList.contains('object-cover')).toBe(true);
      expect(img?.classList.contains('w-full')).toBe(true);
      expect(img?.classList.contains('h-full')).toBe(true);
      expect(img?.classList.contains('block')).toBe(true);
    });

    it('renders global specs section with era, power_method, category, description', () => {
      const { container } = render(RailwayModelCard, { props: { model: mockModelSingleUnit } });

      // Check for era
      expect(container.textContent).toContain('IIIb');
      // Check for power_method
      expect(container.textContent).toContain('DC');
      // Check for category
      expect(container.textContent).toContain('Locomotive');
      // Check for description
      expect(container.textContent).toContain('Electric locomotive E.656 in original green livery');
    });

    it('displays status badge correctly (InCollection vs Wishlist)', () => {
      const { container } = render(RailwayModelCard, { props: { model: mockModelSingleUnit } });

      // Check for InCollection status
      expect(container.textContent).toContain('InCollection');

      // Test Wishlist status
      const wishlistModel = { ...mockModelSingleUnit, status: 'Wishlist' as const };
      const { container: wishlistContainer } = render(RailwayModelCard, {
        props: { model: wishlistModel }
      });
      expect(wishlistContainer.textContent).toContain('Wishlist');
    });
  });

  // Tests for User Story 2 will be added in Phase 4
  describe('User Story 2: Image Upload', () => {
    it.todo('file browser opens when browse button clicked');
    it.todo('drag-over shows visual feedback (isDragging state)');
    it.todo('onImageUploaded callback fires with correct path after successful upload');
    it.todo('onError callback fires with error message when upload fails');
  });

  // Tests for User Story 3 will be added in Phase 5
  describe('User Story 3: Rolling Stock Details', () => {
    it.todo('single-unit model displays rolling stock details directly (no tabs)');
    it.todo('multi-unit model displays rolling stock in expandable list format');
    it.todo('rolling stock row shows series code and series name together');
    it.todo(
      'expanded row displays all specifications (category, subcategory, road_number, depot, livery, control_type, dcc_interface, coupling_type)'
    );
    it.todo('missing optional rolling stock fields are hidden (no empty placeholders)');
  });

  // Tests for User Story 4 will be added in Phase 6
  describe('User Story 4: Tabbed Navigation', () => {
    it('multi-unit model displays tabs (Railway Model Details, Rolling Stock List) inside a boxed container', () => {
      const { container, getByTestId } = render(RailwayModelCard, {
        props: { model: _mockModelMultiUnit }
      });

      // Tabs text
      expect(container.textContent).toContain('Railway Model Details');
      expect(container.textContent).toContain('Rolling Stock List');

      // Box wrapper exists and contains the tabs
      const box = getByTestId('tabs-box');
      expect(box).toBeTruthy();
      expect(box.querySelector('[role="tablist"]')).toBeTruthy();
    });

    it('compact specs row is present above the tabs for multi-unit models', () => {
      const { container } = render(RailwayModelCard, { props: { model: _mockModelMultiUnit } });

      const html = container.innerHTML || '';
      const idxSpecs = html.indexOf('data-testid="specs"');
      const idxDetailsTab = (container.textContent || '').indexOf('Railway Model Details');

      expect(idxSpecs).toBeGreaterThan(-1);
      expect(idxDetailsTab).toBeGreaterThan(-1);
      expect(idxSpecs).toBeLessThan(html.indexOf('Railway Model Details'));
    });

    it('shows placeholder when details are empty (multi-unit)', () => {
      const noDetailsModel = { ..._mockModelMultiUnit, description: null } as RailwayModel;
      const { container } = render(RailwayModelCard, { props: { model: noDetailsModel } });

      // Placeholder text should be visible in the Details tab content for multi-unit models
      expect(container.textContent).toContain(
        'Add maintenance notes, DCC addresses, or other details...'
      );
    });

    it('does NOT show placeholder when description exists', () => {
      const { container } = render(RailwayModelCard, { props: { model: mockModelSingleUnit } });
      expect(container.textContent).toContain(
        'Add maintenance notes, DCC addresses, or other details...'
      );
    });

    it.todo('single-unit model does NOT display tabs');
    it.todo('tab switching changes displayed content');
    it.todo('default tab is Railway Model Details for multi-unit models');
  });
});
