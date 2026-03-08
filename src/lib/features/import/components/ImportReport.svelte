<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import type { ImportResultResponse, RecordCounts } from '$lib/bindings';
  import { Button } from '$lib/components';

  interface Props {
    result: ImportResultResponse;
    onClose?: () => void;
  }

  const { result, onClose = () => {} }: Props = $props();

  const totalAdded = $derived(
    result.added.manufacturers +
      result.added.railwayModels +
      result.added.collectionItems +
      result.added.sellers
  );
  const totalSkipped = $derived(
    result.skipped.manufacturers +
      result.skipped.railwayModels +
      result.skipped.collectionItems +
      result.skipped.sellers
  );
  const hasWarnings = $derived(result.warnings.length > 0);
  const hasImageFailures = $derived(result.imagesFailed.length > 0);
  const isSuccess = $derived(
    result.status === 'success' || result.status === 'successWithWarnings'
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

  function formatDuration(ms: bigint): string {
    const msNumber = Number(ms);
    if (msNumber < 1000) return `${msNumber}ms`;
    const seconds = (msNumber / 1000).toFixed(1);
    return `${seconds}s`;
  }
</script>

<div class="import-report">
  <div class="report-header">
    <div class="status-icon {isSuccess ? 'success' : 'error'}">
      {isSuccess ? '✓' : '✗'}
    </div>
    <h2>Import {isSuccess ? 'Complete' : 'Failed'}</h2>
    <p class="report-subtitle">
      Completed in {formatDuration(result.durationMs)}
    </p>
  </div>

  <!-- Summary Cards -->
  <div class="report-summary">
    <div class="summary-card success">
      <div class="card-header">
        <h3>Successfully Added</h3>
      </div>
      <div class="card-body">
        <div class="count-large">{totalAdded}</div>
        <div class="count-detail">{formatRecordCounts(result.added)}</div>
      </div>
    </div>

    <div class="summary-card neutral">
      <div class="card-header">
        <h3>Skipped (Duplicates)</h3>
      </div>
      <div class="card-body">
        <div class="count-large">{totalSkipped}</div>
        <div class="count-detail">{formatRecordCounts(result.skipped)}</div>
      </div>
    </div>

    <div class="summary-card info">
      <div class="card-header">
        <h3>Images</h3>
      </div>
      <div class="card-body">
        <div class="count-large">{result.imagesImported}</div>
        {#if hasImageFailures}
          <div class="count-detail warning-text">{result.imagesFailed.length} failed</div>
        {/if}
      </div>
    </div>
  </div>

  <!-- Warnings -->
  {#if hasWarnings}
    <div class="report-warnings">
      <h3 class="warning-title">Warnings</h3>
      <ul class="warning-list">
        {#each result.warnings as warning (warning.message)}
          <li class="warning-item">{warning.message}</li>
        {/each}
      </ul>
    </div>
  {/if}

  <!-- Image Failures -->
  {#if hasImageFailures}
    <div class="report-errors">
      <h3 class="error-title">Image Import Failures</h3>
      <ul class="error-list">
        {#each result.imagesFailed as failure (failure.filename)}
          <li class="error-item">
            <strong>{failure.filename}:</strong>
            {failure.reason}
          </li>
        {/each}
      </ul>
    </div>
  {/if}

  <!-- Actions -->
  <div class="report-actions">
    <Button onclick={onClose} type="button">{m.common_cancel()}</Button>
  </div>
</div>

<style>
  .import-report {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    padding: 1.5rem;
    background: hsl(var(--card));
    border-radius: var(--radius-lg);
  }

  .report-header {
    text-align: center;
  }

  .status-icon {
    width: 4rem;
    height: 4rem;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 2rem;
    font-weight: bold;
    margin: 0 auto 1rem;
  }

  .status-icon.success {
    background: hsl(var(--success) / 0.1);
    color: hsl(var(--success));
  }

  .status-icon.error {
    background: hsl(var(--destructive) / 0.1);
    color: hsl(var(--destructive));
  }

  .report-subtitle {
    color: hsl(var(--muted-foreground));
    font-size: 0.875rem;
    margin-top: 0.5rem;
  }

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

  .report-warnings,
  .report-errors {
    padding: 1rem;
    border-radius: var(--radius-md);
  }

  .report-warnings {
    border: 1px solid hsl(var(--warning) / 0.5);
    background: hsl(var(--warning) / 0.05);
  }

  .report-errors {
    border: 1px solid hsl(var(--destructive) / 0.5);
    background: hsl(var(--destructive) / 0.05);
  }

  .warning-title,
  .error-title {
    font-size: 0.875rem;
    font-weight: 600;
    margin-bottom: 0.75rem;
  }

  .warning-list,
  .error-list {
    list-style: disc inside;
    margin: 0;
    padding: 0;
  }

  .warning-item,
  .error-item {
    font-size: 0.875rem;
    padding: 0.25rem 0;
  }

  .report-actions {
    display: flex;
    justify-content: center;
    padding-top: 1rem;
    border-top: 1px solid hsl(var(--border));
  }
</style>
