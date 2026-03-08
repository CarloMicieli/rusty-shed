<script lang="ts">
  import type { ImportPreviewResponse } from '$lib/bindings';

  interface Props {
    preview: ImportPreviewResponse;
  }

  const { preview }: Props = $props();

  let duplicatesExpanded = $state(false);
</script>

<div class="duplicate-details">
  <button
    class="details-toggle"
    type="button"
    onclick={() => (duplicatesExpanded = !duplicatesExpanded)}
  >
    <span class="toggle-icon">{duplicatesExpanded ? '▼' : '▶'}</span>
    View Duplicates
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

<style>
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
</style>
