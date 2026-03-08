<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import type { ImportPreviewResponse } from '$lib/bindings';
  import ImportSummaryCards from './ImportSummaryCards.svelte';
  import ImportDuplicateDetails from './ImportDuplicateDetails.svelte';
  import ImportErrorSection from './ImportErrorSection.svelte';
  import ImportWarningsSection from './ImportWarningsSection.svelte';
  import ImportPreviewActions from './ImportPreviewActions.svelte';

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

  const hasDuplicates = $derived(
    preview.duplicateRecords.manufacturers > 0 ||
      preview.duplicateRecords.railwayModels > 0 ||
      preview.duplicateRecords.collectionItems > 0 ||
      preview.duplicateRecords.sellers > 0
  );
</script>

<div class="import-preview">
  <div class="preview-header">
    <h2>{m['import.preview.title']()}</h2>
    <p class="preview-subtitle">Review the import details before proceeding</p>
  </div>

  <!-- Summary Cards -->
  <ImportSummaryCards {preview} />

  <!-- Duplicate Details (Expandable) -->
  {#if hasDuplicates}
    <ImportDuplicateDetails {preview} />
  {/if}

  <!-- Errors -->
  <ImportErrorSection {preview} />

  <!-- Warnings -->
  <ImportWarningsSection {preview} />

  <!-- Actions -->
  <ImportPreviewActions {preview} {onConfirm} {onCancel} {loading} />
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
</style>
