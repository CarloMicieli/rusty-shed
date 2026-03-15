import { describe, it, expect, beforeEach, vi } from 'vitest';
import { toaster } from '$lib/toaster';

vi.mock('$lib/toaster', () => ({
  toaster: {
    success: vi.fn(),
    error: vi.fn()
  }
}));

vi.mock('$lib/paraglide/messages.js', () => ({
  export_archive_success: ({ path }: { path: string }) => `Exported to ${path}`,
  export_archive_error: ({ error }: { error: string }) => `Export failed: ${error}`
}));

const { mockSafeInvoke } = vi.hoisted(() => ({
  mockSafeInvoke: vi.fn()
}));

vi.mock('$lib/services', () => ({
  safeInvoke: mockSafeInvoke
}));

// Import after mocks are set up
import {
  ExportController,
  createExportController,
  getExportController
} from '$lib/features/export/export.controller.svelte';

describe('ExportController', () => {
  let controller: ExportController;

  beforeEach(() => {
    controller = new ExportController();
    vi.clearAllMocks();
  });

  describe('initial state', () => {
    it('should initialize with isExporting as false', () => {
      expect(controller.isExporting).toBe(false);
    });

    it('should initialize with error as null', () => {
      expect(controller.error).toBeNull();
    });
  });

  describe('handleExport', () => {
    it('should do nothing when already exporting', async () => {
      controller.isExporting = true;
      await controller.handleExport();
      expect(mockSafeInvoke).not.toHaveBeenCalled();
    });

    it('should not proceed when user cancels the dialog', async () => {
      mockSafeInvoke.mockResolvedValueOnce({ ok: true, data: null });

      await controller.handleExport();

      expect(mockSafeInvoke).toHaveBeenCalledTimes(1);
      expect(toaster.success).not.toHaveBeenCalled();
      expect(controller.isExporting).toBe(false);
    });

    it('should show success toast when export succeeds', async () => {
      mockSafeInvoke
        .mockResolvedValueOnce({ ok: true, data: '/home/user/export.zip' })
        .mockResolvedValueOnce({
          ok: true,
          data: {
            archive_path: '/home/user/export.zip',
            file_size_bytes: BigInt(1024),
            records_exported: 42,
            warnings: []
          }
        });

      await controller.handleExport();

      expect(toaster.success).toHaveBeenCalledWith({
        title: 'Exported to /home/user/export.zip'
      });
      expect(controller.isExporting).toBe(false);
      expect(controller.error).toBeNull();
    });

    it('should call execute_export with the selected destination path', async () => {
      const destinationPath = '/home/user/my-export.zip';
      mockSafeInvoke
        .mockResolvedValueOnce({ ok: true, data: destinationPath })
        .mockResolvedValueOnce({
          ok: true,
          data: {
            archive_path: destinationPath,
            file_size_bytes: BigInt(512),
            records_exported: 10,
            warnings: []
          }
        });

      await controller.handleExport();

      expect(mockSafeInvoke).toHaveBeenNthCalledWith(2, 'execute_export', { destinationPath });
    });

    it('should show error toast when file dialog fails', async () => {
      mockSafeInvoke.mockResolvedValueOnce({
        ok: false,
        error: { message: 'Dialog failed', kind: 'unknown' }
      });

      await controller.handleExport();

      expect(toaster.error).toHaveBeenCalledWith({
        title: 'Export failed: Dialog failed'
      });
      expect(controller.error).toBe('Dialog failed');
      expect(controller.isExporting).toBe(false);
    });

    it('should show error toast when export execution fails', async () => {
      mockSafeInvoke
        .mockResolvedValueOnce({ ok: true, data: '/home/user/export.zip' })
        .mockResolvedValueOnce({
          ok: false,
          error: { message: 'Insufficient disk space', kind: 'unknown' }
        });

      await controller.handleExport();

      expect(toaster.error).toHaveBeenCalledWith({
        title: 'Export failed: Insufficient disk space'
      });
      expect(controller.error).toBe('Insufficient disk space');
      expect(controller.isExporting).toBe(false);
    });

    it('should reset isExporting to false after success', async () => {
      mockSafeInvoke
        .mockResolvedValueOnce({ ok: true, data: '/path/out.zip' })
        .mockResolvedValueOnce({
          ok: true,
          data: {
            archive_path: '/path/out.zip',
            file_size_bytes: BigInt(100),
            records_exported: 1,
            warnings: []
          }
        });

      await controller.handleExport();

      expect(controller.isExporting).toBe(false);
    });

    it('should reset isExporting to false after error', async () => {
      mockSafeInvoke
        .mockResolvedValueOnce({ ok: true, data: '/path/out.zip' })
        .mockResolvedValueOnce({
          ok: false,
          error: { message: 'Write failed', kind: 'unknown' }
        });

      await controller.handleExport();

      expect(controller.isExporting).toBe(false);
    });
  });

  describe('createExportController factory', () => {
    it('should create a new ExportController instance', () => {
      const c1 = createExportController();
      const c2 = createExportController();
      expect(c1).not.toBe(c2);
      expect(c1).toBeInstanceOf(ExportController);
    });

    it('should create independent instances', () => {
      const c1 = createExportController();
      const c2 = createExportController();
      c1.error = 'Some error';
      expect(c2.error).toBeNull();
    });
  });

  describe('getExportController singleton', () => {
    it('should return same instance on multiple calls', () => {
      const c1 = getExportController();
      const c2 = getExportController();
      expect(c1).toBe(c2);
    });
  });
});
