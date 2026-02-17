<script lang="ts">
  import { Button } from '$lib/components';
  import { getDatabaseBackupController } from '../index';
  import * as m from '$lib/paraglide/messages.js';

  const controller = getDatabaseBackupController();

  let isExporting = $derived(controller.isExporting);
  let isImporting = $derived(controller.isImporting);
  let isDisabled = $derived(controller.isOperationInProgress);
</script>

<div class="card border-surface-700/40 border p-6 shadow-xl">
  <div class="space-y-4">
    <div>
      <h2 class="text-xl font-bold">{m.data_management_title()}</h2>
      <p class="text-surface-400 mt-1 text-sm">{m.data_management_subtitle()}</p>
    </div>

    <div class="flex flex-wrap gap-4">
      <Button variant="default" disabled={isDisabled} onclick={() => controller.handleExport()}>
        {isExporting ? m.data_management_exporting() : m.data_management_export_button()}
      </Button>

      <Button variant="outline" disabled={isDisabled} onclick={() => controller.handleImport()}>
        {isImporting ? m.data_management_importing() : m.data_management_import_button()}
      </Button>
    </div>

    <div class="text-warning-600 bg-warning-50 border-warning-200 rounded-md border p-3 text-sm">
      <p>{m.data_management_import_warning()}</p>
    </div>
  </div>
</div>
