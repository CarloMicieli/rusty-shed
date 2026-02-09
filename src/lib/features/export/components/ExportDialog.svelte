<script lang="ts">
  /**
   * ExportDialog component
   * Main export workflow dialog that shows file picker, preview, and progress
   */

  export let isOpen = false;
  export let isLoading = false;
  export let error: string | null = null;

  function handleClose() {
    isOpen = false;
  }

  function handleExport() {
    // Export logic to be implemented
    console.log('Export started');
  }
</script>

<div class="export-dialog">
  {#if isOpen}
    <div class="dialog-overlay">
      <div class="dialog-content">
        <h2>Export Collection</h2>
        <p>Select the data you want to export.</p>
        {#if error}
          <div class="error">{error}</div>
        {/if}
        <button on:click={handleClose}>Cancel</button>
        <button on:click={handleExport} disabled={isLoading}>
          {isLoading ? 'Exporting...' : 'Export'}
        </button>
      </div>
    </div>
  {/if}
</div>
