<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import type { RecordCounts } from '$lib/bindings';

  interface Props {
    added: RecordCounts;
    skipped: RecordCounts;
    imagesImported: number;
    imagesFailed: Array<{ filename: string; reason: string }>;
  }

  const { added, skipped, imagesImported, imagesFailed }: Props = $props();

  const totalAdded = $derived(
    added.manufacturers + added.railwayModels + added.collectionItems + added.sellers
  );
  const totalSkipped = $derived(
    skipped.manufacturers + skipped.railwayModels + skipped.collectionItems + skipped.sellers
  );
  const hasImageFailures = $derived(imagesFailed.length > 0);

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

<div class="report-summary">
  <div class="summary-card success">
    <div class="card-header">
      <h3>{m.import_report_added_title()}</h3>
    </div>
    <div class="card-body">
      <div class="count-large">{totalAdded}</div>
      <div class="count-detail">{formatRecordCounts(added)}</div>
    </div>
  </div>

  <div class="summary-card neutral">
    <div class="card-header">
      <h3>{m.import_report_skipped_title()}</h3>
    </div>
    <div class="card-body">
      <div class="count-large">{totalSkipped}</div>
      <div class="count-detail">{formatRecordCounts(skipped)}</div>
    </div>
  </div>

  <div class="summary-card info">
    <div class="card-header">
      <h3>{m.import_report_images_title()}</h3>
    </div>
    <div class="card-body">
      <div class="count-large">{imagesImported}</div>
      {#if hasImageFailures}
        <div class="count-detail warning-text">
          {m.import_report_images_failed_count({ count: imagesFailed.length })}
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .report-summary {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
  }

  .summary-card {
    padding: 1.5rem;
    border-radius: var(--radius-md);
    border: 1px solid hsl(var(--border));
    background: hsl(var(--background));
  }

  .summary-card.success {
    border-color: hsl(var(--success) / 0.5);
    background: hsl(var(--success) / 0.05);
  }

  .summary-card.neutral {
    border-color: hsl(var(--muted) / 0.5);
    background: hsl(var(--muted) / 0.05);
  }

  .summary-card.info {
    border-color: hsl(var(--primary) / 0.5);
    background: hsl(var(--primary) / 0.05);
  }

  .card-header h3 {
    font-size: 0.75rem;
    font-weight: 600;
    color: hsl(var(--muted-foreground));
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin-bottom: 0.75rem;
  }

  .count-large {
    font-size: 2.5rem;
    font-weight: 700;
    line-height: 1;
    margin-bottom: 0.5rem;
  }

  .count-detail {
    font-size: 0.875rem;
    color: hsl(var(--muted-foreground));
  }

  .warning-text {
    color: hsl(var(--warning));
    font-weight: 500;
  }
</style>
