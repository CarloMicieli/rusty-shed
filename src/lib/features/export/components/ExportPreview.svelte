<script lang="ts">
  /**
   * ExportPreview component
   * Displays a summary of what will be exported
   */
  import type { ExportPreview } from '../types';

  export let preview: ExportPreview | null = null;
  export let isLoading = false;
</script>

<div class="export-preview">
  {#if preview}
    <div class="preview-content">
      <h3>Export Preview</h3>
      <div class="preview-items">
        <div class="item">
          <span>Railway Models</span>
          <span>{preview.railway_model_count}</span>
        </div>
        <div class="item">
          <span>Collection Items</span>
          <span>{preview.collection_item_count}</span>
        </div>
        <div class="item">
          <span>Images</span>
          <span>{preview.image_count}</span>
        </div>
      </div>
      {#if preview.warnings.length > 0}
        <div class="warnings">
          {#each preview.warnings as warning (warning)}
            <p>{warning}</p>
          {/each}
        </div>
      {/if}
    </div>
  {:else if !isLoading}
    <p>Loading preview...</p>
  {/if}
</div>
