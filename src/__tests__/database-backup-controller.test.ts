import { beforeEach, describe, expect, it, vi } from 'vitest';

const dialogMocks = vi.hoisted(() => ({
  save: vi.fn(),
  open: vi.fn()
}));

const serviceMocks = vi.hoisted(() => ({
  exportDatabase: vi.fn(),
  importDatabase: vi.fn()
}));

const toasterMocks = vi.hoisted(() => ({
  success: vi.fn(),
  error: vi.fn()
}));

vi.mock('@tauri-apps/plugin-dialog', () => dialogMocks);
vi.mock('$lib/services', () => serviceMocks);
vi.mock('$lib/toaster', () => ({ toaster: toasterMocks }));
vi.mock('$lib/paraglide/messages.js', () => ({
  data_management_file_picker_export_title: () => 'Export database',
  data_management_file_picker_import_title: () => 'Import database',
  data_management_export_success: ({ path }: { path: string }) => `Exported ${path}`,
  data_management_export_error: ({ error }: { error: string }) => `Export failed: ${error}`,
  data_management_import_success: () => 'Import complete',
  data_management_import_error: ({ error }: { error: string }) => `Import failed: ${error}`
}));

import { DatabaseBackupController } from '$lib/features/database-backup/DatabaseBackupController.svelte';

describe('DatabaseBackupController', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('exports to the selected destination and resets operation state', async () => {
    dialogMocks.save.mockResolvedValue('/backups/rusty.sqlite');
    serviceMocks.exportDatabase.mockResolvedValue({
      ok: true,
      data: { file_path: '/backups/rusty.sqlite' }
    });

    const controller = new DatabaseBackupController();
    await controller.handleExport();

    expect(serviceMocks.exportDatabase).toHaveBeenCalledWith('/backups/rusty.sqlite');
    expect(toasterMocks.success).toHaveBeenCalledWith({
      title: 'Exported /backups/rusty.sqlite'
    });
    expect(controller.isExporting).toBe(false);
    expect(controller.isOperationInProgress).toBe(false);
    expect(controller.error).toBeNull();
  });

  it('does nothing when the export destination dialog is cancelled', async () => {
    dialogMocks.save.mockResolvedValue(null);
    const controller = new DatabaseBackupController();

    await controller.handleExport();

    expect(serviceMocks.exportDatabase).not.toHaveBeenCalled();
    expect(controller.isOperationInProgress).toBe(false);
  });

  it('stores structured export errors and exposes an error toast', async () => {
    dialogMocks.save.mockResolvedValue('/backups/rusty.sqlite');
    serviceMocks.exportDatabase.mockResolvedValue({
      ok: false,
      error: { kind: 'permission_denied', message: 'Permission denied' }
    });
    const controller = new DatabaseBackupController();

    await controller.handleExport();

    expect(controller.error).toBe('Permission denied');
    expect(controller.isOperationInProgress).toBe(false);
    expect(toasterMocks.error).toHaveBeenCalledWith({
      title: 'Export failed: Permission denied'
    });
  });

  it('converts thrown export failures to state and toasts', async () => {
    dialogMocks.save.mockResolvedValue('/backups/rusty.sqlite');
    serviceMocks.exportDatabase.mockRejectedValue(new Error('Disk full'));
    const controller = new DatabaseBackupController();

    await controller.handleExport();

    expect(controller.error).toBe('Disk full');
    expect(controller.isExporting).toBe(false);
  });

  it('blocks concurrent operations', async () => {
    dialogMocks.save.mockResolvedValue('/backups/rusty.sqlite');
    let resolveExport!: (value: { ok: true; data: { file_path: string } }) => void;
    serviceMocks.exportDatabase.mockImplementation(
      () =>
        new Promise<{ ok: true; data: { file_path: string } }>((resolve) => {
          resolveExport = resolve;
        })
    );
    const controller = new DatabaseBackupController();

    const firstExport = controller.handleExport();
    await vi.waitFor(() => expect(controller.isOperationInProgress).toBe(true));
    await controller.handleImport();

    expect(dialogMocks.open).not.toHaveBeenCalled();
    resolveExport({ ok: true, data: { file_path: '/backups/rusty.sqlite' } });
    await firstExport;
  });

  it('imports the selected file with the RESTORE confirmation', async () => {
    dialogMocks.open.mockResolvedValue('/backups/source.sqlite');
    serviceMocks.importDatabase.mockResolvedValue({
      ok: true,
      data: { restored: true }
    });
    const controller = new DatabaseBackupController();

    await controller.handleImport();

    expect(serviceMocks.importDatabase).toHaveBeenCalledWith('/backups/source.sqlite', 'RESTORE');
    expect(toasterMocks.success).toHaveBeenCalledWith({ title: 'Import complete' });
    expect(controller.isImporting).toBe(false);
    expect(controller.isOperationInProgress).toBe(false);
  });

  it('handles import cancellation and array-shaped dialog results safely', async () => {
    dialogMocks.open.mockResolvedValueOnce(null).mockResolvedValueOnce([]);
    const controller = new DatabaseBackupController();

    await controller.handleImport();
    await controller.handleImport();

    expect(serviceMocks.importDatabase).not.toHaveBeenCalled();
  });

  it('clears an existing error', async () => {
    dialogMocks.save.mockResolvedValue('/backups/rusty.sqlite');
    serviceMocks.exportDatabase.mockResolvedValue({
      ok: false,
      error: { kind: 'database', message: 'I/O failure' }
    });
    const controller = new DatabaseBackupController();

    await controller.handleExport();
    expect(controller.error).toBe('I/O failure');

    controller.clearError();
    expect(controller.error).toBeNull();
  });
});
