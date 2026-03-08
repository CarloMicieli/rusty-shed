<script lang="ts">
  /**
   * ExportProgress component
   * Displays progress bar and status during export
   */

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
    {#if phase}
      <p>Phase: {phase}</p>
    {/if}
    {#if currentItem}
      <p>Processing: {currentItem}</p>
    {/if}
    {#if estimatedSecondsRemaining > 0}
      <p>ETA: {estimatedSecondsRemaining}s</p>
    {/if}
  </div>
</div>
