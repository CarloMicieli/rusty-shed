import { beforeEach, describe, expect, it, vi } from 'vitest';

const safeInvokeMock = vi.hoisted(() => vi.fn());
vi.mock('$lib/services/tauri', () => ({ safeInvoke: safeInvokeMock }));

import { exportDatabase, importDatabase } from '$lib/services/database-backup';

describe('database backup services', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('exports through the destination-path command argument', async () => {
    const response = { ok: true as const, data: { file_path: '/backups/rusty.sqlite' } };
    safeInvokeMock.mockResolvedValue(response);

    await expect(exportDatabase('/backups/rusty.sqlite')).resolves.toEqual(response);
    expect(safeInvokeMock).toHaveBeenCalledWith('export_database', {
      args: { destination_path: '/backups/rusty.sqlite' }
    });
  });

  it('preserves structured export failures', async () => {
    const response = {
      ok: false as const,
      error: { kind: 'permission_denied' as const, message: 'Denied' }
    };
    safeInvokeMock.mockResolvedValue(response);

    await expect(exportDatabase('/backups/rusty.sqlite')).resolves.toEqual(response);
  });

  it('imports with RESTORE confirmation and source path', async () => {
    const response = { ok: true as const, data: { restored: true } };
    safeInvokeMock.mockResolvedValue(response);

    await expect(importDatabase('/backups/source.sqlite', 'RESTORE')).resolves.toEqual(response);
    expect(safeInvokeMock).toHaveBeenCalledWith('import_database', {
      args: { source_path: '/backups/source.sqlite', confirmation: 'RESTORE' }
    });
  });

  it('preserves structured import failures', async () => {
    const response = {
      ok: false as const,
      error: { kind: 'validation' as const, message: 'Wrong confirmation' }
    };
    safeInvokeMock.mockResolvedValue(response);

    await expect(importDatabase('/backups/source.sqlite', 'WRONG')).resolves.toEqual(response);
  });
});
