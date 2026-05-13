// Export feature controller
// Manages the frontend state for the archive export workflow

import { setContext, getContext } from 'svelte';
import { safeInvoke } from '$lib/services';
import { toaster } from '$lib/toaster';
import * as m from '$lib/paraglide/messages.js';
import type { ExportResult } from '$lib/bindings';

const EXPORT_CONTEXT_KEY = Symbol('export-controller');

export class ExportController {
  isExporting: boolean = $state(false);
  error: string | null = $state(null);

  async handleExport(): Promise<void> {
    if (this.isExporting) return;

    // Open the native file save dialog
    const pathResult = await safeInvoke<string | null>('open_export_file_dialog');
    if (!pathResult.ok) {
      this.error = pathResult.error.message;
      toaster.error({ title: m.export_archive_error({ error: pathResult.error.message }) });
      return;
    }

    const destinationPath = pathResult.data;
    if (!destinationPath) {
      // User cancelled the dialog
      return;
    }

    this.isExporting = true;
    this.error = null;

    try {
      const exportResult = await safeInvoke<ExportResult>('execute_export', {
        destinationPath
      });

      if (exportResult.ok) {
        toaster.success({
          title: m.export_archive_success({ path: exportResult.data.archivePath })
        });
      } else {
        this.error = exportResult.error.message;
        toaster.error({
          title: m.export_archive_error({ error: exportResult.error.message })
        });
      }
    } finally {
      this.isExporting = false;
    }
  }
}

/** Singleton instance — import this directly instead of using setContext/getContext. */
export const exportController = new ExportController();

export function createExportController(): ExportController {
  return new ExportController();
}

export function setExportContext(controller: ExportController) {
  setContext(EXPORT_CONTEXT_KEY, controller);
}

export function getExportContext(): ExportController {
  return getContext(EXPORT_CONTEXT_KEY);
}
