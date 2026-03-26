<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import type { ImportPreviewResponse, RecordCounts } from '$lib/bindings';

  interface Props {
    preview: ImportPreviewResponse;
  }

  const { preview }: Props = $props();

  const hasDuplicates = $derived(
    preview.duplicateRecords.manufacturers > 0 ||
      preview.duplicateRecords.railwayModels > 0 ||
      preview.duplicateRecords.collectionItems > 0 ||
      preview.duplicateRecords.sellers > 0
  );

  function formatRecordCounts(counts: RecordCounts): string {
    const parts: string[] = [];
    if (counts.manufacturers > 0) parts.push(`${counts.manufacturers} manufacturers`);
    if (counts.railwayCompanies > 0) parts.push(`${counts.railwayCompanies} companies`);
    if (counts.railwayModels > 0) parts.push(`${counts.railwayModels} models`);
    if (counts.collectionItems > 0) parts.push(`${counts.collectionItems} items`);
    if (counts.sellers > 0) parts.push(`${counts.sellers} sellers`);
    if (counts.maintenanceCards > 0) parts.push(`${counts.maintenanceCards} maintenance`);
    return parts.join(', ') || '0 records';
  }
</script>

<div class="preview-summary">
  <div class="summary-card">
    <div class="card-header">
      <h3>{m.import_total_records_heading()}</h3>
    </div>
    <div class="card-body">
      <div class="count-display">{formatRecordCounts(preview.totalRecords)}</div>
    </div>
  </div>

  <div class="summary-card success">
    <div class="card-header">
      <h3>{m.import_new_records_heading()}</h3>
    </div>
    <div class="card-body">
      <div class="count-display">{formatRecordCounts(preview.newRecords)}</div>
    </div>
  </div>

  {#if hasDuplicates}
    <div class="summary-card warning">
      <div class="card-header">
        <h3>Duplicate Records</h3>
      </div>
      <div class="card-body">
        <div class="count-display">{formatRecordCounts(preview.duplicateRecords)}</div>
        <p class="help-text">These records already exist and will be skipped</p>
      </div>
    </div>
  {/if}
</div>

<style>
  .preview-summary {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1rem;
  }

  .summary-card {
    padding: 1rem;
    border-radius: var(--radius-md);
    border: 1px solid hsl(var(--border));
    background: hsl(var(--background));
  }

  .summary-card.success {
    border-color: hsl(var(--success) / 0.5);
    background: hsl(var(--success) / 0.05);
  }

  .summary-card.warning {
    border-color: hsl(var(--warning) / 0.5);
    background: hsl(var(--warning) / 0.05);
  }

  .card-header h3 {
    font-size: 0.875rem;
    font-weight: 600;
    color: hsl(var(--muted-foreground));
    margin-bottom: 0.5rem;
  }

  .count-display {
    font-size: 1.125rem;
    font-weight: 600;
    margin-bottom: 0.25rem;
  }

  .help-text {
    font-size: 0.75rem;
    color: hsl(var(--muted-foreground));
    margin-top: 0.5rem;
  }
</style>
