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

      let sessionIdValue: string | null = null;
      controller.sessionId$.subscribe((id) => {
        sessionIdValue = id;
      });

      let recordCountsValue: RecordCounts | null = null;
      controller.recordCounts$.subscribe((counts) => {
        recordCountsValue = counts;
      });

      let canImportValue = false;
      controller.canImport$.subscribe((value) => {
        canImportValue = value;
      });

      await controller.analyzePackage('/path/to/file.zip');

      expect(sessionIdValue).toBe('session-123');
      expect(recordCountsValue).toEqual(mockRecordCounts);
      expect(canImportValue).toBe(true);
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

      let canImportValue = true;
      controller.canImport$.subscribe((value) => {
        canImportValue = value;
      });

      await controller.analyzePackage('/path/to/file.zip');

      expect(canImportValue).toBe(false);
    });

    it('should handle analysis errors gracefully', async () => {
      (mockCommands.analyzeImportPackage as any).mockResolvedValue({
        status: 'error',
        error: 'Invalid archive format'
      });

      let isLoadingValue = true;
      controller.isLoading$.subscribe((value) => {
        isLoadingValue = value;
      });

      // Mock console.error to prevent test output pollution
      const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

      await controller.analyzePackage('/path/to/file.zip');

      expect(isLoadingValue).toBe(false);
      consoleErrorSpy.mockRestore();
    });

    it('should handle network errors during analysis', async () => {
      mockCommands.analyzeImportPackage.mockRejectedValue(new Error('Network error'));

      const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

      let isLoadingValue = true;
      controller.isLoading$.subscribe((value) => {
        isLoadingValue = value;
      });

      await controller.analyzePackage('/path/to/file.zip');

      expect(isLoadingValue).toBe(false);
      consoleErrorSpy.mockRestore();
    });

    it('should set isLoading to true during analysis', async () => {
      mockCommands.analyzeImportPackage.mockImplementation(
        () =>
          new Promise((resolve) => {
            setTimeout(
              () => resolve({ status: 'ok', data: { sessionId: 'id', recordCounts: {} } }),
              10
            );
          })
      );

      const loadingValues: boolean[] = [];
      controller.isLoading$.subscribe((value) => {
        loadingValues.push(value);
      });

      // Force initial subscription
      const analysisPromise = controller.analyzePackage('/path/to/file.zip');

      // Give time for loading state to be set
      await new Promise((r) => setTimeout(r, 5));

      await analysisPromise;

      // Should have started with false (initial), then true during operation
      expect(loadingValues.includes(true)).toBe(true);
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

      let previewValue: ImportPreviewResponse | null = null;
      controller.preview$.subscribe((preview) => {
        previewValue = preview;
      });

      let errorsValue: ImportValidationError[] = [];
      controller.errors$.subscribe((errors) => {
        errorsValue = errors;
      });

      let warningsValue: ImportWarning[] = [];
      controller.warnings$.subscribe((warnings) => {
        warningsValue = warnings;
      });

      let canImportValue = false;
      controller.canImport$.subscribe((value) => {
        canImportValue = value;
      });

      await controller.getPreview();

      expect(previewValue).toEqual(mockPreview);
      expect(errorsValue).toEqual(mockErrors);
      expect(warningsValue).toEqual(mockWarnings);
      expect(canImportValue).toBe(true);
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

      let previewValue: ImportPreviewResponse | null = {
        sessionId: '',
        errors: [],
        warnings: []
      } as any;
      controller.preview$.subscribe((preview) => {
        previewValue = preview;
      });

      await controller.getPreview();

      expect(previewValue).toBeNull();
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

      let resultValue: ImportResultResponse | null = null;
      controller.result$.subscribe((result) => {
        resultValue = result;
      });

      let canImportValue = true;
      controller.canImport$.subscribe((value) => {
        canImportValue = value;
      });

      await controller.executeImport();

      expect(resultValue).toEqual(mockResult);
      expect(canImportValue).toBe(false);
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

      let isLoadingValue = true;
      controller.isLoading$.subscribe((value) => {
        isLoadingValue = value;
      });

      await controller.executeImport();

      expect(isLoadingValue).toBe(false);
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

      let sessionIdValue: string | null = 'session-333';
      controller.sessionId$.subscribe((id) => {
        sessionIdValue = id;
      });

      let recordCountsValue: RecordCounts | null = { manufacturers: 5 } as any;
      controller.recordCounts$.subscribe((counts) => {
        recordCountsValue = counts;
      });

      await controller.cancelSession();

      expect(mockCommands.cancelImportSession).toHaveBeenCalledWith({ sessionId: 'session-333' });
      expect(sessionIdValue).toBeNull();
      expect(recordCountsValue).toBeNull();
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

      let sessionIdValue: string | null = 'session-444';
      controller.sessionId$.subscribe((id) => {
        sessionIdValue = id;
      });

      await controller.cancelSession();

      expect(sessionIdValue).toBeNull();
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

      let sessionIdValue: string | null = 'session-555';
      let recordCountsValue: RecordCounts | null = { manufacturers: 5 } as any;
      let canImportValue = true;
      let isLoadingValue = true;
      let errorsValue: ImportValidationError[] = [
        { recordId: 'rec-1', message: 'Error', severity: 'error' } as any
      ];

      controller.sessionId$.subscribe((id) => {
        sessionIdValue = id;
      });
      controller.recordCounts$.subscribe((counts) => {
        recordCountsValue = counts;
      });
      controller.canImport$.subscribe((value) => {
        canImportValue = value;
      });
      controller.isLoading$.subscribe((value) => {
        isLoadingValue = value;
      });
      controller.errors$.subscribe((errors) => {
        errorsValue = errors;
      });

      // Reset all state
      controller.reset();

      expect(sessionIdValue).toBeNull();
      expect(recordCountsValue).toBeNull();
      expect(canImportValue).toBe(false);
      expect(isLoadingValue).toBe(false);
      expect(errorsValue).toEqual([]);
    });

    it('should reset warnings and preview', async () => {
      const mockPreview: ImportPreviewResponse = {
        sessionId: 'session-666',
        errors: [],
        warnings: [{ recordId: 'rec-1', message: 'Warning', severity: 'warning' } as any],
        canImport: true,
        summary: { totalRecords: 10, validRecords: 10, invalidRecords: 0 }
      } as any;

      let previewValue: ImportPreviewResponse | null = mockPreview;
      let warningsValue: ImportWarning[] = mockPreview.warnings;

      controller.preview$.subscribe((preview) => {
        previewValue = preview;
      });
      controller.warnings$.subscribe((warnings) => {
        warningsValue = warnings;
      });

      controller.reset();

      expect(previewValue).toBeNull();
      expect(warningsValue).toEqual([]);
    });
  });

  describe('store subscriptions', () => {
    it('should provide readonly store interfaces', () => {
      expect(controller.sessionId$).toHaveProperty('subscribe');
      expect(controller.recordCounts$).toHaveProperty('subscribe');
      expect(controller.errors$).toHaveProperty('subscribe');
      expect(controller.warnings$).toHaveProperty('subscribe');
      expect(controller.progress$).toHaveProperty('subscribe');
      expect(controller.preview$).toHaveProperty('subscribe');
      expect(controller.result$).toHaveProperty('subscribe');
      expect(controller.canImport$).toHaveProperty('subscribe');
      expect(controller.isLoading$).toHaveProperty('subscribe');
    });

    it('should allow multiple subscriptions to same store', () => {
      const values1: (string | null)[] = [];
      const values2: (string | null)[] = [];

      controller.sessionId$.subscribe((id) => values1.push(id));
      controller.sessionId$.subscribe((id) => values2.push(id));

      (mockCommands.analyzeImportPackage as any).mockResolvedValue({
        status: 'ok',
        data: { sessionId: 'session-777', recordCounts: {}, validationStatus: 'valid' }
      });

      controller.analyzePackage('/path/to/file.zip').then(() => {
        expect(values1).toContain('session-777');
        expect(values2).toContain('session-777');
      });
    });
  });
});
