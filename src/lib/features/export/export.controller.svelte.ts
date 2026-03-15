// Export feature controller
// Manages the frontend state for the archive export workflow

import { safeInvoke } from '$lib/services';
import { toaster } from '$lib/toaster';
import * as m from '$lib/paraglide/messages.js';
import type { ExportResult } from '$lib/bindings';

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

let controllerInstance: ExportController | null = null;

export function createExportController(): ExportController {
  return new ExportController();
}

export function getExportController(): ExportController {
  if (!controllerInstance) {
    controllerInstance = new ExportController();
  }
  return controllerInstance;
}
