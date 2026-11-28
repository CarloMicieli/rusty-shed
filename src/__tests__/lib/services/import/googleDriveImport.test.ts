import { beforeEach, describe, expect, it, vi } from 'vitest';

const { openUrlMock, safeInvokeMock } = vi.hoisted(() => ({
  openUrlMock: vi.fn(),
  safeInvokeMock: vi.fn()
}));

vi.mock('@tauri-apps/plugin-opener', () => ({
  openUrl: openUrlMock
}));

vi.mock('$lib/services', () => ({
  safeInvoke: safeInvokeMock
}));

import { runGoogleDriveImport } from '$lib/services/import/googleDriveImport';

describe('runGoogleDriveImport', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('imports the first supported backup after OAuth flow', async () => {
    openUrlMock.mockResolvedValueOnce(undefined);
    safeInvokeMock
      .mockResolvedValueOnce({ ok: true, data: { oauthCode: 'code-123' } })
      .mockResolvedValueOnce({
        ok: true,
        data: [
          { id: 'note', name: 'notes.txt' },
          { id: 'backup-json', name: 'backup.json' },
          { id: 'backup-db', name: 'backup.db' }
        ]
      })
      .mockResolvedValueOnce({ ok: true, data: { accepted: true } });

    const result = await runGoogleDriveImport();

    expect(result).toEqual({ ok: true });
    expect(openUrlMock).toHaveBeenCalled();
    expect(safeInvokeMock).toHaveBeenNthCalledWith(3, 'onboarding_import_google_drive', {
      oauthCode: 'code-123',
      backupFileId: 'backup-json'
    });
  });

  it('returns oauth/list/import command errors', async () => {
    openUrlMock.mockResolvedValue(undefined);
    safeInvokeMock.mockResolvedValueOnce({
      ok: false,
      error: { kind: 'unknown', message: 'oauth failed' }
    });

    await expect(runGoogleDriveImport()).resolves.toEqual({ ok: false, error: 'oauth failed' });

    safeInvokeMock
      .mockResolvedValueOnce({ ok: true, data: { oauthCode: 'code-1' } })
      .mockResolvedValueOnce({
        ok: false,
        error: { kind: 'unknown', message: 'list failed' }
      });

    await expect(runGoogleDriveImport()).resolves.toEqual({ ok: false, error: 'list failed' });

    safeInvokeMock
      .mockResolvedValueOnce({ ok: true, data: { oauthCode: 'code-2' } })
      .mockResolvedValueOnce({ ok: true, data: [{ id: 'backup-db', name: 'latest.db' }] })
      .mockResolvedValueOnce({
        ok: false,
        error: { kind: 'unknown', message: 'import failed' }
      });

    await expect(runGoogleDriveImport()).resolves.toEqual({ ok: false, error: 'import failed' });
  });

  it('fails when no supported backup exists and when opener throws', async () => {
    openUrlMock.mockResolvedValueOnce(undefined);
    safeInvokeMock
      .mockResolvedValueOnce({ ok: true, data: { oauthCode: 'code-3' } })
      .mockResolvedValueOnce({
        ok: true,
        data: [{ id: 'readme', name: 'README.md' }]
      });

    await expect(runGoogleDriveImport()).resolves.toEqual({
      ok: false,
      error: 'No supported Google Drive backups found.'
    });

    openUrlMock.mockRejectedValueOnce(new Error('blocked by browser'));

    await expect(runGoogleDriveImport()).resolves.toEqual({
      ok: false,
      error: 'Error: blocked by browser'
    });
  });
});
