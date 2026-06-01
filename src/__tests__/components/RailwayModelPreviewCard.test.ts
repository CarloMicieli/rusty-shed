import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import RailwayModelPreviewCard, {
  type ModelCategory
} from '$lib/components/RailwayModelPreviewCard.svelte';

// Mock Paraglide messages
vi.mock('$lib/paraglide/messages.js', () => ({
  components_deleteConfirmTitle: () => 'Delete Model?',
  components_deleteConfirmMessage: ({ model }: { model: string }) =>
    `Are you sure you want to delete ${model}? This action cannot be undone.`,
  components_unknownManufacturer: () => 'Unknown',
  depot_category: () => 'Category',
  depot_scale: () => 'Scale',
  enum_category_locomotives: () => 'Locomotives',
  enum_category_freight_cars: () => 'Freight Cars',
  enum_category_passenger_cars: () => 'Passenger Cars',
  enum_category_railcars: () => 'Railcars',
  enum_category_train_sets: () => 'Train Sets',
  components_purchaseDate: () => 'PURCHASED',
  dashboard_card_price: () => 'PRICE',
  components_deleteButton: () => 'Delete model',
  common_delete: () => 'Delete',
  common_cancel: () => 'Cancel',
  railway_model_digital_features_label: () => 'Digital features',
  railway_model_sound_equipped_title: () => 'Sound equipped',
  railway_model_dcc_equipped_title: () => 'DCC equipped',
  railway_model_fallback_en_title: () => 'Content displayed in English (fallback)',
  depot_era: () => 'Era',
  railway_model_image_alt: () => 'Railway model image'
}));

vi.mock('$lib/bindings', () => ({
  commands: {
    getRailwayModelImage: vi.fn().mockResolvedValue({
      status: 'ok',
      data: {
        hasImage: false,
        imagePath: null,
        placeholderHtml: null
      }
    })
  }
}));

describe('RailwayModelPreviewCard', () => {
  const mockModel = {
    id: 'test-001',
    manufacturer: 'Märklin',
    productCode: '37586',
    series: 'Class 66',
    category: 'DieselLocomotive' as const,
    scale: 'H0',
    powerMethod: 'DCC',
    era: 'VI',
    purchaseDate: '2024-06-15',
    photoUrl: null,
    unitCount: 1,
    digitalFeatures: ['Sound' as const]
  };

  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('Rendering', () => {
    it('should render with card structure, apply custom class, and display all core model info', () => {
      const { container } = render(RailwayModelPreviewCard, {
        props: { model: mockModel, class: 'custom-class' }
      });

      const card = container.querySelector('.card');
      expect(card).toBeTruthy();
      expect(card?.classList.contains('custom-class')).toBe(true);
      expect(card?.classList.contains('transition-all')).toBe(true);

      expect(screen.getByText('Märklin')).toBeTruthy();
      expect(screen.getByText(/37586/)).toBeTruthy();
      expect(screen.getByText(/Class 66/)).toBeTruthy();
      expect(screen.getByText('Locomotives')).toBeTruthy();
    });
  });

  describe('User Story 1: Core Model Information Display', () => {
    it('renders thumbnail with 16:9 aspect ratio when photoUrl provided', () => {
      const { container } = render(RailwayModelPreviewCard, {
        props: { model: { ...mockModel, photoUrl: 'https://example.com/photo.jpg' } }
      });

      const thumbnail = container.querySelector('.aspect-video');
      expect(thumbnail).toBeTruthy();
    });

    it('displays fallback for missing manufacturer', () => {
      render(RailwayModelPreviewCard, {
        props: { model: { ...mockModel, manufacturer: null } }
      });
      expect(screen.getByText('Unknown')).toBeTruthy();
    });

    it('displays category label in specs row', () => {
      render(RailwayModelPreviewCard, { props: { model: mockModel } });
      expect(screen.getByText('Category')).toBeTruthy();
      expect(screen.getByText('Locomotives')).toBeTruthy();
    });
  });

  describe('User Story 2: Metadata Badges and Status Indicators', () => {
    it('displays scale, power method, era badges; omits them when data is missing', () => {
      const { container, unmount } = render(RailwayModelPreviewCard, {
        props: { model: mockModel }
      });
      expect(screen.getByText('H0')).toBeTruthy();
      expect(screen.getByText('DCC')).toBeTruthy();
      expect(screen.getByText(/VI/)).toBeTruthy();
      expect(screen.getByText('PURCHASED')).toBeTruthy();
      expect(container.querySelectorAll('[class*="sm:grid-cols-3"]').length).toBe(1);
      expect(container.querySelectorAll('[class*="sm:grid-cols-2"]').length).toBe(1);
      unmount();

      render(RailwayModelPreviewCard, {
        props: {
          model: { ...mockModel, scale: null, powerMethod: null, era: null, purchaseDate: null }
        }
      });
      expect(screen.queryByText('H0')).toBeFalsy();
      expect(screen.queryByText('DCC')).toBeFalsy();
      expect(screen.queryByText(/VI/)).toBeFalsy();
      expect(screen.queryByText(/PURCHASED/)).toBeFalsy();
    });

    it('displays unit count badge when > 1 and hides it when unitCount is 1', () => {
      const { container, unmount } = render(RailwayModelPreviewCard, {
        props: { model: { ...mockModel, unitCount: 3 } }
      });
      expect(screen.getByText(/×3/)).toBeTruthy();
      expect(container.querySelector('.absolute.bottom-2.left-2')).toBeTruthy();
      unmount();

      render(RailwayModelPreviewCard, { props: { model: { ...mockModel, unitCount: 1 } } });
      expect(screen.queryByText(/×1/)).toBeFalsy();
    });

    it('displays digital feature icons in top-left with flex layout for multiple features', () => {
      const { container } = render(RailwayModelPreviewCard, {
        props: { model: { ...mockModel, digitalFeatures: ['Sound', 'DCC', 'Light'] } }
      });
      expect(container.querySelector('.absolute.top-2.left-2')).toBeTruthy();
      expect(container.querySelector('.flex.gap-1')).toBeTruthy();
    });
  });

  describe('User Story 4: Delete Functionality', () => {
    it('renders delete button when onDelete provided, hides it when omitted', () => {
      const onDelete = vi.fn();
      const { unmount } = render(RailwayModelPreviewCard, {
        props: { model: mockModel, onDelete }
      });
      expect(screen.getByLabelText(/delete/i)).toBeTruthy();
      unmount();

      render(RailwayModelPreviewCard, { props: { model: mockModel } });
      expect(screen.queryByLabelText(/delete/i)).toBeFalsy();
    });

    it('should open confirmation dialog on delete button click', async () => {
      const onDelete = vi.fn();
      render(RailwayModelPreviewCard, {
        props: { model: mockModel, onDelete }
      });

      const deleteButton = screen.getByLabelText(/delete/i);
      await fireEvent.click(deleteButton);

      // Dialog should be open
      expect(screen.getByText(/Delete Model\?/i)).toBeTruthy();
      expect(screen.getByText(/Are you sure/i)).toBeTruthy();
    });

    it('should call onDelete when user confirms', async () => {
      const onDelete = vi.fn();
      render(RailwayModelPreviewCard, {
        props: { model: mockModel, onDelete }
      });

      // Click delete button
      const deleteButton = screen.getByLabelText(/delete model/i);
      await fireEvent.click(deleteButton);

      // Click confirm button in dialog (not the icon button)
      const confirmButtons = screen.getAllByRole('button');
      const confirmButton = confirmButtons.find((btn) => btn.textContent === 'Delete');
      expect(confirmButton).toBeTruthy();
      await fireEvent.click(confirmButton!);

      expect(onDelete).toHaveBeenCalledWith('test-001');
    });

    it('should not call onDelete when user cancels', async () => {
      const onDelete = vi.fn();
      render(RailwayModelPreviewCard, {
        props: { model: mockModel, onDelete }
      });

      // Click delete button
      const deleteButton = screen.getByLabelText(/delete/i);
      await fireEvent.click(deleteButton);

      // Click cancel button in dialog
      const cancelButton = screen.getByRole('button', { name: /cancel/i });
      await fireEvent.click(cancelButton);

      expect(onDelete).not.toHaveBeenCalled();
    });
  });

  describe('User Story 3: Category Placeholder Icons', () => {
    it('displays placeholder SVG icon for various categories (steam, electric, wagon, unknown)', () => {
      for (const category of [
        'SteamLocomotive',
        'ElectricLocomotive',
        'Wagon',
        'Unknown'
      ] as ModelCategory[]) {
        const { container, unmount } = render(RailwayModelPreviewCard, {
          props: { model: { ...mockModel, category, photoUrl: null } }
        });
        expect(container.querySelector('svg.lucide-icon')).toBeTruthy();
        unmount();
      }
    });

    it('should display photo when photoUrl provided', () => {
      const { container } = render(RailwayModelPreviewCard, {
        props: {
          model: { ...mockModel, photoUrl: 'https://example.com/photo.jpg' }
        }
      });

      // Should display img element
      const img = container.querySelector('img');
      expect(img).toBeTruthy();
      expect(img?.getAttribute('src')).toBe('https://example.com/photo.jpg');
    });
  });

  describe('Accessibility', () => {
    it('has ARIA label on delete button, semantic heading, lazy-loaded image, and overlay ARIA labels', () => {
      const onDelete = vi.fn();
      const { unmount } = render(RailwayModelPreviewCard, {
        props: { model: mockModel, onDelete }
      });
      expect(screen.getByLabelText(/delete model/i)).toBeTruthy();
      unmount();

      const { container } = render(RailwayModelPreviewCard, {
        props: {
          model: {
            ...mockModel,
            photoUrl: 'https://example.com/photo.jpg',
            unitCount: 3,
            digitalFeatures: ['Sound', 'DCC']
          }
        }
      });
      const heading = container.querySelector('h3');
      expect(heading).toBeTruthy();
      expect(heading?.textContent).toContain('Class 66');

      const imgs = container.querySelectorAll('img');
      const img = imgs[1];
      expect(img?.getAttribute('loading')).toBe('lazy');
      expect(img?.getAttribute('decoding')).toBe('async');

      expect(screen.queryByLabelText(/digital features/i)).toBeTruthy();
      expect(screen.queryByText(/×3/)).toBeTruthy();
    });
  });

  describe('Responsive Design', () => {
    it('should use responsive grid classes and hover transition on card', () => {
      const { container } = render(RailwayModelPreviewCard, {
        props: { model: mockModel }
      });

      expect(container.querySelector('.grid')).toBeTruthy();
      const card = container.querySelector('.card');
      expect(card).toBeTruthy();
      expect(card?.classList.contains('transition-all')).toBe(true);
    });
  });
});
