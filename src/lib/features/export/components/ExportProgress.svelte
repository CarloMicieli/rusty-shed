<script lang="ts">
  /**
   * ExportProgress component
   * Displays progress bar and status during export
   */

  export let progress = 0;
  export let phase: 'collecting' | 'compressing' | 'finalizing' | null = null;
  export let currentItem: string | null = null;
  export let estimatedSecondsRemaining = 0;
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
