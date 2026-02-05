<script lang="ts">
  import type { ImportPreviewResponse, RecordCounts } from '$lib/bindings';

  interface Props {
    preview: ImportPreviewResponse;
    onConfirm?: () => Promise<void>;
    onCancel?: () => void;
    loading?: boolean;
  }

  const {
    preview,
    onConfirm = async () => {},
    onCancel = () => {},
    loading = false
  }: Props = $props();

  const hasErrors = $derived(preview.errors.length > 0);
  const hasWarnings = $derived(preview.warnings.length > 0);
  const hasDuplicates = $derived(
    preview.duplicateRecords.manufacturers > 0 ||
      preview.duplicateRecords.railwayModels > 0 ||
      preview.duplicateRecords.collectionItems > 0 ||
      preview.duplicateRecords.sellers > 0
  );

  let duplicatesExpanded = $state(false);

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

<div class="import-preview">
  <div class="preview-header">
    <h2>Import Preview</h2>
    <p class="preview-subtitle">Review the import details before proceeding</p>
  </div>

  <!-- Summary Cards -->
  <div class="preview-summary">
    <div class="summary-card">
      <div class="card-header">
        <h3>Total Records</h3>
      </div>
      <div class="card-body">
        <div class="count-display">{formatRecordCounts(preview.totalRecords)}</div>
      </div>
    </div>

    <div class="summary-card success">
      <div class="card-header">
        <h3>New Records</h3>
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

  <!-- Duplicate Details (Expandable) -->
  {#if hasDuplicates}
    <div class="duplicate-details">
      <button
        class="details-toggle"
        type="button"
        onclick={() => (duplicatesExpanded = !duplicatesExpanded)}
      >
        <span class="toggle-icon">{duplicatesExpanded ? '▼' : '▶'}</span>
        View Duplicate Records
      </button>

      {#if duplicatesExpanded}
        <div class="details-content">
          {#if preview.duplicateDetails.manufacturers.length > 0}
            <div class="detail-section">
              <h4>Manufacturers ({preview.duplicateDetails.manufacturers.length})</h4>
              <ul class="detail-list">
                {#each preview.duplicateDetails.manufacturers as name (name)}
                  <li>{name}</li>
                {/each}
              </ul>
            </div>
          {/if}

          {#if preview.duplicateDetails.railwayModels.length > 0}
            <div class="detail-section">
              <h4>Railway Models ({preview.duplicateDetails.railwayModels.length})</h4>
              <ul class="detail-list">
                {#each preview.duplicateDetails.railwayModels as id (id)}
                  <li>{id}</li>
                {/each}
              </ul>
            </div>
          {/if}

          {#if preview.duplicateDetails.collectionItems.length > 0}
            <div class="detail-section">
              <h4>Collection Items ({preview.duplicateDetails.collectionItems.length})</h4>
              <ul class="detail-list">
                {#each preview.duplicateDetails.collectionItems as id (id)}
                  <li>{id}</li>
                {/each}
              </ul>
            </div>
          {/if}

          {#if preview.duplicateDetails.sellers.length > 0}
            <div class="detail-section">
              <h4>Sellers ({preview.duplicateDetails.sellers.length})</h4>
              <ul class="detail-list">
                {#each preview.duplicateDetails.sellers as name (name)}
                  <li>{name}</li>
                {/each}
              </ul>
            </div>
          {/if}
        </div>
      {/if}
    </div>
  {/if}

  <!-- Errors -->
  {#if hasErrors}
    <div class="preview-errors">
      <h3 class="error-title">❌ Validation Errors</h3>
      <p class="error-subtitle">Import cannot proceed until these issues are fixed:</p>
      <ul class="error-list">
        {#each preview.errors as error (error.code + error.message + error.path)}
          <li class="error-item">
            <div class="error-content">
              <strong class="error-code">{error.code}</strong>
              <span class="error-message">{error.message}</span>
              {#if error.path}
                <span class="error-path">at {error.path}</span>
              {/if}
            </div>
          </li>
        {/each}
      </ul>
    </div>
  {/if}

  <!-- Warnings -->
  {#if hasWarnings}
    <div class="preview-warnings">
      <h3 class="warning-title">Warnings</h3>
      <ul class="warning-list">
        {#each preview.warnings as warning (warning.message)}
          <li class="warning-item">
            {warning.message}
          </li>
        {/each}
      </ul>
    </div>
  {/if}

  <!-- Actions -->
  <div class="preview-actions">
    <button class="btn btn-secondary" onclick={onCancel} disabled={loading} type="button">
      Cancel
    </button>
    <button
      class="btn btn-primary"
      onclick={onConfirm}
      disabled={!preview.canImport || loading}
      type="button"
    >
      {loading ? 'Importing...' : 'Confirm Import'}
    </button>
  </div>
</div>

<style>
  .import-preview {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    padding: 1.5rem;
    background: hsl(var(--card));
    border-radius: var(--radius-lg);
  }

  .preview-header {
    text-align: center;
  }

  .preview-subtitle {
    color: hsl(var(--muted-foreground));
    font-size: 0.875rem;
    margin-top: 0.5rem;
  }

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

  .preview-errors,
  .preview-warnings {
    padding: 1rem;
    border-radius: var(--radius-md);
  }

  .preview-errors {
    border: 1px solid hsl(var(--destructive) / 0.5);
    background: hsl(var(--destructive) / 0.05);
  }

  .error-subtitle {
    font-size: 0.8125rem;
    color: hsl(var(--destructive));
    margin-bottom: 0.75rem;
    font-weight: 500;
  }

  .error-content {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .error-code {
    font-size: 0.75rem;
    color: hsl(var(--destructive));
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .error-message {
    font-size: 0.875rem;
    color: hsl(var(--foreground));
  }

  .error-path {
    font-size: 0.75rem;
    color: hsl(var(--muted-foreground));
    font-family: monospace;
  }

  .preview-warnings {
    border: 1px solid hsl(var(--warning) / 0.5);
    background: hsl(var(--warning) / 0.05);
  }

  .duplicate-details {
    padding: 1rem;
    border-radius: var(--radius-md);
    border: 1px solid hsl(var(--warning) / 0.5);
    background: hsl(var(--warning) / 0.05);
  }

  .details-toggle {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    width: 100%;
    padding: 0.75rem;
    background: none;
    border: none;
    cursor: pointer;
    font-weight: 500;
    font-size: 0.875rem;
    color: hsl(var(--foreground));
    transition: background-color 0.2s;
    border-radius: var(--radius-sm);
  }

  .details-toggle:hover {
    background: hsl(var(--accent) / 0.5);
  }

  .toggle-icon {
    font-size: 0.75rem;
    transition: transform 0.2s;
  }

  .details-content {
    margin-top: 1rem;
    padding-top: 1rem;
    border-top: 1px solid hsl(var(--border));
  }

  .detail-section {
    margin-bottom: 1.5rem;
  }

  .detail-section:last-child {
    margin-bottom: 0;
  }

  .detail-section h4 {
    font-size: 0.875rem;
    font-weight: 600;
    margin-bottom: 0.5rem;
    color: hsl(var(--foreground));
  }

  .detail-list {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .detail-list li {
    padding: 0.25rem 0.75rem;
    background: hsl(var(--muted));
    border-radius: var(--radius-sm);
    font-size: 0.8125rem;
    color: hsl(var(--foreground));
  }

  .error-title,
  .warning-title {
    font-size: 0.875rem;
    font-weight: 600;
    margin-bottom: 0.75rem;
  }

  .error-list,
  .warning-list {
    list-style: disc inside;
    margin: 0;
    padding: 0;
  }

  .error-item,
  .warning-item {
    font-size: 0.875rem;
    padding: 0.25rem 0;
  }

  .preview-actions {
    display: flex;
    justify-content: flex-end;
    gap: 1rem;
    padding-top: 1rem;
    border-top: 1px solid hsl(var(--border));
  }

  .btn {
    padding: 0.5rem 1.5rem;
    border-radius: var(--radius-md);
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-secondary {
    background: hsl(var(--secondary));
    color: hsl(var(--secondary-foreground));
    border: 1px solid hsl(var(--border));
  }

  .btn-secondary:hover:not(:disabled) {
    background: hsl(var(--secondary) / 0.8);
  }

  .btn-primary {
    background: hsl(var(--primary));
    color: hsl(var(--primary-foreground));
    border: none;
  }

  .btn-primary:hover:not(:disabled) {
    background: hsl(var(--primary) / 0.9);
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
