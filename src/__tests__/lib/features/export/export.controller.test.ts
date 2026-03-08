import { describe, it, expect, beforeEach } from 'vitest';

// Mock svelte context functions
import { vi } from 'vitest';

vi.mock('svelte', async () => {
  const actual = await vi.importActual('svelte');
  return {
    ...actual,
    getContext: vi.fn(),
    setContext: vi.fn()
  };
});

// Import after mocks are set up
import { ExportController, createExportController } from '$lib/features/export/export.controller.svelte';

describe('ExportController', () => {
  let controller: ExportController;

  beforeEach(() => {
    controller = new ExportController();
  });

  describe('initial state', () => {
    it('should initialize with dialog closed', () => {
      expect(controller.isOpen).toBe(false);
    });

    it('should initialize with isLoading as false', () => {
      expect(controller.isLoading).toBe(false);
    });

    it('should initialize with error as null', () => {
      expect(controller.error).toBeNull();
    });

    it('should initialize with progress as 0', () => {
      expect(controller.progress).toBe(0);
    });

    it('should initialize with currentPhase as null', () => {
      expect(controller.currentPhase).toBeNull();
    });

    it('should initialize with default entity selection', () => {
      expect(controller.entitySelection).toEqual({
        include_railway_models: true,
        include_collection_items: true,
        include_sellers: true,
        include_maintenance_logs: true,
        include_dcc_roster: true,
        include_orphaned_images: false
      });
    });

    it('should initialize with preview as null', () => {
      expect(controller.preview).toBeNull();
    });

    it('should initialize with selectedPath as null', () => {
      expect(controller.selectedPath).toBeNull();
    });

    it('should initialize with result as null', () => {
      expect(controller.result).toBeNull();
    });
  });

  describe('openDialog', () => {
    it('should open the dialog', () => {
      controller.isOpen = false;
      controller.openDialog();

      expect(controller.isOpen).toBe(true);
    });

    it('should clear error when opening', () => {
      controller.error = 'Previous error';
      controller.openDialog();

      expect(controller.error).toBeNull();
    });

    it('should clear preview when opening', () => {
      controller.preview = {
        estimated_size_mb: 100,
        record_counts: {
          railway_models: 50,
          collection_items: 100,
          sellers: 10,
          maintenance_logs: 5,
          dcc_roster_entries: 0
        }
      };
      controller.openDialog();

      expect(controller.preview).toBeNull();
    });

    it('should clear selectedPath when opening', () => {
      controller.selectedPath = '/path/to/export.zip';
      controller.openDialog();

      expect(controller.selectedPath).toBeNull();
    });

    it('should clear result when opening', () => {
      controller.result = {
        filename: 'export.zip',
        size_bytes: 5000000,
        exported_at: '2025-03-08T12:00:00Z'
      };
      controller.openDialog();

      expect(controller.result).toBeNull();
    });

    it('should open dialog multiple times', () => {
      controller.openDialog();
      expect(controller.isOpen).toBe(true);

      controller.closeDialog();
      expect(controller.isOpen).toBe(false);

      controller.openDialog();
      expect(controller.isOpen).toBe(true);
    });
  });

  describe('closeDialog', () => {
    it('should close the dialog', () => {
      controller.isOpen = true;
      controller.closeDialog();

      expect(controller.isOpen).toBe(false);
    });

    it('should close dialog when already closed', () => {
      controller.isOpen = false;
      controller.closeDialog();

      expect(controller.isOpen).toBe(false);
    });

    it('should not clear other state when closing', () => {
      controller.error = 'Some error';
      controller.closeDialog();

      expect(controller.error).toBe('Some error');
    });
  });

  describe('resetState', () => {
    it('should reset loading state', () => {
      controller.isLoading = true;
      controller.resetState();

      expect(controller.isLoading).toBe(false);
    });

    it('should clear error', () => {
      controller.error = 'Export error occurred';
      controller.resetState();

      expect(controller.error).toBeNull();
    });

    it('should reset progress to 0', () => {
      controller.progress = 75;
      controller.resetState();

      expect(controller.progress).toBe(0);
    });

    it('should clear current phase', () => {
      controller.currentPhase = 'compressing';
      controller.resetState();

      expect(controller.currentPhase).toBeNull();
    });

    it('should reset all state fields together', () => {
      controller.isLoading = true;
      controller.error = 'Test error';
      controller.progress = 50;
      controller.currentPhase = 'collecting';

      controller.resetState();

      expect(controller.isLoading).toBe(false);
      expect(controller.error).toBeNull();
      expect(controller.progress).toBe(0);
      expect(controller.currentPhase).toBeNull();
    });

    it('should not affect dialog state', () => {
      controller.isOpen = true;
      controller.resetState();

      expect(controller.isOpen).toBe(true);
    });
  });

  describe('setError', () => {
    it('should set an error message', () => {
      controller.setError('Failed to create export');

      expect(controller.error).toBe('Failed to create export');
    });

    it('should overwrite previous error', () => {
      controller.setError('First error');
      controller.setError('Second error');

      expect(controller.error).toBe('Second error');
    });

    it('should accept various error messages', () => {
      const errors = [
        'File not found',
        'Permission denied',
        'Disk space insufficient',
        'Database error',
        'Network error'
      ];

      for (const errorMsg of errors) {
        controller.setError(errorMsg);
        expect(controller.error).toBe(errorMsg);
      }
    });

    it('should handle empty string error', () => {
      controller.setError('');

      expect(controller.error).toBe('');
    });
  });

  describe('updateProgress', () => {
    it('should update progress percentage and phase', () => {
      controller.updateProgress(25, 'collecting');

      expect(controller.progress).toBe(25);
      expect(controller.currentPhase).toBe('collecting');
    });

    it('should handle collecting phase', () => {
      controller.updateProgress(10, 'collecting');

      expect(controller.progress).toBe(10);
      expect(controller.currentPhase).toBe('collecting');
    });

    it('should handle compressing phase', () => {
      controller.updateProgress(50, 'compressing');

      expect(controller.progress).toBe(50);
      expect(controller.currentPhase).toBe('compressing');
    });

    it('should handle finalizing phase', () => {
      controller.updateProgress(90, 'finalizing');

      expect(controller.progress).toBe(90);
      expect(controller.currentPhase).toBe('finalizing');
    });

    it('should update progress to 100', () => {
      controller.updateProgress(100, 'finalizing');

      expect(controller.progress).toBe(100);
      expect(controller.currentPhase).toBe('finalizing');
    });

    it('should support progress workflow', () => {
      controller.updateProgress(25, 'collecting');
      expect(controller.progress).toBe(25);

      controller.updateProgress(60, 'compressing');
      expect(controller.progress).toBe(60);

      controller.updateProgress(100, 'finalizing');
      expect(controller.progress).toBe(100);
    });

    it('should allow progress to decrease', () => {
      controller.updateProgress(100, 'finalizing');
      controller.updateProgress(50, 'compressing');

      expect(controller.progress).toBe(50);
      expect(controller.currentPhase).toBe('compressing');
    });
  });

  describe('entity selection', () => {
    it('should allow modifying entity selection', () => {
      controller.entitySelection.include_railway_models = false;

      expect(controller.entitySelection.include_railway_models).toBe(false);
    });

    it('should allow toggling multiple selections', () => {
      const original = { ...controller.entitySelection };

      controller.entitySelection.include_railway_models = false;
      controller.entitySelection.include_collection_items = false;
      controller.entitySelection.include_sellers = false;

      expect(controller.entitySelection.include_railway_models).toBe(false);
      expect(controller.entitySelection.include_collection_items).toBe(false);
      expect(controller.entitySelection.include_sellers).toBe(false);
      expect(controller.entitySelection.include_maintenance_logs).toBe(original.include_maintenance_logs);
    });

    it('should start with orphaned images disabled', () => {
      expect(controller.entitySelection.include_orphaned_images).toBe(false);
    });

    it('should allow enabling orphaned images', () => {
      controller.entitySelection.include_orphaned_images = true;

      expect(controller.entitySelection.include_orphaned_images).toBe(true);
    });
  });

  describe('data properties', () => {
    it('should allow setting preview', () => {
      const mockPreview = {
        estimated_size_mb: 250,
        record_counts: {
          railway_models: 100,
          collection_items: 500,
          sellers: 20,
          maintenance_logs: 1000,
          dcc_roster_entries: 50
        }
      };

      controller.preview = mockPreview;

      expect(controller.preview).toEqual(mockPreview);
    });

    it('should allow setting selectedPath', () => {
      controller.selectedPath = '/home/user/exports/backup.zip';

      expect(controller.selectedPath).toBe('/home/user/exports/backup.zip');
    });

    it('should allow setting result', () => {
      const mockResult = {
        filename: 'export_2025-03-08.zip',
        size_bytes: 25000000,
        exported_at: '2025-03-08T14:30:00Z'
      };

      controller.result = mockResult;

      expect(controller.result).toEqual(mockResult);
    });
  });

  describe('createExportController factory', () => {
    it('should create a new ExportController instance', () => {
      const controller1 = createExportController();
      const controller2 = createExportController();

      expect(controller1).not.toBe(controller2);
      expect(controller1).toBeInstanceOf(ExportController);
      expect(controller2).toBeInstanceOf(ExportController);
    });

    it('should create independent instances', () => {
      const controller1 = createExportController();
      const controller2 = createExportController();

      controller1.isOpen = true;
      controller1.error = 'Error in controller 1';

      expect(controller2.isOpen).toBe(false);
      expect(controller2.error).toBeNull();
    });
  });

  describe('workflow scenarios', () => {
    it('should handle complete export workflow', () => {
      // Open dialog
      controller.openDialog();
      expect(controller.isOpen).toBe(true);

      // Set preview
      controller.preview = {
        estimated_size_mb: 100,
        record_counts: {
          railway_models: 50,
          collection_items: 100,
          sellers: 10,
          maintenance_logs: 5,
          dcc_roster_entries: 0
        }
      };

      // Update entity selection
      controller.entitySelection.include_orphaned_images = true;

      // Start export
      controller.isLoading = true;
      controller.updateProgress(25, 'collecting');

      expect(controller.isOpen).toBe(true);
      expect(controller.isLoading).toBe(true);
      expect(controller.progress).toBe(25);

      // Progress through phases
      controller.updateProgress(60, 'compressing');
      expect(controller.progress).toBe(60);

      controller.updateProgress(100, 'finalizing');
      expect(controller.progress).toBe(100);

      // Complete
      controller.isLoading = false;
      controller.result = {
        filename: 'export.zip',
        size_bytes: 5000000,
        exported_at: '2025-03-08T12:00:00Z'
      };

      expect(controller.isLoading).toBe(false);
      expect(controller.result).not.toBeNull();
    });

    it('should handle export error scenario', () => {
      controller.openDialog();
      controller.isLoading = true;
      controller.updateProgress(50, 'compressing');

      // Simulate error
      controller.setError('Failed to write file: Permission denied');
      controller.isLoading = false;

      expect(controller.error).toBe('Failed to write file: Permission denied');
      expect(controller.isLoading).toBe(false);

      // Reset and retry
      controller.resetState();
      expect(controller.error).toBeNull();
      expect(controller.progress).toBe(0);
      expect(controller.isLoading).toBe(false);
    });

    it('should handle user canceling export', () => {
      controller.openDialog();
      controller.isLoading = true;
      controller.updateProgress(30, 'collecting');

      // User closes dialog
      controller.closeDialog();
      controller.resetState();

      expect(controller.isOpen).toBe(false);
      expect(controller.isLoading).toBe(false);
      expect(controller.progress).toBe(0);
      expect(controller.currentPhase).toBeNull();
    });
  });
});
