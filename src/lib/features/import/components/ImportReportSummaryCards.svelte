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

<hr class="border-t border-dashed border-border/50" />

<div class="report-summary">
  <div class="summary-card success">
    <div class="card-label font-mono text-[10px] tracking-widest text-muted-foreground uppercase">
      {m.import_report_added_title()}
    </div>
    <div class="count-large font-mono">{totalAdded}</div>
    <div class="count-detail font-mono text-[10px] text-muted-foreground">
      {formatRecordCounts(added)}
    </div>
  </div>

  <div class="summary-card neutral">
    <div class="card-label font-mono text-[10px] tracking-widest text-muted-foreground uppercase">
      {m.import_report_skipped_title()}
    </div>
    <div class="count-large font-mono">{totalSkipped}</div>
    <div class="count-detail font-mono text-[10px] text-muted-foreground">
      {formatRecordCounts(skipped)}
    </div>
  </div>

  <div class="summary-card info">
    <div class="card-label font-mono text-[10px] tracking-widest text-muted-foreground uppercase">
      {m.import_report_images_title()}
    </div>
    <div class="count-large font-mono">{imagesImported}</div>
    {#if hasImageFailures}
      <div class="count-detail text-warning font-mono text-[10px]">
        {m.import_report_images_failed_count({ count: imagesFailed.length })}
      </div>
    {/if}
  </div>
</div>

<style>
  .report-summary {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
    gap: 1rem;
  }

  .summary-card {
    padding: 1.25rem;
    border-radius: var(--radius-sm);
    border: 1px solid hsl(var(--border));
    background: hsl(var(--background));
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .summary-card.success {
    border-color: hsl(var(--success) / 0.4);
    background: hsl(var(--success) / 0.05);
  }

  .summary-card.neutral {
    border-color: hsl(var(--border) / 0.6);
    background: hsl(var(--muted) / 0.05);
  }

  .summary-card.info {
    border-color: hsl(var(--primary) / 0.4);
    background: hsl(var(--primary) / 0.05);
  }

  .card-label {
    margin-bottom: 0.5rem;
  }

  .count-large {
    font-size: 2.5rem;
    font-weight: 700;
    line-height: 1;
    color: hsl(var(--foreground));
  }
</style>
