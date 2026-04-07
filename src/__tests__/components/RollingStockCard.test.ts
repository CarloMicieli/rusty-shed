import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, fireEvent, waitFor } from '@testing-library/svelte';
import RollingStockCard from '$lib/components/model-details/RollingStockCard.svelte';
import type { OwnedRollingStockView, RailwayModelId, RollingStockId } from '$lib/bindings';

// ── Mock $lib/bindings ───────────────────────────────────────────────────────
vi.mock('$lib/bindings', () => ({
  commands: {
    getRailwayCompanies: vi.fn(),
    getPrototypes: vi.fn().mockResolvedValue({ status: 'ok', data: [] }),
    updateRollingStockIdentification: vi.fn(),
    updateRollingStockRailwayCompany: vi.fn(),
    updateRollingStockDcc: vi.fn(),
    updateRollingStockCategory: vi.fn(),
    getRailwayModelById: vi.fn(),
    updateRollingStockSpecifications: vi.fn(),
    getCouplerTypes: vi.fn().mockResolvedValue({ status: 'ok', data: [] }),
    setRollingStockCoupler: vi.fn().mockResolvedValue({ status: 'ok', data: null })
  }
}));

// ── Mock settings state ───────────────────────────────────────────────────────
vi.mock('$lib/features/settings/SettingsState.svelte.ts', () => ({
  settingsState: { settings: { measureUnit: 'Metric' } }
}));

// ── Mock paraglide runtime ───────────────────────────────────────────────────
vi.mock('$lib/paraglide/runtime.js', () => ({
  getLocale: () => 'en'
}));

// ── Mock Paraglide messages (both paths used by sub-components) ──────────────
// NOTE: vi.mock is hoisted so we cannot reference top-level variables here.
//       The factory must be fully self-contained — no external const references.
vi.mock('$lib/paraglide/messages', () => ({
  model_rolling_stock_unknown_series: () => 'Unknown',
  model_rolling_stock_na: () => 'N/A',
  model_rolling_stock_field_series: () => 'Series',
  model_rolling_stock_field_road_number: () => 'Road Number',
  model_rolling_stock_field_livery: () => 'Livery',
  model_rolling_stock_field_company: () => 'Railway Company',
  model_rolling_stock_field_control: () => 'Control',
  model_rolling_stock_field_digital_setup: () => 'Digital Setup',
  model_rolling_stock_digital_interface: () => 'Interface',
  model_rolling_stock_digital_address: () => 'Address',
  model_rolling_stock_digital_decoder_id: () => 'Decoder ID',
  rolling_stock_field_series: () => 'Series',
  rolling_stock_field_series_code: () => 'Series Code',
  rolling_stock_field_friendly_name: () => 'Friendly Name',
  rolling_stock_field_road_number: () => 'Road Number',
  rolling_stock_field_livery: () => 'Livery',
  rolling_stock_field_depot: () => 'Depot',
  rolling_stock_placeholder_series_code: () => 'e.g. Re 4/4',
  rolling_stock_placeholder_series: () => 'e.g. Re 4/4 II, Class 37',
  rolling_stock_placeholder_friendly_name: () => 'e.g. Highland Chief',
  rolling_stock_placeholder_road_number: () => 'e.g. 218 101-3',
  rolling_stock_placeholder_livery: () => 'e.g. Swiss Federal',
  rolling_stock_placeholder_depot: () => 'e.g. Basel',
  rolling_stock_field_railway_company: () => 'Railway Company',
  rolling_stock_field_length: () => 'Length',
  rolling_stock_field_dcc_interface: () => 'DCC Interface',
  rolling_stock_field_control_type: () => 'Control Type',
  rolling_stock_field_interior_lights: () => 'Interior Lights',
  rolling_stock_field_lights: () => 'Lights',
  rolling_stock_edit_specs_button: () => 'Edit Specs',
  edit_field_save: () => 'Save',
  edit_field_cancel: () => 'Cancel',
  edit_field_placeholder_empty: () => 'Click to add...',
  edit_save_error: () => 'Failed to save. Your changes are preserved.',
  badge_picker_close: () => 'Close',
  specs_drawer_title: () => 'Edit Specifications',
  specs_drawer_subtitle: () => 'Manage the rolling stock specifications',
  specs_drawer_cancel: () => 'Cancel',
  specs_drawer_save: () => 'Save',
  specs_drawer_save_error: () => 'Failed to save',
  specs_drawer_save_success: () => 'Saved',
  specs_drawer_unsaved_title: () => 'Unsaved Changes',
  specs_drawer_unsaved_message: () => 'You have unsaved changes. Discard them?',
  specs_drawer_unsaved_confirm: () => 'Discard',
  specs_drawer_section_identification: () => 'Identification',
  specs_drawer_section_technical: () => 'Technical',
  specs_drawer_section_control: () => 'Control',
  specs_drawer_section_coupling: () => 'Coupling',
  specs_drawer_field_flywheel: () => 'Flywheel',
  specs_drawer_field_body_material: () => 'Body Material',
  specs_drawer_field_chassis_material: () => 'Chassis Material',
  specs_drawer_field_lighting: () => 'Lighting',
  specs_drawer_field_control_type: () => 'Control Type',
  specs_drawer_field_dcc_interface: () => 'DCC Interface',
  specs_drawer_field_coupling_socket: () => 'Coupling Socket',
  specs_drawer_field_sprung_buffers: () => 'Sprung Buffers',
  specs_drawer_field_close_coupling: () => 'Close Couplers',
  specs_drawer_field_digital_shunting: () => 'Digital Shunting',
  rolling_stock_create_section_prototype: () => 'Prototype',
  rolling_stock_prototype_search_placeholder: () => 'Search prototype library…',
  rolling_stock_prototype_no_results: () => 'No prototypes found',
  rolling_stock_prototype_clear: () => 'Clear prototype',
  rolling_stock_field_category: () => 'Category',
  rolling_stock_select_company: () => '— Select company —'
}));
vi.mock('$lib/paraglide/messages.js', () => ({
  model_rolling_stock_unknown_series: () => 'Unknown',
  model_rolling_stock_na: () => 'N/A',
  model_rolling_stock_field_series: () => 'Series',
  model_rolling_stock_field_road_number: () => 'Road Number',
  model_rolling_stock_field_livery: () => 'Livery',
  model_rolling_stock_field_company: () => 'Railway Company',
  model_rolling_stock_field_control: () => 'Control',
  model_rolling_stock_field_digital_setup: () => 'Digital Setup',
  model_rolling_stock_digital_interface: () => 'Interface',
  model_rolling_stock_digital_address: () => 'Address',
  model_rolling_stock_digital_decoder_id: () => 'Decoder ID',
  rolling_stock_field_series: () => 'Series',
  rolling_stock_field_series_code: () => 'Series Code',
  rolling_stock_field_friendly_name: () => 'Friendly Name',
  rolling_stock_field_road_number: () => 'Road Number',
  rolling_stock_field_livery: () => 'Livery',
  rolling_stock_field_depot: () => 'Depot',
  rolling_stock_placeholder_series_code: () => 'e.g. Re 4/4',
  rolling_stock_placeholder_series: () => 'e.g. Re 4/4 II, Class 37',
  rolling_stock_placeholder_friendly_name: () => 'e.g. Highland Chief',
  rolling_stock_placeholder_road_number: () => 'e.g. 218 101-3',
  rolling_stock_placeholder_livery: () => 'e.g. Swiss Federal',
  rolling_stock_placeholder_depot: () => 'e.g. Basel',
  rolling_stock_field_railway_company: () => 'Railway Company',
  rolling_stock_field_length: () => 'Length',
  rolling_stock_field_dcc_interface: () => 'DCC Interface',
  rolling_stock_field_control_type: () => 'Control Type',
  rolling_stock_field_interior_lights: () => 'Interior Lights',
  rolling_stock_field_lights: () => 'Lights',
  rolling_stock_edit_specs_button: () => 'Edit Specs',
  edit_field_save: () => 'Save',
  edit_field_cancel: () => 'Cancel',
  edit_field_placeholder_empty: () => 'Click to add...',
  edit_save_error: () => 'Failed to save. Your changes are preserved.',
  badge_picker_close: () => 'Close',
  specs_drawer_title: () => 'Edit Specifications',
  specs_drawer_subtitle: () => 'Manage the rolling stock specifications',
  specs_drawer_cancel: () => 'Cancel',
  specs_drawer_save: () => 'Save',
  specs_drawer_save_error: () => 'Failed to save',
  specs_drawer_save_success: () => 'Saved',
  specs_drawer_unsaved_title: () => 'Unsaved Changes',
  specs_drawer_unsaved_message: () => 'You have unsaved changes. Discard them?',
  specs_drawer_unsaved_confirm: () => 'Discard',
  specs_drawer_section_identification: () => 'Identification',
  specs_drawer_section_technical: () => 'Technical',
  specs_drawer_section_control: () => 'Control',
  specs_drawer_section_coupling: () => 'Coupling',
  specs_drawer_field_flywheel: () => 'Flywheel',
  specs_drawer_field_body_material: () => 'Body Material',
  specs_drawer_field_chassis_material: () => 'Chassis Material',
  specs_drawer_field_lighting: () => 'Lighting',
  specs_drawer_field_control_type: () => 'Control Type',
  specs_drawer_field_dcc_interface: () => 'DCC Interface',
  specs_drawer_field_coupling_socket: () => 'Coupling Socket',
  specs_drawer_field_sprung_buffers: () => 'Sprung Buffers',
  specs_drawer_field_close_coupling: () => 'Close Couplers',
  specs_drawer_field_digital_shunting: () => 'Digital Shunting',
  rolling_stock_create_section_prototype: () => 'Prototype',
  rolling_stock_prototype_search_placeholder: () => 'Search prototype library…',
  rolling_stock_prototype_no_results: () => 'No prototypes found',
  rolling_stock_prototype_clear: () => 'Clear prototype',
  rolling_stock_field_category: () => 'Category',
  rolling_stock_select_company: () => '— Select company —'
}));

vi.mock('$lib/toaster', () => ({
  toaster: { error: vi.fn(), success: vi.fn() }
}));

// ── Import mocked commands AFTER vi.mock declarations ────────────────────────
import { commands } from '$lib/bindings';

// ── Test fixtures ────────────────────────────────────────────────────────────
const RAILWAY_MODEL_ID = 'trn:railway-model:rivarossi:hr2906' as RailwayModelId;
const ROLLING_STOCK_ID = 'trn:rolling-stock:rs-001' as RollingStockId;

const mockRollingStock: OwnedRollingStockView = {
  id: 'owned-rs-001',
  rollingStockId: ROLLING_STOCK_ID,
  notes: null,
  series: 'E.656',
  seriesName: null,
  roadNumber: '001',
  livery: 'Verde FS',
  countryCode: 'IT',
  category: 'LOCOMOTIVE',
  subcategory: 'ELECTRIC_LOCOMOTIVE',
  control: null,
  railwayCompanyName: 'FS',
  digital: null,
  depot: null,
  dccInterface: null,
  lengthOverBuffers: null,
  currentCouplerId: null
};

/** Minimal RailwayModelView for the specs drawer fixture */
const mockRailwayModelView = {
  id: RAILWAY_MODEL_ID,
  manufacturer: { id: 'rivarossi', name: 'Rivarossi' },
  productCode: 'HR2906',
  description: 'Test model',
  details: null,
  powerMethod: 'DC',
  scale: 'H0',
  epoch: 'IV',
  category: 'Electric Locomotive',
  deliveryDate: null,
  availabilityStatus: null,
  metadata: { version: 1, created_at: '2024-01-01', updated_at: '2024-01-01' },
  rollingStock: [
    {
      locomotive: {
        id: ROLLING_STOCK_ID,
        railway: { railwayCompanyId: 'trn:railway-company:fs', display: 'FS' },
        livery: 'Verde FS',
        length_over_buffer: null,
        technical_specifications: null,
        friendly_name: null,
        series_code: 'E.656',
        road_number: '001',
        series: 'Serie I',
        depot: null,
        locomotive_type: 'ELECTRIC',
        dcc_interface: null,
        control: null,
        is_dummy: false
      }
    }
  ]
};

// ── Setup ─────────────────────────────────────────────────────────────────────
beforeEach(() => {
  vi.mocked(commands.getRailwayCompanies).mockResolvedValue({ status: 'ok', data: [] });
  vi.mocked(commands.updateRollingStockIdentification).mockResolvedValue({
    status: 'ok',
    data: null
  });
  vi.mocked(commands.updateRollingStockDcc).mockResolvedValue({ status: 'ok', data: null });
  vi.mocked(commands.getRailwayModelById).mockResolvedValue({
    status: 'ok',
    data: mockRailwayModelView as never
  });
  vi.mocked(commands.updateRollingStockSpecifications).mockResolvedValue({
    status: 'ok',
    data: null
  });
});

// ── Helpers ───────────────────────────────────────────────────────────────────
async function renderExpandedCard(editable = true) {
  const result = render(RollingStockCard, {
    props: {
      rollingStock: mockRollingStock,
      railwayModelId: RAILWAY_MODEL_ID,
      editable
    }
  });
  // Click the header button to expand the card
  const headerBtn = result.container.querySelector('button') as HTMLElement;
  await fireEvent.click(headerBtn);
  return result;
}

// ── Tests ─────────────────────────────────────────────────────────────────────
describe('RollingStockCard', () => {
  describe('Rendering', () => {
    it('renders the series and road number in the card header', async () => {
      const { container } = await renderExpandedCard();
      expect(container.textContent).toContain('E.656');
      expect(container.textContent).toContain('001');
    });

    it('shows "Edit Specs" button when editable', async () => {
      const { container } = await renderExpandedCard(true);
      expect(container.textContent).toContain('Edit Specs');
    });

    it('does not show "Edit Specs" button when not editable', async () => {
      const { container } = await renderExpandedCard(false);
      expect(container.textContent).not.toContain('Edit Specs');
    });
  });

  describe('US1: Empty field display shows "—"', () => {
    it('shows "—" for null Livery, Control Type, and Depot; renders all grid labels in read-only mode', async () => {
      const rs = {
        ...mockRollingStock,
        livery: null,
        control: null,
        depot: null,
        railwayCompanyName: null
      };
      const result = render(RollingStockCard, {
        props: { rollingStock: rs, railwayModelId: RAILWAY_MODEL_ID, editable: false }
      });
      const headerBtn = result.container.querySelector('button') as HTMLElement;
      await fireEvent.click(headerBtn);

      // All grid field labels must be present
      expect(result.getByText('Livery')).toBeTruthy();
      expect(result.getByText('Control Type')).toBeTruthy();
      expect(result.getByText('Depot')).toBeTruthy();
      expect(result.container.textContent).toContain('Livery');
      expect(result.container.textContent).toContain('Control Type');
      expect(result.container.textContent).toContain('Depot');

      // The "—" character appears for null fields
      const dashes = Array.from(result.container.querySelectorAll('span')).filter(
        (el) => el.textContent?.trim() === '—'
      );
      expect(dashes.length).toBeGreaterThan(0);
    });
  });

  describe('US2: In-place series code editing', () => {
    it('InPlaceEdit on series code field triggers save callback with correct args', async () => {
      const { container } = await renderExpandedCard();

      // Find and click the series code InPlaceEdit display area.
      // Note: the Category InPlaceSelectEdit also has role="button" and appears first,
      // so we locate the Series edit by its text content ('E.656').
      const inPlaceEdits = Array.from(container.querySelectorAll('button'));
      const seriesEdit = inPlaceEdits.find(
        (el) => el.textContent?.trim() === 'E.656'
      ) as HTMLElement;
      await fireEvent.click(seriesEdit);

      // Type a new value in the input
      const input = container.querySelector('input') as HTMLInputElement;
      await fireEvent.input(input, { target: { value: 'E.646' } });

      // Blur to trigger save
      await fireEvent.blur(input);

      // Wait for async save
      await waitFor(() => {
        expect(commands.updateRollingStockIdentification).toHaveBeenCalled();
      });

      expect(commands.updateRollingStockIdentification).toHaveBeenCalledWith(
        expect.objectContaining({
          railwayModelId: RAILWAY_MODEL_ID,
          rollingStockId: ROLLING_STOCK_ID,
          seriesCode: 'E.646'
        })
      );
    });

    it('keeps editing state and does not update local value when save fails', async () => {
      vi.mocked(commands.updateRollingStockIdentification).mockResolvedValue({
        status: 'error',
        error: { kind: 'validation', message: 'invalid' }
      } as never);

      const { container } = await renderExpandedCard();

      const inPlaceEdits = Array.from(container.querySelectorAll('button'));
      const seriesEdit = inPlaceEdits.find(
        (el) => el.textContent?.trim() === 'E.656'
      ) as HTMLElement;
      await fireEvent.click(seriesEdit);

      const input = container.querySelector('input') as HTMLInputElement;
      await fireEvent.input(input, { target: { value: 'Bad Value' } });
      await fireEvent.blur(input);

      // Wait for async rejection to settle
      await new Promise((resolve) => setTimeout(resolve, 0));

      // Component should remain in error state (input still visible or error shown)
      const errorAlert = container.querySelector('[role="alert"]');
      expect(errorAlert).not.toBeNull();
    });
  });

  describe('FR-013: Card save → drawer shows updated value', () => {
    it('after a successful in-place save, opening the drawer fetches updated data', async () => {
      // Update the mock to return the updated series code
      const updatedModelView = {
        ...mockRailwayModelView,
        rollingStock: [
          {
            locomotive: {
              ...mockRailwayModelView.rollingStock[0].locomotive,
              series_code: 'E.646'
            }
          }
        ]
      };
      vi.mocked(commands.getRailwayModelById).mockResolvedValue({
        status: 'ok',
        data: updatedModelView as never
      });

      const { container } = await renderExpandedCard();

      // Save a new series code in-place.
      // Category InPlaceSelectEdit also uses role="button", so locate Series by text.
      const inPlaceEdits = Array.from(container.querySelectorAll('button'));
      const seriesEdit = inPlaceEdits.find(
        (el) => el.textContent?.trim() === 'E.656'
      ) as HTMLElement;
      await fireEvent.click(seriesEdit);

      const input = container.querySelector('input') as HTMLInputElement;
      await fireEvent.input(input, { target: { value: 'E.646' } });
      await fireEvent.blur(input);

      // Wait for save to complete
      await waitFor(() => {
        expect(commands.updateRollingStockIdentification).toHaveBeenCalled();
      });

      // Open the specs drawer
      const editSpecsBtn = Array.from(container.querySelectorAll('button')).find((b) =>
        b.textContent?.includes('Edit Specs')
      ) as HTMLElement;
      await fireEvent.click(editSpecsBtn);

      // Wait for the drawer to fetch data
      await waitFor(() => {
        expect(commands.getRailwayModelById).toHaveBeenCalledWith(
          RAILWAY_MODEL_ID,
          expect.any(String)
        );
      });

      // Drawer should be visible with the updated series code
      const seriesCodeInput = container.querySelector('#proto-series-code') as HTMLInputElement;
      expect(seriesCodeInput).not.toBeNull();
      expect(seriesCodeInput.value).toBe('E.646');
    });
  });
});
