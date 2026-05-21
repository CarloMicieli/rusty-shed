import { openUrl } from '@tauri-apps/plugin-opener';
import { safeInvoke } from '$lib/services';
import type { ImportActionResult } from './localImport';

const GOOGLE_DRIVE_AUTH_URL = 'https://accounts.google.com/o/oauth2/v2/auth';
const SUPPORTED_BACKUP_EXTENSIONS = ['.json', '.db'] as const;

interface GoogleDriveBackupFile {
  id: string;
  name: string;
}

function isSupportedBackup(name: string): boolean {
  return SUPPORTED_BACKUP_EXTENSIONS.some((ext) => name.toLowerCase().endsWith(ext));
}

export async function runGoogleDriveImport(): Promise<ImportActionResult> {
  try {
    await openUrl(GOOGLE_DRIVE_AUTH_URL);

    const oauthResult = await safeInvoke<{ oauthCode: string }>(
      'onboarding_get_google_drive_oauth_code'
    );
    if (!oauthResult.ok) {
      return { ok: false, error: oauthResult.error.message };
    }

    const listResult = await safeInvoke<GoogleDriveBackupFile[]>(
      'onboarding_list_google_drive_backups',
      {
        oauthCode: oauthResult.data.oauthCode
      }
    );
    if (!listResult.ok) {
      return { ok: false, error: listResult.error.message };
    }

    const supportedBackups = listResult.data.filter((file) => isSupportedBackup(file.name));
    if (supportedBackups.length === 0) {
      return { ok: false, error: 'No supported Google Drive backups found.' };
    }

    const selectedBackup = supportedBackups[0];

    const result = await safeInvoke<{ accepted: boolean }>('onboarding_import_google_drive', {
      oauthCode: oauthResult.data.oauthCode,
      backupFileId: selectedBackup.id
    });

    if (!result.ok) {
      return { ok: false, error: result.error.message };
    }

    return { ok: true };
  } catch (error) {
    return { ok: false, error: String(error) };
  }
}
