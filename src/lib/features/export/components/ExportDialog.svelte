<script lang="ts">
  /**
   * ExportDialog component
   * Main export workflow dialog that shows file picker, preview, and progress
   */
  import * as m from '$lib/paraglide/messages.js';

  interface Props {
    isOpen?: boolean;
    isLoading?: boolean;
    error?: string | null;
    onclose?: () => void;
    onexport?: () => void;
  }

  const { isOpen = false, isLoading = false, error = null, onclose, onexport }: Props = $props();
</script>

<div class="export-dialog">
  {#if isOpen}
    <div class="dialog-overlay">
      <div class="dialog-content">
        <h2>{m.export_dialog_title()}</h2>
        <p>Select the data you want to export.</p>
        {#if error}
          <div class="error">{error}</div>
        {/if}
        <button onclick={onclose}>{m.export_button_cancel()}</button>
        <button onclick={onexport} disabled={isLoading}>
          {isLoading ? m.app_loading() : m.export_button_export()}
        </button>
      </div>
    </div>
  {/if}
</div>
