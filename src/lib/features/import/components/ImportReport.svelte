<script lang="ts">
  import type { ImportResultResponse } from '$lib/bindings';
  import ImportReportHeader from './ImportReportHeader.svelte';
  import ImportReportSummaryCards from './ImportReportSummaryCards.svelte';
  import ImportReportWarnings from './ImportReportWarnings.svelte';
  import ImportReportImageFailures from './ImportReportImageFailures.svelte';
  import ImportReportActions from './ImportReportActions.svelte';

  interface Props {
    result: ImportResultResponse;
    onClose?: () => void;
  }

  const { result, onClose = () => {} }: Props = $props();

  const isSuccess = $derived(
    result.status === 'success' || result.status === 'successWithWarnings'
  );
</script>

<div class="import-report">
  <ImportReportHeader {isSuccess} durationMs={result.durationMs} />
  <ImportReportSummaryCards
    added={result.added}
    skipped={result.skipped}
    imagesImported={result.imagesImported}
    imagesFailed={result.imagesFailed}
  />
  <ImportReportWarnings warnings={result.warnings} />
  <ImportReportImageFailures failures={result.imagesFailed} />
  <ImportReportActions {onClose} />
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
</style>
