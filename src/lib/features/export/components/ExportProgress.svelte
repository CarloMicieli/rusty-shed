<script lang="ts">
  /**
   * ExportProgress component
   * Displays progress bar and status during export
   */
  import * as m from '$lib/paraglide/messages.js';

  interface Props {
    progress?: number;
    phase?: 'collecting' | 'compressing' | 'finalizing' | null;
    currentItem?: string | null;
    estimatedSecondsRemaining?: number;
  }

  const {
    progress = 0,
    phase = null,
    currentItem = null,
    estimatedSecondsRemaining = 0
  }: Props = $props();
</script>

<div class="export-progress">
  <div class="progress-bar">
    <div class="progress-fill" style="width: {progress}%"></div>
  </div>
  <div class="progress-info">
    <p>Progress: {progress}%</p>
    {#if phase === 'collecting'}
      <p>{m.export_progress_collecting()}</p>
    {:else if phase === 'compressing'}
      <p>{m.export_progress_compressing()}</p>
    {:else if phase === 'finalizing'}
      <p>{m.export_progress_finalizing()}</p>
    {/if}
    {#if currentItem}
      <p>Processing: {currentItem}</p>
    {/if}
    {#if estimatedSecondsRemaining > 0}
      <p>ETA: {estimatedSecondsRemaining}s</p>
    {/if}
  </div>
</div>
