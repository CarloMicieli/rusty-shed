import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import RailwayModelPreviewCard from '$lib/components/RailwayModelPreviewCard.svelte';

// Mock Paraglide messages
vi.mock('$lib/paraglide/messages.js', () => ({
  components_deleteConfirmTitle: () => 'Delete Model?',
  components_deleteConfirmMessage: ({ model }: { model: string }) =>
    `Are you sure you want to delete ${model}? This action cannot be undone.`,
  components_unknownManufacturer: () => 'Unknown',
  components_noRoadNumber: () => '---',
  components_purchaseDate: () => 'PURCHASED',
  components_deleteButton: () => 'Delete model',
  common_delete: () => 'Delete',
  common_cancel: () => 'Cancel'
}));

describe('RailwayModelPreviewCard', () => {
  const mockModel = {
    id: 'test-001',
    manufacturer: 'Märklin',
    productCode: '37586',
    series: 'Class 66',
    category: 'DieselLocomotive' as const,
    roadNumber: '66 001',
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
    it('should render the component with card structure', () => {
      const { container } = render(RailwayModelPreviewCard, {
        props: { model: mockModel }
      });

      // Check that card element is rendered
      const card = container.querySelector('.card.gauge-frame');
      expect(card).toBeTruthy();
      expect(card?.classList.contains('ring-1')).toBe(true);
      expect(card?.classList.contains('ring-border/40')).toBe(true);
    });

    it('should apply custom class to card', () => {
      const { container } = render(RailwayModelPreviewCard, {
        props: { model: mockModel, class: 'custom-class' }
      });

      const card = container.querySelector('.card');
      expect(card?.classList.contains('custom-class')).toBe(true);
    });
  });

  describe('User Story 1: Core Model Information Display', () => {
    it('should display manufacturer and product code', () => {
      render(RailwayModelPreviewCard, {
        props: { model: mockModel }
      });

      expect(screen.getByText('Märklin')).toBeTruthy();
      expect(screen.getByText(/37586/)).toBeTruthy();
    });

    it('should display series and category as title', () => {
      render(RailwayModelPreviewCard, {
        props: { model: mockModel }
      });

      expect(screen.getByText(/Class 66/)).toBeTruthy();
    });

    it('should display road number in identification plate', () => {
      render(RailwayModelPreviewCard, {
        props: { model: mockModel }
      });

      expect(screen.getByText(/66 001/)).toBeTruthy();
      // Check for monospaced font
      const roadNumberElement = screen.getByText(/66 001/).closest('span');
      expect(roadNumberElement?.classList.contains('font-mono')).toBe(true);
    });

    it('should render thumbnail with 16:9 aspect ratio', () => {
      const { container } = render(RailwayModelPreviewCard, {
        props: { model: { ...mockModel, photoUrl: 'https://example.com/photo.jpg' } }
      });

      const thumbnail = container.querySelector('.aspect-video');
      expect(thumbnail).toBeTruthy();
    });

    it('should truncate long road numbers (>25 chars)', () => {
      const longRoadNumber = '12 34 56 78 90 12 34 56 78 90 123'; // 35 characters
      render(RailwayModelPreviewCard, {
        props: { model: { ...mockModel, roadNumber: longRoadNumber } }
      });

      // Should show truncated version
      expect(screen.getByText(/\.\.\./)).toBeTruthy();
      // Should not show full number in initial view
      const fullText = screen.queryByText(longRoadNumber);
      expect(fullText).toBeFalsy();
    });

    it('should display fallback for missing road number', () => {
      render(RailwayModelPreviewCard, {
        props: { model: { ...mockModel, roadNumber: null } }
      });

      expect(screen.getByText(/# ---/)).toBeTruthy();
    });

    it('should display fallback for missing manufacturer', () => {
      render(RailwayModelPreviewCard, {
        props: { model: { ...mockModel, manufacturer: null } }
      });

      expect(screen.getByText('Unknown')).toBeTruthy();
    });
  });

  describe('User Story 2: Metadata Badges and Status Indicators', () => {
    it('should display scale and power method badges', () => {
      render(RailwayModelPreviewCard, {
        props: { model: mockModel }
      });

      expect(screen.getByText('H0')).toBeTruthy();
      expect(screen.getByText('DCC')).toBeTruthy();
    });

    it('should display era badge', () => {
      render(RailwayModelPreviewCard, {
        props: { model: mockModel }
      });

      expect(screen.getByText(/VI/)).toBeTruthy();
    });

    it('should display purchase date badge', () => {
      render(RailwayModelPreviewCard, {
        props: { model: mockModel }
      });

      expect(screen.getByText(/PURCHASED/)).toBeTruthy();
      expect(screen.getByText(/2024-06-15/)).toBeTruthy();
    });

    it('should display unit count badge when > 1', () => {
      const { container } = render(RailwayModelPreviewCard, {
        props: { model: { ...mockModel, unitCount: 3 } }
      });

      expect(screen.getByText(/×3/)).toBeTruthy();
      // Check badge is positioned absolutely in bottom-right
      const badge = container.querySelector('.absolute.bottom-2.right-2');
      expect(badge).toBeTruthy();
    });

    it('should not display unit count badge when unitCount is 1', () => {
      render(RailwayModelPreviewCard, {
        props: { model: { ...mockModel, unitCount: 1 } }
      });

      expect(screen.queryByText(/×1/)).toBeFalsy();
    });

    it('should display digital feature icons in top-left corner', () => {
      const { container } = render(RailwayModelPreviewCard, {
        props: { model: { ...mockModel, digitalFeatures: ['Sound', 'DCC'] } }
      });

      // Check overlay is positioned in top-left
      const overlay = container.querySelector('.absolute.top-2.left-2');
      expect(overlay).toBeTruthy();
    });

    it('should stack multiple digital features horizontally', () => {
      const { container } = render(RailwayModelPreviewCard, {
        props: { model: { ...mockModel, digitalFeatures: ['Sound', 'DCC', 'Light'] } }
      });

      // Check flexbox layout with gap
      const overlay = container.querySelector('.flex.gap-1');
      expect(overlay).toBeTruthy();
    });

    it('should omit badges when data is missing', () => {
      render(RailwayModelPreviewCard, {
        props: {
          model: {
            ...mockModel,
            scale: null,
            powerMethod: null,
            era: null,
            purchaseDate: null
          }
        }
      });

      // Should not display missing badges
      expect(screen.queryByText('H0')).toBeFalsy();
      expect(screen.queryByText('DCC')).toBeFalsy();
      expect(screen.queryByText(/VI/)).toBeFalsy();
      expect(screen.queryByText(/PURCHASED/)).toBeFalsy();
    });
  });

  describe('User Story 4: Delete Functionality', () => {
    it('should render delete button when onDelete prop provided', () => {
      const onDelete = vi.fn();
      render(RailwayModelPreviewCard, {
        props: { model: mockModel, onDelete }
      });

      const deleteButton = screen.getByLabelText(/delete/i);
      expect(deleteButton).toBeTruthy();
    });

    it('should hide delete button when onDelete prop omitted', () => {
      render(RailwayModelPreviewCard, {
        props: { model: mockModel }
      });

      const deleteButton = screen.queryByLabelText(/delete/i);
      expect(deleteButton).toBeFalsy();
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
    it('should display steam locomotive placeholder icon', () => {
      const { container } = render(RailwayModelPreviewCard, {
        props: {
          model: { ...mockModel, category: 'SteamLocomotive', photoUrl: null }
        }
      });

      // Should display icon (SVG) for steam locomotives
      const icon = container.querySelector('svg.lucide-icon');
      expect(icon).toBeTruthy();
    });

    it('should display electric locomotive placeholder icon', () => {
      const { container } = render(RailwayModelPreviewCard, {
        props: {
          model: { ...mockModel, category: 'ElectricLocomotive', photoUrl: null }
        }
      });

      // Should display icon (SVG) for electric locomotives
      const icon = container.querySelector('svg.lucide-icon');
      expect(icon).toBeTruthy();
    });

    it('should display wagon/freight car placeholder icon', () => {
      const { container } = render(RailwayModelPreviewCard, {
        props: {
          model: { ...mockModel, category: 'Wagon', photoUrl: null }
        }
      });

      // Should display icon (SVG) for wagons
      const icon = container.querySelector('svg.lucide-icon');
      expect(icon).toBeTruthy();
    });

    it('should display generic train placeholder when category unknown', () => {
      const { container } = render(RailwayModelPreviewCard, {
        props: {
          model: { ...mockModel, category: 'Unknown', photoUrl: null }
        }
      });

      // Should display generic icon (SVG)
      const icon = container.querySelector('svg.lucide-icon');
      expect(icon).toBeTruthy();
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
    it('should have ARIA label on delete button', () => {
      const onDelete = vi.fn();
      render(RailwayModelPreviewCard, {
        props: { model: mockModel, onDelete }
      });

      const deleteButton = screen.getByLabelText(/delete model/i);
      expect(deleteButton).toBeTruthy();
    });

    it('should have semantic HTML structure', () => {
      const { container } = render(RailwayModelPreviewCard, {
        props: { model: mockModel }
      });

      // Should have proper heading
      const heading = container.querySelector('h3');
      expect(heading).toBeTruthy();
      expect(heading?.textContent).toContain('Class 66');
    });

    it('should have lazy loading on images', () => {
      const { container } = render(RailwayModelPreviewCard, {
        props: { model: { ...mockModel, photoUrl: 'https://example.com/photo.jpg' } }
      });

      const img = container.querySelector('img');
      expect(img?.getAttribute('loading')).toBe('lazy');
      expect(img?.getAttribute('decoding')).toBe('async');
    });

    it('should have ARIA labels on overlays', () => {
      render(RailwayModelPreviewCard, {
        props: {
          model: {
            ...mockModel,
            unitCount: 3,
            digitalFeatures: ['Sound', 'DCC']
          }
        }
      });

      // Digital features overlay
      const digitalOverlay = screen.getByLabelText(/digital features/i);
      expect(digitalOverlay).toBeTruthy();

      // Unit count overlay
      const unitOverlay = screen.getByLabelText(/3 units/i);
      expect(unitOverlay).toBeTruthy();
    });
  });

  describe('Responsive Design', () => {
    it('should use responsive grid classes', () => {
      const { container } = render(RailwayModelPreviewCard, {
        props: { model: mockModel }
      });

      const grid = container.querySelector('.grid');
      expect(grid?.classList.contains('sm:grid-cols-1')).toBe(true);
      expect(grid?.classList.contains('lg:grid-cols-[auto_1fr]')).toBe(true);
    });

    it('should have hover transition classes on card', () => {
      const { container } = render(RailwayModelPreviewCard, {
        props: { model: mockModel }
      });

      const card = container.querySelector('.card');
      expect(card?.classList.contains('hover:scale-[1.02]')).toBe(true);
      expect(card?.classList.contains('hover:ring-primary-500')).toBe(true);
      expect(card?.classList.contains('transition-all')).toBe(true);
    });
  });
});
