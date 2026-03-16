<script lang="ts">
  import { Button } from '$lib/components';
  import { getExportContext } from '../export.controller.svelte';
  import ImportDrawer from '$lib/features/import/components/ImportDrawer.svelte';
  import * as m from '$lib/paraglide/messages.js';

  const controller = getExportContext();

  let isExporting = $derived(controller.isExporting);
  let importDrawerOpen = $state(false);
</script>

<div class="card border border-border/60 bg-card/50 shadow-xl">
  <header class="flex items-center justify-between gap-4 border-b border-border/60 p-6">
    <div>
      <p class="text-surface-400 text-sm font-semibold tracking-widest uppercase">
        {m.export_archive_title()}
      </p>
      <p class="mt-1 text-sm text-muted-foreground">{m.export_archive_subtitle()}</p>
    </div>
  </header>

  <div class="space-y-4 p-6">
    <div class="flex flex-wrap gap-4">
      <Button variant="default" disabled={isExporting} onclick={() => controller.handleExport()}>
        {isExporting ? m.export_archive_exporting() : m.export_archive_button()}
      </Button>

      <Button variant="outline" onclick={() => (importDrawerOpen = true)}>
        {m.export_archive_import_button()}
      </Button>
    </div>
  </div>
</div>

<ImportDrawer bind:open={importDrawerOpen} />
