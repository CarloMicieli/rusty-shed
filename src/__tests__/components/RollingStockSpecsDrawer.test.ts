import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, fireEvent, waitFor } from '@testing-library/svelte';
import RollingStockSpecsDrawer from '$lib/features/rolling-stock-edit/components/RollingStockSpecsDrawer.svelte';
import type { RailwayModelId, RollingStockId } from '$lib/bindings';

// ── Mock $lib/bindings ───────────────────────────────────────────────────────
vi.mock('$lib/bindings', () => ({
  commands: {
    getRailwayModelById: vi.fn(),
    getRailwayCompanies: vi.fn().mockResolvedValue({ status: 'ok', data: [] }),
    updateRollingStockSpecifications: vi.fn(),
    updateRollingStockRailwayCompany: vi.fn()
  }
}));

// ── Mock Paraglide messages ──────────────────────────────────────────────────
// NOTE: vi.mock is hoisted so factories must be fully self-contained.
vi.mock('$lib/paraglide/messages', () => ({
  specs_drawer_title: () => 'Edit Specifications',
  specs_drawer_cancel: () => 'Cancel',
  specs_drawer_save: () => 'Save',
  specs_drawer_save_error: () => 'Failed to save. Please try again.',
  specs_drawer_save_success: () => 'Saved successfully.',
  specs_drawer_unsaved_title: () => 'Unsaved Changes',
  specs_drawer_unsaved_message: () => 'You have unsaved changes. Discard them?',
  specs_drawer_unsaved_confirm: () => 'Discard',
  drawer_discard_title: () => 'Unsaved Changes',
  drawer_discard_description: () => 'You have unsaved changes. Discard them?',
  drawer_discard_confirm: () => 'Discard',
  drawer_discard_cancel: () => 'Cancel',
  specs_drawer_section_identification: () => 'Identification',
  specs_drawer_section_technical: () => 'Technical',
  specs_drawer_section_control: () => 'Control',
  specs_drawer_section_coupling: () => 'Coupling',
  rolling_stock_field_series_code: () => 'Series Code',
  rolling_stock_field_series: () => 'Series',
  rolling_stock_field_road_number: () => 'Road Number',
  rolling_stock_field_friendly_name: () => 'Friendly Name',
  rolling_stock_field_livery: () => 'Livery',
  rolling_stock_field_depot: () => 'Depot',
  rolling_stock_placeholder_series_code: () => 'e.g. Re 4/4',
  rolling_stock_placeholder_series: () => 'e.g. Re 4/4 II, Class 37',
  rolling_stock_placeholder_road_number: () => 'e.g. 218 101-3',
  rolling_stock_placeholder_friendly_name: () => 'e.g. Highland Chief',
  rolling_stock_placeholder_livery: () => 'e.g. Swiss Federal',
  rolling_stock_placeholder_depot: () => 'e.g. Basel',
  model_rolling_stock_field_company: () => 'Company',
  rolling_stock_select_company: () => '— Select company —',
  specs_drawer_field_flywheel: () => 'Flywheel Fitted',
  specs_drawer_field_body_material: () => 'Body Material',
  specs_drawer_field_chassis_material: () => 'Chassis Material',
  specs_drawer_field_lighting: () => 'Lighting',
  specs_drawer_field_control_type: () => 'Control Type',
  specs_drawer_field_dcc_interface: () => 'DCC Interface',
  specs_drawer_field_coupling_socket: () => 'Coupling Socket',
  specs_drawer_field_close_coupling: () => 'Close Couplers',
  specs_drawer_field_digital_shunting: () => 'Digital Shunting',
  boolean_yes: () => 'Yes',
  boolean_no: () => 'No'
}));
vi.mock('$lib/paraglide/messages.js', () => ({
  specs_drawer_title: () => 'Edit Specifications',
  specs_drawer_cancel: () => 'Cancel',
  specs_drawer_save: () => 'Save',
  specs_drawer_save_error: () => 'Failed to save. Please try again.',
  specs_drawer_save_success: () => 'Saved successfully.',
  specs_drawer_unsaved_title: () => 'Unsaved Changes',
  specs_drawer_unsaved_message: () => 'You have unsaved changes. Discard them?',
  specs_drawer_unsaved_confirm: () => 'Discard',
  drawer_discard_title: () => 'Unsaved Changes',
  drawer_discard_description: () => 'You have unsaved changes. Discard them?',
  drawer_discard_confirm: () => 'Discard',
  drawer_discard_cancel: () => 'Cancel',
  specs_drawer_section_identification: () => 'Identification',
  specs_drawer_section_technical: () => 'Technical',
  specs_drawer_section_control: () => 'Control',
  specs_drawer_section_coupling: () => 'Coupling',
  rolling_stock_field_series_code: () => 'Series Code',
  rolling_stock_field_series: () => 'Series',
  rolling_stock_field_road_number: () => 'Road Number',
  rolling_stock_field_friendly_name: () => 'Friendly Name',
  rolling_stock_field_livery: () => 'Livery',
  rolling_stock_field_depot: () => 'Depot',
  rolling_stock_placeholder_series_code: () => 'e.g. Re 4/4',
  rolling_stock_placeholder_series: () => 'e.g. Re 4/4 II, Class 37',
  rolling_stock_placeholder_road_number: () => 'e.g. 218 101-3',
  rolling_stock_placeholder_friendly_name: () => 'e.g. Highland Chief',
  rolling_stock_placeholder_livery: () => 'e.g. Swiss Federal',
  rolling_stock_placeholder_depot: () => 'e.g. Basel',
  model_rolling_stock_field_company: () => 'Company',
  rolling_stock_select_company: () => '— Select company —',
  specs_drawer_field_flywheel: () => 'Flywheel Fitted',
  specs_drawer_field_body_material: () => 'Body Material',
  specs_drawer_field_chassis_material: () => 'Chassis Material',
  specs_drawer_field_lighting: () => 'Lighting',
  specs_drawer_field_control_type: () => 'Control Type',
  specs_drawer_field_dcc_interface: () => 'DCC Interface',
  specs_drawer_field_coupling_socket: () => 'Coupling Socket',
  specs_drawer_field_close_coupling: () => 'Close Couplers',
  specs_drawer_field_digital_shunting: () => 'Digital Shunting',
  boolean_yes: () => 'Yes',
  boolean_no: () => 'No'
}));

vi.mock('$lib/toaster', () => ({
  toaster: { error: vi.fn(), success: vi.fn() }
}));

// ── Import mocked commands AFTER vi.mock declarations ────────────────────────
import { commands } from '$lib/bindings';

// ── Test fixtures ─────────────────────────────────────────────────────────────
const RAILWAY_MODEL_ID = 'trn:railway-model:rivarossi:hr2906' as RailwayModelId;
const ROLLING_STOCK_ID = 'trn:rolling-stock:rs-001' as RollingStockId;

/** A fully-populated locomotive fixture for the drawer */
const mockLocomotive = {
  id: ROLLING_STOCK_ID,
  railway: { railwayCompanyId: 'trn:railway-company:fs', display: 'FS' },
  livery: 'Verde FS',
  length_over_buffer: null,
  technical_specifications: {
    minimum_radius: null,
    flywheel_fitted: 'YES',
    body_shell: 'METAL_DIE_CAST',
    chassis: 'PLASTIC',
    interior_lights: 'NO',
    lights: 'YES',
    sprung_buffers: null,
    coupling: {
      socket: 'NEM_362',
      close_couplers: null,
      digital_shunting: null
    }
  },
  friendly_name: null,
  series_code: 'E.656',
  road_number: '656 001',
  series: 'Serie I',
  depot: 'Milano Centrale',
  locomotive_type: 'ELECTRIC',
  dcc_interface: 'NEXT_18',
  control: 'DCC_FITTED',
  is_dummy: false
};

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
  rollingStock: [{ locomotive: mockLocomotive }]
};

// ── Setup ─────────────────────────────────────────────────────────────────────
beforeEach(() => {
  vi.mocked(commands.getRailwayModelById).mockResolvedValue({
    status: 'ok',
    data: mockRailwayModelView as never
  });
  vi.mocked(commands.updateRollingStockSpecifications).mockResolvedValue({
    status: 'ok',
    data: null
  });
});

// ── Tests ─────────────────────────────────────────────────────────────────────
describe('RollingStockSpecsDrawer', () => {
  describe('Field population (FR-022–FR-026)', () => {
    it('populates the series code field from getRailwayModelById response when opened', async () => {
      const { container } = render(RollingStockSpecsDrawer, {
        props: {
          open: true,
          railwayModelId: RAILWAY_MODEL_ID,
          rollingStockId: ROLLING_STOCK_ID,
          onClose: vi.fn()
        }
      });

      // Wait for loadData to complete
      await waitFor(() => {
        const input = container.querySelector('#drawer-series-code') as HTMLInputElement;
        expect(input?.value).toBe('E.656');
      });
    });

    it('populates road number, livery, and depot fields from the mocked response', async () => {
      const { container } = render(RollingStockSpecsDrawer, {
        props: {
          open: true,
          railwayModelId: RAILWAY_MODEL_ID,
          rollingStockId: ROLLING_STOCK_ID,
          onClose: vi.fn()
        }
      });

      await waitFor(() => {
        const roadNumber = container.querySelector('#drawer-road-number') as HTMLInputElement;
        const livery = container.querySelector('#drawer-livery') as HTMLInputElement;
        const depot = container.querySelector('#drawer-depot') as HTMLInputElement;
        expect(roadNumber?.value).toBe('656 001');
        expect(livery?.value).toBe('Verde FS');
        expect(depot?.value).toBe('Milano Centrale');
      });
    });

    it('populates technical specification selects from the mocked response', async () => {
      const { container } = render(RollingStockSpecsDrawer, {
        props: {
          open: true,
          railwayModelId: RAILWAY_MODEL_ID,
          rollingStockId: ROLLING_STOCK_ID,
          onClose: vi.fn()
        }
      });

      // FormSelect renders as bits-ui Select.Trigger (button), not a native <select>.
      // Verify the trigger shows the correct label text for each populated field.
      await waitFor(() => {
        const flywheel = container.querySelector('#drawer-flywheel') as HTMLButtonElement;
        const bodyShell = container.querySelector('#drawer-body-shell') as HTMLButtonElement;
        const couplingSocket = container.querySelector(
          '#drawer-coupling-socket'
        ) as HTMLButtonElement;
        expect(flywheel?.textContent?.trim()).toBe('Yes'); // 'YES' → true → label 'Yes'
        expect(bodyShell?.textContent?.trim()).toBe('Metal die-cast'); // 'METAL_DIE_CAST' → label
        expect(couplingSocket?.textContent?.trim()).toBe('NEM 362'); // 'NEM_362' → label
      });
    });

    it('renders all four sections (Identification, Technical, Control, Coupling)', async () => {
      const { container } = render(RollingStockSpecsDrawer, {
        props: {
          open: true,
          railwayModelId: RAILWAY_MODEL_ID,
          rollingStockId: ROLLING_STOCK_ID,
          onClose: vi.fn()
        }
      });

      await waitFor(() => {
        expect(container.textContent).toContain('Identification');
        expect(container.textContent).toContain('Technical');
        expect(container.textContent).toContain('Control');
        expect(container.textContent).toContain('Coupling');
      });
    });
  });

  describe('FR-027: Dirty-check confirmation dialog', () => {
    it('shows a confirmation dialog when closing the drawer with unsaved changes', async () => {
      const onClose = vi.fn();
      const { container } = render(RollingStockSpecsDrawer, {
        props: {
          open: true,
          railwayModelId: RAILWAY_MODEL_ID,
          rollingStockId: ROLLING_STOCK_ID,
          onClose
        }
      });

      // Wait for drawer to load
      await waitFor(() => {
        const input = container.querySelector('#drawer-series-code') as HTMLInputElement;
        expect(input).not.toBeNull();
      });

      // Modify a field to make the form dirty
      const seriesInput = container.querySelector('#drawer-series-code') as HTMLInputElement;
      seriesInput.value = 'E.646';
      await fireEvent.input(seriesInput);

      // Click the close (X) button
      const closeBtn = container.querySelector('[aria-label="close"]') as HTMLElement;
      await fireEvent.click(closeBtn);

      // Confirmation dialog should appear
      await waitFor(() => {
        expect(container.textContent).toContain('Unsaved Changes');
        expect(container.textContent).toContain('Discard');
      });

      // onClose should NOT have been called yet
      expect(onClose).not.toHaveBeenCalled();
    });

    it('calls onClose and discards changes when user confirms discard', async () => {
      const onClose = vi.fn();
      const { container } = render(RollingStockSpecsDrawer, {
        props: {
          open: true,
          railwayModelId: RAILWAY_MODEL_ID,
          rollingStockId: ROLLING_STOCK_ID,
          onClose
        }
      });

      // Wait for drawer to load
      await waitFor(() => {
        expect(container.querySelector('#drawer-series-code')).not.toBeNull();
      });

      // Make the form dirty
      const seriesInput = container.querySelector('#drawer-series-code') as HTMLInputElement;
      seriesInput.value = 'E.646';
      await fireEvent.input(seriesInput);

      // Click close to trigger dirty-check
      const closeBtn = container.querySelector('[aria-label="close"]') as HTMLElement;
      await fireEvent.click(closeBtn);

      // Wait for dialog to appear and click "Discard"
      await waitFor(() => {
        expect(container.textContent).toContain('Unsaved Changes');
      });

      const discardBtn = Array.from(container.querySelectorAll('button')).find(
        (b) => b.textContent?.trim() === 'Discard'
      ) as HTMLElement;
      await fireEvent.click(discardBtn);

      expect(onClose).toHaveBeenCalled();
    });

    it('does not close and hides dialog when user cancels the discard', async () => {
      const onClose = vi.fn();
      const { container } = render(RollingStockSpecsDrawer, {
        props: {
          open: true,
          railwayModelId: RAILWAY_MODEL_ID,
          rollingStockId: ROLLING_STOCK_ID,
          onClose
        }
      });

      await waitFor(() => {
        expect(container.querySelector('#drawer-series-code')).not.toBeNull();
      });

      // Make dirty and trigger dirty-check dialog
      const seriesInput = container.querySelector('#drawer-series-code') as HTMLInputElement;
      seriesInput.value = 'E.646';
      await fireEvent.input(seriesInput);

      const closeBtn = container.querySelector('[aria-label="close"]') as HTMLElement;
      await fireEvent.click(closeBtn);

      await waitFor(() => {
        expect(container.textContent).toContain('Unsaved Changes');
      });

      // Click the "Cancel" button in the dialog (not the drawer's Cancel)
      const dialogCancelBtns = Array.from(container.querySelectorAll('button')).filter(
        (b) => b.textContent?.trim() === 'Cancel'
      );
      // The last Cancel button belongs to the confirmation dialog
      const dialogCancelBtn = dialogCancelBtns[dialogCancelBtns.length - 1] as HTMLElement;
      await fireEvent.click(dialogCancelBtn);

      // Dialog should be dismissed
      await waitFor(() => {
        expect(container.textContent).not.toContain('Unsaved Changes');
      });

      // Drawer should still be open (onClose not called)
      expect(onClose).not.toHaveBeenCalled();
    });
  });

  describe('FR-028: Save failure keeps drawer open with values preserved', () => {
    it('shows inline error and keeps drawer open when updateRollingStockSpecifications rejects', async () => {
      vi.mocked(commands.updateRollingStockSpecifications).mockResolvedValue({
        status: 'error',
        error: { kind: 'validation', message: 'series_code required' }
      } as never);

      const onClose = vi.fn();
      const { container } = render(RollingStockSpecsDrawer, {
        props: {
          open: true,
          railwayModelId: RAILWAY_MODEL_ID,
          rollingStockId: ROLLING_STOCK_ID,
          onClose
        }
      });

      // Wait for drawer to load
      await waitFor(() => {
        expect(container.querySelector('#drawer-series-code')).not.toBeNull();
      });

      // Change a value so we have something to save
      const seriesInput = container.querySelector('#drawer-series-code') as HTMLInputElement;
      await fireEvent.input(seriesInput, { target: { value: 'E.646' } });

      // Click the Save button
      const saveBtn = Array.from(container.querySelectorAll('button')).find(
        (b) => b.textContent?.includes('Save') && !b.disabled
      ) as HTMLElement;
      await fireEvent.click(saveBtn);

      // Wait for save to complete (error case)
      await waitFor(() => {
        expect(commands.updateRollingStockSpecifications).toHaveBeenCalled();
      });

      // Inline error should be shown
      await waitFor(() => {
        expect(container.textContent).toContain('Failed to save. Please try again.');
      });

      // Drawer should still be open (onClose not called)
      expect(onClose).not.toHaveBeenCalled();

      // Entered values should be preserved in the form
      const stillVisible = container.querySelector('#drawer-series-code') as HTMLInputElement;
      expect(stillVisible?.value).toBe('E.646');
    });

    it('does not close the drawer when save fails — onClose is NOT called', async () => {
      vi.mocked(commands.updateRollingStockSpecifications).mockResolvedValue({
        status: 'error',
        error: { kind: 'database', message: 'connection lost' }
      } as never);

      const onClose = vi.fn();
      const { container } = render(RollingStockSpecsDrawer, {
        props: {
          open: true,
          railwayModelId: RAILWAY_MODEL_ID,
          rollingStockId: ROLLING_STOCK_ID,
          onClose
        }
      });

      await waitFor(() => {
        expect(container.querySelector('#drawer-series-code')).not.toBeNull();
      });

      const saveBtn = Array.from(container.querySelectorAll('button')).find(
        (b) => b.textContent?.includes('Save') && !b.disabled
      ) as HTMLElement;
      await fireEvent.click(saveBtn);

      await waitFor(() => {
        expect(commands.updateRollingStockSpecifications).toHaveBeenCalled();
      });

      expect(onClose).not.toHaveBeenCalled();
    });
  });

  describe('Clean close (no dirty state)', () => {
    it('calls onClose immediately when form is clean and close is triggered', async () => {
      const onClose = vi.fn();
      const { container } = render(RollingStockSpecsDrawer, {
        props: {
          open: true,
          railwayModelId: RAILWAY_MODEL_ID,
          rollingStockId: ROLLING_STOCK_ID,
          onClose
        }
      });

      await waitFor(() => {
        expect(container.querySelector('#drawer-series-code')).not.toBeNull();
      });

      // Click the close button without making any changes
      const closeBtn = container.querySelector('[aria-label="close"]') as HTMLElement;
      await fireEvent.click(closeBtn);

      // onClose should be called immediately (no dirty-check dialog)
      expect(onClose).toHaveBeenCalled();
      expect(container.textContent).not.toContain('Unsaved Changes');
    });
  });

  describe('Closed state', () => {
    it('renders nothing when open=false', () => {
      const { container } = render(RollingStockSpecsDrawer, {
        props: {
          open: false,
          railwayModelId: RAILWAY_MODEL_ID,
          rollingStockId: ROLLING_STOCK_ID,
          onClose: vi.fn()
        }
      });

      expect(container.querySelector('[role="dialog"]')).toBeNull();
      expect(commands.getRailwayModelById).not.toHaveBeenCalled();
    });
  });
});
