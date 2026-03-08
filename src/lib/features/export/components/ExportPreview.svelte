<script lang="ts">
  /**
   * ExportPreview component
   * Displays a summary of what will be exported
   */
  import * as m from '$lib/paraglide/messages.js';
  import type { ExportPreview } from '../types';

  interface Props {
    preview?: ExportPreview | null;
    isLoading?: boolean;
  }

  const { preview = null, isLoading = false }: Props = $props();
</script>

<div class="export-preview">
  {#if preview}
    <div class="preview-content">
      <h3>{m.export_preview_title()}</h3>
      <div class="preview-items">
        <div class="item">
          <span>{m.export_record_count()}</span>
          <span>{preview.railway_model_count}</span>
        </div>
        <div class="item">
          <span>Collection Items</span>
          <span>{preview.collection_item_count}</span>
        </div>
        <div class="item">
          <span>{m.export_image_count()}</span>
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
    <p>{m.app_loading()}</p>
  {/if}
</div>
