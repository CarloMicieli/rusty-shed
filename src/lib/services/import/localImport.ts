import { open } from '@tauri-apps/plugin-dialog';
import { safeInvoke } from '$lib/services';

export interface ImportActionResult {
  ok: boolean;
  error?: string;
}

export async function runLocalArchiveImport(): Promise<ImportActionResult> {
  try {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: 'Rusty Shed Backup',
          extensions: ['json', 'db']
        }
      ]
    });

    if (!selected || Array.isArray(selected)) {
      return { ok: false, error: 'No archive selected.' };
    }

    const invokeResult = await safeInvoke<{ accepted: boolean }>('onboarding_import_local', {
      filePath: selected,
      formatHint: selected.endsWith('.db') ? 'db' : 'json'
    });

    if (!invokeResult.ok) {
      return { ok: false, error: invokeResult.error.message };
    }

    return { ok: true };
  } catch (error) {
    return { ok: false, error: String(error) };
  }
}
