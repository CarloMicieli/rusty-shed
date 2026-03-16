import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import type {
  RecordCounts,
  ImportValidationError,
  ImportWarning,
  ImportPreviewResponse,
  ImportResultResponse
} from '$lib/bindings';
import { ImportController } from '$lib/features/import/import.controller.svelte';

// Mock svelte context functions
vi.mock('svelte', async () => {
  const actual = await vi.importActual('svelte');
  return {
    ...actual,
    getContext: vi.fn() as any,
    setContext: vi.fn() as any
  };
});

// Mock $lib/bindings
const mockCommands = {
  analyzeImportPackage: vi.fn() as any,
  getImportPreview: vi.fn() as any,
  executeImport: vi.fn() as any,
  cancelImportSession: vi.fn() as any
};

vi.mock('$lib/bindings', () => ({
  commands: mockCommands
}));

describe('ImportController', () => {
  let controller: ImportController;

  beforeEach(() => {
    controller = new ImportController();
    vi.clearAllMocks();
  });

  afterEach(() => {
    controller.reset();
  });

  describe('analyzePackage', () => {
    it('should set session ID and record counts on successful analysis', async () => {
      const mockRecordCounts: RecordCounts = {
        manufacturers: 5,
        railway_models: 10,
        rolling_stocks: 25,
        sellers: 3
      } as any;

      (mockCommands.analyzeImportPackage as any).mockResolvedValue({
        status: 'ok',
        data: {
          sessionId: 'session-123',
          recordCounts: mockRecordCounts,
          validationStatus: 'valid'
        }
      });

      await controller.analyzePackage('/path/to/file.zip');

      expect(controller.sessionId).toBe('session-123');
      expect(controller.recordCounts).toEqual(mockRecordCounts);
      expect(controller.canImport).toBe(true);
      expect(mockCommands.analyzeImportPackage).toHaveBeenCalledWith({
        filePath: '/path/to/file.zip'
      });
    });

    it('should set canImport to false when validation status is not valid', async () => {
      (mockCommands.analyzeImportPackage as any).mockResolvedValue({
        status: 'ok',
        data: {
          sessionId: 'session-123',
          recordCounts: {},
          validationStatus: 'invalid'
        }
      });

      await controller.analyzePackage('/path/to/file.zip');

      expect(controller.canImport).toBe(false);
    });

    it('should handle analysis errors gracefully', async () => {
      (mockCommands.analyzeImportPackage as any).mockResolvedValue({
        status: 'error',
        error: 'Invalid archive format'
      });

      const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

      await controller.analyzePackage('/path/to/file.zip');

      expect(controller.isLoading).toBe(false);
      consoleErrorSpy.mockRestore();
    });

    it('should handle network errors during analysis', async () => {
      mockCommands.analyzeImportPackage.mockRejectedValue(new Error('Network error'));

      const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

      await controller.analyzePackage('/path/to/file.zip');

      expect(controller.isLoading).toBe(false);
      consoleErrorSpy.mockRestore();
    });

    it('should set isLoading to true during analysis', async () => {
      let wasLoadingDuringOp = false;

      mockCommands.analyzeImportPackage.mockImplementation(
        () =>
          new Promise((resolve) => {
            setTimeout(
              () => resolve({ status: 'ok', data: { sessionId: 'id', recordCounts: {} } }),
              10
            );
          })
      );

      const analysisPromise = controller.analyzePackage('/path/to/file.zip');

      // Give time for loading state to be set
      await new Promise((r) => setTimeout(r, 5));
      wasLoadingDuringOp = controller.isLoading;

      await analysisPromise;

      expect(wasLoadingDuringOp).toBe(true);
      expect(controller.isLoading).toBe(false);
    });
  });

  describe('getPreview', () => {
    it('should fetch and set preview data when session ID exists', async () => {
      // First set a session ID
      (mockCommands.analyzeImportPackage as any).mockResolvedValue({
        status: 'ok',
        data: {
          sessionId: 'session-456',
          recordCounts: {},
          validationStatus: 'valid'
        }
      });

      await controller.analyzePackage('/path/to/file.zip');

      const mockErrors: ImportValidationError[] = [
        { recordId: 'rec-1', message: 'Invalid data', severity: 'error' } as any
      ];
      const mockWarnings: ImportWarning[] = [
        { recordId: 'rec-2', message: 'Potential duplicate', severity: 'warning' } as any
      ];

      const mockPreview: ImportPreviewResponse = {
        sessionId: 'session-456',
        errors: mockErrors,
        warnings: mockWarnings,
        canImport: true,
        summary: {
          totalRecords: 20,
          validRecords: 19,
          invalidRecords: 1,
          duplicateWarnings: 1
        }
      } as any;

      (mockCommands.getImportPreview as any).mockResolvedValue({
        status: 'ok',
        data: mockPreview
      });

      await controller.getPreview();

      expect(controller.preview).toEqual(mockPreview);
      expect(controller.errors).toEqual(mockErrors);
      expect(controller.warnings).toEqual(mockWarnings);
      expect(controller.canImport).toBe(true);
      expect(mockCommands.getImportPreview).toHaveBeenCalledWith({ sessionId: 'session-456' });
    });

    it('should not fetch preview when no session ID exists', async () => {
      await controller.getPreview();

      expect(mockCommands.getImportPreview).not.toHaveBeenCalled();
    });

    it('should handle preview fetch errors', async () => {
      // Set session ID first
      (mockCommands.analyzeImportPackage as any).mockResolvedValue({
        status: 'ok',
        data: { sessionId: 'session-789', recordCounts: {} as any, validationStatus: 'valid' }
      });
      await controller.analyzePackage('/path/to/file.zip');

      (mockCommands.getImportPreview as any).mockRejectedValue(new Error('Preview failed'));

      const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

      await controller.getPreview();

      expect(controller.preview).toBeNull();
      consoleErrorSpy.mockRestore();
    });
  });

  describe('executeImport', () => {
    it('should execute import and set result when session ID exists', async () => {
      // Set session ID first
      (mockCommands.analyzeImportPackage as any).mockResolvedValue({
        status: 'ok',
        data: { sessionId: 'session-111', recordCounts: {}, validationStatus: 'valid' }
      });
      await controller.analyzePackage('/path/to/file.zip');

      const mockWarnings: ImportWarning[] = [];
      const mockResult: ImportResultResponse = {
        sessionId: 'session-111',
        status: 'completed' as any,
        recordsImported: { manufacturers: 5, railway_models: 10 } as any,
        warnings: mockWarnings,
        importedAt: new Date().toISOString()
      } as any;

      (mockCommands.executeImport as any).mockResolvedValue({
        status: 'ok',
        data: mockResult
      });

      await controller.executeImport();

      expect(controller.result).toEqual(mockResult);
      expect(controller.canImport).toBe(false);
      expect(mockCommands.executeImport).toHaveBeenCalledWith({ sessionId: 'session-111' });
    });

    it('should not execute import when no session ID exists', async () => {
      await controller.executeImport();

      expect(mockCommands.executeImport).not.toHaveBeenCalled();
    });

    it('should handle import execution errors', async () => {
      // Set session ID first
      (mockCommands.analyzeImportPackage as any).mockResolvedValue({
        status: 'ok',
        data: { sessionId: 'session-222', recordCounts: {}, validationStatus: 'valid' }
      });
      await controller.analyzePackage('/path/to/file.zip');

      (mockCommands.executeImport as any).mockRejectedValue(new Error('Import failed'));

      const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

      await controller.executeImport();

      expect(controller.isLoading).toBe(false);
      consoleErrorSpy.mockRestore();
    });
  });

  describe('cancelSession', () => {
    it('should cancel session and reset state when session ID exists', async () => {
      // Set session ID first
      (mockCommands.analyzeImportPackage as any).mockResolvedValue({
        status: 'ok',
        data: {
          sessionId: 'session-333',
          recordCounts: { manufacturers: 5 } as any,
          validationStatus: 'valid'
        }
      });
      await controller.analyzePackage('/path/to/file.zip');

      (mockCommands.cancelImportSession as any).mockResolvedValue(undefined);

      await controller.cancelSession();

      expect(mockCommands.cancelImportSession).toHaveBeenCalledWith({ sessionId: 'session-333' });
      expect(controller.sessionId).toBeNull();
      expect(controller.recordCounts).toBeNull();
    });

    it('should not cancel when no session ID exists', async () => {
      await controller.cancelSession();

      expect(mockCommands.cancelImportSession).not.toHaveBeenCalled();
    });

    it('should reset state even if cancellation fails', async () => {
      // Set session ID first
      (mockCommands.analyzeImportPackage as any).mockResolvedValue({
        status: 'ok',
        data: { sessionId: 'session-444', recordCounts: {}, validationStatus: 'valid' }
      });
      await controller.analyzePackage('/path/to/file.zip');

      (mockCommands.cancelImportSession as any).mockRejectedValue(new Error('Cancel failed'));

      const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

      await controller.cancelSession();

      expect(controller.sessionId).toBeNull();
      consoleErrorSpy.mockRestore();
    });
  });

  describe('reset', () => {
    it('should clear all state when reset is called', async () => {
      // Set some state first
      (mockCommands.analyzeImportPackage as any).mockResolvedValue({
        status: 'ok',
        data: {
          sessionId: 'session-555',
          recordCounts: { manufacturers: 5, railway_models: 10 } as any,
          validationStatus: 'valid'
        }
      });

      await controller.analyzePackage('/path/to/file.zip');

      controller.reset();

      expect(controller.sessionId).toBeNull();
      expect(controller.recordCounts).toBeNull();
      expect(controller.canImport).toBe(false);
      expect(controller.isLoading).toBe(false);
      expect(controller.errors).toEqual([]);
    });

    it('should reset warnings and preview', () => {
      controller.reset();

      expect(controller.preview).toBeNull();
      expect(controller.warnings).toEqual([]);
    });
  });

  describe('state properties', () => {
    it('should expose all state fields as direct properties', () => {
      expect(controller.sessionId).toBeNull();
      expect(controller.recordCounts).toBeNull();
      expect(controller.errors).toEqual([]);
      expect(controller.warnings).toEqual([]);
      expect(controller.progress).toBeNull();
      expect(controller.preview).toBeNull();
      expect(controller.result).toBeNull();
      expect(controller.canImport).toBe(false);
      expect(controller.isLoading).toBe(false);
    });

    it('should reflect state changes after operations', async () => {
      (mockCommands.analyzeImportPackage as any).mockResolvedValue({
        status: 'ok',
        data: { sessionId: 'session-777', recordCounts: {}, validationStatus: 'valid' }
      });

      await controller.analyzePackage('/path/to/file.zip');

      expect(controller.sessionId).toBe('session-777');
      expect(controller.canImport).toBe(true);
    });
  });
});
